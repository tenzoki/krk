//! Die Operationsmaschine: Kopieren, Verschieben, Loeschen, Anlegen,
//! Umbenennen (C4), Packen.
//!
//! ```text
//!            Auftrag ──> starten ──> Arbeitsfaden ──> ausfuehren
//!                          │                             │
//!                          │                             ├─> zippen (ein Ziel)
//!                          │                             │
//!                          │                    quelle_fuer_quelle
//!                          │                             ├─> kopieren
//!  Hauptfaden <── Lauf ────┘                             ├─> verschieben
//!    Meldung  <── Kanal <── Steuerung <──────────────────┴─> loeschen
//!    abbrechen ─> AtomicBool ─┘                                 │
//!                                                     Papierkorb (injiziert)
//!
//!            anlegen, umbenennen: ohne Faden, sofort fertig
//! ```
//!
//! # Der Hauptfaden fuehrt keine Dateisystem-Arbeit aus
//!
//! Kein Sonderfall, keine Ausnahme. Jeder Auftrag laeuft auf einem eigenen
//! Arbeitsfaden, der Fortschritt und uebersprungene Eintraege ueber einen Kanal
//! meldet und ueber ein [`AtomicBool`](std::sync::atomic::AtomicBool)
//! abgebrochen wird. Damit haelt die Zusage L9 strukturell und nicht durch
//! Sorgfalt. Sie lautet seit dem 260807-1900: waehrend einer laufenden Kopie
//! erreicht jede Eingabe spaetestens das zweite Bild, und mindestens 65 Prozent
//! erreichen das erste. (Bis dahin stand hier "keine Eingabe wartet laenger als
//! 16 ms"; diese Fassung ist seit dem 260803-1810 ueberholt. Der Anteil stand
//! vom 260807-0832 bis zum 260807-1900 auf 85 Prozent.)
//!
//! # Drei Dinge, die dieses Modul bewusst nicht selbst tut
//!
//! **Es zaehlt nicht vor.** Wie viele Eintraege ein Ordnerbaum traegt, weiss
//! die Maschine erst, wenn sie ihn abgearbeitet hat. Ein Durchlauf vorweg
//! koennte die 200 ms aus L8 allein aufbrauchen; das Fortschrittsblatt aus S16
//! erscheint deshalb nach einer Zeitspanne und nicht nach einer Schwelle.
//!
//! **Es steigt nicht selbst durch Verzeichnisse.** Der Abstieg laeuft ueber
//! [`crate::verzeichnis::lesen`] aus Schritt 2. Der Leser ist die eine Auskunft
//! darueber, was in einem Ordner steht.
//!
//! **Es kennt AppKit nicht.** Der Papierkorb kommt als Schnittstelle herein,
//! siehe [`loeschen`].
//!
//! # Eine gescheiterte Einzelposition bricht den Stapel nicht ab
//!
//! Sie sammelt Eintrag und Grund und kommt in die Abschlussliste (C4). Nur zwei
//! Dinge beenden einen Vorgang vorzeitig: der Abbruchbefehl des Nutzers und
//! seine Antwort "abbrechen" auf eine Konfliktfrage.

pub mod anlegen;
pub mod auftrag;
pub mod fortschritt;
mod kopieren;
pub mod loeschen;
pub mod umbenennen;
mod verschieben;
mod zippen;

use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::thread;

use crate::verzeichnis::Typ;

pub use anlegen::{datei_anlegen, ordner_anlegen};
pub use auftrag::{Art, Auftrag, Konfliktregel};
pub use fortschritt::{
    Abbruchgriff, Abschluss, Bericht, Fortschritt, Konfliktantwort, Konfliktentscheid, Lauf,
    MELDEABSTAND, Meldung, Uebersprungen,
};
pub use loeschen::{OhnePapierkorb, Papierkorb};
pub use umbenennen::{Namensfehler, freier_name, name_pruefen, umbenennen};

use fortschritt::Steuerung;
use umbenennen::name_pruefen as namen_pruefen;

/// Ein Eintrag, so wie die Maschine ihn anfasst.
///
/// Der Typ und die Groesse stehen daneben, weil der Aufrufer sie schon hat: im
/// Baum aus dem Leser, an der Oberflaeche aus einem einzelnen `stat`. Wer sie
/// hier noch einmal erfragte, fragte das Dateisystem zweimal dasselbe.
pub(crate) struct Quelle<'a> {
    /// Wo der Eintrag liegt.
    pub pfad: &'a Path,
    /// Ordner, Datei oder Verknuepfung. Entscheidet ueber den Abstieg.
    pub typ: Typ,
    /// Die Groesse in Bytes. Ordner und Verknuepfungen tragen 0.
    pub groesse: u64,
}

/// Ob nach einem Eintrag weitergemacht wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ablauf {
    /// Weiter mit dem naechsten Eintrag.
    Weiter,
    /// Der ganze Vorgang endet.
    Abgebrochen,
}

/// Wohin ein Eintrag geht, nachdem ein Konflikt geklaert ist.
pub(crate) enum Zielentscheid {
    /// An diesen Pfad. Er kann vom urspruenglichen abweichen, wenn der Nutzer
    /// umbenennen gewaehlt hat.
    Nach(PathBuf),
    /// Diesen Eintrag auslassen. Er steht bereits in der Abschlussliste.
    Ueberspringen,
    /// Den ganzen Vorgang beenden.
    Abbrechen,
}

/// Startet einen Auftrag auf einem eigenen Arbeitsfaden und kehrt sofort
/// zurueck.
///
/// Der [`Lauf`] ist der Griff daran: er traegt den Kanal mit den Meldungen und
/// den Abbruch. Wird er fallen gelassen, endet der Vorgang.
pub fn starten(auftrag: Auftrag, papierkorb: Arc<dyn Papierkorb>) -> Lauf {
    let abbruch = Arc::new(AtomicBool::new(false));
    let faden_abbruch = Arc::clone(&abbruch);
    let (sender, empfaenger) = channel();
    let abschlusssender = sender.clone();

    let faden = thread::Builder::new()
        .name("krk-operation".to_owned())
        .spawn(move || {
            let mut steuerung = Steuerung::neu(faden_abbruch, Some(sender), auftrag.konfliktregel);
            let abschluss = ausfuehren(&auftrag, papierkorb.as_ref(), &mut steuerung);
            let _ = abschlusssender.send(Meldung::Fertig(steuerung.bericht(abschluss)));
        })
        .expect("Arbeitsfaden fuer eine Dateioperation laesst sich nicht starten");

    Lauf::neu(abbruch, empfaenger, faden)
}

/// Arbeitet einen Auftrag ab.
///
/// **Die Verzweigung ist vollstaendig und hat keinen Auffangzweig**: eine
/// weitere Operationsart bricht hier den Bau ab und erzwingt die Einordnung in
/// eine der beiden Bahnen, statt still in der falschen zu landen.
///
/// Vier der fuenf Arten haben je Quelle ein eigenes Ziel und laufen deshalb
/// ueber [`quelle_fuer_quelle`]. Das Packen hat **ein** Ziel fuer den ganzen
/// Lauf, das einmal geoeffnet und einmal geschlossen wird; die Begruendung
/// steht im Kopf von [`zippen`].
fn ausfuehren(
    auftrag: &Auftrag,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Abschluss {
    match &auftrag.art {
        Art::Zippen { ziel } => zippen::lauf(auftrag, ziel, steuerung),
        Art::Kopieren { .. }
        | Art::Verschieben { .. }
        | Art::InDenPapierkorb
        | Art::UmbenennenImStapel { .. } => quelle_fuer_quelle(auftrag, papierkorb, steuerung),
    }
}

/// Arbeitet einen Auftrag Quelle fuer Quelle ab.
fn quelle_fuer_quelle(
    auftrag: &Auftrag,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Abschluss {
    // Die Stelle laeuft mit, weil das Stapel-Umbenennen den neuen Namen an ihr
    // findet: er steht in der Art, Stelle fuer Stelle zu `quellen`. Die drei
    // uebrigen Arten dieser Bahn sehen sie nicht.
    for (stelle, pfad) in auftrag.quellen.iter().enumerate() {
        if steuerung.abgebrochen() {
            return Abschluss::Abgebrochen;
        }
        if einen_abarbeiten(auftrag, stelle, pfad, papierkorb, steuerung) == Ablauf::Abgebrochen {
            return Abschluss::Abgebrochen;
        }
    }
    if steuerung.abgebrochen() {
        Abschluss::Abgebrochen
    } else {
        Abschluss::Fertig
    }
}

fn einen_abarbeiten(
    auftrag: &Auftrag,
    stelle: usize,
    pfad: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Ablauf {
    let (typ, groesse) = match typ_und_groesse(pfad) {
        Ok(werte) => werte,
        Err(fehler) => {
            steuerung.ueberspringen(pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    };
    let quelle = Quelle { pfad, typ, groesse };

    match &auftrag.art {
        Art::Kopieren { ziel } => match zielpfad(&quelle, ziel, steuerung) {
            Some(ziel) => {
                kopieren::eintrag_kopieren(&quelle, &ziel, auftrag.uebertragung, steuerung)
            }
            None => Ablauf::Weiter,
        },
        Art::Verschieben { ziel } => match zielpfad(&quelle, ziel, steuerung) {
            Some(ziel) => {
                verschieben::eintrag_verschieben(&quelle, &ziel, auftrag.uebertragung, steuerung)
            }
            None => Ablauf::Weiter,
        },
        Art::InDenPapierkorb => loeschen::in_den_papierkorb(&quelle, papierkorb, steuerung),
        Art::UmbenennenImStapel { .. } => match auftrag.neuer_name(stelle) {
            Some(neuer_name) => umbenennen::eintrag_umbenennen(&quelle, neuer_name, steuerung),
            // Die beiden Listen entstehen aus denselben Paaren und sind damit
            // gleich lang. Der Fall ist trotzdem behandelt, weil ein leiser
            // Ausfall hier hiesse, einen Eintrag stillschweigend auszulassen.
            None => {
                steuerung.ueberspringen(pfad, "es fehlt der neue Name");
                Ablauf::Weiter
            }
        },
        // **Das Packen erreicht diese Schleife nicht**: [`ausfuehren`]
        // verzweigt vorher und gibt es an [`zippen::lauf`]. Der Zweig steht
        // trotzdem da, weil die Fallunterscheidung vollstaendig ist und keinen
        // Auffangzweig hat; sein Rumpf meldet statt stillzuschweigen, damit ein
        // spaeterer Umbau der Verzweigung nicht unbemerkt hier landet.
        Art::Zippen { .. } => {
            steuerung.ueberspringen(pfad, "das Packen laeuft nicht Quelle fuer Quelle");
            Ablauf::Weiter
        }
    }
}

/// Rechnet den Zielpfad einer Quelle aus und weist die drei Faelle ab, in denen
/// es keinen gibt.
///
/// Der dritte ist der gefaehrliche: ein Ordner, der in sich selbst kopiert
/// wird, waechst waehrend des Kopierens weiter, und der Abstieg fuellt den
/// Datentraeger. Die Pruefung steht hier oben und nicht im Abstieg, weil der
/// Abstieg sie sonst bei jedem Eintrag wiederholen muesste.
///
/// # Beide Fragen sind Fragen nach der Naemlichkeit und nicht nach der
/// Schreibweise
///
/// Bis zum 260819 verglich diese Funktion Pfade als Text: `ziel == quelle.pfad`
/// und `ziel.starts_with(quelle.pfad)`. Beides faengt allein den Fall, in dem
/// derselbe Ordner in beiden Pfaden **gleich geschrieben** steht. Derselbe
/// Ordner unter zwei Schreibweisen — `/tmp` gegen `/private/tmp`, ein
/// Lesezeichen ueber einen symbolischen Verweis, ein Unterschied in der Gross-
/// und Kleinschreibung auf dem hier ueblichen Datentraeger — kam an beiden
/// vorbei.
///
/// **Was daran haengt, ist kein Schoenheitsfehler, sondern die Datei des
/// Nutzers.** Kam eine Quelle an der ersten Frage vorbei, fand
/// [`ziel_klaeren`] am Ziel einen vorhandenen Eintrag, fragte den Nutzer, und
/// `Konfliktantwort::Ueberschreiben` raeumte diesen Eintrag ueber
/// [`loeschen::baum_entfernen`] weg — und weggeraeumt war die Quelle selbst.
/// Danach scheiterte das Kopieren an einer Quelle, die es nicht mehr gab, und
/// der Nutzer las in der Abschlussliste "kein Eintrag dieses Namens" ueber eine
/// Datei, die es vor seinem Abwurf noch gab. Kam eine Ordnerquelle an der
/// zweiten Frage vorbei, stieg der Kopiervorgang in den eigenen Baum ab und
/// fuellte den Datentraeger.
///
/// **Erreichbar geworden ist beides mit dem Abwurf aus einer fremden Anwendung
/// (Runde 13).** Bis dahin kamen beide Pfade aus KRK selbst und trugen die
/// Schreibweise, die der Nutzer erlaufen hatte; seither schreibt die abgebende
/// Anwendung die Quellpfade, und sie schreibt sie aufgeloest. Die Vorpruefung
/// waehrend des Ziehens (`ziel_ist_quellordner` in
/// `DateifensterQuelle::abwurf_pruefen`) vergleicht weiterhin als Text und ist
/// damit eine **Vorhersage**; entschieden wird die Frage hier, im Augenblick
/// des Zugriffs, und dort kann sie entschieden werden.
///
/// **Die zwei Fragen folgen den Verweisen verschieden**, und das ist kein
/// Versehen: die erste fragt, ueber welchen Eintrag geschrieben wuerde, und
/// bleibt deshalb beim Namen stehen (`lstat(2)`); die zweite fragt, wohin ein
/// Pfad **laeuft**, und muss deshalb folgen (`stat(2)`). Die Begruendung im
/// Einzelnen steht an [`benennen_denselben_eintrag`] und [`liegt_im_ordner`].
///
/// Der Preis sind zwei `lstat(2)` je Eintrag und, fuer eine Ordnerquelle, ein
/// `stat(2)` je Ebene des Zielpfades dazu. Sie fallen je Eintrag eines
/// laufenden Vorgangs an und nicht je Zeigerbewegung; neben dem Kopieren selbst
/// sind sie nicht zu messen.
fn zielpfad(quelle: &Quelle<'_>, zielordner: &Path, steuerung: &mut Steuerung) -> Option<PathBuf> {
    let Some(name) = quelle.pfad.file_name() else {
        steuerung.ueberspringen(quelle.pfad, "der Pfad benennt keinen Eintrag");
        return None;
    };
    let ziel = zielordner.join(name);
    if benennen_denselben_eintrag(&ziel, quelle.pfad) {
        steuerung.ueberspringen(quelle.pfad, "Quelle und Ziel sind derselbe Eintrag");
        return None;
    }
    if quelle.typ == Typ::Ordner && liegt_im_ordner(zielordner, quelle.pfad) {
        steuerung.ueberspringen(quelle.pfad, "das Ziel liegt in der Quelle");
        return None;
    }
    Some(ziel)
}

/// Die Stelle, die ein Eintrag im Dateisystem einnimmt: `(st_dev, st_ino)`.
///
/// Das Paar benennt einen Eintrag auf einem Datentraeger eindeutig und ist
/// damit die einzige Antwort auf "ist das dasselbe", die eine Schreibweise
/// nicht taeuschen kann.
fn stelle(angaben: &fs::Metadata) -> (u64, u64) {
    (angaben.dev(), angaben.ino())
}

/// Ob zwei Namen denselben Eintrag benennen, **ohne** dem letzten Namensteil zu
/// folgen.
///
/// Die Frage der ersten Pruefung in [`zielpfad`]: schreibe ich hier ueber das,
/// was ich gerade lese. Gefragt wird ueber `fs::symlink_metadata`, also
/// `lstat(2)`, und das ist hier die richtige Frage: [`ziel_klaeren`] wuerde beim
/// Ueberschreiben den **Namen** wegraeumen und nicht das, worauf er zeigt. Ist
/// das Ziel selbst eine Verknuepfung auf die Quelle, faellt beim Ueberschreiben
/// die Verknuepfung und die Quelle bleibt stehen; das ist kein Fall fuer diese
/// Pruefung. Verweise **innerhalb** des Pfades loest der Kern ohnehin auf, und
/// genau die sind der Fall, um den es geht.
///
/// **Ein Name, den es nicht gibt, ist keine Stelle**: schlaegt eines der beiden
/// `lstat(2)` fehl, lautet die Antwort `false`. Das ist die harmlose Seite —
/// ein Ziel, das es nicht gibt, kann nicht die Quelle sein, und eine Quelle, die
/// es nicht mehr gibt, scheitert gleich darauf mit ihrem eigenen Grund in der
/// Abschlussliste.
///
/// **Zwei harte Verweise auf dieselbe Datei gelten als derselbe Eintrag**, und
/// das ist gewollt, auch wenn das Ueberschreiben dort nur einen der beiden
/// Namen naehme: die Aussage "Quelle und Ziel sind derselbe Eintrag" ist dann
/// wahr, und der Eintrag steht mit seinem Grund in der Abschlussliste, statt
/// eine Datei auf sich selbst zu kopieren.
fn benennen_denselben_eintrag(einer: &Path, anderer: &Path) -> bool {
    let (Ok(einer), Ok(anderer)) = (fs::symlink_metadata(einer), fs::symlink_metadata(anderer))
    else {
        return false;
    };
    stelle(&einer) == stelle(&anderer)
}

/// Ob `zielordner` der Quellordner selbst ist oder unter ihm liegt, **mit**
/// Verfolgung der Verweise.
///
/// Die Frage der zweiten Pruefung in [`zielpfad`]: steigt der Kopiervorgang in
/// den eigenen Baum ab. Gefragt wird hier ueber `fs::metadata`, also `stat(2)`,
/// und der Unterschied zur Pruefung darueber ist tragend: ein Pfad **laeuft**
/// durch seine Verweise hindurch. Ist `verweis` eine Verknuepfung auf den
/// Quellordner, dann liegt `verweis/unten` wirklich in der Quelle, und ein
/// `lstat(2)` auf `verweis` saehe die Verknuepfung statt des Ordners, auf den
/// sie zeigt.
///
/// Gelaufen wird ueber `Path::ancestors`, das mit dem Ordner selbst beginnt.
/// Der Fall "das Ziel ist die Quelle" ist damit mit erfasst.
fn liegt_im_ordner(zielordner: &Path, quellordner: &Path) -> bool {
    let Ok(quelle) = fs::metadata(quellordner) else {
        return false;
    };
    zielordner
        .ancestors()
        .any(|oben| fs::metadata(oben).is_ok_and(|oben| stelle(&oben) == stelle(&quelle)))
}

/// Klaert, wohin ein Eintrag geht, wenn am Ziel schon etwas steht.
///
/// Ein Ordner auf einen gleichnamigen Ordner ist **kein** Konflikt: sein Inhalt
/// wandert in den vorhandenen. Andernfalls entscheidet die Konfliktregel des
/// Auftrags, notfalls durch Nachfragen beim Nutzer.
pub(crate) fn ziel_klaeren(
    quelle: &Quelle<'_>,
    ziel: &Path,
    steuerung: &mut Steuerung,
) -> Zielentscheid {
    let Ok(vorhanden) = fs::symlink_metadata(ziel) else {
        return Zielentscheid::Nach(ziel.to_path_buf());
    };
    if quelle.typ == Typ::Ordner && vorhanden.is_dir() {
        return Zielentscheid::Nach(ziel.to_path_buf());
    }

    match steuerung.konflikt_loesen(quelle.pfad, ziel) {
        Konfliktantwort::Ueberschreiben => match loeschen::baum_entfernen(ziel) {
            Ok(()) => Zielentscheid::Nach(ziel.to_path_buf()),
            Err(fehler) => {
                steuerung.ueberspringen(
                    quelle.pfad,
                    format!("das Ziel liess sich nicht ersetzen: {}", grund(&fehler)),
                );
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Ueberspringen => {
            steuerung.ueberspringen(quelle.pfad, "am Ziel steht schon ein Eintrag");
            Zielentscheid::Ueberspringen
        }
        Konfliktantwort::UmbenennenIn(name) => match namen_pruefen(&name) {
            Ok(()) => Zielentscheid::Nach(ziel.with_file_name(name)),
            Err(fehler) => {
                steuerung.ueberspringen(quelle.pfad, fehler.grund());
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Abbrechen => Zielentscheid::Abbrechen,
    }
}

/// Fragt Typ und Groesse eines Eintrags ab, ohne einer Verknuepfung zu folgen.
fn typ_und_groesse(pfad: &Path) -> io::Result<(Typ, u64)> {
    let angaben = fs::symlink_metadata(pfad)?;
    let typ = if angaben.is_symlink() {
        Typ::Verknuepfung
    } else if angaben.is_dir() {
        Typ::Ordner
    } else {
        Typ::Datei
    };
    let groesse = if typ == Typ::Datei { angaben.len() } else { 0 };
    Ok((typ, groesse))
}

/// Uebersetzt einen Systemfehler in den Grund, den die Abschlussliste zeigt.
///
/// Die vier haeufigen Faelle stehen auf Deutsch da, weil der Nutzer sie liest.
/// Alles andere behaelt den Wortlaut des Systems: eine erfundene Uebersetzung
/// waere ungenauer als das Original.
pub(crate) fn grund(fehler: &io::Error) -> String {
    match fehler.kind() {
        io::ErrorKind::PermissionDenied => "keine Rechte".to_owned(),
        io::ErrorKind::NotFound => "gibt es nicht mehr".to_owned(),
        io::ErrorKind::AlreadyExists => "am Ziel steht schon ein Eintrag".to_owned(),
        io::ErrorKind::StorageFull => "kein Platz mehr auf dem Datentraeger".to_owned(),
        _ => fehler.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die beiden Pruefungen von [`zielpfad`] stehen **nicht** hier, sondern in
    /// `tests/operation.rs`.
    ///
    /// Bis zum 260819 standen sie hier und reichten [`zielpfad`] erfundene
    /// Pfade wie `/tmp/krk-ordner`, die es auf keinem Datentraeger gab. Das
    /// ging, solange die beiden Fragen Text verglichen. Seit sie nach `st_dev`
    /// und `st_ino` fragen, brauchen sie einen Ordner, den es wirklich gibt,
    /// und ein selbstabraeumender Pruefordner ist in dieser Kiste genau einer:
    /// `tests/gemeinsam/mod.rs`. Eine vierte Fassung daneben verbietet
    /// `CLAUDE.md` ausdruecklich, und ein Testziel unter `tests/` erreicht sie.
    ///
    /// Was dort steht, deckt beide Fragen in beiden Schreibweisen ab:
    /// `eine_quelle_kann_nicht_auf_ihren_eigenen_ordner_kopiert_werden` und
    /// `ein_ziel_das_ueber_einen_verweis_die_quelle_selbst_ist_wird_uebersprungen`
    /// fuer die erste, `ein_ordner_laesst_sich_nicht_in_sich_selbst_kopieren`
    /// und `ein_ziel_das_ueber_einen_verweis_in_der_quelle_liegt_wird_uebersprungen`
    /// fuer die zweite.
    #[test]
    fn die_haeufigen_gruende_stehen_auf_deutsch_da() {
        let fehler = io::Error::from(io::ErrorKind::PermissionDenied);
        assert_eq!(grund(&fehler), "keine Rechte");
    }
}

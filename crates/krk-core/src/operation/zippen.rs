//! Packen: die betroffenen Eintraege wandern in **ein** Archiv.
//!
//! ```text
//! lauf ──> zielarchiv_klaeren  ──> steuerung.konflikt_loesen (einmal, vorweg)
//!                              └─> "ueberschreiben" ──> Papierkorb
//!      ──> File::create ──> ZipWriter
//!      ──> je Quelle: eintrag_packen ──> Typ::Datei        ──> start_file + Stuecke
//!                                    ──> Typ::Ordner       ──> add_directory
//!                                                              + verzeichnis::lesen ──┐
//!                                    ──> Typ::Verknuepfung ──> add_symlink            │
//!                                    <─────────────────────────── je Eintrag ─────────┘
//!      ──> finish; nach einem Abbruch das halbe Archiv wegraeumen
//! ```
//!
//! # Warum das Packen neben der Quelle-fuer-Quelle-Schleife steht
//!
//! Die uebrigen Arten haben je Quelle ein eigenes Ziel; das Packen hat **ein**
//! Ziel fuer den ganzen Lauf, und dieses Ziel wird einmal geoeffnet und einmal
//! geschlossen. Wer den Schreiber durch [`super::einen_abarbeiten`] faedelte,
//! reichte einen Zustand durch vier Arten hindurch, die ihn nicht ansehen.
//! [`super::ausfuehren`] verzweigt deshalb ueber die Art, vollstaendig und ohne
//! Auffangzweig.
//!
//! # Der Konflikt wird einmal geklaert, und zwar vor dem ersten Byte
//!
//! Ein Lauf erzeugt genau eine Zieldatei. Steht sie schon da, ist das der
//! einzige Konflikt dieses Vorgangs, und er wird beantwortet, **bevor**
//! `File::create` die vorhandene Datei abschneidet. Umgekehrt waere die Frage
//! eine Hoeflichkeit ueber eine Datei, die es schon nicht mehr gaebe.
//!
//! # "Ueberschreiben" raeumt in den Papierkorb und loescht nicht
//!
//! Dieselbe Bindung wie beim Entpacken, und seit dem 260825 auch derselbe Weg:
//! seit der Runde 12 gibt es genau einen Loeschweg, und der fuehrt in den
//! Papierkorb. Ein vorhandener Eintrag am Archivnamen geht deshalb ueber die
//! hereingereichte [`Papierkorb`]-Schnittstelle und **nicht** ueber
//! [`super::loeschen::baum_entfernen`]. Bis dahin nahm dieser Zweig den
//! Baumloescher, und ein Ordner am Archivnamen war damit unwiederbringlich weg;
//! der Nutzer hat den Unterschied aufgehoben
//! (`issues/260825-0942_*_ueberschreiben-loescht-beim-packen-endgueltig-und-beim-entpacken-in-den-papierkorb.md`,
//! Moeglichkeit 1). "Ueberschreiben"
//! bedeutet seither im ganzen Kontextmenue dasselbe.
//!
//! # Angetastet wird allein der Eintrag, der genau so heisst wie das Archiv
//!
//! Die Zusage, die der Nutzer der Antwort mitgegeben hat: in den Papierkorb geht
//! **ein** Eintrag, und zwar der, dessen Name dem Archivnamen genau gleicht. Ein
//! vorhandenes `Projekte.zip` geht; ein daneben liegender Ordner `Projekte`
//! **nicht**, gleich wie aehnlich er heisst. Gehalten wird das von der Probe
//! `ueberschreiben_raeumt_allein_den_gleichnamigen_eintrag_in_den_papierkorb`
//! in `tests/operation.rs`.
//!
//! **Dass dieser Eintrag keine Quelle desselben Laufs ist, sichert der Rufer und
//! nicht dieser Zweig.** Bis zum 260825 stand hier ein anderes Argument: keine
//! der zwei Stellen, die etwas wegnehmen ([`zielarchiv_klaeren`] und
//! [`halbes_archiv_wegraeumen`]), nenne `auftrag.quellen`. Das ist wahr und
//! beantwortet die Frage nicht — es sagt, welche Variable gelesen wird, und
//! nicht, ob der **Pfadwert** `ziel` mit einem Quellpfad zusammenfaellt. Er kann
//! es: beim zweiten Zip-Lauf ueber denselben Ordner steht das
//! `Projekte/Projekte.zip` des ersten selbst in der Markierung und wird erneut
//! zum Archivnamen.
//!
//! Der Kern vergleicht die Pfade trotzdem nicht, und das ist die Antwort des
//! Nutzers vom 260825
//! (`issues/260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-papierkorb-*`,
//! der kleinere der zwei Wege): der Schnitt faellt in der
//! Oberflaeche, die beide Listen bildet, naemlich in
//! `kommandos::kontextmenue::packziel` der Kiste `krk-ui`, gehalten von dessen
//! Probe `das_archiv_des_vorigen_laufs_faellt_aus_den_quellen`. Hier gilt
//! deshalb schlicht: geraeumt wird der Zielpfad, wer immer ihn hereinreicht.
//! Steht er doch einmal auf der Quellenliste, geht er in den Papierkorb und
//! fehlt dem Lauf danach als Quelle, die er als ausgelassen meldet.
//!
//! **Und das Glied dazwischen ist seit dem 260825 ebenfalls gehalten.** Bis
//! dahin sagte dieser Abschnitt „sichert der Rufer" und liess offen, wer den
//! Rufer sichert: `Auftrag::zippen(quellen, ziel)` nimmt zwei unabhaengige
//! Listen, und wer im Ausfuehrungszweig weiterhin `packziel` ruft, davon aber
//! nur das Ziel nimmt und die ungeschnittene Markierung als Quellen
//! weiterreicht, bekam den Defekt zurueck, ohne dass eine Probe rot wurde
//! (`issues/260825-1249_*_die-zusage-haengt-jetzt-am-rufer-in-einer-anderen-kiste-*`).
//! Zwei Zaehlproben im Rufer schliessen die Kette:
//! `der_packauftrag_reicht_die_quellen_aus_packziel_weiter` haelt, dass die
//! geschnittene Liste den Auftrag erreicht, und
//! `ein_packauftrag_entsteht_in_der_oberflaeche_genau_einmal`, dass daneben kein
//! zweiter Eingang entsteht; beide stehen in
//! `krk-ui/src/appkit/anwendung.rs`, Pruefmodul `kontextproben`.
//!
//! # Einer Verknuepfung wird nicht gefolgt
//!
//! Gepackt wird die Verknuepfung, nicht ihr Ziel — dieselbe Wahl und derselbe
//! Grund wie in [`super::kopieren`]: wer einem Verweis folgte, packte einen
//! Ordner doppelt, sobald er auf sich selbst zeigt, und der Abstieg endete nie.
//!
//! # Gelesen wird ohne zu warten, und der Typ wird am Deskriptor gefragt
//!
//! Jede Datei geht durch
//! [`verzeichnis::sys::ohne_warten_oeffnen`](crate::verzeichnis::sys::ohne_warten_oeffnen)
//! und nicht durch `File::open`. Das Oeffnen selbst haengt damit nicht: eine
//! benannte Roehre, an der kein Schreiber steht, liesse `File::open` warten, bis
//! jemand sie oeffnet.
//!
//! **Das Oeffnen ist aber nur die halbe Sperre, und bis zum 260825 stand hier
//! nur diese Haelfte.** Die Huelle nimmt `O_NONBLOCK` wieder ab, bevor sie den
//! Deskriptor herausgibt; an einer Roehre, an der ein Schreiber **steht**,
//! bliebe `read(2)` danach unbegrenzt stehen, und der Abbruch wird erst nach
//! einem geglueckten `read` geprueft — `Esc` erreichte den Lauf also nicht mehr
//! (`issues/260825-0942_*_das-packen-haengt-an-einer-benannten-roehre-mit-schreiber-und-die-probe-kann-es-nicht-sehen.md`).
//! [`datei_packen`] fragt
//! deshalb `metadata()` **am offenen Deskriptor** und laesst alles aus, was
//! `is_file()` nicht bejaht, mit seinem Grund in der Abschlussliste.
//!
//! Die Frage steht hier und nicht in [`super::typ_und_groesse`]: dessen
//! [`Typ::Datei`] ist das Auffangfach fuer alles, was weder Ordner noch
//! Verknuepfung ist, und traegt Roehren, Geraete und Sockel mit. Und sie steht
//! am Deskriptor und nicht am Pfad, aus demselben Grund wie in
//! [`crate::text::datei`]: zwischen einer Frage am Pfad und dem Oeffnen liegt
//! ein Fenster, in dem der Eintrag ein anderer werden kann.
//!
//! # Jeder Eintrag traegt das Aenderungsdatum seiner Quelle, und zwar dreimal
//!
//! Das Zip-Format kennt genau ein Pflichtfeld fuer die Zeit, und es ist eine
//! **buergerliche Ortszeit ohne Zonenangabe** im MS-DOS-Format, gerastert auf
//! zwei Sekunden. Es allein genuegt nicht, und das ist gemessen: `ditto(1)`
//! rechnet es beim Auspacken mit dem **heute** geltenden Zonenversatz zurueck
//! und legt eine Januardatei im August eine Stunde daneben ab. Deshalb stehen
//! daneben zwei Zusatzfelder, die die Zeit in Epochensekunden tragen und keine
//! Zone brauchen: [`FELD_ERWEITERTE_ZEIT`] (`0x5455`), das `unzip` liest, und
//! [`FELD_INFOZIP_UNIX`] (`0x5855`), das `ditto(1)` liest. Welches Werkzeug
//! welches uebergeht, steht als Messtabelle in der Wurzel-`Cargo.toml` neben
//! dem Merkmal `unreserved`, das `0x5855` ueberhaupt erst zulaesst.
//!
//! Die Umrechnung in die Ortszeit macht [`crate::verzeichnis::sys::ortszeit`]
//! ueber `localtime_r(3)`, also **mit dem Versatz, der zum Dateidatum galt**.
//! Ein Zeitpunkt, den das MS-DOS-Feld nicht fasst (vor 1980, nach 2107),
//! faellt auf `DateTime::DEFAULT` zurueck und bekommt eine Zeile in der
//! Abschlussliste; abgewiesen wird der Eintrag deswegen nicht, so wie das
//! Packen auch sonst aufschreibt statt abzuweisen.
//!
//! Die Gegenrichtung steht in [`super::entpacken`], und die zwei Enden gehoeren
//! zusammen: was hier hineingeschrieben wird, liest dort [`super::entpacken`]
//! aus denselben zwei Zusatzfeldern wieder heraus.

use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use zip::DateTime;
use zip::ZipWriter;
use zip::write::FullFileOptions;

use crate::verzeichnis::{Typ, lesen};

use super::fortschritt::Steuerung;
use super::umbenennen::name_pruefen;
use super::{
    Abschluss, Auftrag, Konfliktantwort, Papierkorb, Quelle, STUECK, Zielentscheid, grund,
    typ_und_groesse,
};

/// Was nach einem Eintrag geschieht.
///
/// Drei Werte und nicht zwei: ein **gescheiterter Eintrag** haelt den Stapel
/// nicht auf (C4), ein **hin gewordenes Archiv** dagegen schon. Nach einem
/// Schreibfehler am Archiv ist jeder weitere Eintrag verlorene Arbeit, denn was
/// dasteht, laesst sich ohnehin nicht mehr oeffnen.
///
/// **`#[must_use]` steht am Typ und nicht an den fuenf Funktionen, die ihn
/// liefern** — dieselbe Marke aus demselben Grund wie an [`super::Ablauf`], und
/// die zwei sind als Paar zu lesen. Ein fallen gelassenes `Abgebrochen` liesse
/// den Lauf ueber die abgebrochene Stelle hinaus weiterlaufen; ein fallen
/// gelassenes `ArchivHin` liesse ihn in ein Archiv weiterschreiben, das nicht
/// mehr zu schreiben ist. Beides bliebe ohne die Marke unbemerkt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
enum Packschritt {
    /// Weiter mit dem naechsten Eintrag.
    Weiter,
    /// Der Nutzer hat abgebrochen.
    Abgebrochen,
    /// Das Archiv selbst laesst sich nicht mehr schreiben.
    ArchivHin,
}

/// Packt die Quellen des Auftrags in das genannte Archiv.
///
/// Der [`Papierkorb`] reist mit, weil "ueberschreiben" ihn braucht; gerufen
/// wird er hoechstens einmal je Lauf, naemlich in [`zielarchiv_klaeren`].
pub(crate) fn lauf(
    auftrag: &Auftrag,
    ziel: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Abschluss {
    if steuerung.abgebrochen() {
        return Abschluss::Abgebrochen;
    }

    let archiv = match zielarchiv_klaeren(auftrag, ziel, papierkorb, steuerung) {
        Zielentscheid::Nach(archiv) => archiv,
        // Ein Lauf, ein Ziel: wird es ausgelassen, bleibt nichts zu tun. Der
        // Grund steht bereits in der Abschlussliste.
        Zielentscheid::Ueberspringen => return Abschluss::Fertig,
        Zielentscheid::Abbrechen => return Abschluss::Abgebrochen,
    };

    let datei = match File::create(&archiv) {
        Ok(datei) => datei,
        Err(fehler) => {
            steuerung.ueberspringen(&archiv, grund(&fehler));
            return Abschluss::Fertig;
        }
    };
    // `set_auto_large_file` schaltet die ZIP64-Form fuer den einzelnen Eintrag
    // ein, der die 4-GiB-Grenze reisst, und nur fuer ihn. Ohne sie bricht ein
    // solcher Eintrag mit einem Fehler ab, statt sich packen zu lassen.
    let mut schreiber = ZipWriter::new(BufWriter::new(datei)).set_auto_large_file();

    let stand = quellen_packen(auftrag, &mut schreiber, steuerung);

    let abgeschlossen = match stand {
        Packschritt::Weiter => match schreiber.finish() {
            Ok(_) => return Abschluss::Fertig,
            Err(fehler) => {
                steuerung.ueberspringen(&archiv, format!("das Archiv blieb unfertig: {fehler}"));
                Abschluss::Fertig
            }
        },
        Packschritt::Abgebrochen => {
            drop(schreiber);
            Abschluss::Abgebrochen
        }
        Packschritt::ArchivHin => {
            drop(schreiber);
            Abschluss::Fertig
        }
    };

    halbes_archiv_wegraeumen(&archiv, steuerung);
    abgeschlossen
}

/// Klaert das Zielarchiv, bevor ein Byte geschrieben wird.
///
/// **Alle vier Antworten sind behandelt**, auch wenn das Blatt der Oberflaeche
/// bei genau einer Zieldatei nur drei davon anbietet: der Kern kennt vier, und
/// die Konfliktregel des Auftrags kann jede davon liefern, ohne dass ein Blatt
/// im Spiel waere.
///
/// "Umbenennen in" prueft den Namen und legt das Archiv daneben. Ein zweiter
/// Konflikt unter dem neuen Namen wird **nicht** noch einmal erfragt, so wie
/// [`super::ziel_klaeren`] ihn nicht noch einmal erfragt: der Vorschlag der
/// Oberflaeche ist ein freier Name, und eine Kette von Rueckfragen ueber
/// dieselbe eine Datei waere keine Auskunft mehr.
///
/// **"Ueberschreiben" nimmt `ziel` und sonst nichts**, und `ziel` ist der volle
/// Pfad des Archivs. Ein Nachbar mit aehnlichem Namen und die Quellen des Laufs
/// kommen an dieser Zeile nicht vor; die Zusage steht im Kopf dieser Datei.
fn zielarchiv_klaeren(
    auftrag: &Auftrag,
    ziel: &Path,
    papierkorb: &dyn Papierkorb,
    steuerung: &mut Steuerung,
) -> Zielentscheid {
    if fs::symlink_metadata(ziel).is_err() {
        return Zielentscheid::Nach(ziel.to_path_buf());
    }

    // Die Zeile "Quelle:" des Blattes nennt den ersten der gepackten Eintraege.
    // Er ist bei genau einem Eintrag der ganze Gegenstand des Laufs und bei
    // mehreren wenigstens einer von ihnen; ohne Quellen faellt die Wahl auf den
    // Ordner, in dem das Archiv entstehen soll, damit dort kein leerer Pfad
    // steht.
    let herkunft = auftrag
        .quellen
        .first()
        .map(PathBuf::as_path)
        .or_else(|| ziel.parent())
        .unwrap_or(ziel);

    match steuerung.konflikt_loesen(herkunft, ziel) {
        Konfliktantwort::Ueberschreiben => match papierkorb.in_den_papierkorb(ziel) {
            Ok(_) => Zielentscheid::Nach(ziel.to_path_buf()),
            Err(fehler) => {
                steuerung.ueberspringen(
                    ziel,
                    format!(
                        "das Ziel liess sich nicht in den Papierkorb raeumen: {}",
                        grund(&fehler)
                    ),
                );
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Ueberspringen => {
            steuerung.ueberspringen(ziel, "am Ziel steht schon ein Eintrag");
            Zielentscheid::Ueberspringen
        }
        Konfliktantwort::UmbenennenIn(name) => match name_pruefen(&name) {
            Ok(()) => Zielentscheid::Nach(ziel.with_file_name(name)),
            Err(fehler) => {
                steuerung.ueberspringen(ziel, fehler.grund());
                Zielentscheid::Ueberspringen
            }
        },
        Konfliktantwort::Abbrechen => Zielentscheid::Abbrechen,
    }
}

/// Raeumt ein Archiv weg, das nicht fertig geworden ist.
///
/// **Dieselbe Ueberlegung wie bei der halben Kopie in [`super::kopieren`]:** was
/// dasteht, ist kein Ergebnis, sondern ein Rest. Ein halbes Archiv traegt kein
/// Verzeichnis am Ende und laesst sich von keinem Werkzeug oeffnen; wer es
/// stehen liesse, hinterliesse dem Nutzer eine Datei, die aussieht wie sein
/// Archiv und keines ist.
fn halbes_archiv_wegraeumen(archiv: &Path, steuerung: &mut Steuerung) {
    if let Err(fehler) = fs::remove_file(archiv)
        && fehler.kind() != io::ErrorKind::NotFound
    {
        steuerung.ueberspringen(
            archiv,
            format!("nach dem Abbruch nicht weggeraeumt: {}", grund(&fehler)),
        );
    }
}

/// Packt jede Quelle des Auftrags unter ihrem eigenen Namen.
fn quellen_packen(
    auftrag: &Auftrag,
    schreiber: &mut ZipWriter<BufWriter<File>>,
    steuerung: &mut Steuerung,
) -> Packschritt {
    for pfad in &auftrag.quellen {
        if steuerung.abgebrochen() {
            return Packschritt::Abgebrochen;
        }
        let Some(name) = pfad.file_name() else {
            steuerung.ueberspringen(pfad, "der Pfad benennt keinen Eintrag");
            continue;
        };
        let (typ, groesse) = match typ_und_groesse(pfad) {
            Ok(werte) => werte,
            Err(fehler) => {
                steuerung.ueberspringen(pfad, grund(&fehler));
                continue;
            }
        };
        let quelle = Quelle { pfad, typ, groesse };
        let stand = eintrag_packen(&quelle, &name.to_string_lossy(), schreiber, steuerung);
        if stand != Packschritt::Weiter {
            return stand;
        }
    }
    Packschritt::Weiter
}

/// Packt einen Eintrag unter dem genannten Namen im Archiv.
///
/// Der Name traegt Schraegstriche als Trenner, wie das Format es verlangt, und
/// ist damit **kein** Pfad des Dateisystems. Gebildet wird er beim Abstieg aus
/// dem Namen des uebergeordneten Eintrags; die Wurzel ist der blosse Name der
/// Quelle, nicht ihr voller Pfad.
fn eintrag_packen(
    quelle: &Quelle<'_>,
    name_im_archiv: &str,
    schreiber: &mut ZipWriter<BufWriter<File>>,
    steuerung: &mut Steuerung,
) -> Packschritt {
    match quelle.typ {
        Typ::Datei => datei_packen(quelle, name_im_archiv, schreiber, steuerung),
        Typ::Ordner => ordner_packen(quelle, name_im_archiv, schreiber, steuerung),
        Typ::Verknuepfung => verknuepfung_packen(quelle, name_im_archiv, schreiber, steuerung),
    }
}

/// Packt eine einzelne Datei, Stueck fuer Stueck.
///
/// Der Abbruch wird **innerhalb** der Datei geprueft und nicht nur zwischen
/// zwei Eintraegen: eine Datei von zwei Gigabyte liefe sonst zu Ende, gleich
/// wie oft der Nutzer `Esc` drueckt.
fn datei_packen(
    quelle: &Quelle<'_>,
    name_im_archiv: &str,
    schreiber: &mut ZipWriter<BufWriter<File>>,
    steuerung: &mut Steuerung,
) -> Packschritt {
    let mut gelesen = match crate::verzeichnis::sys::ohne_warten_oeffnen(quelle.pfad) {
        Ok(datei) => datei,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Packschritt::Weiter;
        }
    };

    // **Die Typfrage am offenen Deskriptor**, und zwar vor `start_file`: was
    // hier ausgelassen wird, soll auch keine leere Zeile im Archiv bekommen.
    // Der Grund im Einzelnen steht im Kopf dieser Datei; kurz: `Typ::Datei` aus
    // [`super::typ_und_groesse`] ist das Auffangfach und traegt auch Roehren,
    // Geraete und Sockel, und an einer Roehre mit Schreiber bliebe das `read`
    // darunter unbegrenzt stehen.
    // Dieselbe Antwort traegt drei Fragen: den Typ, die Rechte und das
    // Aenderungsdatum. Sie am Pfad ein zweites Mal zu stellen, kostete einen
    // Systemaufruf und oeffnete das Fenster wieder, das der Deskriptor schliesst.
    let angaben = match gelesen.metadata() {
        Ok(angaben) if angaben.is_file() => angaben,
        Ok(_) => {
            steuerung.ueberspringen(quelle.pfad, "keine gewoehnliche Datei");
            return Packschritt::Weiter;
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Packschritt::Weiter;
        }
    };

    let wahl = dateiwahl(quelle.pfad, &angaben, steuerung);
    if let Err(fehler) = schreiber.start_file(name_im_archiv, wahl) {
        steuerung.ueberspringen(quelle.pfad, format!("kein Platz im Archiv: {fehler}"));
        return Packschritt::ArchivHin;
    }

    let mut puffer = vec![0_u8; STUECK];
    let mut bytes = 0_u64;
    loop {
        let stueck = match gelesen.read(&mut puffer) {
            Ok(0) => break,
            Ok(gelesen) => gelesen,
            Err(fehler) => {
                // Der Eintrag ist halb geschrieben. `abort_file` nimmt ihn
                // wieder aus dem Archiv; die uebrigen Quellen laufen weiter,
                // wie C4 es fuer eine gescheiterte Einzelposition verlangt.
                steuerung.teilstueck(bytes);
                steuerung.ueberspringen(quelle.pfad, grund(&fehler));
                return match schreiber.abort_file() {
                    Ok(()) => Packschritt::Weiter,
                    Err(fehler) => {
                        steuerung.ueberspringen(
                            quelle.pfad,
                            format!("der halbe Eintrag blieb im Archiv: {fehler}"),
                        );
                        Packschritt::ArchivHin
                    }
                };
            }
        };
        if let Err(fehler) = schreiber.write_all(&puffer[..stueck]) {
            steuerung.teilstueck(bytes);
            steuerung.ueberspringen(
                quelle.pfad,
                format!("nicht ins Archiv geschrieben: {fehler}"),
            );
            return Packschritt::ArchivHin;
        }
        bytes += stueck as u64;
        steuerung.zwischenstand(quelle.pfad, bytes);
        if steuerung.abgebrochen() {
            steuerung.teilstueck(bytes);
            return Packschritt::Abgebrochen;
        }
    }

    steuerung.eintrag_fertig(quelle.pfad, bytes);
    Packschritt::Weiter
}

/// Packt einen Ordner samt Inhalt, Eintrag fuer Eintrag.
///
/// Der leere Ordnereintrag steht **vor** seinem Inhalt im Archiv. Er ist keine
/// Zierde: ohne ihn verlaere ein leerer Ordner sich beim Entpacken, denn im
/// Zip-Format steht ein Ordner allein durch seine eigene Zeile da.
fn ordner_packen(
    quelle: &Quelle<'_>,
    name_im_archiv: &str,
    schreiber: &mut ZipWriter<BufWriter<File>>,
    steuerung: &mut Steuerung,
) -> Packschritt {
    let wahl = ordnerwahl(quelle.pfad, steuerung);
    if let Err(fehler) = schreiber.add_directory(name_im_archiv, wahl) {
        steuerung.ueberspringen(quelle.pfad, format!("kein Platz im Archiv: {fehler}"));
        return Packschritt::ArchivHin;
    }

    let eintraege = match lesen(quelle.pfad) {
        Ok(eintraege) => eintraege,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Packschritt::Weiter;
        }
    };

    for eintrag in eintraege {
        if steuerung.abgebrochen() {
            return Packschritt::Abgebrochen;
        }
        let unterquelle = quelle.pfad.join(&eintrag.name);
        let kind = Quelle {
            pfad: &unterquelle,
            typ: eintrag.typ,
            groesse: eintrag.groesse,
        };
        let untername = format!("{name_im_archiv}/{}", eintrag.name);
        let stand = eintrag_packen(&kind, &untername, schreiber, steuerung);
        if stand != Packschritt::Weiter {
            return stand;
        }
    }

    steuerung.eintrag_fertig(quelle.pfad, 0);
    Packschritt::Weiter
}

/// Packt eine symbolische Verknuepfung als Verknuepfung, nicht ihr Ziel.
///
/// Der Inhalt des Eintrags ist das Verweisziel, und die Rechte tragen
/// `S_IFLNK`. Wer setzt das Kennzeichen und warum die Wahl es trotzdem nennt,
/// steht an [`verknuepfungswahl`].
fn verknuepfung_packen(
    quelle: &Quelle<'_>,
    name_im_archiv: &str,
    schreiber: &mut ZipWriter<BufWriter<File>>,
    steuerung: &mut Steuerung,
) -> Packschritt {
    let verweis = match fs::read_link(quelle.pfad) {
        Ok(verweis) => verweis,
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Packschritt::Weiter;
        }
    };
    let wahl = verknuepfungswahl(quelle.pfad, steuerung);
    match schreiber.add_symlink(name_im_archiv, verweis.to_string_lossy(), wahl) {
        Ok(()) => {
            steuerung.eintrag_fertig(quelle.pfad, 0);
            Packschritt::Weiter
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, format!("kein Platz im Archiv: {fehler}"));
            Packschritt::ArchivHin
        }
    }
}

/// Die Wahl fuer einen Dateieintrag: verdichtet, mit den Rechten und dem
/// Aenderungsdatum der Quelle.
///
/// `angaben` ist die Antwort, die [`datei_packen`] schon am offenen Deskriptor
/// eingeholt hat, und sie wird hier nicht ein zweites Mal am Pfad erfragt.
#[must_use = "ohne die Wahl traegt der Eintrag weder Rechte noch Datum"]
fn dateiwahl(
    pfad: &Path,
    angaben: &fs::Metadata,
    steuerung: &mut Steuerung,
) -> FullFileOptions<'static> {
    let wahl = zeit_uebernehmen(FullFileOptions::default(), pfad, Some(angaben), steuerung);
    rechte_uebernehmen(wahl, Some(angaben), 0o644)
}

/// Die Wahl fuer einen Ordnereintrag.
///
/// Das Verfahren steht hier nicht: `add_directory` setzt es selbst auf
/// "gespeichert", denn ein Ordnereintrag hat keinen Inhalt, den zu verdichten
/// sich lohnte.
///
/// Gefragt wird am Pfad, denn einen Deskriptor auf den Ordner haelt der Packlauf
/// nicht: er liest ihn ueber [`lesen`] und nicht ueber ein `open`.
#[must_use = "ohne die Wahl traegt der Eintrag weder Rechte noch Datum"]
fn ordnerwahl(pfad: &Path, steuerung: &mut Steuerung) -> FullFileOptions<'static> {
    let angaben = fs::metadata(pfad).ok();
    let wahl = zeit_uebernehmen(
        FullFileOptions::default(),
        pfad,
        angaben.as_ref(),
        steuerung,
    );
    rechte_uebernehmen(wahl, angaben.as_ref(), 0o755)
}

/// Die Wahl fuer eine Verknuepfung: ihr eigenes Datum, nicht das ihres Ziels.
///
/// Gefragt wird mit `lstat(2)` und nicht mit `stat(2)`, aus demselben Grund, aus
/// dem der Packlauf einer Verknuepfung nicht folgt: gepackt ist die
/// Verknuepfung, also gehoert in den Eintrag ihr eigenes Aenderungsdatum.
///
/// **Die Rechte sind fest und kommen nicht aus den Angaben.** Der Typ steht in
/// den oberen Modusbits, und `unix_permissions` maskiert mit `& 0o777`, wirft
/// `S_IFLNK` also fort; gesetzt wird das Kennzeichen von `add_symlink`, das die
/// Rechte nur ergaenzt, wenn keine dastehen. `0o120777` steht hier trotzdem,
/// damit der Eintrag es auch dann traegt, wenn `add_symlink` es einmal nicht
/// mehr selbst setzt.
#[must_use = "ohne die Wahl traegt der Eintrag weder Rechte noch Datum"]
fn verknuepfungswahl(pfad: &Path, steuerung: &mut Steuerung) -> FullFileOptions<'static> {
    let angaben = fs::symlink_metadata(pfad).ok();
    let wahl = zeit_uebernehmen(
        FullFileOptions::default(),
        pfad,
        angaben.as_ref(),
        steuerung,
    );
    wahl.unix_permissions(0o120777)
}

/// Uebernimmt die Rechte der Quelle in die Wahl, notfalls die genannte Vorgabe.
///
/// Ein Archiv, das die Rechte fortwirft, macht aus einem ausfuehrbaren Skript
/// beim Entpacken eine gewoehnliche Datei. Laesst sich die Quelle gerade nicht
/// befragen, steht die uebliche Vorgabe da; ein Eintrag ohne Rechte waere
/// schlechter als ein Eintrag mit den ueblichen.
#[must_use = "ohne die Wahl traegt der Eintrag keine Rechte"]
fn rechte_uebernehmen(
    wahl: FullFileOptions<'static>,
    angaben: Option<&fs::Metadata>,
    vorgabe: u32,
) -> FullFileOptions<'static> {
    use std::os::unix::fs::PermissionsExt;

    let rechte = angaben.map_or(vorgabe, |angaben| angaben.permissions().mode());
    wahl.unix_permissions(rechte)
}

/// Kennung des erweiterten Zeitfeldes (`Extended Timestamp`), das `unzip` liest.
///
/// Rumpf: ein Kennzeichenbyte, dann die genannten Zeiten als Epochensekunden in
/// der Reihenfolge Aenderung, Zugriff, Erzeugung. KRK setzt die ersten zwei
/// Kennzeichenbits, schreibt also acht Byte Zeit hinter das Kennzeichen.
pub(super) const FELD_ERWEITERTE_ZEIT: u16 = 0x5455;

/// Kennung des alten Info-ZIP-Unix-Feldes, das `ditto(1)` liest.
///
/// Rumpf in der kurzen Form: Zugriffszeit, dann Aenderungszeit, je vier Byte
/// Epochensekunden. Die lange Form haengt Benutzer- und Gruppenkennung an; KRK
/// schreibt sie nicht, denn `ditto` setzt sie beim Auspacken um, und dieses
/// Vorhaben packt Zeitstempel und keine Eigentumsverhaeltnisse.
///
/// **Diese Kennung ist der einzige Grund fuer das Merkmal `unreserved`** in der
/// Wurzel-`Cargo.toml`; ohne es weist `add_extra_data` sie ab.
pub(super) const FELD_INFOZIP_UNIX: u16 = 0x5855;

/// Traegt das Aenderungsdatum der Quelle in die Wahl ein, dreifach.
///
/// Einmal als MS-DOS-Feld, das jeder Leser versteht und das keine Zone kennt,
/// und zweimal als Zusatzfeld in Epochensekunden. Warum es zwei Zusatzfelder
/// sein muessen, steht als Messtabelle in der Wurzel-`Cargo.toml`.
///
/// **Ein Zeitpunkt, den keines der Felder fasst, weist den Eintrag nicht ab.**
/// Er bekommt eine Zeile in der Abschlussliste und das Vorgabedatum des
/// Formats; ein Archiv ohne diese Datei waere die schlechtere Antwort als eine
/// Datei mit einem Datum, das der Nutzer als falsch erkennt.
#[must_use = "ohne die Wahl traegt der Eintrag kein Datum"]
fn zeit_uebernehmen(
    mut wahl: FullFileOptions<'static>,
    pfad: &Path,
    angaben: Option<&fs::Metadata>,
    steuerung: &mut Steuerung,
) -> FullFileOptions<'static> {
    let Some(geaendert) = angaben.and_then(|angaben| angaben.modified().ok()) else {
        steuerung.ueberspringen(
            pfad,
            "das Aenderungsdatum war nicht zu lesen; der Eintrag traegt das Vorgabedatum",
        );
        return wahl;
    };
    // Die Zugriffszeit ist die Zugabe und nicht der Gegenstand: fehlt sie, steht
    // das Aenderungsdatum an beiden Stellen, so wie `ditto(1)` es auch haelt.
    let gelesen = angaben
        .and_then(|angaben| angaben.accessed().ok())
        .unwrap_or(geaendert);

    match archivzeitpunkt(geaendert) {
        Some(zeitpunkt) => wahl = wahl.last_modified_time(zeitpunkt),
        None => steuerung.ueberspringen(
            pfad,
            "das Aenderungsdatum liegt ausserhalb dessen, was ein Zip-Eintrag fassen kann; \
             der Eintrag traegt das Vorgabedatum",
        ),
    }

    if let (Some(geaendert), Some(gelesen)) = (epochensekunden(geaendert), epochensekunden(gelesen))
    {
        let mut erweitert = Vec::with_capacity(9);
        // Bit 0: die Aenderungszeit steht da. Bit 1: die Zugriffszeit steht da.
        erweitert.push(0b0000_0011);
        erweitert.extend_from_slice(&geaendert.to_le_bytes());
        erweitert.extend_from_slice(&gelesen.to_le_bytes());

        let mut infozip = Vec::with_capacity(8);
        infozip.extend_from_slice(&gelesen.to_le_bytes());
        infozip.extend_from_slice(&geaendert.to_le_bytes());

        // Beide Felder gehen in den lokalen Kopf; die Kiste wiederholt ihn im
        // Hauptverzeichnis, ein zweiter Eintrag als "nur zentral" stuende dort
        // also doppelt.
        for (kennung, rumpf) in [
            (FELD_ERWEITERTE_ZEIT, erweitert),
            (FELD_INFOZIP_UNIX, infozip),
        ] {
            if let Err(fehler) = wahl.add_extra_data(kennung, rumpf, false) {
                steuerung.ueberspringen(
                    pfad,
                    format!("das Zeitfeld {kennung:#06x} kam nicht in den Eintrag: {fehler}"),
                );
            }
        }
    }

    wahl
}

/// Rechnet ein Aenderungsdatum in die MS-DOS-Zeitform des Zip-Formats um.
///
/// `None` heisst: der Zeitpunkt liegt ausserhalb von 1980 bis 2107, oder das
/// System konnte ihn nicht in einen Kalendertag uebersetzen. Die Sekunde faellt
/// dabei auf ein gerades Raster, denn das Format traegt fuer sie nur fuenf Bit.
fn archivzeitpunkt(zeitpunkt: SystemTime) -> Option<DateTime> {
    let ortszeit = crate::verzeichnis::sys::ortszeit(zeitpunkt)?;
    DateTime::from_date_and_time(
        u16::try_from(ortszeit.jahr).ok()?,
        ortszeit.monat,
        ortszeit.tag,
        ortszeit.stunde,
        ortszeit.minute,
        ortszeit.sekunde,
    )
    .ok()
}

/// Macht aus einem Zeitpunkt die Zahl der Sekunden seit 1970, wie die zwei
/// Zusatzfelder sie tragen.
///
/// `None` heisst: der Zeitpunkt liegt vor 1970 oder nach 2106 und passt damit
/// nicht in die vier Byte, die beide Felder dafuer vorsehen.
fn epochensekunden(zeitpunkt: SystemTime) -> Option<u32> {
    let seither = zeitpunkt.duration_since(UNIX_EPOCH).ok()?;
    u32::try_from(seither.as_secs()).ok()
}

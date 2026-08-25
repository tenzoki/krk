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

use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use zip::ZipWriter;
use zip::write::SimpleFileOptions;

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
    match gelesen.metadata() {
        Ok(angaben) if angaben.is_file() => {}
        Ok(_) => {
            steuerung.ueberspringen(quelle.pfad, "keine gewoehnliche Datei");
            return Packschritt::Weiter;
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Packschritt::Weiter;
        }
    }

    if let Err(fehler) = schreiber.start_file(name_im_archiv, dateiwahl(quelle.pfad)) {
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
    if let Err(fehler) = schreiber.add_directory(name_im_archiv, ordnerwahl(quelle.pfad)) {
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
/// `S_IFLNK`. **Gesetzt wird das Kennzeichen von `add_symlink` und nicht von
/// `unix_permissions`:** dessen Rumpf maskiert mit `& 0o777` und wirft die
/// oberen Modusbits fort, `0o120777` kaeme dort also als `0o777` an und das
/// Archiv truege eine gewoehnliche Datei mit dem Pfad als Inhalt. Die Wahl
/// nennt `0o120777` trotzdem, weil `add_symlink` die Rechte nur ergaenzt, wenn
/// keine dastehen.
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
    let wahl = SimpleFileOptions::default().unix_permissions(0o120777);
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

/// Die Wahl fuer einen Dateieintrag: verdichtet, mit den Rechten der Quelle.
fn dateiwahl(pfad: &Path) -> SimpleFileOptions {
    rechte_uebernehmen(SimpleFileOptions::default(), pfad, 0o644)
}

/// Die Wahl fuer einen Ordnereintrag.
///
/// Das Verfahren steht hier nicht: `add_directory` setzt es selbst auf
/// "gespeichert", denn ein Ordnereintrag hat keinen Inhalt, den zu verdichten
/// sich lohnte.
fn ordnerwahl(pfad: &Path) -> SimpleFileOptions {
    rechte_uebernehmen(SimpleFileOptions::default(), pfad, 0o755)
}

/// Uebernimmt die Rechte der Quelle in die Wahl, notfalls die genannte Vorgabe.
///
/// Ein Archiv, das die Rechte fortwirft, macht aus einem ausfuehrbaren Skript
/// beim Entpacken eine gewoehnliche Datei. Laesst sich die Quelle gerade nicht
/// befragen, steht die uebliche Vorgabe da; ein Eintrag ohne Rechte waere
/// schlechter als ein Eintrag mit den ueblichen.
fn rechte_uebernehmen(wahl: SimpleFileOptions, pfad: &Path, vorgabe: u32) -> SimpleFileOptions {
    use std::os::unix::fs::PermissionsExt;

    let rechte = fs::metadata(pfad).map_or(vorgabe, |angaben| angaben.permissions().mode());
    wahl.unix_permissions(rechte)
}

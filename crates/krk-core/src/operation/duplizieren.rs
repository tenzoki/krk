//! Duplizieren: eine gewoehnliche Datei entsteht unter einem anderen Namen ein
//! zweites Mal, im selben Ordner.
//!
//! ```text
//! eintrag_duplizieren ──> lstat: ist die Quelle eine gewoehnliche Datei
//!                     ──> je Name: name_pruefen
//!                                  kopieren::datei_uebertragen ──> copyfile(3), ausschliessend
//!                                     │
//!                                     ├─ angelegt ────────────> fertig
//!                                     ├─ der Name ist vergeben ─> steuerung.namen_erfragen
//!                                     │                             ├─ ein Name ──> naechster Versuch
//!                                     │                             └─ sonst ─────> Abbruch
//!                                     └─ jeder andere Fehler ──> uebersprungen, mit Grund
//! ```
//!
//! Jeder Versuch mit einem gueltigen Namen hat damit genau einen von drei
//! Ausgaengen: angelegt, vergeben und deshalb erneut gefragt, oder gescheitert
//! mit seinem Grund in der Abschlussliste.
//!
//! # Warum diese Art den Zielklaerer des Kopierens nicht ruft
//!
//! Das Kopieren fragt **vor** dem Schreiben, ob am Ziel etwas steht, und bietet
//! das Ersetzen an: `super::ziel_klaeren` liest die Konfliktregel, und sein
//! Zweig "ueberschreiben" nimmt den vorhandenen Eintrag ueber den Baumloescher
//! endgueltig weg. Fuer das Duplizieren waere dieser Zweig die eine Stelle, an
//! der es einen Eintrag des Nutzers entfernen koennte, und es soll keine haben.
//! Dieses Modul ruft deshalb weder den Zielklaerer noch den Baumloescher noch
//! den Papierkorb, und es benennt nichts um; die Probe
//! `das_duplizieren_kennt_keinen_weg_der_einen_eintrag_entfernt`
//! (`tests/baum.rs`) haelt das an den Codezeilen dieser Datei.
//!
//! Dazu kommt ein Defekt, den der Zielklaerer traegt und dieser Weg nicht
//! erbt: nach der Antwort "umbenennen in" prueft er den getippten Namen nicht
//! noch einmal
//! (`260825-1130_*_ein-selbst-getippter-name-im-konfliktblatt-kann-einen-belegten-treffen-und-wird-ohne-rueckfrage-ueberschrieben.md`).
//! Hier geht jeder Name, auch der zweite und der dritte, durch denselben
//! ausschliessenden Versuch.
//!
//! # `COPYFILE_EXCL` ist die eine Pruefung
//!
//! **Ob der gewuenschte Name frei ist, sagt dieses Modul nicht vorher.** Aus
//! der gelesenen Liste und aus einem `lstat` vor dem Anlegen ist die Frage
//! nicht entscheidbar: ein fremdes Programm kann den Namen dazwischen vergeben,
//! und ob zwei Schreibungen derselbe Eintrag sind, weiss allein der
//! Datentraeger. Eine Vorabpruefung waere die zweite Wahrheit ueber denselben
//! Ordner. Der Arbeitsfaden versucht stattdessen das ausschliessende Anlegen:
//! `sys::datei_kopieren` setzt `COPYFILE_EXCL` in beiden Uebertragungsarten,
//! und das Dateisystem antwortet mit `EEXIST`, das hier als
//! [`io::ErrorKind::AlreadyExists`] ankommt.
//!
//! Gemessen am 260930 unter macOS 15.8 auf APFS, in beiden
//! Uebertragungsarten: `EEXIST` fuer den eigenen Namen der Quelle, fuer eine
//! andere Gross- und Kleinschreibung, fuer eine vorhandene Datei, fuer einen
//! verwaisten symbolischen Verweis und fuer einen Ordner; Quelle und Ziel
//! danach bytegleich. Die Messung steht im Plan dieser Arbeit
//! (`260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md`,
//! Kopfzeile `**Decidability:**`); die Proben in `tests/operation.rs` fahren
//! davon nach, was nicht am Datentraeger haengt, also alles bis auf die andere
//! Gross- und Kleinschreibung.
//!
//! **Der unveraenderte Name der Quelle bekommt keinen Sonderfall.** Er ist der
//! haeufigste vergebene Name, und das Dateisystem beantwortet ihn wie jeden
//! anderen.
//!
//! **Auf einem Datentraeger, der Gross- und Kleinschreibung unterscheidet, ist
//! ein Name frei, der auf APFS in der Vorgabe vergeben waere.** Das ist
//! gewollt: der Datentraeger entscheidet, und genau deshalb steht hier keine
//! Pruefung am Text.
//!
//! # Warum die Konfliktregel des Auftrags hier nicht gilt
//!
//! Die Regel sagt, was mit einem vorhandenen Ziel geschieht, und sie ist fuer
//! einen Stapel gebaut: ersetzen, ueberspringen, von selbst umbenennen, je
//! Quelle oder "fuer alle weiteren". Das Duplizieren hat eine Quelle und
//! ersetzt nie. Gefragt wird deshalb ueber [`Steuerung::namen_erfragen`], das
//! die Regel nicht liest, und jede Antwort, die kein Name ist, beendet den
//! Vorgang, ohne etwas anzulegen. Ein Auftrag mit der Regel "ueberschreiben"
//! fragt genauso wie einer mit der Vorgabe.
//!
//! # Was eine gewoehnliche Datei ist, entscheidet ein `lstat` an dieser Stelle
//!
//! `super::typ_und_groesse` hat die Quelle schon einmal befragt, und seine
//! Antwort genuegt hier nicht: sein `Typ::Datei` ist das Auffangfach fuer
//! alles, was weder Ordner noch Verknuepfung ist, und traegt Roehre, Socket und
//! Geraetedatei mit. Die zweite Frage ist also eine andere und keine
//! Wiederholung. Wer sie ausliesse, reichte eine benannte Roehre an
//! `copyfile(3)`, und das wartete auf einen Schreiber.
//!
//! **Das Fenster, das bleibt.** Zwischen diesem `lstat` und `copyfile(3)` kann
//! ein fremdes Programm die Quelle durch eine Roehre ersetzen; dann wartet
//! `copyfile(3)`. Betroffen ist der Arbeitsfaden und nie der Hauptfaden, und
//! der Fall ist ein Wettlauf und kein gewoehnlicher Weg, dieselbe Lage wie
//! beim `File::open` des Kopierens auf einem eben selbst angelegten Pfad.
//! Schliessen liesse es sich allein mit einer Typfrage am offenen Deskriptor,
//! und `copyfile(3)` nimmt Pfade.

use std::fs;
use std::io;

use crate::sprache::{Text, text};
use crate::verzeichnis::sys::Uebertragungsart;

use super::fortschritt::Steuerung;
use super::kopieren::datei_uebertragen;
use super::umbenennen::name_pruefen;
use super::{Ablauf, Quelle, grund};

/// Dupliziert eine gewoehnliche Datei unter `neuer_name` in ihren eigenen
/// Ordner.
///
/// Die Reihenfolge ist die des Schaubilds im Modulkopf:
///
/// 1. `lstat` an der Quelle. Ein Fehler wird mit seinem Grund uebersprungen,
///    alles ausser einer gewoehnlichen Datei (Ordner, Verknuepfung, Roehre,
///    Socket, Geraetedatei) mit `Text::VorgangKeineGewoehnlicheDatei`, dem
///    Grund, den auch das Packen nennt.
///    Einer Verknuepfung wird dabei nicht gefolgt: auch eine Verknuepfung auf
///    eine Datei ist keine gewoehnliche Datei.
/// 2. Je Name, beginnend mit `neuer_name`: [`name_pruefen`], dann der
///    ausschliessende Versuch ueber [`datei_uebertragen`]. Ein Text, der kein
///    Name ist, wird mit dem Satz aus `Namensfehler::grund` uebersprungen. Ein
///    vergebener Name fragt ueber [`Steuerung::namen_erfragen`] nach einem
///    anderen; jeder andere Fehler wird mit seinem Grund uebersprungen.
///
/// **Die Schleife laeuft allein so oft, wie der Nutzer einen vergebenen Namen
/// nennt.** Jeder Durchgang endet oder fragt, und ohne eine Antwort mit einem
/// Namen endet sie mit [`Ablauf::Abgebrochen`]: ein ausgebliebener Name ist
/// der Abbruch des Vorgangs und keine ausgelassene Position, denn es gibt
/// keine zweite, mit der der Stapel weiterlaufen koennte.
///
/// Der Name reist als Name und nicht als Pfad: das Ziel ist der Pfad der Quelle
/// mit dem Namen an der letzten Stelle, und `name_pruefen` hat vorher jeden
/// Schraegstrich abgewiesen. "Im selben Ordner" folgt damit aus der Bauform.
pub(crate) fn eintrag_duplizieren(
    quelle: &Quelle<'_>,
    neuer_name: &str,
    art: Uebertragungsart,
    steuerung: &mut Steuerung,
) -> Ablauf {
    match fs::symlink_metadata(quelle.pfad) {
        Ok(angaben) if angaben.file_type().is_file() => {}
        Ok(_) => {
            steuerung.ueberspringen(quelle.pfad, text(Text::VorgangKeineGewoehnlicheDatei));
            return Ablauf::Weiter;
        }
        Err(fehler) => {
            steuerung.ueberspringen(quelle.pfad, grund(&fehler));
            return Ablauf::Weiter;
        }
    }

    let mut name = neuer_name.to_owned();
    loop {
        if let Err(fehler) = name_pruefen(&name) {
            steuerung.ueberspringen(quelle.pfad, fehler.grund());
            return Ablauf::Weiter;
        }
        let ziel = quelle.pfad.with_file_name(&name);
        match datei_uebertragen(quelle, &ziel, art, steuerung) {
            Ok(ablauf) => return ablauf,
            Err(fehler) if fehler.kind() == io::ErrorKind::AlreadyExists => {
                match steuerung.namen_erfragen(quelle.pfad, &ziel) {
                    Some(anderer) => name = anderer,
                    None => return Ablauf::Abgebrochen,
                }
            }
            Err(fehler) => {
                steuerung.ueberspringen(quelle.pfad, grund(&fehler));
                return Ablauf::Weiter;
            }
        }
    }
}

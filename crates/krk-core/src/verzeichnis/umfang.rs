//! Wie viele Eintraege haengen an dieser Auswahl? Gezaehlt bis zu einem Deckel
//! und nicht weiter (C3).
//!
//! Der sechste Ausloeser der lauten Rueckfrage fragt nach dem Umfang: raeumt
//! der Nutzer mehr als [`SCHWELLE`] Eintraege weg, wird die Rueckfrage laut.
//! Die Frage ist damit **nicht**, wie viele es sind, sondern ob es mehr als
//! [`SCHWELLE`] sind, und dieses Modul beantwortet genau die zweite. Eine
//! genaue Zahl ueber einem Ordner mit einer Million Eintraegen zu bilden waere
//! teuer und fuer den Ausloeser ohne Wert.
//!
//! ```text
//!  zaehlen(&[PathBuf])
//!    │
//!    ├─ je ausgewaehlter Eintrag: eins        ──> lstat: ist es ein Ordner?
//!    │                                              │ ja: vormerken
//!    │                                              └ nein: fertig
//!    ├─ je vorgemerkter Ordner: oeffnen, ganz lesen ──> Unterordner vormerken
//!    │
//!    └──> Umfang::{Genau(n), MehrAls(SCHWELLE), Unentschieden}
//! ```
//!
//! # Der Deckel und die Schranken, die daraus folgen
//!
//! Gezaehlt wird bis `SCHWELLE + 1`, und dann ist die Frage entschieden: mehr
//! als [`SCHWELLE`]. Die Zahl steht als Ausdruck da und nicht als zweite
//! Konstante, damit eine Aenderung an [`SCHWELLE`] den Deckel mitnimmt.
//!
//! **Jeder Abstieg kostet mindestens einen Zaehler**, denn der Unterordner
//! zaehlt selbst mit, bevor er vorgemerkt wird. Daraus folgen alle Schranken
//! dieses Moduls, und zwar ohne Naeherung:
//!
//! - **hoechstens `SCHWELLE + 1` geoeffnete Verzeichnisse** ueber den ganzen
//!   Lauf, also 26 Paare aus `open(2)` und `getattrlistbulk(2)`;
//! - **hoechstens `SCHWELLE + 1` vorgemerkte Pfade** auf dem Stapel, denn
//!   vorgemerkt wird nur, was schon gezaehlt ist;
//! - **genau ein offener Verzeichnisdeskriptor**, zu jedem Zeitpunkt.
//!
//! Die dritte Schranke ist die einzige, die nicht am Deckel haengt, und sie ist
//! die wichtigste; der Abschnitt darunter sagt, warum.
//!
//! # Ein Stapel von Pfaden und keine Rekursion
//!
//! **Die Bauform ist die des [`super::durchlauf`], und das ist kein
//! Geschmack.** Ein Ordner wird ganz gelesen, seine Unterordner wandern dabei
//! als **Pfad** auf einen Stapel, und erst wenn er zu Ende ist, faellt sein
//! [`Schwungleser`] und der naechste wird geoeffnet. Ein Abstieg, der den Leser
//! der uebergeordneten Ebene offen haelt, ist genau der Defekt `260815-0211`:
//! der Durchlauf hielt einen Deskriptor je Ebene, erzeugte damit seinen eigenen
//! `EMFILE` und legte ihn dann als Befund ueber einen fremden Ordner aus.
//!
//! **Der Deckel von 26 macht diesen Fehler seltener und nicht falsch.** Die
//! Deskriptortabelle teilt sich dieses Modul mit dem Editor, der Vorschau, den
//! Kopiervorgaengen und den Lesevorgaengen beider Dateilisten; ein aus dem
//! Finder gestartetes Buendel bekommt sie klein, und 26 gleichzeitig gehaltene
//! Deskriptoren sind darin keine Kleinigkeit. Mit dem Stapel von Pfaden ist
//! einer gehalten, gleich wie tief der Baum ist, und die Frage stellt sich
//! nicht.
//!
//! **Rekursion gibt es hier deshalb gar keine**, auch nicht die auf 26 Ebenen
//! begrenzte, die der Plan dieser Runde vorsah: der Stapel steht auf dem Haufen
//! und nicht auf dem Fadenstapel, und die Zahl der Ebenen ist damit ohne
//! Bedeutung fuer die Kosten.
//!
//! # Warum ueber [`Schwungleser`] und nicht ueber [`super::leser::lesen`]
//!
//! [`super::leser::lesen`] liest ein Verzeichnis **vollstaendig** in einen
//! `Vec<Eintrag>` und baut je Eintrag zwei Sortierschluessel ueber die
//! Kollation. Beides ist hier verkehrt: ein Ordner mit einer Million Eintraegen
//! hielte damit den Hauptfaden auf, gleich wie klein der Deckel ist, und die
//! Sortierschluessel braucht niemand, der bloss zaehlt. Ein Schwung ist
//! dagegen ein Systemaufruf ueber einen Puffer von 256 KB, und die Zaehlung
//! bricht nach dem ersten Schwung ab, in dem der Deckel faellt.
//!
//! Damit entsteht auch keine zweite Lesemechanik: [`Schwungleser`] ist dieselbe
//! Huelle, die der Verzeichnisleser und der Durchlauf schon benutzen.
//!
//! # Die Zaehlung laeuft auf dem Hauptfaden
//!
//! Und darf es, weil die Schranken oben feststehen: hoechstens 26 Paare aus
//! `open(2)` und `getattrlistbulk(2)`. Ein Arbeitsfaden mit Kanal und
//! Abbruchkennzeichen — die Bauart des [`super::durchlauf`] — waere fuer eine
//! Frage, die vor einer Rueckfrage einmal gestellt wird und deren Kosten
//! beschraenkt sind, die teurere Loesung und brauchte einen zweiten Zustand im
//! Fenstermodell.
//!
//! # Was eine Verknuepfung beitraegt, und woran das entschieden wird
//!
//! **Eins, und sie wird nicht verfolgt.** Sonst zaehlte ein Verweis auf den
//! Benutzerordner den halben Rechner mit, und der Nutzer raeumt beim Loeschen
//! einer Verknuepfung die Verknuepfung weg und nicht ihr Ziel.
//!
//! Entschieden wird das an zwei Stellen mit zwei Werkzeugen, und beide
//! antworten dasselbe:
//!
//! - **unterhalb eines Ordners** am [`Typ`] des gelesenen Eintrags.
//!   `getattrlistbulk(2)` liefert `VLNK` fuer die Verknuepfung selbst und folgt
//!   ihr nicht, [`Typ::Ordner`] steht hier also fuer einen echten Ordner.
//! - **an der obersten Ebene** ueber `symlink_metadata`, also `lstat(2)`. Die
//!   Auswahl traegt die Pfade und die Zahl der Ordner, aber nicht den Typ je
//!   Eintrag; gefragt werden muss also hier, und `metadata` waere die falsche
//!   Frage, weil es der Verknuepfung folgt.
//!
//! `lstat(2)` ist dabei keine Rueckkehr zu der Pfadpruefung, die der Defekt
//! `260809-1652` abgeschafft hat: jene fragte nach dem Typ, **um danach zu
//! oeffnen**, und blockierte an einer benannten Roehre. Hier wird nach dem
//! Oeffnen nichts anderes entschieden als der Abstieg, und ein Eintrag, der
//! sich nicht oeffnen laesst, ist gezaehlt und fertig.
//!
//! # Was ein Fehlschlag bedeutet, und die eine Regel dahinter
//!
//! **Ein Mangel an Deskriptoren macht die Zaehlung [`Umfang::Unentschieden`],
//! jeder andere Fehlschlag beendet das Lesen dieses einen Ordners.** Das ist
//! dieselbe Unterscheidung, die [`ist_deskriptormangel`] fuer den
//! [`super::durchlauf`] traegt, und der Grund ist derselbe: `EMFILE` und
//! `ENFILE` sagen nichts ueber den Pfad, sondern ueber die Deskriptortabelle
//! des Prozesses. Aus ihnen eine Zahl abzuleiten hiesse, einen Zustand der
//! eigenen Sitzung als Aussage ueber ein Loeschziel auszugeben — und weil
//! „unentschieden gilt als laut" ([`super::Loeschzielbefund`]), kostet die
//! ehrliche Antwort hier nichts als eine lautere Rueckfrage.
//!
//! Die Regel gilt an beiden Stellen, an denen gelesen wird, beim Oeffnen und
//! beim naechsten Schwung. `lstat(2)` an der obersten Ebene braucht keinen
//! Deskriptor und kann den Mangel deshalb nicht melden; ein Fehlschlag dort
//! spricht immer ueber den Pfad, der Eintrag zaehlt eins, und abgestiegen wird
//! nicht.
//!
//! **Der Preis des zweiten Halbsatzes ist benannt: ein Ordner, der sich nicht
//! oeffnen laesst, zaehlt eins statt seines Inhalts.** Die Zahl kann damit zu
//! klein sein, und die Rueckfrage bliebe ruhig, wo sie laut sein muesste. Der
//! Fehlbetrag ist auf den ungelesenen Rest dieses einen Ordners beschraenkt,
//! und die Alternative waere schlechter: ein einziger fremder Unterordner —
//! etwa ein Ordner eines anderen Nutzers — machte jede Rueckfrage darueber
//! unentschieden und damit laut, und die laute Form verlore genau die
//! Unterscheidungskraft, um die es bei ihr geht.
//!
//! # Wer sie ruft
//!
//! Genau einer, und er steht seit dem elften Schritt derselben Runde da:
//! `Anwendungsdelegierter::loeschtexte` in `krk-ui` beschafft die Tatsachen
//! fuer die Ausloesertafel, und die Zaehlung geschieht dabei einmal je
//! Loeschbefehl. Der Rumpf, der `loeschtexte` ruft, ist
//! `Anwendungsdelegierter::loeschen_nach_rueckfrage`, und er ruft es im
//! **vierten** Zweig seiner Stufenregel. Die Zaehlung faellt deshalb erst an,
//! wenn die beiden billigen Stufen jenes Rumpfes durch sind und das Blatt
//! wirklich erscheint; ein Befehl, den ein laufender Vorgang oder eine leere
//! Auswahl anhaelt, oeffnet hier kein Verzeichnis. `dead_code` traf das Modul auch vorher nicht, denn `krk-core`
//! ist eine Bibliothek und alles hier ist von ihrer Wurzel aus erreichbar; eine
//! Ausnahme nach dem Vorbild von `krk-ui/src/kommandos/rueckschritt.rs` brauchte
//! es nie.
//!
//! Die bindende Grundlage ist
//! `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.

use std::path::PathBuf;

use super::eintrag::Typ;
use super::sys::{Schwungleser, ist_deskriptormangel};

/// Ab wie vielen Eintraegen der Umfang ein Warngrund ist.
///
/// Mehr als diese Zahl macht die Rueckfrage laut; genau diese Zahl noch nicht.
/// Der Wert steht hier und nicht in `krk-ui`, weil er zur Frage gehoert und
/// nicht zur Anzeige: [`zaehlen`] hoert bei `SCHWELLE + 1` auf, und ein
/// Aufrufer, der eine andere Schwelle meinte, bekaeme eine Zahl, die seine
/// Frage nicht beantwortet.
pub const SCHWELLE: u32 = 25;

/// Bis hierher wird gezaehlt, und nicht weiter.
///
/// Ein Eintrag mehr als [`SCHWELLE`], denn genau das ist die Frage: „mehr als
/// [`SCHWELLE`]?" ist mit dem Eintrag Nummer `SCHWELLE + 1` entschieden. Die
/// Zahl steht als Ausdruck ueber [`SCHWELLE`] da und nicht als eigene
/// Konstante, damit sie nicht getrennt von ihr altern kann.
const DECKEL: u32 = SCHWELLE + 1;

/// Wie viele Eintraege an einer Auswahl haengen — genau, oder gedeckelt.
///
/// **Drei Werte, ohne Auffangzweig bei den Aufrufern.** `Genau` und `MehrAls`
/// sind kein Wahrheitswert mit Zahl daneben: die Zahl in `Genau` ist die
/// Auskunft, die Zahl in `MehrAls` ist die ueberschrittene Schwelle, und beide
/// zusammenzuziehen liesse einen Aufrufer die eine fuer die andere lesen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Umfang {
    /// So viele Eintraege, abgezaehlt. Die Zahl liegt bei hoechstens
    /// [`SCHWELLE`], denn darueber wird nicht weitergezaehlt.
    Genau(u32),
    /// Mehr als diese Zahl, und die Zahl ist [`SCHWELLE`]. **Nicht die Zahl der
    /// gezaehlten Eintraege**: gezaehlt wurde einer mehr, und wie viele es
    /// wirklich sind, ist nicht ermittelt und war nicht gefragt.
    MehrAls(u32),
    /// Die Zaehlung ist nicht dazu gekommen: dem Prozess fehlte ein
    /// Verzeichnisdeskriptor. Das ist keine Aussage ueber die Auswahl, sondern
    /// eine ueber KRKs Kenntnis von ihr, und sie gilt als warnwuerdig — siehe
    /// [`super::Loeschzielbefund`].
    Unentschieden,
}

/// Zaehlt die Auswahl und alles darunter, bis zum Deckel.
///
/// Gezaehlt wird **jeder ausgewaehlte Eintrag als eins** und jeder Eintrag
/// unterhalb eines ausgewaehlten Ordners. Ein ausgewaehlter Ordner mit vier
/// Dateien darin sind also fuenf. Eine leere Auswahl ist `Genau(0)`; ein
/// ausgewaehlter Pfad, den es nicht mehr gibt, zaehlt eins, weil die Auswahl
/// ihn fuehrt und der Loeschauftrag ihn mitnehmen wuerde.
///
/// Die Auswahl kommt aus **einem** Dateifenster und traegt damit nur
/// Geschwister; ein ausgewaehlter Eintrag kann nicht unter einem anderen liegen,
/// und doppelt gezaehlt wird nichts. Eine Pruefung darauf steht hier nicht, weil
/// sie einen Fall abwehrte, den kein Aufrufer herstellen kann.
///
/// Die Kosten sind beschraenkt: hoechstens `SCHWELLE + 1` geoeffnete
/// Verzeichnisse, hoechstens ein offener Deskriptor zugleich, keine Rekursion.
/// Die Ableitung steht im Modulkopf.
///
/// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: der Wert ist
/// der einzige Ertrag des Aufrufs, und ohne ihn faellt der sechste Ausloeser der
/// lauten Rueckfrage aus, ohne dass irgendwo etwas fehlte.
#[must_use = "der Umfang ist der einzige Ertrag des Aufrufs; fallengelassen faellt der Ausloeser aus"]
pub fn zaehlen(auswahl: &[PathBuf]) -> Umfang {
    let mut zaehler: u32 = 0;
    // Die vorgemerkten Ordner, als Pfad und nicht als offener Leser. Die
    // Begruendung steht im Modulkopf unter „Ein Stapel von Pfaden und keine
    // Rekursion"; der Stapel haelt hoechstens `DECKEL` Pfade, weil nur
    // vorgemerkt wird, was schon gezaehlt ist.
    let mut offen: Vec<PathBuf> = Vec::new();

    // Erste Ebene: die Auswahl selbst. Jeder Eintrag zaehlt eins, gleich was er
    // ist, und nur ein echter Ordner wird vorgemerkt.
    for pfad in auswahl {
        zaehler += 1;
        if zaehler >= DECKEL {
            return Umfang::MehrAls(SCHWELLE);
        }
        // `symlink_metadata` und nicht `metadata`: eine Verknuepfung auf einen
        // Ordner ist hier keiner. Der Modulkopf sagt, warum die Frage an dieser
        // Stelle ueberhaupt gestellt werden muss.
        match std::fs::symlink_metadata(pfad) {
            // Der einzige Eingang zum Abstieg.
            Ok(angaben) if angaben.file_type().is_dir() => offen.push(pfad.clone()),
            // Eine Datei, eine Verknuepfung, eine Roehre, ein Socket: gezaehlt
            // und fertig.
            Ok(_) => {}
            // Ein Fehlschlag am Namen spricht ueber den Pfad und nicht ueber
            // den Vorrat an Deskriptoren, denn `lstat(2)` braucht keinen.
            Err(_) => {}
        }
    }

    while let Some(pfad) = offen.pop() {
        // Hier faellt die Unterscheidung aus dem Modulkopf: was sich aus einem
        // Grund am Pfad nicht oeffnen laesst, hat mit seiner eigenen Eins
        // beigetragen und ist fertig — was sich mangels Deskriptor nicht
        // oeffnen laesst, laesst die ganze Zaehlung unentschieden.
        let mut leser = match Schwungleser::oeffnen(&pfad) {
            Ok(leser) => leser,
            Err(fehler) if ist_deskriptormangel(&fehler) => return Umfang::Unentschieden,
            Err(_) => continue,
        };

        loop {
            let geliefert = match leser.naechster_schwung(|roh| {
                // Der Deckel wirkt auch innerhalb eines Schwungs: der Zaehler
                // steigt nie ueber `DECKEL`, und vorgemerkt wird ab dort nichts
                // mehr. Abbrechen laesst sich ein laufender Schwung nicht, und
                // er muss es nicht — er ist ein Systemaufruf und hoechstens
                // 256 KB gross.
                if zaehler >= DECKEL {
                    return;
                }
                zaehler += 1;
                // Die Fallunterscheidung ueber den Typ ist vollstaendig und hat
                // keinen Auffangzweig; eine vierte Art haelt den Bau an.
                match roh.typ {
                    Typ::Ordner => offen.push(pfad.join(&*roh.name)),
                    // Beide zaehlen eins und fuehren nirgendwohin: eine Datei
                    // hat nichts darunter, einer Verknuepfung wird nicht
                    // gefolgt.
                    Typ::Datei | Typ::Verknuepfung => {}
                }
            }) {
                Ok(geliefert) => geliefert,
                Err(fehler) if ist_deskriptormangel(&fehler) => return Umfang::Unentschieden,
                // Ein Fehler mitten im Ordner sagt dasselbe wie einer beim
                // Oeffnen: von hier ist nichts mehr zu holen.
                Err(_) => break,
            };

            // Erst der Deckel, dann das Ende des Ordners: faellt der Deckel im
            // letzten Schwung des letzten Ordners, ist die Antwort trotzdem
            // `MehrAls` und nicht `Genau(DECKEL)`.
            if zaehler >= DECKEL {
                return Umfang::MehrAls(SCHWELLE);
            }
            if geliefert == 0 {
                break;
            }
        }
        // `leser` faellt hier, und mit ihm der eine offene Deskriptor. Erst
        // danach wird der naechste vorgemerkte Ordner geoeffnet.
    }

    Umfang::Genau(zaehler)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Deckelzahl ist einer mehr als die Schwelle, und beide altern
    /// zusammen.
    ///
    /// Eine Probe ueber zwei Konstanten sieht muessig aus und ist es nicht: sie
    /// haelt fest, dass der Deckel als Ausdruck ueber [`SCHWELLE`] dasteht.
    /// Schreibt jemand die 26 aus, weil sie kuerzer ist, und aendert spaeter die
    /// Schwelle, zaehlte [`zaehlen`] gegen eine Zahl, die niemand mehr meint.
    #[test]
    fn der_deckel_haengt_an_der_schwelle() {
        assert_eq!(
            DECKEL,
            SCHWELLE + 1,
            "der Deckel ist nicht mehr einer mehr als die Schwelle"
        );
    }

    /// Eine leere Auswahl ist null Eintraege und kein Sonderfall.
    ///
    /// Die Prueflinge mit echten Baeumen stehen in
    /// `crates/krk-core/tests/umfang.rs`, weil sie den `Pruefordner` brauchen
    /// und der unter `tests/gemeinsam/` liegt. Diese eine Probe braucht ihn
    /// nicht.
    #[test]
    fn eine_leere_auswahl_ist_genau_null() {
        assert_eq!(zaehlen(&[]), Umfang::Genau(0));
    }
}

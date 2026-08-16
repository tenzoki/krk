//! Die eine Antwort auf „traegt diese Datei die getippte Zeichenfolge?"
//!
//! ```text
//! Pfad, Filtertext, Grenze
//!         │
//!         v
//! text::datei::bis_zur_grenze_lesen
//!         │
//!         ├── Ok(bytes) ──> String::from_utf8 ─ gelingt ─> filter::traegt_die_folge
//!         │                        │                         │ ja  ─> Traegt
//!         │                        │                         └ nein ─> TraegtNicht
//!         │                        └ misslingt ────────────────────────> TraegtNicht
//!         ├── Err(ZuGross) ───────────────────────────────────────────> ZuGross
//!         ├── Err(KeineDatei) ────────────────────────────────────────> TraegtNicht
//!         ├── Err(Fehler) ────────────────────────────────────────────> TraegtNicht
//!         └── Err(Deskriptormangel) ──────────────────────────────────> Unentschieden
//! ```
//!
//! Das Bild ist die ganze Datei. Sie fuegt dem Leseweg nichts hinzu, was sie
//! nicht abbildet: [`super::filter::traegt_die_folge`] ist derselbe Vergleich,
//! der schon ueber Namen entscheidet, und
//! [`crate::text::datei::bis_zur_grenze_lesen`] ist dieselbe Huelle, durch die
//! auch die Vorschau liest. Der dritte Rufer des Vergleichs entsteht hier und
//! nicht in [`super::durchlauf`], weil „lies eine Datei und vergleiche ihren
//! Text" eine andere Aufgabe ist als „schreite ein Verzeichnis ab".
//!
//! # Die Datei wird ganz gelesen und nicht streifenweise
//!
//! Naheliegend waere, in Streifen zu lesen und abzubrechen, sobald die Folge
//! gefunden ist. **Es geht nicht**, und der Grund liegt nicht bei der Suche,
//! sondern bei der Typfrage: *ist das ueberhaupt Text?* beantwortet dieser Baum
//! an genau einer Stelle, naemlich mit `String::from_utf8` ueber die gelesenen
//! Bytes. Eine Endungsliste schliesst [`crate::text::datei`] ausdruecklich aus,
//! und sie waere auch die falsche Antwort, weil eine Endung nichts ueber den
//! Inhalt aussagt.
//!
//! Streifenweise muesste die Typfrage also **je Streifen** beantwortet werden.
//! Eine Datei, die erst bei Byte 900.000 ungueltiges UTF-8 traegt, haette aus
//! ihren ersten Streifen laengst einen Treffer gemeldet — und sie soll gar
//! nicht stehen. Die Streifen aenderten damit nicht nur die Suche, sondern die
//! Frage, welche Datei KRK als Text annimmt, und das ist die eine Frage, die im
//! Baum nur einmal beantwortet wird.
//!
//! Die Groessengrenze traegt den Preis dafuer: gelesen wird hoechstens, was der
//! Aufrufer zulaesst, und was darueber liegt, wird gar nicht erst geholt.
//!
//! # Kein Abbruchkennzeichen in dieser Datei
//!
//! Sie beantwortet eine Frage ueber **eine** Datei und weiss nichts von Faeden.
//! Der Abbruch steht beim [`super::durchlauf`], und eine gelesene Datei ist
//! dort die kleinste nicht unterbrochene Einheit. Ein zweites Kennzeichen hier
//! waere ein zweiter Ort fuer dieselbe Zusage.
//!
//! # Die Grenze reist als Argument
//!
//! `krk-core` kennt die 1 MB der Vorschau nicht und soll sie nicht kennen. Sie
//! wohnt in `krk-ui` und kommt von dort herein, wie bei jedem Rufer von
//! [`crate::text::datei::bis_zur_grenze_lesen`].
//!
//! # Die Kistenrichtung, ausgeschrieben
//!
//! Dieses Modul haengt an [`crate::text::datei`], und `text::datei` haengt
//! seinerseits an [`super::sys`] fuer die eine Tuer. Auf Modulebene laeuft die
//! Abhaengigkeit damit in beide Richtungen zwischen `text` und `verzeichnis`,
//! auf Dateiebene nicht: `sys` fragt niemanden, und diese Datei wird von `sys`
//! nicht gefragt. Der Zuschnitt ist gewollt — die eine Tuer und die eine
//! begrenzte Lesehuelle stehen je einmal, und wer sie braucht, ruft sie dort,
//! wo sie stehen.

use std::path::Path;

use crate::text::datei::{Lesehindernis, bis_zur_grenze_lesen};

use super::filter::traegt_die_folge;

/// Was das Lesen einer Datei ueber den Filtertext ergeben hat.
///
/// **Vier Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig**, in
/// Entsprechung zu den vier Werten von [`Lesehindernis`]. Ein fuenfter dort
/// haelt jede Uebersetzung hier an, und das ist der Zweck der fehlenden
/// Auffangzweige.
///
/// **Die drei Ausgaenge sind nicht zwei.** [`ZuGross`](Self::ZuGross) und
/// [`Unentschieden`](Self::Unentschieden) sind kein
/// [`TraegtNicht`](Self::TraegtNicht), und beide Male aus demselben Grund: sie
/// sagen etwas ueber den **Lauf** und nichts ueber die Datei. Ueber einer
/// Groessengrenze wurde gar nicht gelesen, und bei `EMFILE` oder `ENFILE` ist
/// dem Prozess der Deskriptor ausgegangen. Wer die drei zusammenzoege,
/// entschiede negativ, wo nichts entschieden ist — derselbe Fehlgriff, den der
/// Durchlauf ueber den Unterbaum seit der Runde 10 vermeidet.
///
/// **Ungueltiges UTF-8 ist dagegen sehr wohl ein
/// [`TraegtNicht`](Self::TraegtNicht)**, und das ist kein Widerspruch zum
/// Absatz darueber: die Bytes lagen vor, die Frage war entscheidbar, und die
/// Antwort lautet, dass diese Datei kein Text ist. Ein eigener Wert dafuer
/// haette keinen Frager — die Dateiliste zeigt eine Datei, die kein Text ist,
/// nicht anders an als eine, deren Text die Folge nicht traegt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inhaltsbefund {
    /// Der gelesene Text traegt die Folge.
    Traegt,
    /// Der gelesene Text traegt sie nicht, oder die Datei ist kein Text, oder
    /// sie liess sich nicht lesen. Die Zeile faellt weg.
    TraegtNicht,
    /// Ueber der uebergebenen Grenze, also ungelesen. Kein Befund ueber die
    /// Datei, sondern eine Zahl fuer die Statuszeile.
    ZuGross,
    /// Der Vorrat an Deskriptoren ist erschoepft. Eine Lage des Prozesses und
    /// kein Befund ueber die Datei.
    Unentschieden,
}

/// Traegt der Inhalt hinter `pfad` die Folge `filter_klein`?
///
/// `filter_klein` ist **bereits kleingeschrieben**, wie bei
/// [`traegt_die_folge`]; kleingeschrieben wird hier der gelesene Text. Das ist
/// dieselbe Asymmetrie wie beim Namen und aus demselben Grund: der Filtertext
/// wird einmal je Suche umgeschrieben, der Gegenstand einmal je Vergleich.
///
/// `grenze` ist die groesste Zahl Bytes, die gelesen werden darf. Eine Datei
/// darueber wird nicht gelesen und liefert [`Inhaltsbefund::ZuGross`]; sie gilt
/// ausdruecklich **nicht** als Nichttreffer.
///
/// **Dieselbe Folge gibt am Namen und am Inhalt dieselbe Antwort**, weil beide
/// Wege durch [`traegt_die_folge`] laufen: Teilzeichenfolge an jeder Stelle,
/// ohne Ruecksicht auf die Schreibung, ohne Faltung von Umlauten und Akzenten.
/// Zwei Fassungen davon hiessen, dass ein Nutzer zwei Regeln lernen muesste,
/// ohne dass ihm jemand die zweite gesagt haette.
///
/// `#[must_use]`, weil der Aufruf ausser dem Lesen nichts tut: wer den Befund
/// fallen laesst, hat eine Datei umsonst gelesen, und still.
#[must_use]
pub fn traegt_der_inhalt(pfad: &Path, filter_klein: &str, grenze: u64) -> Inhaltsbefund {
    match bis_zur_grenze_lesen(pfad, grenze) {
        Ok(bytes) => match String::from_utf8(bytes) {
            // Gueltiges UTF-8: die Datei ist Text, und der eine Vergleich
            // entscheidet.
            Ok(text) => {
                if traegt_die_folge(&text, filter_klein) {
                    Inhaltsbefund::Traegt
                } else {
                    Inhaltsbefund::TraegtNicht
                }
            }
            // Kein gueltiges UTF-8: die Datei ist kein Text, und damit ist
            // entschieden, dass ihre Zeile nicht steht.
            Err(_) => Inhaltsbefund::TraegtNicht,
        },
        Err(Lesehindernis::ZuGross) => Inhaltsbefund::ZuGross,
        Err(Lesehindernis::KeineDatei | Lesehindernis::Fehler) => Inhaltsbefund::TraegtNicht,
        Err(Lesehindernis::Deskriptormangel) => Inhaltsbefund::Unentschieden,
    }
}

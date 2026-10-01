//! Der Wortlaut der Rueckfrage vor dem Zuruecksetzen auf Werkseinstellungen
//! (C1.4 und C1.5 des Spec
//! `260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md`).
//!
//! **Keine Zeile AppKit**, wie jedes Modul dieses Verzeichnisses. Das Blatt ist
//! dasselbe wie vor dem Raeumen in den Papierkorb,
//! `appkit::blaetter::loeschbestaetigung`; hier stehen allein die zwei Texte,
//! die es zeigt. Den Ausgang und die Abbruchsaetze nach dem Vorgang formt der
//! Kern, `krk_core::ablage::werkszustand`.
//!
//! **Eine Aufzaehlung der Ortsfolgen gibt es nicht**, weil der Befehl den
//! Notizordner nie wechselt (Spec, Abschnitt „Änderung 260929“). Die Rueckfrage
//! sagt deshalb in jedem Fall, dass der Notizordner und seine Dateien bleiben,
//! und haengt allein an einer Frage: ob eine eigene `keymap.toml` steht.
//!
//! Jeder Wortlaut kommt aus der Sprachtabelle; die Schaltflaeche ist deshalb
//! eine Funktion und keine Konstante, denn eine Konstante stuende in der
//! Quellsprache, gleich was macOS gewaehlt hat.

use krk_core::sprache::{Text, satz, text};

/// Was die Rueckfrage sagen muss, erhoben vor ihr.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vorlage {
    /// Ob im Ablageordner eine eigene `keymap.toml` steht; dann gehen alle
    /// eigenen Tastenzuweisungen aus dem Betrieb, und die Rueckfrage sagt es.
    pub keymap_steht: bool,
}

/// Die Beschriftung der Schaltflaeche, die den Vorgang ausloest.
#[must_use]
pub fn schaltflaeche() -> &'static str {
    text(Text::WerksSchaltflaeche)
}

/// Frage und Erlaeuterung der Rueckfrage.
///
/// Die Frage nennt die drei Dateien; die Erlaeuterung sagt, dass die alten
/// Fassungen mit Zeitstempel beiseitegelegt und nicht geloescht werden, was
/// danach gilt, dass der Notizordner bleibt, und bei einer eigenen
/// `keymap.toml` in einem eigenen Absatz, dass die eigenen Zuweisungen aus dem
/// Betrieb gehen.
#[must_use]
pub fn rueckfrage(vorlage: &Vorlage) -> (String, String) {
    let frage = text(Text::WerksFrage).to_owned();
    // Der Satz zum Notizordner steht in jeder Rueckfrage, als eigener
    // Eintrag, damit die Probe ihn an der Tabelle halten kann.
    let mut erlaeuterung = satz(
        Text::WerksErlaeuterung,
        &[("notizordner", &text(Text::WerksNotizordnerBleibt))],
    );
    if vorlage.keymap_steht {
        erlaeuterung.push_str("\n\n");
        erlaeuterung.push_str(text(Text::WerksEigeneZuweisungen));
    }
    (frage, erlaeuterung)
}

#[cfg(test)]
mod tests {
    use krk_core::sprache::Sprache;

    use super::*;

    /// C1.4 und C1.5: fuer beide Werte von `keymap_steht` nennt die Rueckfrage
    /// die drei Dateien, das Beiseitelegen mit Zeitstempel und dass keine
    /// geloescht wird, in jedem Fall den Satz zum Notizordner, und den Satz zu
    /// den eigenen Zuweisungen genau bei einer eigenen `keymap.toml`.
    #[test]
    fn die_rueckfrage_nennt_jede_folge_die_eintritt() {
        for keymap_steht in [false, true] {
            let (frage, erlaeuterung) = rueckfrage(&Vorlage { keymap_steht });
            let ganz = format!("{frage}\n{erlaeuterung}");
            for name in ["readers.toml", "settings.toml", "keymap.toml"] {
                assert!(frage.contains(name), "die Frage nennt {name} nicht");
            }
            assert!(ganz.contains("mit angehängtem Zeitstempel beiseite"));
            assert!(ganz.contains("löscht keine davon"));
            assert!(ganz.contains(Sprache::De.text(Text::WerksNotizordnerBleibt)));
            assert!(
                !ganz.contains("~/krkhome"),
                "die Rueckfrage spricht von einem Ortswechsel"
            );
            assert_eq!(
                erlaeuterung.contains("eigenen Tastenzuweisungen"),
                keymap_steht,
                "der Satz zu den eigenen Zuweisungen steht nicht genau bei keymap_steht"
            );
        }
    }
}

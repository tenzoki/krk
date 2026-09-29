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

/// Was die Rueckfrage sagen muss, erhoben vor ihr.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vorlage {
    /// Ob im Ablageordner eine eigene `keymap.toml` steht; dann gehen alle
    /// eigenen Tastenzuweisungen aus dem Betrieb, und die Rueckfrage sagt es.
    pub keymap_steht: bool,
}

/// Die Beschriftung der Schaltflaeche, die den Vorgang ausloest.
pub const SCHALTFLAECHE: &str = "Zurücksetzen";

/// Der Satz, dass der Notizordner bleibt; er steht in jeder Rueckfrage.
const NOTIZORDNER_BLEIBT: &str = "Der Notizordner und alle Dateien darin bleiben unberührt.";

/// Frage und Erlaeuterung der Rueckfrage.
///
/// Die Frage nennt die drei Dateien; die Erlaeuterung sagt, dass die alten
/// Fassungen mit Zeitstempel beiseitegelegt und nicht geloescht werden, was
/// danach gilt, dass der Notizordner bleibt, und bei einer eigenen
/// `keymap.toml` in einem eigenen Absatz, dass die eigenen Zuweisungen aus dem
/// Betrieb gehen.
#[must_use]
pub fn rueckfrage(vorlage: &Vorlage) -> (String, String) {
    let frage = String::from(
        "readers.toml, settings.toml und keymap.toml auf Werkseinstellungen zurücksetzen?",
    );
    let mut erlaeuterung = format!(
        "Jede der drei Dateien, die im Ablageordner steht, legt KRK unter ihrem Namen mit \
         angehängtem Zeitstempel beiseite, etwa readers.toml.JJMMTT-HHMM, und löscht keine \
         davon. Danach stehen readers.toml und settings.toml so da, wie diese Fassung von KRK \
         sie mitbringt, nur behält settings.toml den eingestellten Notizordner; keymap.toml \
         fehlt, und es gilt die mitgelieferte Tastenbelegung. {NOTIZORDNER_BLEIBT} KRK liest \
         den neuen Stand sofort ein."
    );
    if vorlage.keymap_steht {
        erlaeuterung.push_str(
            "\n\nAlle eigenen Tastenzuweisungen aus keymap.toml gehen damit aus dem Betrieb; \
             sie liegen danach allein in der Sicherung.",
        );
    }
    (frage, erlaeuterung)
}

#[cfg(test)]
mod tests {
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
            assert!(ganz.contains(NOTIZORDNER_BLEIBT));
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

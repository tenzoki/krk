//! Das Blatt fuer einen unbekannten Rechnerschluessel (Schritt 8 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`,
//! Entscheidung 6).
//!
//! ```text
//!  Faden krk-verbinden: vorpruefen ──> unbekannt ──> scannen
//!  Hauptfaden: zeigen(Rechner, Port, je Schluessel Typ und SHA256:…)
//!     Abbrechen (Return, Esc) ──> Meldung „Schluessel unbekannt“
//!     Annehmen (Cmd+Return)   ──> Faden krk-verbinden: anhaengen, verbinden
//! ```
//!
//! **Das Blatt zeigt, was angehaengt wird, und nichts sonst.** Rechner,
//! Port und Fingerabdruecke kommen aus `krk_core::fern::rechnerschluessel::Gescannt`,
//! dessen Zeilen [`texte`] nicht sieht und nicht aendern kann; die
//! Fingerabdruecke hat `ssh-keygen` gerechnet. Was der Nutzer annimmt, sind
//! genau diese Schluessel, und `ssh` prueft danach gegen sie.
//!
//! # Vorbelegt ist Abbrechen
//!
//! Dieselbe Reihenfolge und dieselben Tasten wie die Rueckfrage vor dem
//! Raeumen ([`super::loeschbestaetigung`]): „Abbrechen“ steht vorn und traegt
//! die Eingabetaste, „Annehmen“ steht hinten auf Cmd+Return. Ein Schluessel,
//! den der Nutzer nicht verglichen hat, soll nicht mit einem reflexhaften
//! Return in `known_hosts` landen. Das Warnzeichen des Systems steht dabei,
//! weil die Annahme eine Entscheidung ueber Vertrauen ist. `esc` erreicht das
//! Blatt ueber den Befehl `abbrechen` und den Griff im Schlitz, wie jedes
//! Blatt dieser Huelle.
//!
//! **Eine unbekannte Antwort nimmt nichts an**: die Rueckfallstelle kommt aus
//! der [`Wirkung`] der Schaltflaechen ([`super::abbruchstelle`]), und die
//! Probe darunter liest nach, dass sie nicht die annehmende ist.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Eine einzige AppKit-Klasse, `NSWindow`, und die Datei reicht sie nur
//! weiter; sie steht seit macOS 10.0 zur Verfuegung. `MainThreadMarker`
//! gehoert `objc2` und nicht AppKit. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`), und nichts hier ist nach macOS 15 hinzugekommen;
//! `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und die Nennung ist
//! die Gegenmassnahme. Alles, was `NSAlert` betrifft, steht im Kopf von
//! [`Blatt`].

use std::path::Path;

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use krk_core::fern::rechnerschluessel::Gescannt;
use krk_core::sprache::{Text, satz, text};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Die Stelle der Schaltflaeche, die die Schluessel annimmt.
const ANNEHMENDE_STELLE: usize = 1;

/// Die beiden Schaltflaechen, in bindender Reihenfolge: „Abbrechen“ vorn
/// auf der Eingabetaste, „Annehmen“ hinten auf Cmd+Return.
///
/// `pub(super)` aus demselben Grund wie bei
/// [`super::loeschbestaetigung::schaltflaechen`].
#[must_use]
pub(super) fn schaltflaechen() -> [Schaltflaeche<'static>; 2] {
    [
        Schaltflaeche::neu(
            text(Text::BlattAbbrechen),
            Taste::Eingabe,
            Wirkung::Liegenlassen,
        ),
        Schaltflaeche::neu(
            text(Text::RechnerschluesselAnnehmen),
            Taste::EingabeMitBefehl,
            Wirkung::Ausfuehren,
        ),
    ]
}

/// Frage und Erlaeuterung des Blattes: Rechner und Port oben, darunter je
/// Schluessel eine Zeile mit Typ und Fingerabdruck und die Datei, an die
/// „Annehmen“ anhaengt. Rein und ohne AppKit pruefbar.
#[must_use]
pub fn texte(name: &str, gescannt: &Gescannt, datei: &Path) -> (String, String) {
    let port = gescannt.port().to_string();
    let frage = satz(
        Text::RechnerschluesselFrage,
        &[("rechner", &gescannt.rechner()), ("port", &port)],
    );
    let abdruecke: Vec<String> = gescannt
        .fingerabdruecke()
        .iter()
        .map(|abdruck| {
            satz(
                Text::RechnerschluesselAbdruck,
                &[("typ", &abdruck.typ), ("abdruck", &abdruck.abdruck)],
            )
        })
        .collect();
    let abdruecke = abdruecke.join("\n");
    let datei = datei.display().to_string();
    let erlaeuterung = satz(
        Text::RechnerschluesselErlaeuterung,
        &[
            ("name", &name),
            ("abdruecke", &abdruecke),
            ("datei", &datei),
        ],
    );
    (frage, erlaeuterung)
}

/// Zeigt das Blatt und meldet, ob der Nutzer die Schluessel angenommen hat.
///
/// `fertig` laeuft auf dem Hauptfaden und genau einmal; `false` heisst
/// abgebrochen, auch bei `esc` und bei einer unbekannten Antwort.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    erlaeuterung: &str,
    fertig: impl Fn(bool) + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(mtm, frage, &schaltflaechen());
    blatt.erlaeuterung_setzen(erlaeuterung);
    blatt.als_warnung();
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| {
        fertig(stelle == ANNEHMENDE_STELLE)
    })
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::{abbruchstelle, bestaetigungsstelle};

    use super::{ANNEHMENDE_STELLE, Taste, Wirkung, schaltflaechen, texte};

    /// Die Texte nennen Rechner, Port, jeden Fingerabdruck mit seinem Typ und
    /// die Datei, und kein Platzhalter bleibt stehen; in jeder Sprache, die
    /// `geltende()` in `cargo test` liefert, also Deutsch.
    ///
    /// Das `Gescannt` entsteht wie im Betrieb ueber `gescannt_aus` auf einem
    /// Faden mit Pfand; `ssh-keygen` rechnet den Abdruck, ohne Netz.
    #[test]
    fn die_texte_nennen_rechner_port_abdruecke_und_datei() {
        use std::path::{Path, PathBuf};

        use krk_core::fern::rechnerschluessel::{Sshkonfiguration, gescannt_aus};

        let datei = PathBuf::from("/pruef/punktssh/known_hosts");
        let konfiguration = Sshkonfiguration {
            hostname: "example.test".into(),
            port: 2222,
            hostkeyalias: None,
            dateien: vec![datei.clone()],
            eigene_datei: Some(datei.clone()),
        };
        let ausgabe = "[example.test]:2222 ssh-ed25519 \
             AAAAC3NzaC1lZDI1NTE5AAAAINCchDA8qQ402glvs5GmSdmMtjRAi1vwvNLCJKg3j3dn\n";
        let gescannt = krk_core::fern::faden("krk-probe-blatt".to_owned(), move |pfand| {
            gescannt_aus(pfand, &konfiguration, "[example.test]:2222", ausgabe)
        })
        .expect("ein Faden")
        .join()
        .expect("kein Absturz")
        .expect("ein Schluessel");
        let (frage, erlaeuterung) = texte("strato", &gescannt, Path::new(&datei));
        assert!(frage.contains("example.test"), "{frage}");
        assert!(frage.contains("2222"), "{frage}");
        for teil in [
            "strato",
            "ED25519",
            "SHA256:9LQx2fmiP9B1NG/QxKfQPvLAD5VQ4wpNq3wCleSOvg4",
            "/pruef/punktssh/known_hosts",
        ] {
            assert!(erlaeuterung.contains(teil), "{teil} fehlt: {erlaeuterung}");
        }
        assert!(!frage.contains('{') && !erlaeuterung.contains('{'));
    }

    /// Eine unbekannte Antwort und die Eingabetaste nehmen nichts an, und die
    /// annehmende Stelle zeigt auf die annehmende Schaltflaeche.
    #[test]
    fn abbrechen_ist_vorbelegt_und_annehmen_liegt_auf_cmd_return() {
        let schaltflaechen = schaltflaechen();
        assert_eq!(abbruchstelle(&schaltflaechen), 0);
        assert_ne!(abbruchstelle(&schaltflaechen), ANNEHMENDE_STELLE);
        assert_eq!(bestaetigungsstelle(&schaltflaechen), 0);
        assert_eq!(
            schaltflaechen[ANNEHMENDE_STELLE].wirkung,
            Wirkung::Ausfuehren
        );
        assert_eq!(
            schaltflaechen[ANNEHMENDE_STELLE].taste,
            Taste::EingabeMitBefehl
        );
    }
}

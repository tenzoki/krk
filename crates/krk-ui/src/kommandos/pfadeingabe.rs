//! Einen Pfad pruefen und sagen, wohin KRK geht (C2 und C10).
//!
//! **Die eine Stelle, die einen Pfad prueft.** Zwei Ausloeser benutzen sie:
//!
//! ```text
//! Shift+Cmd+G ──> Blatt am Fenster ──┐
//!                                    ├──> pruefen ──> Ergebnis ──> Dateifenster
//! Opt+Cmd+G ──> Zwischenablage ──────┘
//!               (Ziel::Pfad)
//! ```
//!
//! Der Unterschied ist allein, woher der Wert kommt, aus dem Eingabeblatt oder
//! aus der Zwischenablage. Ein zweiter Navigationsweg daneben waere die zweite
//! Wahrheit darueber, was KRK fuer einen gangbaren Pfad haelt, und die erste
//! Abweichung zwischen beiden faende keine Pruefung.
//!
//! [`Ergebnis`] ist ein gewoehnlicher Rust-Wert und keine Ausfuehrung: diese
//! Datei navigiert nicht, sie sagt der Ansicht, wohin. Angewandt wird das
//! Ergebnis an genau einer Stelle in `crate::appkit::tabelle`.

use std::path::{Path, PathBuf};

use krk_core::verzeichnis::aufwaerts;

/// Wohin das aktive Dateifenster nach der Pruefung geht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ergebnis {
    /// In diesen Ordner wechseln, danach die Auswahl auf diesen Eintrag setzen.
    Wechseln {
        /// Der Ordner, der gelesen wird.
        ordner: PathBuf,
        /// Der Name, auf den die Auswahl springt, sobald gelesen ist.
        auswahl: Option<String>,
    },
    /// Der Ordner steht schon da: allein die Auswahl setzen und den Eintrag ins
    /// Bild blaettern (C10).
    NurAuswahl {
        /// Der Name des Eintrags.
        name: String,
    },
    /// Nicht verwertbar. Der Text geht in die Statuszeile aus C1.
    Meldung(String),
}

/// Prueft einen Pfad und sagt, wohin das aktive Dateifenster geht.
///
/// `angezeigt` ist der Ordner, den das Dateifenster gerade zeigt. Er
/// entscheidet allein den Fall aus C10, dass die genannte Datei bereits vor dem
/// Nutzer liegt: dann wechselt KRK den Ordner nicht.
///
/// Der Pfad muss absolut sein. Diese Regel ist die von C2, und C10 erbt sie.
pub fn pruefen(pfad: &Path, angezeigt: &Path) -> Ergebnis {
    if !pfad.is_absolute() {
        return Ergebnis::Meldung(format!("{} ist kein absoluter Pfad", pfad.display()));
    }
    // `metadata` folgt einer Verknuepfung. Das ist hier richtig: eine
    // Verknuepfung auf einen Ordner ist als Ziel eines Sprungs derselbe Ordner.
    // In der Dateiliste meldet der Leser sie weiter als Verknuepfung; die
    // beiden Fragen sind verschieden.
    let angaben = match std::fs::metadata(pfad) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            return Ergebnis::Meldung(format!("{} gibt es nicht: {fehler}", pfad.display()));
        }
    };

    if angaben.is_dir() {
        // Das Leserecht jetzt pruefen und nicht dem Lesevorgang ueberlassen:
        // C2 verlangt eine Meldung fuer den nicht lesbaren Pfad, und ein
        // Ordnerwechsel in eine leere Liste waere die wortlose Variante.
        if let Err(fehler) = std::fs::read_dir(pfad) {
            return Ergebnis::Meldung(format!(
                "{} laesst sich nicht lesen: {fehler}",
                pfad.display()
            ));
        }
        return Ergebnis::Wechseln {
            ordner: pfad.to_path_buf(),
            auswahl: None,
        };
    }

    // Keine Ordner, also eine Datei: in ihren Ordner wechseln und die Auswahl
    // auf sie stellen (C10).
    let Some((ordner, name)) = aufwaerts(pfad) else {
        return Ergebnis::Meldung(format!("{} liegt in keinem Ordner", pfad.display()));
    };
    if gleicher_ordner(&ordner, angezeigt) {
        Ergebnis::NurAuswahl { name }
    } else {
        Ergebnis::Wechseln {
            ordner,
            auswahl: Some(name),
        }
    }
}

/// Ob zwei Pfade denselben Ordner benennen.
///
/// Verglichen wird der aufgeloeste Pfad: `/tmp` und `/private/tmp` sind
/// derselbe Ordner, und ein abschliessender Schraegstrich macht keinen
/// Unterschied. Laesst sich einer der beiden nicht aufloesen, bleibt der
/// woertliche Vergleich. Ein falsches "verschieden" kostet dann ein zweites
/// Lesen desselben Ordners und keinen falschen Sprung.
fn gleicher_ordner(links: &Path, rechts: &Path) -> bool {
    match (links.canonicalize(), rechts.canonicalize()) {
        (Ok(links), Ok(rechts)) => links == rechts,
        _ => links == rechts,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static ZAEHLER: AtomicU64 = AtomicU64::new(0);

    /// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
    struct Pruefordner {
        pfad: PathBuf,
    }

    impl Pruefordner {
        fn neu(zweck: &str) -> Self {
            let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let mut pfad = std::env::temp_dir();
            pfad.push(format!(
                "krk-pfadeingabe-{zweck}-{}-{laufnummer}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&pfad);
            fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
            Self { pfad }
        }
    }

    impl Drop for Pruefordner {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.pfad);
        }
    }

    #[test]
    fn ein_relativer_pfad_meldet_und_navigiert_nicht() {
        let ergebnis = pruefen(Path::new("Projekte"), Path::new("/"));
        let Ergebnis::Meldung(text) = ergebnis else {
            panic!("ein relativer Pfad fuehrte zu einer Navigation");
        };
        assert!(
            text.contains("absoluter Pfad"),
            "die Meldung sagt nicht warum: {text}"
        );
    }

    #[test]
    fn ein_nicht_vorhandener_pfad_meldet_und_navigiert_nicht() {
        let ordner = Pruefordner::neu("fehlt");
        let ergebnis = pruefen(&ordner.pfad.join("gibtsnicht"), &ordner.pfad);
        let Ergebnis::Meldung(text) = ergebnis else {
            panic!("ein fehlender Pfad fuehrte zu einer Navigation");
        };
        assert!(
            text.contains("gibtsnicht"),
            "die Meldung nennt den Pfad nicht: {text}"
        );
    }

    #[test]
    fn ein_ordner_fuehrt_zum_wechsel_ohne_auswahl() {
        let ordner = Pruefordner::neu("wechsel");
        let unten = ordner.pfad.join("unten");
        fs::create_dir(&unten).expect("Unterordner");

        assert_eq!(
            pruefen(&unten, &ordner.pfad),
            Ergebnis::Wechseln {
                ordner: unten,
                auswahl: None
            }
        );
    }

    #[test]
    fn eine_datei_woanders_fuehrt_zum_wechsel_mit_auswahl() {
        let ordner = Pruefordner::neu("datei-woanders");
        let unten = ordner.pfad.join("unten");
        fs::create_dir(&unten).expect("Unterordner");
        let datei = unten.join("idee.txt");
        fs::write(&datei, b"x").expect("Datei");

        assert_eq!(
            pruefen(&datei, &ordner.pfad),
            Ergebnis::Wechseln {
                ordner: unten,
                auswahl: Some("idee.txt".to_owned())
            }
        );
    }

    /// Der Fall aus C10, den das Abnahmekriterium einzeln nennt.
    #[test]
    fn eine_datei_im_angezeigten_ordner_wechselt_den_ordner_nicht() {
        let ordner = Pruefordner::neu("datei-hier");
        let datei = ordner.pfad.join("idee.txt");
        fs::write(&datei, b"x").expect("Datei");

        assert_eq!(
            pruefen(&datei, &ordner.pfad),
            Ergebnis::NurAuswahl {
                name: "idee.txt".to_owned()
            }
        );
    }

    /// Derselbe Ordner, anders geschrieben, ist derselbe Ordner.
    #[test]
    fn ein_abschliessender_schraegstrich_erzwingt_keinen_ordnerwechsel() {
        let ordner = Pruefordner::neu("schraegstrich");
        let datei = ordner.pfad.join("idee.txt");
        fs::write(&datei, b"x").expect("Datei");
        let mit_strich = format!("{}/", ordner.pfad.display());

        assert_eq!(
            pruefen(&datei, Path::new(&mit_strich)),
            Ergebnis::NurAuswahl {
                name: "idee.txt".to_owned()
            }
        );
    }

    #[test]
    fn ein_ordner_ohne_leserecht_meldet_und_navigiert_nicht() {
        use std::os::unix::fs::PermissionsExt;

        let ordner = Pruefordner::neu("kein-leserecht");
        let gesperrt = ordner.pfad.join("gesperrt");
        fs::create_dir(&gesperrt).expect("Unterordner");
        fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
            .expect("Rechte lassen sich setzen");

        let ergebnis = pruefen(&gesperrt, &ordner.pfad);

        // Aufraeumen, bevor die Probe fehlschlagen kann: sonst bleibt ein
        // Ordner liegen, den `remove_dir_all` nicht mehr betreten darf.
        let _ = fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o700));

        let Ergebnis::Meldung(text) = ergebnis else {
            panic!("ein Ordner ohne Leserecht fuehrte zu einer Navigation");
        };
        assert!(
            text.contains("nicht lesen"),
            "die Meldung sagt nicht warum: {text}"
        );
    }
}

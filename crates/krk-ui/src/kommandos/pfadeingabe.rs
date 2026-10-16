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
//!
//! # In einem fernen Tab
//!
//! Steht das Dateifenster auf einem Ordner auf einem Server, ist der Text ein
//! Pfad auf demselben Server, und [`fern_pruefen`] beantwortet ihn **rein
//! lexikalisch und ohne Systemaufruf** (Entscheidung 15 des Plans
//! `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`): absolut, oder
//! mit `~/` unter dem Anmeldeverzeichnis. Ob dort ein Ordner steht, sagt der
//! Lesevorgang im Tab und nicht diese Datei; eine Pruefung hier waere eine
//! Anfrage an den Server, und die darf der Hauptfaden nicht stellen. Zurueck zu
//! einem lokalen Ordner geht es ueber ein Lesezeichen, die Geraeteleiste, das
//! Angleichen von einem lokalen Fenster aus oder die Zwischenablage.
//!
//! **Die Zwischenablage traegt in einem fernen Tab einen Pfad auf diesem Mac**,
//! nie einen auf dem Server: was dort steht, kommt aus dem Finder, aus „Pfad
//! kopieren“ eines lokalen Fensters oder als Dateiverweis, und keiner dieser
//! Wege legt einen Serverpfad ab; die Pfadkopien sind am fernen Ort
//! ausgegraut. „Zur Zwischenablage springen“ prueft den Pfad deshalb mit
//! [`pruefen_fuer_fernen_tab`] wie lokal und stellt den Tab auf den lokalen
//! Ordner (Befund
//! `261017-0004-zur-zwischenablage-springen-liest-in-einem-fernen-tab-einen-lokalen-pfad-als-serverpfad`).
//! Die Pfadeingabe von Hand bleibt beim Pfad auf dem Server.

use std::path::{Path, PathBuf};

use krk_core::fern::Fernpfad;
use krk_core::sprache::{Text, satz};
use krk_core::verzeichnis::{aufwaerts, datenschutzsperre};

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
#[must_use]
pub fn pruefen(pfad: &Path, angezeigt: &Path) -> Ergebnis {
    pruefen_gegen(pfad, Some(angezeigt))
}

/// Prueft einen lokalen Pfad fuer einen Tab, der einen Ordner auf einem
/// Server zeigt: dieselbe Pruefung wie [`pruefen`], nur liegt keine Datei
/// schon vor dem Nutzer, und eine Datei fuehrt immer in ihren Ordner.
///
/// Gerufen wird sie von „Zur Zwischenablage springen“ in einem fernen Tab
/// (Modulkopf, „In einem fernen Tab“): der Pfad dort ist ein Pfad auf diesem
/// Mac und nie einer auf dem Server.
#[must_use]
pub fn pruefen_fuer_fernen_tab(pfad: &Path) -> Ergebnis {
    pruefen_gegen(pfad, None)
}

fn pruefen_gegen(pfad: &Path, angezeigt: Option<&Path>) -> Ergebnis {
    if !pfad.is_absolute() {
        return Ergebnis::Meldung(satz(Text::PfadNichtAbsolut, &[("pfad", &pfad.display())]));
    }
    // `metadata` folgt einer Verknuepfung. Das ist hier richtig: eine
    // Verknuepfung auf einen Ordner ist als Ziel eines Sprungs derselbe Ordner.
    // In der Dateiliste meldet der Leser sie weiter als Verknuepfung; die
    // beiden Fragen sind verschieden.
    let angaben = match std::fs::metadata(pfad) {
        Ok(angaben) => angaben,
        Err(fehler) => {
            return Ergebnis::Meldung(satz(
                Text::PfadGibtEsNicht,
                &[("pfad", &pfad.display()), ("fehler", &fehler)],
            ));
        }
    };

    if angaben.is_dir() {
        // Das Leserecht jetzt pruefen und nicht dem Lesevorgang ueberlassen:
        // C2 verlangt eine Meldung fuer den nicht lesbaren Pfad, und der
        // Pfadsprung soll gar nicht erst wechseln.
        //
        // **Der Doppelklick in der Dateiliste prueft hier nichts nach und ist
        // trotzdem nicht wortlos.** Er geht hinein, und die Auskunft kommt
        // einen Einzugstakt spaeter aus dem Lesevorgang selbst
        // (`crate::tabs::lesemeldungen_einziehen`, Zweig `Abschluss::Fehler`).
        // Beide Wege melden also; sie unterscheiden sich darin, ob vorher
        // gewechselt wird. Diese Pruefung hier ist der Preis dafuer, dass der
        // Pfadsprung nicht wechselt, und keine zweite Wahrheit ueber das
        // Leserecht (Nutzerentscheid vom 260907-1210,
        // `shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`).
        if let Err(fehler) = std::fs::read_dir(pfad) {
            return Ergebnis::Meldung(datenschutzsperre(pfad, &fehler).unwrap_or_else(|| {
                satz(
                    Text::PfadNichtLesbar,
                    &[("pfad", &pfad.display()), ("fehler", &fehler)],
                )
            }));
        }
        return Ergebnis::Wechseln {
            ordner: pfad.to_path_buf(),
            auswahl: None,
        };
    }

    // Keine Ordner, also eine Datei: in ihren Ordner wechseln und die Auswahl
    // auf sie stellen (C10).
    let Some((ordner, name)) = aufwaerts(pfad) else {
        return Ergebnis::Meldung(satz(Text::PfadInKeinemOrdner, &[("pfad", &pfad.display())]));
    };
    if angezeigt.is_some_and(|angezeigt| gleicher_ordner(&ordner, angezeigt)) {
        Ergebnis::NurAuswahl { name }
    } else {
        Ergebnis::Wechseln {
            ordner,
            auswahl: Some(name),
        }
    }
}

/// Wohin ein ferner Tab nach der Pruefung geht.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FernErgebnis {
    /// Diesen Pfad auf demselben Server lesen.
    Wechseln(Fernpfad),
    /// Nicht verwertbar. Der Text geht in die Statuszeile aus C1.
    Meldung(String),
}

/// Prueft einen Pfad fuer einen fernen Tab, rein lexikalisch.
///
/// `~` und `~/…` stehen fuer das Anmeldeverzeichnis, das der Server beim
/// Aufbau genannt hat; jeder andere Text muss absolut sein, die Regel aus C2.
/// `.` und `..` werden lexikalisch aufgeloest (`Fernpfad::aus_text`).
/// **Kein Systemaufruf und keine Anfrage**: die Rumpfprobe
/// `die_ferne_pfadeingabe_stellt_keinen_systemaufruf` haelt es.
#[must_use]
pub fn fern_pruefen(eingabe: &str, anmeldeverzeichnis: &Fernpfad) -> FernErgebnis {
    let unter_heim = if eingabe == "~" {
        Some("")
    } else {
        eingabe.strip_prefix("~/")
    };
    let text = match unter_heim {
        Some(rest) => format!("{anmeldeverzeichnis}/{rest}"),
        None => eingabe.to_owned(),
    };
    match Fernpfad::aus_text(&text) {
        Some(pfad) => FernErgebnis::Wechseln(pfad),
        None => FernErgebnis::Meldung(satz(Text::PfadNichtAbsolut, &[("pfad", &eingabe)])),
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

    use super::*;
    use crate::pruefordner::Pruefordner;

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
        let ergebnis = pruefen(&ordner.unter("gibtsnicht"), ordner.pfad());
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
        let unten = ordner.unter("unten");
        fs::create_dir(&unten).expect("Unterordner");

        assert_eq!(
            pruefen(&unten, ordner.pfad()),
            Ergebnis::Wechseln {
                ordner: unten,
                auswahl: None
            }
        );
    }

    #[test]
    fn eine_datei_woanders_fuehrt_zum_wechsel_mit_auswahl() {
        let ordner = Pruefordner::neu("datei-woanders");
        let unten = ordner.unter("unten");
        fs::create_dir(&unten).expect("Unterordner");
        let datei = unten.join("idee.txt");
        fs::write(&datei, b"x").expect("Datei");

        assert_eq!(
            pruefen(&datei, ordner.pfad()),
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
        let datei = ordner.unter("idee.txt");
        fs::write(&datei, b"x").expect("Datei");

        assert_eq!(
            pruefen(&datei, ordner.pfad()),
            Ergebnis::NurAuswahl {
                name: "idee.txt".to_owned()
            }
        );
    }

    /// Derselbe Ordner, anders geschrieben, ist derselbe Ordner.
    #[test]
    fn ein_abschliessender_schraegstrich_erzwingt_keinen_ordnerwechsel() {
        let ordner = Pruefordner::neu("schraegstrich");
        let datei = ordner.unter("idee.txt");
        fs::write(&datei, b"x").expect("Datei");
        let mit_strich = format!("{}/", ordner.pfad().display());

        assert_eq!(
            pruefen(&datei, Path::new(&mit_strich)),
            Ergebnis::NurAuswahl {
                name: "idee.txt".to_owned()
            }
        );
    }

    fn heim() -> Fernpfad {
        Fernpfad::aus_text("/home/kai").expect("absolut")
    }

    /// Ein ferner Pfad wird lexikalisch gedeutet: absolut, mit `~/` unter dem
    /// Anmeldeverzeichnis, `..` aufgeloest; ein relativer Text meldet.
    #[test]
    fn ein_ferner_pfad_wird_lexikalisch_gedeutet() {
        let wechsel =
            |text: &str| FernErgebnis::Wechseln(Fernpfad::aus_text(text).expect("absolut"));
        assert_eq!(fern_pruefen("/var/www", &heim()), wechsel("/var/www"));
        assert_eq!(
            fern_pruefen("/var/www/../log/", &heim()),
            wechsel("/var/log")
        );
        assert_eq!(
            fern_pruefen("~/htdocs", &heim()),
            wechsel("/home/kai/htdocs")
        );
        assert_eq!(fern_pruefen("~", &heim()), wechsel("/home/kai"));
        assert_eq!(fern_pruefen("~/", &heim()), wechsel("/home/kai"));
        let FernErgebnis::Meldung(text) = fern_pruefen("htdocs", &heim()) else {
            panic!("ein relativer ferner Pfad fuehrte zu einer Navigation");
        };
        assert!(
            text.contains("htdocs"),
            "die Meldung nennt den Pfad nicht: {text}"
        );
        assert!(
            matches!(fern_pruefen("~kai/x", &heim()), FernErgebnis::Meldung(_)),
            "`~konto` ist keine Schreibweise dieser Pruefung"
        );
    }

    /// Die ferne Pfadeingabe stellt keinen Systemaufruf (Schritt 3 des Plans
    /// der sftp-Ordner).
    ///
    /// Gelesen wird der Rumpf von [`fern_pruefen`] in dieser Datei und der
    /// ferne Zweig von `pfad_anspringen` in `appkit/tabelle.rs`, dem einen
    /// Anwender des Ergebnisses; keiner darf das Dateisystem fragen.
    ///
    /// **Was sie nicht sieht:** einen Systemaufruf in einer tiefer gerufenen
    /// Funktion. `Fernpfad::aus_text` steht im Kern und traegt die Zusage in
    /// seinem Modulkopf, und `Fernzugang::anmeldeverzeichnis` liest einen
    /// gemerkten Wert unter einem Mutex, den keine Anfrage haelt.
    #[test]
    fn die_ferne_pfadeingabe_stellt_keinen_systemaufruf() {
        let diese = include_str!("pfadeingabe.rs");
        let rumpf = diese
            .split_once(concat!("pub fn ", "fern_pruefen("))
            .and_then(|(_, rest)| rest.split_once("\n}\n"))
            .map(|(rumpf, _)| rumpf)
            .expect("fern_pruefen steht in dieser Datei");
        let tabelle = include_str!("../appkit/tabelle.rs");
        let zweig = tabelle
            .split_once(concat!("fn ", "fern_pfad_anspringen("))
            .and_then(|(_, rest)| rest.split_once("\n    }\n"))
            .map(|(rumpf, _)| rumpf)
            .expect("fern_pfad_anspringen steht in appkit/tabelle.rs");
        for (name, text) in [("fern_pruefen", rumpf), ("fern_pfad_anspringen", zweig)] {
            for nadel in ["fs::", "metadata(", "canonicalize(", "read_dir("] {
                assert!(
                    !text.contains(nadel),
                    "{name} enthaelt {nadel}: ein Systemaufruf auf dem Hauptfaden"
                );
            }
        }
    }

    /// Fuer einen fernen Tab liegt keine Datei schon vor dem Nutzer: eine
    /// Datei fuehrt in ihren lokalen Ordner, mit Auswahl, auch wenn es der
    /// Ordner ist, aus dem der Tab vor dem Verbinden kam.
    #[test]
    fn fuer_einen_fernen_tab_fuehrt_eine_lokale_datei_in_ihren_ordner() {
        let ordner = Pruefordner::neu("fern-zwischenablage");
        let datei = ordner.unter("idee.txt");
        fs::write(&datei, b"x").expect("Datei");

        assert_eq!(
            pruefen_fuer_fernen_tab(&datei),
            Ergebnis::Wechseln {
                ordner: ordner.pfad().to_path_buf(),
                auswahl: Some("idee.txt".to_owned())
            }
        );
        assert_eq!(
            pruefen_fuer_fernen_tab(ordner.pfad()),
            Ergebnis::Wechseln {
                ordner: ordner.pfad().to_path_buf(),
                auswahl: None
            }
        );
        assert!(matches!(
            pruefen_fuer_fernen_tab(Path::new("htdocs")),
            Ergebnis::Meldung(_)
        ));
    }

    /// „Zur Zwischenablage springen“ liest in einem fernen Tab einen Pfad auf
    /// diesem Mac und nie einen auf dem Server (Befund
    /// `261017-0004-zur-zwischenablage-springen-liest-in-einem-fernen-tab-einen-lokalen-pfad-als-serverpfad`).
    ///
    /// Bis zur Behebung ging der Pfad aus der Zwischenablage an
    /// `pfad_anspringen`, das ihn in einem fernen Tab an
    /// `fern_pfad_anspringen` reichte. Gelesen werden die Rumpfe in
    /// `appkit/tabelle.rs`: der Sprung ruft `lokalen_pfad_anspringen`, und
    /// dessen ferner Zweig prueft lokal und erreicht den fernen Weg nicht.
    ///
    /// **Was sie nicht sieht:** ob `ordner_lesen` den fernen Tab wirklich auf
    /// lokal stellt; das haelt `Tabliste::ordner_setzen` mit `Tabort::Lokal`.
    #[test]
    fn die_zwischenablage_liest_in_einem_fernen_tab_einen_lokalen_pfad() {
        let tabelle = include_str!("../appkit/tabelle.rs");
        let rumpf = |name: &str| {
            tabelle
                .split_once(&format!("fn {name}("))
                .and_then(|(_, rest)| rest.split_once("\n    }\n"))
                .map(|(rumpf, _)| rumpf)
                .unwrap_or_else(|| panic!("{name} steht in appkit/tabelle.rs"))
        };
        let sprung = rumpf(concat!("zwischenablage", "_springen"));
        assert!(
            sprung.contains("self.lokalen_pfad_anspringen(&pfad)"),
            "der Sprung aus der Zwischenablage geht nicht ueber den lokalen Weg"
        );
        assert!(
            !sprung.contains("self.pfad_anspringen("),
            "der Sprung aus der Zwischenablage erreicht den Weg der Pfadeingabe"
        );
        let lokal = rumpf(concat!("lokalen_pfad", "_anspringen"));
        let (_, ferner_zweig) = lokal
            .split_once("Ort::Fern(_) =>")
            .expect("lokalen_pfad_anspringen verzweigt ueber den Ort");
        assert!(ferner_zweig.contains("pruefen_fuer_fernen_tab("));
        assert!(
            !ferner_zweig.contains("fern_pfad_anspringen("),
            "ein ferner Tab liest den Pfad der Zwischenablage als Serverpfad"
        );
    }

    #[test]
    fn ein_ordner_ohne_leserecht_meldet_und_navigiert_nicht() {
        use std::os::unix::fs::PermissionsExt;

        let ordner = Pruefordner::neu("kein-leserecht");
        let gesperrt = ordner.unter("gesperrt");
        fs::create_dir(&gesperrt).expect("Unterordner");
        fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
            .expect("Rechte lassen sich setzen");

        // Kein Aufraeumen von Hand davor: `Pruefordner::drop` raeumt seit dem
        // 260908 zweistufig ab und kommt an dem `0o000` vorbei, auch wenn die
        // Probe hier fehlschlaegt (Defekt `260826-1442`).
        let Ergebnis::Meldung(text) = pruefen(&gesperrt, ordner.pfad()) else {
            panic!("ein Ordner ohne Leserecht fuehrte zu einer Navigation");
        };
        assert!(
            text.contains("nicht lesen"),
            "die Meldung sagt nicht warum: {text}"
        );
    }
}

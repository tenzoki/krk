//! Der Nachweis, dass die beiden Kisten fuer die Syntaxhervorhebung eingebunden
//! sind und liefern, was S32 von ihnen verlangt.
//!
//! Dieser Pruefcode baut keine Formatansicht — die entsteht in S33. Er beantwortet
//! allein die beiden Fragen, die die Wahl der Kisten getragen haben und die am
//! Papier nicht zu entscheiden waren: fuehrt der Satz der Sprachdefinitionen die
//! vier Sprachen, die der Nutzer in KRK selbst bearbeitet, und gibt es eine helle
//! und eine dunkle Farbtafel zur Auswahl.
//!
//! Er laeuft ohne Fenster und ohne Vordergrund und gehoert damit zu `cargo test`,
//! nicht zur Messstrecke. Die Frage nach der Geschwindigkeit auf dem
//! Referenzgeraet beantwortet er nicht; sie ist mit dem Abnahmelauf aus dieser
//! Runde ausgeklammert.

use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

/// Die vier Sprachen aus dem fuenften Abnahmekriterium von C3.
///
/// Gesucht wird ueber die Dateiendung und nicht ueber den Namen der
/// Sprachdefinition: die Endung ist das, was der Editor spaeter zur Hand hat,
/// wenn er eine Datei oeffnet, und der Name der Definition ist es nie.
const ENDUNGEN: [&str; 4] = ["rs", "toml", "md", "sh"];

/// Der Satz, den KRK ab S33 benutzt. Er kommt aus `two-face` und nicht aus
/// `syntect`, weil er der einzige der beiden ist, der TOML fuehrt.
fn sprachen() -> SyntaxSet {
    two_face::syntax::extra_newlines()
}

#[test]
fn der_satz_fuehrt_die_vier_sprachen() {
    let satz = sprachen();

    for endung in ENDUNGEN {
        let treffer = satz
            .find_syntax_by_extension(endung)
            .unwrap_or_else(|| panic!("keine Sprachdefinition fuer die Endung `{endung}`"));
        assert!(
            !treffer.name.is_empty(),
            "die Sprachdefinition fuer `{endung}` hat keinen Namen"
        );
    }
}

#[test]
fn ohne_two_face_fehlt_toml() {
    // Das ist der gemessene Grund fuer die zweite Kiste und keine Behauptung aus
    // dem Plan: der Vorgabesatz von `syntect` fuehrt Rust, Markdown und Shell,
    // aber kein TOML, und C3 verlangt TOML ausdruecklich.
    //
    // Schlaegt diese Zeile eines Tages fehl, weil `syntect` TOML nachgereicht
    // hat, dann ist das keine Stoerung, sondern der Anlass zu pruefen, ob
    // `two-face` noch gebraucht wird. Deshalb steht sie hier und nicht als Satz
    // in einem Bericht.
    let vorgabe = SyntaxSet::load_defaults_newlines();

    assert!(
        vorgabe.find_syntax_by_extension("toml").is_none(),
        "`syntect` fuehrt TOML inzwischen selbst — `two-face` neu bewerten"
    );
    for endung in ["rs", "md", "sh"] {
        assert!(
            vorgabe.find_syntax_by_extension(endung).is_some(),
            "der Vorgabesatz von `syntect` fuehrt `{endung}` nicht mehr"
        );
    }
}

#[test]
fn eine_unbekannte_endung_faellt_zurueck_statt_zu_scheitern() {
    // Das sechste Abnahmekriterium von C3 verlangt, dass eine Sprache, die die
    // Kiste nicht kennt, keinen Fehler meldet. Der Satz liefert dafuer `None`
    // und haelt daneben eine Definition fuer einfachen Text bereit; auf beides
    // stuetzt sich der Rueckfall in S33.
    let satz = sprachen();

    assert!(
        satz.find_syntax_by_extension("krk-gibt-es-nicht").is_none(),
        "eine erfundene Endung darf keine Sprachdefinition treffen"
    );
    assert_eq!(
        satz.find_syntax_plain_text().name,
        "Plain Text",
        "der Rueckfall auf einfachen Text fehlt"
    );
}

#[test]
fn es_gibt_eine_helle_und_eine_dunkle_farbtafel() {
    // S34 waehlt zwischen zwei fertigen Tafeln, statt eine eigene zu schreiben.
    // Diese Zusage haengt daran, dass es beide fertig gibt. Welche zwei es
    // werden, entscheidet S34; hier steht nur, dass die Auswahl besteht.
    let tafeln = ThemeSet::load_defaults();

    let hell = tafeln
        .themes
        .get("base16-ocean.light")
        .expect("keine helle Farbtafel im Vorgabesatz");
    let dunkel = tafeln
        .themes
        .get("base16-ocean.dark")
        .expect("keine dunkle Farbtafel im Vorgabesatz");

    // Eine Tafel ohne Vordergrundfarbe traegt keine Einfaerbung. Der Grund der
    // Textflaeche bleibt in KRK die Systemfarbe (S34), deshalb wird hier allein
    // der Vordergrund geprueft.
    assert!(
        hell.settings.foreground.is_some(),
        "die helle Farbtafel nennt keine Vordergrundfarbe"
    );
    assert!(
        dunkel.settings.foreground.is_some(),
        "die dunkle Farbtafel nennt keine Vordergrundfarbe"
    );
    assert_ne!(
        hell.settings.foreground, dunkel.settings.foreground,
        "die beiden Tafeln faerben gleich; dann traegt die Wahl zwischen ihnen nichts"
    );
}

#[test]
fn die_einfaerbung_setzt_die_wortarten_in_zwei_sprachen_gegeneinander_ab() {
    // Der eigentliche Nachweis: die Kiste faerbt ein, und sie tut es fuer mehr
    // als eine Sprache. Zwei Sprachen genuegen dafuer, eine aus dem Vorgabesatz
    // von `syntect` und eine, die erst `two-face` mitbringt. Damit ist zugleich
    // belegt, dass beide Kisten zusammenarbeiten.
    use syntect::easy::HighlightLines;
    use syntect::util::LinesWithEndings;

    let satz = sprachen();
    let tafeln = ThemeSet::load_defaults();
    let tafel = &tafeln.themes["base16-ocean.dark"];

    let proben = [
        ("rs", "fn haupt() { let x = \"Text\"; } // Kommentar\n"),
        ("toml", "# Kommentar\n[abschnitt]\nname = \"Wert\"\n"),
    ];

    for (endung, quelle) in proben {
        let sprache = satz
            .find_syntax_by_extension(endung)
            .unwrap_or_else(|| panic!("keine Sprachdefinition fuer `{endung}`"));
        let mut faerber = HighlightLines::new(sprache, tafel);

        let mut farben = std::collections::BTreeSet::new();
        for zeile in LinesWithEndings::from(quelle) {
            for (stil, _) in faerber
                .highlight_line(zeile, &satz)
                .unwrap_or_else(|fehler| panic!("`{endung}` liess sich nicht einfaerben: {fehler}"))
            {
                let vordergrund = stil.foreground;
                farben.insert((vordergrund.r, vordergrund.g, vordergrund.b));
            }
        }

        // Drei verschiedene Vordergrundfarben sind der Unterschied zwischen einer
        // Einfaerbung und einem fast einfarbigen Text. Beide Proben tragen
        // Schluesselwort, Zeichenkette und Kommentar, also mindestens drei
        // Wortarten.
        assert!(
            farben.len() >= 3,
            "`{endung}` bekam nur {} Vordergrundfarbe(n); die Wortarten sind damit \
             nicht gegeneinander abgesetzt",
            farben.len()
        );
    }
}

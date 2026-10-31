//! Strip und Format der Quicknote (`krk_core::text::glaetten`), Schritt 1 des
//! Plans `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`.
//!
//! Die Abnahme von Ziel 3 des Briefs ist die erste Probe: Eingabe und
//! erwartete Ausgabe stehen woertlich so, wie der Nutzer sie angegeben hat, und
//! verglichen wird Zeichen fuer Zeichen. Die uebrigen Proben halten die Faelle,
//! in denen die Schaetzung danebenliegen koennte, in der Richtung, die nichts
//! verbindet, und die Shell-Lagen, die Strip nicht anfassen darf.

use krk_core::text::glaetten::{befehl, fliesstext};

/// Die Eingabe aus dem Brief, so wie sie aus dem Chat kommt.
const BEISPIEL: &str = r#"Von den 202 registrierten Frameworks sind alle 202 strukturell vollständig, und 194 davon kann der Kernel heute auswählen. Acht schließt die Auswahl bewusst aus,
  weil sie ein Readiness-Tag tragen. Die eigentliche Lücke liegt nicht mehr in den Daten, sondern in der Darstellung: Nur 3 der 104 Canvases werden in ihrer
  Originalgeometrie gezeichnet.

  Was vollständig ist (alle 202)
  - Jedes Framework hat ein Manifest mit Shape, Depth, Topics, Phasen, Phasen- und Workstream-Levels, Produces und Feldern.
  - Alle 2.304 Felder haben einen Typ, ein englisches Label und einen Ausfüll-Hinweis für das Modell. Kein Select-Feld ist ohne Optionen.
  - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur
    Kundenlage findet; nachgetragen in c3cabfcb5).
  - Alle zwölf Ontologie-Validatoren enden mit Exit-Code 0, auch die Prüfung der Zählungen gegen ontology/uif-stats.yaml.

  Nicht auswählbar (8), und warum
  Die Auswahl verwirft jedes Framework, dessen Readiness-Tag weder fehlt noch ready-for-client-use lautet (codebase/go/pkg/strategy/readiness.go:67):
  - work-in-progress (6): scope-landscape (Stefan braucht eine Whiteboard-Engine), digital-maturity-assessment (nur der Quick Scan wird gebaut), 360-feedback und
    decision-making-options (beide nach deinen Rulings zurückgestellt), cost-of-change (laut Meta Model V23 WIP) und strategy-self-assessment (der Fragebogen fehlt).
  - workshop-ready (2): dlsf-7-step und value-driven-roadmapping. Laut Meta Model V23 sind sie workshop-tauglich, die Auswahl verwirft sie aber genauso.
  - Folge: Der Project Type h2-business-known listet dlsf-7-step und scope-landscape als must. Dort fehlen zur Laufzeit also zwei Pflicht-Frameworks. Das meldet der
    Validator als Warnung.

  Schwächen, die die Nutzung nicht blockieren
  - Canvas-Geometrie: Ein eigenes Layout haben nur BMC, Extended BMC und SWOT. Die übrigen 101 Canvases werden generisch gerendert.
  - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre
    Quellen abgeglichen. Ob auch die Feldstrukturen der 121 fachlich stimmen, habe ich nicht gemessen.
  - Kleinere Datenpunkte: 55 Canvases haben keine Aliase (Warnung). 81 Warnungen melden typisierte Entitäten, die im Freitext der Inputs/Outputs nicht vorkommen. 7
    Assessments haben Zahlen statt Text als Select-Optionen (offenes Issue 260917-1102). 26 Frameworks haben keine Feldgruppen. Bei 2 fehlt die Altitude, bei 4 die
    Consumes-Liste; beides ist laut Schema zulässig.
  - Sprache: Framework-Texte gibt es nur auf Englisch (ontology/i18n/en/). Eine deutsche Fassung existiert nicht.
"#;

/// Die erwartete Ausgabe aus dem Brief, so vom Nutzer angegeben.
const ERWARTET: &str = r#"Von den 202 registrierten Frameworks sind alle 202 strukturell vollständig, und 194 davon kann der Kernel heute auswählen. Acht schließt die Auswahl bewusst aus, weil sie ein Readiness-Tag tragen. Die eigentliche Lücke liegt nicht mehr in den Daten, sondern in der Darstellung: Nur 3 der 104 Canvases werden in ihrer Originalgeometrie gezeichnet.

Was vollständig ist (alle 202)
 - Jedes Framework hat ein Manifest mit Shape, Depth, Topics, Phasen, Phasen- und Workstream-Levels, Produces und Feldern.
 - Alle 2.304 Felder haben einen Typ, ein englisches Label und einen Ausfüll-Hinweis für das Modell. Kein Select-Feld ist ohne Optionen.
 - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur Kundenlage findet; nachgetragen in c3cabfcb5).
 - Alle zwölf Ontologie-Validatoren enden mit Exit-Code 0, auch die Prüfung der Zählungen gegen ontology/uif-stats.yaml.

Nicht auswählbar (8), und warum
Die Auswahl verwirft jedes Framework, dessen Readiness-Tag weder fehlt noch ready-for-client-use lautet (codebase/go/pkg/strategy/readiness.go:67):
 - work-in-progress (6): scope-landscape (Stefan braucht eine Whiteboard-Engine), digital-maturity-assessment (nur der Quick Scan wird gebaut), 360-feedback und decision-making-options (beide nach deinen Rulings zurückgestellt), cost-of-change (laut Meta Model V23 WIP) und strategy-self-assessment (der Fragebogen fehlt).
 - workshop-ready (2): dlsf-7-step und value-driven-roadmapping. Laut Meta Model V23 sind sie workshop-tauglich, die Auswahl verwirft sie aber genauso.
 - Folge: Der Project Type h2-business-known listet dlsf-7-step und scope-landscape als must. Dort fehlen zur Laufzeit also zwei Pflicht-Frameworks. Das meldet der Validator als Warnung.

Schwächen, die die Nutzung nicht blockieren
 - Canvas-Geometrie: Ein eigenes Layout haben nur BMC, Extended BMC und SWOT. Die übrigen 101 Canvases werden generisch gerendert.
 - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre Quellen abgeglichen. Ob auch die Feldstrukturen der 121 fachlich stimmen, habe ich nicht gemessen.
 - Kleinere Datenpunkte: 55 Canvases haben keine Aliase (Warnung). 81 Warnungen melden typisierte Entitäten, die im Freitext der Inputs/Outputs nicht vorkommen. 7 Assessments haben Zahlen statt Text als Select-Optionen (offenes Issue 260917-1102). 26 Frameworks haben keine Feldgruppen. Bei 2 fehlt die Altitude, bei 4 die Consumes-Liste; beides ist laut Schema zulässig.
 - Sprache: Framework-Texte gibt es nur auf Englisch (ontology/i18n/en/). Eine deutsche Fassung existiert nicht.
"#;

/// Eine Zeile aus `zeichen` der Laenge `laenge`, zum Bau breiter Zeilen.
fn reihe(zeichen: char, laenge: usize) -> String {
    std::iter::repeat_n(zeichen, laenge).collect()
}

#[test]
fn das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe() {
    assert_eq!(fliesstext(BEISPIEL), ERWARTET);
}

#[test]
fn format_ist_idempotent() {
    let einmal = fliesstext(BEISPIEL);
    assert_eq!(fliesstext(&einmal), einmal);

    let absatz = format!(
        "{}\n  {} weiter\n\n  Kurz\n  - {}\n    {}\n",
        reihe('a', 70),
        reihe('b', 12),
        reihe('c', 66),
        reihe('d', 10)
    );
    let einmal = fliesstext(&absatz);
    assert_ne!(einmal, absatz, "der Absatz wird verbunden");
    assert_eq!(fliesstext(&einmal), einmal);

    let punkte = format!(
        "  1. {}\n     {}\n    * tiefer\n  2) zwei",
        reihe('e', 60),
        reihe('f', 9)
    );
    let einmal = fliesstext(&punkte);
    assert_eq!(fliesstext(&einmal), einmal);
}

#[test]
fn ein_text_ohne_einrueckung_wird_nicht_verbunden() {
    let text = format!(
        "{}\n{} zwei\n{}",
        reihe('a', 80),
        reihe('b', 20),
        reihe('c', 79)
    );
    assert_eq!(fliesstext(&text), text);
    assert_eq!(befehl(&text), text);
}

#[test]
fn unter_der_mindestbreite_wird_nichts_verbunden() {
    // Die laengste Zeile hat 39 Zeichen; jede Zeile fuellt sie fast, und
    // trotzdem bleibt alles, wie es ist.
    let text = format!(
        "{}\n  {}\n  {}",
        reihe('a', 39),
        reihe('b', 37),
        reihe('c', 30)
    );
    assert_eq!(
        fliesstext(&text),
        format!("{}\n{}\n{}", reihe('a', 39), reihe('b', 37), reihe('c', 30))
    );
    assert_eq!(
        befehl(&text),
        format!("{}\n{}\n{}", reihe('a', 39), reihe('b', 37), reihe('c', 30))
    );
}

#[test]
fn ein_code_zaun_bleibt_woertlich() {
    let text = format!(
        "{}\n  ```sh\n  {}\n    {} eingerueckt\n  ```\n  danach",
        reihe('a', 60),
        reihe('b', 58),
        reihe('c', 40)
    );
    assert_eq!(
        fliesstext(&text),
        format!(
            "{}\n```sh\n{}\n  {} eingerueckt\n```\ndanach",
            reihe('a', 60),
            reihe('b', 58),
            reihe('c', 40)
        )
    );
}

#[test]
fn ein_tiefer_punkt_behaelt_seinen_abstand() {
    let text = "  - oben\n      - tiefer\n  - wieder oben\n";
    assert_eq!(fliesstext(text), " - oben\n     - tiefer\n - wieder oben\n");
}

#[test]
fn strip_verbindet_einen_terminalumbruch_mit_einem_leerzeichen() {
    // Zwei volle Zeilen belegen die Breite: der Befehl ist zweimal umgebrochen.
    let text = format!("echo {}\n  {}\n  ende", reihe('a', 55), reihe('b', 58));
    assert_eq!(
        befehl(&text),
        format!("echo {} {} ende", reihe('a', 55), reihe('b', 58))
    );
}

#[test]
fn eine_einzelne_lange_zeile_belegt_keinen_umbruch() {
    // Bis zum 261031 galt die laengste Zeile immer als umgebrochen; ein einmal
    // umgebrochener Text bleibt seitdem, wie er ist.
    let text = format!("echo {}\n  {} ende", reihe('a', 55), reihe('b', 8));
    assert_eq!(
        befehl(&text),
        format!("echo {}\n{} ende", reihe('a', 55), reihe('b', 8))
    );
    let text = format!("{}\n  {} weiter", reihe('a', 70), reihe('b', 12));
    assert_eq!(
        fliesstext(&text),
        format!("{}\n{} weiter", reihe('a', 70), reihe('b', 12))
    );
}

#[test]
fn strip_schiebt_keinen_befehl_in_den_kommentar_darueber() {
    let text = "# Erst die Abhaengigkeiten und dann die Proben des Kerns bauen\n  cargo build\n  cargo test";
    assert_eq!(
        befehl(text),
        "# Erst die Abhaengigkeiten und dann die Proben des Kerns bauen\ncargo build\ncargo test"
    );
    // Auch wenn zwei volle Zeilen die Breite belegen, verbindet Strip nicht
    // ueber das Ende eines Kommentars hinweg.
    let text = format!("ls # {}\n  echo {}", reihe('k', 60), reihe('a', 59));
    assert_eq!(
        befehl(&text),
        format!("ls # {}\necho {}", reihe('k', 60), reihe('a', 59))
    );
}

#[test]
fn strip_haengt_zwei_befehle_nicht_aneinander_wenn_der_laengere_zuerst_steht() {
    let text =
        "cargo test --workspace --all-targets -- --skip appkit::\n  cargo clippy --workspace";
    assert_eq!(
        befehl(text),
        "cargo test --workspace --all-targets -- --skip appkit::\ncargo clippy --workspace"
    );
    let text = "cd /Users/kai/Projects\n  git log --oneline 135960bf..be1a725d -- crates/krk-ui/src\n  git status";
    assert_eq!(
        befehl(text),
        "cd /Users/kai/Projects\ngit log --oneline 135960bf..be1a725d -- crates/krk-ui/src\ngit status"
    );
}

#[test]
fn format_erhaelt_eine_kastentabelle_zeile_fuer_zeile() {
    let tabelle = "\
  ┌────────────────────────┬────────────────────────┐
  │ Datei                  │ Zustand                │
  ├────────────────────────┼────────────────────────┤
  │ scratch.txt            │ gesichert              │
  └────────────────────────┴────────────────────────┘
";
    let erwartet = "\
┌────────────────────────┬────────────────────────┐
│ Datei                  │ Zustand                │
├────────────────────────┼────────────────────────┤
│ scratch.txt            │ gesichert              │
└────────────────────────┴────────────────────────┘
";
    assert_eq!(fliesstext(tabelle), erwartet);
    // Hinter einem langen Absatz, der die Breite belegt, ebenso, und die
    // Tabelle haengt sich nicht an ihn.
    let text = format!(
        "{}\n  {}\n\n  Kurz davor\n  | Spalte eins | Spalte zwei, deutlich laenger als alles |\n  |---|---|\n  | a | b |\n",
        reihe('a', 70),
        reihe('b', 66)
    );
    assert_eq!(
        fliesstext(&text),
        format!(
            "{} {}\n\nKurz davor\n| Spalte eins | Spalte zwei, deutlich laenger als alles |\n|---|---|\n| a | b |\n",
            reihe('a', 70),
            reihe('b', 66)
        )
    );
}

#[test]
fn format_erhaelt_eine_baumskizze_samt_ausrichtung() {
    let skizze = "\
krk/
  ├── Cargo.toml            # Workspace, Mitglieder unter `members`
  ├── crates/krk-core/      # Kern ohne AppKit, Verzeichnisleser
  │   └── src/text/         # glaetten.rs und die Nachbarn
  └── HowTo.md              # die Anleitung fuer den Nutzer
";
    let erwartet = "\
krk/
├── Cargo.toml            # Workspace, Mitglieder unter `members`
├── crates/krk-core/      # Kern ohne AppKit, Verzeichnisleser
│   └── src/text/         # glaetten.rs und die Nachbarn
└── HowTo.md              # die Anleitung fuer den Nutzer
";
    assert_eq!(fliesstext(skizze), erwartet);
    let ascii = "  +------+------+\n  | eins | zwei |\n  +------+------+";
    assert_eq!(
        fliesstext(ascii),
        "+------+------+\n| eins | zwei |\n+------+------+"
    );
}

#[test]
fn strip_entfernt_backslash_fortsetzungen_ohne_die_bedeutung_zu_aendern() {
    // Leerraum vor dem `\`: ein Leerzeichen bleibt, wie die Shell zwei Woerter liest.
    assert_eq!(
        befehl("cargo build \\\n  --release"),
        "cargo build --release"
    );
    // Kein Leerraum auf beiden Seiten: die Shell liest ein Wort.
    assert_eq!(befehl("echo foo\\\nbar"), "echo foobar");
    // Ein `\` mit nachlaufendem Leerzeichen ist nach dem Kopieren dieselbe
    // Fortsetzung, und das Leerzeichen faellt.
    assert_eq!(befehl("ls \\ \n  -la \\  \n  /tmp"), "ls -la /tmp");
    // Ein maskiertes `\` am Zeilenende ist keine Fortsetzung.
    assert_eq!(befehl("echo a\\\\\nls"), "echo a\\\\\nls");
}

#[test]
fn strip_laesst_einen_heredoc_rumpf_samt_einrueckung_stehen() {
    let lang = reihe('x', 60);
    // Die Kopiereinrueckung von zwei Zeichen faellt, die Einrueckung des
    // Rumpfs darueber hinaus bleibt, und kein Rumpfumbruch wird verbunden.
    let text = format!("cat <<EOF > a.txt\n  {lang}\n      eingerueckt\n  EOF\n  ls");
    assert_eq!(
        befehl(&text),
        format!("cat <<EOF > a.txt\n{lang}\n    eingerueckt\nEOF\nls")
    );

    let text = format!("cat <<-ENDE\n  \t{lang}\n  \t  weiter\n  \tENDE");
    assert_eq!(
        befehl(&text),
        format!("cat <<-ENDE\n\t{lang}\n\t  weiter\n\tENDE")
    );

    let text = format!("cat <<'EOF'\n  $HOME {lang}\n    nicht verbinden\n  EOF");
    assert_eq!(
        befehl(&text),
        format!("cat <<'EOF'\n$HOME {lang}\n  nicht verbinden\nEOF")
    );

    // Zwei Heredocs in einer Zeile, in ihrer Reihenfolge.
    let text = format!(
        "paste <<A /dev/fd/0 <<\"B\"\n  {lang}\n    eins\n  A\n  {lang}\n    zwei\n  B\n  echo fertig"
    );
    assert_eq!(
        befehl(&text),
        format!("paste <<A /dev/fd/0 <<\"B\"\n{lang}\n  eins\nA\n{lang}\n  zwei\nB\necho fertig")
    );

    // `<<<` ist ein Here-String und oeffnet keinen Rumpf: der Terminalumbruch
    // danach wird verbunden.
    let text = format!("grep -c x <<< {lang}\n  {}\n  ende", reihe('y', 72));
    assert_eq!(
        befehl(&text),
        format!("grep -c x <<< {lang} {} ende", reihe('y', 72))
    );
}

#[test]
fn strip_laesst_einen_umbruch_in_anfuehrungszeichen_stehen() {
    let lang = reihe('q', 60);
    for (auf, zu) in [("'", "'"), ("\"", "\""), ("$'", "'")] {
        let text = format!("echo {auf}{lang}\n  {}{zu}", reihe('r', 10));
        assert_eq!(
            befehl(&text),
            format!("echo {auf}{lang}\n{}{zu}", reihe('r', 10)),
            "Anfuehrung {auf}"
        );
    }
}

#[test]
fn strip_laesst_kurze_befehle_untereinander_stehen() {
    let lang = reihe('z', 45);
    let text = format!("  cd /tmp\n  ls -la\n  echo {lang}\n");
    assert_eq!(befehl(&text), format!("cd /tmp\nls -la\necho {lang}"));
}

#[test]
fn strip_ist_idempotent() {
    let lang = reihe('a', 55);
    for text in [
        format!("echo {lang}\n  {} ende\n  ls", reihe('b', 8)),
        format!("echo {lang}\n  {}\n  ende", reihe('b', 58)),
        "# ein Kommentar ueber einem Befehl, der lang genug ist\n  cargo build".to_owned(),
        "cargo test \\\n  --workspace\n  cat <<EOF\n      rumpf\n  EOF".to_owned(),
        format!("echo '{lang}\n   offen'\n  pwd"),
    ] {
        let einmal = befehl(&text);
        assert_eq!(befehl(&einmal), einmal, "an {text:?}");
    }
}

#[test]
fn strip_endet_ohne_zeilenumbruch_und_ohne_nachlaufenden_leerraum() {
    assert_eq!(befehl("\n\n   ls -la   \n  pwd \t\n\n\n"), "ls -la\npwd");
    assert_eq!(befehl("  \n \n"), "");
    assert_eq!(befehl(""), "");
    assert_eq!(fliesstext(""), "");
}

#[test]
fn ein_kommentar_oeffnet_kein_anfuehrungszeichen() {
    let lang = reihe('k', 60);
    // Das Apostroph im Kommentar oeffnet nichts: der Umbruch nach `ls` bleibt
    // keiner in Anfuehrung, und der Terminalumbruch danach wird verbunden.
    let text = format!("ls # don't\n  echo {lang}\n  {}", reihe('m', 63));
    assert_eq!(
        befehl(&text),
        format!("ls # don't\necho {lang} {}", reihe('m', 63))
    );
    // Ein `#` mitten im Wort ist kein Kommentar, und das Apostroph danach
    // oeffnet eine Anfuehrung.
    let text = format!("echo a#'b\n  {lang}'");
    assert_eq!(befehl(&text), format!("echo a#'b\n{lang}'"));
}

#[test]
fn die_beispiele_der_anleitung_stimmen() {
    // `HowTo.md`, Abschnitt „Strip und Format“: beide Beispiele woertlich.
    let strip = "cd ~/Projekte/krk && export PATH=\"$HOME/.cargo/bin:$PATH\" && cargo test\n  --workspace -- --skip appkit:: && cargo clippy --workspace --all-targets\n  -- -D warnings\n";
    assert_eq!(
        befehl(strip),
        "cd ~/Projekte/krk && export PATH=\"$HOME/.cargo/bin:$PATH\" && cargo test --workspace -- --skip appkit:: && cargo clippy --workspace --all-targets -- -D warnings"
    );
    let format = "Das ist ein langer Absatz aus dem Chat, den das Terminal bei seiner Breite\n  umgebrochen hat.\n\n  Eine Überschrift\n  - Ein Punkt, der ebenfalls so lang ist, dass das Terminal ihn umbrechen\n    musste.\n  - Ein kurzer Punkt.\n";
    assert_eq!(
        fliesstext(format),
        "Das ist ein langer Absatz aus dem Chat, den das Terminal bei seiner Breite umgebrochen hat.\n\nEine Überschrift\n - Ein Punkt, der ebenfalls so lang ist, dass das Terminal ihn umbrechen musste.\n - Ein kurzer Punkt.\n"
    );
}

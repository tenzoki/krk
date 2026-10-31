//! Singlestrip und Format der Quicknote (`krk_core::text::glaetten`), Schritt 1
//! des Plans `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`,
//! Singlestrip nach der Antwort `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`.
//!
//! Die Abnahme von Ziel 3 des Briefs ist die erste Probe: Eingabe und
//! erwartete Ausgabe stehen woertlich so, wie der Nutzer sie angegeben hat, und
//! verglichen wird Zeichen fuer Zeichen. Die uebrigen Proben halten die Faelle,
//! in denen die Schaetzung von Format danebenliegen koennte, und fuer
//! Singlestrip, dass jeder Umbruch ausserhalb faellt und die Shell-Lagen, die
//! es nicht anfassen darf, stehen bleiben.

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
    // Singlestrip schaetzt nicht: die Eingabe ist ein Befehl.
    assert_eq!(
        befehl(&text),
        format!(
            "{} {} zwei {}",
            reihe('a', 80),
            reihe('b', 20),
            reihe('c', 79)
        )
    );
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
    // Singlestrip kennt keine Mindestbreite.
    assert_eq!(
        befehl(&text),
        format!("{} {} {}", reihe('a', 39), reihe('b', 37), reihe('c', 30))
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
fn singlestrip_verbindet_einen_einmal_umbrochenen_befehl() {
    // Der Fall aus dem Issue `261031-1226-ein-einmal-umbrochener-text-…`: bis
    // zum 261031 blieb die Fortsetzung stehen, und eingefuegt lief
    // `git push --force-with-lease` ohne Ziel.
    let text = "git add -A && git commit -m 'quicknote: strip und format' && git push --force-with-lease\n  origin feature/quicknote-strip-und-format";
    assert_eq!(
        befehl(text),
        "git add -A && git commit -m 'quicknote: strip und format' && git push --force-with-lease origin feature/quicknote-strip-und-format"
    );
}

#[test]
fn singlestrip_verbindet_einen_zweimal_umbrochenen_befehl() {
    let text = format!("echo {}\n  {}\n  ende", reihe('a', 55), reihe('b', 58));
    assert_eq!(
        befehl(&text),
        format!("echo {} {} ende", reihe('a', 55), reihe('b', 58))
    );
}

#[test]
fn singlestrip_verbindet_einen_dreimal_umbrochenen_befehl() {
    let text = "cargo test -p krk-core --test glaetten --test heimordner --test\n  belegung -- --nocapture && cargo clippy --workspace --all-targets -- -D\n  warnings && cargo fmt --all --check && git status --short\n  --branch";
    assert_eq!(
        befehl(text),
        "cargo test -p krk-core --test glaetten --test heimordner --test belegung -- --nocapture && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check && git status --short --branch"
    );
}

#[test]
fn eine_einzelne_lange_zeile_belegt_keinen_umbruch() {
    // Format: bis zum 261031 galt die laengste Zeile immer als umgebrochen.
    // Ohne belegte Breite wird die Fortsetzung nicht verbunden und behaelt
    // seitdem ihre Einrueckung (Issue `261031-1226-ein-einmal-umbrochener-text-…`).
    let text = format!("{}\n  {} weiter", reihe('a', 70), reihe('b', 12));
    assert_eq!(
        fliesstext(&text),
        format!("{}\n  {} weiter", reihe('a', 70), reihe('b', 12))
    );
    // Singlestrip fragt keine Breite: der Befehl wird eine Zeile.
    let text = format!("echo {}\n  {} ende", reihe('a', 55), reihe('b', 8));
    assert_eq!(
        befehl(&text),
        format!("echo {} {} ende", reihe('a', 55), reihe('b', 8))
    );
}

#[test]
fn format_laesst_einen_einmal_umbrochenen_punkt_und_absatz_als_fortsetzung_stehen() {
    // Ein Punkt aus dem Beispiel des Nutzers, allein ausgewaehlt.
    let punkt = "- Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur\n    Kundenlage findet; nachgetragen in c3cabfcb5).";
    let einmal = fliesstext(punkt);
    assert_eq!(
        einmal,
        " - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur\n    Kundenlage findet; nachgetragen in c3cabfcb5)."
    );
    assert_eq!(fliesstext(&einmal), einmal, "idempotent");
    // Steht derselbe Punkt spaeter neben einem zweiten, ebenso allein
    // formatierten, belegen beide die Breite, und ein Format verbindet sie.
    let groesser = format!(
        "{einmal}\n - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre\n    Quellen abgeglichen."
    );
    assert_eq!(
        fliesstext(&groesser),
        " - Jede englische Textdatei hat Name, Beschreibung, Abgrenzung, KI-Hinweise, Outputs und eine Client-Match-Surface (der Text, über den die Auswahl ein Framework zur Kundenlage findet; nachgetragen in c3cabfcb5).\n - Herkunft: 121 Manifeste stammen aus dem generierten UFDS-Import, 81 sind aus Quellen erstellt. Laut Commit 263a0989e wurden die Texte der UFDS-Frameworks gegen ihre Quellen abgeglichen."
    );

    // Der erste Absatz des Beispiels, allein auf zwei Zeilen.
    let absatz = "Von den 202 registrierten Frameworks sind alle 202 strukturell vollständig, und 194 davon kann der Kernel heute auswählen. Acht schließt die Auswahl bewusst aus,\n  weil sie ein Readiness-Tag tragen.";
    assert_eq!(fliesstext(absatz), absatz);
}

#[test]
fn singlestrip_nimmt_einen_kommentar_samt_seinen_folgezeilen_heraus() {
    // Die sichere Wahl (Modulkopf): der Kommentar faellt, damit keine Shell,
    // die `#` in der Eingabe nicht als Kommentar liest (die zsh von macOS ab
    // Werk), seine Woerter als Argumente bekommt; die Folgezeilen gehoeren nach
    // der Annahme „ein Befehl“ zu ihm und fallen mit.
    assert_eq!(befehl("ls -la # zeigt alle\n  dateien an"), "ls -la");
    let text =
        "ls -la # zeigt alles, auch was das Terminal des Chats beim Kopieren\n  umgebrochen hat";
    assert_eq!(befehl(text), "ls -la");
    // Ein Befehl unter einer Kommentarzeile faellt mit: es laeuft weniger,
    // nie ein Wort, das niemand als Befehl geschrieben hat.
    let text = "# Erst die Abhaengigkeiten und dann die Proben des Kerns bauen\n  cargo build\n  cargo test";
    assert_eq!(befehl(text), "");
    // Im Kommentar liest der Automat nichts mehr: kein Anfuehrungszeichen,
    // kein Heredoc, kein `\`.
    let text = "ls # don't\n  cat <<EOF \\\n  echo 'x";
    assert_eq!(befehl(text), "ls");
    // Auf der letzten Zeile, nach einer Fortsetzung und nach dem Ende eines
    // Heredocs faellt er ebenso, samt dem Leerraum davor.
    assert_eq!(
        befehl("cargo build \\\n  --release   # schnell"),
        "cargo build --release"
    );
    assert_eq!(
        befehl("cargo build \\\n  # schnell\n  --release"),
        "cargo build"
    );
    assert_eq!(
        befehl("cat <<EOF\n  rumpf\n  EOF\n  # erledigt\n  ls"),
        "cat <<EOF\nrumpf\nEOF"
    );
    // Was ausserhalb kein Kommentar ist, bleibt, und was vor dem `#` steht,
    // auch hinter `;`.
    assert_eq!(befehl("echo a;# don't\n  weiter"), "echo a;");
    assert_eq!(befehl("echo ${#x}\n  $# a#b"), "echo ${#x} $# a#b");
    assert_eq!(
        befehl("curl https://example.org/seite#anker\n  -o seite.html"),
        "curl https://example.org/seite#anker -o seite.html"
    );
    assert_eq!(
        befehl("echo '# kein'\n  \"# auch keiner\""),
        "echo '# kein' \"# auch keiner\""
    );
}

#[test]
fn ein_kommentar_vor_einem_heredoc_rumpf_reisst_den_rumpf_nicht_mit() {
    // Die Zeile oeffnet den Heredoc vor dem Kommentar: der Rumpf ist Inhalt,
    // in jeder Shell, und bleibt samt Endzeile; allein der Kommentar faellt.
    let text = "cat <<'EOF' > a.txt   # legt die Datei an\n  zeile eins  \n    zeile zwei\n  EOF\n  ls a.txt";
    assert_eq!(
        befehl(text),
        "cat <<'EOF' > a.txt\nzeile eins  \n  zeile zwei\nEOF\nls a.txt"
    );
    // Ein Kommentar im Rumpf ist Text des Rumpfs.
    assert_eq!(
        befehl("cat <<EOF # an\n  # im rumpf\n  EOF"),
        "cat <<EOF\n# im rumpf\nEOF"
    );
    // Steht er auf der letzten Zeile, bleibt allein, was davor steht.
    assert_eq!(befehl("cat <<EOF # an"), "cat <<EOF");
}

#[test]
fn singlestrip_ist_ohne_kommentar_idempotent() {
    for text in [
        "ls -la # zeigt alle\n  dateien an",
        "# Kommentar\n  cargo build",
        "cat <<'EOF' > a.txt # legt an\n  zeile  \n  EOF\n  ls",
        "cargo build \\\n  --release # schnell",
    ] {
        let einmal = befehl(text);
        assert!(!einmal.contains("# "), "Kommentar blieb in {einmal:?}");
        assert_eq!(befehl(&einmal), einmal, "an {text:?}");
    }
}

#[test]
fn eine_leerzeile_nach_einer_fortsetzung_trennt_zwei_woerter() {
    // Die Leerzeile ist ein Umbruch und ergibt ein Leerzeichen; `foobar` waere
    // ein Wort, das niemand geschrieben hat.
    assert_eq!(befehl("foo\\\n\n  bar"), "foo bar");
    assert_eq!(befehl("foo \\\n\n  bar\n"), "foo bar");
    assert_eq!(befehl("ls\n\n  -la\n"), "ls -la");
    // Ohne Leerzeile bleibt es ein Wort, wie die Shell es liest.
    assert_eq!(befehl("foo\\\nbar"), "foobar");
    let einmal = befehl("foo\\\n\n  bar");
    assert_eq!(befehl(&einmal), einmal);
}

#[test]
fn singlestrip_haengt_getrennte_befehle_aneinander_denn_die_eingabe_ist_einer() {
    // Der Preis der Annahme (Issue `261031-1226-strip-verbindet-getrennte-
    // befehle-…`): zwei Befehle untereinander werden einer, der zweite als
    // Argumente des ersten. `cmd+z` nimmt das zurueck.
    let text = "rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b\n  ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978";
    assert_eq!(
        befehl(text),
        "rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978"
    );
    let text = "  cd /tmp\n  ls -la\n\n  pwd\n";
    assert_eq!(befehl(text), "cd /tmp ls -la pwd");
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
fn singlestrip_entfernt_backslash_fortsetzungen_ohne_die_bedeutung_zu_aendern() {
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
    assert_eq!(befehl("echo a\\\\\nls"), "echo a\\\\ ls");
}

#[test]
fn singlestrip_laesst_einen_heredoc_rumpf_samt_einrueckung_stehen() {
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

    // `<<<` ist ein Here-String und oeffnet keinen Rumpf: der Umbruch danach
    // wird verbunden.
    let text = format!("grep -c x <<< {lang}\n  {}\n  ende", reihe('y', 72));
    assert_eq!(
        befehl(&text),
        format!("grep -c x <<< {lang} {} ende", reihe('y', 72))
    );

    // Ein Befehl mit Heredoc, wie er aus dem Chat kommt: vor dem Rumpf wird
    // verbunden, der Rumpf bleibt, und nach der Endzeile beginnt eine Zeile.
    let text = "cat <<'EOF' | sed -e 's/a/b/' | sort -u > /tmp/ausgabe-der-quicknote.txt\n  2>/dev/null\n  zeile eins\n    zeile zwei\n  EOF";
    assert_eq!(
        befehl(text),
        "cat <<'EOF' | sed -e 's/a/b/' | sort -u > /tmp/ausgabe-der-quicknote.txt\n2>/dev/null\nzeile eins\n  zeile zwei\nEOF"
    );
}

#[test]
fn singlestrip_laesst_einen_umbruch_in_anfuehrungszeichen_stehen() {
    let lang = reihe('q', 60);
    for (auf, zu) in [("'", "'"), ("\"", "\""), ("$'", "'")] {
        let text = format!("echo {auf}{lang}\n  {}{zu}", reihe('r', 10));
        assert_eq!(
            befehl(&text),
            format!("echo {auf}{lang}\n{}{zu}", reihe('r', 10)),
            "Anfuehrung {auf}"
        );
    }
    // Eine Anfuehrung ueber den Umbruch: in ihr bleibt er, danach wird
    // verbunden.
    let text = "git commit -m 'quicknote: singlestrip\n\n  nimmt jeden Umbruch ausserhalb' --author\n  'Kai <ks@example.org>'";
    assert_eq!(
        befehl(text),
        "git commit -m 'quicknote: singlestrip\n\nnimmt jeden Umbruch ausserhalb' --author 'Kai <ks@example.org>'"
    );
}

#[test]
fn singlestrip_ist_idempotent() {
    let lang = reihe('a', 55);
    for text in [
        format!("echo {lang}\n  {} ende\n  ls", reihe('b', 8)),
        format!("echo {lang}\n  {}\n  ende", reihe('b', 58)),
        "git push --force-with-lease\n  origin main".to_owned(),
        "# ein Kommentar ueber einem Befehl, der lang genug ist\n  cargo build".to_owned(),
        "cargo test \\\n  --workspace\n  cat <<EOF\n      rumpf\n  EOF\n  ls".to_owned(),
        format!("echo '{lang}\n  offen'\n  pwd"),
        "  cd /tmp\n  ls -la\n\n  pwd\n".to_owned(),
    ] {
        let einmal = befehl(&text);
        assert_eq!(befehl(&einmal), einmal, "an {text:?}");
    }
    // Der eine Fall, den der Modulkopf nennt: nach dem ersten Lauf traegt jede
    // Zeile ab der zweiten noch Einrueckung, und ein zweiter Lauf nimmt sie.
    let einmal = befehl(&format!("echo '{lang}\n   offen'\n  pwd"));
    assert_eq!(einmal, format!("echo '{lang}\n offen' pwd"));
    assert_eq!(befehl(&einmal), format!("echo '{lang}\noffen' pwd"));
}

#[test]
fn singlestrip_endet_ohne_zeilenumbruch_und_ohne_nachlaufenden_leerraum() {
    assert_eq!(befehl("\n\n   ls -la   \n  pwd \t\n\n\n"), "ls -la pwd");
    assert_eq!(befehl("  \n \n"), "");
    assert_eq!(befehl(""), "");
    assert_eq!(fliesstext(""), "");
}

#[test]
fn ein_kommentar_oeffnet_kein_anfuehrungszeichen() {
    let lang = reihe('k', 60);
    // Das Apostroph im Kommentar oeffnet nichts: kein Umbruch bleibt als einer
    // in Anfuehrung stehen, und der Kommentar faellt samt seinen Folgezeilen.
    let text = format!("ls # don't\n  echo {lang}\n  {}", reihe('m', 63));
    assert_eq!(befehl(&text), "ls");
    // Ein `#` mitten im Wort ist kein Kommentar, und das Apostroph danach
    // oeffnet eine Anfuehrung.
    let text = format!("echo a#'b\n  {lang}'");
    assert_eq!(befehl(&text), format!("echo a#'b\n{lang}'"));
}

#[test]
fn die_beispiele_der_anleitung_stimmen() {
    // `HowTo.md`, Abschnitt „Singlestrip und Format“: jedes Beispiel woertlich.
    let strip = "cd ~/Projekte/krk && export PATH=\"$HOME/.cargo/bin:$PATH\" && cargo test\n  --workspace -- --skip appkit:: && cargo clippy --workspace --all-targets\n  -- -D warnings\n";
    assert_eq!(
        befehl(strip),
        "cd ~/Projekte/krk && export PATH=\"$HOME/.cargo/bin:$PATH\" && cargo test --workspace -- --skip appkit:: && cargo clippy --workspace --all-targets -- -D warnings"
    );
    let mehrere = "rm -rf build/alt\n  ls -la build/neu\n";
    assert_eq!(befehl(mehrere), "rm -rf build/alt ls -la build/neu");
    let kommentar = "ls -la # zeigt alle\n  dateien an\n";
    assert_eq!(befehl(kommentar), "ls -la");
    let format = "Das ist ein langer Absatz aus dem Chat, den das Terminal bei seiner Breite\n  umgebrochen hat.\n\n  Eine Überschrift\n  - Ein Punkt, der ebenfalls so lang ist, dass das Terminal ihn umbrechen\n    musste.\n  - Ein kurzer Punkt.\n";
    assert_eq!(
        fliesstext(format),
        "Das ist ein langer Absatz aus dem Chat, den das Terminal bei seiner Breite umgebrochen hat.\n\nEine Überschrift\n - Ein Punkt, der ebenfalls so lang ist, dass das Terminal ihn umbrechen musste.\n - Ein kurzer Punkt.\n"
    );
}

#[test]
fn singlestrip_laesst_nachlaufenden_leerraum_im_heredoc_rumpf_und_in_anfuehrung_stehen() {
    // Die zwei Leerzeichen eines harten Umbruchs in Markdown sind Inhalt; die
    // Endzeile verliert ihren Leerraum, damit sie den Rumpf beendet.
    assert_eq!(
        befehl("cat <<EOF > a.md\n  zeile mit hartem Umbruch  \n  weiter\n  EOF  "),
        "cat <<EOF > a.md\nzeile mit hartem Umbruch  \nweiter\nEOF"
    );
    // Endet eine Zeile in offener Anfuehrung, gehoert ihr Leerraum zum Wert.
    assert_eq!(
        befehl("echo 'wert mit leerraum  \n  zweite zeile'  \n  ls  "),
        "echo 'wert mit leerraum  \nzweite zeile' ls"
    );
    // Ausserhalb faellt er weiter, auch vor der Endzeile und hinter einem `\`.
    assert_eq!(befehl("ls -la   \n  pwd  "), "ls -la pwd");
    assert_eq!(befehl("ls \\  \n  -la"), "ls -la");
}

#[test]
fn eine_arithmetik_oeffnet_keinen_heredoc() {
    let lang = reihe('w', 60);
    for anfang in [
        "echo $((1 << 4))",
        "(( x = 1 << 4 ))",
        "echo $(( (1 << 2) + 1 ))",
    ] {
        // Hinter der Arithmetik wird weiter verbunden: kein Rumpf ist offen.
        let text = format!("{anfang}\n  echo {lang}\n  {}\n  -n", reihe('v', 63));
        assert_eq!(
            befehl(&text),
            format!("{anfang} echo {lang} {} -n", reihe('v', 63)),
            "an {anfang}"
        );
    }
    // Ausserhalb einer Arithmetik bleibt `<<` ein Heredoc.
    assert_eq!(
        befehl("cat <<E\n  $((1 << 4))\n  E"),
        "cat <<E\n$((1 << 4))\nE"
    );
}

#[test]
fn singlestrip_bleibt_mit_erhaltenem_leerraum_idempotent() {
    for text in [
        "cat <<EOF > a.md\n  zeile mit hartem Umbruch  \n  weiter\n  EOF",
        "echo 'wert  \n  zweite'\n  ls",
        "echo $((1 << 4))\n  ls",
    ] {
        let einmal = befehl(text);
        assert_eq!(befehl(&einmal), einmal, "an {text:?}");
    }
}

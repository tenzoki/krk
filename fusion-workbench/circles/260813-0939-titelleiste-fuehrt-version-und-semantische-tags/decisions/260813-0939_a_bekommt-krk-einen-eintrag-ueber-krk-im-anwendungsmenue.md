# Bekommt KRK zugleich einen Eintrag "Über KRK" im Anwendungsmenü?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_*_circle.md` (Directive und Grounding-Aufnahme); `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` (Runde 7, das vollständige Hauptmenü); `crates/krk-ui/src/menuemodell.rs` (die eine Gliederung des Menüs)

---

## Question

Diese Runde bringt Namen und Version in die Titelleiste. Auf dem Mac ist der übliche Ort für dieselbe Angabe ein Eintrag "Über KRK" ganz oben im Anwendungsmenü, und den führt KRK nicht: die Runde 7 hat das vollständige Hauptmenü gebaut, das Anwendungsmenü trägt den Sonderposten für die Markdown-Ausgabe, einen Trenner und das Beenden, aber keinen Über-Eintrag. Die Frage muss jetzt entschieden werden, weil die beiden Anzeigen dieselbe Zahl zeigen und weil ein späterer Nachtrag eine zweite Stelle anlegt, an der Name und Version zusammengesetzt werden.

## Options

1. **Nur die Titelleiste, kein Menüeintrag** — der Zuschnitt des Backlog-Eintrags, unverändert.
   - Pro: kleinster Umfang, keine Berührung des Menüs der Runde 7, keine neue Frage nach Kürzel und Platzierung.
   - Contra: der auf dem Mac erwartete Ort bleibt leer. Wer die Version sucht, sucht sie zuerst dort.
2. **Zusätzlich der Standard-Über-Dialog von AppKit** — ein Eintrag, der das Systemfenster öffnet, das Name, Version und Symbol aus der `Info.plist` des Bündels liest.
   - Pro: sehr wenig Code, und die Zahl kommt aus derselben einen Quelle wie die Titelleiste, weil `cargo xtask bundle` sie in die `Info.plist` einsetzt. Das Symbol ist da, seit die Runde 4 es baut.
   - Contra: der Dialog ist von macOS gestaltet und nicht von KRK; seine Beschriftungen folgen den Sprachen aus `CFBundleLocalizations`. Der Eintrag braucht eine Einordnung als Sonderposten oder als Belegungseintrag, und die Runde 7 hat dafür eine Regel: ein Kürzel machte ihn zwingend zum Belegungseintrag.
3. **Zusätzlich ein eigenes Über-Fenster von KRK** — eine eigene Fläche mit Name, Version, Tag und was sonst gewollt ist.
   - Pro: KRK bestimmt Inhalt und Sprache selbst und kann den Git-Tag oder den Bauzeitpunkt mitnennen.
   - Contra: teuerste Möglichkeit, eine weitere AppKit-Fläche, und sie öffnet die Frage nach dem Arbeitsstand wieder, die Antwort 3 der Klärungsrunde für die Titelleiste verneint hat.

## Constraints

- Die Zahl bleibt an einer Stelle: `[workspace.package] version` in der `Cargo.toml`. Eine zweite Anzeige darf keine zweite Quelle anlegen.
- Ein Menüeintrag mit Kürzel wird nach dem Entscheid vom 260805-0000 zwingend ein Eintrag in `resources/default-keymap.toml` und wächst damit in `Kommando`, `Kommando::wirkungsbereich` und `bereich_des_kommandos` mit. Ohne Kürzel bleibt er ein Sonderposten, wie die Markdown-Ausgabe der Runde 3.
- Die Gliederung des Menüs steht an genau drei Stellen im Baum; wer das Menü ändert, ändert sie dort und nicht daneben.

## Recommendation

Möglichkeit 2, sofern der Nutzer den Eintrag überhaupt will. Sie kostet am wenigsten, hält die eine Quelle der Zahl ein und braucht kein Kürzel, bleibt also ein Sonderposten neben der Markdown-Ausgabe. Möglichkeit 3 lohnt erst, wenn im Über-Fenster mehr stehen soll als Name und Version, und genau dieses Mehr hat Antwort 3 der Klärungsrunde für die Titelleiste ausgeschlossen.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md, Abschnitt "Drei Fragen beantwortet" — Antwort: Möglichkeit 2, der Standard-Über-Dialog von AppKit, Menüeintrag ohne Kürzel als Sonderposten.

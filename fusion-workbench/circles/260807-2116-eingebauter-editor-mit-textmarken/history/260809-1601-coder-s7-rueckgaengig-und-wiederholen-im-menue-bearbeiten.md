# S7: Rückgängig und Wiederholen im Menü „Bearbeiten"

- Agent: `coder`
- Datum: 260809-1601
- Plan: `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Phase A, Schritt 7
- Status: Complete

## Was umgesetzt ist

Das Untermenü „Bearbeiten" trägt zwei Einträge mehr, beide über dieselbe
Funktion `befehl(mtm, belegung, titel, sel, kennung)` wie die vier bestehenden:

```text
Bearbeiten
├── Rückgängig      undo:        text_rueckgaengig
├── Wiederholen     redo:        text_wiederholen
├── ───── Trenner ─────
├── Ausschneiden    cut:         text_ausschneiden
├── Kopieren        copy:        text_kopieren
├── Einfügen        paste:       text_einfuegen
└── Alles auswählen selectAll:   text_alles_auswaehlen
```

Die beiden stehen an der Mac-üblichen Stelle ganz oben und sind durch
`NSMenuItem::separatorItem` von den vier Zwischenablage-Befehlen getrennt.

**Kein Ziel wird gesetzt.** `roher_befehl` legt das `NSMenuItem` ohne Ziel an,
so wie für alle sieben bestehenden Einträge; die Antwortkette entscheidet, wer
`undo:` beantwortet. Im Editor ist das der Rückgängigverwalter der
`NSTextView`.

**Kein Kürzel steht als Zeichenkette im Programmtext.** Beide Einträge holen
ihre Kombination unter ihrer Kennung aus der Belegung, wie der Modulkopf es
ohne Ausnahme verlangt. `resources/default-keymap.toml` führt
`text_rueckgaengig` auf `cmd+z` und `text_wiederholen` auf `shift+cmd+z`, beide
mit `gehalten_von = "menue"`; die Datei ist mit S6 gelandet und wurde nicht
angefasst.

## Zwei Stellen daneben, die mitwandern mussten

**Der Modulkopf.** Drei Abschnitte nannten die Textbefehle namentlich oder
zählten sie; sie nennen jetzt auch `undo:` und `redo:` und sprechen von sechs
statt vier Textbefehlen. Der Abschnitt „Warum es das Menue Bearbeiten
ueberhaupt gibt" bekommt den zweiten Grund dazu: Cmd+Z liegt auf dem Mac
genauso wenig im Textsystem wie Cmd+V, und ohne diese beiden Einträge hätte der
Editor kein Rückgängig.

**Die Probe `jede_kennung_des_hauptmenues_steht_in_der_auslieferungsbelegung`.**
Sie zählt die Kennungen auf, unter denen `hauptmenue` seine Kürzel sucht, und
hätte die beiden neuen sonst nicht gedeckt. Ohne den Nachtrag fiele ein Tippfehler
in einer Kennung erst beim Start auf, als Meldung auf der Fehlerausgabe.

## Abnahme

`make menue` am gebauten und signierten Bündel, ohne Fenster und ohne
Vordergrund:

```text
menue="Bearbeiten" eintrag="Rückgängig" kombination=cmd+z kuerzel="z" zusatztasten=1048576 zweitform=nein verdeckt=nein selektor=undo:
menue="Bearbeiten" eintrag="Wiederholen" kombination=shift+cmd+z kuerzel="z" zusatztasten=1179648 zweitform=nein verdeckt=nein selektor=redo:
menue="Bearbeiten" trenner
```

Beide Zeilen tragen die geforderte Kombination und den geforderten Selektor.
Die Ausgabe zeigt darüber hinaus keinen vom System dazugestellten Eintrag; für
die spät gestellten Zweitformen gilt der Vorbehalt, den `protokollieren` unter
„Was diese Marke nicht sieht" führt.

Die vier Abnahmekommandos laufen grün: `cargo build --workspace`,
`cargo test --workspace` (alle 15 Prüfziele, 602 Proben, 0 Fehlschläge),
`cargo clippy --workspace --all-targets` ohne Warnung, `cargo fmt --all --check`
ohne Abweichung.

**Dass Rückgängig im Editor wirkt, ist mit diesem Schritt nicht geprüft.** Es
verlangt eine laufende Anwendung mit Fenster und Fokus in der Textfläche; S42
führt es als Nutzerarbeit.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/menue.rs`

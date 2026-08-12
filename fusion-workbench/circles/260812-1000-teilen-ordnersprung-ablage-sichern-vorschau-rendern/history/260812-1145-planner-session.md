# Planner-Lauf 260812-1145: Implementierungsplan der sechsten Runde

**Circle:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`
**Erzeugt:** `planning/260812-1145_o_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`
**Dazu abgelegt:** `decisions/260812-1145_o_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
**Baumstand:** `6463ac6` und die unversionierten Aenderungen darueber

## Was gelesen wurde

`CLAUDE.md`, der Circle-Datensatz `_t_circle.md`, alle vierzehn Datensaetze unter
`decisions/` samt ihrer Antworten vom 260812-1105, der Bericht
`history/260812-1055-orchestrator-session.md`, die Abschlussnotiz der Runde 5, und
der Plan der Runde 5 als Vorlage. Am Baum gelesen: `statuszeile.rs`,
`aufteilung.rs`, `fenster.rs`, `bereichsleiste.rs`, `tabelle.rs`,
`vorschaumodell.rs`, `vorschau.rs`, `hervorhebung.rs`, `editor.rs` (die
Formatierungsstellen), `standardprogramm.rs`, `ablage/mod.rs`, `ablage/atomar.rs`,
`ablage/pfade.rs`, `tasten/belegung.rs`, `belegungsmodell.rs`,
`kommandos/operationen.rs`, `kommandos/fokus.rs`, `messmodus.rs` (die
Endbedingungen), `krk-bench/src/fixture.rs` und `krk-bench/src/messen.rs`.

## Was gemessen statt angenommen wurde

- **`pulldown-cmark` 0.13.4, `--no-default-features`:** drei Abhaengigkeiten
  (`bitflags 2.13.1`, `memchr 2.8.3`, `unicase 2.9.0`), davon die ersten beiden in
  genau diesen Fassungen bereits in `Cargo.lock`. Kein C-Code, `build.rs` ohne das
  Merkmal `gen-tests` leer, `rust-version = 1.71.1`.
- **Geschwindigkeit:** 1,05 MB Markdown, `--release`, drei Laeufe: 29,8 / 22,1 /
  20,9 ms, also 34 bis 48 MB/s. Zum Vergleich `syntect` im Baum: 0,3 MB/s.
- **Tabellen ohne das Tabellenmerkmal:** die Zeilen kommen als drei `Text`-
  Ereignisse mit erhaltenen Zwischenraeumen. Das Quelltextraster aus dem
  Nutzerentscheid entsteht ohne Sonderregel.
- **Am SDK gelesen:** `NSSharingServicePicker` seit 10.8, `standardShareMenuItem`
  seit 13.0, `textView:menu:forEvent:atIndex:` seit 10.5, `NSMenuDelegate` und
  `menuNeedsUpdate:` ohne eigene Angabe, die vier `NSScrollView`-Setzer ohne
  eigene Angabe. Hoechste Untergrenze der Runde: **13.0**, Bauziel 15.0.
- **Apples Markdown-Weg geprueft und verworfen:** `initWithMarkdownString:` gibt
  Absichten statt einer Darstellung, zieht Zwischenraeume zusammen
  (`NSAttributedString.h:147`) und liefert ein Objective-C-Objekt, das ohne
  Fenster nicht mehr pruefbar waere.
- **`shift+cmd+s` und `opt+cmd+o` sind in `default-keymap.toml` unbelegt**,
  nachgezaehlt am 260812.

## Die drei Entscheidungen, die der Plan selbst trifft

1. **Womit die Vorschau Markdown zerlegt** — der Circle uebergibt die Frage
   ausdruecklich dem Plan. Antwort: `pulldown-cmark` ohne Vorgabemerkmale, mit
   drei geprueften Alternativen und den Zahlen dazu.
2. **Was geschieht, wenn beide Dateifenster zugleich eine Meldung haben** — der
   Datensatz `260812-1105_a_…` verweist die Frage in den Plan. Antwort: erst der
   Rang, dann die aktive Seite, also die bestehende Rangfolge um eine zweite
   Stelle erweitert statt einer zweiten Ordnung daneben. Der Namenszusatz steht
   genau bei der inaktiven Seite.
3. **Wo die neue Statuszeile liegt und was mit `MINDESTGROESSE` geschieht** —
   ueber der Bereichsleiste, und die Mindesthoehe steigt um genau 18 auf 336
   Punkte, als Summe und nicht als Wahl. Die Dateiliste verliert dabei keine
   Hoehe; die Rechnung steht im Plan.

## Was aufgefallen ist und abgelegt wurde

Der Rechtsklick in der Dateiliste wirkt nach der Regel der Runde 4 auf die
markierten oder ausgewaehlten Eintraege und nicht auf die Zeile unter dem Zeiger.
Solange das Menue nur das Teilen traegt, ist das eine Irritation; mit einem
zweiten Eintrag waere es ein Schaden. Als Nutzerfrage abgelegt.

## Zuschnitt

Elf Schritte, neun fuer `coder`, zwei fuer `ontocoder`. Sechs sind vollstaendig
ohne KRK im Vordergrund abzunehmen. Zwei Belegungsschritte lassen
`cargo test -p krk-ui` fuer je genau einen Schritt rot; der Teilsatz, der gruen
sein muss, steht am Schritt. Die Runde 5 hat dieselbe Lage ueber drei Schritte
gehalten und zwei Defektdatensaetze dafuer bezahlt.

`Kommando` waechst von 73 auf 75, die Auslieferungsbelegung von 79 auf 81
Funktionen mit 87 Kombinationen. `Wirkungsbereich`, `Bereich` und `Fokus` wachsen
nicht.

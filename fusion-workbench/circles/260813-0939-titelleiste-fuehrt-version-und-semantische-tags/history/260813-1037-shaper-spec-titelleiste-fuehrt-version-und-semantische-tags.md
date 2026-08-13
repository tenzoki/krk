# Shaper: Spec für den aktiven Circle „Titelleiste führt Version und semantische Tags"

**Datum:** 2026-08-13, 10:37
**Agent:** shaper
**Modus:** in-Circle, Spec für den aktiven Circle
**Circle:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/` (aktiv seit 260813-1006)
**Status:** Complete

---

## Auftrag

Den Spec für den aktiven Circle schreiben. Alle sieben Fragen des Circles waren beantwortet, vier in der Klärungsrunde vom 260813-0939 und drei am 260813-1010 an den Datensätzen in `decisions/`, alle drei nach der Empfehlung des Datensatzes. Keine Klärungsrunde war zu fahren und keine wurde gefahren.

Zwei Punkte hatte der Auftrag ausdrücklich zu entscheiden: ob der offene Defekt zum Auslieferungsort hereingeholt wird, und wie die elf Abnahmekriterien von C11 der Runde 2 nach dieser Runde lauten.

## Gelesen

- Circle-Datensatz `_t_circle.md` vollständig, samt Grounding-Aufnahme und Aktivierungsvorschlag des Playmakers
- die drei beantworteten Entscheidungsdatensätze des Circles
- Spec der Runde 2, Abschnitt `### C11: Der volle Pfad im Fenstertitel`
- Spec der Runde 7 als Formvorlage
- die beiden offenen Defekte am Auslieferungsweg (`260813-0026`, `260812-1628`)
- am Baum: `fenstertitel.rs`, `appkit/fenster.rs`, `appkit/anwendung.rs`, `appkit/aufteilung.rs`, `appkit/menue.rs`, `menuemodell.rs`, `belegungsmodell.rs`, `xtask/src/release.rs`, `xtask/src/bundle.rs`, `resources/Info.plist`, `resources/default-keymap.toml`, `Cargo.toml`, `README.md`
- `git tag -l`, `git status --porcelain`, `xcrun stapler validate target/KRK.app`

## Erhoben

- Tags: null, bei sieben geschlossenen Runden
- Version einquellig in `[workspace.package]` auf `0.1.0`, von `krk-ui` über `version.workspace = true` geerbt
- `xtask` ruft kein `git`; acht andere Programme über `Command::new`
- `NSTitlebarAccessoryViewController` kommt unter `crates/` nicht vor
- Aufzählungen: `Kommando` 76, `Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5, `Funktionsbereich` 9; Belegung 82 Funktionen mit 88 Kombinationen; `appkit/mod.rs` 27 Modulnamen
- Menü: genau ein Sonderposten und ein Trenner, festgehalten von der Probe `die_leiste_traegt_genau_einen_zusatz`
- verfolgter Arbeitsbaum: sieben Änderungen, alle unter `fusion-workbench/`
- unter `target/KRK.app` liegt ein beglaubigtes Bündel mit angeheftetem Ticket

## Ergebnis

Spec unter `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1037_o_spec-titelleiste-fuehrt-version-und-semantische-tags.md`, sechs Fähigkeiten:

| | Fähigkeit | Kriterien |
|---|---|---|
| C1 | Namen und Version links in der Titelleiste | 11 |
| C2 | C11 der Runde 2, fortgeschrieben | 11, davon 2 geändert |
| C3 | Semantische Versionstags und die Prüfung in `cargo xtask release` | 15 |
| C4 | Der Abschnitt über die Versionsstufen in `README.md` | 7 |
| C5 | Der Eintrag „Über KRK" im Anwendungsmenü | 7 |
| C6 | Was der Bau erzwingt | 8 |

Zwei Mermaid-Diagramme: die eine Zahl mit ihren drei Abnehmern und dem Prüfer, und der Auslieferungsweg mit der neuen Station vor allen teuren.

### Die beiden verlangten Entscheidungen

**Der Defekt `260813-0026` bleibt draußen, ausdrücklich und mit benanntem Preis.** Begründung im Spec unter `## Der offene Defekt am Auslieferungsort bleibt ausdrücklich draußen`: die Runde beantwortet, ob der Stand benannt ist, aus dem gebaut wird; der Defekt beantwortet, wo das Ergebnis liegt und wer es überschreiben darf. Der zweite hat drei Zuschnitte mit verschiedenen Kosten, und keinen davon hat der Nutzer gewählt. Der Preis des Draußenbleibens steht im Spec: nach der Runde sagt die Prüfung beim Bau, dass das Bündel einen benannten Stand trägt, und der nächste Entwicklungsbau macht am selben Ort ein unbenanntes daraus.

**C11 wird fortgeschrieben und nicht ergänzt.** Alle elf Kriterien stehen im Spec als C2 im Wortlaut nach dieser Runde. Geändert sind das erste (der Titel trägt keinen Namen; Name und Version stehen daneben) und das neunte (macOS kürzt früher, weil der linke Bereich Breite nimmt; die Zusage bleibt gehalten, weil sie das Kürzen durch KRK ausschließt).

### Ein neuer Entscheidungsdatensatz

`decisions/260813-1037_o_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`. Der Über-Dialog ist kein Blatt: `blatt_steht` fragt `NSWindow::attachedSheet` und sieht ihn nicht, `ersthelfer_gehoert_appkit` fragt nach der Klasse des Ersthelfers im Schlüsselfenster. Ob ein Tastendruck damit einen Befehl im Fenster dahinter auslöst, hängt daran, welchen Ersthelfer AppKit im Panel einsetzt, und das ist ungemessen. Die Runde legt die Lücke nicht an, sie legt eine zweite Stelle an, an der sie eintritt; die erste ist der Freigabedialog der Runde 6 mit einem offenen Defekt. Empfehlung: die Lücke einmal und allgemein schließen. Der Spec fährt auf dieser Empfehlung und hält nicht an.

## Nicht getan

- Der Spec-Eintrag im Circle-Datensatz ist **nicht** gesetzt. Das Feld `**Active spec/plan:**` steht weiter auf `(none yet)`; der Shaper darf den Datensatz außerhalb des Aktivierungsmodus nicht anfassen. Orchestrator oder Nutzer tragen den Pfad nach.
- Keine Klärungsrunde, keine Rückfrage an den Nutzer.
- Kein Plan. Die Bauart entscheidet der Planner.

## Randnotiz

`CLAUDE.md` ist an mehreren Zahlen veraltet: es nennt vier gefahrene Runden, es sind sieben; es nennt 68 `Kommando`-Varianten, es sind 76. Der Befund gehört nicht in diese Runde und ist hier nur festgehalten.

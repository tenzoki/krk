# Orchestrator-Sitzung — 260823-1424

**Directive:** KRK 1.0.0 ausliefern, und vorher die Deckungslücke schließen, die die Sitzung
`260823-0442` hinterlassen hat.
**Mode:** custom
**Status:** Läuft

## Ausgangslage

| Größe | Wert |
|---|---|
| git HEAD | `b58e9d1` |
| Version in `Cargo.toml` | 0.5.6, jüngster Tag `v0.5.6` |
| Arbeitsbaum | sauber |
| `gh` | 2.98.0, vorhanden (Station 1 verlangt es) |
| Turn-Budget | 12 |
| Ungedeckt | `28cbb7b..HEAD`, acht Commits, davon einer mit Code (`52fba42`, sechs Dateien) |

## Die zwei Nutzerentscheidungen zum Start

1. **Die Zahl ist 1.0.0**, nicht 0.6.0. Der Nutzer folgt damit der Versionsregel seiner eigenen
   `README.md` wörtlich: Major steigt, wenn „eine Datei unter `~/Library/Application Support/KRK/`
   nicht mehr gelesen wird, wie sie geschrieben wurde". Genau das tut die Umbenennung von
   `editor_aus_vorschau` auf `editor_rundweg` aus `28cbb7b` — eine bestehende `keymap.toml` wird
   beim Start vollständig abgewiesen (`crates/krk-core/src/tasten/belegung.rs:1423`). Dass auf
   keiner der Maschinen des Nutzers eine solche Datei liegt, ändert die Eigenschaft nicht, nur
   ihren Schaden. Eine 0.x-Sonderregel führt die README nicht.
2. **Erst durchsehen, dann ausliefern.** Der Nutzer hat die Durchsicht des ungedeckten Bereichs
   der Auslieferung vorgezogen, mit der Begründung, dass Station 8 sich nicht zurücknehmen lässt.

## Verlauf

(wird fortgeschrieben)

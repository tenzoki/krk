cmd+y und shift+cmd+y lösen nichts aus, f3 schon

---

Der Nutzer hat am 260807-2112 am laufenden Bündel festgestellt: **`cmd+y` und
`shift+cmd+y` haben keine Wirkung.** `f3` wirkt.

---

## Was in der Belegung steht

Beide Kürzel sind belegt, und zwar in `resources/default-keymap.toml`:

| Zeile | Kennung | Tasten |
|---|---|---|
| 101 | `vorschau_umschalten` | `["f3", "cmd+y"]` |
| 349 | `fokus_vorschau` | `["shift+cmd+y"]` |

Die Taste `y` selbst steht in der Tabelle des Parsers
(`crates/krk-core/src/tasten/parser.rs:209`, `kVK_ANSI_Y`, Code 16). Sie ist
also nicht unbekannt.

## Warum der Befehl und sein Empfänger ausscheiden

`f3` und `cmd+y` lösen **denselben** Befehl aus, `vorschau_umschalten`. `f3`
wirkt. Damit sind der Befehl, seine Zuordnung und die Stelle, die ihn ausführt,
belegt in Ordnung. Was fehlschlägt, liegt auf dem Weg vom Tastendruck zum
Nachschlagen, und zwar nur für die Form mit Zusatztaste.

## Die Vermutung

`inference:`, nicht gemessen. Beide fehlschlagenden Kürzel tragen `cmd`, das
wirkende trägt keine Zusatztaste. Das ist der schärfste Unterschied, den der
Dateibestand hergibt.

Zwei Stellen kommen dafür in Frage, und der Modulkopf von
`crates/krk-ui/src/appkit/menue.rs` beschreibt genau ihr Zusammenspiel: der
Ereignisabgriff sieht den Tastendruck **vor** der Menübehandlung, kehrt bei
Fokus in einem Textfeld sofort zurück und reicht weiter, sonst schlägt er in der
Belegung nach.

1. **Das Menü greift `cmd+y` ab, bevor die Belegung es sieht.** Zu prüfen mit
   `make menue`, das die Kürzel der laufenden Anwendung ausgibt.
2. **Die Normalisierung der Zusatztasten** (`crates/krk-core/src/tasten/normalisierung.rs:181`,
   `normalisieren`) liefert für `cmd` eine Maske, die beim Nachschlagen nicht
   trifft.

Welche der beiden zutrifft, ist nicht geprüft. Die Reihenfolge oben ist eine
Rangfolge nach Aufwand der Prüfung, keine nach Wahrscheinlichkeit.

## Was daran hängt

`shift+cmd+y` ist laut der Übergabe an die Editor-Runde
(`shared/history/260807-1930-uebergabe-an-die-editor-runde.md`, Abschnitt „Die
Anzeige steht, die Bearbeitung fehlt") **der einzige Tastenweg in das
Vorschaufenster**. Ohne ihn sind die vier Tabbefehle aus C1 dort nur mit der
Maus erreichbar, und der Entscheidungsdatensatz
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md`
hat genau diesen Weg beschlossen.

Für `vorschau_umschalten` bleibt `f3` als zweiter Weg, dort fehlt nur das
Kürzel.

**Aufgefallen bei:** eigener Bedienung durch den Nutzer am 260807-2112, während
der Klärung der Directive für die Editor-Runde.

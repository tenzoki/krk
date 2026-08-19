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

---

Resolved: Keiner der beiden Verdächtigen traf zu. Die Ursache liegt nicht im
Programm, sondern in der Tastaturbelegung des Geräts, und sie ist im Projekt
bereits einmal gefunden worden. Belegt am Code am 260808-0916, Schritt S1 des
Plans `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`.

## Der erste Verdächtige scheidet aus: das Menü greift nichts ab

Das Hauptmenü trägt sieben Einträge (`crates/krk-ui/src/appkit/menue.rs:184-252`):
`beenden`, `text_ausschneiden`, `text_kopieren`, `text_einfuegen`,
`text_alles_auswaehlen`, `fenster_einblenden`, `fenster_schliessen`. Ihre Kürzel
holt der Menüaufbau aus der Belegung, und sie lauten dort `cmd+q`, `cmd+x`,
`cmd+c`, `cmd+v`, `cmd+a`, `cmd+n` und `shift+cmd+w`
(`resources/default-keymap.toml`, Zeilen 378, 387, 500, 506, 512, 525, 533).
Kein Eintrag trägt ein `y`.

Die Reihenfolge schlösse es ohnehin aus: der lokale Ereignisabgriff sieht den
Tastendruck vor `NSApplication::sendEvent:` und damit vor jedem Menükürzel
(`crates/krk-ui/src/appkit/menue.rs:31-42`).

## Der zweite Verdächtige scheidet aus: die Normalisierung vergleicht `u8` gegen `u8`

`normalisieren` liest genau vier Bits aus den rohen Flaggen und wirft
Feststelltaste, Zehnerblock, Hilfe und Funktionstastenbit weg
(`crates/krk-core/src/tasten/normalisierung.rs:181-196`). Auf der anderen Seite
des Vergleichs steht dieselbe Maskenform aus `Kombination::lesen`
(`crates/krk-core/src/tasten/parser.rs:369-410`). Eine rohe AppKit-Maske kommt
im Vergleich nicht vor.

Der Prüfstein: `f3` trägt am Referenzgerät das Funktionstastenbit
(`spikes/fn-tasten/messung-A.txt`, `roh=0x00800100`) und wirkt trotzdem. Ein
roher Maskenvergleich ließe `f3` ebenso scheitern.

## Die Ursache

KRK belegt den virtuellen Tastencode und nicht das gemeldete Zeichen; das ist
die Festlegung aus C3 der Runde 1, und für die Funktionstasten ist sie richtig.
Ein Tastencode benennt eine **Stelle** auf der Tastatur
(`crates/krk-core/src/tasten/parser.rs:105-107`). Die Stelle `kVK_ANSI_Y` trägt
den Code 16 (`parser.rs:209`), und auf einer deutschen Tastatur steht dort ein
**Z**. Wer die Taste mit der Aufschrift Y drückt, erzeugt Code 6, also
`kVK_ANSI_Z` (`parser.rs:210`), und dieser Code steht in der ganzen
Auslieferungsbelegung in keiner Tastenliste.

`cmd+y` und `shift+cmd+y` wirken deshalb, aber auf der Taste mit der Aufschrift
Z. Kein Programmteil kann daran etwas ändern, ohne die Festlegung aus C3
anzufassen: aus einem Tastencode ist nicht ableitbar, welcher Buchstabe auf der
Taste steht.

Die Probe
`die_y_kuerzel_liegen_auf_kvk_ansi_y_und_die_stelle_kvk_ansi_z_ist_unbelegt` in
`crates/krk-core/tests/belegung.rs` hält beide Aussagen fest, aus denen das
folgt: `cmd+y` und `shift+cmd+y` benennen die Stelle `kVK_ANSI_Y` mit Code 16,
und die Stelle `kVK_ANSI_Z` mit Code 6 trägt unter keiner Zusatztaste eine
Funktion.

## Was daraus folgt, entscheidet der Nutzer

Derselbe Sachverhalt ist am 260803 schon einmal gefunden und am 260804-0830 mit
dem Grund geschlossen worden, `f3` sei der Hauptweg und `cmd+y` nur der zweite:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2317_*_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md`.
Dieser Grund trägt seit dem 260807 nicht mehr, weil `fokus_vorschau`
(`resources/default-keymap.toml:349`) nur `shift+cmd+y` trägt und keinen zweiten
Weg hat.

Die Wahl zwischen den drei Wegen liegt als
`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`
vor und wird in Schritt S2 desselben Plans umgesetzt. Dieser Defekt ist damit
abgeschlossen: er fragte, warum nichts ausgelöst wird, und die Antwort steht
oben.

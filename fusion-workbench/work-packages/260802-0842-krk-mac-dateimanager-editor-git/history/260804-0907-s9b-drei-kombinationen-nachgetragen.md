# S9b: Drei Kombinationen in der Auslieferungsbelegung nachgetragen

---
**Datum:** 260804-0907
**Ausführender:** ontocoder
**Status:** Complete
**Schritt:** S9b aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`
**Geänderte Datei:** `resources/default-keymap.toml`
**Commit:** keiner — der Auftrag verlangte ausdrücklich, nicht zu committen

---

## Was getan wurde

Drei neue `[[funktion]]`-Blöcke in `resources/default-keymap.toml`, nach dem
Aufbau und in der Kombinationsschreibweise, die der Kopfkommentar der Datei
festlegt. Kein vorhandener Block ist angefasst.

| `id` | `name` | `tasten` | Abschnitt |
|---|---|---|---|
| `fenster_einblenden` | Fenster einblenden | `["cmd+n"]` | C7 |
| `zwischenablage_ansehen` | Zwischenablage ansehen | `["shift+f3"]` | C10 (neu) |
| `zwischenablage_springen` | Zum Inhalt der Zwischenablage springen | `["opt+cmd+g"]` | C10 (neu) |

Die Datei wächst damit von 46 auf 49 Funktionen und von 52 auf 55
Kombinationen, genau wie S9b es ansagt.

## Wo die drei Blöcke stehen

`fenster_einblenden` steht im vorhandenen Abschnitt "C7: Sichtbarkeit und
Breiten der Bereiche", hinter `zweites_fenster_umschalten` und vor
`bereich_verbreitern`. Der Abschnitt ist in sich nach seiner Überschrift
geordnet: erst die Sichtbarkeit, dann die Breiten. Der neue Eintrag ist eine
Sichtbarkeitsfunktion und gehört deshalb vor den Bruch, nicht ans Ende des
Abschnitts.

Für C10 ist ein eigener Abschnitt am Dateiende entstanden, im Stil der
vorhandenen: eine Trennlinie mit Überschrift, darunter der erklärende
Kommentar, darunter die Einträge. Am Ende, weil die Abschnitte der Datei der
Nummer ihrer Fähigkeit folgen und C10 die höchste ist.

Der Abschnittskommentar hält drei Dinge fest, alle aus dem Spec und aus dem
Entscheidungsdatensatz belegt und nicht aus dem Auftragstext abgeleitet:

- Was die beiden Funktionen tun, einschließlich der drei Ausgänge des Sprungs
  (lokaler Pfad, `http`- oder `https`-Adresse, nichts Verwertbares) und des
  Einblendens des Vorschaufensters (C10, Abnahmekriterien 1, 3 und 8).
- Dass die Auswertung **Text und Dateiverweis** liest. Nutzerentscheid vom
  260804, `decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md`.
  Der Kommentar zitiert den Datensatz mit Pfad.
- Dass Cmd+C und Cmd+V davon unberührt und ab Werk frei bleiben. C10 liest nur
  und legt nichts ab; der Kopfkommentar der Datei sagt dasselbe an anderer
  Stelle, und ein neuer Zwischenablage-Abschnitt ohne diesen Satz lädt zum
  Gegenschluss ein.

Die Einträge selbst tragen je einen Zeilenkommentar mit der Begründung ihrer
Kombination, wie sie C3 des Specs im Absatz vom 260804-0830 gibt: `shift+f3`
als F3 mit gewechselter Quelle, parallel zu `f6` gegen `shift+f6`; `opt+cmd+g`
neben `shift+cmd+g`, der Pfadeingabe von Hand. Bei `fenster_einblenden` steht,
warum die Beschriftung nicht "Neues Fenster" heißt.

## Abnahme

**Gültiges TOML.** Beide Rust-Prüfungen, die die Datei über `include_str!`
einkompilieren, parsen sie fehlerfrei; ein Parserfehler hätte jede der 26
Prüfungen umgeworfen statt einer.

**Blockzahl.** `grep -c '^\[\[funktion\]\]' resources/default-keymap.toml`
liefert `49`.

**Genau drei neue Blöcke.** `git diff --stat` zeigt für die Datei
`39 insertions(+)` und keine Löschung. Der Diff besteht aus zwei Hunks, beide
reine Einfügungen.

**Konfliktfreiheit, am vollständigen Eintrag geprüft.** Ein Skript liest jede
Tastenliste als Liste ganzer Einträge und zählt, wie viele Funktionen jede
Kombination beanspruchen: keine Kombination erscheint bei zwei Funktionen.
`cmd+n`, `shift+f3` und `opt+cmd+g` treffen je genau eine Funktion.
`shift+delete`, `cmd+c` und `cmd+v` kommen in keiner Tastenliste vor. Die
Prüfung auf Teilzeichenketten hätte hier wieder Fehlalarm gegeben:
`shift+cmd+v` enthält `cmd+v`, und `shift+cmd+g` enthält `cmd+g` nicht, wohl
aber `opt+cmd+g` das `cmd+g`. Maschinell bestätigt hat das
`die_auslieferungsbelegung_ist_konfliktfrei` aus
`crates/krk-core/tests/belegung.rs`, die grün durchläuft.

**`cargo test -p krk-core --test belegung` beendet nicht mit 0.** 25 Prüfungen
laufen durch, eine fällt um, und der Grund liegt nicht in der Datei:
`crates/krk-core/tests/belegung.rs:488` schreibt die Zahl 46 fest und erwartet
sie von einer Datei, die jetzt 49 Funktionen trägt. Dieselbe Zahl steht ein
zweites Mal in `crates/krk-core/src/tasten/belegung.rs:578`, und die 52 der
Kombinationen ein drittes Mal in Zeile 584 derselben Datei; `cargo test
-p krk-core --lib` fällt deshalb ebenso mit einer von 26 Prüfungen um. Rust
liegt außerhalb des `ontocoder`, also ist das als Defekt abgelegt statt
behoben:
`issues/260804-0907_o_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`.

Die inhaltliche Aussage des Kriteriums hält trotzdem: die eigentliche Prüfung,
dass die eingebettete Auslieferungsbelegung konfliktfrei ist, läuft grün, und
ebenso `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`. Was
umfällt, sind allein zwei Zählprüfungen gegen die alte Größe der Datei.

## Abgelegte Defekte

- `issues/260804-0907_o_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`
  — die drei festen Zahlen im Rust-Code, für den `coder`. Blockiert das
  Abnahmekriterium von S9b.
- `issues/260804-0907_o_kopfkommentar-der-auslieferungsbelegung-nennt-c10-nicht.md`
  — der Kopfkommentar der Datei nennt als Quelle "Faehigkeiten C1 bis C7" und
  müsste C10 mitnennen. Nicht behoben, weil der Auftrag den Eingriff auf drei
  neue Blöcke begrenzt hat.
- `issues/260804-0907_o_zwischenablage-entscheidung-traegt-im-rumpf-noch-offen-und-zwei-antwortbloecke.md`
  — der Entscheidungsdatensatz zur Zwischenablage-Auswertung sagt im Kopf
  `Status: open`, obwohl der Dateiname `_a_` trägt, und hat zwei
  Abschlussblöcke übereinander.
- `issues/260804-0907_o_c10-sagt-nicht-welcher-bereich-den-fokus-haben-muss.md`
  — offene Frage aus dem Spec, siehe unten. Als Defekt abgelegt, weil der
  `ontocoder` keinen Schreibort für Entscheidungsdatensätze hat; der nächste
  Abgleich sollte sie nach `decisions/` umtragen.
- `issues/260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`
  — Shift+Cmd+W liegt allein im Menü und wird von der Konflikterkennung nicht
  gesehen, während sein Gegenstück Cmd+N seit heute in der Datei steht.

## Was beim Lesen von C10 aufgefallen ist

Zwei Punkte berühren die Belegung und sind noch nicht festgelegt. Beide sind
oben als Defekt abgelegt und stehen hier nur im Zusammenhang.

**Der Fokus.** Beide C10-Funktionen sagen nicht, in welchem Zustand der
Eingabefokus sie erreicht. `shift+f3` und `opt+cmd+g` stehen in der Datei als
gewöhnliche Belegungen, und der Abgriff aus S7 reicht jeden Tastendruck
weiter, gleich welcher Bereich den Fokus hat. C10 sagt, "das aktive
Dateifenster" wechsle den Ordner, sagt aber nicht, was gilt, wenn der Fokus in
der Lesezeichenleiste oder im Vorschaufenster liegt. C5 hat dieselbe Frage für
seine Funktionen ausdrücklich geregelt, und die Antwort steht als Kommentar im
C5-Abschnitt dieser Datei. Für C10 fehlt sie. Die Datei bindet das heute
nicht, weil sie Kombinationen führt und keine Fokusregeln; S13 und S19
brauchen die Antwort.

**Das Gegenstück zu Cmd+N.** Mit `fenster_einblenden` steht das Einblenden des
Fensters in der Belegungsdatei, das Schließen auf Shift+Cmd+W nach dem
beschlossenen Weg dagegen allein im Menü. Damit hat KRK eine Kombination, die
eine Funktion auslöst, ohne dass die Konflikterkennung aus C3 sie sieht oder
der Nutzer sie umbelegen kann. Der ältere Defekt zu Cmd+W benennt diese
Blindstelle, seine Auflösung vom 260804 schließt sie aber nicht.

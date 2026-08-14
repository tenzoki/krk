# Zwei Kommentare in `default-keymap.toml` beschreiben das Tippen noch als Sprungmarke

**Status:** Open
**Domain:** Auslieferungsbelegung, Kommentarprosa
**Filed by:** coder, beim Umsetzen von A2
**Executor:** `ontocoder`
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt A2

## Befund

Schritt A2 hat die Sprungmarke aus C2 der Runde 1 abgebaut. Zwei Kommentarzeilen in
`resources/default-keymap.toml` beschreiben das Tippen weiterhin als Sprungmarke und
sind damit still falsch geworden:

- **Zeile 95**, im Kopfkommentar über den Fokusvorbehalt: „… und in einer eigenen
  Abfrage für das getippte Zeichen der Sprungmarke". Der Zweig besteht unverändert; die
  Sprungmarke, nach der er benannt ist, nicht mehr. Er füllt seit B1 den Filtertext des
  sichtbaren Tabs.
- **Zeile 120**: „Das Tippen der Anfangsbuchstaben aus C2 ist keine Belegung und steht
  darum nicht hier". Die Aussage über die Belegung stimmt weiter und ist der Zweck des
  Absatzes; falsch ist allein „der Anfangsbuchstaben aus C2" — getippt wird jetzt in
  einen Filtertext, der auf Teilzeichenfolgen passt und nicht auf den Namensanfang.

## Warum das hier steht und nicht behoben ist

`resources/default-keymap.toml` gehört dem `ontocoder`; der `coder` fasst sie nicht an.
Beide Stellen sind Kommentarprosa, keine Daten: kein Wert, keine Kennung und keine
Kombination ändert sich, und die Datei bleibt bei 84 Funktionen.

## Vorschlag

Beide Sätze auf den Filtertext des sichtbaren Tabs umschreiben. Der Absatz um Zeile 120
behält seine Aussage und tauscht nur den Gegenstand: das Tippen ist weiterhin keine
Belegung, sondern der Rückfall für jede Taste ohne Zusatztaste, die keiner Funktion
zugeordnet ist.

Sieben Prosastellen der Ablage nennen die Zahl der Dateien und den Umfang von `leerbefund` falsch

---

Die Ablage führt seit der Runde 9 sechs Dateien, seit `073448e` zusätzlich die abgeleitete
Frage `Datei::leerbefund`. Fünf Doc-Kommentare sprechen weiter von „den vier Dateien", wo der
beschriebene Weg alle sechs annimmt, und zwei Stellen geben den Umfang der neuen
Fallunterscheidung enger an, als sie ist. Alle sieben sind einzeln gegen den Baum geprüft.

---

**Gemessen am Baumstand `e688238`.**

## Fünf Stellen sagen „vier Dateien" über einen Weg, der sechs annimmt

`Ablage::pfad` und `Zugang::pfad` nehmen ein beliebiges `Datei` entgegen und reichen es an
`Ablageort::datei` weiter (`crates/krk-core/src/ablage/mod.rs:471-473`, `:509-511`). Dass
tatsächlich alle sechs durchkommen, steht in derselben Datei: `text_laden` und `text_sichern`
rufen `self.pfad(welche)` mit einem `Datei::Zettel` (`mod.rs:658`, `:716`). `Ablageort::datei`
selbst sagt es richtig: „Der Pfad einer der sechs Dateien" (`pfade.rs:354`).

| Stelle | Wortlaut | Wahr ist |
|---|---|---|
| `mod.rs:45` | „`Ablage::pfad` liefert den Pfad einer der vier Dateien" | sechs |
| `mod.rs:425` | „Der Ablageordner mit den vier Dateien" | sechs |
| `mod.rs:427` | „Wer eine der vier Dateien anfassen will" | sechs |
| `mod.rs:467` | „Der Pfad einer der vier Dateien" (`Ablage::pfad`) | sechs |
| `mod.rs:508` | „Der Pfad einer der vier Dateien" (`Zugang::pfad`) | sechs |

**Drei benachbarte Stellen sind richtig und dürfen nicht mitgezogen werden:** `mod.rs:59`
(„Eine der vier Dateien entsteht einmal"), `:513` („Liest eine der vier Dateien", `laden`) und
`:599` („Schreibt eine der vier Dateien", `sichern`) sprechen über die vier TOML-Dateien, und
`laden` wie `sichern` weisen ein Textformat mit `debug_assert_eq!` ab.

**Diese fünf sind älter als der Turn.** `git log -S` nennt für zwei von ihnen `3caa2b7`
(260813, Runde 7); die zweite Zetteldatei kam mit der Runde 9 dazu und hat sie stehen lassen.
Der Datensatz steht hier, weil die Durchsicht des Turns 1 sie gefunden hat, nicht weil
`073448e` sie verursacht hätte.

## Zwei Stellen geben `Datei::leerbefund` enger an, als es ist

**`mod.rs:142`** — „`bookmarks.toml` trägt dort `Leerbefund::Beschaedigt` … **Die drei
übrigen** tragen `Leerbefund::Vorgabe`". Es sind fünf: `Datei::Belegung`, `Datei::Sitzung`,
`Datei::Einstellungen` **und** `Datei::Zettel(_)` mit seinen zwei Werten
(`pfade.rs:234-241`). Der Satz ist auf die vier TOML-Dateien zu lesen und wird dann richtig,
aber sein Bezugswort ist die Fallunterscheidung, die sechs Werte einordnet. `pfade.rs:224`
schreibt es vollständig aus („Die drei übrigen TOML-Dateien **und die zwei Zettel**"), und die
Auflösung des Ausgangsdefekts ebenfalls („die fünf übrigen Ablagedateien"). `mod.rs` ist die
eine Stelle mit der engeren Zahl.

**`pfade.rs:1-2`** — der Modulkopf zählt auf, was das Modul beantwortet: „Wo die sechs
Ablagedateien liegen, in welchen zwei Formaten sie stehen, und wie der Ordner beim ersten
Start entsteht." `Leerbefund` ist die dritte abgeleitete Frage und steht seit `073448e` in
dieser Datei, ohne in der Aufzählung vorzukommen. Der Kopf trägt für die zweite Frage einen
eigenen Abschnitt (`# Zwei Formate, und warum die Zettel kein TOML tragen`), für die neue
keinen.

## Eine Nebenbeobachtung, kein Befund

`Datei::leerbefund` beantwortet die Frage auch für `Datei::Zettel(_)`, und diese Antwort ist
unerreichbar: `Zugang::laden` ist der einzige Rufer (`mod.rs:566`) und weist ein Textformat
vorher ab. Der Doc-Kommentar begründet den Wert trotzdem sachlich („ein leerer Notizzettel ist
ein leerer Notizzettel", `pfade.rs:219-220`) und liest sich damit wie eine geltende Zusage.
Für eine Textdatei ist „kein einziger oberster Schlüssel" keine beantwortbare Frage — es gibt
dort keine Schlüssel. Die Vollständigkeit der Fallunterscheidung verlangt den Zweig; der
Kommentar könnte sagen, dass der Wert nie gelesen wird, statt ihn inhaltlich zu rechtfertigen.
Für eine siebte Ablagedatei im Textformat hielte der Übersetzer sonst eine Einordnung an, die
nichts steuert.

## Vorschlag

Die fünf Stellen der Tabelle auf „sechs" setzen und die drei benachbarten richtigen dabei
nicht anfassen. `mod.rs:142` auf „Die fünf übrigen" setzen oder den Bezug auf die vier
TOML-Dateien im Satz ausschreiben. Den Modulkopf von `pfade.rs` um die dritte Frage ergänzen.

**Schwere:** niedrig. Kein Fehlverhalten, aber dieses Projekt führt Abweichungen zwischen
Prosa und Code als Defekte, und die Zahl „vier" steht an genau den Stellen, an denen ein
Leser sich fragt, ob ein Zettel über `Ablage::pfad` erreichbar ist.

**Gefunden:** coderev, Durchsicht des Turns 1 am 260821-1023, Bereich `01d2365..e688238`

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:45`, `:142`, `:425`, `:427`, `:467`, `:508`,
`crates/krk-core/src/ablage/pfade.rs:1-2`, `:219-220`

**Domain:** code

---

## Nachtrag 260821-1401: die Zeilennummern stehen jetzt richtig hier

`d771ec6` hat `mod.rs` um Prosa verlängert und die sechs Fundstellen dieses Datensatzes nach
unten geschoben. Der Wortlaut aller sieben ist unverändert, der Befund gilt weiter. Am
Baumstand `d771ec6` nachgezählt:

| Stelle bei `e688238` | jetzt | Wortlaut zur Wiedererkennung |
|---|---|---|
| `mod.rs:45` | `:45` (unverschoben) | „`Ablage::pfad` liefert den Pfad einer der vier Dateien" |
| `mod.rs:142` | `:151` | „Die drei übrigen tragen `Leerbefund::Vorgabe`" |
| `mod.rs:425` | `:461` | „Der Ablageordner mit den vier Dateien" |
| `mod.rs:427` | `:463` | „Wer eine der vier Dateien anfassen will" |
| `mod.rs:467` | `:503` | „Der Pfad einer der vier Dateien" (`Ablage::pfad`) |
| `mod.rs:508` | `:544` | „Der Pfad einer der vier Dateien" (`Zugang::pfad`) |
| `pfade.rs:1-2`, `:219-220` | unverändert | `pfade.rs` hat `d771ec6` nicht angefasst |

**Die Verschiebungstabelle im geschlossenen Datensatz
`260821-1023_c_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`
nennt fünf davon um genau eine Zeile zu niedrig** (`:150`, `:460`, `:462`, `:502`, `:543`).
Wer sie übernimmt, landet auf einer Leerzeile, auf `///` oder auf `impl Zugang<'_> {`. Die
Tabelle darüber ist die geprüfte.

**Drei benachbarte richtige Stellen sind ebenfalls verschoben** und dürfen weiterhin nicht
mitgezogen werden: `:59` bleibt `:59`, `:513` steht auf `:549` (`laden`), `:599` auf `:645`
(`sichern`).

**Nachgetragen von:** coderev, Durchsicht des Commits `d771ec6` am 260821-1401, Bereich
`073448e..d771ec6`

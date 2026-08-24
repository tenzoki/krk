Zwei Doc-Kommentare in `datei.rs` tragen eine Messung an einem Werkbankdatensatz im Präsens

---

Schritt 4 hat die Begründung für `datei::anlesen` an zwei Stellen in
`crates/krk-core/src/text/datei.rs` mit einer Messung belegt: „der groesste Circle-Datensatz
dieser Werkbank ist 119.614 Bytes gross, und seine Zeile `## Directive` steht bei Byte 222"
(`:145-146` im Modulkopf, `:678-679` am Doc-Kommentar von `anlesen`). Beide Zahlen stimmen am
260824. Beide sprechen im Präsens über eine Datei der Werkbank, die der Quellbaum nicht
enthält, die keine Probe liest und die ein Archivlauf verschiebt.

---

**Gemessen am Baumstand `b76800b`, am 260824-1014 nachgezählt:**

```
119614  fusion-workbench/circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_d_circle.md
222:## Directive
```

Beides trifft heute zu.

**Die Datei ist gerade der wahrscheinlichste Archivkandidat der Werkbank.** Ihr Circle trägt
den Marker `_d_`: der Nutzer hat den Web-Betrachter am 260821-2202 abgesagt
(`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
Zurückgestellte Runden sind das, was `/fusion:cleanup --only archive` wegräumt. Danach ist der
zitierte Pfad tot, die Zahl unbelegbar, und ein späterer Leser kann nicht mehr entscheiden, ob
die Begründung je gestimmt hat.

**Das Projekt führt genau dieses Muster als Defekt.** `CLAUDE.md` schreibt an mehreren Stellen
aus, warum keine Zahl in Prosa steht, die sich mit dem Bestand ändert („**Eine Zahl steht hier
nicht:** sie fällt mit jedem Archivlauf"), und der Plan dieser Runde sagt es für die
Schwesterzahlen selbst: „**Die Zahlen dieser Werkbank stehen in keiner Probe.** 54 offene
Defekte, 82 Datensätze, 118 Verläufe und 18 Circle-Verzeichnisse sind Stände vom 260824 und
ändern sich mit jeder Sitzung." Die 119.614 ist eine Zahl derselben Sorte, und sie ist als
einzige nicht in einem Werkbanktext gelandet, sondern im ausgelieferten Quelltext.

**Was daran nicht der Befund ist.** Die Begründung selbst trägt: eine Hülle, die über der
Grenze abweist, kann keinen Titel aus dem Dateianfang liefern, und das gilt unabhängig davon,
welche Datei gerade die größte ist. Der Befund ist die Form der Aussage, nicht ihr Inhalt.

## Vorschlag

Die zwei Stellen so fassen, dass sie das Verhältnis nennen und nicht den Messwert: eine Datei,
die im Ganzen weit über der Grenze liegt, während der gesuchte Wert in ihren ersten
Hundertfünfzig Bytes steht. Wer den Beleg behalten will, nennt ihn einmal mit Datum und
Herkunft („am 260824 an dieser Werkbank gemessen"), statt ihn im Präsens zu behaupten — das ist
dieselbe Form, die die Kostenangaben in der Wurzel-`Cargo.toml` tragen („Am 260824 auf diesem
Geraet erhoben").

**Schwere:** niedrig. Kein Fehlverhalten, und die getragene Aussage bleibt richtig. Die Zahl
veraltet still, und nichts im Baum hält sie.

**Gefunden:** coderev, Durchsicht des Bereichs `278a008..b76800b` am 260824-1014.

**Betroffen:** `crates/krk-core/src/text/datei.rs:145-146`, `:678-679`

**Domain:** code

---
Resolved: Der Vorschlag, in beiden Teilen. Die Stelle im Modulkopf (`:145-146`) nennt jetzt allein
das Verhältnis und keine Zahl: „eine Datei kann im Ganzen weit über der Grenze liegen und ihn
trotzdem in ihren ersten hundert Bytes tragen". Die Stelle am Doc-Kommentar von `anlesen`
(`:678-679`) trägt dasselbe Verhältnis und den Beleg daneben, einmal, mit Datum und Herkunft —
„am 260824 war der größte Circle-Datensatz der Werkbank dieses Projekts 119.614 Bytes groß" —
samt dem Satz, warum die Zahl kein Präsens verträgt: der Datensatz liegt außerhalb des
Quellbaums, keine Probe liest ihn, und ein Archivlauf verschiebt ihn. Die Form ist die der
Kostenangaben in der Wurzel-`Cargo.toml` („Am 260824 auf diesem Geraet erhoben").

Die getragene Aussage ist unverändert: eine Hülle, die über der Grenze abweist, kann keinen Titel
aus dem Dateianfang liefern. Genau das war am Befund nicht zu beanstanden, und es steht jetzt
ohne die Zahl da, die es tragen sollte.

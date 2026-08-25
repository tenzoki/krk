`CLAUDE.md` nennt zwei Aufrufer von `ohne_warten_oeffnen`, der Baum trägt drei

---

Der Absatz „Die Typprüfung vor dem Öffnen einer Textdatei steht am Deskriptor und nicht am Pfad"
in `CLAUDE.md` sagt: „**Die Hülle hat zwei Aufrufer, und beide liegen seit der Runde 11 in
`krk-core/src/text/datei.rs`:** `lesen`, auf dem `oeffnen` für den Editor aufsetzt, und
`bis_zur_grenze_lesen` für die Vorschau und den Inhaltsfilter". Es sind drei. Der ungenannte ist
`anlesen`.

---

**Gemessen am Baumstand `428fbc4` am 260825-0727, beim Erkunden für den Plan der Runde 17.**
Nicht durch diese Sitzung entstanden: `anlesen` steht seit der Runde 16 im Baum.

## Was der Baum trägt

Der Satz nennt selbst das Kommando, mit dem er zu prüfen ist, und das Kommando widerlegt ihn:

```
$ grep -n 'sys::ohne_warten_oeffnen(' crates/krk-core/src/text/datei.rs
434:    let mut datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
620:    let datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
692:    let datei = match crate::verzeichnis::sys::ohne_warten_oeffnen(pfad) {
```

Die drei Zeilen liegen in `lesen` (434), `bis_zur_grenze_lesen` (620) und `anlesen` (692). Der
Modulkopf der Hülle selbst zählt ebenfalls falsch: `crates/krk-core/src/verzeichnis/sys.rs:821`
sagt „der zweite Aufrufer ist mit `260810-1247` dazugekommen" und kennt den dritten nicht.

## Warum die Zahl hier trägt und nicht bloß Zierat ist

Der Absatz ist die eine Stelle, an der ein Entwickler nachschlägt, wie in diesem Vorhaben eine
Datei geöffnet wird, ohne an einer benannten Röhre zu blockieren. Wer die Aufzählung für
vollständig hält, hält `anlesen` für einen Weg, der die Hülle **nicht** nimmt, und liest daraus
eine Ausnahme, die es nicht gibt. Dieselbe Falle hat das Vorhaben bei `Kommando` schon viermal
gestellt (`shared/issues/260812-2253_*`), und die Antwort war dort, die Zahl durch das
Zählkommando zu ersetzen.

## Vorschlag

Die Aufzählung der Aufrufer streichen und an ihre Stelle das Zählkommando setzen, das der Satz
ohnehin schon nennt. Dieselbe Bewegung, die `CLAUDE.md` für `Kommando` und für die
`#[must_use]`-Stellen bereits vollzogen hat: die Zahl wächst mit jeder Runde, die einen weiteren
Leser baut, und ein Plan dieser Runde legt einen vierten Aufrufer nahe (das Packen liest die
Quelldateien).

Der Modulkopf von `verzeichnis/sys.rs` trägt dieselbe Aussage ein zweites Mal und gehört mit
berichtigt; zwei Zählungen für eine Frage sind der Zustand, aus dem die Abweichung entsteht.

**Schwere:** niedrig. Keine Fehlfunktion, eine falsche Auskunft an der Stelle, an der sie
nachgeschlagen wird.

**Gefunden:** planner, beim Erkunden für den Plan der Runde 17 am 260825-0727

**Betroffen:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß";
`crates/krk-core/src/verzeichnis/sys.rs`, Modulkopf von `ohne_warten_oeffnen`

**Domain:** code

Also seen: 260825-1230 by reconciler — die Runde 17 hat den Befund verbreitert: der Baum trägt jetzt **fünf** Aufrufer, und zwei davon liegen außerhalb von `text/datei.rs` (`operation/zippen.rs:348`, `operation/entpacken.rs:118`). Damit ist neben der Zahl auch der Satzteil „beide liegen seit der Runde 11 in `krk-core/src/text/datei.rs`" falsch, und das Zählkommando, das derselbe Absatz mitgibt (`grep -n 'sys::ohne_warten_oeffnen(' crates/krk-core/src/text/datei.rs`), ist auf eine Datei verengt, die den Bestand nicht mehr hält. Der Modulkopf von `verzeichnis/sys.rs` ist in dieser Runde neu geschrieben worden (`299d1e1`, `ab74c9e`) und zählt an zwei Stellen weiterhin daneben: Zeile 52–53 nennt „die zwei ältesten … in `text/datei.rs`" und lässt `anlesen` erneut aus, Zeile 848–849 sagt „der dritte und der vierte mit der Runde 17", wo es der vierte und der fünfte sind. Die vom Vorschlag verlangte Bewegung ist im Modulkopf halb vollzogen: Zeile 49 nennt das Zählkommando bereits, die Aufzählung darunter widerspricht ihm.

---
**Zur Hälfte behoben am 260825, und der Marker bleibt deshalb auf `_o_`.**

**Die `CLAUDE.md`-Hälfte ist erledigt.** Der Absatz heißt jetzt „Die Prüfung dessen, was da
geöffnet wurde, steht am Deskriptor und nicht am Pfad" — die alte Überschrift sprach von einer
Textdatei, und zwei der Aufrufer öffnen keine. An die Stelle der Aufzählung ist das
Zählkommando getreten, und zwar das breite: `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src`
statt des auf `text/datei.rs` verengten, das der Absatz bisher mitgab. Der Satzteil „beide
liegen seit der Runde 11 in `krk-core/src/text/datei.rs`" ist ersetzt durch die Ortsangabe nach
Klassen: die Textwege dort, das Packen und das Entpacken unter `krk-core/src/operation/`.
Dazugekommen ist, warum ein weiterer Aufrufer die Typprüfung nicht in die Hülle zieht, sondern
sie jedes Mal besser begründet — mit den vier verschiedenen Antworten, die kein gemeinsamer
Rumpf geben könnte.

**Offen bleibt der Modulkopf von `crates/krk-core/src/verzeichnis/sys.rs`.** Er ist in der
Runde 17 neu geschrieben worden und zählt an zwei Stellen weiterhin daneben, wie der Zusatz
vom 260825-1230 es beschreibt: Zeile 52–53 nennt „die zwei aeltesten … in `text/datei.rs`" und
lässt `anlesen` aus, Zeile 848–849 sagt „der dritte und der vierte mit der Runde 17", wo es der
vierte und der fünfte sind. Am Baumstand `7ba5a20` nachgemessen:
`grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` gibt fünf Aufruferzeilen aus
(`text/datei.rs:434,620,692`, `operation/zippen.rs:348`, `operation/entpacken.rs:118`), dazu die
Definition, die Prüfzeile 1120 und die Zeile mit dem Zählkommando selbst. **Der Aufrufer in
`entpacken.rs` entgeht dem engeren Muster `sys::ohne_warten_oeffnen(`**, weil die Datei den
Namen über `use` hereinholt und ihn unqualifiziert ruft; wer mit dem engeren Muster zählt,
kommt auf vier statt fünf.

Notiert vom coder, der allein `CLAUDE.md` anfassen durfte; die Codedatei gehört einer
Sitzung, die den Baum gerade durchliest.

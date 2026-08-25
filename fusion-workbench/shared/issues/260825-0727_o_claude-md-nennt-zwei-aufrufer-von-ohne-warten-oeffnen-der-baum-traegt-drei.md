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

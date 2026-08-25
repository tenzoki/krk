# Der Modulkopf von `sys.rs` sagt zweimal, `ortszeit` habe keinen Rufer im Baum

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/verzeichnis/sys.rs:22-23` (das Kästchen), `:39-45` (der Absatz); `crates/krk-core/src/operation/zippen.rs:715`; `crates/krk-core/src/leseprofil/bausteine.rs:192`, `:788`; Commits `c0050bf`, `e922c9e`, `66c779c`

---

## Was ist

`c0050bf` hat `ortszeit` gebunden und in den Modulkopf zwei Stellen geschrieben, die damals
wahr waren und heute nicht mehr:

```
//! localtime_r(3)     ──> ortszeit             ──> (noch ohne Rufer im Baum,
//!                                                  siehe den Absatz dazu)
```

und

```
//! **Gerufen sind neun davon aus dem Baum, und die zehnte ist es noch nicht.**
//! [`ortszeit`] steht seit der Runde 18 bereit und bekommt ihre Rufer mit dem
//! Zeitstempel des Packens und mit der Datumszeile eines Leseprofils, beide in
//! derselben Runde und beide nach dieser Stelle. Bis dahin ruft sie allein die
//! Probe. Wer den Satz spaeter liest und die zwei Rufer im Baum findet,
//! streicht diesen Absatz; wer sie nicht findet, hat einen Rueckbau vor sich
//! und keinen toten Zweig.
```

Die zwei Rufer sind seit `e922c9e` und `66c779c` da: `operation::zippen::archivzeitpunkt`
(`zippen.rs:715`) und `leseprofil::bausteine::kalendertext` (`bausteine.rs:788`, `use` in
`:192`). Der Absatz sagt selbst, was dann zu tun ist, und es ist nicht getan worden.

## Warum das zählt

Der Absatz ist nicht bloß veraltet, er ist eine ausgeschriebene Anweisung an seinen nächsten
Leser, und die zwei Commits, die die Bedingung erfüllt haben, sind in derselben Runde
gelaufen. Wer den Kopf heute liest, hält den zehnten Aufruf für ungenutzten Code und ist
eingeladen, ihn zurückzubauen — genau die Lesart, die der Absatz für den anderen Fall
vorsieht. Dazu ist die Zeile im Kästchen die einzige der sechs, die keinen Aufrufer nennt,
obwohl sie zwei hat.

## Was zu tun wäre

Den Absatz streichen und die Kästchenzeile auf die Form der fünf anderen bringen, also mit
ihren Aufrufern:

```
//! localtime_r(3)     ──> ortszeit             ──> operation::zippen
//!                                             └─> leseprofil::bausteine
```

Eine Zahl der Rufer gehört dabei nicht in die Prosa: die Zeile zu `fcntl(2)` im selben Kopf
schreibt aus, warum, und nennt den `grep`, mit dem man zählt.

**Schwere:** gering. Kein Verhalten, aber eine Prosastelle, die zum Rückbau einlädt.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.

# Die Probe für die angezeigte Datei zählt `return Some(` und hängt damit an der Schreibweise

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C2.11; `crates/krk-ui/src/tabs.rs`, `die_angezeigte_datei_bleibt_bei_zwei_quellen`; `crates/krk-ui/src/angezeigtedatei.rs:52-57`; `crates/krk-ui/src/quellbaum.rs`, Abschnitt `# Was keine Zaehlung entscheiden kann`

---

## Befund

C2.11 sagt zu, `angezeigtedatei::welche` bekomme keine dritte Quelle. Die Probe zählt dafür die Zeichenfolge `return Some(` im Rumpf des Moduls:

```rust
let rumpf = quelltext.split("#[cfg(test)]").next()…;
assert_eq!(rumpf.matches("return Some(").count(), 2, …);
```

Heute steht jede der beiden Quellen tatsächlich als früher Rücksprung da (`angezeigtedatei.rs:53` und `:56`), und die Zahl stimmt. Eine dritte Quelle muss aber nicht in dieser Schreibweise dazukommen. Ein Wert am Ende der Funktion (`Some(pfad)` ohne `return`), ein `.or_else(…)`, ein `if let … else` mit einem Zweigwert oder ein `match` mit `Some(…)` als Armwert — keines davon zählt die Probe mit, und alle vier wären eine dritte Quelle.

Es ist genau die Blindheit, die `crates/krk-ui/src/quellbaum.rs` unter `# Was keine Zaehlung entscheiden kann` beschreibt und für die es dort eine Bauanleitung gibt: „Nach dem Gegenstand suchen, wo es geht, und nicht nach seinem Namen", „jede Schreibweise erfassen, die der Baum schon kennt", „die verbleibende Blindheit am Doc-Kommentar der Probe benennen". Der Doc-Kommentar dieser Probe sagt stattdessen, jede Quelle **sei** genau ein `return Some(`, und macht die heutige Schreibweise zur Regel, ohne dass irgendetwas sie hielte.

## Was zu tun wäre

Zwei Möglichkeiten, und die zweite ist billiger:

1. Die Nadel um die übrigen Schreibweisen erweitern, die der Baum kennt.
2. Die Blindheit im Doc-Kommentar benennen: die Probe hält gegen eine dritte Quelle **in der Form eines frühen Rücksprungs** und gegen keine andere. Dann sagt die Probe, was sie kann, und der nächste Leser weiß es.

Die Fallunterscheidung selbst ist über alle acht Kombinationen im Prüfmodul von `angezeigtedatei.rs` geprüft; dieser Befund betrifft allein die Zählung.

---
Resolved: Gefahren ist keine der beiden genannten Möglichkeiten, sondern die Bauform aus `quellbaum.rs`: nach dem Gegenstand suchen und nicht nach seinem Namen. `die_angezeigte_datei_bleibt_bei_zwei_quellen` liest keine Zeile Quelltext mehr und hält stattdessen zwei Zusicherungen.

**Die Signatur.** `welche` wird an einen Funktionszeiger mit genau diesen vier Eingaben gebunden. Eine dritte Quelle, die eine fünfte Eingabe braucht — die Auswahl der Dateiliste, der angezeigte Ordner —, hält damit den Bau an, statt still dazuzukommen.

**Die Antwort.** Über alle sechzehn Kombinationen der vier Eingaben ist das Ergebnis entweder `None` oder genau einer der beiden übergebenen Pfade. Eine Quelle außerhalb der Eingaben — ein Ivar, eine Umgebungsvariable, ein Blick ins Dateisystem — läge außerhalb dieser Menge und fällt heraus. Die Schreibweise spielt dabei keine Rolle mehr: ein Rumpfwert, ein `.or_else`, ein `match`-Armwert werden alle vier gleich behandelt, weil keiner von ihnen gelesen wird.

Die verbleibende Blindheit steht am Doc-Kommentar: eine dritte Quelle, die genau einen der beiden übergebenen Pfade liefert, sähe die Probe nicht — sie wäre von den beiden aber auch nicht zu unterscheiden, und C2.11 spricht über die Antwort. Welche Quelle in welcher Lage gewinnt, prüft `angezeigtedatei.rs` in seinem eigenen Probenmodul über die volle Tafel; daran ist nichts geändert.

Berührte Datei: `crates/krk-ui/src/tabs.rs`.

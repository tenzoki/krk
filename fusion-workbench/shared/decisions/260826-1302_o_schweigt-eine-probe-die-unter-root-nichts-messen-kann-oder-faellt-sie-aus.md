# Schweigt eine Probe, die unter `root` nichts messen kann, oder fällt sie aus?

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/tests/text.rs:734-758`, `:1225-1250` (die zwei, die schweigen); `crates/krk-core/tests/operation.rs:528-565`, `:770-812` (die zwei, die ausfallen); `crates/krk-core/tests/arbeitsbaum.rs:274-287` (die dritte Antwort: den Fall gar nicht über Rechte herstellen)

---

## Question

Vier Abnahmeproben des Kerns stellen ihren Prüffall über entzogene Rechte her — `chmod 0o000` beziehungsweise `0o500`. Unter `root` greift die Rechteprüfung des Systems nicht, und der hergestellte Fall tritt nicht ein. **Der Baum beantwortet das heute an zwei Stellen entgegengesetzt**, und keine der beiden Antworten steht irgendwo als Regel.

`text.rs` schweigt und kehrt zurück:

```rust
if fs::read(&gesperrt).is_ok() {
    // Unter root liest sich auch eine gesperrte Datei. Dann sagt die Probe
    // nichts aus, und eine Probe, die nichts aussagt, behauptet hier auch
    // nichts.
    eprintln!("uebersprungen: die Rechtesperre wirkt auf dieser Kennung nicht");
    return;
}
```

`operation.rs` prüft gar nicht erst und würde unter `root` schlicht rot: `ein_eintrag_ohne_leserecht_wird_uebersprungen_und_gemeldet` erwartet genau einen übersprungenen Eintrag mit dem Grund „keine Rechte" und bekäme null.

`arbeitsbaum.rs` geht einen dritten Weg und begründet ihn ausdrücklich (`:280-283`): es stellt den unentscheidbaren Zugriff über einen 300 Zeichen langen Namensbestandteil her statt über Rechte, „denn eine Probe mit `chmod 0o000` bestuende unter einem Lauf als `root` nicht … und behauptete dann still das Gegenteil".

Die Frage steht jetzt an, weil die Durchsicht R6 sie sichtbar gemacht hat und weil jede künftige Probe, die einen Rechtefall braucht, sich für eine der drei Formen entscheiden muss — heute nach Gefühl, weil keine Regel dasteht.

## Options

1. **Schweigen, wie `text.rs`.** Die Probe prüft ihre Voraussetzung und kehrt mit einer Zeile auf der Fehlerausgabe zurück, wenn sie nicht gilt.
   - Pro: Ein Lauf als `root` bleibt grün, statt vier Fehlschläge zu melden, die nichts über KRK sagen. Die Ausgabe nennt den Grund.
   - Contra: Ein stiller Weg durch die Probe ist genau die Klasse, die diese Durchsicht sonst als Befund meldet. `--nocapture` ist bei `cargo test` nicht gesetzt; die Zeile sieht niemand, solange die Probe grün ist. Wer versehentlich unter `root` misst, bekommt einen grünen Lauf über eine Zusage, die nicht gemessen wurde.

2. **Ausfallen, wie `operation.rs` es faktisch tut — aber mit Ansage.** Die Probe prüft ihre Voraussetzung und **bricht ab**, wenn sie nicht gilt: „dieser Lauf hat die Rechtesperre nicht; die Probe kann ihre Zusage nicht messen".
   - Pro: Kein stiller Weg. Ein Lauf, der eine Zusage nicht messen konnte, sagt es. Deckt sich mit der Form, die dieselben Dateien an einem Dutzend Stellen fahren (`assert!(geprueft > 0, …)`, „sonst belegt die Probe nichts").
   - Contra: Vier rote Proben bei jedem Lauf als `root`. Ob dieses Projekt je unter `root` gemessen wird, ist offen — heute tut es das nicht.

3. **Den Fall ohne Rechte herstellen, wie `arbeitsbaum.rs`.** Wo möglich, den Prüffall so bauen, dass er jeden Lauf gleich trifft.
   - Pro: Die Frage entfällt. Der Weg ist im Baum schon begangen und begründet.
   - Contra: Für „kein Leserecht" gibt es keinen zweiten Weg — das ist der Prüffall selbst, nicht seine Herstellung. Die Möglichkeit trägt für `arbeitsbaum.rs` und für keine der vier.

## Constraints

- Die vier Proben halten Zusagen, die sonst nichts hält: „ein Eintrag ohne Leserecht wird übersprungen und gemeldet" ist ein eigener Abnahmepunkt von C4.
- Der Baum wird heute nicht unter `root` gebaut oder geprüft; keine Anweisung in `README.md` oder `Makefile` verlangt es.
- Eine Antwort muss für alle vier gelten. Zwei Antworten in einer Prüfsammlung sind der heutige Zustand und der Grund für diesen Datensatz.

## Recommendation

Möglichkeit 2, mit einer Einschränkung. Die Begründung, die `text.rs` gibt — „eine Probe, die nichts aussagt, behauptet hier auch nichts" — ist richtig und führt trotzdem zum falschen Schluss: sie darf auch nicht *behaupten, gemessen zu haben*, und genau das tut ein grüner Lauf. Ein Abbruch mit klarem Text ist die Auskunft, die der Absatz eigentlich will.

Die Einschränkung: das gilt nur, wenn niemand vorhat, diesen Baum je unter `root` zu prüfen. Ist das anders, kehrt sich die Abwägung um, und dann ist Möglichkeit 1 richtig — aber dann gehört der übersprungene Fall gezählt und am Ende des Laufs gemeldet, nicht in eine Zeile geschrieben, die `cargo test` verschluckt.

Die Frage, welche der beiden Lagen gilt, kann nur der Nutzer beantworten.

# Drei Prüfkommentare nennen nach `Marke::Unbekannt` noch fünf Marken oder einen falschen Dateiinhalt

---
Mit `3c065f79` hat `Marke::ALLE` sechs Werte. Drei Kommentare in Proben beschreiben noch den Stand davor oder einen Inhalt, den der Helfer gar nicht schreibt.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261026-1947-reviewer-abschlussdurchsicht-git-markenspalte-cloud-dateien.md`

## Defekt

1. `crates/krk-core/src/verzeichnis/modell.rs`, Prüfmodul, Doc-Kommentar von `fuenf_marken`: „Die fuenf Marken in der Reihenfolge von [`Marke::ALLE`]“. `ALLE` hat seit `3c065f79` sechs Werte. Die Liste nimmt die ersten fünf in der Reihenfolge von `ALLE`, also nicht „die“ Marken von `ALLE`.
2. `crates/krk-core/tests/git.rs`, Zwischenüberschrift `// C5: die fuenf Markenzustaende` und Doc-Kommentar von `die_fuenf_zustaende_tragen_ihre_fuenf_buchstaben`. Die Probe prüft weiter genau fünf Zustände, und das stimmt, weil `?` ohne eingespritzte Platzhalterfrage nicht entsteht. Die Überschrift liest sich aber so, als gäbe es fünf Markenzustände. Sie sollte sagen, dass die Probe die fünf ohne Platzhalter prüft und `Unbekannt` unter „Eine Datei, die nur in der Cloud liegt“ geprüft wird.
3. `crates/krk-core/tests/git.rs`, Doc-Kommentar von `verfolgt`: „Jede Datei traegt `grund\n`, sieben Bytes“. Der Rumpf schreibt `"grund!\n"`. Sieben Bytes stimmt, der Wortlaut nicht (`grund\n` hat sechs). Die Proben (2) und (3) bauen auf der Länge auf: „gleich lang“ heißt sieben Bytes.

## Abnahme

- Die drei Kommentare stimmen mit dem Code überein. Keine Zahl wird behauptet, ohne aus der Liste daneben zu folgen (`rules/critical-stance.md` §5).
- `cargo test -p krk-core --test git` und `cargo test -p krk-core --lib verzeichnis::modell` grün.

# Vier Platzhalter nennen ihren ablösenden Schritt nicht, obwohl der Plan ihn führt

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs`, `crates/krk-ui/src/appkit/aufteilung.rs`
**Cross-references:** Plan S16, S18

---

## Der Befund

Turn 1 hat sieben Stellen mit einem Vorwärtsverweis versehen. Drei nennen ihren
Schritt, vier nicht.

**Mit Schrittnummer:**

| Stelle | Text |
|---|---|
| `appkit/anwendung.rs:1108` | "**S17 löst diese Zeile ab** und setzt den Ersthelfer auf die Textfläche des Editors." |
| `appkit/anwendung.rs:1562` | "**S17 löst diese Zeile ab** und reicht das Kommando an den Editor." |
| `leistenmodell.rs:346-351` | "**S39** teilt die Auswahl nach der Sorte auf … Bis **S38** das Anlegen bringt …" |
| `leistenmodell.rs:666-668` | "**S39 löst die Probe ab**" |

**Ohne Schrittnummer:**

| Stelle | Text | Der Schritt laut Plan |
|---|---|---|
| `fenstermodell.rs:19-21` | "der gegenseitige Ausschluss selbst kommt in einem späteren Schritt" | S18 |
| `fenstermodell.rs:275-276` | "mit einem Unterschied, den ein späterer Schritt trägt: das Einblenden des einen blendet das andere aus" | S18 |
| `aufteilung.rs:296-298` | "Ein Bereich, dessen Unteransicht die Aufteilung noch nicht trägt … Das trifft heute den Editor, dessen Textfläche ein späterer Schritt einhängt." | S16 |
| `aufteilung.rs:329-330` | "Bereiche, deren Unteransicht die Aufteilung noch nicht trägt, überspringt die Schleife." | S16 |

Der Plan selbst nennt beide Nummern. Zu `fenstermodell.rs:275` schreibt er unter
S13 wörtlich: "`umschalten(Bereich::Editor)` verhält sich wie `Bereich::Vorschau`,
**mit einem Unterschied, den S18 trägt**". Der Code hat den Satz übernommen und
die Nummer dabei fallen lassen.

## Warum das zählt

Ein Vorwärtsverweis ohne Nummer ist nicht nachschlagbar. Wer bei
`fenstermodell.rs:275` steht und wissen will, ob der Ausschluss noch aussteht
oder schon vergessen wurde, muss den Plan durchsuchen; bei `anwendung.rs:1108`
genügt ein `grep S17`. Die Runde hat beide Formen nebeneinander, und die
schwächere steht ausgerechnet an der Stelle, die eine Zusage aus C1 offen lässt
("Beide zugleich sichtbar zu haben ist über keinen Weg erreichbar" ist bis S18
nicht eingelöst).

Der vierte Eintrag, `aufteilung.rs:329-330`, nennt nicht einmal einen späteren
Schritt — er beschreibt das Überspringen als dauerhafte Eigenschaft der
Schleife, obwohl es der Zwischenstand bis S16 ist.

## Was zu tun ist

Die vier Stellen um ihre Schrittnummer ergänzen, in der Form, die
`anwendung.rs:1108` schon führt. Kein Verhalten ändert sich.

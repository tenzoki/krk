# Schuldet das CSV-Paket einen Abnahmelauf gegen die zehn Zusagen aus C8?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md

---

## Question

Das Paket berührt eine gemessene Strecke: L7, die Vorschau des ausgewählten Eintrags. Prüfordner A enthält `.csv`-Dateien (`crates/krk-bench/src/fixture.rs`, Liste `ENDUNGEN`). Für sie liest der Faden `krk-vorschau` künftig bis 1 MB an, zerlegt und richtet aus, statt allein `String::from_utf8` zu fragen, und eine CSV über 1 MB zeigt künftig ihren Anfang statt der Angaben. Beides liegt vor der Endbedingung von L7 (`laedt_noch`). Die übrigen neun Zusagen berühren keine geänderte Stelle: der Editor ist nicht gemessen, und `Lage` bekommt keinen neuen Feldtyp, nur einen weiteren Wert in `Editorform` und `Wirkungsbereich`.

## Options

1. **Messen.** Der Nutzer fährt den Abnahmelauf am Bündel im Vordergrund.
2. **Verzichten.** Kein Lauf für dieses Paket. L7 steht seit dem 260819 ohnehin auf den Gegenständen der späteren Messrunde und bleibt dort.

## Constraints

- Kein Agent kann den Lauf fahren; er verlangt KRK im Vordergrund.
- Die Antwort hält keinen Bauschritt an, nur das Schließen des Pakets.

## Recommendation

Möglichkeit 2. Die zusätzliche Arbeit ist eine lineare Zerlegung von höchstens 1 MB auf dem Faden; der Plan misst sie kopflos in Schritt 2 und nennt die Zeit im Bericht. Das entspricht der Lage beim sftp-Paket, wo der Nutzer bei breiterem Umbau verzichtet hat.

---
Answered: 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md — Empfehlung des Plans angenommen („ja“); ruled by user, Kai Stalmann <kai@stalmann.org>

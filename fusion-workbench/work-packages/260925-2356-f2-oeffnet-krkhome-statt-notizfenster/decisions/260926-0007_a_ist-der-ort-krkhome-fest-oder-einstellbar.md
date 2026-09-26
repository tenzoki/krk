# Ist der Ort ~/krkhome fest, oder lässt er sich einstellen?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md

---

## Question

Die Directive nennt `~/krkhome/` als Ort. Ein einstellbarer Ort erlaubte etwa einen Ordner in iCloud Drive oder Dropbox, damit Notizen und Aufgaben auf mehreren Macs dieselben sind. Die Entscheidung muss vor Stufe 1 fallen, weil sie bestimmt, woher der Befehl hinter F2 seinen Ordner nimmt, und weil ein nachträglich eingeführter Ort die Frage aufwirft, was mit dem Inhalt am alten Ort geschieht.

## Options

1. **Fest `~/krkhome/`.**
   - Pros: genau die Directive; nichts einzustellen, nichts zu erklären. Wer den Ordner synchronisieren will, kann ihn von Hand durch einen symbolischen Verweis ersetzen (Inferenz: KRK folgt Verweisen beim Hineingehen bereits).
   - Cons: ein Wunsch nach einem anderen Ort braucht eine spätere Arbeit.
2. **Ein Schlüssel in `settings.toml` mit `~/krkhome` als Vorgabe.**
   - Pros: der Ort ist frei wählbar, ohne neue Oberfläche; `settings.toml` ist die bestehende Datei für von Hand gepflegte Einstellungen, und die Startmeldung der Runde 24 meldet einen neuen Schlüssel dort bereits von selbst.
   - Cons: ein weiterer Schlüssel, dessen Fehlerfälle zu regeln sind (Ort existiert nicht, ist nicht beschreibbar, liegt auf einem nicht eingehängten Laufwerk). Wer den Ort ändert, findet seine alten Einträge nicht am neuen Ort; KRK verschiebt nichts.
3. **Fest in dieser Arbeit, die Einstellbarkeit als ausdrücklich spätere Arbeit vermerkt.**
   - Pros: wie 1, und die Frage ist aufgeschrieben statt vergessen.
   - Cons: wie 1.

## Constraints

- KRK schreibt `settings.toml` im Betrieb nie (`Datei::Einstellungen` in `crates/krk-core/src/ablage/pfade.rs`); ein Schlüssel dort ist nur von Hand zu setzen.
- Ein Ort in einem Synchronisierungsdienst bringt die Frage gleichzeitiger Änderungen auf zwei Macs mit; keine Möglichkeit hier beantwortet sie.

## Recommendation

Wir empfehlen Möglichkeit 3. Die Directive nennt den Ort ausdrücklich, und jede Einstellbarkeit zieht Fehlerfälle nach sich, die der Kern dieser Arbeit nicht braucht. Der Vermerk hält die Frage fest, und Möglichkeit 2 bleibt ohne Umbau nachrüstbar.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 3, fest `~/krkhome/`, Einstellbarkeit als spätere Arbeit; der Ordner wird auch über einen symbolischen Verweis erkannt; ruled by user, Kai Stalmann <kai@stalmann.org>

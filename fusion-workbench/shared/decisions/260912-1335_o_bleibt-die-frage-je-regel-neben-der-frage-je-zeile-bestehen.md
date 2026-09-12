# Bleibt `Regel::ist_wirkungslos` neben `Vorschauzeile::wird_umbenannt` bestehen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`, `260912-1149_*_was-geschieht-mit-einem-oeffentlichen-namen-ohne-rufer-im-betriebscode.md`

---

## Question

`Regel::ist_wirkungslos` (`crates/krk-core/src/stapelumbenennen/regel.rs`) beantwortet für die
ganze Regel dieselbe Frage, die `Vorschauzeile::wird_umbenannt`
(`crates/krk-core/src/stapelumbenennen/vorschau.rs:38-40`) je Zeile beantwortet: eine Regel ist
genau dann wirkungslos, wenn keine Zeile umbenannt wird. Zwei Antworten auf dieselbe Frage,
und `Vorschau::auszufuehren` benutzt die zweite zum Filtern, während die erste nur ihre eigene
Probe hat.

Der Datensatz `260826-1221_*` hat das am 260826 als Teilbefund festgehalten und offengelassen.
Die Sichtbarkeitsfrage daran ist am 260912 entschieden und in `00fb99a` umgesetzt — die
Methode trägt jetzt `pub(crate)` und `#[cfg(test)]`. **Die Doppelung ist davon nicht berührt**
und wird hier als eigene Frage geführt, weil sie keine Sichtbarkeitsfrage ist.

## Options

1. **`ist_wirkungslos` fällt.** Wer die Frage für die ganze Regel braucht, stellt sie über die
   Vorschau: keine Zeile wird umbenannt.
   - Pro: eine Antwort auf eine Frage, wie es dieses Projekt sonst hält.
   - Contra: die Probe, die heute an ihr hängt, muss über die Vorschau gehen und braucht dafür
     einen Bestand, den sie sich bauen muss; sie prüft heute die Regel allein.
2. **`ist_wirkungslos` wird auf die Vorschau zurückgeführt** statt eigenständig zu rechnen.
   - Pro: eine Quelle für die Antwort, der Name bleibt für den, der ihn lesbar findet.
   - Contra: eine Regel ohne Dateien hätte dann keine Vorschau und keine Antwort; heute
     antwortet sie aus ihren eigenen Feldern und braucht keine Dateien.
3. **Beide bleiben, und der Grund kommt an beide Stellen.** Die Fragen sind verwandt, aber
   nicht dieselbe: die eine rechnet aus den Feldern der Regel, die andere aus einem konkreten
   Namen.
   - Pro: billigster Zug, und die zwei Rechenwege haben verschiedene Voraussetzungen.
   - Contra: die Doppelung bleibt, und der nächste Leser stellt die Frage wieder.

## Constraints

- `Vorschauzeile::wird_umbenannt` hat einen Betriebsrufer (`Vorschau::auszufuehren`);
  `ist_wirkungslos` hat seit `00fb99a` nur noch die innere Probe und existiert im
  ausgelieferten Bau nicht mehr.
- Ein Verhalten ändert keine der drei Möglichkeiten. Es geht um die Frage, wie viele Wege
  dieselbe Auskunft geben.

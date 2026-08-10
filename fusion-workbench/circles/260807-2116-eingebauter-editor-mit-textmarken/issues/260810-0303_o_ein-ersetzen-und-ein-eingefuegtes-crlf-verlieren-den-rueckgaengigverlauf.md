# Ein Ersetzen und ein eingefügtes CRLF verlieren den Rückgängigverlauf

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, abgetrennt beim Beheben von `260809-1727`
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::stand_einsetzen`, `flaeche_richten`, `treffer_ersetzen`, `alle_treffer_ersetzen`)
**Cross-references:** `issues/260809-1727_c_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`, `issues/260810-0215_*_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`, S37

---

## Der Befund

`Editorbereich::stand_einsetzen` leert seit der Behebung von `260809-1727` den
Rückgängigstapel der Textfläche. Das ist beim Dateiwechsel richtig: ein Stapel,
der auf den Text der vorigen Datei zeigt, gehört weg. Zwei andere Wege gehen
durch dieselbe Funktion, ohne die Datei zu wechseln, und verlieren dabei den
Verlauf des Nutzers:

- **Ein Ersetzen** (`shift+cmd+r` und `ctrl+cmd+r`, S37) schreibt den geänderten
  Stand über `stand_erneuern` zurück.
- **Ein eingefügtes CRLF** aus einer Windows-Quelle bringt `flaeche_richten` auf
  denselben Weg (Behebung von `260810-0215`, Commit `d5993f1`).

Nach beiden tut ein `cmd+z` nichts. Vorher tat es das Falsche — es wirkte gegen
einen Stand, den die Fläche nicht mehr trug. Der Verlust ist damit die kleinere
der beiden Fehlwirkungen und ausdrücklich als Preis der Behebung angenommen,
nicht übersehen; die Doc-Kommentare von `stand_einsetzen` und `flaeche_richten`
halten ihn fest.

Am schwersten wiegt das Sammelersetzen: `ctrl+cmd+r` ändert eine ganze Datei in
einem Zug, und genau dort erwartet ein Nutzer, es zurücknehmen zu können.

## Warum die Behebung von 260809-1727 ihn nicht mit erledigt hat

Beide Fälle brauchen entgegengesetzte Behandlungen an derselben einen Stelle:

```
                    Rückgängigstapel danach
  Dateiwechsel  ──> muss leer sein      (sonst zeigt er auf eine andere Datei)
  Ersetzen      ──> soll gefüllt sein   (der Nutzer will das Ersetzen zurücknehmen)
  CRLF-Richten  ──> soll gefüllt sein   (der Nutzer will das Einfügen zurücknehmen)
```

`stand_einsetzen` ist die eine Stelle, die den Text der Fläche ersetzt, und sie
kennt ihren Anlass nicht. Der Zuschnitt der Behebung ist deshalb offen und
gehört in diesen Defekt, nicht in den behobenen.

## Was zu prüfen wäre

Ein rückgängigfähiger Schreibweg statt `setString:`, also
`shouldChangeTextInRange:replacementString:`, dann
`NSTextStorage::replaceCharactersInRange:withString:`, dann `didChangeText`.
**Drei Fragen hängen daran**, und keine ist beantwortet:

1. **`didChangeText` löst den Rückweg aus.** Es verschickt
   `NSTextDidChangeNotification`, der Delegierte ruft `text_zurueckschreiben`
   und damit `Editormodell::bearbeiten`. Eine frisch geöffnete Datei trüge
   danach sofort das Abweichungszeichen. Der Modulkopf von `editor.rs` führt
   diese Annahme ausdrücklich; sie zu halten verlangte eine Sperre um den
   eigenen Schreibvorgang.
2. **Der Dateiwechsel braucht das Gegenteil** und müsste den Stapel weiterhin
   leeren. Die eine Schreibstelle bekäme damit zwei Betriebsarten, und die
   Zusage "eine Stelle, ein Verhalten" fiele.
3. **Ein Ersetzen ist eine Änderung am Modell und nicht an der Fläche.** Der
   Stand geht durch `krk_core::text` und kommt gewandelt zurück; was AppKit als
   rückgängigfähigen Schritt aufzeichnete, wäre das Ergebnis und nicht der
   Schritt. Ob ein `cmd+z` darauf den Stand im Modell mitnimmt, ist offen.

Möglicherweise ist der richtige Ort dafür nicht AppKits Rückgängigverwaltung,
sondern ein Verlauf im `Editormodell`. Das entscheidet dieser Defekt nicht.

## Was heute hält

Kein Verlust von Text und keine falsche Wirkung: nach beiden Wegen trägt die
Fläche den Stand, den das Modell hält, und `cmd+z` tut nichts.

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

---
Resolved: Das Ersetzen aus S37 traegt seinen Umbau seit dem 260810 als
Rueckgaengig-Handlung; die CRLF-Haelfte bleibt und hat einen eigenen Datensatz.

**Der Zuschnitt, den dieser Defekt offengelassen hat.** Der Anlass kommt jetzt
als Wert in die eine Schreibstelle herein, statt dort geraten zu werden: der
neue Typ `Verlauf` in `crates/krk-ui/src/appkit/editor.rs` hat die beiden Werte
`Faellt` und `Traegt(Umkehrpunkt)`, `stand_einsetzen` und `stand_erneuern`
nehmen ihn, und alle sieben Aufrufstellen nennen ihn. Die Aufzaehlung ist
vollstaendig und ohne Auffangzweig, ein achter Anlass haelt also den Bau an.

```
  Anlass                     Verlauf danach
  Dateiwechsel, Schliessen ─> Faellt   der Verlauf gehoerte einer anderen Datei
  Ersetzen (S37)           ─> Traegt   der Nutzer nimmt das Ersetzen zurueck
  CRLF-Richten             ─> Faellt   siehe unten
```

`treffer_ersetzen` und `alle_treffer_ersetzen` nehmen vor dem Ruf ins Modell
einen `Umkehrpunkt` — den gehaltenen Stand und die Auswahl der Flaeche — und
melden ihn ueber `umkehrung_anmelden` beim Rueckgaengigverwalter der Flaeche an.
`umkehren` stellt ihn her, meldet den Gegenweg an (waehrend eines Rueckgaengig
legt `NSUndoManager` jede Anmeldung auf den Wiederherstellungsstapel) und setzt
die Auswahl beschnitten zurueck.

**Die drei Fragen aus "Was zu prueefen waere" sind damit beantwortet, und zwei
davon anders als vermutet.** Der Weg ist **nicht**
`shouldChangeTextInRange:replacementString:` geworden:

1. **`didChangeText` loest den Rueckweg aus.** Richtig, und genau deshalb faellt
   der Weg fort: er brauchte eine Sperre um den eigenen Schreibvorgang, und der
   nachgezogene Ruf von `Editormodell::bearbeiten` kostete ein zweites Mal den
   ganzen Stand — die Kette, die `260810-0424` bemaengelt, um zwei Durchlaeufe
   laenger. Die eigene Handlung am Verwalter kommt ohne beides aus: `setString:`
   meldet nichts an (gemessen, siehe unten) und ruft den Delegierten nicht.
2. **Der Dateiwechsel braucht das Gegenteil.** Er bekommt es, und die eine
   Schreibstelle bekommt dafuer keine zwei Betriebsarten, sondern einen
   Parameter. Die Zusage "eine Stelle, ein Verhalten" war die falsche; richtig
   ist "eine Stelle, und der Anlass steht als Wert daneben", wie bei
   `Wirkungsbereich`, `Bereich` und `Fokus`.
3. **Nimmt ein `cmd+z` den Stand im Modell mit?** Ja, und ohne Umweg: die
   Handlung laeuft ueber `Editormodell::bearbeiten` und ist damit eine Aenderung
   am Modell, nicht an der Flaeche. Die Vermutung am Ende dieses Datensatzes —
   der richtige Ort sei ein Verlauf im `Editormodell` und nicht AppKits
   Rueckgaengigverwaltung — trifft die Sache halb: der **Stapel** ist AppKits,
   die **Handlung** ist eine im Modell. Ein zweiter Stapel daneben truege den
   Umbau in einen anderen als das Tippen, und ein `cmd+z` nahme die beiden dann
   in der falschen Reihenfolge zurueck.

**Gemessen am 260810 auf macOS 15.7.7 (Build 24G720)**, weil zwei Annahmen
dieses Defekts daran haengen:

- `NSTextView` mit `allowsUndo` meldet nach einem `setString:` keine Handlung an
  (`canUndo` bleibt `false`). Der Stapel bleibt also unberuehrt, und eine eigene
  Handlung ist die einzige, die den Umbau abbildet.
- `undo:` beantwortet in der ganzen Antwortkette allein `NSWindow`, und
  `NSWindow` nimmt dabei den Verwalter des **Ersthelfers**. Die angemeldete
  Handlung ist damit vom Menueeintrag aus erreichbar.

**Was nicht behoben ist und warum.** Die CRLF-Haelfte bleibt: der Text, den die
Flaeche vor dem Richten trug, ist kein gueltiger Stand (er traegt das `\r`), und
der Stand vor dem Einfuegen ist an dieser Stelle schon ueberschrieben; ihn
vorzuhalten hiesse, je Tastendruck den ganzen Stand zu kopieren. Was der Nutzer
zurueckhaben will, ist ohnehin das Einfuegen und nicht die Wandlung, und das
gehoert an den Eingang der Flaeche. Der Rest ist als
`260810-1044_o_ein-eingefuegtes-crlf-bleibt-nicht-ruecknehmbar-und-der-grund-liegt-am-eingang-der-flaeche.md`
abgelegt, mit der Begruendung im Einzelnen an `flaeche_richten`.

Was Nutzerarbeit bleibt: die Wirkung am laufenden Buendel, also dass ein `cmd+z`
nach `shift+cmd+r` und nach `ctrl+cmd+r` den vorigen Stand samt Schreibmarke
zeigt und ein zweites den Anschlag davor.

Abnahme: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets`, `cargo fmt -p krk-ui --check` — alle
exit 0.

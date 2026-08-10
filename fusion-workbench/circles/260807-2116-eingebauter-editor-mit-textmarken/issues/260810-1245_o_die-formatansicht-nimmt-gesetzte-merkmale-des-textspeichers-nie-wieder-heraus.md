Die Formatansicht nimmt gesetzte Merkmale des Textspeichers nie wieder heraus
---
`formatierung_anwenden` **fuegt** die Auszeichnungen des Textspeichers hinzu und nimmt keine heraus. Wer eine Markdown-Ueberschrift zur gewoehnlichen Zeile macht, sieht sie weiter gross und fett. Die eine Stelle, die zuruecknimmt, laeuft nur bei vier Anlaessen und nicht beim Tippen — und sie nimmt ohnehin nur den Absatzstil heraus, nicht die Schrift.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`
**Alter:** aelter als dieser Diff; `formatierung_anwenden` ist darin nur im Kommentar angefasst.

## Belegstellen

`crates/krk-ui/src/appkit/editor.rs:2432-2463` setzt und nimmt nichts:

```rust
speicher.beginEditing();
for stelle in &formatierung.auszeichnungen {
    let bereich = NSRange::new(stelle.anfang, stelle.laenge);
    let merkmale = match stelle.art { ... };
    unsafe { speicher.addAttributes_range(&merkmale, bereich) };
}
speicher.endEditing();
```

Die voruebergehenden Merkmale werden vorweg geleert (`editor.rs:2471`, `setTemporaryAttributes_forCharacterRange(&NSDictionary::new(), ganz)`); die Merkmale des Textspeichers nicht.

Die eine Stelle, die zurueknimmt, ist `merkmale_zuruecksetzen` (`editor.rs:2275-2289`), und sie nimmt nur eines von zwei:

```rust
speicher.removeAttribute_range(NSParagraphStyleAttributeName, ganz);
```

Der Doc-Kommentar sagt, warum: „Schrift und Farbe brauchen hier nichts, weil `setFont:` und `setTextColor:` in `grundschrift_setzen` den ganzen Speicher ueberschreiben." Das trifft — aber `grundschrift_setzen` und `merkmale_zuruecksetzen` haben beide **denselben einen Aufrufer**, `darstellung_nachziehen`, und der laeuft bei vier Anlaessen: Aufbau, gelungenes Oeffnen, Schliessen, Ansichtswechsel (`editor.rs:1068`, `:1289`, `:1698`, `:2147`). Beim Tippen laeuft er nicht. Dort geht der Weg `textDidChange:` → `text_zurueckschreiben` → `einfaerbung_anfordern` → `einfaerbung_einziehen` → `formatierung_anwenden`, und der faengt bei `addAttributes:range:` an.

## Fehlszenario, gemessen

Nachgebaut in AppKit (macOS 15.7.7, `swiftc -O`), mit KRKs Reihenfolge: Grundschrift ueber den ganzen Speicher, dann die Ueberschrift als Merkmal des Textspeichers, dann das Loeschen des `#` wie ein Tastendruck es tut:

```text
--- nach dem Setzen der Ueberschrift ---
vor dem Loeschen: Stelle 0 -> .AppleSystemUIFontBold 25.6
vor dem Loeschen: Stelle 2 -> .AppleSystemUIFontBold 25.6
Text jetzt: "Kopf\nText\n"
--- nach dem Loeschen, ohne Ruecknahme ---
nachher: Stelle 0 -> .AppleSystemUIFontBold 25.6
nachher: Stelle 1 -> .AppleSystemUIFontBold 25.6
==> BEFUND: die Zeile traegt weiter 25.6 pt statt 16.0 pt.
```

Am laufenden Buendel heisst das: `# Kopf` in einer Markdown-Datei, Formatansicht. Das `#` mit dem Leerzeichen loeschen. Die Zeile ist keine Ueberschrift mehr, die neue Formatierung fuehrt keine `Auszeichnung::Ueberschrift` dafuer, und „Kopf" bleibt gross und fett stehen — bis der Nutzer die Ansicht umschaltet oder die Datei neu oeffnet.

Dasselbe gilt fuer die beiden anderen Werte von `Auszeichnung`: ein geloeschtes `-` laesst den Absatzeinzug stehen, ein entfernter Zaun laesst die feste Schrift stehen.

Die drei Werte sind eine vollstaendige Fallunterscheidung ohne Auffangzweig, und das ist Absicht — aber sie deckt nur „was ist zu setzen", nicht „was war gesetzt und ist es nicht mehr".

## Vorschlag

Die Wirkung, die `formatierung_anwenden` haben soll, ist **setzen**, nicht **hinzufuegen**: nach dem Ruf traegt der Textspeicher genau die Merkmale der uebergebenen Formatierung und keine aelteren. Zwei Wege, beide innerhalb des vorhandenen `beginEditing`/`endEditing`:

1. Vorweg zuruecksetzen, so wie es bei den voruebergehenden Merkmalen schon geschieht: `removeAttribute:range:` fuer `NSParagraphStyleAttributeName` ueber den ganzen Text und `addAttribute:` der Grundschrift ueber den ganzen Text, dann die Schleife. Das ist zwei Nachrichten je Durchgang statt einer je Stelle und kostet damit nichts, was auffiele.
2. `merkmale_zuruecksetzen` um die Grundschrift erweitern und es aus `formatierung_anwenden` heraus rufen. Dann steht das Zuruecknehmen an einer Stelle statt an zwei — und der Satz „`setFont:` erledigt die Schrift" faellt, weil er nur fuer die vier Anlaesse von `darstellung_nachziehen` gilt.

Weg 2 ist der empfohlene: er macht aus zwei Halbwahrheiten eine Stelle. Die Grundschrift ist dort ohnehin schon ausgerechnet (`grundschrift_setzen` kennt sie aus Ansicht und Darstellungsart).

Dazu eine Probe. Sie braucht kein Fenster: `NSTextStorage`, Merkmale setzen, Text aendern, Merkmale lesen. Der Rest der Formatansicht ist in `crate::hervorhebung` ohne AppKit geprueft; diese eine Zusage ist die, die nur in AppKit messbar ist.

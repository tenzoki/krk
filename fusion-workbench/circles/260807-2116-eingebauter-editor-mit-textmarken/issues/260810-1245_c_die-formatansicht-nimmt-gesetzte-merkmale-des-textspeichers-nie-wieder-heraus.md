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

---
Resolved: Weg 2, der empfohlene, in der Fassung mit einer dritten Zusammenlegung
dazu. `merkmale_zuruecksetzen` ist die eine Stelle, die zuruecknimmt, und
`formatierung_anwenden` ruft sie, statt vorweg selbst zu leeren.

Drei Aenderungen, und die dritte macht die zweite ueberhaupt moeglich:

1. **Die Grundschrift steht jetzt an einer Stelle.** Die Fallunterscheidung aus
   `grundschrift_setzen` ist als freie Funktion `grundschrift(ansicht, art)`
   herausgezogen. Sie hat zwei Aufrufer: `grundschrift_setzen` setzt sie ueber
   `setFont:` an der Flaeche und damit auch fuer den naechsten Anschlag,
   `merkmale_zuruecksetzen` setzt sie als Merkmal ueber den ganzen Textspeicher.
   Zwei Rechnungen waeren die erste Gelegenheit, dass eine geloeschte
   Ueberschrift in einer anderen Schrift landete als der, in der ihre Zeile
   getippt wird.
2. **`merkmale_zuruecksetzen(ansicht, art)` nimmt jetzt beides heraus**, den
   Absatzstil wie bisher und die Schrift dazu: `removeAttribute:range:` fuer
   `NSParagraphStyleAttributeName` und `addAttributes:range:` mit der
   Grundschrift, beides ueber den ganzen Text, dazu wie bisher das Leeren der
   voruebergehenden Merkmale. Der Satz „`setFont:` erledigt die Schrift" ist
   damit gefallen; er galt nur fuer die vier Anlaesse von
   `darstellung_nachziehen` und nicht fuer den fuenften, das Tippen.
3. **`formatierung_anwenden` faengt nicht mehr bei `addAttributes:range:` an.**
   Es holt Ansicht und Darstellungsart aus dem Modell (die Ausleihe endet vor dem
   Ruf ins Textsystem, wie ueberall in dieser Datei), ruft
   `merkmale_zuruecksetzen` hinter der Laengenpruefung und setzt danach. Das
   eigene `setTemporaryAttributes:` mit dem leeren Verzeichnis ist **fort**: es
   waere die zweite Stelle mit einer Meinung darueber, was zurueckzunehmen ist,
   und `merkmale_zuruecksetzen` deckt dieselbe Liste mit ab.

Damit ist die Wirkung **setzen** und nicht hinzufuegen: nach dem Ruf traegt der
Textspeicher genau die Merkmale der uebergebenen Formatierung. Ein geloeschtes
`#` laesst keine 25,6 pt stehen, ein entferntes `-` keinen Einzug und ein
entfernter Zaun keine feste Schrift — und zwar auf dem Weg, der beim Tippen
laeuft (`textDidChange:` → `text_zurueckschreiben` → `einfaerbung_anfordern` →
`einfaerbung_einziehen` → `formatierung_anwenden`), nicht erst beim
Ansichtswechsel.

**`crate::hervorhebung` ist nicht angefasst.** Die Behebung sitzt an der Stelle
in `appkit/editor.rs`, an der die Merkmale auf den Textspeicher gehen; das
Fortschreiben der Einfaerbung mit seinem aufgehobenen Zerlegerzustand alle 32
Zeilen bleibt unberuehrt, weil es die **voruebergehenden** Merkmale liefert und
diese Behebung die des Textspeichers betrifft.

**Die Probe aus dem Vorschlag ist nicht gebaut, und der Grund gehoert dazu.** Sie
braucht kein Fenster, wohl aber eine Instanz: entweder einen `Editorbereich`
(dann Fenster, Takt, Aufteilung) oder eine `NSTextStorage` samt dem
`NSLayoutManager` einer Flaeche. Das erste liegt ausserhalb dessen, was
`an_einer_flaeche` gemessen hat, das zweite waere eine fuenfte Instanzprobe unter
der offenen Frage aus `decisions/260810-1044_o_…`, die ausdruecklich nicht in
einem Nebenzug beantwortet werden soll. Was die Behebung heute haelt, ist die
Zusammenlegung selbst: es gibt nur noch **eine** Stelle, die zuruecknimmt, und
`formatierung_anwenden` kann sie nicht uebergehen, ohne dass die Grundschrift
sichtbar falsch waere. Der Abnahmelauf am laufenden Buendel prueft es an
`# Kopf` in einer Markdown-Datei.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0,
`cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0.

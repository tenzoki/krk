# Die Schreibwerkzeuge aus macOS 15 schreiben den Text um und sind nicht abgewählt

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coder, bei der Behebung von 260809-1650
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`textflaeche_bauen`), Modulkopf
**Cross-references:** C4, S16, 260809-1650

---

## Der Befund

Bei der Behebung von 260809-1650 war zu prüfen, ob es neben den fünf
textverändernden Automatiken eine sechste gibt. Unter den Schaltern der Form
`set…Enabled:` gibt es keine — das hält jetzt die Probe
`keine_unbekannte_automatik_steht_an_der_textflaeche` fest, die die Klasse zur
Laufzeit aufzählt.

**Außerhalb dieser Form gibt es einen Fund.** `NSTextView` trägt seit macOS 15
die Schreibwerkzeuge:

```
writingToolsBehavior            NSWritingToolsBehavior (None / Default / Complete / Limited)
allowedWritingToolsResultOptions
writingToolsAllowedInputOptions
allowsWritingToolsAffordance
```

`NSWritingToolsBehaviorDefault` ist der Vorgabewert und überlässt dem System die
Wahl; für eine bearbeitbare `NSTextView` fällt sie auf die volle Ausstattung.
Die Schreibwerkzeuge **ersetzen markierten Text durch umgeschriebenen**, und das
Korrekturlesen wendet seine Änderungen über eine ganze Datei an. Was danach in
`NSTextView::string` steht, ist nicht mehr das Getippte, und über
`Editormodell::stand` geht es beim Sichern in die Datei.

**Der Zielwert der Anwendung ist genau die Fassung, in der es das gibt.** Das
Bündel zielt auf macOS 15.0 (`.cargo/config.toml`); die Eigenschaft steht seit
15.0 zur Verfügung und braucht deshalb keine Prüfung zur Laufzeit.

Der Vorgabewert ist auf diesem Gerät **nicht gemessen**, anders als der von
`smartInsertDeleteEnabled`: `speculation:` er ist `NSWritingToolsBehaviorDefault`,
so steht es in der AppKit-Dokumentation. Die Messung selbst ist billig — dieselbe
Stelle, an der `smartInsertDeleteEnabled` als `true` gemessen wurde.

## Warum das ein eigener Datensatz ist und nicht mit 260809-1650 erledigt

**Die fünf greifen ohne Zutun des Nutzers, die Schreibwerkzeuge auf seinen
ausdrücklichen Aufruf.** Eine Ersetzung von Anführungszeichen geschieht beim
Tippen, ein Smart-Insert-Leerzeichen beim Einfügen; die Schreibwerkzeuge öffnet
man aus dem Kontextmenü oder dem Menü *Bearbeiten*. Das ist ein Unterschied in
der Art und nicht im Grad, und er entscheidet die Frage nicht, sondern stellt
sie:

- Liest man C4 als „kein Zeichen ohne Zutun des Nutzers", dann sind die
  Schreibwerkzeuge zulässig und bleiben an.
- Liest man C4 als „der gesicherte Stand ist der getippte", dann sind sie es
  nicht, und `NSWritingToolsBehaviorNone` gehört zu den fünf abgeschalteten.

Der Editor ist außerdem ein **Programmtext**-Editor: ein Umschreiben von
Programmtext in flüssigere Prosa ist dort in keiner Lesart gemeint, und das
Kontextmenü steht an jeder Textfläche ohne Zutun.

Die Frage gehört dem Nutzer und nicht dem Übersetzer; deshalb steht sie hier und
ist nicht nebenbei entschieden worden.

## Vorschlag

Zuerst messen, dann entscheiden:

1. Den Vorgabewert an der Fläche aus `textflaeche_bauen` erheben, wie es für
   `smartInsertDeleteEnabled` geschehen ist.
2. Die Lesart von C4 festlegen — als Entscheidungsdatensatz, weil sie über
   diesen einen Schalter hinaus bindet.
3. Fällt sie gegen die Schreibwerkzeuge, dann eine Zeile bei den fünf:

```rust
text.setWritingToolsBehavior(NSWritingToolsBehavior::None);
```

Dazu die Aufzählung im Modulkopf und die Probe: die Schreibwerkzeuge tragen
keinen Schalter der Form `set…Enabled:` und fallen deshalb nicht unter
`ABGESCHALTET`/`GEDULDET`. Sie brauchen entweder eine eigene Aufstellung oder
eine erweiterte Form in derselben.

**Zwei Nachbarn sind geprüft und nicht als Befund geführt.**
`setImportsGraphics:` schaltet `setRichText(false)` von AppKit aus mit ab.
`setEnabledTextCheckingTypes:` ist die gesammelte Maske über dieselben Prüfungen,
die die fünf einzelnen Schalter bereits abwählen; sie daneben zu setzen wäre eine
zweite Stelle mit einer Meinung darüber, was abgeschaltet ist.

Gemeldet von: `coder`, im Durchgang zu 260809-1650.

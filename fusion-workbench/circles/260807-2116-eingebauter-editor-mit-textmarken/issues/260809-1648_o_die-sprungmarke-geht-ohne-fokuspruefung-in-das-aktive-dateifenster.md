# Die Sprungmarke geht ohne Fokusprüfung in das aktive Dateifenster

---
**Domain:** code
**Schwere:** High
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:1457-1481` (`eingabe_ausfuehren`)
**Cross-references:** `crates/krk-ui/src/appkit/ereignisse.rs:413-419`, `crates/krk-core/src/tasten/belegung.rs:780-798` (`nachschlag`), `crates/krk-ui/src/kommandos/fokus.rs:157-176`, C7 Abnahmekriterium 1, S4

---

## Der Befund

`Belegung::nachschlag` beantwortet jeden Tastendruck **ohne Zusatztaste**, der
keiner Funktion gehört, mit `Nachschlag::Sprungmarke` (`belegung.rs:793-797`).
Der Abgriff macht daraus `Eingabe::Zeichen` und gibt es an die Senke
(`ereignisse.rs:416-419`). Die Senke ist `eingabe_ausfuehren`:

```rust
// anwendung.rs:1466-1480
match eingabe {
    Eingabe::Kommando(kommando) => self.kommando_ausfuehren(kommando),
    Eingabe::Zeichen(zeichen) => {
        if self.blatt_steht() {
            return false;
        }
        let aktiv = self.ivars().modell.borrow().aktiv();
        self.dateifenster(aktiv).quelle().sprungmarke_tippen(zeichen)
    }
}
```

Ein getipptes Zeichen geht **immer** in das aktive Dateifenster. Die eine
Fokusabfrage des Programms sitzt in `kommando_ausfuehren`
(`anwendung.rs:1522-1525`) und damit im anderen Zweig; der Zeichenzweig kennt
sie nicht. Ein Zeichen ist kein Kommando und trägt deshalb auch keinen
`Wirkungsbereich`, an dem `fokus::wirkt` es messen könnte.

Der Doc-Kommentar über der Funktion (`anwendung.rs:1459-1461`) sagt es
ausdrücklich:

> Ein getipptes Zeichen gehört immer dem aktiven Dateifenster, weil die
> Sprungmarke aus C2 die Liste durchsucht, die vor dem Nutzer steht.

Der Satz war richtig, solange der Fokusvorbehalt jeden Tastendruck abfing,
sobald der Ersthelfer eine Textansicht war. **Seit S4 ist er falsch.** Die
Textfläche des Editors ist die benannte Ausnahme vom Vorbehalt, und mit dem Cursor
darin läuft jeder Buchstabe in den Nachschlag, fällt auf die Sprungmarke und
landet im Dateifenster.

## Warum das zählt

Das erste Abnahmekriterium von C7 lautet:

> Mit dem Fokus im Editor fügt eine Zeichentaste ihr Zeichen in den Text ein,
> und die Pfeiltasten bewegen die Schreibmarke, wie auf dem Mac üblich.

Was heute geschieht: das Zeichen fügt nichts ein. Es wandert in den Suchpuffer
der Sprungmarke des aktiven Dateifensters, verschiebt dort die Auswahl, und
`sprungmarke_tippen` liefert `true`, worauf der Abgriff das Ereignis schluckt.
Der Buchstabe erreicht die `NSTextView` nie.

Der Befund ist von
`issues/260809-1640_o_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`
zu unterscheiden und wird von dessen Behebung **nicht** miterledigt. Selbst
wenn `fokus()` den Editor kennt, geht der Zeichenzweig weiterhin ungeprüft in
das Dateifenster: er fragt den Fokus gar nicht.

## Vorschlag

Der Zeichenzweig braucht dieselbe Adressierung wie `bereichskommando`. Die
Sprungmarke ist eine Fähigkeit des Dateifensters aus C2; sie gehört an
`Fokus::Dateifenster` und, wie das Kommando, an `Fokus::Anderswo`, wo der Nutzer
zuletzt eine Liste bedient hat:

```rust
Eingabe::Zeichen(zeichen) => {
    if self.blatt_steht() {
        return false;
    }
    match self.fokus() {
        Fokus::Dateifenster | Fokus::Anderswo => {
            let aktiv = self.ivars().modell.borrow().aktiv();
            self.dateifenster(aktiv).quelle().sprungmarke_tippen(zeichen)
        }
        // Leiste, Vorschau und Editor tragen keine Sprungmarke; der
        // Tastendruck laeuft unveraendert an AppKit weiter und wird im
        // Editor zu einem Zeichen im Text.
        Fokus::Leiste | Fokus::Vorschau | Fokus::Editor => false,
    }
}
```

Die Fallunterscheidung ist vollständig und ohne Auffangzweig, wie die übrigen
über `Fokus`. Wichtig ist der Rückgabewert `false`: nur dann reicht der Abgriff
das Ereignis weiter, und nur dann tippt die Textfläche das Zeichen.

Zu klären ist noch, ob die Leiste und die Vorschau bis heute stillschweigend von
diesem Weg profitiert haben — mit dem Fokus in der Leiste tippt heute ebenfalls
jeder Buchstabe in die Sprungmarke des Dateifensters. Der Spec sagt zur Leiste
nichts darüber; die vorgeschlagene Zeile ändert das Verhalten dort mit und ist
insofern mehr als eine Editor-Frage.

Gemeldet von: `coderev`, Durchsicht Turn 2.

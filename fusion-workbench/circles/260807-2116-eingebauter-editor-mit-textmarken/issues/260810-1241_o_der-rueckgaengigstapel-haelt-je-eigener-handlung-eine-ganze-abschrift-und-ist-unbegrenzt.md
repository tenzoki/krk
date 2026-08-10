Der Rueckgaengigstapel haelt je eigener Handlung eine ganze Abschrift des Standes, und er ist unbegrenzt
---
`Umkehrpunkt` traegt den ganzen Stand als `String`. Jede Handlung, die `Verlauf::Traegt` anmeldet, legt eine weitere Abschrift in den Stapel, und nichts begrenzt seine Tiefe. An der Editorgrenze von 16 MB sind das 16 MB je Ersetzen.
---
**Schwere:** Hoch
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`

## Belegstellen

`crates/krk-ui/src/appkit/editor.rs:590-596`:

```rust
struct Umkehrpunkt {
    /// Die Zeichen, die das Modell vor dem Umbau hielt. In gehaltener Form,
    /// weil sie aus dem Modell kommen.
    stand: String,
    /// Die Auswahl der Flaeche vor dem Umbau, in AppKits Koordinate.
    auswahl: NSRange,
}
```

`crates/krk-ui/src/appkit/editor.rs:1613-1628` — der Punkt wandert in den Block, der Block in den Stapel:

```rust
let handlung = RcBlock::new(move |_ziel: NonNull<AnyObject>| {
    if let Some(editor) = selbst.load() {
        editor.umkehren(&punkt);
    }
});
unsafe { verwalter.registerUndoWithTarget_handler(self, &handlung) };
```

`setLevelsOfUndo` steht nirgends im Baum:

```sh
$ grep -rn "setLevelsOfUndo\|levelsOfUndo" crates/ resources/
$   # keine Fundstelle
```

`levelsOfUndo` steht bei einem `NSUndoManager` ab Werk auf `0`, also unbegrenzt.

`Verlauf::Traegt` laesst den bestehenden Stapel ausdruecklich stehen und meldet eine Handlung dazu (`editor.rs:598-657`, Aufzaehlung im Doc-Kommentar). Die Abschriften summieren sich damit; nur `Verlauf::TraegtNurDiese` leert vorher und haelt genau eine.

## Fehlszenario

Eine Datei von 16 MB im Editor, `cmd+f` nach einem haeufigen Wort, dann 100-mal `shift+cmd+r`, also der Weg, den C5 mit „der wievielte gerade angesteuert ist" ausdruecklich anbietet: einen Treffer ersetzen, weiterlaufen, den naechsten ersetzen. Nach 100 Ersetzungen halten 100 Handlungen 100 Abschriften à 16 MB, also rund 1,6 GB, und sie werden erst mit dem naechsten Dateiwechsel frei (`Verlauf::Faellt`).

Der Doc-Kommentar an `Umkehrpunkt` nennt „eine Kopie des Standes je Umbau, also bis zu 16 MB". Die Zahl je Handlung steht da, die Summe ueber einen Stapel ohne Tiefengrenze nicht.

## Zweiter Teil desselben Preises: die Abschrift entsteht auch ohne Treffer

`crates/krk-ui/src/appkit/editor.rs:2086-2098`:

```rust
// Der Umkehrpunkt entsteht vor der Aenderung, also auch dann, wenn kein
// Treffer ersetzt wird; danach haelt das Modell schon den neuen Stand.
// Ohne Treffer wird er hier fallengelassen.
let punkt = self.umkehrpunkt();
let zahl = self.ivars().modell.borrow_mut().alle_treffer_ersetzen(&ersatz);
if zahl > 0 {
    self.stand_erneuern(Verlauf::Traegt(punkt));
}
```

Der Kommentar sagt es, und die Reihenfolge ist zwingend richtig. Der Preis steht nicht dabei: ein `ctrl+cmd+r` auf einen Suchlauf mit null Treffern kopiert an einer Datei von 16 MB 16 MB und wirft sie fort. Die Zahl der Treffer liegt zu diesem Zeitpunkt schon vor.

## Vorschlag

Zwei Zeilen, die einander nicht ersetzen:

1. Eine Tiefengrenze setzen. `setLevelsOfUndo` mit einer Zahl, die C4 hergibt, macht aus „unbegrenzt mal Dateigroesse" ein Produkt mit zwei bekannten Faktoren. Die Frage „wie viele Schritte sagt C4 zu" ist damit beantwortet statt offengelassen.
2. Den Umkehrpunkt bei `alle_treffer_ersetzen` erst bauen, wenn ein Treffer steht: `if self.ivars().modell.borrow().suchlauf().map_or(0, Suchlauf::zahl) > 0` vor dem Ruf bricht die zwingende Reihenfolge nicht.

Eine dritte Moeglichkeit, die tiefer greift und deshalb hierher gehoert, ohne empfohlen zu sein: der Umkehrpunkt braucht nicht den ganzen Stand, sondern den Unterschied. Ein Ersetzen kennt seine Stellen; ein Umkehrpunkt aus Stelle, alter und neuer Zeichenfolge waere in der Groesse des Ersetzten statt in der der Datei. Das ist ein Umbau und keine Zeile, und ob C4 ihn braucht, entscheidet die Tiefengrenze aus Punkt 1 mit.

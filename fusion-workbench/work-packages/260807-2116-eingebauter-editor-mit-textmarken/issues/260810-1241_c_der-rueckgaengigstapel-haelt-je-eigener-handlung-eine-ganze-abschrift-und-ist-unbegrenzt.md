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

---
Resolved: Die dritte Moeglichkeit ist genommen, die erste ausdruecklich nicht.
`Umkehrpunkt` traegt nicht mehr den Stand, sondern den geaenderten Bereich:
`anfang`, `entfernt: String`, `eingefuegt: usize`, dazu die Auswahl wie bisher.
Gebildet wird er von `Umkehrpunkt::zwischen(vorher, nachher, auswahl)` aus dem
gemeinsamen Anfang und dem gemeinsamen Schwanz der beiden Staende, angewandt von
`Umkehrpunkt::angewandt_auf(stand)`. Beide Funktionen stehen einmal, und alle
vier Anlaesse gehen durch sie.

**Die Abschrift war nicht unvermeidlich, und die Frage der Aufgabenstellung ist
damit beantwortet:** eine Handlung braucht den geaenderten Bereich und nicht die
Datei. Ein Ersetzen aendert **einen** Bereich; ein Sammelersetzen aendert viele,
und der Punkt fasst sie in einen von der ersten bis zur letzten Stelle zusammen.
Das ist mehr als das Notwendige und trotzdem exakt: die Wiederherstellung ist
zeichengleich, und die Zahl der Bereiche zu fuehren hiesse, die Regeln des
Ersetzens in `appkit/editor.rs` ein zweites Mal zu tragen. Was ein Ersetzen
geaendert hat, weiss `krk_core::text::suche`.

**Gemessen, nicht geschaetzt** — an `krk_core::text::datei::EDITORGRENZE`, also
16 MB, mit einer Ersetzung von `foo` durch `quux` darin. Beide Zahlen sind an
derselben Probe abgelesen, die zweite durch Wiedereinsetzen der alten
Darstellung als Gegenprobe:

```text
                                je Handlung        100 Handlungen
  ganzer Stand (bis 260810-1241)  16 777 219 B      1 677 721 900 B
  geaenderter Bereich (jetzt)              3 B                300 B
```

Die Probe ist
`ein_umkehrpunkt_traegt_den_geaenderten_bereich_und_nicht_den_ganzen_stand`. Sie
haelt den Bau an, sobald eine Handlung an einer Datei von 16 MB mehr als 1 kB
haelt, und die Gegenprobe ist gefahren: mit der alten Darstellung meldet sie
`der Punkt haelt mehr als das ersetzte foo: 16777219 Bytes`. Zwei weitere Proben
halten, dass der Umbau nichts verliert: `…und_sein_gegenweg_stellen_beide_staende_zeichengleich_her`
prueft vierzehn Faelle in beiden Richtungen (mehrbytige Zeichen unmittelbar an
der Aenderung, gleich lange Ersetzung, Anfang, Ende, leerer Text in beide
Richtungen, mehrere Stellen auf einmal), und
`ein_wiederhergestellter_stand_ist_in_gehaltener_form` haelt die Zusicherung in
`umkehren`, an der `260810-0215` von der anderen Seite haengt.

**Der zweite Teil ist behoben.** `alle_treffer_ersetzen` fragt vor der Abschrift
`suchlauf().map(Suchlauf::zahl)` und laesst einen Suchlauf ohne Treffer mit
`Editormeldung::Ersetzt { zahl: 0 }` gehen, ohne 16 MB zu kopieren und
fortzuwerfen. Die Zahl ist keine zweite Wahrheit ueber die Treffer: `Suchlauf`
und `suche::alle_ersetzen` zaehlen mit **derselben** Funktion `suche::alle` im
selben Stand. Der Zweig `if zahl > 0` bleibt trotzdem stehen — er ist damit
unerreichbar, und der Grund steht als Kommentar dort.

**`setLevelsOfUndo` steht nirgends, und das ist eine Entscheidung.** Eine
Tiefengrenze gaebe es nur fuer den ganzen Verwalter, und der traegt nach
`umkehrung_anmelden` die Handlungen der Flaeche mit, also das Tippen. Eine
Grenze dort aenderte eine Zusage, die weder C4 noch C5 macht. Vor allem loeste
sie den einen Fall nicht, der nach diesem Umbau bleibt: eine Grenze in
**Handlungen** faengt einen Preis in **Bytes** nicht.

**Was bleibt, steht als eigener Datensatz** (`260810-1314_o_ein-wiederholtes-sammelersetzen-…`):
enthaelt der Ersatztext den Suchtext, findet der naechste `ctrl+cmd+r` wieder
Treffer, und der Bereich zwischen dem ersten und dem letzten deckt beinahe die
ganze Datei. Der Fall ist am Code belegt und im Doc-Kommentar von
`alle_treffer_ersetzen` benannt; er ist nach dem Umbau nicht schlimmer als
vorher, aber auch nicht besser.

**Die voruebergehende Abschrift bleibt und wird nicht verschwiegen.** Wer einen
Punkt bildet, hat beide Staende gleichzeitig zu halten, und bei drei der vier
Anlaesse kommt der alte als Kopie aus dem Modell. Sie faellt am Ende des Blocks,
in dem sie entstand, und geht in keinen Stapel ein; der Preis ist ein `memcpy`
je Handlung neben den beiden Durchgaengen, die `krk_core::text::suche` fuer
dieselbe Handlung ohnehin faehrt. Beim vierten Anlass, dem Rueckgaengig selbst,
faellt sie ganz weg: `umkehren` baut den wiederhergestellten Stand, den es
`bearbeiten` ohnehin uebergibt, und vergleicht gegen ihn.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0
(751 Proben, 0 Fehlschlaege), `cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0.

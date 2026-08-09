# S33 und S34: die beiden Ansichten und die Farbtafeln

**Status:** Complete
**Agent:** coder
**Datum:** 260810-0056
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritte 33 und 34, `### Frage 7` und `### Frage 3`
**Spec:** `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, Fähigkeit C3

## Was entstanden ist

Ein neues Modul `crates/krk-ui/src/hervorhebung.rs` — **ohne eine Zeile
AppKit**, wie `editormodell.rs` daneben — rechnet aus Text, Pfad, Dateityp und
Farbtafel aus, welche Stelle welche Farbe trägt, welche unterstrichen ist und
welche eine Markdown-Auszeichnung ist. `crates/krk-ui/src/appkit/editor.rs`
setzt das Ergebnis in Merkmale um, wechselt Grundschrift und Umbruch und bindet
die Tafel an das Erscheinungsbild des Systems.

Der Befehl `Kommando::EditorAnsichtUmschalten` hat seinen Zweig im
Anwendungsdelegierten bekommen; bis dahin fiel er in `bereichskommando` und
wurde dort mit `false` beantwortet.

## Der Befund, der den Plan an einer Stelle widerlegt

`### Frage 7` des Plans und der Datensatz
`decisions/260808-0140_a_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md`
nennen für die ganze Formatansicht **eine** Mechanik: vorübergehende Merkmale
des Layoutverwalters. Unter dieselbe Mechanik zählt die vom Nutzer gewählte
Möglichkeit 1 aber vier Wirkungen, darunter „Überschriften größer und fett,
Listen eingerückt, Quelltextblöcke in fester Schrift".

Drei davon trägt diese Mechanik nicht, und der Grund steht im Kopf des Systems
selbst:

> Clients may set any attributes they wish, but the only attributes that the
> layout manager will recognize for drawing are those that do not affect layout
> (color, underline, etc.).
>
> `MacOSX.sdk/…/AppKit.framework/Headers/NSLayoutManager.h:351`

Als vorübergehendes Merkmal gesetzt täte eine Schriftgröße nichts — nicht etwas
Falsches, sondern gar nichts.

**Der Zuschnitt, der daraus folgt**, ist nicht „Farbe gegen Rest", sondern
**„wirkt auf die Auslegung oder nicht"**; die Frage ist trennscharf und
vollständig, und welche Seite gilt, sagt der SDK-Kopf und keine Liste in KRK:

| Wirkung | Wohin sie geht |
|---|---|
| Farbe der Wortarten, Unterstreichung der Links | vorübergehende Merkmale, `NSLayoutManager` |
| Überschrift größer und fett, Listeneinzug, feste Schrift für Quelltext | Merkmale des `NSTextStorage` |

**Die Zusage, an der der Plan hängt, hält dabei unverändert**, und sie hängt
nicht an den vorübergehenden Merkmalen: gesichert wird `Editormodell::stand`,
eine gewöhnliche Zeichenkette, und die kommt aus `NSTextView::string`, also aus
den **Zeichen** der Fläche. Kein Merkmal wird auf dem Sicherungsweg auch nur
gelesen, gleich in welchem der beiden Speicher es liegt.

Festgehalten als
`issues/260810-0053_o_der-plan-legt-die-markdown-auszeichnung-in-voruebergehende-merkmale-und-die-tragen-sie-nicht.md`,
zusammen mit den drei Stellen in Plan und Datensatz, die zu korrigieren sind.
Nichts davon ändert, was der Nutzer sieht; es ändert, was der Plan über den Weg
dorthin behauptet.

## Die Messung, die die zweite Bauentscheidung getragen hat

Bevor die erste Zeile stand, ist gemessen worden, was die Kiste kostet:
`--release`, an `crates/krk-ui/src/appkit/anwendung.rs` (193 kB) und Vielfachen
davon.

```
    192 866 Byte ->  0,64 s   (0,30 MB/s)
  1 542 928 Byte ->  4,60 s   (0,34 MB/s)
  7 714 640 Byte -> 23,12 s   (0,33 MB/s)
```

Drei Wege gemessen — nur parsen; parsen mit `HighlightIterator`; parsen mit
Wortartenstapel und Farbe —, alle drei gleich schnell. Der Aufwand steckt
vollständig in `ParseState::parse_line`. **Damit war entschieden, dass das
Einfärben nicht auf den Hauptfaden gehört:** die Grenze des Editors liegt bei
16 MB, und der Hauptfaden stünde dafür knapp eine Minute. S24 hat das Lesen aus
genau diesem Grund auf einen Faden gelegt, und der Modulkopf von
`editormodell.rs` schreibt aus, warum es nicht zwei Wahrheiten darüber geben
darf, wann der Hauptfaden anhält.

Gebaut ist deshalb `Einfaerbungsvorgang` in derselben Bauart wie `Ladevorgang`:
ein Faden je Anfrage, `sync_channel(1)`, kein Generationszähler, weil eine neue
Anfrage den alten Empfänger fallen lässt. Abgeholt wird über **denselben**
Zeitgeber, der schon das Lesen abholt; ein zweiter fragte im selben Sechzigstel
dieselbe Laufschleife ein zweites Mal.

Schnelle Anfragen fasst eine einzige Marke zusammen: läuft schon ein Faden, wird
kein zweiter gestartet, sondern vermerkt, dass sein Ergebnis überholt sein wird.
Damit lebt zu jedem Zeitpunkt höchstens ein Faden, und der letzte Stand wird
genau einmal eingefärbt statt jeder Zwischenstand einmal. Dieselbe Marke trägt
beide Anlässe, den geänderten Text und die gewechselte Farbtafel: beide
verlangen dasselbe.

Der Preis, der bleibt, ist gemessen und benannt:
`issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`.
Er nennt den Ausweg (`ParseState` je Zeile fortschreiben) und den Grund, ihn
zusammen mit `260809-2322` zu bewerten: beide Stellen stellen dieselbe Frage.

## Ein Durchgang, zwei Verbraucher

Die Kiste läuft **einmal** über den Text. `ScopeRegionIterator` liefert je Stück
zugleich den Wortartenstapel und, über `Highlighter::style_for_stack`, die
Farbe; die Messung oben zeigt, dass der Stapel nichts dazukostet. Zwei
Durchgänge, einer für die Farbe und einer für die Markdown-Auszeichnung,
kosteten das Doppelte und könnten auseinanderlaufen.

Die Wortartennamen sind am 260810 an der eingebundenen Fassung **abgelesen** und
nicht angenommen: `markup.heading.<n>.markdown`, `markup.list.…`,
`markup.raw.…`, `markup.underline.link.markdown`.

## Die drei Besetzungen sind eine Mechanik und nicht drei

`hervorhebung::art` ist die eine Stelle, die entscheidet, und sie zieht die
Grenze zwischen Code und einfachem Text genau so, wie das sechste
Abnahmekriterium von C3 sie zieht: **„die eingebundene Kiste kennt eine Sprache
dafür"**. Der Dateityp im Modell trägt weiter zwei Werte und keine drei; die
dritte Frage wird beim Darstellen gestellt.

| Ansicht | Besetzung | Grundschrift | Umbruch | Einfärbung |
|---|---|---|---|---|
| Roh | alle | fest, Systemgröße | aus, waagerechter Schieber | keine |
| Format | Code | fest, Systemgröße | am Fensterrand | ja |
| Format | Markdown | proportional, +2 pt | am Fensterrand | ja, dazu Überschriften, Listen, Quelltext |
| Format | einfacher Text | proportional, +2 pt | am Fensterrand | keine |

Eine Sprache, die die Kiste nicht kennt, fällt auf die erste Form zurück und
meldet keinen Fehler. Ein Stück in der Grundfarbe der Tafel bekommt **kein**
Merkmal und behält die Systemfarbe; aus der Tafel kommen allein die
Vordergrundfarben der Wortarten, und damit stimmt der Kontrast in beiden
Erscheinungsbildern ohne Zutun.

## S34: zwei Tafeln, eine Zeile

`base16-ocean.light` und `base16-ocean.dark`, gewählt über
`bestMatchFromAppearancesWithNames:` — die eine Stelle, die AppKit für die Frage
vorsieht, und die auch die Erscheinungsbilder mit erhöhtem Kontrast auf eines
der beiden abbildet. Keine Farbe steht als Zahlenwert im Programmtext.
`grep -c 'setBackgroundColor' crates/krk-ui/src/appkit/editor.rs` liefert 0: der
Grund bleibt die Systemfarbe.

Bemerkt wird der Wechsel über `viewDidChangeEffectiveAppearance`. Weil das eine
Methode einer Ansicht ist und der `Editorbereich` ein `NSObject`, trägt die
Ansicht, in der Kopf und Textfläche hängen, jetzt die Klasse `Editorsicht`; sie
hält den Rückverweis **schwach**, sonst schlösse sich ein Ring. Hat sich die
Tafel nicht geändert, geschieht nichts — die Meldung kommt auch bei Wechseln,
die Hell und Dunkel nicht betreffen.

## Wie belegt ist, dass der Wechsel nichts verliert

Vier Sachen zusammen, keine davon eine Zusage der Sorgfalt:

1. **Es gibt genau einen Textbestand.** `grep -c 'setString'` in `editor.rs`
   findet fünf Treffer, davon vier in Kommentaren und **einen** Aufruf, in
   `stand_einsetzen`. Im Umschaltweg steht keiner. `NSTextStorage::alloc` und
   `NSTextStorage::new` kommen in der Datei nicht vor: den Speicher bringt die
   `NSTextView` selbst mit.
2. **Der Umschaltweg fasst den Speicher nicht an.**
   `Editorbereich::ansicht_umschalten` ruft `Editormodell::ansicht_umschalten`
   und `darstellung_nachziehen`; das erste rührt weder Stand noch
   Abweichungsmarke an, das zweite setzt Schrift, Umbruch und Merkmale. Kein
   Weg dazwischen liest oder schreibt Zeichen.
3. **Die Schreibmarke bleibt ohne eigenen Bau stehen**, weil sie an
   Zeichenstellen hängt und die Zeichen dieselben bleiben. Das elfte
   Abnahmekriterium von C3 fällt daraus an.
4. **Kein Merkmal kann in die Datei geraten**, und das ist geprüft und nicht
   angenommen: `Editormodell::sichern` schreibt `self.stand`, und der einzige
   Weg, auf dem etwas in `stand` kommt, ist `bearbeiten` mit dem Ergebnis von
   `NSTextView::string`. Merkmale werden auf diesem Weg nicht gelesen, und
   `setRichText(false)` steht daneben.

Was daran ohne Fenster prüfbar war, steht als Pruefcode da: zwölf Proben in
`hervorhebung.rs` decken den Rückfall auf einfachen Text, die vier Sprachen aus
C3, die Datei ohne Endung, die Abgesetztheit der Wortarten, den Unterschied der
beiden Tafeln, Markdown-Überschriften mit ihren Stufen, die Listenzeile über
eine ganze Zeile, die UTF-16-Koordinate, die Reihenfolge und
Überschneidungsfreiheit der Stellen, den leeren Text und die Gleichheit von
Arbeitsfaden und unmittelbarem Ruf.

## Geänderte Dateien

- `crates/krk-ui/src/hervorhebung.rs` — **neu**, ohne AppKit
- `crates/krk-ui/src/appkit/editor.rs` — die beiden Darstellungen, die
  Merkmalswege, der Einfärbungstakt, `Editorsicht`, die Tafelwahl
- `crates/krk-ui/src/appkit/nummernspalte.rs` — `spalte_neu_zeichnen`, der eine
  Weg von außen (Vermerk aus S46)
- `crates/krk-ui/src/appkit/anwendung.rs` — der Zweig für
  `Kommando::EditorAnsichtUmschalten`
- `crates/krk-ui/src/editormodell.rs` — zwei Kommentare nachgezogen; kein
  Verhalten geändert
- `crates/krk-ui/src/main.rs` — `mod hervorhebung;`

## Abnahme

```
cargo build --workspace          0
cargo test --workspace           0   (15 Prüfziele, alle grün)
cargo clippy --workspace --all-targets   0, keine Meldung
cargo fmt --all --check          0
cargo xtask bundle               0   (signiert, target/KRK.app)
```

`grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src crates/krk-ui/src`
nennt weiterhin genau zwei Dateien; `hervorhebung.rs` trägt kein `unsafe`.

## Was Nutzerarbeit bleibt

Sieben Prüfungen am laufenden Bündel, sechs aus S33 und eine aus S34; sie stehen
im Bericht an den Nutzer und verlangen KRK im Vordergrund.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-0053_o_der-plan-legt-die-markdown-auszeichnung-in-voruebergehende-merkmale-und-die-tragen-sie-nicht.md`,
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`,
`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_a_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md`,
`shared/decisions/260802-0842_a_editor-formatansicht-je-dateityp.md`

# S4: Der Ersthelfer des Editors bricht den Fokusvorbehalt

- Agent: `coder`
- Datum: 260809-1519
- Plan: `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Phase A, Schritt 4
- Status: Complete

## Was umgesetzt ist

Der Fokusvorbehalt des Ereignisabgriffs stellt seine Frage neu.
`ersthelfer_nimmt_text` heißt jetzt `ersthelfer_gehoert_appkit`
(`crates/krk-ui/src/appkit/ereignisse.rs:443-459`) und beantwortet "behält
dieser Ersthelfer seine AppKit-Bedeutung?" statt "nimmt dieser Ersthelfer Text
entgegen?". Sie fragt zuerst über einen Abschluss, ob der Ersthelfer dasselbe
Objekt wie die Textfläche des Editors ist; trifft das zu, antwortet sie mit
`false`, und der Tastendruck läuft weiter in den Nachschlag.

Die Prüfung auf `NSTextView`, `NSTextField` und `NSText` steht unverändert
darunter, samt ihrer Begründung zum Feldeditor. Eine vierte Klasse ist nicht
hinzugekommen: `grep -c 'isKindOfClass' crates/krk-ui/src/appkit/ereignisse.rs`
liefert vorher wie nachher **3**.

`Tastenabgriff::einrichten` und `behandeln` tragen den dritten Abschluss
`ist_editorflaeche: impl Fn(&NSResponder) -> bool`, in derselben Form wie
`faenger` und `senke` und an der Stelle in der Argumentliste, an der er
abgefragt wird: nach dem Fänger, vor der Senke.

Der Ersthelfer kommt als Argument in den Abschluss und wird nicht ein zweites
Mal beim Schlüsselfenster erfragt. Damit gibt es weiterhin genau eine Stelle,
die sagt, wer der Ersthelfer ist.

`Anwendungsdelegierter::abgriff_aufsetzen`
(`crates/krk-ui/src/appkit/anwendung.rs:1191-1213`) gibt den Abschluss mit,
über dieselbe schwache Rückreferenz wie die beiden bestehenden. Die neue
Methode `Anwendungsdelegierter::ist_editorflaeche`
(`anwendung.rs:1229-1231`) antwortet heute immer mit `false`: solange kein
Editor gebaut ist, gibt es keine Textfläche, mit der zu vergleichen wäre, und
das Verhalten bleibt das heutige. Der Kommentar nennt **S16** als Ablösung, weil
dort die Textfläche entsteht und ihre Nämlichkeit nach außen meldet.

`appkit/ereignisse.rs` kennt den Editor nicht und hält ihn nicht; es kennt allein
die Frage, die der Anwendungsdelegierte beantwortet.

## Warum die Nämlichkeit und nicht die Art

Der Modulkopf von `ereignisse.rs` schreibt es in drei Absätzen aus, unter dem
Abschnitt `# Der Fokusvorbehalt`: die Textfläche des Editors ist eine
`NSTextView` wie der Feldeditor eines Textfeldes auch, und eine Frage nach der
Art kann zwei Objekte derselben Art nicht trennen. Der Vergleich läuft deshalb
über die Objektgleichheit der Objective-C-Zeiger und nicht über einen
Klassennamen, ein Kennzeichen an der Ansicht oder einen Gang durch den
Ansichtsbaum. Er ist trennscharf, weil ein Objekt mit genau einem anderen
identisch ist, und vollständig, weil die Frage für jeden Ersthelfer eine Antwort
hat; eine Liste von Ausnahmen entsteht nirgends.

Der Doc-Kommentar an `ersthelfer_gehoert_appkit` sagt dasselbe noch einmal für
die Reihenfolge: die Nämlichkeitsfrage steht vor der Klassenprüfung, weil sie ihr
sonst zum Opfer fiele.

## Das Schaubild im Modulkopf

Das ASCII-Schaubild zeigte den Fokusvorbehalt vor der Normalisierung und
beschrieb damit einen Weg, den der Code nie gegangen ist. Es ist auf die
tatsächliche Reihenfolge gezogen und von einer waagerechten Kette auf einen
Baum umgestellt, weil der Vorbehalt jetzt zwei Ausgänge hat:

```text
NSEvent
   │
   ├─ Tastendruck::aus_ereignis ..... die Maske ist normalisiert
   │
   ├─ Faenger der Belegungsansicht .. nimmt er auf: Ereignis verbraucht
   │
   ├─ Fokusvorbehalt
   │    ├─ Ersthelfer = Textflaeche des Editors? ──ja──> weiter zum Nachschlag
   │    └─ sonst Textfeld, Feldeditor, Blatt? ────ja──> unveraendert an AppKit
   │
   └─ Belegung::nachschlag
        ├─ Kommando ─────> Senke des Aufrufers
        ├─ Sprungmarke ──> Zeichen ──> Senke des Aufrufers
        └─ unbelegt ─────> unveraendert an AppKit
```

## Wie die Reihenfolge belegt ist

`grep -n` über `crates/krk-ui/src/appkit/ereignisse.rs`:

```
443:fn ersthelfer_gehoert_appkit(
453:    if ist_editorflaeche(&ersthelfer) {
456:    ersthelfer.isKindOfClass(NSTextView::class())
457:        || ersthelfer.isKindOfClass(NSTextField::class())
458:        || ersthelfer.isKindOfClass(NSText::class())
```

Die Nämlichkeitsfrage steht in Zeile 453 und kehrt im Trefferfall unmittelbar mit
`false` zurück; die drei Klassenprüfungen stehen darunter in 456 bis 458 und sind
in diesem Fall unerreichbar.

Eine Probe deckt das nicht ab und kann es nicht: `ersthelfer_gehoert_appkit`
fragt `NSApplication::sharedApplication(mtm).keyWindow()`, und ein
Schlüsselfenster gibt es nur in der laufenden Anwendung. Der Beleg ist der Diff
und das `grep` oben, so wie das Abnahmekriterium des Schrittes es verlangt.

## Nichts außerhalb des Umfangs mitgezogen

Geändert sind genau die beiden Dateien, die der Schritt nennt. Die beiden Zweige
`Fokus::Editor => false` aus S3 (`anwendung.rs:1110` und `:1594`) stehen
unverändert; sie gehören S17.

## Abnahme

| Kommando | Ergebnis |
|----------|----------|
| `cargo build --workspace` | grün |
| `cargo test --workspace` | grün, 590 Proben, 0 Fehler, 1 ignoriert |
| `cargo clippy --workspace --all-targets` | grün, keine Warnung |
| `cargo fmt --all --check` | sauber |

**Zweimal gefahren, das zweite Mal abgeschottet.** Der erste Lauf um 15:15 im
Arbeitsbaum war grün. Kurz danach brach der Bau im Arbeitsbaum an
`crates/krk-ui/src/belegungsmodell.rs:154` mit `E0004`: das parallel laufende S5
hatte die zwölf Editor-Kommandos in `Kommando` angelegt, und die erschöpfende
Fallunterscheidung dort war noch nicht nachgezogen. Beide Dateien sind für S5
reserviert und von diesem Schritt nicht angefasst.

Der zweite Lauf lief deshalb auf einer abgeschotteten Kopie: `git archive HEAD`
in ein Verzeichnis außerhalb des Arbeitsbaums, darüber genau die beiden Dateien
dieses Schrittes. Alle vier Kommandos grün, mit den Zahlen aus der Tafel oben.
Damit ist belegt, dass dieser Schritt für sich allein baut und der Bruch im
Arbeitsbaum ihm nicht gehört.

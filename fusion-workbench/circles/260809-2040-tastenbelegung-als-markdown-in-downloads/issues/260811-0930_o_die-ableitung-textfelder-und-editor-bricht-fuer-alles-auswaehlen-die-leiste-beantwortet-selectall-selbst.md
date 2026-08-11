Die Ableitung „Textfelder und Editor" bricht für Alles auswählen: die Leiste beantwortet `selectAll:` selbst

---

Der Spec dieses Circles beschriftet die dritte Spalte für die sechs vom Hauptmenü zugestellten
Textbefehle mit „Textfelder und Editor" und kennzeichnet das ausdrücklich als **Ableitung, nicht
als Messung**. Die Messung aus Schritt S1 ist gefahren und hat die Ableitung für einen der sechs
gebrochen: `NSTableView` beantwortet `selectAll:` **von sich aus**, aus einer Methode an
`NSTableView` selbst. Die Lesezeichen- und Geräteleiste ist eine `NSTableView`.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Umsetzung von S1 — also durch genau die Prüfung, die der Spec als
zwölftes Abnahmekriterium von C3 dafür vorgesehen hat
**Betroffen:** die dritte Spalte der Ausgabe, `text_alles_auswaehlen`
**Domain:** code

## Die Messung

Gemessen über `AnyClass::responds_to` gegen die sechs Klassen, die in KRK einen Ersthelfer
stellen können — ohne Instanz, ohne Hauptfaden, ohne Vordergrund. Zusätzlich ist die
Vererbungskette hinaufgelaufen, um zu sehen, **welche** Klasse die Methode trägt; daran hängen
die drei Befunde.

| Selektor | antwortet an | trägt die Methode | Befund |
|---|---|---|---|
| `cut:` | NSTextView | NSText | Ableitung **bestätigt** |
| `copy:` | NSTextView | NSText | Ableitung **bestätigt** |
| `paste:` | NSTextView | NSText | Ableitung **bestätigt** |
| `selectAll:` | **NSTableView** und NSTextView | **NSTableView** bzw. NSText | Ableitung **gebrochen** |
| `undo:` | NSWindow | NSWindow | **nicht entscheidbar** |
| `redo:` | NSWindow | NSWindow | **nicht entscheidbar** |

`NSTextField`, `NSScrollView` und `NSApplication` beantworten keinen der sechs.

**Warum die drei bestätigten wirklich bestätigt sind:** sie hängen an `NSText`, nicht an
`NSTextView`, und `NSTextField` beantwortet sie selbst nicht. Erreicht wird also der
**Feldeditor** des Textfeldes, und der ist eine `NSTextView` und bringt `NSText` mit. Genau das
behauptete der Modulkopf von `menue.rs` schon; jetzt ist es gemessen.

**Warum `undo:` und `redo:` nicht entscheidbar sind:** beide stehen an `NSWindow` und nicht an
der Textklasse. `responds_to` liefert `false` für einen weitergeleiteten Selektor, und das ist
hier der Fall. Ein `false` an der Textklasse belegt deshalb **nicht**, dass im Editor niemand
antwortet.

## Was daraus folgt

Der Plan gibt die Regel vor: wo die Messung keine Antwort gibt, bleibt die Zelle leer, denn eine
leere Zelle ist eine ehrliche Auskunft und eine falsche ist es nicht. Für
`text_alles_auswaehlen` ist die Zelle damit **leer**, und S3 zitiert diesen Datensatz in seiner
`match`-Verzweigung.

## Was die Messung ausdrücklich nicht entschieden hat

**Ob der in der Leiste bedienbare Eintrag dort auch etwas bewirkt.** Der stumme Fokusvorbehalt
weist `alle_markieren` in der Leiste ab, der Tastendruck geht unverändert an AppKit und erreicht
den Menüeintrag — der Eintrag ist dort also **bedienbar**. Ob `NSTableView` daraufhin Zeilen
auswählt, braucht eine Instanz und damit den Hauptfaden, und S1 misst ausdrücklich ohne beides.
Die Erwägung des Plans zu `setAllowsMultipleSelection(false)` bleibt damit eine Vermutung über
die Wirkung und ist nicht zur Messung erhoben worden.

Wer das messen will, braucht einen Lauf mit KRK im Vordergrund — dieselbe Bedingung, an der der
Abnahmelauf hängt (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).

## Warum dieser Datensatz steht, obwohl nichts kaputt ist

Am Code ist nichts falsch. Falsch gewesen wäre die Spalte, hätte niemand gemessen. Der Datensatz
hält fest, **dass** gemessen wurde und **was** dabei herauskam, damit die leere Zelle in S3 nicht
später als Versäumnis gelesen wird und jemand sie „vervollständigt".

Drei Proben halten die Messung im Baum: die vollständige Tabelle (schlägt fehl, sobald sich eine
Antwort ändert), der Verdachtsfall als eigene Zusicherung, und eine für `undo:`/`redo:`, die
festhält, was das `false` **nicht** bedeutet.

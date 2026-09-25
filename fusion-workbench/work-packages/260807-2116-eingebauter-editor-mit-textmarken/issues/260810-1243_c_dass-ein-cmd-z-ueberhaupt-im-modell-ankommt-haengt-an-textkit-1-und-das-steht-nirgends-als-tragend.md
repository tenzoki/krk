Dass ein `cmd+z` ueberhaupt im Modell ankommt, haengt an TextKit 1, und das steht nirgends als tragend
---
`textDidChange:` ist der einzige Rueckweg aus der Textflaeche in das `Editormodell`. Gemessen: eine `NSTextView` auf TextKit 2 aendert bei `undo` ihren Text **ohne** `textDidChange:` zu verschicken; auf TextKit 1 verschickt sie es. KRK laeuft auf TextKit 1, aber nur, weil die Nummernspalte und `merkmale_zuruecksetzen` `layoutManager` anfassen. Keine Zeile sagt, dass der Rueckweg daran haengt.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/nummernspalte.rs`

## Die Messung

macOS 15.7.7 (Build 24G720), `swiftc -O`, eine `NSTextView` mit `allowsUndo = true` in einem Fenster als Ersthelfer, ein Delegierter, der `textDidChange:` und `textView:shouldChangeTextIn:replacementString:` zaehlt, und ein `NotificationCenter`-Beobachter auf `NSText.didChangeNotification`. Getippt ueber `insertText:replacementRange:`, dazwischen Umlaeufe der Laufschleife.

| Aufbau | `undo` aendert den Text | `textDidChange:` | `NSTextDidChangeNotification` |
|---|---|---|---|
| ohne Zugriff auf `layoutManager` (TextKit 2) | ja | **0x** | **0x** |
| mit `_ = sicht.layoutManager` (TextKit 1) | ja | 1x | 1x |

Der Unterschied ist einzig der Zugriff auf `layoutManager`; sonst ist der Code Zeile fuer Zeile derselbe. Beide Zeilen sind je dreimal gelaufen und reproduzieren.

Dann derselbe Versuch mit KRKs Aufbaureihenfolge nachgebaut — `textflaeche_bauen` samt der sieben abgewaehlten Automatiken, danach `merkmale_zuruecksetzen` wie `darstellung_nachziehen` es beim Aufbau tut (`editor.rs:1068` → `:2181` → `:2283 text.layoutManager()`), drei getrennte Tippgruppen mit `breakUndoCoalescing`:

```text
getippt -> "erste Zeile\neins zwei drei "   didChange 15x
undo 1: -> "erste Zeile\neins zwei "   didChange 1x, der Rueckweg saehe: ["erste Zeile\neins zwei "]
undo 2: -> "erste Zeile\neins "        didChange 1x, der Rueckweg saehe: ["erste Zeile\neins "]
undo 3: -> "erste Zeile\n"             didChange 1x, der Rueckweg saehe: ["erste Zeile\n"]
redo:   -> "erste Zeile\neins "        didChange 1x, saehe ["erste Zeile\neins "]
```

**Heute ist also nichts kaputt.** Jedes `cmd+z` und `shift+cmd+z` erreicht `text_zurueckschreiben`, und der Rueckweg sieht den schon zurueckgenommenen Text — genau das, was er braucht.

## Warum es trotzdem ein Befund ist

Der Rueckfall auf TextKit 1 entsteht als **Nebenwirkung** von zwei Stellen, die ihn aus einem anderen Grund auslösen:

- `crates/krk-ui/src/appkit/nummernspalte.rs:89-90` sagt es ausdruecklich: „Der Zugriff auf `layoutManager` laesst AppKit auf den aelteren `NSLayoutManager` statt auf `NSTextLayoutManager` zurueckfallen." Der Grund dort ist das Zeichnen der Nummern.
- `merkmale_zuruecksetzen` (`editor.rs:2283`) und `formatierung_anwenden` (`editor.rs:2421`) fassen ihn an, um voruebergehende Merkmale zu setzen.

Der Modulkopf von `editor.rs` fuehrt in `editor.rs:1833-1837` die Wege auf, die Flaeche und Stand zeichengleich halten, und `textDidChange:` steht als „die eine Stelle, die AppKit dafuer vorsieht" (`editor.rs:56`). Dass diese eine Stelle bei einer Flaeche auf TextKit 2 **nicht** feuert, steht nirgends.

## Fehlszenario

Jemand zieht die Nummernspalte auf `NSTextLayoutManager` nach — das ist der Weg, den Apple fuer neue Arbeit vorsieht, und C10 zwingt zu nichts anderem — und nimmt dabei die beiden `layoutManager`-Zugriffe im Editor mit. Der Bau bleibt gruen, alle 744 Proben bleiben gruen (keine von ihnen faehrt ein `undo` an einer Flaeche in einem Fenster), und dann:

1. Datei oeffnen, einen Absatz tippen, `cmd+z`, `cmd+s`. Der Editor zeigt die Datei ohne den Absatz, in der Datei steht er — `Editormodell::sichern` schreibt `&self.stand` (`editormodell.rs:996`), und der Stand ist nie nachgezogen worden.
2. `cmd+j` und `cmd+d` rechnen gegen den falschen Text: `schreibmarke_in_utf16` liest aus der Flaeche, `stelle_zeigen` rechnet gegen `modell.stand()`.

Kein Absturz — `setSelectedRange:` beschneidet —, aber ein stiller Datenverlust im ersten Fall.

## Vorschlag

Zwei Zeilen und keine Mechanik:

1. Den Satz in den Modulkopf von `editor.rs`, zu den Wegen, die Flaeche und Stand zeichengleich halten: der Rueckweg `textDidChange:` deckt Rueckgaengig und Wiederherstellen **nur auf TextKit 1** ab, gemessen am 260810 auf 15.7.7, und die Flaeche ist TextKit 1, weil `merkmale_zuruecksetzen` beim Aufbau `layoutManager` anfasst. Der Verweis gehoert in beide Richtungen: auch `nummernspalte.rs:89-90` sollte wissen, dass an seinem Zugriff mehr haengt als die Nummern.
2. Eine Probe, die es haelt. `assert!(flaeche.layoutManager().is_some() && flaeche.textLayoutManager().is_none())` unmittelbar nach `textflaeche_bauen` ist eine Zeile und faellt aus, sobald jemand den Rueckfall wegnimmt. Sie gehoert zu den vier Instanzproben und traegt deren offene Frage aus `decisions/260810-1044_o_…` mit.

Was **nicht** hilft: sich auf die Messung zu verlassen. Dass `undo` auf TextKit 1 `textDidChange:` verschickt, ist eine gemessene Eigenschaft und keine zugesagte; sie kann sich mit einer macOS-Fassung aendern. Was der Befund verlangt, ist, dass die Abhaengigkeit benannt ist — nicht, dass sie beseitigt wird.

---
Resolved: Der Rueckfall auf TextKit 1 ist von einer Nebenwirkung zu einer
benannten Voraussetzung geworden, und eine Probe haelt sie.

**Die Aufschreibung steht jetzt am Ort, der sie herstellt.** `textflaeche_bauen`
fasst `layoutManager` eigens an, unmittelbar hinter `setAllowsUndo(true)`, und
der Kommentar daneben nennt den Grund: ohne die Zeile kommt keine der Handlungen,
die die vorige Zeile ermoeglicht, im Modell an. Der Modulkopf traegt denselben
Satz an der Stelle, an der `textDidChange:` als „die eine Stelle, die AppKit
dafuer vorsieht" steht, samt der Messung und samt dem Hinweis, dass sie eine
gemessene und keine zugesagte Eigenschaft ist. Vorher entstand der Rueckfall aus
`merkmale_zuruecksetzen` und aus `super::nummernspalte`, die den Verwalter aus
einem anderen Grund anfassen — die Bauart, die die Durchsicht als „Modul A
verlaesst sich stillschweigend darauf, dass Modul B einen Nebeneffekt hat"
beschreibt.

**Die Probe ist `die_gebaute_flaeche_steht_auf_textkit_1`**, und die Reihenfolge
ihrer beiden Fragen ist ihre ganze Aussage: gefragt wird zuerst
`textLayoutManager`, denn ein Zugriff auf `layoutManager` **loest** den Rueckfall
aus, und wer ihn zuerst fragte, machte seine eigene Antwort wahr. Die Zusicherung
aus dem Vorschlag ist deshalb umgedreht: `assert!(textLayoutManager().is_none())`
zuerst, `assert!(layoutManager().is_some())` danach.

**Die Gegenprobe ist gefahren, und sie hat mehr gezeigt als erwartet.** Mit
entfernter Zeile meldet die Probe „die Flaeche aus textflaeche_bauen steht auf
TextKit 2" und faellt aus. Das heisst: **vor** dieser Behebung war die Flaeche am
Ende von `textflaeche_bauen` tatsaechlich noch TextKit 2, und der Rueckfall
geschah erst Augenblicke spaeter im `darstellung_nachziehen` aus
`Editorbereich::bauen`. Die Zusage hing damit an der Reihenfolge zweier
Aufrufe in einer anderen Funktion, nicht nur an einem Nebeneffekt.

**Zwei Punkte bleiben offen, und beide sind Grenzen und keine Versehen:**

- **Der Verweis in der Gegenrichtung fehlt.** Der Datensatz verlangt, dass auch
  `nummernspalte.rs:89-90` weiss, dass an seinem Zugriff mehr haengt als die
  Nummern. Die Datei liegt ausserhalb der Dateigrenze dieser Aufgabe; der Verweis
  ist als eigener Datensatz abgelegt
  (`260810-1314_o_der-verweis-in-die-gegenrichtung-…`). Tragend ist er nach dieser
  Behebung nicht mehr: der Rueckfall haengt nicht an der Nummernspalte, sondern an
  der Zeile in `textflaeche_bauen`, und die Probe faellt aus, wenn sie fehlt.
- **`NSTextView.textLayoutManager` steht seit macOS 12**, also unter der
  Untergrenze 15.0; der Modulkopf fuehrt sie jetzt als dritte Methode, die
  juenger ist als ihre Klasse. `NSTextLayoutManager` selbst wird nirgends
  benannt.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0,
`cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0.

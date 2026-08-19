Kein Zeichen mit Umschalttaste erreicht den Dateifilter, also weder „_" noch ein Großbuchstabe

---

Der Nutzer hat gemeldet, dass der Dateifilter den Unterstrich nicht annimmt; er
braucht ihn, um nach Mustern wie `_o_` zu suchen. Die Zeichenregel ist es nicht:
`traegt_ein_dateiname` (`crates/krk-core/src/verzeichnis/filter.rs:74`) lässt den
Unterstrich durch. Der Tastendruck kommt dort gar nicht erst an.

---

**Schwere:** mittel. Kein Datenverlust, aber eine zugesagte Funktion ist für
einen großen Teil der Zeichen nicht erreichbar, und die Datensätze dieses
Projekts selbst tragen den Unterstrich in jedem Dateinamen.
**Gefunden von:** Nutzer, gemeldet am 260816-1130
**Betroffen:** `crates/krk-core/src/tasten/belegung.rs`, `nachschlag`
**Domain:** code

## Die Kette, ganz durchgesehen

Drei Stellen, jede am Baum gelesen:

1. `Tastendruck::aus_ereignis` (`crates/krk-core/src/tasten/mod.rs:98`) baut die
   Maske über `normalisieren`, und das nimmt die Umschalttaste als eines von
   vier Bits auf (`crates/krk-core/src/tasten/normalisierung.rs`).
2. `Belegung::nachschlag` (`crates/krk-core/src/tasten/belegung.rs:1161`) sucht
   zuerst eine belegte Kombination. Findet es keine, entscheidet **eine
   einzige Bedingung**, wohin der Druck geht:

   ```rust
   if druck.maske.ist_leer() {
       Nachschlag::Sprungmarke   // tippt in den Filtertext
   } else {
       Nachschlag::Unbelegt      // faellt an AppKit zurueck
   }
   ```

3. `behandeln` (`crates/krk-ui/src/appkit/ereignisse.rs:642-647`) reicht allein
   `Sprungmarke` als `Eingabe::Zeichen` an die Senke weiter; `Unbelegt` liefert
   `false`, und der Druck läuft unverändert an AppKit.

**Folge: jede Taste mit einer Zusatztaste ist für den Filter verloren**, sofern
sie nicht belegt ist. Auf einer deutschen Tastatur betrifft das nicht nur den
Unterstrich (Umschalt und Bindestrich), sondern auch `:` `;` `!` `?` `°`, jeden
**Großbuchstaben**, und über die Wahltaste `@` (Wahl und L), `|`, `~`, `\`. Alle
sind gültige Bestandteile eines Dateinamens.

**Der Großbuchstabe fällt nicht auf**, weil `traegt_die_folge` ohne Rücksicht auf
Groß- und Kleinschreibung vergleicht: wer `A` tippt und nichts bekommt, tippt `a`
und ist zufrieden. Beim Unterstrich gibt es diesen Ausweg nicht.

**Der Doc-Kommentar von `getipptes_zeichen` sagt das Gegenteil**
(`ereignisse.rs:729`): „getippt wird, was auf dem Bildschirm stuende, **samt
Grossschreibung**". Die Quelle ist richtig gewählt, `characters` liefert das
Zeichen mit Umschalttaste; es erreicht die Senke nur nie.

## Warum eine Erweiterung keine Belegung verschlucken kann

Die Fallunterscheidung steht **hinter** der Suche nach einer belegten
Kombination: `nachschlag` durchläuft erst alle Funktionen und kommt nur hierher,
wenn keine passt. Eine Erweiterung kann deshalb keinem belegten Kürzel etwas
wegnehmen.

## Zwei Zuschnitte

1. **Die Umschalttaste zählt für den Zeichenweg nicht als Zusatztaste.** Eine
   Bedingung mehr an einer Stelle. Deckt den gemeldeten Fall, die Großbuchstaben
   und die Umschalt-Interpunktion. Deckt die Wahltaste **nicht**, also `@`, `|`,
   `~`, `\` auf einer deutschen Tastatur.
2. **Umschalt und Wahl sind Schreibtasten, Befehl und Steuerung sind
   Befehlstasten.** Der Zeichenweg nimmt die ersten beiden an und die letzten
   beiden nicht. Deckt alles, was ein Nutzer tippen kann, und hält Cmd und Ctrl
   dort, wo sie hingehören: ein unbelegtes `cmd+irgendwas` tippt weiterhin
   nichts in den Filter. Der Zuschnitt ist der, den macOS selbst fährt.

Beide stehen hinter der Belegungssuche und ändern an keiner Kombination etwas.
Zuschnitt 2 ist der vollständigere und kostet dieselbe eine Stelle.

## Was zu prüfen ist, bevor gebaut wird

Ob `Nachschlag` seine Bedeutung behält: `Sprungmarke` heißt heute „Taste ohne
Zusatztaste, die keiner Funktion gehört". Trägt der Name die neue Bedeutung
nicht mehr, gehört er mitgeändert, samt der Prosa an beiden Stellen.

---

## Nutzerentscheid vom 260816-1105: Zuschnitt 2

**Umschalt und Wahl sind Schreibtasten, Befehl und Steuerung sind Befehlstasten.**
Der Zeichenweg nimmt die ersten beiden an, die letzten beiden nicht. Ein
unbelegtes `cmd+irgendwas` tippt weiterhin nichts in den Filter.

Damit erreichbar: `_`, die Umschalt-Interpunktion, jeder Großbuchstabe, und über
die Wahltaste `@`, `|`, `~`, `\`.

---

Resolved: Zuschnitt 2 gebaut, `make check` grün (exit 0). Die eine Bedingung in
`Belegung::nachschlag` (`crates/krk-core/src/tasten/belegung.rs`) fragt nicht
mehr, ob die Maske leer ist, sondern ob sie eine **Befehlstaste** hält:
`druck.maske.enthaelt(ModMaske::BEFEHL) || druck.maske.enthaelt(ModMaske::STEUERUNG)`.
Sie steht unverändert hinter der Belegungssuche, also kann sie keinem belegten
Kürzel etwas wegnehmen; `jede_belegte_kombination_wird_weiterhin_als_funktion_gefunden`
misst das an jeder Kombination der Auslieferungsbelegung und nicht an einem
Beispiel.

**Der Name ist mitgefallen.** `Nachschlag::Sprungmarke` heißt jetzt
`Nachschlag::Tippen`. Der Wert hatte den Namen über die Runde 10 hinweg
behalten, weil seine Aussage weiter zutraf — „eine Taste **ohne** Zusatztaste,
die keiner Funktion gehört" —, und genau dieser Satz ist mit diesem Entscheid
falsch geworden. Die Prosa ist an vier Stellen nachgezogen: dem Modulkopf von
`belegung.rs` (dort steht die Unterscheidung Schreibtaste/Befehlstaste jetzt
ausgeschrieben), dem Variantenkommentar, `crates/krk-ui/src/appkit/ereignisse.rs`
(Diagramm, Abschnitt „die zwei Fragen", der Zweig in `behandeln`, die Zeile des
Protokollmodus, die jetzt `(Tippen)` schreibt) und dem Modulkopf von
`crates/krk-core/src/verzeichnis/filter.rs`, dessen Verweis sonst ins Leere
gezeigt hätte. `traegt_ein_dateiname` selbst ist unangetastet: es entscheidet
weiter über das **Zeichen**, der Nachschlag über den **Tastendruck**.

**Was die Wahltaste liefert, ist gemessen und nicht angenommen** (260816, aktive
Belegung `com.apple.keylayout.German`, zwei Wege mit demselben Ergebnis —
`NSEvent` aus einem `CGEvent` gebaut und `characters` gelesen, und
`UCKeyTranslate` unmittelbar auf den Layoutdaten):

| Anschlag | `characters` | was im Filter ankommt |
|---|---|---|
| `opt+l` | `"@"` | `@` |
| `opt+u` (Trema, tote Taste) | `""` (leer) | nichts, `getipptes_zeichen` liefert `None` |
| `shift+a` | `"A"` | `A` |
| `ctrl+a` | `U+0001` | nichts, `ctrl` fällt auf `Nachschlag::Unbelegt` |

Eine tote Taste setzt also **kein** Akzentzeichen in den Filtertext, sondern gar
keines. Was der **nächste** Anschlag danach meldet — `ä` oder `a` —, entscheidet
der Totentastenstand des Ereignisstroms und nicht KRK; der Abgriff liest das
fertige `characters` und ruft `interpretKeyEvents:` nirgends. Beide Ausgänge
sind gutartig: es kommt in jedem Fall ein Zeichen an, das ein Dateiname tragen
kann. Der Befund steht im Doc-Kommentar von `getipptes_zeichen`.

**Was der Nutzer von Hand prüfen muss**, weil kein Agent einen wirklichen
Tastendruck am laufenden Bündel auslösen kann (`make run`, KRK im
Vordergrund, Fokus im Dateifenster):

1. Der gemeldete Fall: `_o_` in den Filter tippen und die Datensätze dieses
   Projekts wiederfinden.
2. Ein Großbuchstabe, etwa `A` — er verkürzt die Liste jetzt selbst und nicht
   nur, weil `a` daneben dasselbe täte.
3. `opt+l` für `@`.
4. `opt+u` — es darf **nichts** im Filtertext erscheinen; danach `a`, und was
   dann dasteht (`ä` oder `a`), ist die eine Frage, die diese Messung offen
   lässt.
5. Ein unbelegtes `cmd+<Buchstabe>` und ein unbelegtes `ctrl+<Buchstabe>` — der
   Filtertext darf sich nicht rühren.

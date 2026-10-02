# Ein Ordner traegt in der Dateiliste einen Schraegstrich hinter dem Namen

**Status:** Complete
**Agent:** coder
**Datum:** 260815-2110
**Quelldatensatz:** `shared/decisions/260815-2056_a_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`,
Abschnitt `## Nutzerentscheid vom 260815-2058` (Option 3)

## Auftrag

Die Namensspalte des Dateifensters bekommt hinter dem Namen eines Ordners einen
Schraegstrich. Er ist Anzeige und nie Name: Sortierung, Filter, Zwischenablage, Vorschau
und jede Dateioperation lesen weiter `eintrag.name`. Dasselbe `NSTextField` ist Zelle und
Editor des Umbenennens aus C4, und daran haengen die drei Zusagen des Entscheids: die
Bearbeitung beginnt ohne das Zeichen, Return liest den getippten Text unveraendert, Escape
stellt die Anzeigeform wieder her. Eine Verknuepfung auf einen Ordner bekommt kein Zeichen.
Grenze: eine Datei, `crates/krk-ui/src/appkit/tabelle.rs`. Kein Commit.

## Was gemessen wurde, bevor etwas geschrieben wurde

Der Entscheid nennt zwei Kandidaten fuer den Weg und misst keinen. **Beide sind am 260815
mit weggeworfenen Programmen auf dem wirklichen Hauptfaden nachgefahren**, mit einer
`NSTableView`, einer bearbeitbaren Zelle und `editColumn:row:withEvent:select:`. Das
Ergebnis kippt die Empfehlung des Datensatzes:

| Frage | gemessen |
|---|---|
| Kommt `control:textShouldBeginEditing:`, wenn die Bearbeitung beginnt? | **Nein.** Der Feldeditor steht, der Haken ist nicht gerufen worden. `NSText` schickt ihn erst beim ersten Aendern des Textes. |
| Kommt `controlTextDidEndEditing:` bei Escape? | **Nein.** Escape laeuft ueber `cancelOperation:` in `abortEditing`, ohne Meldung. |
| Kommt `controlTextDidEndEditing:` bei Return? | Ja, und zwar **vor** der Aktion des Feldes. |
| Stellt `abortEditing` den Stand vor der Bearbeitung wieder her? | Ja, und das ist genau der Stand, den der Beginn hinterlassen hat. |
| Laesst sich `stringValue` waehrend der Bearbeitung setzen, ohne den Feldeditor zu beruehren? | **Nein.** Ein `setStringValue:` waehrend der Bearbeitung schreibt in den Feldeditor zurueck. |
| Legt `+labelWithString:` ueber die empfangende Klasse an? | Ja. Auf einer Unterklasse gerufen liefert es eine Instanz der Unterklasse. |

Damit faellt Kandidat (a) des Datensatzes: der Haken, den er nennt, kommt beim Einstieg in
die Bearbeitung nicht. Kandidat (b) deckt den Klick ins Feld nicht ab. **Die dritte
Ordnung, die aus der Messung folgt, ist `becomeFirstResponder`:** AppKit haengt den
Feldeditor genau dort ein, gleich auf welchem Weg die Bearbeitung beginnt.

Die Messung hat daneben eine **Reihenfolgefalle** aufgedeckt, in die ein Zeichendurchgang
in `controlTextDidEndEditing:` gelaufen waere: die Meldung kommt vor der Aktion, also haette
`umbenennung_beenden` danach die Anzeigeform statt des getippten Textes gelesen und einen
Ordner auf `Name/` umbenennen wollen. Deshalb steht der Zeichendurchgang in `abortEditing`
und nicht dort.

## Was geaendert ist

### `crates/krk-ui/src/appkit/tabelle.rs`

- **Die Anzeige.** `ORDNERZEICHEN`, `namensform(&Eintrag) -> String` und
  `ohne_ordnerzeichen(&str) -> Option<&str>` sind neu, beide rein und beide `#[must_use]`.
  `DateifensterDelegierter::beschriften` ruft fuer `Spalte::Name` jetzt `namensform`. Die
  Bedingung ist `Eintrag::ist_ordner()`, also dieselbe wie beim `--` der Spalte `Groesse`;
  kein `stat` je Zeile, L3 und L10 bleiben unberuehrt.
- **Die neue Klasse `Namensfeld`**, eine Unterklasse von `NSTextField` mit zwei
  Ueberschreibungen und ohne Zustandsvariablen:
  - `becomeFirstResponder` nimmt das Zeichen von `stringValue`, **bevor** es die Fassung
    der Oberklasse ruft; erst diese haengt den Feldeditor ein und fuellt ihn aus der Zelle.
    Die Auswahl des Textes richtet AppKit danach selbst ein.
  - `abortEditing` ruft die Oberklasse und laesst danach die Zeile neu zeichnen. Der Weg
    dahin ist der Delegierte, den das Feld als Ziel seiner Aktion ohnehin schon haelt
    (`NSControl::target` plus `downcast`), und der Zeichendurchgang ist derselbe, mit dem
    schon eine abgelehnte Eingabe verschwindet.
- `DateifensterQuelle::umbenennung_abgebrochen` und `DateifensterDelegierter::namenszelle_zuruecksetzen`
  reichen die Zelle durch, wie es `umbenennungBeendet:` daneben tut.
- `DateifensterDelegierter::feld` baut fuer die beschreibbare Spalte ein `Namensfeld` und
  fuer die drei uebrigen weiter unmittelbar ein `NSTextField`. Es ist dieselbe Bedingung
  `Spalte::beschreibbar()` wie beim `setEditable(true)` daneben.
- Der Modulkopf nennt die dritte Klasse und traegt die Untergrenzen der drei neu
  angesprochenen Methoden nach: `becomeFirstResponder` (`NSResponder.h:105`), `abortEditing`
  (`NSControl.h:89`) und `target` (`NSControl.h:24`), alle seit 10.0.

## Proben

Zwei neue, beide rein und beide in `#[cfg(test)] mod tests` neben dem Code:

- `allein_ein_ordner_traegt_den_schraegstrich` — Ordner mit Zeichen, Datei ohne,
  Verknuepfung ohne.
- `die_anzeigeform_laesst_sich_auf_den_namen_zuruecknehmen` — der Weg, den
  `becomeFirstResponder` geht, fuer alle drei Arten, dazu die beiden Randfaelle des
  Schraegstrichs mitten im Text.

**Die drei Umbenenn-Zusagen stehen in keiner Probe, und das ist eine Eigenschaft der
Umgebung und keine Nachlaessigkeit.** Eine laufende Bearbeitung braucht einen Feldeditor,
ein Feldeditor braucht ein Fenster, und `NSWindow` wirft ausserhalb des Hauptfadens.
Nachgefahren: drei Fensterproben in diesem Prueflauf brachen mit `SIGABRT` ab, noch vor der
ersten Zeile Ausgabe. `MainThreadMarker::new_unchecked` behauptet den Hauptfaden nur
gegenueber Rust; eine `NSTextView` traegt die Behauptung, ein `NSWindow` nicht. Die Proben
sind wieder entfernt, der Befund steht im Kopf von `Namensfeld`.

## Nachweis am wirklichen Hauptfaden

Was `cargo test` nicht fahren kann, ist mit einem weggeworfenen Programm gefahren worden:
eine `NSTableView` mit derselben Verdrahtung wie in der Datei (Zellenwiederverwendung ueber
`makeViewWithIdentifier:owner:`, `Namensfeld` als Zelle, Ziel und Aktion am Feld, der
Zeichendurchgang der Zeile im Delegierten). Ergebnis, am 260815 auf macOS 15.7.7:

| Zusage | gemessen |
|---|---|
| Anzeige | Zeile 0 `"Bilder/"` (Ordner), Zeile 1 `"Ablage.rs"` (Datei), Zeile 2 `"Kurz"` (Verknuepfung) |
| 1: Beginn ohne Zeichen | Feldeditor `"Bilder"`, Auswahl 0..6, also der ganze Name |
| 2: Return liest den getippten Text | die Aktion bekommt `stringValue = "Fotos"` |
| 3: Escape stellt die Anzeigeform her | `abortEditing` zeichnet Zeile 0 neu, die Zelle steht wieder auf `"Bilder/"` |
| Gegenprobe Datei | Feldeditor `"Ablage.rs"`, unveraendert |

**Was auch dieser Nachweis nicht abdeckt** und beim Nutzer bleibt: der Klick ins Feld als
Einstieg in die Bearbeitung (er braucht ein echtes Mausereignis) und der Weg der Tasten
durch KRKs eigenen Ereignisabgriff. Der Nachweis stellt Return und Escape mit
`doCommandBySelector:` am Feldeditor nach, also eine Stufe hinter der Taste.

## Datensaetze

- `shared/decisions/260815-2056_a_…` bleibt vorerst `_a_`; die Umsetzung ist gebaut, aber
  die drei Zusagen sind am laufenden Buendel nicht abgenommen. Der Marker zieht nach, wenn
  der Nutzer sie geprueft hat.

## Pruefung

```
make check
```

Exit 0, „alle vier gruen": `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all --check`.
Keine neue Warnung, die beiden neuen Proben laufen mit.

## Offen

- **Die Abnahme am laufenden Buendel bleibt beim Nutzer.** Zu pruefen sind der Klick ins
  Feld als Einstieg in die Bearbeitung und die beiden Tasten auf ihrem wirklichen Weg
  durch KRKs Ereignisabgriff. Der Nachweis oben deckt alles darunter ab.
- **Angelegt:**
  `shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-bleibt-der-getippte-text-stehen-und-das-ordnerzeichen-weg.md`
  — der dritte Ausgang der Bearbeitung, der Fokusverlust, schickt weder Aktion noch
  `abortEditing`. Der Befund bestand vor dem Ordnerzeichen und ist seitdem sichtbarer; er
  liegt ausserhalb der drei Zusagen, und die Behebung braucht zuerst eine Nutzerantwort
  darauf, ob ein Klick daneben verwirft oder uebernimmt.
- **Kein Commit**, wie beauftragt. Der Entscheidungsdatensatz `260815-2056` traegt deshalb
  weiter `_a_`: der Weg von `_a_` nach `_i_` verlangt den Kurzhash der Umsetzung in der
  Zeile `Implemented:`, und den gibt es noch nicht.

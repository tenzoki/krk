# Die fünfte Automatik ist aus, und eine Probe hält fest, dass es fünf sind

---
**Agent:** coder
**Status:** Complete
**Anlass:** `issues/260809-1650_c_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`
**Umfang:** `crates/krk-ui/src/appkit/editor.rs` (Modulkopf, `textflaeche_bauen`, `mod tests`)
**Ergebnis:** `make check` grün, Rückgabewert 0 — Bau, Proben, `fmt --all --check` und `clippy --all-targets -D warnings`
**Geschlossen:** `issues/260809-1650_c_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`
**Neu:** `issues/260810-0512_o_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`

---

## Was geändert ist

Eine Zeile in `textflaeche_bauen`, abgesetzt von den vier bestehenden:

```rust
// Die fuenfte greift beim Einfuegen und Ausschneiden statt beim Tippen und
// steht deshalb fuer sich. Ab Werk ist sie **an**; ohne diese Zeile setzte
// ein Einfuegen ein Leerzeichen dazu, das niemand getippt hat.
text.setSmartInsertDeleteEnabled(false);
```

Der Modulkopf zählt jetzt fünf statt vier und benennt den Unterschied zwischen
den beiden Gruppen, statt die Zahl still hochzusetzen: vier greifen beim Tippen,
die fünfte beim Einfügen und Ausschneiden. Genau dieser Unterschied war der Grund
dafür, dass sie durchgerutscht ist — wer nach Automatiken sucht, die *das
Getippte* verändern, findet sie nicht.

## Der Vorgabewert ist gemessen und nicht angenommen

Der Datensatz führte den Vorgabewert `YES` als `speculation:` und die Messung als
Nutzerarbeit. Beides war zu pessimistisch: eine `NSTextView` entsteht auch ohne
Fenster, und `textflaeche_bauen` liefert sie fertig. Gemessen an genau dieser
Fläche steht `smartInsertDeleteEnabled` vor der neuen Zeile auf `true`. Die
Vermutung des Datensatzes stimmte.

**Die Messung ist danach wieder gefallen und steht nicht als Probe da.** Eine
Textfläche im Prüfstand entsteht außerhalb des Hauptfadens, und die Begründung an
`verwalter_ohne_fenster` sagt ausdrücklich, dass `MainThreadMarker::new_unchecked`
dort und sonst nirgends vertretbar ist — weil ein `NSUndoManager` an keinem
Fenster hängt. Eine `NSView` hängt am Fensterwerkzeug; die Ausnahme deckt sie
nicht. Eine Probe, die auf anderen Systemen bricht, wäre außerdem ein `make
check`, dem man nicht mehr glaubt.

## Die Suche nach einer sechsten

Nicht der Datensatz beantwortet die Frage, sondern die Objective-C-Laufzeit:
`AnyClass::get(c"NSTextView")` und `instance_methods()` zählen auf, was die
Klasse wirklich trägt. Zwölf Schalter der Form `set…Enabled:` auf macOS 15.6:

```
abgeschaltet (5)   setAutomaticQuoteSubstitutionEnabled:
                   setAutomaticDashSubstitutionEnabled:
                   setAutomaticTextReplacementEnabled:
                   setAutomaticSpellingCorrectionEnabled:
                   setSmartInsertDeleteEnabled:            <- die fünfte

geduldet (7)       setAutomaticLinkDetectionEnabled:       zeichnet aus, ändert nicht
                   setAutomaticDataDetectionEnabled:       dasselbe
                   setContinuousSpellCheckingEnabled:      vorübergehende Merkmale
                   setGrammarCheckingEnabled:              dasselbe
                   setAutomaticLanguageIdentificationEnabled:  wählt, woran gemessen wird
                   setAutomaticTextCompletionEnabled:      Kandidaten, der Nutzer wählt
                   setIncrementalSearchingEnabled:         wählt aus, schreibt nicht
```

**Eine sechste dieser Form gibt es nicht.** Zwei der sieben kannte der Datensatz
schon und hatte sie richtig eingeordnet; drei kannte er nicht
(`AutomaticLanguageIdentification`, `AutomaticTextCompletion`,
`IncrementalSearching`), und keine davon fasst den Textspeicher an.

## Was außerhalb der Form liegt, und warum es ein eigener Datensatz ist

`NSTextView` trägt seit macOS 15 die Schreibwerkzeuge — `writingToolsBehavior`
und drei Nachbarn. Sie **ersetzen markierten Text durch umgeschriebenen**, ihr
Vorgabewert überlässt dem System die Wahl, und der Zielwert des Bündels ist genau
die Fassung, in der es sie gibt. Sie tragen keinen Schalter der Form
`set…Enabled:` und fallen deshalb nicht unter die Probe.

Sie sind **nicht** mit behoben, und das mit Absicht: die fünf greifen ohne Zutun
des Nutzers, die Schreibwerkzeuge auf seinen ausdrücklichen Aufruf aus dem
Kontextmenü. Ob C4 sie trotzdem ausschließt, hängt an der Lesart von C4 — „kein
Zeichen ohne Zutun" oder „der gesicherte Stand ist der getippte" — und diese
Lesart bindet über den einen Schalter hinaus. Das ist eine Frage an den Nutzer:
`issues/260810-0512_o_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`.

Zwei weitere Nachbarn sind geprüft und nicht geführt: `setImportsGraphics:`
schaltet `setRichText(false)` von AppKit aus mit ab, und
`setEnabledTextCheckingTypes:` ist die gesammelte Maske über dieselben Prüfungen,
die die fünf Einzelschalter bereits abwählen.

## Die Proben

Zwei, beide in `crates/krk-ui/src/appkit/editor.rs` unter `mod tests`, beide ohne
Fläche und ohne Fenster:

- `keine_unbekannte_automatik_steht_an_der_textflaeche` — zählt zur Laufzeit auf,
  was die Klasse trägt, und hält es gegen `ABGESCHALTET` und `GEDULDET`. Ein
  dreizehnter Schalter aus einem späteren macOS hält den Bau an, bis jemand ihn
  eingeordnet hat.
- `die_fuenfte_automatik_steht_unter_den_abgeschalteten` — die fünfte steht in
  `ABGESCHALTET`, und keiner steht in beiden Aufstellungen. Der Mengenvergleich
  der ersten Probe fände einen doppelt geführten nicht.

**Die erste Probe ist negativ nachgewiesen**, nicht nur grün: mit einem
erfundenen Selektor in der Aufstellung fällt sie und nennt beide Mengen im
Klartext.

**Was sie nicht misst**, steht in ihrem Kommentar: ob die fünf Zeilen in
`textflaeche_bauen` stehen und was sie am laufenden Bündel bewirken. Das bleibt
Nutzerarbeit. Die Probe hält die Vollständigkeit der Aufzählung — und das ist
genau die Stelle, an der dieser Defekt entstanden ist.

## Nicht committet

Der Nutzer committet selbst, sobald die Abnahme durch ist.

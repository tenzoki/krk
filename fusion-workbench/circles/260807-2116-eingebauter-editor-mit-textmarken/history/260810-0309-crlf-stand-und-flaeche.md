# Der Stand und die Textfläche laufen nach einem eingefügten CRLF nicht mehr auseinander

**Status:** Complete
**Agent:** coder
**Domain:** code
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Behobener Defekt:** `issues/260810-0215_c_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`

---

## Die Ursache

Der Editor führt zwei Zeichenketten, die Zeichen für Zeichen dieselben sein
sollen: den gehaltenen Stand in `Editormodell` und den Text des `NSTextStorage`
der Textfläche. `Editormodell::bearbeiten` führte den Stand aus der Fläche seit
der Behebung von `260809-1646` durch `krk_core::text::datei::in_gehaltene_form`
— richtig und notwendig, sonst landete ein eingefügtes `\r\n` beim Sichern auf
der Platte. Die Wandlung schrieb aber nicht zurück. Wer Text aus einem
Windows-Projekt einfügte, hatte danach in der Fläche zwei Zeichen, wo der Stand
eines trug, und von der eingefügten Stelle an war jede Stelle um die Zahl der
`\r` verschoben. Seit S35, S36 und S37 rechnen vier Funktionen zwischen den
beiden Koordinaten — `suche_beginnen`, `stelle_zeigen`, `zeile_anspringen` und
`schreibmarkenzeile` —, und sie rechneten damit gegen den falschen Text.

## Die Wahl, die der Datensatz verlangt hat

Der Datensatz stellte zwei Wege gegenüber und verlangte eine Entscheidung statt
des nächstbesten Griffs.

**Genommen ist der Vergleich des Ergebnisses**, nicht der Filter am Eingang der
Fläche. `textView:shouldChangeTextInRanges:replacementStrings:` ist zwar der
Ort, den AppKit für die Frage vorsieht, müsste die Regeln der Wandlung aber ein
zweites Mal tragen — und es wären **nicht dieselben Regeln**: die
Bytefolgenmarke fällt nach ihrer Stelle im ganzen Text, ein eingefügtes Stück
kennt seine Stelle aber nur im Augenblick des Einfügens. Ein Löschen, das eine
Marke aus der Mitte an den Anfang rückt, ginge an einem solchen Filter vorbei
und brächte die beiden erneut auseinander. Der Vergleich des Ergebnisses prüft
dagegen die Zusage selbst und kommt ohne eine einzige Regel der Wandlung aus.

Die beiden Einwände des Datensatzes gegen den Vergleich sind damit umgegangen
oder benannt:

- **Die Schreibmarke bleibt stehen**, wo sie stand.
  `krk_core::text::datei::versatz_nach_der_wandlung` rechnet vom Ende her: was
  hinter einer Stelle steht, wandelt sich unabhängig von allem davor, also
  liefert die Länge des gewandelten Restes die neue Stelle, ohne dass die
  Rechnung wüsste, welche Zeichen wegfallen.
- **Der Rückgängigstapel bleibt der Preis.** Der Weg führt über
  `stand_erneuern` und damit über `setString:`. Ein `cmd+z` unmittelbar nach
  einem eingefügten `\r\n` wirkt gegen einen Stand, den die Fläche nicht mehr
  trägt. Es ist derselbe Preis, den das Ersetzen aus S37 schon zahlt, und
  `260809-1727` führt ihn; ein zweiter Schreibweg in die Fläche neben
  `stand_einsetzen` entsteht dafür nicht.

## Was geändert wurde

```
NSTextView ──> text_zurueckschreiben ──> Editormodell::bearbeiten ──> bool
                        │                                              │
                        │<─── „gewandelt“ ─────────────────────────────┘
                        v
                 flaeche_richten ──> versatz_nach_der_wandlung
                        │        ──> stand_erneuern (Fläche = Stand)
                        └────────────> stelle_zeigen (Schreibmarke)
```

**`crates/krk-core/src/text/datei.rs`**

- `ist_in_gehaltener_form(&str) -> bool` neu: die eine Stelle, die die Frage
  beantwortet. `in_gehaltene_form` nimmt an ihr ihren kurzen Weg, statt die
  Bedingung ein zweites Mal zu schreiben.
- `versatz_nach_der_wandlung(vorher, versatz, nachher) -> usize` neu: wohin eine
  Stelle wandert. Ein Fall geht um ein Zeichen daneben — zwei Bytefolgenmarken
  in einem Text, die erste ganz vorn — und steht im Doc-Kommentar, statt einen
  Sonderfall zu bekommen.
- Der Modulkopf trägt den Abschnitt „Wer aus einem Textbestand liest, muss ihn
  nachziehen“.

**`crates/krk-ui/src/editormodell.rs`**

- `bearbeiten` liefert ein `bool`: der hereingegebene Text war nicht in
  gehaltener Form, also trägt die Fläche jetzt andere Zeichen als der Stand.
  Der Wert kommt aus `ist_in_gehaltener_form` und nicht aus einem Vergleich
  zweier Zeichenketten, der eine Kopie des ganzen Standes voraussetzte.

**`crates/krk-ui/src/appkit/editor.rs`**

- `flaeche_richten` neu, gerufen allein aus `text_zurueckschreiben` und allein,
  wenn die Wandlung zugegriffen hat. Der gewöhnliche Anschlag kommt daran
  vorbei.
- Der Doc-Kommentar von `schreibmarke_in_utf16`, der den Defekt bisher als
  offen führte, nennt jetzt vier Wege, die Stand und Fläche zusammenhalten.

## Proben

| Probe | Datei |
|---|---|
| `die_frage_nach_der_gehaltenen_form_und_die_wandlung_sagen_dasselbe` | `krk-core/tests/text.rs` |
| `eine_stelle_wandert_mit_der_wandlung_in_die_gehaltene_form` | `krk-core/tests/text.rs` |
| `ein_eingefuegtes_crlf_meldet_sich_und_ein_gewoehnlicher_anschlag_nicht` | `krk-ui/src/editormodell.rs` |
| `nach_einem_eingefuegten_crlf_zeigt_dieselbe_stelle_in_beiden_texten_auf_dasselbe` | `krk-ui/src/appkit/editor.rs` |

Die letzte hält die Abweichung selbst fest: sie zeigt erst, dass dieselbe Zahl
in Fläche und Stand auf verschiedene Zeilen zeigt, und danach, dass nach dem
Richten jede Zeichengrenze des Standes über beide Koordinatenrichtungen wieder
auf sich selbst zurückkommt. Ein Fenster braucht sie nicht: die Fläche steht
als gewöhnliches `String` da.

## Abnahme

`make check` läuft mit Rückgabewert 0 durch: `cargo build --workspace`,
`cargo test --workspace` (714 Prüfungen, keine gefallen),
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`.

## Nicht getan

Nicht committet, auf Ansage des Nutzers. Kein neuer Defekt gefiled: der
Rückgängigstapel steht schon als `260809-1727` offen, und die Behebung fügt ihm
keinen neuen Mechanismus hinzu, sondern zahlt denselben Preis ein zweites Mal.

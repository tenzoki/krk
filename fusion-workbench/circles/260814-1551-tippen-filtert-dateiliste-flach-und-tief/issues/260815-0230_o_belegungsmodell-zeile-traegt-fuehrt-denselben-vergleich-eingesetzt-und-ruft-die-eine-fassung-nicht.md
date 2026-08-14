# `Belegungsmodell::zeile_traegt` führt denselben Vergleich eingesetzt und ruft die eine Fassung nicht

**Status:** Open
**Domain:** Filter der Runde 10 / Tippsuche der Belegungsansicht aus der Runde 7
**Filed by:** coder, beim Umsetzen von A2
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt A2; `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.3

## Befund

Schritt A2 ist gefahren, und der Vergleich des Filters steht danach an genau einer
Stelle: `krk_core::verzeichnis::filter::traegt_die_folge`, gerufen von
`Ordnermodell::sichtbar` und vom Durchlauf. Das ist, was A2 verlangt hat.

**Eine dritte Stelle im Baum führt dieselbe Regel weiterhin eingesetzt**, und sie ist
diejenige, die der Spec als Maßstab nennt. `crates/krk-ui/src/belegungsmodell.rs:547`,
in `Belegungsmodell::zeile_traegt`:

```rust
.any(|text| text.to_lowercase().contains(gesucht))
```

Das ist Zeichen für Zeichen der Rumpf von `traegt_die_folge`, samt derselben
Vereinbarung über die Argumente: `gesucht` kommt bereits kleingeschrieben herein, damit
die Umschreibung einmal je Suche läuft und nicht einmal je Zeile. C1.3 des Spec sagt
über den Vergleich des Filters wörtlich: „Es ist derselbe Vergleich, den
`Belegungsmodell::zeile_traegt` führt."

## Warum er nicht mitgezogen ist

Drei Gründe, und keiner davon ist „übersehen":

1. **A2 verlangt es nicht.** Der Schritt nennt zwei Stellen, die die eine Fassung rufen
   sollen, und beide sind es jetzt. `belegungsmodell.rs` steht in seiner Dateiliste
   allein für den `use`-Pfad der Zeichenregel.
2. **C1.4 lässt die Runde-7-Seite ausdrücklich unangetastet:** die Tippsuche der
   Belegungsansicht „liest **unverändert** dieselbe Funktion". Das ist über die
   Zeichenregel gesagt, und ein Umbau ihres Vergleichs im selben Zug ginge darüber
   hinaus.
3. **Die zwei Fragen sind nicht dieselbe Frage.** `traegt_die_folge` beantwortet „trägt
   dieser **Dateiname** den Filtertext"; `zeile_traegt` beantwortet „trägt eine der zwei
   **Spalten dieser Zeile** den Suchtext". Der Rumpf ist gleich, der Gegenstand nicht,
   und das Modul, in dem die eine Fassung wohnt, heißt nach dem Filter der Dateiliste.

## Was daran trotzdem ein Befund ist

Der Baum trägt damit zwei wortgleiche Fassungen einer Regel, deren Gleichheit ein
Abnahmekriterium behauptet. Läuft eine der beiden auseinander — jemand faltet in der
einen die Umlaute, jemand vergleicht in der anderen ohne Umschreibung —, dann ist C1.3
still falsch geworden, und keine Probe im Baum sagt es. Die Zählprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
(`crates/krk-core/tests/verzeichnis.rs`) hält die **Rufer** der einen Fassung fest; eine
zweite Fassung unter anderem Namen und in anderer Schreibweise findet sie
ausdrücklich nicht, und der Kopf von `tests/baum.rs` schreibt aus, warum keine Nadel das
leisten kann.

## Vorschlag

Eine Zeile, und der Schnitt ist zu entscheiden, nicht zu wählen:

```rust
.any(|text| traegt_die_folge(text, gesucht))
```

Dafür wäre zu klären, ob `traegt_die_folge` „trägt dieser Name den Filtertext" heißt
oder „trägt dieser Text die gesuchte Folge". Das Zweite ist die allgemeinere und
zutreffendere Aussage; sie hieße, den Doc-Kommentar der Funktion zu weiten und die
Zählprobe um `krk-ui/src/belegungsmodell.rs` als dritten Rufer zu ergänzen. Ein eigener
Schritt, keine Nacharbeit an A2.

**Gegen den Umbau spricht**, dass der Filter der Dateiliste und die Tippsuche der
Belegungsansicht sich damit eine Funktion teilen, die in `krk-core` wohnt und nach dem
Filter benannt ist; wer künftig eine der beiden Suchen ändern will, ändert die andere
mit. Genau das ist der Zweck einer einzigen Quelle, aber es ist eine Bindung, die heute
nicht besteht, und deshalb ist es eine Frage an den Nutzer und keine an den Umsetzer.

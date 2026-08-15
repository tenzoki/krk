# Die Auffrischung schiebt auf, solange eine Namenszelle in Bearbeitung steht

**Agent:** coder
**Status:** Complete
**Auftrag:** T4, Umsetzung des Nutzerentscheids vom 260816-0021
(`shared/decisions/260815-2247_a_was-geschieht-mit-einer-offenen-umbenennung-die-ohne-aktion-endet.md`,
Option 1)
**Dateigrenze:** `crates/krk-ui/src/auffrischung.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/tabelle.rs` — eingehalten, keine vierte Datei am Code

---

## Ergebnis in einem Satz

Solange in einem Dateifenster eine Namenszelle bearbeitet wird, liest es seinen Ordner
nicht neu, sondern merkt die ausgefallene Auffrischung vor; das Ende der Bearbeitung holt
sie nach. Der Takt eines schon laufenden Lesevorgangs ist damit **nicht** abgedeckt und
liegt als eigener Befund.

## Die Naht: `ordner_neu_lesen` und nicht `aufgeschobene_ordner`

Der Auftrag nennt `Anwendungsdelegierter::aufgeschobene_ordner` als naheliegenden Weg —
eine zweite Quelle für dieselbe Liste. Geprüft und **verworfen**, aus drei Gründen, von
denen jeder für sich trägt:

1. **Die Liste ist pfadbezogen, der Aufschub ist fensterbezogen.**
   `auffrischung_aufgeschoben` beantwortet einen gemeldeten Pfad, und der Rufer
   überspringt ihn dann für **beide** Dateifenster. Zeigen beide denselben Ordner und
   steht die Zelle nur links offen, hielte das auch das rechte an. Der Auftrag verlangt
   das Gegenteil: „Der Aufschub gilt für den Ordner des betroffenen Dateifensters, nicht
   global."
2. **Die Liste hängt an einem Auslöser, der Entscheid an beiden.**
   `aufgeschobene_ordner` wird allein im Rückruf der Dateisystemwache abgefragt. Der
   Abschluss einer Dateioperation (S16) ruft `ordner_neu_lesen` unmittelbar und käme an
   ihr vorbei. Der Entscheid sagt: kein Programmweg dieser Datei beendet die Bearbeitung.
3. **`ordner_neu_lesen` ist der eine Auffrischungspfad**, sagt sein eigener
   Doc-Kommentar. Eine Frage, die für jeden Auffrischungsweg gilt, gehört dorthin.

Die Entscheidung selbst steht damit weiter in `auffrischung.rs` und außerhalb von AppKit;
die Trennung, die `aufgeschobene_ordner` für den Vorgang festhält — die Aufzählung gehört
dem Rufer, die Entscheidung steht ohne Fenster —, ist unverändert. Vier neue Proben in
`cargo test` decken sie ab.

## Der Nachhol-Weg

Neu, weil es beim Vorgangsaufschub nichts Vergleichbares gibt. Zwei Kennzeichen an der
Quelle, beide `Cell<bool>`:

- `namensbearbeitung` — gesetzt in `Namensfeld::becomeFirstResponder`, gelöscht an den
  beiden Enden, die AppKit hat.
- `auffrischung_vorgemerkt` — gesetzt statt zu lesen, eingelöst am Ende der Bearbeitung,
  **gelöscht von jedem wirklichen `neu_lesen`**. Letzteres ist der Punkt, an dem die
  Auffrischung, die die Umbenennung selbst auslöst, das Nachholen überflüssig macht statt
  es zu verdoppeln; ohne diese Zeile liefe nach jedem Return ein zweiter Lesevorgang, der
  dem ersten seine vorgemerkte Auswahl nähme (`Tabliste::auswahl_auf_namen` merkt nur vor,
  solange gelesen wird).

## Messung 1: die beiden Enden einer Bearbeitung

Am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem wirklichen Hauptfaden,
an einer `NSTableView` in einer `NSScrollView` mit derselben Verdrahtung wie in der Datei.

| Anlass | Bearbeitung danach | Text in der Zelle | Rückrufe in dieser Reihenfolge |
|---|---|---|---|
| Return (`insertNewline:`) | beendet | `getippt` | `textDidEndEditing:` (NSTextMovement=16) → Aktion `umbenennungBeendet:` |
| Escape (`cancelOperation:`) | beendet | `alpha` | `abortEditing` |
| Fokusverlust (`makeFirstResponder:` auf die Tabelle) | beendet | `getippt` | `textDidEndEditing:` (NSTextMovement=0) |
| `reloadData` | beendet | — | `textDidEndEditing:` (NSTextMovement=0) |
| `reloadDataForRowIndexes:columnIndexes:` | beendet | `alpha` | `textDidEndEditing:` (NSTextMovement=0) |
| `selectRowIndexes:byExtendingSelection:` | beendet | `getippt` | `textDidEndEditing:` (NSTextMovement=0) |
| `noteNumberOfRowsChanged` | steht weiter | `getippt` | — |
| `makeFirstResponder:` auf das Fenster | beendet | `getippt` | `textDidEndEditing:` (NSTextMovement=0) |

**Sieben Enden, zwei Rückrufe, keine Lücke.** `-[NSTextField textDidEndEditing:]` trägt
jedes Ende außer Escape, `abortEditing` trägt Escape. `controlTextDidEndEditing:` wäre die
falsche Stelle, weil die Delegiertenmeldung **vor** der Aktion kommt und
`textDidEndEditing:` die Aktion **schickt**. Die Fallunterscheidung ist damit vollständig
und braucht keinen dritten Haken.

`abortEditing` läuft nebenbei auch an Feldern, die gar nichts bearbeiten — jedes
`reloadData` ruft es. Der vorhandene Rückgabewert-Vorbehalt (`if abgebrochen`) trägt das
schon und trägt jetzt auch das Kennzeichen.

## Messung 2: warum das Nachholen nach `super` steht

Dieselbe Vorrichtung, ein Zeichendurchgang an drei Stellungen:

| Stellung des `reloadData` | Ablauf |
|---|---|
| keiner (heutiger Stand) | AKTION: `rowForView`=0, `stringValue`=`"getippt"` |
| **vor** `super` | reloadData → AKTION: `rowForView`=**-1**, `stringValue`=`"getippt"` |
| **nach** `super` | AKTION: `rowForView`=0, `stringValue`=`"getippt"` → reloadData |

Ein Zeichendurchgang vor `super` nimmt dem Feld seine Zeile; `umbenennung_beenden` kehrt
dann an seinem `usize::try_from` um, und die Umbenennung fiele still aus. Das Kennzeichen
fällt deshalb vor `super`, nachgeholt wird nach `super`.

## Die zweite offene Frage des Entscheids: der Takt des Lesevorgangs

**Gemessen und benannt: der Aufschub deckt ihn nicht ab.** `DateifensterQuelle::einziehen`
ruft `reloadData` und `auswahl_anzeigen` unmittelbar an der Tabelle und läuft an
`ordner_neu_lesen` vorbei.

**Mit abgedeckt wurde er nicht, und das ist eine Entscheidung mit Grund.** Die Frage
(„steht hier eine Zelle offen?") ist dieselbe, die Folge ist es nicht: beim behobenen Weg
bleibt das Ordnermodell unangetastet, beim Takt ändert es sich unter der offenen Zelle.
`einzug.fertig` sortiert um, und `umbenennung_beenden` liest seinen alten Namen über
`rowForView:` aus dem Modell — ein Aufschub über die Umsortierung hinweg benennte **eine
andere Datei** um. Das ist schwerer als das, was behoben werden soll.

Abgelegt als
`shared/issues/260816-0040_o_der-takt-eines-laufenden-lesevorgangs-beendet-eine-offene-namenszelle-und-der-aufschub-erreicht-ihn-nicht.md`
mit drei Wegen; welcher es wird, ist eine Nutzerfrage.

## Der Präzedenzfall 260805-1337

Geprüft. Der Aufschub kann die Liste nicht leerlaufen lassen: er verhindert, dass ein
Lesevorgang **beginnt**. `Ordnermodell::lesevorgang_beginnen` wird nicht gerufen, das
Modell behält seinen Bestand, die Tabelle behält jede Zeile. Die leere Liste von damals
entstand am entgegengesetzten Ende — ein Lesevorgang, der begann und vorab leerte.

## Was am Befund 260815-2125 abgetragen ist

Zwei der drei Ausgänge, nämlich die beiden ohne Zutun des Nutzers (Dateisystemwache und
Abschluss einer Dateioperation). Der dritte, der wirkliche Klick neben die Zelle, ist
unberührt und wartet auf
`shared/decisions/260816-0021_o_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`.
Die Anzeigehälfte des Befunds — der getippte Text bleibt nach einem Klick daneben stehen —
ist ebenfalls unberührt; sie hängt an derselben Frage.

## Abnahme

`make check` — exit 0 (Bau, alle Proben grün — 1.187 über alle Prüfziele —, clippy
unter `-D warnings`, `fmt --check`).
Vier neue Proben in `auffrischung.rs`.

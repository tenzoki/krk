# Jedes Ende der Bearbeitung ohne Umbenennung stellt die Anzeigeform wieder her

**Agent:** coder
**Status:** Complete
**Auftrag:** T5, Umsetzung des Nutzerentscheids vom 260816-0935
(`shared/decisions/260816-0021_a_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`,
Option 1: ein Klick neben die offene Namenszelle verwirft, wie Escape)
**Dateigrenze:** `crates/krk-ui/src/appkit/tabelle.rs` — eingehalten, eine Datei am Code

---

## Ergebnis in einem Satz

`-[Namensfeld textDidEndEditing:]` stellt nach `super` bedingungslos die Anzeigeform der
Zeile her, über dieselbe Methode, die Escape schon rief; damit trägt jedes Ende einer
Bearbeitung, dem keine Umbenennung folgt, die Zusage, und keines mehr behauptet eine
Umbenennung, die nicht stattgefunden hat.

## Die Naht: eine Methode für beide Enden, nicht zwei

`DateifensterQuelle::umbenennung_abgebrochen` konnte das schon, hieß aber nach dem einen
Weg, den es hatte. Sie heißt jetzt `anzeigeform_herstellen` und hat zwei Rufer über den
Delegierten (`DateifensterDelegierter::anzeigeform_herstellen`, ebenfalls umbenannt):
`Namensfeld::bearbeitung_abbrechen` für Escape, `Namensfeld::bearbeitung_beendet` für
jedes übrige Ende. Ein zweiter Mechanismus daneben ist nicht entstanden; der
Zeichendurchgang bleibt `zeile_neu_zeichnen`, also der Weg, auf dem auch eine abgelehnte
Eingabe verschwindet.

**Bedingungslos und nicht "nur wenn keine Aktion kam".** Eine solche Fallunterscheidung
wäre eine Regel ohne Wirkung; die Messungen C, F und G unten zeigen, dass der zweite
Durchgang nach Return folgenlos ist und nach einer ausgelösten Auffrischung ohnehin still
ausfällt. Damit bleibt die Fallunterscheidung der Datei vollständig ohne einen dritten
Zweig.

## Messung: ein weggeworfenes Programm auf dem wirklichen Hauptfaden

Am 260816 auf macOS 15.7.7, `NSTableView` in einer `NSScrollView`, beschreibbare
Namensspalte, Zellenansichten über `makeViewWithIdentifier:owner:` wiederverwendet, Aktion
`umbenennungBeendet:` am Feld, Unterklasse mit denselben drei Überschreibungen — also
dieselbe Verdrahtung wie in `tabelle.rs`. Jede Zeile trägt einen Ordner, die Anzeigeform
ist also `name/`, der getippte Text `getippt`.

| | Ausgang | Zeichendurchgang nach `super` | `rowForView` dabei | Zelle danach | Modell danach |
|---|---|---|---|---|---|
| A | Fokusverlust | an | 0 | `alpha/` | `alpha` |
| B | Fokusverlust | **aus** (Stand vor dieser Änderung) | — | **`getippt`** | `beta` |
| C | Return, Name geändert, Auffrischung folgt | an | **-1**, fällt still aus | `gamma-neu/` | `gamma-neu` |
| D | Fokusverlust, Auffrischung vorgemerkt | an, danach das Nachholen | 3 | `delta/` | `delta` |
| E | Escape | an (der bisherige Weg) | 4 | `epsilon/` | `epsilon` |
| F | Return, Name unverändert | an, zweiter Durchgang | 5 | `zeta/` | `zeta` |
| G | Return, Name geändert, keine Auffrischung | an, zweiter Durchgang | 6 | `eta-neu/` | `eta-neu` |
| H | wie D, aber Nachholen **vor** dem Durchgang | an | **-1**, fällt still aus | `theta/` | `theta` |

**A gegen B ist der Befund und seine Behebung** in einer Zeile: derselbe Ausgang, einmal
mit und einmal ohne den Zeichendurchgang, und nur mit ihm steht die Anzeigeform wieder da.

**C ist der Fall, in dem der Durchgang nichts tut, und das ist richtig so.** Die Aktion
löst über `umbenennen_ausfuehren` synchron eine Auffrischung aus; deren `reloadData`
nimmt dem Feld seine Zeile, `rowForView` liefert danach -1, und der Wächter
`usize::try_from` kehrt um. Die Anzeigeform steht trotzdem, weil der Zeichendurchgang der
Auffrischung sie geholt hat.

**F und G sind der zweite Durchgang, den der Entscheid als folgenlos angenommen hat.** Er
ist es: das Feld behält seine Zeile, die Zelle zeigt danach den Stand des Modells.

## Die geprüfte Frage: kommen Zeichendurchgang und Nachhol-Weg einander in die Quere?

**Nein**, und der Grund ist eine Eigenschaft und keine Messung allein: ein
Zeichendurchgang ruft `zeile_neu_zeichnen` und nicht `neu_lesen`. Er beginnt keinen
Lesevorgang, fasst `auffrischung_vorgemerkt` nicht an und liest allein das Modell. Die
zwei Lesevorgänge hintereinander, wegen derer es das Kennzeichen aus `27dca57` gibt,
entstehen daraus nicht. Messung D fährt beides zusammen nach: Durchgang, dann Nachholen,
danach steht `alpha/`-Form in der Zelle und das Kennzeichen auf `false`.

**Die Reihenfolge ist trotzdem gewählt und nicht beliebig.** Messung H dreht sie um: läuft
das Nachholen zuerst, nimmt sein `reloadData` dem Feld die Zeile, und der Durchgang fällt
still aus. Die Anzeigeform steht dort zwar auch — geholt hat sie aber das `reloadData`,
und das gibt es nur, wenn etwas vorgemerkt war. Die Zusage hinge damit an einer Bedingung,
die sie nicht haben soll. Deshalb: Durchgang, dann Nachholen.

## Der Nebenbefund: die Zuschreibung an C4

Der Doc-Kommentar von `umbenennung_beenden` zitierte "Return übernimmt, Escape verwirft"
als Wortlaut von C4. Das Abnahmekriterium sagt allein "ein Tastenbefehl benennt den
ausgewählten Eintrag um, direkt in der Liste"
(`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:254`);
der Satz stammt aus dem **Plan** der Runde 1. Der Kommentar sagt das jetzt, führt den
dritten Ausgang samt Entscheid und hält fest, dass ein Ende ohne Return überhaupt keine
Aktion schickt — bis zum 260816 stand dort "oder die Zelle verlässt", und das widersprach
der eigenen Messtabelle der Datei. Der Plan der Runde 1 ist unangetastet: er ist die
Aufzeichnung seines damaligen Standes.

## Was nicht angefasst ist

`shared/issues/260816-0040` (der Takt eines laufenden Lesevorgangs) und
`shared/issues/260815-2202` (der falsche L3/L10-Satz), wie beauftragt. Der Takt endet
inzwischen ebenfalls über `textDidEndEditing:` und stellt damit die Anzeigeform her; was
dort offen bleibt, ist der **Verlust des getippten Textes**, und genau den führt sein
eigener Datensatz.

Keine neu angesprochene AppKit-Methode: `rowForView:` (10.7) und
`reloadDataForRowIndexes:columnIndexes:` (10.6) stehen im Modulkopf schon.

## Abnahme

`make check` — exit 0 (Bau, 1.187 Proben über alle Prüfziele grün, clippy unter
`-D warnings`, `fmt --check`). Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` lief im selben Zug durch.

**Was der Nutzer von Hand prüfen muss:** den wirklichen Klick mit der Maus neben eine
offene Namenszelle. Das Messprogramm fährt jedes Ende programmatisch nach
(`makeFirstResponder:` auf die Tabelle für den Fokusverlust); ein echtes Mausereignis
kann kein Agent erzeugen. Was er sieht, sollte die Zeile A sein: die Zelle nimmt ihren
Namen samt Schrägstrich zurück, umbenannt wird nichts.

# Sitzung: Der Klick unter die letzte Zeile holt das aktive Dateifenster

**Datum:** 2026-08-19 12:10
**Agent:** coder
**Auftrag:** Den Defekt `260819-1043` beheben, nach Möglichkeit 1 des Entscheidungsdatensatzes `260819-1043`
**Status:** Complete

## Was gelaufen ist

Ein Klick in die freie Fläche unter der letzten Zeile eines Dateifensters ließ
`aktiv` auf der anderen Seite stehen, und die Fokusanzeige aus C9 malte den Rahmen
damit auf die Liste, in die niemand geklickt hatte. Der Nutzerentscheid vom 260819
lautet: jede Fläche eines Bereichs holt den Fokus, und ein Klick in eine Dateiliste
macht sie zur aktiven, ob er eine Zeile trifft oder nicht.

Gebaut ist genau das, in drei Handgriffen und ohne eine neue Mechanik.

## Wo der dritte Anlass sitzt, und warum dort

**Am Ersthelferwechsel des Hauptfensters, nicht an der Tabelle.** Die Analyse hat
am Nachbau gemessen, dass AppKit den Klick auf die freie Fläche von sich aus in ein
`makeFirstResponder:` übersetzt und die Tabelle den Rang annimmt. KRK muss den Klick
also nicht abfangen; es muss nur auf den Rangwechsel hören, und dafür gibt es seit C9
genau einen Auslösepunkt, `Hauptfenster::makeFirstResponder:` → `melden`. Ein
`mouseDown:` an der Tabelle wäre die zweite Tür, die deren Modulkopf ausschließt, und
träfe die Lesezeichenleiste ohnehin nicht mit.

Der Melder hatte bisher einen Empfänger, `fokusanzeige_nachziehen`. Er hat jetzt zwei,
und der neue steht davor.

## Was geändert ist

`crates/krk-ui/src/appkit/anwendung.rs`

- `ersthelferbereich` ist in zwei Teile zerlegt. Der Durchgang über `Bereich::ALLE`
  steht als `bereich_des_ersthelfers` und liefert den **Bereich**; `ersthelferbereich`
  ist nur noch dessen Übersetzung in einen Fokuswert. Der Grund: der Fokuswert wirft
  die beiden Dateifenster zusammen, und gerade die Unterscheidung zwischen ihnen ist
  die gesuchte Auskunft. Die drei Rückfallzweige auf `Fokus::Dateifenster` sind zu
  einem `None` zusammengelaufen; das Verhalten ist unverändert.
- `aktives_dem_ersthelfer_nachziehen` ist neu. Es fragt `bereich_des_ersthelfers`,
  filtert über `Bereich::seite` — die eine Stelle, die aufzählt, welche Bereiche
  Dateifenster sind — und ruft `aktives_setzen`. Liegt der Rang in der Leiste, der
  Vorschau oder dem Editor, geschieht nichts, und `AktivOhneFokus` bleibt erhalten.
- Der Melder des Hauptfensters ruft es **vor** `fokusanzeige_nachziehen`, damit die
  Anzeige schon mit dem neuen `aktiv` rechnet. Das nimmt nebenbei das Flackern weg,
  das die Analyse für den Klick auf eine Zeile beschreibt.

`crates/krk-ui/src/appkit/tabelle.rs`

- Der Doc-Kommentar an `tableView:shouldSelectRow:` ist umgeschrieben. Er behauptete,
  die Umschaltung stehe **genau deshalb** dort, weil AppKit nur eine vom Nutzer
  ausgehende Auswahl meldet. Das galt für den Klick auf eine Zeile und wurde
  stillschweigend für die ganze Regel genommen. Der Kommentar sagt jetzt, was der Ruf
  weiter deckt und was seit dem 260819 daneben steht.

**Die Zeile bleibt stehen und ist keine Wiederholung.** Der Weg über den Ersthelfer
greift nur, wenn der Rang wirklich wechselt; verweigert der bisherige Ersthelfer ihn —
der Editor kann das über seinen Delegierten —, ist `shouldSelectRow:` die einzige
Zeile, die den Klick auf eine Zeile noch umsetzt. Beide münden in `aktives_setzen`,
und die zweite Meldung ist dort folgenlos.

## Was ausdrücklich nicht gebaut ist

- **Keine `mouseDown:`-Überschreibung**, an keiner Ansicht. Die Analyse hat sie als
  toten Code nachgewiesen.
- **Keine Abfrage auf ein stehendes Blatt** in `aktives_dem_ersthelfer_nachziehen`.
  Ein Blatt ist modal zu seinem Fenster, sein Ersthelfer liegt im Blatt, und AppKit
  lässt währenddessen keinen Klick an die Bereiche dahinter. Der Zweig wäre unerreichbar.
  Die Begründung steht am Rumpf.
- **Keine zweite Auflösung von `Fokus::Dateifenster` auf einen Bereich.**
  `bereich_mit_fokus` bleibt die eine; der Entscheidungsdatensatz hat Möglichkeit 3
  genau deshalb verworfen.

## Die Probe, und dass sie feuert

`fokusnachzugproben` in `anwendung.rs`, zwei Prüfungen am Rumpf von
`fokusanzeige_nachziehen` über die schon vorhandene `rumpf`-Hilfe aus `zettelproben`.

Die erste hält den Ring offen: der Nachzug der Anzeige darf `anwenden`, `setHidden`,
`aufteilung_nachziehen` und `aktives_setzen` nicht ansprechen. **Seit dem 260819 trägt
sie ihr Gewicht wirklich.** Bis dahin war der Nachzug der einzige Empfänger der
Fenstermeldung, und die Frage stellte sich niemandem; seither hängt
`aktives_dem_ersthelfer_nachziehen` als zweiter daran, und der geht über
`aktives_setzen` sehr wohl bis `anwenden` durch. Die beiden nebeneinander statt
ineinander zu stellen ist der ganze Unterschied zwischen einem Ring und keinem, und das
lädt dazu ein, sie beim nächsten Mal zusammenzulegen.

Die zweite ist die Gegenprobe: eine Funktion, die nichts mehr täte, bestünde die erste
mühelos. Sie verlangt `rahmen_setzen` und `titel_nachziehen`.

**Beide sind absichtlich rot gemacht worden.** `self.titel_nachziehen(fokus)` wurde
durch `self.aufteilung_nachziehen()` ersetzt; die erste schlug auf
`aufteilung_nachziehen(` an, die zweite auf das fehlende `titel_nachziehen(`. Danach
zurückgesetzt und wieder grün.

Der Klick selbst braucht die Oberfläche und ist Nutzerarbeit; dafür steht hier keine
Probe, die etwas anderes behaupten würde.

## Verification

`make check` — exit 0. 1359 Prüfungen grün, Clippy unter `-D warnings`, `cargo fmt
--all --check` sauber. Vor dem Lauf geprüft: weder `/tmp` noch `$TMPDIR` trug eine
`krk-messplan-*.toml`, es lief also kein Messlauf, dem der Prüflauf den Plan abgeräumt
hätte.

## Was der Nutzer abnimmt

1. `cargo xtask bundle`, KRK starten, beide Dateifenster sichtbar.
2. In die **freie Fläche unter der letzten Zeile** des nicht aktiven Dateifensters
   klicken. Der Fokusrahmen muss auf diese Liste springen, und der Fenstertitel muss
   ihren Ordner zeigen.
3. Danach `F5` drücken: die Quelle muss diese Liste sein, auch ohne Auswahl darin.
   Das ist die mitentschiedene Folge und kein Defekt.
4. Gegenprobe: in die Vorschau oder den Editor klicken. Der Fokusrahmen wandert
   dorthin, das aktive Dateifenster bleibt stehen und behält seinen schwächeren Rahmen.

## Nicht getan, absichtlich

Nicht committet, und der Defektdatensatz `260819-1043` ist nicht geschlossen. Beides
macht der Nutzer.

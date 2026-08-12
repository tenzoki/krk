# Coder, Schritt 2: Die zweite Abweisung, und der Defekt aus Schritt 1

**Datum:** 260812-0512
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 2
**Abnahme:** `make check` — Exit 0

## Auftrag

Zwei Sachen in einem Zug, und keine dritte. Schritt 2 des Plans: `Fenstermodell::umschalten`
bekommt neben der Abweisung am letzten Dateifenster eine zweite an den Mindestbreiten, und dafür
reist die Geometrie der Fensterzeile als `Zeilenmass` bis in das Modell. Dazu die Behebung des
Defekts, den Schritt 1 abgelegt hat
(`issues/260812-0439_*_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`):
der Breitenschritt aus C7 kam skaliert auf dem Schirm an. Beide gehen durch dieselbe
Durchreichung, und genau deshalb standen sie in einem Auftrag. Nicht committen.

## Was entstanden ist

**`crates/krk-ui/src/fenstermodell.rs`**

- `umschalten(&mut self, bereich, mass: Zeilenmass) -> bool` trägt die zweite Abweisung. Sie greift
  **nur beim Einschalten**: ein Ausschaltbefehl kann die Summe der Mindestbreiten nicht
  vergrößern, und ihn an derselben Bedingung scheitern zu lassen hielte ein zu schmal gezogenes
  Fenster in seiner Enge fest.
- Die Rechnung steht in der neuen `mindestbreiten_passen`. Sie bildet die Menge **nach** dem
  Einschalten — der genannte Bereich steht, sein Gegenüber aus `teilt_flaeche_mit` steht nicht, der
  Rest bleibt — und liest aus dieser einen Menge beide Größen: die Summe der Mindestbreiten und die
  Anzahl der Trennlinien, die `Zeilenmass::verfuegbar` abzieht. Dass die Anzahl gleich bleiben kann,
  ist der Grund, aus dem die Frage im Modell steht und nicht beim Aufrufer.
- `einblenden` reicht das Maß durch, weil es durch `umschalten` geht.
- **Beide tragen jetzt `#[must_use]`.** Der Rückgabewert ist mit der zweiten Abweisung zu einer
  Auskunft geworden, deren stilles Fallenlassen unbemerkt bliebe: wer ihn nicht liest, hält eine
  Sichtbarkeit für hergestellt, die das Modell nicht angenommen hat. Das ist die Projektregel seit
  dem 260811-2140 und dieselbe, die der Plan in Schritt 7 für `spalte_umschalten` vorsieht. Der
  Übersetzer hat daraufhin jede Probe genannt, die den Wert fallen ließ.
- Neue private `sichtbare()`: die eine Stelle, die die Liste der sichtbaren Bereiche bildet. Drei
  Rechnungen meinen dieselbe Menge — `bereichsbreiten`, `breiten_uebernehmen` und der neue
  `massstab` —, und sie stand vorher zweimal ausgeschrieben da.

**Die Behebung des Defekts, in derselben Datei**

- `breite_aendern(&mut self, bereich, betrag, mass: Zeilenmass)` rechnet den Schritt über die neue
  `massstab` in gespeicherte Punkte um. **Dieselbe Umrechnung trifft die beiden Mindestbreiten**,
  gegen die der Schritt deckelt; auch sie standen im falschen Maßstab, und der Datensatz sagt es.
- `massstab(mass) -> f64` ist `gespeicherte Summe der sichtbaren / verfügbare Breite`, mit 1 als
  Antwort, wenn eine der beiden Zahlen nicht positiv ist. Sein Kommentar benennt die Grenze: die
  Abbildung ist nicht mehr linear, sobald ein sichtbarer Bereich an seinem Mindestmaß hängt. Das ist
  benannt und nicht behandelt — eine Sonderregel dafür wäre ein zweiter Rechenweg neben
  `bereichsbreiten`.

**`crates/krk-ui/src/appkit/aufteilung.rs`**

- Neue freie Funktion `zeilenmass(teiler)` — die eine Stelle, an der die beiden Zahlen aus AppKit
  gelesen werden. `auslegen` nimmt sie jetzt von dort statt sie selbst zusammenzusetzen, und
  `Aufteilung::zeilenmass` gibt sie nach außen. Zwei Aufrufer, eine Quelle; zweimal ausgeschrieben
  könnten die beiden Wege verschiedene Zeilen meinen.

**`crates/krk-ui/src/appkit/anwendung.rs`**

- `Anwendungsdelegierter::zeilenmass() -> Option<Zeilenmass>` holt das Maß aus der Aufteilung.
  `None` heißt: die Aufteilung steht noch nicht, also läuft der Aufbau, und dann geschieht nichts —
  dieselbe Antwort, die `aufteilung_nachziehen` und `bildschirmbreiten_uebernehmen` in dieser Lage
  geben.
- `bereich_umschalten`, `bereich_einblenden` und `breite_aendern` holen es dort und reichen es
  weiter. Die vier Aufrufer der beiden ersten (`zwischenablage_ansehen`, `fokus_holen`,
  `kommando_ausfuehren` dreifach, `editor_ausblenden`) bleiben unverändert: das Maß endet an der
  Stelle, an der es entsteht.

**`crates/krk-ui/src/kommandos/fokus.rs`** — eine Datei mehr als der Plan unter *Files* nennt, und
er nennt sie unter *Aufzählungen* selbst: eine Probe dort ruft `umschalten` und `einblenden`. Nur
diese Probe ist angefasst.

## Kommentare, die die Änderung falsch gemacht hätte

Vier Stellen sagten nach der Änderung das Gegenteil des Codes und stehen jetzt richtig da: der
Dokumentationskommentar an `umschalten` (er kannte eine Abweisung), der an `breite_aendern` (er
sagte ausdrücklich zu, der Schritt gelte in gespeicherten Punkten), der an
`Anwendungsdelegierter::breite_aendern` und der Kommentar an der Probe zum Tastenbefehl, der die
1280 als die eine messbare Fensterbreite begründete.

## Prüfungen

`fenstermodell.rs` trägt jetzt 34 Proben statt 32. Zwei sind neu:

- `am_engen_fenster_wird_das_einschalten_abgewiesen` — bei 780 Punkten wird das Einschalten des
  Editors neben Lesezeichenleiste und beiden Dateifenstern abgewiesen (920 Punkte Mindestbreite),
  bei 1280 nicht. Geprüft wird daneben, dass der abgewiesene Befehl **nichts** geändert hat: die
  Vorschau steht danach noch.
- `ein_ausschaltbefehl_scheitert_nie_an_den_mindestbreiten` — bei 200 Punkten, also unter jeder
  einzelnen Mindestbreite, geht jeder der drei ausblendbaren Bereiche aus und kommt nicht zurück.

`der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt` misst wieder über mehrere
Fensterbreiten: 1280, 1400 und 1920, also die drei aus der Messtabelle des Defekts, und an jeder
genau 40 Punkte hin und zurück. **Gegengeprobt:** mit einem fest auf 1 gesetzten Maßstab fällt sie
bei 1400 mit 43,75 Punkten — genau die Zahl, die der Datensatz gemessen hatte. Die Wegwerfänderung
ist zurückgenommen.

Drei Probenhelfer sind dazugekommen, weil `umschalten` jetzt zwei Größen mehr braucht: `weit()`
(eine Zeile, in die jede Menge passt — für Proben, die eine Sichtbarkeit nur **herstellen**),
`passend(&modell)` (eine Zeile, in der ein gespeicherter Punkt ein Punkt auf dem Schirm ist) und
`schalten(&mut modell, bereich)`, das auf dem Erfolg besteht. Der letzte ist die Antwort auf das
neue `#[must_use]`: eine stumme Abweisung im Probenaufbau ließe die Probe auf einer anderen Lage
messen als der, die ihr Name nennt.

## Ein Defekt abgelegt, einer geschlossen

**Geschlossen:** `issues/260812-0439_c_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`,
über Weg 1 der beiden benannten, wie es der Datensatz empfohlen hat.

**Neu:** `issues/260812-0512_o_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Fall entsteht mit diesem Schritt: unter rund 920 Punkten Fensterbreite lädt F4 die Datei weiter
in den Editor, aber der Bereich kommt nicht auf den Schirm und der Fokus bleibt stehen. Die Stille
ist nach C2.5 gewollt, der halbe Weg nicht. Der Datensatz führt drei Wege und ordnet sie der schon
offenen Nutzerfrage nach der Mindestbreite des Fensters unter
(`decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`);
er ist nicht im Vorbeigehen behoben worden.

## Am Plan nachgezogen

Schritt 2 steht auf `[DONE]`. Schritt 1 stand noch ohne Marke, obwohl er gebaut und eingetragen ist
(`5e17c9e`); er steht jetzt ebenfalls auf `[DONE]`, gegen den Baum gelesen und nicht gegen sein
Protokoll allein — `Zeilenmass`, die Anteilsregel in `bereichsbreiten` und die Rückrechnung in
`breiten_uebernehmen` liegen vor.

## Abnahme

`make check` (`cargo build`, `cargo test`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, alle über den Workspace) — **Exit 0**,
„alle vier grün". Kein Vordergrund nötig, wie der Plan für diesen Schritt zusagt.

Nicht committet: der Orchestrator trägt ein.

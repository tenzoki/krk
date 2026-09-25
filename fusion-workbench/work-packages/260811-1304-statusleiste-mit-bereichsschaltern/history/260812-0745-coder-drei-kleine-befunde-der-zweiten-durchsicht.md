# Coder: die drei kleinen Befunde der zweiten Durchsicht

**Datum:** 260812-0745
**Agent:** coder
**Status:** Complete
**Maßstab:** `reviews/260812-0727-coderev-bereichsleiste-spaltensichtbarkeit-und-die-wiederhergestellte-naht.md`,
Befunde 2, 3 und 4
**Abnahme:** `make check` — **Exit 0**

## Auftrag

Die drei niedrigen Befunde der zweiten Durchsicht beheben, alle drei an Code dieser Runde.
Befund 1 (die drei Spaltenbefehle in der Markdown-Ausgabe) ausdrücklich **nicht**: er ist als
Textkorrektur an drei Stellen unterwegs, der Nutzerentscheid vom 260811-0110 gilt,
`belegungsausgabe::markdown` bleibt unverändert. `resources/default-keymap.toml` nicht
anfassen, weil ein ontocoder parallel daran arbeitet. Nicht committen.

## Befund 4: der doppelte Nachzug der Bereichsleiste

**Gebaut ist die Fassung, in der die Frage entfällt, statt beantwortet zu werden.** Der
Datensatz nannte zwei Wege, den Kommentar zu berichtigen oder den Nachzug an
`!kommando_ausfuehren(kommando)` zu binden. Der zweite wäre die Fallunterscheidung „war der
Klick angenommen?", die `critical-stance.md` §2 als Merkmal einer falsch geschnittenen Stelle
nennt; der erste ließe die doppelte Zeichenarbeit auf dem Weg stehen, den L1 misst.

Der andere Schnitt liegt an der Frage **wer schreibt einen Schalterzustand**. Die Antwort soll
„allein das Modell" sein, und der Modulkopf sagt das seit Schritt 8. Genau eine fremde
Schreibung gibt es: das Ankreuzfeld kippt sich beim Klick selbst. Sie wird jetzt dort
zurückgenommen, wo sie entsteht.

```
vorher:  Klick ─> Melder ─> kommando_ausfuehren ─> aufteilung_nachziehen ─> 8x setState
                         └────────────────────────────────────────────────> 8x setState

nachher: Klick ─> geklickt ─> 1x setState (Kippung zurück)
                           └> kommando_ausfuehren ─> aufteilung_nachziehen ─> 8x setState
```

`Leistenquelle::geklickt` (`crates/krk-ui/src/appkit/bereichsleiste.rs:282`) ist der Trichter
beider Aktionsmethoden: erst `selbstkippung_zuruecknehmen(absender)`, dann melden, falls aus
der `tag` ein Kommando wird. Der Melder im Anwendungsdelegierten
(`crates/krk-ui/src/appkit/anwendung.rs:777`) ruft nur noch `kommando_ausfuehren`.

Die Bilanz je Weg: angenommener Klick 1 + 8 statt 16 Schreibvorgänge, abgewiesener Klick 1
statt 8, Tastendruck unverändert 8. `bereichsleiste_nachziehen` hat wieder **einen** Anlass.

**Der Preis, benannt und nicht wegverhandelt.** Die Rücknahme setzt voraus, dass das
Ankreuzfeld seinen Zustand vor der Aktion wirklich selbst kippt. Diese Annahme trug schon den
unbedingten Nachzug — sie ist der einzige Grund, aus dem es ihn gab —, aber die Folge eines
Irrtums ist eine andere: bisher wäre der zweite Ruf überflüssig gewesen, jetzt stünde ein
abgewiesener Klick falsch. `inference:` Die Annahme steht in der Dokumentation von AppKit und
ist der übliche Umgang mit einem Ankreuzfeld, gemessen ist sie in diesem Baum nicht. C2.4 ist
ohnehin ein Bündel-Kriterium und steht auf der Liste unter `## Abnahme am laufenden Bündel`;
dort ist sie zu sehen.

## Befund 3: die abgeschriebene Erreichbarkeitsprüfung

`Anwendungsdelegierter::editor_ist_ansprechbar` (`crates/krk-ui/src/appkit/anwendung.rs:1488`)
ist die eine Fassung, gefragt von `fokus_editor_holen` und `editor_umschalten`. In ihrer
bejahenden Form: **der Editor ist ansprechbar, wenn er steht oder wenn er eine Datei hält.**
Beide Aufrufer schrumpfen auf vier Zeilen, und der Satz „dieselbe Bedingung" im
Doc-Kommentar ist von einer Behauptung zu einer Tatsache geworden.

## Befund 2: `beschreibbar` gegen den Modulkopf von `spalten.rs`

**Der Code hält die Zusage; der Modulkopf bleibt.** `Spalte::beschreibbar`
(`crates/krk-ui/src/spalten.rs:106`) ist ein ausgeschriebenes `match` über alle vier Werte,
wie `appkit::tabelle::ausrichtung` es seit Schritt 6 hält. Die Kürzung des Modulkopfs wäre
billiger zu schreiben und teurer zu tragen: er schreibt die Regel dieses Projekts auf, und
eine Datei, die davon eine Stelle ausnimmt, verlangt bei jeder Erweiterung ein Nachzählen,
welche der sieben genannten Stellen gemeint sind. Der Preis ist eine Zeile.

## Proben

**Eine, und die gab es schon.** `genau_die_namensspalte_ist_beschreibbar` hält die
Wahrheitswerte der vier Spalten; die Vollständigkeit hält der Übersetzer, und ihr
Doc-Kommentar sagt das jetzt, damit die nächste Lesung sie nicht der Probe zutraut.

Für die beiden anderen Befunde lässt sich ohne Fenster nichts festhalten: die Rücknahme der
Selbstkippung lebt an einem `NSButton`, `editor_ist_ansprechbar` liest zwei Ivars des
Delegierten, und `anwendung.rs` trägt kein Prüfmodul. Eine Probe für beides bräuchte eine
Instanz und damit `MainThreadMarker::new_unchecked` — eine fünfte und sechste Stelle der
Bauart, die Schritt 8 aus demselben Grund vermieden hat (`issues/260810-1001_*`, als Lage
angenommen; `decisions/260810-1044_*`, zurückgestellt). Die Doppelung aus Befund 3 ist
stattdessen strukturell weg: es gibt keine zweite Stelle mehr, die auseinanderlaufen könnte.

## Nachgezogene Prosa

Jeder Kommentar, den die Änderung falsch gemacht hätte, ist mit ihr richtig geworden:

- Modulkopf von `bereichsleiste.rs` samt seiner Skizze, Doc-Kommentar an
  `zustaende_setzen`, neuer Doc-Kommentar an `geklickt` und
  `selbstkippung_zuruecknehmen`.
- Kommentar am Melder und Doc-Kommentar an `bereichsleiste_nachziehen` in `anwendung.rs`;
  der Doc-Kommentar an `editor_umschalten` verweist auf die gemeinsame Funktion statt auf
  eine gleichlautende Bedingung.
- Doc-Kommentare an `Spalte::beschreibbar` und an ihrer Probe.
- Im Plan `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`: der
  Fluss unter `### Der eine Weg vom Eingang bis zur Anzeige` führt die Rücknahme als eigenen
  Schritt auf dem Klickweg statt als gestrichelte Kante „danach, in jedem Fall", mit dem
  dazugehörigen Absatz und einer datierten Korrekturnotiz nach dem Vorbild von C3.5.
  Schritt 8 der Änderungsliste nennt den Melder nicht mehr als zweiten Rufer.

## Nicht angefasst

- `crates/krk-ui/src/belegungsausgabe.rs` und die beiden Proben, die den heutigen Zustand
  festhalten. Befund 1 ist eine Textkorrektur an drei Stellen und keine Codeänderung.
- `resources/default-keymap.toml` — der ontocoder arbeitet parallel daran.
- Die Risikotafel des Plans: die Zeile über die acht Schalterzustände je ausgeführtem Befehl
  gilt unverändert, denn genau so viele sind es jetzt auf jedem Weg.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/bereichsleiste.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/spalten.rs`
- `fusion-workbench/circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`
- drei Datensätze unter `issues/`, je mit `Resolved:` und auf `_c_` umbenannt

## Abnahme

`make check` — Exit 0 (`cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`).
Nicht committet, wie beauftragt.

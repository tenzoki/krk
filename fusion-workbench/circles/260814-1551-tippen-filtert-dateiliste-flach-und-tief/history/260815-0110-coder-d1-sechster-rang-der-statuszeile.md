# D1 — Der sechste Rang der Statuszeile

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang D, Schritt D1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C4.1 bis C4.10
**Verification:** `make check` — exit 0

## Wo der Rang steht

`Rang::ALLE` trägt sechs Werte, und der neue steht zwischen Tabmeldung und
Markierungsstand:

```text
1  Befehlsantwort    was KRK auf einen Tastenbefehl zu sagen hat
2  Vorgangsanzeige   der Stand einer laufenden Operation
3  Fenstermeldung    ein Ereignis am Fenster, das niemand angefordert hat
4  Tabmeldung        der Zustand des sichtbaren Ordners
5  Filterstand       der stehende Filtertext und was er uebrig laesst   <-- neu
6  Markierungsstand  was im sichtbaren Tab markiert ist
```

`Rang::art` gibt für ihn `Art::Vorgang`; eine Filterzahl ist kein Fehler und wird nicht
rot. Beide Fallunterscheidungen, `Rang::art` und `Quellen::text`, bleiben vollständig und
ohne Auffangzweig: ein siebter Rang hält den Bau an.

**Die Rangstelle ist die eine Zeile, die die offene Nutzerfrage bewegt.** Fällt
`decisions/260814-1552_o_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`
anders aus als ihre Empfehlung, wandert `Rang::Filterstand` in `Rang::ALLE` um eine Stelle
und sonst ändert sich nichts. Weder `zeile` noch `Quellen::text` noch `filterstand_text`
kennen die Nummer; sie steht allein in dem Feld.

## Was die Zeile sagt

`statuszeile::filterstand_text(filtertext, Filterstand)` ist eine reine Funktion ohne
AppKit. Sie liefert `None` auf zwei Wegen, und beide stehen in ihr und nicht beim
Aufrufer, damit sie ohne Fenster prüfbar sind: kein Filtertext (C4.8), und ein begonnener
Lesevorgang, der seinen Bestand noch ablösen muss (C4.7, gefragt über die vorhandene
`Ordnermodell::ersetzt_beim_naechsten_stapel`).

```text
Filter „rs“: 38 von 4.812 angezeigt
Filter „rs“: 38 von 412 angezeigt, eine Markierung ausgeblendet
Filter „rs“: 38 von 412 angezeigt, 2.500 Markierungen ausgeblendet
```

Der vierte Satzteil steht nur da, wenn der Filter Markierungen ausblendet (C4.4). Er ist
die Gegenleistung dafür, dass die Markierungsregel unter dem Filter unverändert bleibt
(`decisions/260814-1552_a_was-geschieht-mit-einer-markierung-die-der-filter-ausblendet.md`):
ohne ihn müsste der Nutzer erraten, dass es die ausgeblendete Markierung gibt.

Beide Zahlen gehen durch `kommandos::operationen::zahl` und tragen damit dieselben
Tausenderpunkte wie ein laufender Vorgang und der Markierungsstand daneben. Ein zweites
Zahlenformat entsteht nicht.

## Warum die Funktion in `statuszeile.rs` steht und nicht in `kommandos/`

Der Rang darunter hat seine Wendung in `kommandos/auswahl.rs`, weil er zu C2 der Runde 1
gehört und sich zwei Bausteine aus `operationen` leiht. Der Filterstand gehört zu keiner
Fähigkeit außer der Zeile selbst. Er steht deshalb bei dem Rang, den er füllt; das hält
zugleich die Dateiliste von D1 ein. AppKit ruft er so wenig wie jener, und beide sind ohne
Fenster prüfbar.

## Ein Durchlauf über die Markierung für beide gerechneten Ränge

Die zwei untersten Ränge haben kein eigenes Feld am Dateifenster, sondern werden bei jedem
Schreiben der Zeile aus dem Modell des sichtbaren Tabs gerechnet. Beide brauchen dieselbe
Erhebung: der Markierungsstand die Zahl, der Filterstand die Differenz zu den markierten
Einträgen der Sichtreihenfolge. `DateifensterQuelle::gerechnete_raenge` erhebt sie in
**einer** Ausleihe und gibt beides heraus; `markierungsstand_text` bekommt den Stand jetzt
herein, statt ihn selbst zu holen. Zwei getrennte Erhebungen wären auf einem Ordner mit
100.000 Einträgen zweimal derselbe Durchlauf, je Schreiben der Zeile.

Die ausgeblendeten Markierungen werden aus vorhandenen Fragen gerechnet und nicht am
`Ordnermodell` neu gestellt: `markierungsstand().zahl` minus den markierten Einträgen in
`sichtreihenfolge()`. Der Kern bekommt dafür keine Zeile.

## Drei Anlässe ziehen den Rang nach

Ein gerechneter Rang hat kein Feld, das jemand setzt, aber gezeichnet werden muss trotzdem
— dieselbe Lage wie beim Markierungsstand seit S16c. Drei Stellen rufen dafür jetzt
`meldung_gewechselt`:

- `nach_filteraenderung` — der eine Weg der Anzeige nach jeder Änderung des Filtertexts,
  also Tippen, Rücknahme eines Zeichens und `Esc`.
- `tiefe_suche_umschalten` — der Schalter ändert, wie viele Zeilen stehen.
- `einziehen` — während eines Lesevorgangs wachsen beide Zahlen. Die Bedingung lautet
  `einzug.meldung_neu || self.filter_steht()`: ohne Filtertext meldet der Rang nichts, und
  dann hat kein Stapel etwas an der Zeile zu ändern. Das hält den Neubau beider
  Quellensätze aus dem Takt heraus, solange kein Filter steht.

## Was an einer Probe abgenommen ist

Acht neue Proben in `crates/krk-ui/src/appkit/statuszeile.rs`, alle ohne AppKit:

| Probe | Kriterium |
|---|---|
| `der_filterstand_steht_zwischen_tabmeldung_und_markierungsstand` | C4.1 |
| `der_filterstand_gilt_nicht_als_fehler` | C4.2 |
| `der_satz_nennt_filtertext_gezeigte_und_vorhandene` | C4.3 |
| `ausgeblendete_markierungen_stehen_daneben_und_sonst_nicht` | C4.4 |
| `die_linke_zahl_waechst_und_zaehlt_zeilen_und_keine_treffer` | C4.5 (Rechnung), C4.6 |
| `waehrend_der_ersatz_aussteht_nennt_der_rang_nichts` | C4.7 |
| `ohne_filtertext_meldet_der_rang_nichts` | C4.8 |
| `jeder_der_sechs_raenge_hat_genau_ein_feld` | C4.10 |

Die Stelle in der Rangfolge wird dabei aus `Rang::ALLE` gelesen und nicht danebengeschrieben,
damit die Probe nicht gegen ihre eigene Kopie prüft. `ueber_alle_zehn_bewerber_…` heißt
jetzt `ueber_alle_zwoelf_bewerber_…` und deckt zwölf Bewerber ab.

## Was nicht an einer Probe hängt

**C4.9 („es bleibt bei einer Statuszeile") ist durch Ansehen belegt und nicht durch eine
Probe.** Der Spec nennt „Probe über die Zahl der Anzeigen"; eine solche Probe gibt es in
diesem Baum nicht und wäre auch keine, denn sie müsste die Abwesenheit einer Ansicht
prüfen. Belegt ist stattdessen zweierlei, beides am Baum: der Diff legt keine Ansicht an
(`git diff -- crates/ | grep '^+' | grep -E 'NSTextField|NSView|addSubview|labelWithString'`
findet nichts), und der Filtertext erreicht genau einen Empfänger, nämlich das Feld
`Quellen::filterstand`, von dem allein `statuszeile::zeile` liest. Weder `fenstertitel.rs`
noch `tableiste.rs` sind angefasst.

**Die Bündelhälfte von C4.5** — die linke Zahl beim Zählen zuzusehen, während die rechte
steht — bleibt Nutzerarbeit und gehört G2. Die Rechnung dahinter ist geprüft, das Mitzählen
im laufenden Bündel nicht: es hängt am Durchlauf aus Strang F, der noch nicht gefahren ist.

## Fünf Dateien statt drei, und warum

D1 nennt drei Dateien; geändert sind fünf. Die zwei weiteren tragen je eine Zeile Prosa:
`kommandos/auswahl.rs` nannte `markierungsstand_text` „den fünften Rang", `appkit/editor.rs`
zählte „fünf Ränge". Beide werden falsch, sobald ein Rang in die Mitte der Rangfolge tritt.
Der Befund ist an
`issues/260814-2357_o_c2-nennt-zwei-dateien-der-weg-an-den-filtertext-des-tabs-fuehrt-durch-eine-dritte.md`
angehängt, mit dem Unterschied zu den vier vorigen Fällen: hier ist die Dateiliste für die
Änderung vollständig und nur für ihre Folge nicht.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/statuszeile.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/kommandos/auswahl.rs`
- `crates/krk-ui/src/appkit/editor.rs`

## Verification

```
make check — exit 0
```

Alle vier Abnahmekommandos grün, `clippy` unter `-D warnings`. Die acht neuen Proben
laufen mit; `cargo test -p krk-ui` meldet 595 bestandene Proben.

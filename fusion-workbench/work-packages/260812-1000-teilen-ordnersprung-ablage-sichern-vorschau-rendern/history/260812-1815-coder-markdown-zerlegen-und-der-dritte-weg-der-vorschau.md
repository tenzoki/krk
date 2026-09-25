# Schritt 8: Markdown wird zerlegt, und das Vorschaumodell kennt den dritten Weg

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 8
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 410, nachher 432

---

## Was gebaut wurde

Die Vorschau kennt seit diesem Schritt ihren dritten Weg. Eine Markdown-Datei
wird auf dem Arbeitsfaden `krk-vorschau` zerlegt, und heraus kommt ein
`Gerendert` aus dem Text ohne Auszeichnungszeichen und derselben
`Formatierung`, die `hervorhebung.rs` liefert. Der Text steht damit in der
Fläche; die Merkmale trägt Schritt 9 nach.

Neu ist `crates/krk-ui/src/markdown.rs`, reines Rust ohne AppKit wie
`hervorhebung.rs` daneben. Neu im Baum ist `pulldown-cmark 0.13.4` ohne
Vorgabemerkmale.

## Die vier Zahlen der fremden Kiste, selbst erhoben

Der Plan nennt vier; alle vier sind an der eingebundenen Fassung nachgemessen
und nicht übernommen.

| Zahl | Befund |
|---|---|
| Drei Abhängigkeiten, davon zwei im Baum | `bitflags`, `memchr`, `unicase`. `bitflags 2.13.1` und `memchr 2.8.3` stehen bereits in genau diesen Fassungen in `Cargo.lock`. `cargo build` meldete „Locking 2 packages": `pulldown-cmark` und `unicase`. |
| Kein C-Code | Weder in `pulldown-cmark-0.13.4` noch in `unicase-2.9.0` liegt eine `.c`- oder `.h`-Datei. `unicase` trägt `build = false`; das `build.rs` von `pulldown-cmark` übersetzt ohne das Merkmal `gen-tests` zu nichts, und `gen-tests` ist kein Vorgabemerkmal. |
| Mindestfassung 1.71.1 | `rust-version = "1.71.1"` in der `Cargo.toml` der Kiste; das Projekt fährt 1.97.1. |
| Geschwindigkeit | An einer aus einem realistischen Muster auf 1 050 075 Bytes vervielfachten Markdown-Datei, `--release`, drei Läufe von `markdown::rendern`: **29,5 ms, 20,5 ms, 19,2 ms**, also 36 bis 55 MB/s. Gemessen mit einer Probe, die danach wieder entfernt wurde; der Wert steht in der Wurzel-`Cargo.toml`. |

**Die Zusicherung gegen C-Code hält, und sie ist belegt und nicht behauptet.**
`grep -nE '^name = "(cc|.*-sys)"$' Cargo.lock` findet allein `windows-sys`, das
schon vorher dastand; `cargo tree --workspace -e normal,build` findet im ganzen
Baum weder `cc` noch `onig` noch einen Namen auf `-sys`.

**Das Merkmal für Tabellen bleibt aus**, und damit erscheint eine Tabelle als
Quelltextraster: die Kiste liefert ihre drei Zeilen als drei `Text`-Ereignisse
mit ihren Zwischenräumen, getrennt durch weiche Umbrüche. Die Probe
`eine_tabelle_bleibt_ein_quelltextraster` hält es fest.

## Die Regel der Zerlegung

Ein Durchgang über `Parser::new_ext(quelle, Options::empty()).into_offset_iter()`.
Die Fallunterscheidung über `Tag` steht in `behandlung` als eine Tabelle:

| Element | Wird zu |
|---|---|
| `Heading` | `Auszeichnung::Ueberschrift { stufe }`, Stufe aus `HeadingLevel` |
| `Item` | `Auszeichnung::Listenzeile` |
| `BlockQuote` | `Auszeichnung::Listenzeile` |
| `CodeBlock`, `Code` | `Auszeichnung::FesteSchrift` |
| `Emphasis` | `Auszeichnung::Betonung` |
| `Strong` | `Auszeichnung::StarkeBetonung` |
| `Link` | `Einfaerbung` mit Farbe und Unterstreichung |
| `Paragraph`, `List` | nur Abstand, keine Auszeichnung |
| alles Übrige | der Quellbereich, wörtlich, und bis zu seinem Ende übersprungen |

`SoftBreak` und `HardBreak` werden zu `\n`. Die Ereignisse ohne Ende — `Rule`,
`Html`, `InlineHtml`, `FootnoteReference`, `InlineMath`, `DisplayMath`,
`TaskListMarker` — gehen ebenfalls wörtlich durch; das ist dieselbe
Auffangregel, nur für die Ereignisse, bei denen es nichts zu überspringen gibt.

**Der Einzug hängt am `Item` und nicht zusätzlich an der `List`.** Beide
gemeinsam einzutragen ergäbe zwei Stellen, die dasselbe sagen: der Bereich der
Liste deckt den jedes ihrer Punkte. Am Punkt ist er zeilengenau, und das ist die
Einheit, die AppKit als Absatz einrückt.

## Zwei Stellen, an denen die Umsetzung dem Plan nicht wörtlich folgt

Beide sind Abweichungen im Detail, nicht in der Sache, und beide sind gemessen
begründet.

### 1. `linkfarbe` fragt den vollen Wortartenstapel und nicht `markup.underline.link`

Der Plan schreibt „ein Nachschlag auf die Wortart `markup.underline.link` in
derselben Tafel". Genau so gebaut liefert der Nachschlag **die Grundfarbe**, und
ein Verweis sähe aus wie Fließtext. Am 260812 an den eingebundenen Tafeln
gemessen:

```
base16-ocean.light  Grundfarbe (79, 91, 102)   markup.underline.link (79, 91, 102)
base16-ocean.dark   Grundfarbe (192,197,206)   markup.underline.link (192,197,206)
```

Keine der beiden Tafeln des Vorgabesatzes führt einen eigenen Eintrag für diese
Wortart. Die Farbe, die ein Verweis im **Editor** heute trägt, kommt von
`meta.link`, und zwar weil `style_for_stack` den ganzen Stapel bewertet und
nicht nur die letzte Wortart. Am Text von `[die Seite](https://example.com)`
abgelesen, mit den eingebundenen Sprachdefinitionen:

```
"die Seite" -> (208, 135, 112)
   ["text.html.markdown", "meta.paragraph.markdown",
    "meta.link.inline.description.markdown"]
```

`linkfarbe` fragt deshalb genau diesen Stapel, festgehalten als
`VERWEISSTAPEL`. Damit trägt ein Verweis in der Vorschau die Farbe, die er im
Editor hat — was das Ziel des Plansatzes war —, statt gar keine. Die Probe
`hervorhebung::tests::die_tafel_faerbt_einen_verweis` misst die Aussage, an der
es hängt: die gelieferte Farbe ist nicht die Grundfarbe der Tafel. Sie hätte den
Fehler gefangen, und sie fängt ihn wieder, wenn eine Sprachdefinition die
Wortart umbenennt.

**Der Rückgabetyp ist `Option<Farbe>` und nicht `Farbe`.** Für eine Tafel, die
aus dem Vorgabesatz verschwindet, gäbe es sonst nur eine erfundene Farbe, und
der Modulkopf von `hervorhebung.rs` sagt zu, dass diese Datei keine Farbe von
KRK kennt. `None` ist dieselbe Antwort, die `rechnen` auf eine fehlende Tafel
gibt: kein Grund, dem Nutzer seinen Text vorzuenthalten.

### 2. `appkit/textmerkmale.rs` steht nicht in der Dateiliste des Schrittes und musste angefasst werden

Die Dateiliste von Schritt 8 nennt es nicht; der Rumpf des Schrittes verlangt es
ausdrücklich: „der Übersetzer hält an `textmerkmale::anwenden` an und erzwingt
die Umsetzung (kursiv beziehungsweise fett in der Grundgröße)". Genau das ist
eingetreten. Die Datei hat zwei Zweige und eine Hilfsfunktion bekommen, mehr
nicht:

- `Auszeichnung::StarkeBetonung` → `NSFont::boldSystemFontOfSize(grundgroesse)`.
- `Auszeichnung::Betonung` → `kursive_schrift(grundgroesse)`, über
  `NSFontDescriptor` mit dem Merkmal `TraitItalic` und nicht über
  `NSFontManager`: der Verwalter ist die Maschinerie hinter dem Schriftfenster
  und baut einen gemeinsamen Zustand auf, den KRK nirgends sonst braucht. Ohne
  kursive Fassung fällt es auf die aufrechte Schrift zurück, wie
  `feste_schrift` daneben.

Der Modulkopf ist um die Untergrenzen ergänzt, alle am SDK gelesen:
`NSFontDescriptor` (`NSFontDescriptor.h:61`), `fontDescriptor` an `NSFont`
(`NSFont.h:87`), `fontDescriptorWithSymbolicTraits:` (`NSFontDescriptor.h:92`),
`fontWithDescriptor:size:` (`NSFont.h:31`) und `NSFontDescriptorTraitItalic`
(`NSFontDescriptor.h:22`) tragen keine Verfügbarkeitsangabe und stehen damit
seit macOS 10.0; das Bündel zielt auf 15.0.

## Die Tafel geht durch das Modell, und in Schritt 8 ist sie die Vorgabe

`markdown::rendern` braucht eine `Tafel`, also brauchen `laden`,
`Ladevorgang::starten` und `Vorschaumodell::datei_anzeigen` sie auch. Der Weg
ist gebaut; `appkit/vorschau.rs` übergibt in diesem Schritt `Tafel::default()`,
mit einem Kommentar an der Stelle.

**Das ist bewusst und nicht vergessen.** Solange die Vorschau ihre
Auszeichnungen nicht in die Fläche trägt — und das tut sie erst mit Schritt 9 —,
ist keine dieser Farben zu sehen. Eine zweite Abfrage des Erscheinungsbildes
neben `tafel_der_erscheinung` in `appkit/editor.rs` wäre die zweite Wahrheit
darüber, was „dunkel" heißt; `editor.rs` gehört nicht zu diesem Schritt, und
Schritt 7 stand ausdrücklich deshalb vorn, damit die Datei nicht zweimal
aufgemacht wird.

**Für Schritt 9 ist damit eine Zeile zu ändern und keine Schnittstelle.** Zu
beachten ist dort: die Dateiliste von Schritt 9 nennt allein
`appkit/vorschau.rs`, und die Zuordnung Erscheinungsbild → `Tafel` liegt heute
privat in `editor.rs`. Wer sie in der Vorschau braucht, öffnet entweder
`editor.rs` mit auf oder zieht sie zu `textmerkmale.rs`; sie ein zweites Mal zu
schreiben wäre der Fehler, den Schritt 7 gerade vermieden hat.

## Die Abstände zwischen den Blöcken

Ein Block **verlangt** eine Zahl von Umbrüchen vor und nach sich — Absätze,
Überschriften, Zitate und Quelltextblöcke zwei, Listen und ihre Punkte einen.
Geschrieben wird nichts davon sofort: der Wunsch wird aufgehoben und erst vor
dem nächsten Zeichen eingelöst, und dabei **aufgefüllt statt angehängt**. Zwei
Wirkungen hängen daran, und beide sind der Grund für diese Form:

- Ein Element, dessen Quelltext schon mit einem Umbruch endet — ein
  Quelltextblock, eine Trennlinie, ein HTML-Block — trägt hinterher keine
  Leerzeile zuviel.
- Ein Block, der noch kein Zeichen bekommen hat, **rückt mit der Trennung
  nach**. Ohne das trüge sein Bereich die Umbrüche des vorigen Absatzes, und ein
  Absatzmerkmal — der Einzug einer Listenzeile — schlüge auf jenen durch. Die
  Probe `ein_block_beginnt_hinter_seiner_trennung` hält es fest.

## Die Reihenfolge der Auszeichnungen

Sortiert wird nach Anfang, bei gleichem Anfang das **längere zuerst**. Damit
steht das äußere Element vor dem inneren. Das ist keine Kosmetik: `anwenden`
setzt sie in dieser Reihenfolge über `addAttributes:range:`, und Überschrift wie
feste Schrift setzen beide die Schrift. Stünde das innere Stück zuerst, trüge
der Quelltext in einer Überschrift die Schrift der Überschrift.
`die_auszeichnungen_stehen_von_aussen_nach_innen` misst es.

## Was die Proben abdecken

Sechzehn in `markdown.rs`, dazu drei Ergänzungen. Alle ohne Fenster.

- Überschrift verliert ihre Doppelkreuze, alle sechs Stufen kommen mit ihrer
  Zahl an
- Betonung und starke Betonung verlieren ihre Sternchen
- Quelltextblock verliert seine Zäune, Quelltext in der Zeile seine Haken
- Verweis behält seinen Text und verliert seine Adresse; seine Farbe ist die der
  Tafel
- Listenpunkt und Zitatblock tragen den Einzug
- Tabelle bleibt Quelltextraster mit ihren Zwischenräumen
- Bild, eingebettetes HTML (als Block und in der Zeile) und Trennlinie
  erscheinen als ihr Quelltext
- jede Stelle liegt innerhalb der Länge, und die Länge ist die UTF-16-Länge des
  Ausgabetextes
- **die Stellen sind UTF-16-Einheiten**, an `"Grüße 😀 an *dich*."` gemessen:
  der Anfang liegt bei 12, in Bytes wären es 16 und in Zeichen 11
- ein Block beginnt hinter seiner Trennung; eine verschachtelte Liste hängt
  nicht aneinander
- eine leere Quelle ergibt nichts; die Formatierung nennt ihre Darstellungsart

In `vorschaumodell.rs`: `die_drei_wege_der_anzeige_haengen_an_der_endung` misst,
dass eine `.md`-Datei als `Inhalt::Markdown` und eine `.rs`- wie eine
`.html`-Datei als `Inhalt::Text` ankommt — das dreizehnte Abnahmekriterium von
C4 daneben. `allein_der_text_einer_datei_traegt_zeilennummern` deckt jetzt alle
sechs Werte von `Inhalt` ab, `Markdown` eingeschlossen.

In `hervorhebung.rs`: `die_tafel_faerbt_einen_verweis`, siehe oben.

## Die beiden Aufzählungen, die gewachsen sind

- **`Inhalt` von fünf auf sechs Werte.** Der Übersetzer hielt an
  `zeigt_dateitext` und an `vorschau::anzeigen` an, wie der Plan es vorhersagt.
  `zeigt_dateitext` liefert für `Markdown` `false`, und der Doc-Kommentar sagt,
  warum: die Zahlen zählten die Zeilen des gerenderten Textes, und das sind
  andere als die der Datei, die danebensteht.
- **`Auszeichnung` von drei auf fünf Werte.** Der Übersetzer hielt an
  `textmerkmale::anwenden` an, siehe oben.

Keine der vier vom Circle benannten Aufzählungen wächst.

## Was nicht getan wurde

Kein Vordergrundlauf, kein Bündelbau, keine Messung der zehn Zusagen. Die
Vorschau trägt in diesem Schritt keine Merkmale in ihre Fläche; der Text steht
gerendert da, ohne Farbe, Schriftschnitt und Einzug. Das ist der vom Plan
benannte, vollständig übersetzbare Zwischenstand.

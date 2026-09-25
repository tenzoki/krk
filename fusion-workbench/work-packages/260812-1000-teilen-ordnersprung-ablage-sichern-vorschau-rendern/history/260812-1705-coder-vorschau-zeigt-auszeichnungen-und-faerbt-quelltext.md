# Schritt 9: Die Vorschau zeigt die Auszeichnungen und färbt Quelltext nach

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 9
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 432, nachher 434

---

## Was gebaut wurde

Die Vorschau trägt ihre Auszeichnungen jetzt in die Fläche. Eine
Markdown-Datei erscheint gerendert samt Überschriften, Einzügen, fester
Schrift, Betonung und eingefärbtem Verweis; eine Quelltextdatei bekommt ihre
Farben von `syntect`, nachgezogen aus einem Arbeitsfaden, der in der **Ansicht**
wohnt und nicht im Modell.

Geändert sind drei Dateien und keine vierte:

- `crates/krk-ui/src/appkit/vorschau.rs` — der Schritt selbst.
- `crates/krk-ui/src/appkit/textmerkmale.rs` — Zielort der Tafel-Zuordnung.
- `crates/krk-ui/src/appkit/editor.rs` — Herkunft derselben.

## Wohin die Zuordnung Erscheinungsbild → Tafel gezogen ist

Nach `crates/krk-ui/src/appkit/textmerkmale.rs`, als
`pub fn tafel_der_erscheinung(sicht: &NSView) -> Tafel`. Der Rumpf ist
unverändert; der Doc-Kommentar hat zwei Sätze über die beiden Aufrufer
bekommen, der Modulkopf einen eigenen Abschnitt mit der Begründung.

**Die Wahl ist eine Ausschlussrechnung über die beiden Kandidaten**, und beide
Ausschlüsse stehen im Modulkopf:

- **Nicht `hervorhebung.rs`.** Die Frage hängt am wirksamen Erscheinungsbild
  einer `NSView`. Jene Datei trägt keine Zeile AppKit, sagt das im eigenen Kopf
  zu, und S16 misst es, indem es die Kistennamen zählt. Sie nimmt die `Tafel`
  als Angabe herein und wählt sie nicht aus.
- **Nicht privat in `editor.rs`.** Solange der Editor der einzige Verbraucher
  war, war das richtig. Die Vorschau braucht dieselbe Antwort zweimal: für die
  Verweisfarbe im gerenderten Markdown und für jeden Einfärbungslauf.

`textmerkmale.rs` ist die AppKit-Seite derselben Naht — nebenan wird mit einer
Tafel gerechnet, hier steht, woher sie kommt und was aus dem Ergebnis wird. Es
gibt weiterhin **eine** Zuordnung; `grep` findet
`bestMatchFromAppearancesWithNames` genau einmal im Baum.

Mitgezogen sind die Verfügbarkeitsangaben: `NSAppearance` und
`effectiveAppearance` 10.9, `bestMatchFromAppearancesWithNames:` und
`NSAppearanceNameDarkAqua` 10.14 — die beiden jüngsten Angaben im Kopf jener
Datei, alle am SDK gelesen. Aus `editor.rs` sind `NSAppearanceCustomization`,
die beiden Erscheinungsnamen und `NSArray` als Einfuhren verschwunden.

## Der Weg durch die Ansicht

```
datei_anzeigen ──> Modell, Arbeitsfaden krk-vorschau ──┐
                   (lesen, Markdown rendern)           │
                                                       v
LADETAKT (1/60 s) ──> einziehen ──> anzeigen ──> Text steht  [Endbedingung L7]
                          │                          │
                          │                          └─> Markdown: anwenden, sofort
                          │                          └─> Code: einfaerbung_nachfuehren
                          │                                      │ krk-einfaerbung
                          └─> einfaerbung_einziehen <────────────┘
                                     └─> anwenden, Farben stehen
```

`Vorschaumodell::laedt_noch` ist unberührt; `vorschaumodell.rs` steht nicht in
der Liste der geänderten Dateien. Der Takt endet erst, wenn weder ein Tab lädt
noch eine Einfärbung läuft — ein zweiter Zeitgeber ist nicht entstanden.

## Vier Stellen, an denen die Umsetzung über den Wortlaut des Plans hinausgeht

Jede einzeln begründet, keine ist eine Abweichung in der Sache.

### 1. Ein dritter Wert in den Ivars: `einfaerbung_erneut`

Der Plan nennt zwei Felder und schreibt: „Ein Tabwechsel oder ein neuer Inhalt
lässt einen laufenden Einfärbungsvorgang fallen; der Empfänger fällt mit, und
das `send` des überholten Fadens scheitert still."

**Wörtlich gebaut ergibt das einen Fädenstau.** Ein fallengelassener Faden hört
nicht auf zu rechnen — er rechnet zu Ende und scheitert erst am `send`. Wer mit
den Pfeiltasten durch einen Ordner geht, löst je Eintrag eine Anfrage aus; bei
Dateien nahe `TEXTGRENZE` sind das je Schritt Sekunden Rechenzeit, und die
liefen gegen das Lesen des nächsten Eintrags — also gegen L7, die Zusage, die
dieser ganze Schnitt schützt.

Gebaut ist deshalb der Handgriff, mit dem der Editor dasselbe Problem löst und
den der Plan im selben Satz verlangt („dieselbe Bauart wie im Editor, ohne
Anfragenummer"): läuft schon ein Faden, wird kein zweiter gestartet, sondern
vermerkt, dass sein Ergebnis überholt sein wird. Fallengelassen wird damit das
**Ergebnis** und nicht der Empfänger. Zu jedem Zeitpunkt lebt höchstens ein
Faden, und eingefärbt wird der letzte Stand statt jedes Zwischenstandes. Eine
Anfragenummer ist nicht entstanden.

### 2. `datei_anzeigen` zieht nur noch die Tableiste nach

Bisher rief es `anzeigen()`. Geändert hat sich zu diesem Zeitpunkt aber allein
die **Beschriftung** des Tabs: Inhalt und Pfad wechseln erst, wenn der
Arbeitsfaden geliefert hat. Bis dahin steht der bisherige Text da.

Vor diesem Schritt war der volle Durchgang bloß überflüssig. Jetzt wäre er
sichtbar falsch: `text_zeigen` setzt den Text neu und nimmt ihm dabei seine
Merkmale, und `einfaerbung_nachfuehren` forderte sie sofort wieder an. Bei jedem
Schritt durch eine Dateiliste ein Flackern und ein Faden für nichts.
`tableiste_nachziehen()` leistet genau das, was zu leisten ist.

### 3. `text_zeigen` nimmt die Merkmale des vorigen Inhalts zurück

Ohne das trüge ein Hinweis nach einer Markdown-Datei deren Überschriften. Ob
`setString:` die Merkmale des Textspeichers mitnimmt, ist eine Zusage von
AppKit, die wir nicht gelesen haben — der Kopf von `NSText.h` sagt zu
`@property (copy) NSString *string` nichts —, und die vorübergehenden Merkmale
des Layoutverwalters stehen ohnehin auf einem eigenen Blatt. Zurückgenommen wird
über `textmerkmale::zuruecksetzen`, die eine Stelle im Programm, die das tut.

**Das hat eine sichtbare Folge, und sie ist hier zu nennen:** die Grundschrift
der Vorschau ist damit die der Rohansicht aus `textmerkmale::grundschrift`, also
die feste Schreibmaschinenschrift in `systemFontSize` statt bisher in
`smallSystemFontSize`. Zwei Punkte größer, dieselbe Schriftart. Eine eigene
Zahl daneben wäre die zweite Wahrheit über die Grundschrift gewesen; deshalb
setzt auch der Aufbau der Fläche jetzt dieselbe Funktion statt eines eigenen
Aufrufs. Am Bündel zu beurteilen.

### 4. Die Darstellungsart kommt aus der Lieferung

`formatierung_anwenden` übergibt `formatierung.art` und fragt
`hervorhebung::art` nicht ein zweites Mal. Eine `Formatierung` nennt die
Besetzung, aus der sie entstanden ist; eine zweite Frage könnte anders
ausfallen als die, die die Listen erzeugt hat.

## Warum die Vorschau beide Werte von `Ansicht` übergibt

Sie kennt keinen Ansichtswechsel und benutzt trotzdem beide Werte — als Aussage
über den **Inhalt** und nicht über eine Wahl des Nutzers:

| Was die Fläche zeigt | Ansicht | Grundschrift |
|---|---|---|
| Leer, Hinweis, Metadaten, Text ohne Pfad, einfacher Text | `Roh` | fest, `systemFontSize` |
| Quelltext, eingefärbt | `Format` | fest, `systemFontSize` |
| gerendertes Markdown | `Format` | System, `systemFontSize + LESEZUSCHLAG` |

Für Quelltext sind beide dieselbe Schrift, weil `grundschrift` dort nicht
unterscheidet. **Deshalb springt die Anzeige nicht, wenn die Farben nachziehen**
— der Text steht sofort in der Schrift, in der er stehen bleibt.

## Der Wechsel des Erscheinungsbildes

`Inhaltsflaeche` nimmt `viewDidChangeEffectiveAppearance` entgegen und reicht es
**schwach** an das Vorschaufenster zurück, in der Bauart von `Editorsicht` im
Editor. Eine zweite Ansicht dafür ist nicht entstanden: die Inhaltsfläche ist
ohnehin da, und das wirksame Erscheinungsbild ist für jede Ansicht derselben
Kette dasselbe.

`erscheinung_nachziehen` setzt die Tafel und fordert neu an. Der aufgehobene
Stand wird dabei **nicht** weggeworfen: er trägt die alte Tafel in seinem
Schlüssel, und `fortschreiben` erkennt das und rechnet von vorn. Ihn hier
wegzuwerfen wäre dieselbe Entscheidung an einer zweiten Stelle.

**Gerendertes Markdown zieht damit noch nicht nach**, und das ist als Datensatz
festgehalten statt verschwiegen:
`issues/260812-1701_o_ein-gerendertes-markdown-behaelt-nach-dem-wechsel-auf-dunkel-die-verweisfarbe-der-hellen-tafel.md`.
Die Verweisfarbe entsteht beim Rendern auf dem Arbeitsfaden des Modells; sie
nachzuziehen hieße, jede Datei ein zweites Mal zu lesen, und das ist ein
Zuschnitt und kein Handgriff. Der Datensatz führt drei Möglichkeiten und wählt
keine.

## Was ausdrücklich nicht gebaut ist

- **Kein `NSLinkAttributeName`.** Farbe und Unterstreichung kommen als
  vorübergehende Merkmale, und die tragen keine Wirkung: kein Zeigefinger, kein
  Klick. Welche Quellen eine Adresse setzen dürfen, ist die erste offene Frage
  des Circles `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`, und
  sie hier nebenbei zu beantworten nähme jenem Circle seine Klärungsrunde. Der
  Modulkopf sagt es aus.
- **Keine zweite Umsetzung.** `grep` findet `addTemporaryAttribute` und
  `addAttributes_range` allein in `textmerkmale.rs`.
- **Keine Größenschranke neben `TEXTGRENZE`.** Eingefärbt wird jede Datei, die
  die Vorschau überhaupt als Text zeigt.
- **`setSelectable(false)` und `setEditable(false)` stehen unverändert.**
- **`nummernspalte.rs` und `vorschaumodell.rs` sind nicht angefasst.**

## Die beiden Proben

Zwei neue, beide ohne Fenster; die Probenzahl im Binärziel `krk` geht von 432
auf 434. Diese Runde baut keine neue Probe, die den Hauptfaden behauptet.

- `eingefaerbt_wird_genau_darstellungsart_code` misst die
  Anforderungsbedingung als reine Fallunterscheidung über `einzufaerben`: eine
  `.rs`-Datei liefert Text und Pfad, eine unbekannte Endung, eine
  `.md`-Endung und ein Text ohne Pfad liefern nichts, und die übrigen fünf
  Werte von `Inhalt` ebenfalls nicht — jeder mit dem Pfad einer Quelltextdatei
  daneben, damit es an ihnen liegt und nicht am Pfad. **Gegengeprobt:** die
  Bedingung testweise auf `!= EinfacherText` geweitet, die Probe schlägt fehl
  („Markdown geht über `Inhalt::Markdown`"); danach zurückgenommen.
- `das_vorschaumodell_weiss_von_der_einfaerbung_nichts` liest den Quellbaum,
  weil die Zusage eine Aussage über den **Ort** ist und an keinem Rückgabewert
  abzulesen: `vorschaumodell.rs` nennt weder `Einfaerbungsvorgang` noch
  `Einfaerbungsstand` noch `fortschreiben`. Die zweite Hälfte ist der Riegel
  gegen eine Probe, die alles bestätigt: `appkit/vorschau.rs` **muss** die
  beiden Namen nennen. Dieselbe Art der Abnahme, mit der `teilen.rs` seine
  Zählproben führt.

## Die Zahl 10.7 für NSLayoutManager

Der Kopf von `vorschau.rs` nennt sie neu, und der von `editor.rs` ist von 10.0
auf 10.7 berichtigt (`NSLayoutManager.h:65`, selbst am SDK nachgelesen). Damit
ist die eine der beiden Stellen aus
`shared/issues/260812-1558_o_zwei-modulkoepfe-nennen-fuer-nslayoutmanager-macos-10-0-das-sdk-sagt-10-7.md`
behoben. **Die zweite steht offen:** `crates/krk-ui/src/appkit/nummernspalte.rs`
führt die Klasse weiterhin ohne eigene Angabe, und die Datei gehört nicht zu
diesem Schritt. Der Datensatz verlangt beide in einem Zug; er ist damit noch
nicht zu schließen.

## Was nicht getan wurde

Kein Vordergrundlauf, kein Bündelbau, keine Messung der zehn Zusagen. Nicht
committet — das tut der Nutzer.

Zwei Datensätze sind neu abgelegt:

- `issues/260812-1701_o_…verweisfarbe-der-hellen-tafel.md` (siehe oben).
- `issues/260812-1702_o_der-kopf-von-appkit-mod-rs-sagt-die-vorschau-rufe-textmerkmale-noch-nicht.md`
  — der Modulkopf von `appkit/mod.rs` sagt „Heute ruft allein `editor` hier
  herein" und führt `textmerkmale` im Kastenbild nur unter dem Editor. Beides
  ist seit diesem Schritt falsch; die Datei stand nicht in der Dateiliste, und
  die Korrektur sind drei Zeilen ohne Entscheidung.

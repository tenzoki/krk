# Ontoreview: Die Datumszeilen der vier flight-Speicher

**Reviewed-range:** `e5ec81a..fb50fcd`
**Not-opened:** `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_c_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md`, `fusion-workbench/shared/decisions/260825-1725_i_liest-eine-zusammenfassung-denselben-unterordner-einmal-oder-je-zeile.md`, `fusion-workbench/shared/decisions/260825-1725_i_nimmt-ein-klick-auf-die-tableiste-des-anderen-dateifensters-den-ersthelferrang-mit.md`, `fusion-workbench/shared/decisions/260825-1725_i_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile.md`, `fusion-workbench/shared/decisions/260825-1725_i_wo-wohnt-die-umrechnung-von-systemtime-in-buergerliche-ortszeit.md`, `fusion-workbench/shared/history/260826-0157-reconciliation.md`, `fusion-workbench/shared/history/260826-0818-curator-run.md`, `fusion-workbench/shared/issues/260825-2230_c_der-plan-der-runde-18-verlangt-in-schritt-3-noch-die-zeile-in-der-abschlussliste-die-acc9671-gestrichen-hat.md`, `fusion-workbench/shared/issues/260826-0149_o_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-an-der-vorschau.md`, `fusion-workbench/shared/issues/260826-0149_o_die-runde-18-hat-keinen-circle-datensatz-und-jede-zaehlung-ueber-circles-uebergeht-sie.md`, `fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`, `fusion-workbench/shared/reviews/260826-0139-coderev-dritte-nachdurchsicht-die-beispielzahl-vier-haelt.md`

**Sender:** ontorev
**Gegenstand:** `180fc53` an `resources/default-readers.toml`, gegen den Handbuchteil im Kopf
derselben Datei, gegen die acht fusion-Profile als Vorbild, gegen den Quelltext in
`crates/krk-core/src/leseprofil/{datei,bausteine}.rs` und
`crates/krk-core/src/ablage/leseprofile.rs`, und gegen die zwei `Resolved:`-Vermerke unter
`shared/issues/260825-2126_c_*.md`. `crates/krk-core/tests/leseprofil.rs` ist gelesen, soweit es
die mitgelieferte Fassung misst; die Probendatei als Ganzes gehört `coderev`.

## Summary

Alle vier Zahlen, die der Ontocoder berichtet, stimmen, und sie stimmen nicht nur in seiner
Herleitung, sondern in einer unabhängigen Messung an zwei Prüfordnern und an der wirklichen
flight-Werkbank. Die zwei Blöcke sind nach der Änderung zeichengleich, die acht fusion-Profile
sind unangetastet, beide `Resolved:`-Vermerke tragen, was sie behaupten, und keine der acht
neuen Zeilen verletzt einen der Bausteine. Der dritte, nicht beauftragte Halbsatz war
tatsächlich schon vorher falsch, und seine neue Fassung ist richtig und vollständig.
Drei niedrige Befunde, alle drei am fehlenden Halt und keiner an einem Wert.
**Der Stand kann ausgeliefert werden.**

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 3 |

## Wie geprüft wurde

Nicht durch Hinsehen. Eine Messhilfe im Sitzungsverzeichnis, eigene Kiste mit einer
Pfadabhängigkeit auf `krk-core`, lädt eine beliebige Profildatei über `toml::from_str` und
`leseprofil::datei::pruefen` und fährt danach `leseprofil::bausteine::zusammenfassen_gezaehlt`
an genannten Orten. Der Baum ist nicht angefasst; die Profildatei ist nicht angefasst.

Gefahren gegen drei Fassungen der Datei — den Stand `e5ec81a`, den Stand nach `180fc53` und
eine Abwandlung mit `zeigt = "titel"` an der Ablagezeile — und an fünf Orten: zwei künstlichen
flight-Werkbänken, einer künstlichen Projektwurzel, der wirklichen Werkbank unter
`/Users/k1/Projects/productive/example/flight-workbench` und der Projektwurzel darüber.

```
cargo test -p krk-core --test leseprofil     47 grün, 1 übersprungen
cargo test -p krk-core --lib leseprofil      10 grün
```

## Die fünf Prüfpunkte der Dispatch

### 1. Die Leselaufrechnung: nachgerechnet, nachgemessen, sie stimmt

Die Regel steht im Kopf der Datei: die Zahl der Leseläufe ist die Zahl der **verschiedenen**
genannten Orte, plus einen Lauf für die Erkennung, wenn das Profil über sein `kennzeichen`
erkannt wurde und keine seiner Zeilen den erkannten Ordner selbst nennt.

`flight-Werkbank: die Wurzel` nennt fünf Orte: den erkannten Ordner, den die drei Feldzeilen
meinen, und `decisions`, `history`, `memos`, `archive`. Der erkannte Ordner ist einer der fünf,
also teilt sich die Erkennung über `.flight-setup` seine Lesung, und es bleibt bei fünf.

`Projektwurzel mit flight-Werkbank` nennt ebenfalls fünf Orte, aber andere: `flight-workbench`
und dessen vier Speicher. Der erkannte Ordner — die Projektwurzel — steht nicht darunter, also
liest ihn allein die Erkennung, und der Lauf kommt obendrauf. Fünf plus eins ist sechs.

**Der Unterschied 5 gegen 6 ist damit genau der Erkennungslauf, und die Begründung des
Kommentars trägt.** Sie ist außerdem dieselbe, die die Datei vierhundert Zeilen weiter oben für
das fusion-Paar gibt (drei gegen vier), und die dort seit `96e32cb` eine Probe hält.

Gemessen, unabhängig von jeder Herleitung:

| Ort | Profil | Leseläufe | Öffnungen | Zeilen |
|---|---|---|---|---|
| künstliche flight-Werkbank, voll besetzt | flight-Werkbank: die Wurzel | 5 | 3 | 11 |
| künstliche Projektwurzel darüber | Projektwurzel mit flight-Werkbank | 6 | 3 | 11 |
| `example/flight-workbench` | flight-Werkbank: die Wurzel | 5 | 3 | 11 |
| `example` | Projektwurzel mit flight-Werkbank | 6 | 3 | 11 |
| leeres `flight-workbench` | Projektwurzel mit flight-Werkbank | 2 | 0 | 11 × `--` |

Und dieselbe Messung gegen den Stand `e5ec81a`, also vor der Änderung: ebenfalls 5 und 3
beziehungsweise 6 und 3, bei sieben Zeilen. **Die vier Datumszeilen kosten wirklich null
zusätzliche Läufe und null Öffnungen.** Sie nennen die Orte, die die Zählungen schon nennen, und
`zeigt = "datum"` liest das Änderungsdatum aus dem Verzeichniseintrag, den der Leselauf ohnehin
liefert; der Typ `Juengstedatei` und die Zuordnung `Anzeigedatei::Datum` in
`crates/krk-core/src/leseprofil/datei.rs` bestätigen das am Quelltext.

Die drei Öffnungen gehen sämtlich auf die drei Feldzeilen, die dieselbe `.flight-setup` dreimal
öffnen — das ist die Regel „zwei Feldbausteine über derselben Datei öffnen sie zweimal" aus dem
Kopf der Datei, und die drei Werte im Ergebnis (`2026-Sommer-Adria`, der
Einrichtungszeitpunkt, `0.8.0`) sind der Nachweis, dass jede Öffnung etwas gefunden hat.

Die zwei Restangaben stimmen ebenfalls: 12 − 5 = 7 und 12 − 6 = 6.

**Ein Vorbehalt, ausdrücklich kein Befund.** Fünf ist die Zahl an einer besetzten Werkbank und
zugleich die obere Schranke; ein Ort, den es nicht gibt, wird nicht gelesen. Gemessen an einem
Prüfordner ohne `history` und `memos`: drei Läufe. Da der Kommentar die Zahl gegen die Schranke
von zwölf stellt, ist die obere Schranke die richtige Angabe, und sie steht richtig da.

### 2. Die zwei Blöcke: zeichengleich, und beide Paare sind es

Nachgemessen mit derselben Rechnung, die die zwei `Resolved:`-Vermerke behaupten. Normalisiert
man im zweiten Block das vorangestellte `flight-workbench/` weg **und** den bei den Feldzeilen
hinzugekommenen Schlüssel `ordner`, sind die elf Zeilen des flight-Paares zeichengleich. Die
Gegenprobe am fusion-Paar mit derselben Normalisierung: ebenfalls zeichengleich, sieben Zeilen.

Der Unterschied zwischen den zwei Normalisierungsschritten ist kein Nebenaspekt, sondern der
dritte Befund unten: der Doppelungshinweis beschreibt nur den ersten.

Gehalten wird die Gleichheit von nichts — das ist der zweite Befund.

### 3. Der neu gefasste Kommentarkopf: beide Behauptungen halten

**Die alte Fassung war schon vorher falsch.** Sie sagte „die vier Profile unten sind aus
demselben Grund kürzer als ihre Vorbilder". Gezählt am Stand `e5ec81a`:

| flight-Profil | Zeilen | fusion-Vorbild | Zeilen |
|---|---|---|---|
| die Wurzel | 7 | die Wurzel | 7 |
| ein Speicher | 2 | ein Speicher | 2 |
| der Ablagespeicher | 2 | der Ablagespeicher | 2 |
| Projektwurzel | 7 | Projektwurzel | 7 |

Kein einziges war kürzer; alle vier waren gleich lang. Mit elf gegen sieben wären zwei davon
nach der Änderung länger gewesen, die Aussage hätte sich also von falsch zu offen falsch
gedreht. Der Eingriff war fällig, und dass er nicht beauftragt war, ändert daran nichts.

**Die neue Fassung stimmt, und ihre Aufrechnung ist überschneidungsfrei und vollständig.** Acht
fusion-Profile stehen vier flight-Profilen gegenüber; die vier fehlenden sind der
Defektspeicher, die zwei Rundenprofile und der gemeinsame Speicher, dessen Aufgabe das
Wurzelprofil übernimmt. Vier plus vier ist acht, und keiner der vier wird zweimal gezählt. Die
Begründung stimmt auch in der Sache: eine flight-Werkbank führt keine Runden, kennt keine
Zustandsmarker im Dateinamen und hat kein `shared/`, und die wirkliche Werkbank unter
`example/` trägt genau die vier Speicher, die das Wurzelprofil jetzt führt, dazu `stilwerk`,
das der Kommentar ausdrücklich ausnimmt.

**Der zweite geänderte Halbsatz, `.flight-setup` gegen `.fusion-setup`, ist ebenfalls
belegt.** `fusion-workbench/.fusion-setup` in diesem Baum trägt zwei Felder,
`example/flight-workbench/.flight-setup` trägt dieselben zwei und dazu `setup_pwd`. Beide
Dateien sind gelesen; die Angabe im Kommentar ist keine Annahme.

### 4. Was ein Schreibfehler kostet: keine der acht neuen Zeilen kann ihn auslösen

Jede neue Zeile trägt eine `beschriftung` und genau einen Baustein. Die vier Schlüssel im
Baustein — `ordner`, `muster`, `anzahl`, `zeigt` — sind genau die Felder von `Juengstedatei`,
und die Struktur trägt `#[serde(deny_unknown_fields)]`; ein verschriebener Schlüssel wäre die
weiteste der drei Reichweiten und nähme die ganze Datei mit. `zeigt = "datum"` ist einer der
zwei Werte, die `Anzeigedatei` unter `rename_all = "lowercase"` annimmt; ein dritter kostete
ebenfalls die Datei. Keine Ortsangabe trägt einen abschließenden Schrägstrich, ein `.`, ein
`..` oder einen Platzhalter — der letzte wäre bei `juengste` ohnehin abgewiesen.

Belegt ist das nicht durch Lesen allein: `datei::pruefen` liefert an der geänderten Datei zwölf
Profile und **keine** Meldung, und das ist derselbe Prüfschritt, den die Nutzerdatei nimmt.

Die eine Fehlerart, die dieser Schritt nicht fängt, ist ein syntaktisch gültiger, sachlich
falscher Ordnername: er zeigte still `--`. Auch die ist ausgeschlossen, gemessen an der
wirklichen Werkbank, wo alle vier Speicherzeilenpaare einen Wert liefern — bis auf „Ablagen,
zuletzt", und dort ist `--` die richtige Antwort, weil das `archive` jener Werkbank leer ist.

Die Gegenprobe zur Behauptung des Kommentars, `zeigt = "datum"` sei am Ablagespeicher keine
Wahl unter zweien: an einem Prüfordner mit zwei Läufen als Ordner liefert die Zeile mit
`zeigt = "datum"` ein Datum und mit `zeigt = "titel"` den Platzhalter. Der Satz stimmt.

### 5. Keine Probe im Baum nennt `flight`

Bestätigt: die Zeichenfolge kommt unter `crates/`, in `xtask/` und in `README.md` an keiner
Stelle vor. Was der Baum trotzdem hält, ist mehr als nichts —
`die_eingebettete_fassung_besteht_ihre_eigene_pruefung` und die Hilfe `ausgelieferte` schicken
die ausgelieferte Fassung durch `datei::pruefen` und halten die Zahl zwölf, ein verschriebener
Schlüssel oder ein dritter `zeigt`-Wert würde also beim Bauen rot. Was der Baum **nicht** hält,
sind die vier Zahlen. Das ist der erste Befund.

### 6. Die zwei Vermerke und die acht fusion-Profile

Beide `Resolved:`-Vermerke tragen, was sie behaupten, Halbsatz für Halbsatz gegen die Datei
gelesen. Der Hinweis steht jetzt über beiden flight-Blöcken, in derselben Aufteilung wie beim
fusion-Paar — ausführlich mit Grund über dem Wurzelprofil, kurz mit Rückverweis über der
Projektwurzel —, und die Zahl darin ist in beiden auf elf nachgezogen. Der Satz über die drei
Felder steht nicht mehr da; an seiner Stelle steht die gemessene Lage samt dem verlangten
Verweis auf den Absatz beim fusion-Wurzelprofil.

Die acht fusion-Profile sind unverändert. Nachgehalten nicht am Diff, sondern durch einen
zeichenweisen Vergleich der zwölf Blöcke zwischen `e5ec81a` und `fb50fcd`: acht unverändert,
zwei geändert, zwei — `flight-Werkbank: ein Speicher` und `flight-Werkbank: der
Ablagespeicher` — ebenfalls unverändert. Das deckt sich mit der Angabe des Ontocoders, dass
`flight-Werkbank: ein Speicher` bewusst nicht mitgewachsen ist, weil sein fusion-Vorbild es
auch nicht ist.

## Die drei Befunde

Alle drei sind niedrig, keiner betrifft einen Wert oder ein Muster, und keiner steht der
Auslieferung entgegen.

**N1 — Keine Probe im Baum hält die vier Zahlen der flight-Profile.**
`shared/issues/260826-0902_o_keine-probe-im-baum-haelt-die-vier-zahlen-der-flight-profile.md`.
Der Fall ist derselbe, den dieses Projekt vor einem Tag für fusion abgelegt
(`260825-2233_c_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md`) und mit
`96e32cb` behoben hat. Zuständig: `coder`.

**N2 — Die Zeichengleichheit der zwei Werkbankpaare wird je Durchsicht von Hand gemessen und
von nichts gehalten.**
`shared/issues/260826-0903_o_die-zeichengleichheit-der-zwei-werkbankpaare-wird-je-durchsicht-von-hand-gemessen-und-von-nichts-gehalten.md`.
Die Datei verwirft eine Vererbung oder Vorlage im Datenformat ausdrücklich, und zu Recht; eine
Probe ist weder das eine noch das andere. Zuständig: `coder`.

**N3 — Der Doppelungshinweis sagt „vor der Ortsangabe", und fünf von sieben fusion-Zeilen haben
keine.**
`shared/issues/260826-0904_o_der-doppelungshinweis-sagt-vor-der-ortsangabe-und-fuenf-von-sieben-fusion-zeilen-haben-keine.md`.
Die Ungenauigkeit ist älter als dieser Commit und mit der Behebung von `260825-2126` wörtlich
auf den vierten Block gekommen. Zuständig: `ontocoder`.

## Was nicht gedoppelt ist

`260825-2044_o_die-zeile-projekt-der-werkbankprofile-haengt-an-einem-feld-das-fusion-nicht-mehr-schreibt.md`
bleibt offen und ist von dieser Änderung berührt, aber nicht erledigt: an einer flight-Werkbank
liefert die Zeile „Projekt" einen Wert, an einer fusion-Werkbank weiter nicht. Der neue
Kommentartext schreibt genau diese Zweiteilung jetzt aus, was den Datensatz besser lesbar
macht und ihn nicht schließt.

`260825-2126_o_die-summenprobe-der-sechs-zustandszeilen-faellt-schon-an-einem-ds-store.md` ist
der dritte Befund der ersten Durchsicht und von dieser Änderung nicht berührt.

`260826-0139_o_*` (zwei Datensätze zur C6.7-Probe) betreffen den fusion-Teil derselben Probe
und sind hier nicht gedoppelt. N1 grenzt an sie an: wer die Probe um die flight-Fälle
erweitert, berührt beide Stellen und sollte sie mitnehmen.

## Cross-cutting

**Dieselbe Änderung, die die fusion-Profile vor einem Tag bekommen haben, ist bei flight ohne
ihre Vorkehrung angekommen.** Die Runde 18 hat für fusion drei Dinge zusammen gebracht: die
Datumszeilen, den Kommentar mit den Zahlen, und — nach einem Befund — die Probe, die die Zahlen
hält. `180fc53` bringt die ersten zwei und nicht die dritte. Das ist kein Fehler an der
Änderung, sondern ein Muster: der zweite Durchgang derselben Arbeit übernimmt die Form und
nicht die Absicherung, die der erste sich erst nach einer Durchsicht zugelegt hat. N1 und N2
sind beide von dieser Art.

**Die Prosa wächst schneller als die Daten, und das ist bei dieser Datei anders zu bewerten als
sonst.** Acht neue Profilzeilen bringen 44 neue Kommentarzeilen; der Anteil bleibt bei
54 Prozent, weil er dort schon war. Bei einer Datei, die beim ersten Start wörtlich kopiert und
danach nie wieder überschrieben wird, ist ausführliche Prosa die richtige Wahl — und sie ist
zugleich der Grund, warum ein falscher Satz darin teurer ist als anderswo: er wandert mit und
niemand zieht ihn beim Nutzer nach. Alle drei Befunde sitzen an dieser Naht.

**Der Ontocoder hat seine eigene Grenze richtig benannt.** Sein Verlaufsprotokoll schreibt aus,
dass keine Probe `flight` nennt und die Zahlen allein durch seine Messung und den Kommentar
gehalten sind. Das ist die Angabe, aus der N1 entstanden ist; sie ist nicht gegen ihn gefunden,
sondern von ihm geliefert. Jede seiner nachprüfbaren Behauptungen hat der Nachmessung
standgehalten.

## Kann der Stand ausgeliefert werden?

**Ja.**

- Die zwölf Profile laden ohne Meldung, und jedes greift an dem Ort, für den es geschrieben
  ist. An der wirklichen flight-Werkbank und der Projektwurzel darüber liefern alle elf Zeilen
  die richtige Auskunft.
- Die vier Zahlen des Kommentars sind unabhängig nachgemessen und stimmen, an einem Prüfordner
  wie an der wirklichen Werkbank.
- Die zwei Blöcke sind zeichengleich, und das fusion-Paar ist es auch.
- Die acht fusion-Profile sind zeichenweise unverändert.
- Keine der acht neuen Zeilen kann die Datei, ein Profil oder eine Zeile kosten; der
  Prüfschritt, den die Nutzerdatei nimmt, beanstandet nichts.
- Die drei Befunde sind sämtlich niedrig und betreffen den Halt gegen künftige Änderungen,
  nicht den heutigen Stand. Keiner kann einen Nutzer zu einer falschen Änderung an seiner
  eigenen Datei verleiten.

Beizulegen ist, was schon die vorige Durchsicht beigelegt hat und was an dieser Änderung nicht
liegt: die Fassung, die jetzt ausgeliefert wird, erreicht keinen Nutzer, der KRK schon einmal
gestartet hat, ohne dessen Handgriff
(`shared/decisions/260825-1725_i_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`,
`README.md`). Wer eine flight-Werkbank hat und die Datumszeilen sehen will, tauscht seine
`readers.toml` von Hand.

## Empfohlene Reihenfolge

1. **Ausliefern.** Kein Befund steht dem entgegen.
2. N1 und N2 zusammen, denn beide sind Proben an derselben Datei und N2 braucht die
   Normalisierung, die N3 beschreibt.
3. N3 — vier gleichlautende Halbsätze, bei nächster Gelegenheit an der Profildatei.

## Was ich nicht geprüft habe

- **Die Anzeige.** Was das Vorschaufenster aus `Zusammenfassung::als_text` macht, ist an einer
  laufenden Anwendung zu sehen und bleibt Nutzerarbeit.
- **`crates/krk-core/tests/leseprofil.rs` als Ganzes.** Gelesen ist, was die ausgelieferte
  Fassung misst; die Bewertung der Probendatei gehört `coderev`.
- **Die fünfzehn Dateien des Bereichs, die die Kopfzeile `Not-opened` nennt.** Sie sind
  Werkbankdatensätze aus dem Abgleich und dem Kuratorenlauf derselben Sitzung und stehen
  außerhalb des Gegenstands dieser Durchsicht.

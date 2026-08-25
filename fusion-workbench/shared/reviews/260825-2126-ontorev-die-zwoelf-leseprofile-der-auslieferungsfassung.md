# Ontoreview: die zwölf Leseprofile der Auslieferungsfassung

**Reviewed-range:** `20eccd4..8478753`
**Not-opened:** `README.md`, `crates/krk-core/src/lib.rs`, `crates/krk-core/src/operation/entpacken.rs`, `crates/krk-core/src/operation/zippen.rs`, `crates/krk-core/src/verzeichnis/mod.rs`, `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/leseprofil.rs`, `crates/krk-core/tests/operation.rs`, `crates/krk-core/tests/zeit.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_c_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md`, `fusion-workbench/shared/decisions/260825-1725_a_liest-eine-zusammenfassung-denselben-unterordner-einmal-oder-je-zeile.md`, `fusion-workbench/shared/decisions/260825-1725_a_nimmt-ein-klick-auf-die-tableiste-des-anderen-dateifensters-den-ersthelferrang-mit.md`, `fusion-workbench/shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`, `fusion-workbench/shared/decisions/260825-1725_a_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md`, `fusion-workbench/shared/decisions/260825-1725_a_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile.md`, `fusion-workbench/shared/decisions/260825-1725_a_wo-wohnt-die-umrechnung-von-systemtime-in-buergerliche-ortszeit.md`, `fusion-workbench/shared/history/260825-1725-plan-runde-18-vorschau-und-zwei-fehler.md`, `fusion-workbench/shared/history/260825-1747-plan-zahl-der-entscheidungsdatensaetze-berichtigt.md`, `fusion-workbench/shared/history/260825-1859-coder-zeitstempel-beim-packen-und-entpacken.md`, `fusion-workbench/shared/history/260825-1920-coder-ein-ort-wird-hoechstens-einmal-gelesen.md`, `fusion-workbench/shared/history/260825-1953-coder-die-ortsangabe-darf-einen-platzhalter-tragen.md`, `fusion-workbench/shared/history/260825-2013-coder-juengste-zeigt-auf-wunsch-ein-datum.md`, `fusion-workbench/shared/history/260825-2051-ontocoder-zwoelf-leseprofile-fuer-fusion-und-flight.md`, `fusion-workbench/shared/history/260825-2058-coder-der-weg-zu-einer-neuen-profildatei-steht-im-readme.md`, `fusion-workbench/shared/history/260825-2107-analyst-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`, `fusion-workbench/shared/issues/260825-1859_o_claude-md-nennt-fuer-zip-das-eine-merkmal-deflate-flate2-es-sind-zwei.md`, `fusion-workbench/shared/issues/260825-1859_o_eine-entpackte-verknuepfung-bekommt-ihr-aenderungsdatum-nicht.md`, `fusion-workbench/shared/issues/260825-1922_o_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md`, `fusion-workbench/shared/issues/260825-1922_o_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md`

**Sender:** ontorev
**Gegenstand:** `resources/default-readers.toml` allein. Der Quelltext daneben bekommt eine
eigene Durchsicht durch `coderev`, die parallel läuft; er ist gelesen, soweit zum Urteilen
nötig (`crates/krk-core/src/leseprofil/{mod,bausteine,datei,erkennung}.rs`,
`crates/krk-core/src/ablage/leseprofile.rs`, `Cargo.toml`), und trägt keinen Befund dieser
Durchsicht.

## Summary

Die zwölf Profile sind mechanisch in Ordnung: sie laden ohne eine einzige Meldung, jedes
antwortet an einem wirklichen Ort, kein Profil greift einen Ordner der jeweils anderen
Werkbank, und alle bleiben deutlich unter den Schranken. Acht Befunde liegen in der Prosa und
nicht in den Profilen — der Kommentarkopf ist das Handbuch dieser Datei, und er sagt an fünf
Stellen etwas anderes als der Mechanismus darunter, an zwei Stellen etwas anderes als er
selbst. Der einzige Befund an einem Muster ist die Zeile „Sitzung", deren `feldmuster` nicht
zeilenverankert ist und deshalb `### Current` oder eine Erwähnung im Fließtext greift, statt
den Platzhalter zu zeigen.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 5 |
| Low | 3 |

Alle acht sind als eigene Datensätze unter `fusion-workbench/shared/issues/` mit dem
Zeitstempel `260825-2126` abgelegt. Keiner doppelt einen der sieben bekannten Datensätze aus
dem Bereich `260825-1859` bis `260825-2107`.

## Wie geprüft wurde

Nicht durch Hinsehen. Ein Wegwerfpaket im Sitzungsverzeichnis mit einer Pfadabhängigkeit auf
`krk-core` lädt `resources/default-readers.toml` über `toml::from_str` und
`leseprofil::datei::pruefen` und fährt `leseprofil::zusammenfassen_gezaehlt` gegen wirkliche
und künstliche Orte. Es ruft ausschließlich öffentliche Schnittstellen, baut in sein eigenes
`target/` und hat keine Zeile im KRK-Baum hinterlassen. Der Baum ist unverändert; die
Profildatei ist nicht angefasst.

Geprüft wurde an drei Beständen: der Werkbank dieses Projekts (19 Runden), der
flight-Beispielwerkbank unter `/Users/k1/Projects/productive/example/` (nur gelesen) und
fünfzehn künstlichen Ordnern für die Randfälle. Dazu `cargo test -p krk-core --lib
leseprofile`: beide Proben grün, `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
eingeschlossen.

Der Ladelauf über die Auslieferungsfassung:

```
== geprüfte Profile: 12    == Meldungen: 0
   keine Zeile ohne Baustein, in keinem der zwölf Profile
```

## Findings nach Themen

### 1. Was eine Zusammenfassung kostet — zwei Angaben stimmen nicht

**M1 — Der Kommentar nennt eine geteilte Öffnung für drei Feldzeilen, gemessen sind es drei.**
`resources/default-readers.toml:253`. Die tote Zeile „Projekt" bleibt unter anderem deshalb
stehen, „weil sie sich ihre Öffnung mit den zwei Zeilen darunter teilt, also nichts kostet".
Gemerkt wird der **Ort** und nicht die Datei; drei Feldzeilen über derselben `.fusion-setup`
kosten drei Öffnungen. Dieselbe Datei sagt es bei `:220-222` richtig. Gemessen an der
Werkbankwurzel: 3 Leseläufe, **4 Öffnungen** bei fünf Feldzeilen, von denen eine ihr Ziel
nicht findet, also 3 + 0 + 1. Der Satz steht ein zweites Mal in
`shared/issues/260825-2044_*`, Möglichkeit 1.
→ `shared/issues/260825-2126_o_der-kommentar-nennt-eine-geteilte-oeffnung-fuer-drei-feldzeilen-gemessen-sind-drei.md`
· Schwere **mittel**

**M2 — Die Leselaufregel zählt den Erkennungslauf nicht mit.** `:218` gibt die Rechenregel
„die Zahl der Leseläufe eines Profils ist die Zahl der VERSCHIEDENEN Orte darin". Ein über
eine Kennzeichendatei erkanntes Profil, in dessen Zeilen der erkannte Ordner nicht vorkommt,
kostet einen mehr. Zwei der zwölf haben diese Gestalt:

| Profil | Orte in den Zeilen | Regel | gemessen |
|---|---|---|---|
| Projektwurzel mit fusion-Werkbank | 3 | 3 | **4** |
| Projektwurzel mit flight-Werkbank | 5 | 5 | **6** |

Die Regel ist keine Beschreibung, sondern eine Anleitung: `:451-452` schickt den Nutzer
ausdrücklich damit los, seinen elften Unterspeicher gegen die Schranke zu rechnen. Der
Doc-Kommentar von `HOECHSTENS_LESELAEUFE` sagt die fehlende Hälfte („Der Erkennungslauf zaehlt
mit"). Dieselben Zahlen stehen unabhängig erhoben in
`shared/analyses/260825-2107-…`, Zeilen 8 und 12.
→ `shared/issues/260825-2126_o_die-leselaufregel-der-datei-zaehlt-den-erkennungslauf-nicht-mit.md`
· Schwere **mittel**

Die übrigen Kostenangaben halten, alle nachgemessen: zwölf Leseläufe, vierundzwanzig
Öffnungen, 2.000 Einträge, 64 KB, zehn bei `anzahl`; „ein Ort kostet genau einen Leselauf"
(`:215`); die zehn Leseläufe des gemeinsamen Speichers bei zwanzig Zeilen (`:449`); die drei
Leseläufe des Rundenverzeichnisses bei acht Zeilen (`:373-375`).

### 2. Verankerung der Muster

**M3 — Das `feldmuster` der Zeile „Sitzung" ist nicht zeilenverankert.** `:296` und `:625`
suchen mit `'## Current\n…'` ohne `(?m)`. Gemessen an künstlichen
`orchestrator-live.md`:

| Datei enthält | Zeile „Sitzung" zeigt |
|---|---|
| `### Current` vor dem echten `## Current` | den Inhalt unter `### Current` |
| Fließtextzeile `siehe ## Current` vor dem echten Abschnitt | die Zeile darunter |
| CRLF-Zeilenenden | `--` |

Der Absatz `:178-183` derselben Datei schreibt `(?m)` für genau diesen Fall vor. Von den sechs
ausgelieferten Feldmustern ist dies das einzige, dessen Aussage an einem Zeilenanfang hängt,
und das einzige, das die eigene Regel nicht anwendet — die Zeile „Directive" trägt `(?sm)` und
verankert richtig. Der Kommentar `:256-258` sagt die richtige Folge einer Formänderung bei
fusion an („zeigt diese eine Zeile ihren Platzhalter"); ein Wert aus dem falschen Abschnitt ist
schlechter als der Platzhalter, weil er wie eine Antwort aussieht.
→ `shared/issues/260825-2126_o_das-feldmuster-der-zeile-sitzung-ist-nicht-zeilenverankert-und-faengt-den-falschen-abschnitt.md`
· Schwere **mittel**

**Die übrigen Muster sind richtig verankert.** Alle fünf `datei`-Muster stehen an beiden Enden
verankert, wie `:161-162` es zusagt: `^\.fusion-setup$`, `^\.active-circle$`,
`^orchestrator-live\.md$`, `^_._circle\.md$`, `^\.flight-setup$`. Die `pfad`-Muster sind vorn
bewusst offen — sie laufen gegen einen absoluten Pfad — und hinten mit `$` geschlossen, was
sie an die genannte Ebene bindet. Die `muster` der Namensbausteine sind entweder verankert
(`^_a_circle\.md$`, `\.md$`) oder als Teilzeichenfolge gewollt (`_o_.*\.md$`, `_._spec-`).

### 3. Vorrang und Überholung der Profile

**Kein Profil greift den Ordner der anderen Werkbank.** Geprüft an allen zwölf Profilen gegen
beide Bäume und gegen künstliche Mischfälle. Die fünf `pfad`-Muster verlangen `fusion-workbench/`
beziehungsweise `flight-workbench/` unmittelbar vor der genannten Ebene; die sieben
`kennzeichen` treffen paarweise verschiedene Namen. Ein Verzeichnis mit **beiden** Werkbänken
bekommt das fusion-Profil, wie `:727-729` es ansagt — nachgemessen.

**Der Vorrang selbst stimmt.** Die Zusage `:189-200` (zwei Durchgänge, `pfad` vor
`kennzeichen`, innerhalb eines Durchgangs die Dateireihenfolge) deckt sich mit dem Modulkopf
von `erkennung.rs`. Auch die Zusage `:543-545`, kein Pfadmuster der Datei treffe ein
Rundenverzeichnis, hält — nachgemessen an zwei wirklichen Runden. Die Speicher unter
`archive/<lauf>/shared/` bleiben ohne Profil (`KEIN PROFIL` an drei geprüften Orten), wie
`:425-428` es behauptet; die dort gegebene Begründung schreibt die Wirkung allerdings dem
Muster des Ablageprofils zu, während sie in Wahrheit von den Mustern der zwei Speicherprofile
kommt. Der Schluss stimmt, der Weg dahin nicht — als Beobachtung festgehalten, ohne Datensatz.

**M4 — Die zwei Projektwurzelprofile erkennen an einem Namen, den die Datei zwölf Zeilen
früher als untauglich verwirft.** `:243-246` begründet, warum das Wurzelprofil über
`.fusion-setup` erkennt und nicht über den Namen: ein Namensmuster „träfe jeden Ordner dieses
Namens, auch einen leeren". `:605` und `:732` tun genau das. Gemessen:

| Ordner enthält | Ergebnis |
|---|---|
| ein **leeres** Verzeichnis `fusion-workbench` | Projektwurzelprofil, sieben Zeilen `--` |
| eine **Datei** namens `fusion-workbench` | dasselbe |
| ein leeres Verzeichnis `flight-workbench` | flight-Projektwurzelprofil, sieben Zeilen `--` |

Sieben Platzhalter sind schlechter als die Metadatenanzeige, die `:15-19` für den Fall „kein
Profil trifft" zusagt. Der Zuschnitt ist der des Plans (Schritt 8: der Nutzer beantwortet die
Frage dort, wo er sie ändern kann) und damit gewollt; was fehlt, ist die Nennung seines
Preises — dieselbe Datei nennt den Preis der Doppelung zwei Absätze weiter ausdrücklich.
→ `shared/issues/260825-2126_o_die-zwei-projektwurzelprofile-erkennen-an-einem-namen-den-die-datei-zwoelf-zeilen-frueher-verwirft.md`
· Schwere **mittel**

### 4. Die sechs Zustandszeilen des Rundenverzeichnisses

**Sie sind überschneidungsfrei und vollständig, und die Summe geht auf.** Nachgerechnet an
dieser Werkbank:

```
Runden 19  =  Vorgesehen 0 + Aktiv 0 + Kohärent 5 + Beschränkt 12 + Überholt 0 + Zurückgestellt 2
```

Gegengeprüft am Dateibestand: 19 Rundenverzeichnisse, jedes mit genau einem
`_X_circle.md`, Marker 12 × `_b_`, 5 × `_c_`, 2 × `_d_`. Die sechs Muster
(`^_a_circle\.md$` bis `^_d_circle\.md$`) decken das Werkbankvokabular vollständig ab und
überschneiden sich nicht. Die Zeile „Offene Defekte, alle Runden" zeigt 116 und geht gegen
`find fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md' | wc -l` auf.

**L1 — Die Summenprobe fällt schon an einem `.DS_Store`.** Die Zeile „Runden" (`:393`) zählt
alle Einträge gleich welchen Typs, die sechs Zustandszeilen sehen über `ordner = "*"` allein
Unterordner. Gemessen an einem künstlichen `circles/` mit zwei Runden, einem `.DS_Store` und
einer `NOTIZ.md`: `Runden 4` gegen eine Summe von 2. Der Kommentar `:382` sagt dem Nutzer, eine
Differenz bedeute einen siebten Marker; sie kann auch eine Datei im Verzeichnis bedeuten. Auf
dem Bauziel dieser Anwendung ist das kein gedachter Fall: den `.DS_Store` legt der Finder an,
und KRK trägt seit der Runde 17 „Im Finder öffnen" im Kontextmenü.
→ `shared/issues/260825-2126_o_die-summenprobe-der-sechs-zustandszeilen-faellt-schon-an-einem-ds-store.md`
· Schwere **niedrig**

### 5. Die Doppelung der sieben Wurzelzeilen

**Heute läuft nichts auseinander.** Mechanisch verglichen: normalisiert man im zweiten Block
das vorangestellte `fusion-workbench/` beziehungsweise `flight-workbench/` weg, sind alle
vierzehn Angaben je Paar zeichengleich — für fusion wie für flight. Auch die Werte stimmen
überein, an beiden Bäumen gemessen.

**Der fusion-Kommentar nennt den Preis und erklärt ihn nicht weg.** `:267-273` sagt, dass die
Zeilen zweimal dastehen, dass nichts die Blöcke aneinanderhält, und warum der Preis bewusst
gezahlt ist. `:599-602` verweist darauf zurück. Das ist die Antwort, die der Plan in Schritt 8
verlangt hat.

**L2 — Bei flight steht der Hinweis nur über einem der beiden Blöcke.** `:721-725` trägt ihn,
`:649-656` nicht. Wer das flight-Wurzelprofil bearbeitet — der Block, den man zuerst findet —,
bekommt keine Warnung. Der Plan verlangt den Hinweis über **beiden** Blöcken; für fusion ist
das erfüllt, für flight nicht.
→ `shared/issues/260825-2126_o_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md`
· Schwere **niedrig**

### 6. Der Kommentarkopf gegen den Mechanismus

**M5 — Die drei Reichweiten eines Schreibfehlers überschneiden sich und sind unvollständig.**
`:41-55`. Gemessen über `datei::pruefen` an sechs Verschreibungen:

| Verschrieben | Reichweite laut Datei | gemessen |
|---|---|---|
| `kennzeichnen` statt `kennzeichen` (Profilebene) | 1 (ganze Datei) | **2** (nur das Profil) |
| `nane`, `beschriftunng`, `musster`, `zahlung`, `zeigt = "beides"` | 1 | 1 |

`Profilblock` trägt als einziger der sieben Tische kein `deny_unknown_fields`, und das ist eine
bewusste Wahl mit Begründung im Quelltext. Reichweite 1 der Datei sagt unqualifiziert „ein
unbekannter Schlüssel" und beansprucht damit einen Fall, den Reichweite 2 korrekt schon
enthält. Umgekehrt fehlen Reichweite 3 zwei Gründe, die es seit dieser Runde gibt und die der
Quelltext ausdrücklich führt: das leere Stück (`ordner = "planning/"`) und der zweite
Platzhalter (`ordner = "*/*/x"`). Beide kosten gemessen die Zeile und nicht die Datei; für den
zweiten Platzhalter sagt `:99-100` „werden beim Laden abgewiesen" ohne Angabe der Reichweite,
und die nächstliegende Lesart der Datei führt zum falschen Schluss.
→ `shared/issues/260825-2126_o_die-drei-reichweiten-eines-schreibfehlers-ueberschneiden-sich-und-sind-unvollstaendig.md`
· Schwere **mittel**

**L3 — Der flight-Kommentar nennt drei Felder in `.fusion-setup`, es sind zwei.** `:646-647`.
Gelesen: `.fusion-setup` trägt heute `setup_at` und `plugin_version`, `.flight-setup` dazu
`setup_pwd`. Dieselbe Datei sagt das bei `:248-254` selbst. Der Schluss des Satzes — dieselben
Muster greifen — stimmt; die Prämisse zeigt gerade auf den Unterschied, den der offene
Datensatz `260825-2044` festhält.
→ `shared/issues/260825-2126_o_der-flight-kommentar-nennt-drei-felder-in-fusion-setup-es-sind-zwei.md`
· Schwere **niedrig**

**Der Rest des Kopfes stimmt, Aussage für Aussage geprüft.** Die vier Bausteine und ihre
Wirkung; die drei Ausgänge von `vorhandensein` (nachgemessen: `ja` an einer Runde mit Plan,
`nein` an einer ohne Spec bei vorhandenem `planning`, `--` bei fehlendem `planning`); die
Titelform und die Datumsform von `juengste` samt der Aussage, dass die Datumsform nichts
öffnet und Einträge jedes Typs sieht (nachgemessen am Ablagespeicher: zwei Ordner, ein Datum,
null Öffnungen); die Zurückweisung von `ordner = "planning/"`; die vier Eigenschaften des
Platzhalters, einschließlich der Zurückweisung an `juengste` und `feld`; die neun Speichernamen
unter `shared/` und die fünf je Runde; die zehn Unterspeicher des gemeinsamen Speichers; die
vier Speicher und die ausgelassene `stilwerk` bei flight.

### 7. Die flight-Profile an der Beispielwerkbank

**Jede Zeile liefert einen Wert, mit einer Ausnahme, und die ist ehrlich.** Gemessen an
`/Users/k1/Projects/productive/example/` (nur gelesen):

| Ort | Ergebnis |
|---|---|
| `example/` | Projekt `2026-Sommer-Adria`, Eingerichtet, Fassung `0.8.0`, 1 / 7 / 3 / 0 |
| `example/flight-workbench/` | dieselben sieben Werte, ein Leselauf weniger |
| `…/decisions` | 1 Datensatz, ein Titel |
| `…/history` | 7 Datensätze, sieben Titel |
| `…/memos` | 3 Datensätze, drei Titel |
| `…/archive` | Läufe `0`, Zuletzt abgelegt `--` |
| `…/stilwerk` | KEIN PROFIL, wie beabsichtigt |

Der Platzhalter am Ablagespeicher ist die richtige Auskunft über einen leeren Ordner und kein
Befund. Die Zeile „Ablagen" am Wurzelprofil zeigt dafür `0` und nicht den Platzhalter, weil
`zaehlung` den vorhandenen leeren Ordner zählt — die zwei Antworten sind verschieden und beide
richtig.

Eine Einschränkung, die ich nicht auflösen kann: der Kommentar `:691-695` begründet das eigene
Ablageprofil damit, dass flight seine Läufe „als Ordner führt und keine Datensätze als
Dateien". Der Ablagespeicher der Beispielwerkbank ist leer; die Aussage ist an diesem Bestand
weder zu bestätigen noch zu widerlegen. Sie ändert am Ergebnis nichts, weil `zeigt = "datum"`
Einträge jedes Typs sieht.

## Cross-cutting

**Sieben der acht Befunde liegen in der Prosa, nicht in den Profilen.** Das ist keine
Nebensache dieser Datei, sondern ihr Gegenstand: 392 der 760 Zeilen sind Kommentar, und die
Datei wird wörtlich in das Heimatverzeichnis des Nutzers gelegt und danach nie wieder
angefasst. Was hier falsch steht, steht bei jedem Nutzer falsch und wird von keinem Update
berichtigt.

**Der Quelltext hat jede dieser Aussagen richtig.** Bei M1, M2 und M5 steht die genaue Fassung
im Doc-Kommentar von `HOECHSTENS_OEFFNUNGEN`, `HOECHSTENS_LESELAEUFE` beziehungsweise im
Modulkopf von `leseprofil::datei`, und die Auslieferungsfassung hat sie beim Übertragen
verkürzt. Das ist ein Muster und keine Häufung von Einzelfällen: die zwei Fassungen derselben
Aussage stehen an zwei Stellen, und nichts hält sie aneinander — dieselbe Lage, die der
Doppelungshinweis für die vierzehn Profilzeilen ausdrücklich benennt und für die Prosa nicht.

**Zwei Befunde sind Widersprüche innerhalb der Datei** (M1 gegen `:220-222`, L3 gegen
`:248-254`), zwei weitere Spannungen zwischen zwei Begründungen (M4 gegen `:243-246`, die
Begründung bei `:425-428`). Wer die Datei künftig erweitert, hat nach jeder Erweiterung zwei
Fragen zu stellen: sagt der neue Absatz dasselbe wie der Quelltext, und sagt er dasselbe wie
der Absatz zweihundert Zeilen weiter oben.

## Empfohlene Reihenfolge

**Vor der nächsten Auslieferung**, weil die Datei beim ersten Start des Nutzers geschrieben
wird und danach nicht mehr:

1. **M3** — das `(?m)^` an beiden Stellen. Die einzige Änderung an einem Muster, und die
   einzige, die heute einen falschen Wert zeigen kann.
2. **M1, M2, M5** — die drei Aussagen, die dem Mechanismus widersprechen. Jede ist ein
   Halbsatz oder ein Listenpunkt.

**Danach, als Aufräumarbeit:**

3. **M4** und **L1** — je ein Satz, der einen Preis nennt, den die Datei heute verschweigt.
4. **L2** und **L3** — je ein Hinweis beziehungsweise eine berichtigte Prämisse.

Nichts davon hält eine Auslieferung auf: kein Profil ist unbrauchbar, keine Zahl ist falsch,
und `die_eingebettete_fassung_besteht_ihre_eigene_pruefung` bleibt grün.

## Was ich nicht geprüft habe

- **Die Anzeige.** Ob die Zusammenfassung im Vorschaufenster so aussieht, wie
  `Zusammenfassung::als_text` sie setzt, ist an einer laufenden Anwendung zu sehen und nicht an
  dieser Datei.
- **Die Zeit.** Diese Durchsicht nennt keine Millisekunde. Gezählt sind Leseläufe und
  Öffnungen, wie in `shared/analyses/260825-2107-…`.
- **Der Weg zur Nutzerdatei.** Ob eine bestehende `readers.toml` die zwölf neuen Profile je
  erreicht, ist die Frage des Datensatzes
  `shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`,
  den ich nicht geöffnet habe.

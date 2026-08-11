# Portfolio

**Generated:** 260811-2223 (by playmaker session 260811-2223-playmaker-direct-dispatch)
**Domain bias:** code

Sechs Circles liegen unter `circles/`: keiner aktiv, zwei vorgesehen, vier beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Die Runde 4 ist am 260811-2210
geschlossen, und das Feld der vorgesehenen Circles ist damit von drei auf zwei geschrumpft.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen zwei
Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden. Beides
zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: die Runde 4 ist am
260811-2210 als beschränkter Abschluss geschlossen worden, und der Orchestrator löscht den Zeiger
bei dieser Umbenennung.

Der nächste Schritt liegt beim Nutzer. Er wählt über `/fusion:next`, welcher der beiden
vorgesehenen Circles aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260811-1304-statusleiste-mit-bereichsschaltern` — seine harte Vorbedingung ist
mit der Runde 4 weggefallen, seine tragende Stelle ist eine einzige Funktion ohne AppKit, und seine
offenen Fragen sind in einer Klärungsrunde mit dem Nutzer zu beantworten.

Vorab eine Einordnung, die beide Kandidaten betrifft und die Rangfolge nicht entscheidet. Alle vier
gefahrenen Runden sind beschränkt abgeschlossen (`_b_`), und beide vorgesehenen Circles hängen
ausschließlich an ihnen. Nach der Rangheuristik zählt allein ein kohärenter Abschluss (`_c_`) als
erfüllte Vorbedingung, also tragen beide dasselbe Kennzeichen, und es unterscheidet sie nicht. Der
Abschnitt `## Warnings` führt aus, warum das in diesem Projekt kein Zufall zweier Läufe ist,
sondern eine Eigenschaft, an der die Heuristik ihre Trennschärfe verliert. Die Rangfolge unten
entsteht deshalb aus den übrigen Signalen.

### Rang 1: `260811-1304-statusleiste-mit-bereichsschaltern`

**KRK trägt eine Statusleiste mit Schaltern für die fünf Bereiche.** Angelegt am 260811-1304,
Domain `code`.

Eine Leiste am unteren Fensterrand führt für jeden der fünf Bereiche der Fensterzeile einen
Schalter, zeigt an, ob sein Bereich steht, und schaltet ihn per Maus oder Tastatur um. Jede
Änderung der Sichtbarkeit teilt die Fensterzeile proportional zur zuletzt sichtbaren Aufteilung neu
auf.

Der Ausschlag für Rang 1 ist der Wegfall einer harten Vorbedingung. In den Läufen vom 260811-1326
und 260811-1415 stand dieser Circle auf Rang 2, und der Grund war der vom Nutzer am 260811-1240
gemeldete Rückfall der Vorschaubreite: eine proportionale Neuaufteilung auf einer Grundlage, die
die Ziehbewegung des Nutzers nicht hält, verteilt die falschen Anteile. Der Defekt ist am
260811-2130 in der Runde 4 gemessen und behoben worden, Commit `1ea5a3d`, und die Messung hat die
erste der beiden im Defektdatensatz genannten Bruchstellen bestätigt: `kommando_ausfuehren` rief
`aufteilung_nachziehen()` nach jedem Befehl, bevor jemand die gezogene Breite nachmaß. Die zweite
Bruchstelle trifft nicht zu, C7 der Runde 1, die Zusage über gesicherte Breiten und Sichtbarkeit,
war also nie gebrochen.

Das zweite Signal ist die Prüfbarkeit ohne Vordergrund. Die tragende Stelle dieser Runde ist eine
einzige Funktion, `bereichsbreiten(verfuegbar, breiten, sichtbar)`
(`crates/krk-ui/src/fenstermodell.rs:609`). Sie ist reines Rust ohne AppKit und damit ohne Fenster
prüfbar; `aufteilung.rs` setzt nur um, was dort herauskommt. Eine Runde, die die Breitenregel neu
fasst, kann ihren Kern an dieser Funktion und ihren Proben abnehmen, statt auf einen Abnahmelauf zu
warten, der KRK im Vordergrund verlangt. Bei einem Projekt, dessen vier bisherige Runden sämtlich
an genau dieser Abnahme hängen geblieben sind, ist das ein Signal von Gewicht.

Gegen die Empfehlung spricht der Zuschnitt, und er ist seit dem Anlegen des Circles gewachsen.
Sechs Entscheidungsdatensätze in seinem `decisions/` tragen `_o_`, und der erste von ihnen
bestimmt den Umfang der ganzen Runde: `260811-1305_*_was-heisst-proportional-zur-letzten-aufteilung.md`
fragt, ob die eine Breitenregel neu geschrieben wird und was aus der Vorrangordnung vom 260808
wird, nach der die Lesezeichenleiste vor dem Editor nicht weicht. Dazu kommt seit dem 260811-1732
ein Nachtrag des Nutzers, der kein Defekt ist, sondern eine Erweiterung des Zuschnitts:
`issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md` verlangt
neben den fünf Bereichsschaltern drei Schalter für die Spalten Größe, Datum und Typ. Die beiden
Sorten verhalten sich verschieden. Ein Bereichsschalter ändert die Aufteilung des Fensters und löst
die proportionale Neuverteilung aus, um die es der Directive geht; ein Spaltenschalter ändert den
Inhalt beider Dateifenster und die Aufteilung gar nicht. Der Nachtrag wirft vier eigene Fragen auf,
darunter die einzige mit Folgen über das Verbergen hinaus: was mit der Sortierung geschieht, wenn
die Spalte weggeschaltet wird, nach der sortiert ist. Ob beide Sorten in eine Runde gehören, ist
selbst eine Frage und gehört an den Anfang der Klärungsrunde.

- **Abhängigkeiten:** `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`, über C7, C1 und C8) und
  `260807-2116-eingebauter-editor-mit-textmarken` (`_b_`, über C1 und die fünf Bereiche). Beide
  beschränkt, keine kohärent; Kennzeichen gesetzt, inhaltlich beide leicht. Der Web-Betrachter
  bindet ihn ausdrücklich nicht.
- **Offene Entscheidungen im eigenen Speicher:** sechs, alle `_o_`, alle Zuschnittfragen mit
  benannten Möglichkeiten. Eine siebte steht seit dem 260811 auf beantwortet.
- **Offener Defekt im eigenen Speicher:** einer, und er ist keiner. Der Nachtrag vom 260811-1732
  vergrößert den Zuschnitt und liegt dort, damit er bei der Aktivierung gefunden wird.
- **Angefügt in diesem Lauf:** `## Parent grounding stale` und `## Activation proposal`.
- **Zu beachten:** `MINDESTGROESSE` (`crates/krk-ui/src/appkit/fenster.rs`) steht auf 780 Punkten
  und deckt die vier Bereiche der Runde 1; der Fünfersatz mit dem Editor summiert sich auf 920.
  Zwischen 780 und 920 Punkten Fensterbreite wird der Editor unter sein Mindestmaß gedrückt. Der
  Beifund stammt aus der Behebung vom 260811-2130 und trifft die sechste offene Frage dieses
  Circles, deren Datensatz die Zahl 920 noch nicht kennt.

### Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**KRK zeigt Web-Seiten in einem eigenen Betrachter.** Angelegt am 260804-0933, Domain `code`.

Eine Web-Adresse erscheint in einem gewöhnlichen Tab des Vorschaufensters statt im Systembrowser.
Bedient wird der Betrachter über die Tastatur, mit Sprungmarken auf jedem sichtbaren Link. Kein
Verlauf, kein dauerhaftes Adressfeld, kein Herunterladen.

Rang 2 trotz des besseren Zählwerts bei den offenen Entscheidungen. Er zitiert genau einen offenen
Datensatz, und der ist der Grund: die Verfügbarkeitsprüfung für macOS-26-Schnittstellen in `objc2`
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
ist eine ungemessene technische Frage. Daneben hält der Circle selbst fest, dass auch das Mittel
der Darstellung von Web-Inhalt offen ist und „in eine eigene Untersuchung vor dem Plan" gehört. Vor
dieser Runde steht damit eine Untersuchung, während vor der Runde auf Rang 1 eine Klärungsrunde mit
dem Nutzer steht. Der Unterschied im Aufwand ist erheblich, und der Zählwert der offenen
Entscheidungen bildet ihn nicht ab.

Das zweite Argument ist das Alter der Grundlage. Sie stammt vom 260804 und kennt weder die
Editor-Runde noch die Belegungs-Runde noch die Runde 4. Ihr Abschnitt `## Dependencies` nennt
`260802-0842-krk-mac-dateimanager-editor-git` „den aktiven Circle"; jene Runde ist seit dem
260807-1035 geschlossen. Das Vorschaufenster, in dem der Betrachter leben soll, hat seither einen
fünften Bereich neben sich bekommen, teilt seine Fläche zeitlich mit dem Editor und trägt seit dem
260811 eine Zeilennummernspalte. Wer diesen Circle aktiviert, erhebt seine Grundlage neu.

- **Abhängigkeiten:** `260802-0842-krk-mac-dateimanager-editor-git` (`_b_`). Er erweitert dessen
  Grenze und ist keine spätere Runde davon: der Datensatz jener Runde schließt einen integrierten
  Browser ausdrücklich aus, und dieser Circle hebt den Ausschluss für den Betrachter auf.
- **Offene Entscheidungen im eigenen Speicher:** keine. Sein `decisions/` ist leer; die drei offenen
  Fragen seiner Grundlage stehen als Prosa im Datensatz und nicht als Datensätze.
- **Angefügt in diesem Lauf:** nichts.
- **Zu beachten:** seine dritte offene Frage, ob der Betrachter eine eigene Zeitzusage bekommt,
  leitet eine elfte Zahl aus den zehn Zeitzusagen der Runde 1 ab. Zwei der naheliegenden
  Bezugsgrößen, L5 für den Tabwechsel und L7 für die Vorschau einer Textdatei, gehören zu dem Teil,
  dessen Beleg die Runde 1 offen gelassen hat. Der Abschnitt `## Parent grounding stale` in seinem
  Datensatz vom 260807-1042 führt das aus.

## Recently closed (_c_ / _b_)

Vier geschlossene Circles, keiner davon kohärent. Alle vier sind aus demselben Grund beschränkt:
der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit, die kein Agent leisten kann
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).

| Circle | Marker | Geschlossen | Abschlussnotiz in einem Satz |
|---|---|---|---|
| `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` | `_b_` | 260811-2210 | Vier Tastenbefehle laufen über die vorhandene Kommando-Maschinerie, die Zwischenablage ist zum ersten Mal auch Ziel, alle fünf Planschritte tragen `[DONE]` und `make check` ist grün mit 795 Proben; die 62 Abnahmekriterien stehen offen, davon trägt der Baum 23 schon heute. |
| `260809-2040-tastenbelegung-als-markdown-in-downloads` | `_b_` | 260811-1415 | KRK schreibt die Belegung als Markdown in den Downloads-Ordner; die drei gefahrenen Planschritte tragen `[DONE]`, der Abnahmeschritt S4 ist vom Nutzer gestrichen, und damit stehen alle 41 Abnahmekriterien offen. |
| `260807-2116-eingebauter-editor-mit-textmarken` | `_b_` | 260810-1445 | Der Editor steht als fünfter Fokusbereich mit Roh- und Formatansicht, Zeilensprung, Suchen, Ersetzen und Textmarken; alle 48 Planschritte tragen `[DONE]`, der Abnahmelauf über 110 Kriterien steht aus. |
| `260802-0842-krk-mac-dateimanager-editor-git` | `_b_` | 260807-1035 | Das Navigator-Gerüst der Runde 1 steht mit allen 38 Planschritten auf `[DONE]`; sieben der zehn Zeitzusagen stehen auf einer Messreihe, die drei spätere Commits haben altern lassen. |

**Die vier Abschluss-Artefakte in je einem Satz**, weil sie die spätere Arbeit binden:

- Runde 4: der Abgleich hat zwölf behauptete Behebungen einzeln gegen den Baum gelesen und eine
  gefunden, die nur zur Hälfte gelaufen war; ein Abnahmekriterium (C5, Konfliktfreiheit „auch gegen
  ein Menükürzel") verspricht mehr, als `Belegung::konflikte` beantworten kann, weil der Vergleich
  seit dem 260805 nur innerhalb desselben `gehalten_von` läuft.
- Runde 3: eine Zusicherung stand dreimal in derselben Sitzung im Text stärker da als im Code, und
  jedes Mal hat erst die Durchsicht sie zurückgezogen. Der Spec hat für diese Fehlerform eine
  Gewohnheit, `inference:` kennzeichnen und die Prüfung zum Kriterium machen, aber keinen
  Mechanismus.
- Runde 2: ein stehendes Blatt hält Tastenbefehle beim Anwendungsdelegierten an und nicht über den
  Fokusvorbehalt; die andere Lesart hat einen Fehlbefund erzeugt.
- Runde 1: eine Messreihe altert an jedem Commit, der einen gemessenen Pfad berührt, und sie sagt es
  nicht selbst.

## Archived (_s_ / _d_)

**(keiner)** — kein Circle-Datensatz trägt `_s_` überholt oder `_d_` zurückgestellt.

Dieser Abschnitt führt lebende Circle-Datensätze unter `circles/` mit diesen beiden Markern. Er hat
nichts mit dem Speicher `archive/` zu tun; dessen Inhalt erscheint im Portfolio nicht.

## Warnings

Keine Zeigerlage und kein Zyklus. Sieben Befunde stehen an. Zwei davon sind in diesem Lauf neu, und
einer davon berichtigt eine Behauptung, die zwei frühere Läufe dieser Datei aufgestellt haben.

- **Kein `STALE-POINTER`, kein `POINTER-MISMATCH`, kein `MULTIPLE-ACTIVE`, kein `MISSING-POINTER`.**
  `fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt `_t_`.
  Beides zusammen ist der reguläre Zustand nach einem Abschluss.

- **Kein `dependency-cycle-detected`.** Der gerichtete Graph über die beiden nicht-terminalen
  Circles hat keine Kante zwischen zwei nicht-terminalen Knoten: alle Kanten enden auf beschränkt
  abgeschlossenen Runden. Die einzige Nennung eines nicht-terminalen Circles durch einen anderen
  steht im Datensatz der Statusleiste und ist ausdrücklich als nicht bindend ausgewiesen. Ein
  Zyklus ist damit ausgeschlossen, und kein Abschnitt `## Dependency warning` wurde angefügt.

- **Warnung 1, neu: die Rangheuristik verliert in diesem Projekt ihre Trennschärfe bei den
  Vorbedingungen.** Nach der Heuristik zählt allein ein kohärenter Abschluss (`_c_`) als erfüllte
  Vorbedingung, und jede Abhängigkeit auf `_b_`, `_a_`, `_t_`, `_s_` oder `_d_` setzt ein
  Kennzeichen. In diesem Projekt sind vier von vier gefahrenen Runden `_b_`, und der Grund ist
  jedes Mal derselbe: der Abnahmelauf verlangt KRK im Vordergrund, kein Agent kann ihn fahren, und
  das gilt unabhängig davon, was die Runde gebaut hat. Kein Circle dieses Projekts wird auf
  absehbare Zeit eine Abhängigkeit tragen, die `_c_` ist. Das Kennzeichen steht damit bei jedem
  Kandidaten, unterscheidet keine zwei mehr und sagt über die Reife eines Kandidaten nichts aus.
  Vier Läufe dieser Datei haben es gesetzt und dreimal im selben Atemzug entkräftet.

  Was daran unbrauchbar geworden ist, ist eng begrenzt: die Unterscheidung zwischen `_b_` und `_c_`.
  Alles andere trägt weiter. Eine Abhängigkeit auf einen vorgesehenen (`_a_`), aktiven (`_t_`) oder
  überholten (`_s_`) Circle bleibt eine echte unerfüllte Vorbedingung, ebenso eine auf einen
  Verzeichnisnamen, den es nicht gibt.

  Wir empfehlen, die Vorbedingung an der Beschränkung selbst zu prüfen und nicht am Marker: ein
  beschränkter Abschluss zählt als erfüllt, wenn seine `## Closure note` die Beschränkung benennt
  und diese Beschränkung kein Bauteil betrifft, das der Nachfolger erbt. Bei allen vier Runden
  dieses Projekts wäre die Antwort „erfüllt", weil die Beschränkung an der Abnahme hängt und nicht
  am Bau. Die Heuristik steht in der Anweisung des Playmakers und damit in der installierten Kopie
  des Plugins; aus diesem Projekt heraus ist sie nicht änderbar, und der Playmaker ändert sie auch
  nicht selbst. Der Befund steht hier zur Entscheidung des Nutzers.

- **Warnung 2, neu: zwei Warnungen des letzten Laufs zitieren Defektdatensätze, die es nicht gibt.**
  Der Lauf vom 260811-1415 schrieb, ein Befund sei „aufgenommen als
  `shared/issues/260811-0932_…die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md`,
  weiterhin offen", und verwies an anderer Stelle auf
  `shared/issues/260810-1730_…die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus….md`.
  Beide Dateien existieren im Baum nicht, weder im gemeinsamen Speicher noch in einem Circle noch
  in der Git-Historie. Die Ursache ist eine Grenze der eigenen Zuständigkeit: der Playmaker darf
  keine Defekte anlegen, und ein Lauf, der einen Befund als aufgenommen beschreibt, behauptet eine
  Handlung, die er nicht ausführen durfte. Beide Befunde bestehen in der Sache fort und stehen
  unten als Warnung 3 und Warnung 6, jetzt ohne Verweis auf einen Datensatz. Wer sie festhalten
  will, legt die Defekte selbst an.

- **Warnung 3: der Kopf des Datensatzes der Runde 3 widerspricht seinem Marker.**
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_b_circle.md` trägt im Kopf
  `**Status:** anticipated`, während der Dateiname `_b_` sagt. Der Datensatz hat seit dem 260809
  zwei Übergänge durchlaufen, vorgesehen auf aktiv und aktiv auf beschränkt abgeschlossen, und
  keiner der beiden hat das Kopffeld nachgezogen. Die Runden 1, 2 und 4 tragen an derselben Stelle
  korrekt `bounded` oder `bounded closure`, der Fehler ist also nicht systematisch. Kein
  Defektdatensatz führt ihn; siehe Warnung 2.

- **Warnung 4: der Aktivierungsvorschlag im Datensatz des Web-Betrachters ist überholt.** Der
  Abschnitt `## Activation proposal` vom 260807-1042 in
  `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md` bezeichnet diesen
  Circle als empfohlenen nächsten Kandidaten. Drei spätere Läufe haben ihn auf den letzten Rang
  gesetzt. Weil der Playmaker anfügt und nicht umschreibt, steht der alte Vorschlag ohne Widerspruch
  daneben; wer nur ihn liest, liest das Gegenteil der heutigen Empfehlung. Derselbe Datensatz
  beschreibt die Runde 1 an mehreren Stellen als aktiv, und seine Grundlage kennt weder die
  Editor-Runde noch die Belegungs-Runde noch die Runde 4.

- **Warnung 5: vier Stellen im Datensatz der Statusleiste sind mit der Runde 4 gealtert.** Der
  Abschnitt `## Parent grounding stale`, den dieser Lauf angefügt hat, führt sie einzeln auf. Kurz:
  die Directive sagt eine Behebung der Vorschaubreite zu, die am 260811-2130 in einer anderen Runde
  gefallen ist; die Zählung „sieben Fragen" trifft nicht mehr zu, offen sind sechs; die
  Fragentabelle führt den beantworteten Datensatz unter seinem damaligen Marker `_o_`; und der
  Abschnitt `## Dependencies` zitiert den behobenen Defekt ebenso. Der Playmaker berichtigt keine
  Zitate und keine Zählung. Wer den Circle aktiviert, zieht sie mit der neuen Grundlage nach.

- **Warnung 6: die Erzeugung dieser Datei setzt die Sternform nicht von selbst.** Die
  Portfolio-Vorlage in `rules/circle-records.md` schweigt zur Zitierform, und das Musterbeispiel in
  der Anweisung des Playmakers führt einen ausgeschriebenen Marker vor. Dieser Lauf hat die
  Sternform von Hand durchgehalten. Der Befund sitzt in der installierten Kopie des Plugins und ist
  aus diesem Projekt heraus nicht behebbar; kein Defektdatensatz führt ihn, siehe Warnung 2.

- **Warnung 7: die Spec-Dateien dreier Runden bleiben auf `_o_`, und das ist so gewollt.** Die
  Runden 2, 3 und 4 halten ihre Abnahmekriterien sämtlich auf `- [ ]`, weil der Abnahmelauf
  aussteht: 110 Kriterien in
  `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md`,
  41 in
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md`
  und 62 in
  `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1552_*_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md`.
  Die zugehörigen Pläne stehen korrekt auf `_c_`. Der Zustand ist kein Versehen, sondern der Grund
  der Beschränkung; er steht hier, damit ein späterer Lauf ihn nicht als Nachlässigkeit liest. Für
  die Runde 4 hat der Abgleich die 62 Kriterien sortiert: 23 trägt der Baum heute schon, 32 kann nur
  ein Mensch am laufenden Bündel sehen, 7 brauchen einen Prüfaufbau.

**Offene Defekte und Entscheidungen, zur Einordnung.** Der Playmaker legt keine Defekte an und
schließt keine. Der gemeinsame Speicher führt drei offene Defekte, und alle drei betreffen das
Werkzeug und nicht KRK: ein fehlendes Durchsichtsdokument, fehlende Aufgabenereignisse des
Orchestrators und fünf Commits hinter der letzten Turn-Grenze. Offene Entscheidungen liegen in vier
Speichern verteilt. Verbindlich ist in beiden Fällen der Dateibestand:

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'
```

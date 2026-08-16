# Der Dateifilter berücksichtigt den Inhalt, eingeschaltet über das Ankreuzfeld „Content"

---
**Domain:** code
**Status:** bounded
**Filed by:** orchestrator
**Active spec/plan:** shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md (Spec), circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_c_plan-inhaltsfilter-der-dateiliste.md (Plan, geschlossen)
**Active session history:** shared/history/260815-2047-orchestrator-session.md

---

## Directive

Der Filter der Dateiliste vergleicht heute Namen. Nach dieser Runde berücksichtigt
er zusätzlich den **Inhalt** der Dateien, und der Nutzer schaltet das über ein
zweites Ankreuzfeld „Content" in der Bereichsleiste ein, neben dem vorhandenen
„Deep".

Vier Festlegungen des Nutzers vom 260816 stehen fest und sind im Spec ausgeführt:

- **Gelesen wird ab einer Mindestlänge des Filtertexts, gestaffelt nach „Deep":**
  ab 5 Zeichen bei eingeschaltetem „Deep", ab 3 Zeichen ohne. Darunter filtert
  allein der Name, wie heute.
- **Nur Text**, und nur bis 1 MB, also bis zur Grenze, mit der die Vorschau Text
  liest. Eine Protokolldatei von 3 MB ist über ihren Inhalt nicht auffindbar; das
  ist angenommen und kein Defekt.
- **Nur Dateien, deren Name die Folge nicht schon trägt.** Ein Namenstreffer
  macht die Zeile sichtbar und erspart das Lesen. Die beiden Treffergründe
  schließen einander damit aus.
- **Keine elfte Zeitzusage.** An ihre Stelle treten Kriterien, die ohne
  Messstrecke prüfbar sind, wie in Runde 2 und Runde 10.

## Grounding snapshot

Gefüllt beim Übergang auf aktiv am 260816-1330. Der Stand:

- Der Spec ist geschrieben und liegt im gemeinsamen Speicher, weil beim Shaping
  kein Circle aktiv war: `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md`,
  sechs Fähigkeiten, 57 Abnahmekriterien.
- **Alle vier Entscheidungsdatensätze sind beantwortet.** Die zwei, die beim
  Schreiben des Specs offen waren, hat der Nutzer am 260816-1330 beantwortet: die
  Statuszeile bekommt einen Satzteil am Filterstand samt Hinweis auf die wegen
  ihrer Größe ungelesenen Dateien, und eine Zeile, die allein wegen ihres Inhalts
  dasteht, wird abgesetzt dargestellt. Beide Antworten hinterlassen je eine
  Bauentscheidung, die in den Plan gehört und keine Nutzerfrage mehr ist: wie der
  Satz im schmalen Fenster kürzt, und welche Aussage die Zelle schreibt, wenn ein
  Inhaltstreffer zugleich markiert ist.
- Der Filter der Runde 10 trägt zwei Regeln, jede genau einmal und mit je zwei
  Ruferrn, gehalten von einer Zählprobe. Der Inhaltsvergleich wird der dritte
  Rufer von `traegt_die_folge`, und die Zählprobe ist bewusst nachzuziehen.
- Die zehn Zeitzusagen aus C8 decken das Tippen nicht: L1 misst zwanzig
  Pfeil-ab-Ereignisse, und die Messstrecke läuft kopflos.
- Die Prüfordner der Messstrecke sind dünnbesetzt, je Datei 512 echte Bytes und
  der Rest ein Loch. Eine Inhaltsmessung darauf wäre wertlos; das ist der
  Gegenstand, den diese Runde für eine spätere Messrunde benennt.

## Dependencies

Kein anderer Circle ist Voraussetzung. Zitiert und bindend sind:

- `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/` — die Runde,
  die den Filter gebaut hat. Ihr Spec, ihr Plan und ihre Entscheide binden weiter,
  insbesondere `decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`.
- `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` — der Spec
  dieser Runde. Er bleibt im gemeinsamen Speicher, weil er dort entstanden ist;
  dieser Circle nimmt ihn über das Feld `Active spec/plan:` an, statt ihn zu
  verschieben (Herkunftsregel: Reichweite wird zitiert, nicht verlegt).
- `shared/decisions/260816-1310_a_welche-vorhandene-groessengrenze-gilt-fuer-den-inhaltsfilter.md`
- `shared/decisions/260816-1310_a_bekommt-der-inhaltsfilter-eine-eigene-messgroesse-oder-kriterien-ohne-messstrecke.md`
- `shared/decisions/260816-1310_o_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md`
- `shared/decisions/260816-1310_o_sieht-der-nutzer-ob-eine-zeile-wegen-des-namens-oder-wegen-des-inhalts-steht.md`

Alle vier Entscheidungsdatensätze sind vor diesem Circle entstanden und bleiben
aus demselben Grund im gemeinsamen Speicher.

## Turn log

- Aktiviert am 260816-1330 durch den Nutzer, aus der Sitzung
  `shared/history/260815-2047-orchestrator-session.md`. Anker `9f5ced5`.
- Turn 7 bis 13 (Sitzung 260815-2047): Commits `9f5ced5..721c6e4`, zwölf
  Planschritte plus zwei Nachträge. Coherence-Urteil: beschränkt, weil der
  Abnahmelauf am Bündel aussteht. Sitzungsprotokoll:
  `shared/history/260815-2047-orchestrator-session.md`.

## Closure note

**Beschränkter Abschluss am 260816-2030, und der Grund ist der übliche dieses
Projekts:** die Directive ist im Baum erreicht, aber nicht am laufenden Bündel
abgenommen. Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit;
kein Agent kann ihn fahren. Die Liste dafür liegt fertig unter
`messungen/260816-abnahme-inhaltsfilter.md`: 28 Beobachtungen an vier Orten, je
mit Handgriff und erwartetem Ergebnis, dazu die Tafel aller 57 Kriterien und der
Kommandoblock, der den Prüfordner anlegt.

**Was gebaut ist.** Der Filter der Dateiliste berücksichtigt den Inhalt,
eingeschaltet über das zehnte Ankreuzfeld „Content" neben „Deep". Gelesen wird
ab 5 Zeichen bei eingeschaltetem „Deep" und ab 3 ohne, nur Text und nur bis
1 MB, und nur bei Dateien, deren Name die Folge nicht schon trägt. Eine Zeile,
die allein wegen ihres Inhalts steht, wird gedämpft; der Filterstand nennt den
laufenden Lesevorgang und die Zahl der wegen ihrer Größe ungelesenen Dateien.

**Die vier Zusagen, auf die es ankam, sind von der Durchsicht am Baum
nachgelesen und halten:** kein Weg öffnet eine Datei, deren Name die Folge
trägt; unterhalb der Schwelle und über 1 MB wird nicht gelesen; der Durchlauf
hält einen Verzeichnis- und höchstens einen Dateideskriptor; die Abbruchprüfung
steht vor jeder Einheit, die dauern kann, ohne Vorbeiweg.

**Keine elfte Zeitzusage.** An ihre Stelle treten Kriterien ohne Messstrecke,
wie in Runde 2 und Runde 10. Der Inhaltsdurchlauf ist als Gegenstand für eine
spätere Messrunde benannt, zusammen mit dem Befund, dass die vorhandenen
Prüfordner dünnbesetzt sind und für eine Inhaltsmessung nicht taugen.

**Sieben Entscheidungsdatensätze**, alle beantwortet, sechs davon umgesetzt.
Der Bruch mit einer Zusage der Runde 10 — ein Tabwechsel beendet jetzt jeden
Durchlauf — ist als Nutzerentscheid festgehalten, samt dem angenommenen Preis,
und die Begründung jener Runde ist im Code ersetzt statt gelöscht.

**Was offen bleibt: sechs Befunde im Circle**, keiner kritisch, jeder mit
Schwere, Fundstelle und Weg:

| Datensatz | Gegenstand |
|---|---|
| `260816-1359` | die Probe gegen Zeitmessung erreicht zwei Dateien des Filterwegs nicht |
| `260816-1710` | der Rückwechsel auf einen Tab setzt seinen beendeten Durchlauf nicht fort |
| `260816-1932` | ein Deskriptormangel beendet den Durchlauf still, und der Lesehinweis verschwindet |
| `260816-1934` | sechs Prosastellen beschreiben den Stand vor dieser Runde |
| `260816-1935` | CLAUDE.md nennt zwei Filterregeln und eine Hülle in `krk-ui`; beides ist abgelöst |
| `260816-2020` | zwei Abnahmekriterien sind keinem Planschritt zugewiesen, halten aber |

Drei weitere Befunde derselben Durchsicht sind vor dem Abschluss abgetragen
worden, und zwar an ihrer gemeinsamen Wurzel statt einzeln (`721c6e4`): dieselbe
Frage stand an drei Stellen in drei Fassungen, und eine Antwort wurde ohne ihre
Frage aufbewahrt. Der Prüfschritt liefert seitdem einen Zeilengrund, den die
drei Leser lesen, statt die Frage neu zu stellen.

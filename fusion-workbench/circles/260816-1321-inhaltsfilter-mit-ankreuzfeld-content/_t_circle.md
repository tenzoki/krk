# Der Dateifilter berücksichtigt den Inhalt, eingeschaltet über das Ankreuzfeld „Content"

---
**Domain:** code
**Status:** active
**Filed by:** orchestrator
**Active spec/plan:** shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md
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

## Closure note

(offen)

# Der Dateifilter berücksichtigt den Inhalt, eingeschaltet über das Ankreuzfeld „Content"

---
**Domain:** code
**Status:** anticipated
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

Wird beim Übergang auf aktiv gefüllt. Der Stand beim Anlegen:

- Der Spec ist geschrieben und liegt im gemeinsamen Speicher, weil beim Shaping
  kein Circle aktiv war: `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md`,
  sechs Fähigkeiten, 57 Abnahmekriterien.
- Zwei Fragen sind offen und tragen im Spec eine benannte Vorbelegung, halten die
  Planung also nicht auf. Zwei weitere sind beantwortet.
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

(noch keine Runde gefahren)

## Closure note

(offen)

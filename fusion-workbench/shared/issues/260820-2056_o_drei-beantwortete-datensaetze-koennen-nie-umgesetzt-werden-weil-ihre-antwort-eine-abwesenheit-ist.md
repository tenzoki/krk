Drei beantwortete Entscheidungsdatensätze können nie auf „umgesetzt" wandern, weil ihre Antwort eine Abwesenheit ist

---

Drei Datensätze tragen `_a_` („beantwortet, noch nicht in Code umgesetzt"), und an keinem von ihnen
steht etwas aus. Ihre Antwort lautet jeweils „es wird nichts gebaut", und eine Abwesenheit hat keine
Fundstelle, die die Zeile `Implemented:` zitieren könnte. Sie bleiben deshalb auf `_a_` stehen,
solange die Vokabel keinen Ausgang für diesen Fall hat — und jede Zählung aktiver Grundlage über
`_o_` + `_a_` meldet sie als offen, obwohl niemand an ihnen arbeitet und niemand arbeiten wird.

---

**Schwere:** gering für den Baum, mittel für die Verlässlichkeit der Grundlagenzählung. Dieselbe
Gestalt hat `shared/issues/260820-1119_*_…` für den Defektspeicher beschrieben: ein Bestand, der als
Arbeitsvorrat gelesen wird und Posten anbietet, die keine sind.
**Gefunden von:** reconciler, Abgleich `shared/history/260820-2056-reconciliation.md`
**Domain:** code

## Die drei, am Baumstand `f5300f4` gemessen

| Datensatz | Antwort | Was ein `Implemented:` zitieren müsste |
|---|---|---|
| `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md` | nein, die Mindestbreite bleibt | `crates/krk-ui/src/fenstermodell.rs:213` trägt unverändert `160.0`. Eine Zeile, die **nicht** geändert wurde. |
| `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_a_wie-kommt-der-nutzer-von-einem-tiefen-treffer-in-dessen-ordner.md` | die Frage löst sich mit der Bauform der tiefen Ansicht auf | `angezeigtedatei::welche` (`crates/krk-ui/src/angezeigtedatei.rs:44-58`) hat unverändert zwei Quellen und keine dritte bekommen. |
| `shared/decisions/260816-1310_a_bekommt-der-inhaltsfilter-eine-eigene-messgroesse-oder-kriterien-ohne-messstrecke.md` | keine elfte Zeitzusage | `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs \| sort -u` liefert L1 bis L10. Eine Zahl, die **nicht** dazugekommen ist. |

**Der erste trägt die Feststellung schon seit dem 260812-2253** in seinem eigenen Rumpf, gesetzt von
einem früheren Abgleich, samt der Begründung, warum `_i_` dort die Suche nach aktiver Grundlage
bräche. Sie ist seither zweimal unabhängig wieder gefunden worden, ohne dass jemand sie als eigenen
Befund abgelegt hätte. Deshalb steht sie jetzt hier.

## Warum das keine Nachlässigkeit eines Abgleichs ist

`rules/fusion-workbench-conventions.md`, Abschnitt `## State Markers — decisions`, verlangt für
`_i_` ausdrücklich `Implemented: <commit hash> or <path>:<line> — <one-line summary>`. Für eine
Antwort, deren ganzer Inhalt „wir ändern nichts" ist, gibt es diesen Zeiger nicht. Die Vokabel
kennt daneben `_d_` (der Nutzer schiebt hinaus), `_s_` (eine spätere Entscheidung überholt) und die
Zeile `Retired:` (der Gegenstand ist entfallen) — keiner der drei trifft zu: nichts ist
hinausgeschoben, nichts überholt, und der Gegenstand besteht fort.

**Es ist auch kein Fehler der Datensätze.** Eine Frage mit dem Ausgang „nichts bauen" ist ein
richtiger Ausgang und in diesem Projekt ein häufiger; er ist bei allen dreien mit Kosten
gegengerechnet worden.

## Was zu entscheiden wäre

Der Befund ist der Sache nach eine Frage und kein Defekt, und die Frage richtet sich an fusion und
nicht an dieses Projekt: **braucht die Entscheidungsvokabel einen Ausgang für „beantwortet mit
nichts zu tun"?** Drei Zuschnitte liegen nahe, und dieser Datensatz wählt keinen:

1. Eine Zeile `Nothing to implement:` neben `Implemented:`, ohne Umbenennung. Billig, ändert keine
   Zählung, und die drei blieben weiter als offene Grundlage gezählt.
2. `_i_` zulassen mit einem Beleg, der eine **unveränderte** Stelle zitiert. Die Zählung stimmte
   danach; der Preis ist, dass `_i_` nicht mehr „hier ist der Commit" heißt.
3. Ein eigener Marker. Am saubersten und am teuersten: jede Zählung, jeder Glob und jede
   Vokabeltafel im Regelkorpus zieht nach.

Dieser Datensatz liegt im Defektspeicher, weil er als Messung eines Bestands begonnen hat. Nach der
Regel in `rules/fusion-workbench-conventions.md`, `## Issues vs Decisions`, gehört er seiner Art nach
in den Entscheidungsspeicher, sobald jemand ihn aufnimmt — verschieben kann ihn nur der Nutzer von
Hand, mit `mv` und einer Umstellung des Markers auf die dortige Vokabel.

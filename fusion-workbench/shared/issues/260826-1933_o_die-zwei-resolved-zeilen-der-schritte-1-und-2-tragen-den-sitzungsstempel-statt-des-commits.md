Die zwei `Resolved:`-Zeilen der Schritte 1 und 2 tragen den Sitzungsstempel statt des Commits

---

Der Plan `260826-1811` verlangt in „Where this Circle stops“: „Jeder der fuenf Datensaetze traegt `Resolved:` mit dem Commit.“ Die zwei geschlossenen Datensaetze der Runde 1 tragen stattdessen den Zeitstempel des Sitzungseintrags des Coders; der Commit, der sie geschlossen hat, steht nirgends im Datensatz.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `shared/issues/260826-1221_c_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-loescht-die-quelle-trotzdem.md` (letzte Zeile), `shared/issues/260826-1221_c_der-schwungleser-oeffnet-mit-file-open-und-haengt-an-einer-benannten-roehre-fuer-immer.md` (letzte Zeile)
**Domain:** code

## Was dasteht

`Resolved: 260826-1900 — ueber_datentraeger merkt den Zaehlstand …` und `Resolved: 260826-1930 — Schwungleser::oeffnen nimmt die Huelle …`. Die Commits sind `36e54b4` und `9c02863`; beide nennen den Datensatz in ihrer `Source:`-Zeile, der Datensatz nennt sie nicht.

Der Grund ist die Reihenfolge: die Schliessung landet im selben Commit wie die Behebung, und der Hash ist beim Schreiben der Zeile noch nicht vergeben. Die Konvention verlangt fuer `Resolved:` nur eine Beschreibung; der Plan verlangt mehr, und sein Schlusskriterium ist damit fuer zwei von fuenf schon verfehlt.

## Was zu tun waere

Entweder die zwei Zeilen um den Hash ergaenzen (`Revised by:` ist dafuer nicht die Form, der Inhalt ist nicht widerrufen, nur unvollstaendig), oder das Schlusskriterium des Plans auf „mit dem Sitzungseintrag, der den Commit nennt“ abschwaechen. Fuer die Schritte 3 bis 6 dieselbe Entscheidung vorab.

Also seen: 260826-2158 by coderev — die Schritte 3, 4 und 6 setzen den Befund fort: `260826-1302_c_…` traegt statt eines Commits einen Dateipfad plus Sitzungseintrag, `260826-1223_c_…` den Stempel `260826-2135` und `260826-1301_c_…` den Stempel `260826-2140`; die zugehoerigen Commits sind `17e5e4e`, `9a4e495` und `960900d`. Damit trifft es fuenf von fuenf Datensaetzen des Plans.

---
Stand 260908-1539: **der Befund besteht unveraendert, und seine fuenf Ziele sind seit dem
260827 nicht mehr im lebenden Baum.**

Alle fuenf Datensaetze, die die `Also seen:`-Zeile oben aufzaehlt, liegen unter
`archive/260827-1534-safe-cleanup-tier-1/`; nachgesehen am 260908-1539. Ihre `Resolved:`-Zeilen
tragen weiter den Sitzungsstempel statt des Commits.

**Das aendert die Frage, die dieser Datensatz stellt, und beantwortet sie nicht.** Sein erster
Weg — „die zwei Zeilen um den Hash ergaenzen" — hiesse jetzt, in einen archivierten Datensatz
zu schreiben, und der zweite — „das Schlusskriterium des Plans abschwaechen" — in einen Plan,
der ebenfalls archiviert ist
(`260826-1811_*_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`). Ob ein
archivierter Datensatz ueberhaupt noch nachgefuehrt wird, sagt keine Regel dieses Projekts:
`CLAUDE.md` `## Bindende Grundlage` nennt sieben Speicher, deren Dateien ihren damaligen Stand
behalten, und `archive/` steht nicht darunter, weil es kein Datensatztyp ist, sondern ein Ort,
an den jeder Typ wandert.

**Was daneben unstrittig ist und der Datensatz nicht sagt.** Die Konvention selbst hat sich
bewegt: `rules/fusion-workbench-conventions.md` `## Inline State Tracking` sagt fuer eine
geschlossene Defektakte ausdruecklich „Leave the `Resolved:` note itself unedited", und
`Revised by:` ist die eine vorgesehene Zutat — die aber, wie dieser Datensatz richtig sagt,
den falschen Inhalt traegt, denn nichts ist widerrufen. Eine Form fuer „nachgetragen, was beim
Schreiben noch nicht vergeben war" kennt die Konvention nicht.

Er bleibt deshalb offen und wartet auf eine Nutzerentscheidung; geraten wird hier nichts.

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

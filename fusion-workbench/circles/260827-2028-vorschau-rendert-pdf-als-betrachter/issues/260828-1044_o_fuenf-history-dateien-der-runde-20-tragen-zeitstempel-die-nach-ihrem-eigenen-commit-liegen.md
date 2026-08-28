Fünf History-Dateien der Runde 20 tragen Zeitstempel, die nach ihrem eigenen Commit liegen

---

Der Commit `03af590` ist um 09:06 am 260828 eingetragen und enthält unter `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/history/` die Dateien `260828-1005-coder-schritt-2-…`, `260828-1120-coder-schritt-6-…`, `260828-1150-coder-schritt-9-…`, `260828-1210-ontocoder-schritt-3-…` und `260828-1230-coder-schritt-3b-…`. Ihre Namen und Kopfzeilen (`# Coder-Sitzung — 260828-1005`, `**Datum:** 260828-1230`) nennen Uhrzeiten, die bei der Eintragung noch nicht erreicht waren (Abgleich um 10:44 am selben Tag; `git log --format='%h %ad' --date=format:'%H:%M' 03af590`). Der Turn log des Circle-Datensatzes gibt den Turn mit „00:35–09:45" an. Die Zeitstempel sind also keine `date +%y%m%d-%H%M`-Werte, und wer die Reihenfolge der Schritte aus den Dateinamen liest, liest sie falsch (Schritt 2 erscheint nach Schritt 7).

---

**Filed by:** reconciler, Kai Stalmann <kai@qantr.com>
**Domain:** code
**Betroffen:** fünf Dateinamen und Kopfzeilen unter `history/` dieses Circles; kein Code

Wirkung: allein die Lesbarkeit der Sitzungsspur. Die Inhalte stimmen mit den Commits überein (jeder Schritt findet sich in `1df8b8d`, `2aee690`, `22b8442`, `5ff1ee4`). Nach der Ortsregel behalten Aufzeichnungen ihren Stand; ob die Namen umbenannt werden oder ein Vermerk genügt, entscheidet der Nutzer. Ursache vermutlich: parallel dispatchte Executor haben ihren Stempel geschätzt statt `date` zu rufen.

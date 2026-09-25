Der Circle-Datensatz führt Status "anticipated", der Dateiname trägt den Marker für aktiv

---

Die Datei heißt `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, der Marker `_t_` steht für einen aktiven Circle. Die Kopfzeile im selben Datensatz meldet dagegen `**Status:** anticipated`. Zusätzlich steht dort `**Active session history:** (none yet)`, obwohl das Verzeichnis `history/` sechs Sitzungsprotokolle enthält.

Der Marker im Dateinamen ist nach `rules/fusion-workbench-conventions.md` der maßgebliche Zustand; die Kopfzeile ist die menschenlesbare Wiederholung und läuft hier auseinander. Wer nur den Kopf liest, hält einen laufenden Circle für einen bloß vorgemerkten.

---

**Was zu tun ist:** `**Status:**` auf `active` setzen und `**Active session history:**` auf das jüngste Sitzungsprotokoll zeigen lassen. Beides gehört in dieselbe Bearbeitung wie der Defekt `260802-1417_o_directive-zeile-sagt-freie-funktionstasten-zu.md`, damit der Datensatz nur einmal angefasst wird.

**Aufgefallen bei:** dem Abgleich des Specs mit der Circle-Directive, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1409-shaper-fn-tasten-messung-und-cmd-kuerzel.md`. Der Shaper hat den Datensatz nicht angefasst, weil der Auftrag dieser Runde das untersagte.

---
Resolved: Am 260802-1423 auf den tatsächlichen Stand gezogen. `**Status:**` steht jetzt auf `active` und stimmt damit mit dem Marker `_t_` im Dateinamen überein. `**Active session history:**` zeigt auf `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1014-orchestrator-session.md`, die seit dem 260802-1014 laufende Orchestrator-Sitzung. Die übrigen sieben Protokolle im Verzeichnis `history/` sind Sitzungen einzelner Agenten innerhalb dieser Orchestrator-Sitzung und kommen für das Feld nicht in Frage; es benennt die Sitzung, die den Circle führt, nicht die jüngste Datei.

Die übrigen Kopffelder sind mitgeprüft. `**Active spec/plan:**` zeigt auf `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`; die Datei liegt dort und trägt den Marker für offen, das Feld stimmt. `**Domain:** code` und `**Filed by:** shaper (anticipated-circle mode)` sind unverändert richtig, das zweite Feld hält fest, wer den Circle angelegt hat, und nicht, wer ihn zuletzt bearbeitet hat.

Bei derselben Bearbeitung ist der bis dahin leere Abschnitt `## Turn log` mit einem Vorlauf-Eintrag für die sechs Commits c0682ff..f865fca gefüllt worden, die vor dem ersten Turn gelandet sind.

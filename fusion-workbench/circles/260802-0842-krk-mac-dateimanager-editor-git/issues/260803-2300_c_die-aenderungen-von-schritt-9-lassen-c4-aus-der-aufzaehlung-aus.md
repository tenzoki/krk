Die Änderungen von Schritt 9 zählen die Fähigkeiten auf, aus denen Funktionen in die Belegung gehören, und lassen C4 aus

---

Schritt 9 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` schrieb unter `Änderungen` vor, welche Funktionen in `resources/default-keymap.toml` gehören: "die sechs Norton-Funktionen mit je zwei Wegen aus der Tabelle in C3, die Papierkorb-Funktion auf `delete` und `cmd+delete`, F4 als Eintrag mit leerer Tastenliste und `reserviert_fuer = "editor"`, sowie alle Funktionen aus C1, C2, C5, C6 und C7."

C4 fehlt in dieser Aufzählung. Vier Funktionen dieser Fähigkeit tragen keine Norton-Taste und sind damit von keinem anderen Glied der Aufzählung abgedeckt: eine leere Datei anlegen, umbenennen, im Stapel umbenennen und eine laufende Operation abbrechen.

Das erste Abnahmekriterium von C2 verlangt für jede Funktion aus C1 bis C7 mindestens einen Tastenbefehl. Wörtlich befolgt hätte Schritt 9 dieses Kriterium verfehlt, und S15 oder S17 müssten sich vier Kombinationen ausdenken, womit die Belegung zwei Quellen hätte.

---

Herkunft: gefunden beim Nachzug der beiden Meldungen vom 260803-2045 durch den `planner`. Die Umsetzung von Schritt 9 war von der Lücke nicht betroffen: der `ontocoder` hat die vier C4-Funktionen aufgenommen und die Abweichung von der Aufzählung in `history/260803-2045-auslieferungsbelegung-als-datentabelle.md` offengelegt, ohne dafür einen Defektdatensatz anzulegen.

Warum es dennoch aufgeschrieben gehört: die Aufzählung in Schritt 9 ist die Prüfliste für jede spätere Durchsicht der Datei. Bliebe sie unvollständig, meldete die nächste Durchsicht die vier C4-Einträge als überzählig.

---
Resolved: Die Aufzählung in Schritt 9 nennt C4 jetzt mit. Ein Absatz darunter benennt die vier Funktionen und den Grund, aus dem sie dort hingehören. Die Datei `resources/default-keymap.toml` ist unverändert; sie war von Anfang an vollständig.

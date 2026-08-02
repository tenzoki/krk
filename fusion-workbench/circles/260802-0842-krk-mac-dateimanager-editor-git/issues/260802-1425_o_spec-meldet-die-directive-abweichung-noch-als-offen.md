Der Spec meldet die Abweichung der Directive-Zeile noch als offen

---

Der Spec `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` beschreibt im Abschnitt `## Abgleich mit der Circle-Directive` einen Zustand, der seit dem 260802-1423 nicht mehr gilt. Betroffen sind zwei Stellen:

1. Der Absatz "Seit dem 260802-1409 weicht die Directive-Zeile erneut ab, aus einem neuen Grund." Er nennt den Defekt `260802-1417_o_directive-zeile-sagt-freie-funktionstasten-zu.md` unter dem Marker für offen und schließt mit "Bis zur Korrektur gilt für die Tastenbelegung C3 dieses Specs". Die Korrektur ist erfolgt, der Defekt trägt jetzt den Marker für geschlossen.

2. Der davorstehende Absatz zitiert die Directive-Zeile im Wortlaut vom 260802-1127. Dieser Wortlaut ist ersetzt.

Zusätzlich nennt der Gatehinweis am Kopf des Specs die Abweichung als bestehend: "Die Directive-Zeile im Circle-Datensatz weicht seitdem erneut ab; der Defekt dazu ist gemeldet."

---

**Was zu tun ist:** die drei Stellen auf den Stand nach der Korrektur ziehen. Der neue Wortlaut der Directive-Zeile lautet: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die jede Funktion der Norton-Reihe auf zwei Wegen erreichbar macht, über die Funktionstasten F3 bis F8 und über ein Cmd-Kürzel." Der Abschnitt soll die Historie behalten, weil er sie bereits für die erste Korrektur vom 260802-1127 führt, und lediglich den Stand als abgeglichen ausweisen.

**Zu beachten:** Der Datensatz `_t_circle.md` trägt an zwei weiteren Stellen noch die überholte Fn-Fassung, siehe `260802-1425_o_circle-datensatz-wiederholt-die-ueberholte-fn-zusage-an-zwei-stellen.md`. Ein vollständiger Abgleich im Spec setzt voraus, dass jener Defekt zuerst behoben ist, sonst muss der Abschnitt zweimal angefasst werden.

**Warum der Shaper es nicht in derselben Bearbeitung behoben hat:** der Auftrag vom 260802-1423 untersagte jeden schreibenden Zugriff auf das Verzeichnis `planning/`.

**Aufgefallen bei:** der Behebung der beiden Defekte vom 260802-1417, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1423-shaper-circle-datensatz-korrektur.md`.

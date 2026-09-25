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

---
Resolved: Am 260802-1445 nachgezogen, nachdem der Circle-Datensatz vollständig auf dem Stand von C3 war; der Abschnitt musste dadurch nur einmal angefasst werden.

Der Abschnitt `## Abgleich mit der Circle-Directive` führt jetzt eine Einleitung, die ihn ausdrücklich als Geschichte des Abgleichs ausweist, und schließt mit einem datierten Stand. Die beiden gemeldeten Absätze stehen im Rückblick statt in der Gegenwart: der erste zitiert den Wortlaut vom 260802-1127 als überholte Fassung, der zweite hält die Abweichung vom 260802-1409 und ihre Behebung am 260802-1423 samt neuem Wortlaut fest und nennt den Defekt unter seinem geschlossenen Namen. Der neue Schlussabsatz nennt auch die beiden am 260802-1445 nachgezogenen Stellen im Circle-Datensatz und stellt fest, dass eine Abweichung derzeit nicht bekannt ist.

Der Gatehinweis am Kopf des Specs meldet die Abweichung nicht mehr als bestehend. An seine Stelle tritt ein Hinweis auf die eine Frage, die seit dem 260802-1428 wieder offen ist: was L4 mit "wiederhergestellten Tabs" meint.

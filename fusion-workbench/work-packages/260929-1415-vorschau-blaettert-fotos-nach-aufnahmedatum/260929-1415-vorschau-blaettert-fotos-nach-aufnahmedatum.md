# Die Vorschau blättert Fotos eines Jahres- oder Monatsordners nach Aufnahmedatum

---
**Domain:** code
**Status:** claimed
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260929-1415
**Mode:** autonomous
**Active spec/plan:** 260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md (Spec), 260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"Allgemeines Reader Profile so erweitern, dass ein Baum dieser Struktur (Fotos/<jahr>/<monat>/<bilder>) ein bestimmtes Scrollverhalten in der Vorschau realisiert: Wenn man innerhalb eines Jahresordners ist, soll erst das erste Bild des ersten Monats angezeigt werden, im Monatsordner das erste Bild dieses Monats. Mit CMD-Pfeil-{hoch/runter} soll durch die Bilder navigiert werden. Enter springt in beiden Fällen in den Ordner zu dem Bild." Antworten auf die Klärungsfragen: der ausgewählte Ordner liefert die Bilder; cmd+Pfeil blättert, solange eine Bildfolge angezeigt wird; Enter springt dann zum Bild; Reihenfolge nach Aufnahmedatum, Lesegrenzen dafür erweitert, Obergrenze 7.500 Fotos. Erreicht ist das, wenn die Kriterien des Spec am Baum gehalten sind; ausgeliefert wird zusammen mit dem Zurücksetzen-Befehl als 2.2.0 (Auftrag des Nutzers).

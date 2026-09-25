# F2 öffnet einen Dateilisten-Tab in ~/krkhome/ mit Notizen, Aufgaben und Geheimnissen statt des blockierenden Notizfensters

---
**Status:** claimed
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260926-0004
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

f2-enhancement das notizfenster wird komplett umgebaut und erweitert. das blockierende fenster entfällt. stattdessen öffne F2 einen neuen dateilisten tab in ~/krkhome/

beim ersten aufruf wird der ordner angelegt und in dem ordner werden ausserdem folgende Dateien nagelegt:

notes.txt enthält in strukturierter form gespeicherte Notizen <topic>  <note>, die der user editieren kann. braucht speziellen editor für tabellenartige einträge und gerenderte darstellung.

tasks.txt enthält in strukturierte Form gespeicherte Todos mit eine checkout  für done. user kann neue hinzufügen, verschieben löschen, checkout anklicken. braucht speziellen editor für operationen und gerendete darstellung.

.secrets.txt eine 'unsichtbare' datei, die hier dennoch immer gelistet ist. beim erzeugen wird eine 4-stellige pin abgefragt zur verschlüsselung. gleiche darstellung der einträge und gleicher editor wie notes. zum editieren muss die pin eingegeben werden. der preview der datei zeigt nichts oder nur einen hinweis an.

Eine leere `.secrets.txt` unter einer dritten Schreibweise des Heimordners geht über den Klartextweg

---

Jede Sperre für `.secrets.txt` fragt `ist_geheimnisdatei` (`crates/krk-ui/src/editormodell.rs`), und die erkennt den Heimordner an genau zwei Pfadformen (Entscheid `260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md`, Möglichkeit 1). Unter einer dritten Schreibweise (andere Groß- und Kleinschreibung auf einem Volume ohne Unterscheidung, `/System/Volumes/Data/…`, ein zweiter Verweis) öffnet eine `.secrets.txt` mit null Bytes als gewöhnlicher Text, und `cmd+s` schreibt Klartext. Das widerspricht der Zusage des Spec, dass Klartext der Geheimnisse nie auf die Platte kommt.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Befund aus `260926-1047-schlussdurchsicht-f2-krkhome.md`. Die Behebung ist eine Wahl an einem beantworteten Entscheid und liegt beim Nutzer; Vorschlag der Schlussdurchsicht: für Dateien mit dem Namen `.secrets.txt` eine zusätzliche genaue Prüfung (Gerät und Inode gegen `~/krkhome/.secrets.txt`) allein beim Öffnen und beim Sichern, ohne die Erkennung im Übrigen zu ändern.

**Abnahme**

Eine `.secrets.txt` im Heimordner, erreicht über eine dritte Schreibweise, öffnet im Editor nur über das PIN-Blatt und wird nie im Klartext gesichert; eine Probe hält es.

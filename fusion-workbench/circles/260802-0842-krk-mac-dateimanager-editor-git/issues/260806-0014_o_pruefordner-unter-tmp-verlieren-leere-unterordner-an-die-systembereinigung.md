Prüfordner unter /tmp verlieren ihre leeren Unterordner an die Systembereinigung

---

Die vier Prüfordner der Messstrecke lagen bis zum 260806 unter `/tmp`
(`krk-pruefordner-a`, `-b`, `-gross`, `-a-l6`, dazu das Kopierziel). In der
Nacht zum 260806 hat eine Systembereinigung dort sämtliche Unterordner aller
vier Bestände gelöscht: A verlor genau seine 214, B seine 200, der 100.000er
seine 2016 und der L6-Ordner seine 19 Unterordner, während alle Dateien und
Verknüpfungen stehen blieben. Ein frisch neu erzeugtes A und ein frisch neu
erzeugter 100.000er verloren ihre Unterordner binnen Minuten erneut; ein
laufender Fünf-Runden-Lauf brach deshalb korrekt ab ("Lauf 14 hat 99699
Einträge gelesen, Lauf 1 aber 100000. Die Läufe messen nicht dasselbe; die
Reihe wird verworfen.").

Geprüft am 260806 zwischen 0:00 und 0:10 mit `find -mindepth 1 -maxdepth 1
-type d | wc -l` vor und nach der Neuerzeugung.

`inference:` Der Mechanismus passt zu einer Bereinigung, die leere
Verzeichnisse mit altem Änderungszeitstempel entfernt: der Prüfordner-Erzeuger
datiert die Zeitstempel seiner Einträge absichtlich breit zurück (Sortierung
nach Datum braucht Vielfalt), die Unterordner sind leer, und genau die
Kombination leer plus alt fiel der Löschung zum Opfer, Dateien mit denselben
alten Stempeln aber nicht. Welcher Systemdienst läuft, ist nicht festgestellt;
`/var/log/daily.out` existiert auf macOS 15 nicht mehr.

**Behelf in S22:** Der Messplatz liegt jetzt unter
`~/Library/Caches/krk-messplatz/` (Prüfordner A, B, gross, das Kopierziel und
die vom Lauf erzeugten L6-Unterordner), auf demselben APFS-Datenträger wie
zuvor. Dort hat kein Bestand mehr Einträge verloren; beide Berichte vom
260805-2207 und 260805-2212 sind auf diesem Messplatz gefahren.

**Was zu tun bleibt (`coder`):** Das `Makefile` führt in den Zielen `fixture`,
`messen`, `alle` und `durchstich` weiterhin die `/tmp`-Pfade als Vorgabe
(`ORDNER_A := /tmp/krk-pruefordner-a` und die drei Geschwister, Zeilen
105-108). Die Vorgabe auf den neuen Messplatz umstellen oder einen anderen
beständigen Ort festlegen; `/tmp` ist als Ort für die Prüfordner ungeeignet,
weil jede Messreihe stillschweigend auf einem beschnittenen Bestand laufen
kann, sobald drei Tage oder eine nächtliche Bereinigung dazwischenliegen.
README beziehungsweise die Kommentarzeile im `Makefile` ("Die Pruefordner
liegen unter /tmp") ziehen mit.

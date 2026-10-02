# Die L9-Gegenmessung ist vor dem Eingriff abgebrochen

**Status:** Complete
**Agent:** coder
**Datum:** 260807-1810
**Auftrag:** dringende Einzelaufgabe, kein Circle aktiv
**Defekt:** `shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`

## Was der Auftrag verlangte

Den vierten Anlass der Lesezeichenprüfung aus `2fbab30` vorübergehend
stilllegen, bauen, und den Nutzer L9 erneut fahren lassen. Vorgeschaltet war
eine Abbruchbedingung: prüft die Sitzung keine oder fast keine Lesezeichen,
ist der Verdacht tot, und der Eingriff unterbleibt.

## Die Abbruchbedingung ist eingetreten

Die Prüfsitzung führt **null** Lesezeichen. Der Beleg steht im Defekt, Abschnitt
„Die Gegenmessung ist ausgefallen: die Vermutung ist widerlegt“, mit den
Fundstellen. In Kürze: die Lesezeichen wohnen in `bookmarks.toml` und nicht in
`session.toml`, die Messstrecke schreibt allein die Sitzung, und
`~/Library/Application Support/KRK/bookmarks.toml` gibt es auf diesem Gerät
nicht. Damit läuft `Leistenmodell::gueltigkeit_pruefen` über eine leere Liste.

## Was geändert wurde

Kein Quelltext. Geändert ist allein der Defekt: drei neue Abschnitte, dazu die
Zeile `Zuständig:`.

Nicht angefasst: `crates/`, `xtask/`, `Makefile`, Spec, Plan,
Entscheidungsdatensätze. Kein `make bundle`, weil es nichts Neues zu bauen gibt;
das Bündel aus `HEAD` ist dasselbe, an dem der Nutzer am 260807-1538 gemessen
hat. Nicht committet, wie beauftragt.

## Was nebenbei auffiel

Der Ausschluss im Defekt war zu eng: L9 unterscheidet von L1 nicht allein die
laufende Kopie, sondern zusätzlich die vollständige Markierung des Prüfordners
A mit 10.000 Einträgen. Der Defekt trägt das jetzt, samt einer nach diesem
Kriterium umsortierten Verdächtigenliste. Alles davon ist `inference:`, nichts
gemessen.

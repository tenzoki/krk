# Menü neu gliedern mit „Navigation“, krkhome auf secrets, tasks und quicknote, neuer Unterordner minutes/

## Directive

Die Menüleiste trägt die Titel „Leiste“, „Dateioperation“, „Dateiliste“ und ein neues Menü „Navigation“ mit den Navigationsbefehlen; `scratch.txt` heißt `quicknote.txt`; die Funktionen um `appointments.md` und `notes.txt` sind entfernt, sodass im Notizordner allein secrets, tasks und quicknote bleiben; der Notizordner hat einen Unterordner `minutes/`, in dem „Leere Datei anlegen“ den Namen immer als `YYMMDD-HHMM-<Name>.md` schreibt, und ein Befehl auf Cmd+M öffnet `minutes/` in einem Tab und springt sofort in dieses Namensblatt. Erreicht ist es, wenn `make check` grün ist und die Abnahmekriterien der Spezifikation am Baum gelesen halten.

## Die Worte des Nutzers

Am 261111, wörtlich:

> Änderung im Menu:
> - Kürze "Leiste und Fokus" -> "Leiste"
> - "Dateioperationen" -> "Dateioperation"
> - "Dateilisting" -> "Dateliste"
> - Lege neues Item an neben Dateiliste: "Navigation"
> - Bewege die ersten 8 Dateilisting Operationen -> Navigation, benso die anderen Navigationsbefehle: Pfad einegebn und springen, zur Zwischenablage springen etc
>
> Die Datei scratch.txt in quicknote.txt umbenennen.
>
> DIe apointment.md und die notes.txt Funktionen komplett entfernen.
> Nur secrets, tasks und quicknote bleiben in krkhome.
>
> krkhome bekommt aber einen neuen Unterordner: minutes/
> Dateien, die in diesem Ornder angelegt werden mit Option+Shiift+n erhalten ein Profix "YYMMDD-TTHHMM-" + <eingegebener Name> + ".md"
> Außerdem brauche ich einen Kurzbefhl, der den Ornder minutes/ in einem neuen Tab öffnet und sofort in den Datei-öffnen Dialog springt: am besten etwas mit M wie cmd+M.

Auf die vier Rückfragen der Spezifikation, wörtlich:

> 1a 3a 4:gemeint war:YYMMDD-HHMM-<..>.md
> 2:Control+Shift+N bleibt der eine Dateianlagebefehl und legt nur in minutes/ die Datei immer mit Prefix an. Zusätzlich kommt ein neues CMD+MBefehl dazu, der wie bei 3a in minutes/ springt und sofort Datei öffnen mit Prefix macht.

## Modus

Der Nutzer am 261111, wörtlich:

> auotnom durhcführen ab hier.

# Zwei Tastenbefehle der Dateiliste öffnen die Datei unter dem Cursor im Terminal, mit vim und mit sudo -e

## Directive

In der Dateiliste öffnet `opt+cmd+e` die Datei unter dem Cursor im Terminal mit `vim <datei>`, und `shift+opt+cmd+e` öffnet sie mit `sudo -e <datei>`. Das Passwort fragt sudo selbst im Terminal ab; KRK hält und speichert es nicht. `opt+cmd+e` wird mit „Editor schließen“ geteilt, das allein im Editor wirkt. Erreicht ist das Ziel, wenn beide Tasten an einer lokalen Datei das Terminal mit dem jeweiligen Befehl öffnen und an einem fernen Ort oder in einem Archiv ausgegraut sind.

Wortlaut des Nutzers (261029): „ok, 2 tasten, bei etwas mit +e, also `vim <datei>` und `sudo -e`“; Tastenwahl „1 1“: `opt+cmd+e` → `vim`, `shift+opt+cmd+e` → `sudo -e`, als Arbeitspaket anlegen und übernehmen.

**Geändert am 261029, vor der Freigabe des Plans:** Ein Befehl statt zwei. Der Nutzer: „eigentlich müsste krk ja feststellen können ob sudo nötig ist und selbst den richtigen weg nehmen, dann brauchen wir nur eine tastenkombi“; Tastenwahl „1“. Damit gilt: `opt+cmd+e`, geteilt mit „Editor schließen“, öffnet die Datei unter dem Cursor im Terminal. Ist sie für den Nutzer schreibbar, mit `vim <datei>`, sonst mit `sudo -e <datei>`. `shift+opt+cmd+e` bleibt frei.

## Abschluss 261029

Geschlossen als `done` (completed). Commits fd82056a..7a69a051: 36efb7f3 Kernmodul terminalskript, 158ef17b Kommando und Belegung opt+cmd+e, d5e891e0 Proben, 80edd741 Ausfuehrung und Uebergabe ans Terminal, cfd389e9 und 7a69a051 Anleitung (der zweite ergaenzt den Hinweis zur Terminal-Einstellung "Wenn die Shell beendet wird", Nutzerentscheid statt Selbstschliessen per AppleScript). Abschlussdurchsicht `reviews/261029-1911-reviewer-abschlussdurchsicht-datei-im-terminal-mit-vim-und-sudo-e.md`: accept, Bereich vollstaendig gedeckt. Alle acht Stoppbedingungen vom Nutzer als erfuellt bestaetigt, Nutzerabnahme (a)-(h) und make check auf seinem Mac bestanden. Offen fuer spaeter: `issues/261029-0958-…` (ctrl+o fragt metadata auf dem Hauptfaden), `issues/261029-1911-drei-proben-zitieren-…` und `issues/261029-1911-der-modulkopf-von-terminal-rs-…` (beide Low, Kommentare).

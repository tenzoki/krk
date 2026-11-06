# Menü- und Tastenwege fragen das Dateisystem nicht mehr auf dem Hauptfaden

## Directive

Wortlaut des Nutzers (261102), auf die Erklärung des Sammeldefekts `261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md` und seine Empfehlung eines eigenen Arbeitspakets mit Plan:

> ok, so machen

Erreicht ist das Ziel, wenn keiner der Wege, die `261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md` nennt, auf dem Hauptfaden das Dateisystem fragt, sodass ein hängendes Netzlaufwerk die Ereignisschleife nicht anhält, und Quelltextproben das festhalten wie bei `ctrl+o` und „Im Finder öffnen“.

## Anlass

Der Sammeldefekt `shared/issues/261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md` listet die Stellen (Stand `8278e685`, am Quelltext gelesen, nicht gemessen). Verwandt und schon eigens offen: der Löschweg, `shared/issues/261016-1956-der-loeschweg-fragt-canonicalize-und-den-papierkorb-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md`.

## Schließungsnotiz

Geschlossen am 261106 mit `done`, Commits `fe48deea..0e9a4c38`. Der Nutzer hat alle neun Grenzen aus `## Where this work stops` des gebundenen Plans bestätigt.

Die Schlussdurchsicht (`reviews/261105-2003-reviewer-schlussdurchsicht-hauptfaden-paket.md`, Bereich `7f448770..7938a6b0`) lautet „nachbessern“, das Urteil bleibt `revise`. Ihre drei Befunde sind in `0e9a4c38` behoben. Auf Wunsch des Nutzers lief keine vierte Durchsicht, und diese Commits hat keine Durchsicht geöffnet:

- `0e9a4c38` fix(statuszeile): der Wartesatz faellt mit der Geltung seines Befehls
- `4f63231a` docs(workbench): Schlussdurchsicht des Hauptfaden-Pakets, drei Defekte
- `85034f51` docs(workbench): Abschlussdurchsicht des Hauptfaden-Pakets, acht Defekte

Geprüft ist alles mit `make check` und am Quelltext. Im Programm ist nichts gelaufen; der Lauf an einer hängenden Freigabe und die Klickwege stehen beim Nutzer aus. Bekannte Reste: Ein dauerhaft hängendes Löschen verdeckt die unteren Meldungen der Statuszeile bis zum nächsten Löschbefehl. „Ort wählen“ und die Wege aus `terminal.rs` zeigen keinen Wartesatz. `unmountVolumeAtURL:` in `auswurf.rs` ist ungeprüft, und `FRIST` ist nicht gemessen.

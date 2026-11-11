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

## Schließungsnotiz

Geschlossen am 261111-1111 als `done`, Ergebnisklasse `completed`. Commits der Arbeit: `02c2f859..76fe5cb5` (`02c2f859` legt das Paket an; `2338b683`, `ef13457a`, `871f1062`, `36846a9b`, `4ae45b1d`, `d60bdb48`, `107bf14e` sind die Planschritte 1 bis 7, `96363ac4`, `2cae597a`, `bc3c2245` die Schritte 8 bis 10, `76fe5cb5` behebt die zwei Befunde der Schlussdurchsicht). `make check` ist am Stand `76fe5cb5` grün.

**Gebaut und durchgesehen, am laufenden Bündel nicht abgenommen.** Kein Lauf hat KRK im Vordergrund gesehen.

**Schlussdurchsicht:** `261111-1104-reviewer-schlussdurchsicht-menue-navigation-quicknote-txt-und-minutes.md` über `02c2f859..2cae597a`, Urteil `accept`, zwei Befunde niedriger Schwere, beide in `76fe5cb5` behoben und geschlossen (`261111-1104-howto-sagt-jede-unbekannte-kennung-verwerfe-die-eigene-keymap-und-eine-zurueckgezogene-wird-seit-heute-uebergangen.md`, `261111-1104-zwei-kommentare-in-belegung-rs-begruenden-das-abhaken-noch-mit-der-notiz-die-kein-kaestchen-hat.md`). Lücke der Durchsicht: `76fe5cb5` selbst hat keine Durchsicht geöffnet, und sieben Dateien des Bereichs sind nicht geöffnet worden (`crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/heimordner.rs`, `crates/krk-core/tests/sprache.rs`, `crates/krk-core/tests/text.rs`, `crates/krk-ui/src/appkit/eintragsansicht.rs`).

**Haltebedingungen.** Gelesen aus dem gebundenen Plan `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`, Abschnitt `## Where this work stops`. Der Modus des Pakets ist `autonomous`: die Klauseln sind dem Nutzer nicht vorgelegt und nicht beurteilt worden, keine ist als verfehlt markiert. Wörtlich:

1. Alle zehn Schritte stehen auf `done`, und `make check` ist nach dem letzten grün.
2. Jedes Kriterium von C1.1 bis C5.11 ist einem Schritt zugeordnet und dort durch eine Probe, eine Erhebung am Baum oder einen der Nutzerpunkte unten belegt.
3. Vor dem Schließen des Pakets hat der Nutzer am laufenden Bündel gesehen, dass `cmd+m` das Namensblatt in `minutes/` öffnet und das Fenster nicht minimiert. Zeigt die Probe, dass macOS die Taste abfängt oder das Fenster minimiert, hält die Arbeit an C5 an, und die Wahl einer anderen Taste geht an den Nutzer (erste Klausel unter `## Stops when` der Spezifikation).
4. Die Bausteine der Notizen, die `secrets.txt` weiter braucht, stehen und sind dem Nutzer genannt: die Eintragsform aus Themenzeile und Text im Kern und die Satzfamilie der Geheimnistabelle, jetzt mit dem Wort „Eintrag“ (E4). Die zweite Klausel unter `## Stops when` hält die Arbeit damit nicht an (condition did not arise: Tabelle und Dateiformat der Geheimnisse bestehen ohne `notes.txt`).
5. Die Frage nach dem Abnahmelauf (`261111-0852-schuldet-das-menue-und-minutes-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`) ist vor dem Schließen beantwortet. Sie hält keinen Schritt an.
6. Diese Arbeit liefert nicht aus, erhöht keine Version und setzt keinen Tag.
7. Nutzerpunkte am Bündel, die kein Agent fahren kann und die das Paket als „gebaut“ und nicht als „abgenommen“ schließen lassen, solange sie ausstehen: die Menüleiste in den drei Sprachen (C1.1 bis C1.3); die Übernahme einer wirklichen `scratch.txt` (C2.2); eine vor dieser Arbeit gesicherte `secrets.txt` öffnet sich mit ihrer PIN (C3.9); `cmd+m` aus jedem Fokus (C5.2, C5.3); `esc` im Blatt lässt den Fokus in der Liste (C5.6).

Zum Stand beim Schließen, ohne Urteil über die Klauseln: den Lauf aus Klausel 3 hat der Nutzer nicht gefahren, und die Frage aus Klausel 5 ist offen. Die Bausteine aus Klausel 4 sind dem Nutzer im Gespräch genannt.

**Offen geblieben:** die Entscheidungen `261111-0852-schuldet-das-menue-und-minutes-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md` und `261111-0852-gilt-die-duldung-einer-zurueckgezogenen-kennung-in-der-eigenen-keymap-fuer-jede-kuenftige-und-rueckwirkend.md`; die Defekte `261111-0852-anlegen-und-umbenennen-im-lokalen-ordner-stellen-ihren-systemaufruf-auf-dem-hauptfaden.md` und `261111-1056-der-modulkopf-der-eintragsansicht-sagt-keine-probe-sehe-einen-ausgelassenen-uebergang-und-eine-quelltextprobe-daneben-sieht-ihn.md`.

Der Nutzer hat am 261111 nach dem Schließen eine Auslieferung beauftragt; sie ist kein Teil dieses Pakets.

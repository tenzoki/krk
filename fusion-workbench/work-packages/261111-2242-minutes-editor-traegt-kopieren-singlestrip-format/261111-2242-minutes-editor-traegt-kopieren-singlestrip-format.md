# Die drei Schaltflächen der Quicknote (Kopieren, Singlestrip, Format) stehen auch im Editor einer Protokolldatei aus minutes/

## Directive

Der Editor zeigt die drei Schaltflächen „Kopieren“, „Singlestrip“ und „Format“, die er heute für die Quicknote zeigt, auch dann, wenn er eine Datei aus dem Unterordner `minutes/` des Notizordners hält, und jede der drei wirkt dort auf den Text der Protokolldatei so, wie sie in der Quicknote auf deren Text wirkt. Erreicht ist es, wenn `make check` grün ist, die Proben das Erscheinen und die Wirkung der drei Schaltflächen für eine Datei in `minutes/` halten und die Fassung mit der Änderung ausgeliefert ist.

## Die Worte des Nutzers

Am 261111, wörtlich:

> Quicknote has three buttons (Kopieren, Singlestrip, Format) which I want you to also integrate in the minutes editor.
> Mach das im autonomous mode, finish the task and make a full release.

Der zweite Satz ist der Auftrag zum Modus `autonomous` und zugleich der ausdrückliche Auftrag zur Auslieferung nach `README.md` `### Versionsstufen`.

## Schließungsnotiz

Geschlossen am 261112-0013 als `done`, Ergebnisklasse `completed`: gebaut und als `v4.1.0` ausgeliefert, im laufenden Bündel nicht abgenommen. Commit-Bereich der Sitzung: `52ad05b5..061999f9`. Die inhaltlichen Commits: `90ea8952`, `fc25958e`, `d9a6caa4`, `40d31378`, `f1ecd449`, `5eba21e5`, `033beb98`, `c12e6e06`, `98d3e74f`, `061999f9`.

Der Plan in Geltung war der am Paket gebundene, `261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md`. Seine Klauseln unter `## Where this work stops` sind dem Nutzer nicht vorgelegt worden, weil das Paket auf `autonomous` steht; keine ist als verfehlt markiert. Wörtlich:

- `make check` ist nach Schritt 7 grün: ja oder nein.
- Die Proben aus den Schritten 2 bis 6 halten Erscheinen und Wirkung der drei Schaltflächen vor einer Datei in `minutes/`, und die Proben der Quicknote sind unverändert grün: ja oder nein.
- Die Fassung 4.1.0 ist über `./release.sh 4.1.0` ausgeliefert, mit Tag und Releaseseite: ja oder nein.
- Vorbedingung der Auslieferung: alle Schritte 1 bis 7 sind eingetragen, und der Arbeitsbaum ist sauber. Die Antwort auf `261111-2248-schuldet-das-protokollknopf-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md` ist keine Vorbedingung; lautet sie „messen“, ist der Lauf Nutzerarbeit nach der Auslieferung.
- Die Antwort auf `261111-2248-leert-und-schliesst-kopieren-auch-eine-protokolldatei-oder-kopiert-es-dort-allein.md` steht vor Schritt 3: ja oder nein. Der Plan ist auf Möglichkeit 1 geschrieben.
- Nicht Teil dieser Arbeit: die Wirkung im laufenden Bündel zu prüfen (Klick, Lage der Reihe, `cmd+z` nach einer Umformung). Das ist Nutzerarbeit, und bis dahin ist die Arbeit gebaut und nicht abgenommen.

Zur fünften Klausel: die Antwort stand vor Schritt 3 nicht und steht auch jetzt nicht. Gebaut ist Möglichkeit 1 (Kopieren kopiert vor einem Protokoll allein); der Datensatz ist offen, und der Nutzer hat darüber nicht entschieden.

Durchsicht: `261112-0002-reviewer-protokollknoepfe-vor-der-auslieferung.md` über `52ad05b5..b000bc71`, Urteil `revise`. Ihr erster Befund ist mit `98d3e74f` über die Anleitung geschlossen; offen bleiben `261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md` und, daneben gefunden, `261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md`. Von keiner Durchsicht geöffnet sind die Commits nach `b000bc71`: `c12e6e06` (die Durchsicht selbst), `98d3e74f` (der Absatz in `HowTo.md`), `061999f9` (die Versionszahl) und die Commits, die allein das Ereignisprotokoll tragen; dazu `52ad05b5`, der vor dem Bereich liegt. Weil das Urteil nicht `accept` lautet, bleibt eine `succeeded`-Kante auf dieses Paket unerfüllt.

Offen für den Nutzer: die zwei Entscheidungsdatensätze des Pakets, und die Frage aus dem Plan, ob die drei Befehlsnamen weiter „Quicknote“ sagen sollen.

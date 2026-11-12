# Die zwei offenen Defekte nach den Protokollknöpfen beheben: Zeilenenden einer CSV in minutes/ und die Zeitprobe im Cloudladen

## Directive

Die zwei Defekte, die das Paket `261111-2242-minutes-editor-traegt-kopieren-singlestrip-format.md` offen hinterlassen hat, sind behoben: Format und Singlestrip lassen einer CSV oder TSV mit `\r\n` in `minutes/` ihre Zeilenenden (`261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md`), und die Zeitprobe zum Abbruch eines Wartenden im Cloudladen fällt unter Last nicht mehr (`261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md`). Erreicht ist es, wenn die Abnahme jedes der zwei Datensätze erfüllt ist, `make check` grün ist und beide Datensätze geschlossen sind.

## Die Worte des Nutzers

Am 261112, wörtlich:

> Lege ein Paket an, um die issues zu fixen und führe es autonom durch

Gelesen ist „die issues“ als die zwei Defekte, die der Abschlussbericht der vorangegangenen Sitzung unmittelbar davor als offen genannt hat, und nicht als jeder offene Defekt der Werkbank. Das Wort „autonom“ ist der Auftrag zum Modus `autonomous`. Eine Auslieferung ist nicht beauftragt.

## Schließungsnotiz

Geschlossen am 261112-0853 als `done`, Ergebnisklasse `completed`. Commit-Bereich: `1cf9abbf..375bb474`, darin die inhaltlichen Commits `f0a60eea` (Zeilenenden einer CSV in `minutes/`), `440d8b15` (die gefallene Zeitprobe im Cloudladen) und `5d9f61fc` (ihre zwei Schwesterproben). Ausgeliefert ist nichts davon; die Fassung im Baum steht weiter auf 4.1.0.

Am Paket ist kein Plan gebunden, und die Sitzung hielt keinen: die zwei Defektdatensätze nannten Dateien und Abnahme selbst, also gab es keine Klauseln, die dem Nutzer vorzulegen gewesen wären.

Geschlossen sind `261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md`, `261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md` und, während der Arbeit angelegt, `261112-0840-zwei-proben-zu-abwarten-im-cloudladen-tragen-dieselbe-freie-20-ms-wanduhrschranke-weiter.md`.

Durchsicht: `261112-0851-reviewer-schlussdurchsicht-zwei-defekte-nach-den-protokollknoepfen.md` über `1cf9abbf..375bb474`, Urteil `accept`; nicht geöffnet hat sie `fusion-workbench/orchestrator-events.jsonl`. Von keiner Durchsicht geöffnet bleiben aus der Sitzung davor `52ad05b5`, `c12e6e06`, `98d3e74f`, `061999f9` und `9ea5f61d`, dazu der Commit, der diese Notiz trägt.

Offen bleiben aus diesem Paket: `261112-0840-die-regel-welches-zeilenende-ein-text-traegt-steht-an-drei-stellen-und-keine-ruft-die-andere.md`, `261112-0851-die-fragenzahl-in-der-probe-zu-abwarten-haelt-den-takt-nicht-den-ihr-kommentar-zusagt.md` und die Nutzerfrage `261112-0851-darf-format-oder-singlestrip-in-einer-crlf-csv-einen-zellumbruch-im-ausschnitt-zu-crlf-machen.md`.

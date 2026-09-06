# Analyst: achtzehn offene Entscheidungsfragen verdichtet

**Datum:** 2026-09-06 21:00
**Agent:** analyst
**Status:** Complete
**Baumstand:** HEAD `8276170`, Zweig `main`, 17 Commits vor `origin/main`

## Auftrag

Die achtzehn offenen Entscheidungsdatensätze 39 bis 56 unter `shared/decisions/` so aufbereiten, dass der Nutzer sie im Chat nacheinander beantworten kann: je Frage ein Satz in Alltagssprache, die Möglichkeiten aus dem Datensatz, eine Empfehlung, und eine Prüfung gegen den heutigen Baum. Nichts entschieden, nichts umbenannt, keine Datei am Baum geändert.

## Ergebnis

Bericht: `260906-2100-achtzehn-offene-fragen-zur-vorlage-im-chat.md`.

Siebzehn der achtzehn Fragen sind unverändert offen und einzeln am Baum belegt. Eine ist gegenstandslos: Nummer 54 (`260906-0202_*_werden-defektdatensaetze-ueber-eingefrorene-spec-und-plantexte-geschlossen-oder-bleiben-sie-offen.md`) ist von der vierten Behebungsschleife überholt. Alle neun namentlich genannten Datensätze tragen den Abschlussmarker; die Verlaufsdatei jener Schleife sagt es selbst (`260906-0509-coder-vierte-behebungsschleife-die-klasse-s-neu-sortiert.md:169-172`), Commit `8276170`.

Keiner der achtzehn Datensätze trägt eine gefüllte Zeile `Answered:` oder `Answer located:`; auf der Platte steht zu keinem eine Antwort.

## Verschiebungen gegenüber dem Wortlaut der Datensätze

- Nummer 40 nennt `gh` als erste Ausnahme des Suchpfads. Heute rufen `rustup`, `iconutil` und `cargo` ebenso über den Suchpfad; `gh` ist der vierte Fall.
- Nummer 44: `plus` und `minus` des Zehnerblocks lösen seit Runde 20 aus. Der Datensatz nennt vier Rechenzeichen als stumm; es sind zwei.
- Nummer 47: der Baum ist der empfohlenen Möglichkeit weiter gefolgt, ohne sie zu bestätigen. Der Helfer `varianten_der_aufzaehlung` läuft an drei Aufzählungen statt an einer, und die Zahl der `ALLE`-Listen ist von elf auf fünfzehn gestiegen.

## Geänderte Dateien

`260906-2100-achtzehn-offene-fragen-zur-vorlage-im-chat.md` im Analysespeicher (neu), diese Verlaufsdatei. Kein Eingriff am Baum, kein Datensatz umbenannt.

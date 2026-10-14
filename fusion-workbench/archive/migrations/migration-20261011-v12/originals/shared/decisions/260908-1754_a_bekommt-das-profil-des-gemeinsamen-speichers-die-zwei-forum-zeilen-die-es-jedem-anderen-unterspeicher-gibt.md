# Bekommt das Profil des gemeinsamen Speichers die zwei Forum-Zeilen, die es jedem anderen Unterspeicher gibt?

**Status:** beantwortet (von der Bahn R1 im Rahmen ihres Auftrags), noch nicht
als offene Nutzerfrage bestätigt
**Gestellt:** 260908-1754
**Answered:** Nein — die verdichtete Zeile steht in den zwei Wurzelprofilen,
und `fusion-Werkbank: der gemeinsame Speicher` bleibt bei seinen zehn
Unterspeichern.

## Die Frage

`resources/default-readers.toml` führt für `fusion-workbench/shared$` ein
Profil, das **jeden** Unterspeicher mit zwei Zeilen aufzählt: wie viele
Datensätze darin stehen und wann zuletzt einer geschrieben wurde. Seit dem
260908 gibt es einen elften Unterspeicher, `forum`, und er steht dort nicht.

Damit zählt ein Profil, dessen ganzer Zuschnitt „alle Unterspeicher" ist, einen
davon nicht mehr auf. Das ist eine bewusste Lücke und keine Auslassung, aber es
ist eine Lücke.

## Warum sie so gewählt ist

**Der Haushalt.** Das Speicherprofil steht mit zehn von zwölf erlaubten
Leseläufen zwei vor der Schranke, und sein eigener Kommentar schreibt aus: „wer
einen elften Unterspeicher aufnimmt, rechnet nach, was er kostet." Ein elfter
Ort nähme den vorletzten Lauf. Eine eigene Probe,
`ein_elfter_unterspeicher_kostet_einen_elften_leselauf`, hält genau diesen
Abstand als Gegenprobe fest; sie würde mit dem elften Ort zur Aussage über
zwölf Läufe und null Abstand.

**Die Sichtbarkeit.** Die zwei Wurzelprofile stehen bei vier von zwölf und drei
von zwölf Läufen, dort ist Platz. Und eine Nachricht, die die Gegenseite kennen
soll, muss dort auffallen, wo man ohnehin steht: an der Projektwurzel, nicht in
einem Speicherordner, den man selten öffnet. Der Nutzer hat die Anforderung als
„dort, wo die fusion-Werkbank zusammengefasst wird" gestellt, und das sind die
Wurzelprofile — `shared/` fasst den gemeinsamen Speicher zusammen und nicht die
Werkbank.

## Was daran offen bleibt

Die Begründung ist eine Kostenrechnung und keine Sachaussage. Fällt die
Schranke von zwölf Leseläufen je Zusammenfassung, oder wird sie angehoben, ist
das Argument weg und die Lücke steht ohne Grund da. Zwei Auslöser machen die
Frage wieder auf:

1. Die Schranke `HOECHSTENS_LESELAEUFE` wird angehoben.
2. Der Nutzer öffnet `fusion-workbench/shared` und vermisst das Forum dort.

**Der Datensatz steht, damit die Lücke nicht stillschweigend zur Regel wird.**
Das ist die Lage, vor der `CLAUDE.md` beim zurückgestellten Datensatz zur
Zusage L9 warnt: wird nie wieder gefragt, ist „so bleiben" der Sache nach
entschieden, ohne dass es jemand aufgeschrieben hätte.

## Umgesetzt in

`resources/default-readers.toml`, Bahn R1 vom 260908. Der Nachzug der Proben
steht in
`260908-1754_*_das-forum-profil-macht-fuenf-zahlenzusagen-in-drei-dateien-unter-crates-falsch.md`.

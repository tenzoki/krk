# Die Runde 18 hat keinen Circle-Datensatz, und jede Zählung über `circles/` übergeht sie

---
**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `CLAUDE.md` (die Rundentabelle, die Zeile über `ls fusion-workbench/circles/*/_*_circle.md`, der Absatz über den Abnahmelauf); `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Abschnitt „Where this Circle stops": „Es ist kein Circle aktiv"); `resources/default-readers.toml` (das Profil „fusion-Werkbank: alle Runden")

---

## Was ist

Die Arbeit der Runde 18 liegt vollständig im gemeinsamen Speicher: der Plan in
`shared/planning/`, die sieben Entscheidungen in `shared/decisions/`, die Defekte in
`shared/issues/`, die Durchsichten in `shared/reviews/`. Ein Verzeichnis unter
`fusion-workbench/circles/` gibt es für sie nicht; `ls fusion-workbench/circles/*/_*_circle.md`
liefert am 260826 siebzehn Datensätze, und keiner davon gehört zu dieser Runde.

Das ist nach der Herkunftsregel richtig abgelegt — es war kein Circle aktiv, also gehört alles
nach `shared/`. Der Befund ist nicht die Ablage, sondern was daraus folgt.

## Warum das zählt

`CLAUDE.md` beantwortet drei Fragen ausdrücklich über den Dateibestand unter `circles/` und
nicht über seinen eigenen Text:

- **Wie viele Runden gefahren sind und wie jede geschlossen hat**: „sagt der Dateibestand und
  nicht diese Zeile: `ls fusion-workbench/circles/*/_*_circle.md`".
- **Welche Runden seit dem letzten Abnahmelauf geschlossen haben**:
  „`ls fusion-workbench/circles/*/_[bc]_circle.md` gegen das Datum `260810`".
- **Welche Runden beschränkt und welche kohärent geschlossen sind**: dieselben zwei Globs.

Alle drei sind für die Runde 18 leer. Wer sie fährt, bekommt einen Bestand, in dem diese Runde
nicht vorkommt, und zwar ohne jeden Hinweis darauf, dass etwas fehlt — dieselbe Gestalt von
Fehlbefund, gegen die `CLAUDE.md` diese Idiome überhaupt eingeführt hat. Die Rundentabelle
darüber endet bei der 17 und ist damit nicht bloß eine Zeile im Rückstand: sie hat für die 18
keinen Ort, an dem sie nachziehen könnte.

Dasselbe trifft das mitgelieferte Leseprofil „fusion-Werkbank: alle Runden", das diese Runde
gerade gebaut hat. Es zählt sechs Zustände über `circles/*/_*_circle.md` und offene Defekte
über `circles/*/issues`. Die 116 offenen Defekte, die es an dieser Werkbank zeigt, enthalten
keinen einzigen der rund zwanzig, die diese Runde in `shared/issues/` abgelegt hat. Die Zahl
ist richtig für das, was sie misst, und wird von einem Leser als Auskunft über die Werkbank
gelesen.

## Möglichkeiten

1. **Einen Circle-Datensatz für die Runde 18 nachtragen** und die Artefakte hineinbewegen. Das
   kehrt die Herkunftsregel um, die für den Ablageort auf die Herkunft und nicht auf die
   spätere Reichweite abstellt; die Regel nennt für so etwas ausdrücklich einen
   Beförderungsschritt und keine zweite Ablageregel.
2. **Einen Circle-Datensatz ohne Umzug anlegen**, der allein den Zustand und die Schließung
   trägt und den Plan im gemeinsamen Speicher zitiert. Billig, und die drei Globs antworten
   wieder vollständig. Der Preis: ein Circle-Verzeichnis, dessen Speicher leer sind.
3. **`CLAUDE.md` sagt es aus**: eine Zeile, die festhält, dass nicht jede Runde einen Circle
   hat und die drei Globs deshalb eine Untergrenze liefern. Kostet nichts und behebt nichts.
4. **So lassen.** Dann ist „wie viele Runden sind gefahren" an diesem Projekt nicht mehr aus
   dem Dateibestand zu beantworten, und `CLAUDE.md` sagt das Gegenteil.

## Empfehlung

Möglichkeit 3 zusammen mit 2, in dieser Reihenfolge: erst der Satz, damit die Auskunft ehrlich
ist, dann die Entscheidung über den Datensatz. Die Wahl gehört dem Nutzer, weil sie festlegt,
was ein Circle in diesem Projekt künftig ist — die Einheit der Arbeit oder die Einheit der
Ablage.

**Schwere:** mittel. Kein Verhalten der Anwendung hängt daran; drei Auskünfte, auf die sich
`CLAUDE.md` ausdrücklich verlässt, sind stillschweigend unvollständig.

**Gefunden:** reconciler, beim Abgleich der Runde 18 gegen `20eccd4..e5ec81a`.

---
Also seen: 260826-0923 by coderev — „Die Arbeit der Runde 18 liegt vollständig im gemeinsamen
Speicher" trifft die Ablage der **erzeugten** Artefakte, nicht alles, was die Runde geschrieben
hat. Zwei Commits der Runde schreiben in den Circle der Runde 17:
`git log --name-only 2a77012..HEAD -- fusion-workbench/circles/` liefert `e922c9e` (260825-19:05)
und `d08dbac` (260826-08:16), beide an
`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_c_jeder-gepackte-eintrag-traegt-den-1-januar-1980-….md`
— also gerade an dem Vorgang, den `CLAUDE.md` Zeile 18 seit `fb50fcd` selbst als Arbeit der
Runde 18 führt („Zip-Zeitstempel behoben"). Der Halbsatz ist mit `fb50fcd` nach `CLAUDE.md`
gewandert und steht dort in derselben Fassung. Wer nach den Artefakten der Runde 18 allein unter
`shared/` sucht, findet diesen einen nicht. Die Folgerung des Datensatzes — die drei Globs
liefern eine Untergrenze — hängt daran nicht.

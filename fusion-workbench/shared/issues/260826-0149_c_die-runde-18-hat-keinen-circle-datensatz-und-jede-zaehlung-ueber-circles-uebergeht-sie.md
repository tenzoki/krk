# Die Runde 18 hat keinen Circle-Datensatz, und jede Zählung über `circles/` übergeht sie

---
**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `CLAUDE.md` (die Rundentabelle, die Zeile über `ls fusion-workbench/circles/*/_*_circle.md`, der Absatz über den Abnahmelauf); `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Abschnitt „Where this Circle stops": „Es ist kein Circle aktiv"); `resources/default-readers.toml` (die Profile „fusion-Werkbank: alle Runden" und „Projektwurzel mit fusion-Werkbank")

---

## Was ist

Die Arbeit der Runde 18 liegt vollständig im gemeinsamen Speicher: der Plan in
`shared/planning/`, die sieben Entscheidungen in `shared/decisions/`, die Defekte in
`shared/issues/`, die Durchsichten in `shared/reviews/`. Ein Verzeichnis unter
`fusion-workbench/circles/` gibt es für sie nicht; `ls fusion-workbench/circles/*/_*_circle.md`
liefert am 260826 siebzehn Datensätze, und keiner davon gehört zu dieser Runde.

Das ist nach der Herkunftsregel richtig abgelegt — es war kein Circle aktiv, also gehört alles
nach `shared/`. Der Befund ist nicht die Ablage, sondern was `CLAUDE.md` daraus macht.

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

**Der Mechanismus der Leseprofile steht dabei nicht zur Debatte.** Das Profil „fusion-Werkbank:
alle Runden" erkennt `fusion-workbench/circles$` und zählt seine sechs Zustandszeilen über
`ordner = "*"`; ein Circle ist ein Verzeichnis darunter, und das ist eindeutig. Nichts an
diesem Befund verlangt daran eine Änderung. Der Leser trifft die Lage sogar besser als
`CLAUDE.md`: er führt beide Zahlen, nur an zwei Orten. Die offenen Defekte des gemeinsamen
Speichers zählt das Profil „Projektwurzel mit fusion-Werkbank"
(`zaehlung = { ordner = "fusion-workbench/shared/issues", … }`), die offenen Defekte über alle
Runden das Rundenprofil (`zaehlung = { ordner = "*/issues", … }`).

Der Befund ist damit eng und trifft allein die Werkbank dieses Projekts: die Arbeit der
Runde 18 liegt in `shared/`, weil kein Circle aktiv war, und `CLAUDE.md` behauptet trotzdem, die
drei `ls`-Idiome beantworteten „wie viele Runden sind gefahren" vollständig. Ein falscher Satz
in `CLAUDE.md`, kein offener Punkt im Vokabular.

## Was zu tun war

`CLAUDE.md` sagt es aus: eine Zeile, die festhält, dass nicht jede Runde einen Circle-Datensatz
hat und jede Zählung über `circles/` in dieser Datei deshalb eine Untergrenze liefert. Das
kostet nichts und richtet die falsche Auskunft dort, wo sie steht.

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

---
Resolved: fb50fcd — `CLAUDE.md` sagt es jetzt an vier Stellen aus, am 260826-1017 einzeln gegen
den Baum gelesen: die Zeile über `ls fusion-workbench/circles/*/_*_circle.md` trägt den Satz
„Nicht jede Runde hat einen Circle-Datensatz, und jede Zählung über `circles/` in dieser Datei
liefert deshalb eine Untergrenze" samt Verweis auf diesen Datensatz (`CLAUDE.md:11`), die
Rundentabelle hat eine Zeile 18 mit einem Strich in der Circle-Spalte (`:32`), der Absatz über
die beschränkten und kohärenten Abschlüsse trägt „Beide Listen lassen jede Runde ohne
Circle-Datensatz aus" (`:71`), und der Absatz über den Abnahmelauf der zehn Zusagen nennt die
Runde 18 ausdrücklich neben dem Glob (`:85`). Der Datensatz ist am 260826-1017 auf Anweisung
des Nutzers auf diese eine Möglichkeit gekürzt; die drei anderen, darunter der nachgetragene
Circle-Datensatz, sind damit nicht offen, sondern verworfen.

# Auf welcher Taste liegt der Terminal-Befehl, und ist die gerufene Anwendung schon in dieser Runde einstellbar?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `planning/260802-1036_*_spec-navigator-geruest.md` (C11), `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` (`### Frage 4`, S18b, S18c), `decisions/260803-2300_*_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`, `decisions/260805-1411_*_ordnernavigation-mit-oder-ohne-zusatztaste.md`, `decisions/260805-0000_*_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`

---

## Frage

Der Nutzer hat am 260805 eine Funktion nachbeauftragt: eine Taste, die im angezeigten Ordner ein Terminal öffnet. Zwei Punkte waren daran zu entscheiden, bevor ein Schritt sie umsetzen kann. Erstens, welche Kombination sie ab Werk trägt, denn die Auslieferungsbelegung führt 63 Kombinationen und jede weitere muss konfliktfrei danebenpassen. Zweitens, ob der Nutzer schon in dieser Runde bestimmen kann, welche Anwendung gerufen wird, oder ob die Runde fest auf Terminal.app läuft und die Wahl einer späteren überlässt.

Beide Punkte hat der Nutzer am 260805 selbst beantwortet. Dieser Datensatz hält die Antworten fest, samt den verworfenen Möglichkeiten und der Begründung, die er dafür genannt hat.

## Frage 1: Die Kombination

### Möglichkeiten

1. **`ctrl+o`** — der Weg von Total Commander, wo Ctrl+O die Kommandozeile öffnet.
   - Für: Die Norton-Linie ist das erklärte Vorbild dieses Vorhabens, und dieselbe Herkunft trägt bereits die Funktionstastenreihe F3 bis F8 und die Belegungsansicht auf F1. Ein Nutzer, der von Total Commander kommt, greift die Taste ohne Nachschlagen.
   - Gegen: `ctrl` ist auf dem Mac die unüblichste der vier Zusatztasten. Die Auslieferungsbelegung führt sie heute achtmal, gegenüber 40 Kombinationen mit `cmd`.
2. **`shift+cmd+t`** — die Empfehlung des Planners, T wie Terminal, neben `cmd+t` für den neuen Tab.
   - Für: Mac-übliche Form, und die Nachbarschaft zu `cmd+t` ist dieselbe Systematik, die schon `f6` gegen `shift+f6` und `f3` gegen `shift+f3` trägt.
   - Gegen: Ohne Vorbild in der Norton-Linie. Der Anfangsbuchstabe stammt aus dem Englischen, während die beiden eigens gewählten Kürzel der Runde, `shift+cmd+k` und `shift+cmd+v`, dem deutschen Verb folgen.
3. **`f9`** — die nächste freie Taste der Funktionstastenreihe.
   - Für: Reiht sich in die Norton-Tastenfolge ein und braucht keine Zusatztaste.
   - Gegen: F9 trägt in der Norton-Reihe das Menü und nicht das Terminal. Auf dem Abnahmegerät liegt die Reihe zudem auf einem Touch Bar, und C3 hält fest, dass ein Befehl, der dort den Blick auf ein Glasfeld verlangt, die Maxime der Tastatursteuerung verfehlt.

### Antwort

**`ctrl+o`.** Der Nutzer hat das Total-Commander-Vorbild als tragenden Grund genannt und die beiden anderen Möglichkeiten ausdrücklich verworfen.

Die Kombination ist am 260805-1623 gegen alle 63 ausgelieferten Kombinationen aus 56 Funktionen geprüft, verglichen am vollständigen Eintrag und nicht an der Teilzeichenkette: `ctrl+o` kommt in keiner Tastenliste vor. Belegt sind in derselben Familie `ctrl+u`, `ctrl+delete`, `ctrl+tab`, `ctrl+shift+tab`, `ctrl+left`, `ctrl+right`, `ctrl+cmd+n` und `ctrl+cmd+u`; keine davon kollidiert.

Die Kombinationsschreibweise aus S9 trägt `ctrl+o` ohne Erweiterung. `ctrl` ist eine der vier Zusatztasten der Schreibweise und `o` ein gewöhnlicher Buchstabe. Der Fall unterscheidet sich damit von der Umbelegung vom 260804-1122, die mit S11b erst acht Tastennamen nachtragen musste, bevor sich zwei der drei Kombinationen überhaupt hinschreiben ließen.

## Frage 2: Einstellbarkeit der gerufenen Anwendung

### Möglichkeiten

1. **Sofort einstellbar, vorbelegt mit Terminal.app.** Die Ablage bekommt einen Eintrag für die Terminal-Anwendung, den der Nutzer von Hand pflegt. Eine Oberfläche dafür entsteht in dieser Runde nicht.
   - Für: Der Nutzer benutzt nicht Terminal.app. Eine Runde, die fest auf Terminal.app läuft, liefert ihm eine Funktion, die er nicht benutzen kann.
   - Gegen: Die Ablage wächst um eine vierte Datei, und die Runde bekommt ihre erste von Hand gepflegte Einstellung ohne Oberfläche.
2. **Fest auf Terminal.app, Einstellbarkeit in einer späteren Runde.**
   - Für: Der kleinere Schnitt. Kein Eintrag in der Ablage, kein neues Datenartefakt, kein Format, das eine spätere Oberfläche wieder einlesen muss.
   - Gegen: Verschiebt den Nutzen der Funktion auf eine Runde, die es noch nicht gibt.

### Antwort

**Sofort einstellbar, vorbelegt mit Terminal.app**, ohne Oberfläche in dieser Runde. Der Nutzer hat Möglichkeit 1 gewählt, obwohl Möglichkeit 2 der kleinere Schnitt gewesen wäre. Sein Grund: er benutzt ein anderes Terminal, und die Einstellung von Hand in einer Datei ist genau der Weg, den `keymap.toml` nach `### Frage 4` des Plans ohnehin vorsieht.

Zwei Folgeentscheidungen hat der Nutzer dem Planner überlassen, und beide stehen im Plan mit ihrer Herleitung:

- **Wohin die Einstellung geht.** Eine vierte Ablagedatei `settings.toml` neben `keymap.toml`, `bookmarks.toml` und `session.toml`. Die Herleitung steht in `### Frage 4` des Plans; verworfen sind alle drei vorhandenen Dateien, jede aus einem eigenen Grund.
- **Wie eine Anwendung benannt wird.** Über ihre Bündelkennung, etwa `com.apple.Terminal`, und nicht über einen Pfad. Die Herleitung steht ebenfalls in `### Frage 4`.

## Randbedingungen

- Die Funktion muss über die Tastatur erreichbar sein und in der Belegung stehen, wie jede andere. C3 lässt keine Kombination zu, die die Konflikterkennung nicht sieht.
- Die Auslieferungsbelegung bleibt konfliktfrei (C3).
- Die Maxime "supersimpel" schließt eine Lösung aus, die die Funktion mit einer eigenen Sonderregel, einer eigenen Ausnahme und einem eigenen Rückfallweg erkauft.
- Der Circle schließt einen integrierten Browser aus. Ein integriertes Terminal wäre der Fall derselben Art; KRK ruft eine fremde Anwendung und zeigt selbst keine Terminal-Ausgabe.

---
Answered: `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` `### Frage 4` und die Schritte S18b und S18c — `ctrl+o` als Kombination, `settings.toml` als vierte Ablagedatei, Bündelkennung als Namensform, Vorbelegung `com.apple.Terminal`. Die Fähigkeit steht als C11 im Spec.

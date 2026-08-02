# Shaper: Spec für Runde 1, Navigator-Gerüst

**Datum:** 2026-08-02
**Zeitstempel:** 260802-1036, fortgeschrieben 260802-1105
**Agent:** shaper, in-Circle-Klärungsmodus
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Status:** Abgeschlossen. Vier Antworten eingearbeitet, eine Teilfrage offen (Referenzgerät).

## Auftrag

Der Orchestrator hat den Shaper innerhalb des aktiven Circles eingesetzt, um den Spec für die erste Umsetzungsrunde zu schreiben. Der Zuschnitt der Runde stammt vom Nutzer aus der Phase-0-Klärung: nur das lauffähige Navigator-Gerüst, ohne Editor und ohne Git. Drei Pflichten kamen hinzu. Die Maxime "superschnell" war in messbare Abnahmekriterien zu überführen, zwei der fünf offenen Entscheidungsdatensätze waren mit dem Nutzer zu klären, und eine Technologiefestlegung war ausdrücklich zu unterlassen.

## Gelesene Grundlagen

- Circle-Datensatz `_t_circle.md`, vollständig
- `CLAUDE.md` im Projektwurzelverzeichnis, einschließlich der neuen Zeile `**Language:** de`
- `idea.txt`, die Quelle der Directive
- alle fünf Entscheidungsdatensätze unter `shared/decisions/`
- `shared/issues/`, ein Eintrag, bereits geschlossen
- die Stilprofile `chat-voice-de.yaml` und `default-voice-de.yaml`

## Ergebnis

Der Spec liegt unter `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`. Er führt neun Fähigkeiten mit Abnahmekriterien: die beiden Dateifenster mit Tabs, die Tastaturnavigation, die konfigurierbare Tastenbelegung, die Dateioperationen einschließlich Stapel, die Lesezeichen- und Geräteleiste, das Vorschaufenster mit seinem Halteverhalten, die verstellbaren und ausblendbaren Fenster, die Leistungszusagen und die Beschränkung auf lokale Laufwerke. Zwei Mermaid-Diagramme gehören dazu, eines für Aufbau und Datenfluss der Runde, eines für das Halteverhalten eines Vorschau-Tabs.

Das Feld `**Active spec/plan:**` im Circle-Datensatz zeigt jetzt auf diesen Spec. Kein anderer Abschnitt des Datensatzes wurde angefasst.

### Die Maxime "superschnell"

Abschnitt C8 des Specs überführt sie in zehn Zusagen mit Messbedingungen. Die Werte stammen aus zwei belastbaren Größen und nicht aus einer Schätzung: der Bildwiederholrate eines Bildschirms, also 16 ms bei 60 Hz, und den Reaktionszeitschwellen aus der Mensch-Maschine-Forschung, die auf Miller 1968 zurückgehen und bei Nielsen 1993 in der heute gebräuchlichen Form von 0,1 s und 1 s zusammengefasst sind. Vier weitere Werte sind daraus abgeleitet oder linear fortgeschrieben. Der Spec kennzeichnet alle zehn ausdrücklich als Vorschlag zur Bestätigung, weil keiner an KRK gemessen ist. Die Messbedingungen nennen einen definierten Prüfordner mit 10.000 Einträgen, zwanzig Wiederholungen, das 95. Perzentil statt des Mittelwerts und getrennte Werte für kalten und warmen Dateisystem-Cache.

### Die beiden zu klärenden Entscheidungen

Der Shaper lief als Unteragent und hatte damit kein Werkzeug für eine unmittelbare Rückfrage an den Nutzer. Nach der Regel im eigenen Prompt gehen die Fragen deshalb als Bündel an den Orchestrator, der sie weiterreicht. Beide Datensätze bleiben unverändert auf `_o_`, weil keine Antwort vorliegt. Eine Antwort einzutragen, die nicht gefallen ist, wäre eine Erfindung.

- `shared/decisions/260802-0842_o_f-tasten-unter-macos-systembelegung.md`, Empfehlung des Shapers ist Möglichkeit 3
- `shared/decisions/260802-0842_o_loeschen-papierkorb-oder-endgueltig.md`, Empfehlung des Shapers ist Möglichkeit 2

### Neu angelegte Entscheidungsdatensätze

Während der Arbeit sind zwei weitere Fragen aufgetaucht, die die Runde binden. Beide entstanden aus der Directive dieses Circles und liegen deshalb nach der Herkunftsregel im Circle, nicht im geteilten Speicher.

- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_o_leistungszusagen-navigator.md`, zu den Zahlen aus C8 und zum fehlenden Referenzgerät
- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_o_umbenennen-im-stapel-umfang.md`, zur Reichweite des Umbenennens im Stapel

### Vom Shaper getroffene Vorbelegungen

Sieben kleinere Punkte hat der Shaper mit einer Begründung im Spec festgelegt, statt sie zu Fragen zu machen: die Wiederherstellung der Sitzung, die Rolle des aktiven Fensters als Quelle, versteckte Dateien anfangs ausgeblendet, Name aufsteigend als Standardsortierung, die einmalige Rückfrage bei Namenskonflikten mit einer Option für alle weiteren Fälle, das Weiterlaufen eines Stapels nach einer gescheiterten Einzelposition und die Vorschau ohne Formatierung. Der Nutzer kann jede davon bei der Durchsicht des Specs umstoßen.

Ein Punkt verdient eine eigene Erwähnung: F4 bleibt in dieser Runde unbelegt. Die Norton-Bedeutung "Bearbeiten" zeigt auf den Editor, und der Editor gehört in eine spätere Runde. Eine Behelfsbelegung mit dem Systemeditor müsste die spätere Runde wieder entfernen.

### Anforderungen, die die Technologiewahl einschränken

Der Spec trifft keine Festlegung zu Sprache oder Werkzeugkasten und benennt zwei Anforderungen, die den Vergleich durch den analyst binden. Erstens muss die Anwendung Tastenereignisse früh genug entgegennehmen, um systemseitig vorbelegte Tasten zu erreichen, und jede Kombination zur Laufzeit umbelegen können. Zweitens sind die Zusagen aus C8 einzuhalten, insbesondere die 16 ms zwischen Tastendruck und sichtbarer Reaktion.

## Nicht angefasst

Die drei Entscheidungsdatensätze zu späteren Runden blieben unberührt: die Bedeutung von "revert", die Formatansicht des Editors je Dateityp und das Code-SDK für die spätere KI-Anbindung. Kein Code, kein Plan, keine Implementierungsschritte, kein Commit.

---

# Zweite Runde: die vier Antworten des Nutzers, 260802-1105

Alle vier Fragen sind beantwortet und im Spec eingearbeitet. Offen bleibt eine einzige Teilfrage, das Referenzgerät für die Zeitzusagen. Der Spec trägt weiterhin den Marker `_o_`.

## Die Antworten und was sie im Spec bewirkt haben

**Funktionstasten.** Der Nutzer hat Möglichkeit 1 gewählt: ausgeliefert wird ausschließlich die Fn-Kombination, Fn+F3 bis Fn+F8, die nackten Funktionstasten bleiben frei. Damit ist die Empfehlung des Shapers, beide Wege ab Werk zu belegen, abgelehnt. Die Wahl hat einen Vorteil, den die Empfehlung nicht hatte: die Belegungsansicht führt je Funktion eine Zeile statt zweier. Der Spec nennt in C3 jetzt sieben Kriterien statt der bisherigen sechs, darunter eines für den Fall, dass der Nutzer die Systemeinstellung "F1, F2 usw. als Standard-Funktionstasten verwenden" von sich aus aktiviert. In diesem Systemzustand erzeugt die nackte Taste dasselbe Tastenereignis wie sonst die Fn-Kombination, sodass die Belegung ohne zweiten Eintrag weitergilt.

**Löschen.** Die Antwort lautet wörtlich: "Delete löscht in Papierkorb, FN+F8 endgültig". Sie folgt der Aufteilung aus Möglichkeit 2, ändert aber die Tasten. Der schnelle Weg liegt auf der Taste Delete allein, das endgültige Löschen auf Fn+F8. Shift+Delete kommt in der Antwort nicht mehr vor und bleibt ab Werk unbelegt. Der Spec hat in C4 sechs Kriterien zum Löschen bekommen, darunter eines, das Delete auf den Fall beschränkt, dass der Eingabefokus im Dateifenster steht; in der Pfadeingabe und im Umbenennen-Feld bleibt sie die Rückschritt-Taste.

**Zeitzusagen.** Alle zehn Zahlen aus C8 gelten unverändert als Abnahmekriterien. Der Vorbehalt "warten auf Bestätigung" ist aus dem Spec verschwunden. Neu ist eine Zusage, die vorher nur in der Empfehlung des Entscheidungsdatensatzes stand: zeigt der Technologievergleich, dass eine Zahl keinen tragfähigen Kandidaten übrig lässt, wird sie über einen neuen Entscheidungsdatensatz abgelöst und nicht stillschweigend gelockert.

**Umbenennen im Stapel.** Der Nutzer hat Möglichkeit 2 gewählt und dabei Suchen und Ersetzen im Namen, fortlaufende Nummerierung und eine Vorschau vor der Ausführung genannt. Die Groß- und Kleinschreibung, die Möglichkeit 2 des Datensatzes zusätzlich aufführte, hat er nicht genannt; sie steht jetzt unter "Nicht in dieser Runde". Eine Umschaltung der Schreibweise lässt sich über Suchen und Ersetzen nicht ausdrücken, wäre also eine eigene Regelart, und sie ohne Auftrag mitzunehmen wäre eine Ausweitung des Umfangs.

## Die eine Festlegung, die der Shaper selbst getroffen hat

Zur Rückfrage vor dem endgültigen Löschen hat der Nutzer nichts gesagt. Der Shaper hat festgelegt: Fn+F8 fragt genau einmal je Vorgang nach, mit Abbrechen als Vorbelegung der Rückfrage, damit ein reflexhaftes Bestätigen mit der Return-Taste nichts löscht.

Die Begründung ergibt sich aus der Antwort selbst. Der Nutzer hat den alltäglichen und den unwiderruflichen Weg auf zwei verschiedene Tasten gelegt, und nur der zweite hat keinen Rückweg. Ein eigener Rückgängig-Speicher scheidet aus, weil er im Kern ein zweiter Papierkorb wäre und gegen die Maxime "supersimpel" liefe. Als Sicherung bleibt allein die Rückfrage. Ihr Preis ist ein Tastendruck je Vorgang, nicht je Eintrag, und sie bremst die Tastaturarbeit nicht, weil das alltägliche Löschen über Delete ohne jede Rückfrage läuft. Der Nutzer kann die Festlegung bei der Durchsicht des Specs umstoßen.

## Abweichung zur Circle-Directive

Die Antwort zum Löschen widerspricht dem Wortlaut der Directive. Deren letzter Satz nennt "Shift+Delete zum Löschen"; die Antwort nennt Delete und Fn+F8 und lässt Shift+Delete weg. Derselbe Satz nennt außerdem "F3 bis F8", während die Antwort auf die Tastenfrage nur die Fn-Kombination belegt.

Verbindlich ist die Antwort des Nutzers. Der Shaper hat die Directive nicht angefasst, weil der in-Circle-Klärungsmodus den Circle-Datensatz nicht zur Bearbeitung freigibt. Stattdessen steht die Abweichung an zwei Stellen sichtbar: als eigener Abschnitt `## Abweichung zur Circle-Directive` im Spec und als Defekt in `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1105_o_directive-zeile-widerspricht-loeschantwort.md`. Der Defekt schlägt einen neuen Wortlaut für den betroffenen Satz vor. Die Entscheidung, die Zeile zu bestätigen oder zu korrigieren, trifft der Nutzer am Plan-Gate.

## Fortgeschriebene Entscheidungsdatensätze

Drei sind auf `_a_` gelaufen, jeweils mit einem Abschnitt "Antwort des Nutzers" im Text und einer `Answered:`-Zeile mit Pfad und Zeilenbereich der Spec-Stelle.

- `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`
- `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`
- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_umbenennen-im-stapel-umfang.md`

Einer bleibt auf `_o_`: `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_o_leistungszusagen-navigator.md`. Der Datensatz ist neu geschrieben. Sein erledigter Teil, die zehn Zahlen, steht als eigener Abschnitt am Anfang. Die eigentliche Frage ist auf das Referenzgerät eingeschärft und trägt drei Möglichkeiten: der Entwicklungs-Mac mit vollständiger Angabe, ein benanntes Mindestgerät, oder beides mit getrennten Rollen. Zu benennen sind Modell, Baujahr, Prozessor, Bildwiederholrate des Bildschirms und Arbeitsspeicher. Die Bildwiederholrate ist gesondert aufgeführt, weil L1 und L9 mit 16 ms den konservativen 60-Hz-Wert nennen; auf einem 120-Hz-Gerät bleibt die Zusage bestehen, die Aussage "eine Reaktion je Bild" trifft dann aber nicht mehr zu.

## Neu angelegte Defekte

Beide entstanden aus dieser Runde und liegen deshalb nach der Herkunftsregel im Circle.

- `issues/260802-1105_o_directive-zeile-widerspricht-loeschantwort.md`, zur Abweichung oben.
- `issues/260802-1105_o_beantwortete-entscheidungen-noch-als-offen-gefuehrt.md`. `CLAUDE.md` und der Circle-Datensatz behaupten beide, fünf Entscheidungen seien unbeantwortet, und nennen zwei Pfade, die nach der Umbenennung ins Leere zeigen. `portfolio.md` nennt dieselben alten Pfade, heilt sich aber selbst, weil der Playmaker die Datei bei jedem Lauf neu erzeugt. Die Historiendateien nennen sie ebenfalls und bleiben unangetastet, weil sie den Stand ihres Zeitpunkts festhalten.

## Warum der Spec auf `_o_` bleibt

Der Marker `_p_` bedeutet nach `rules/fusion-workbench-conventions.md`, dass ein Agent gerade an der Datei arbeitet. Das trifft nach dieser Runde auf niemanden zu; der Spec wartet auf die Abnahme am Plan-Gate. Dazu kommt ein praktisches Argument: das Feld `**Active spec/plan:**` im Circle-Datensatz zeigt auf den Dateinamen mit `_o_`, und dieses Feld darf der Shaper im in-Circle-Klärungsmodus nicht ändern. Eine Umbenennung würde den Verweis brechen. Dasselbe gilt für die Verweise aus beiden Entscheidungsdatensätzen im Circle.

## Nicht angefasst in dieser Runde

Der Circle-Datensatz, `CLAUDE.md`, `portfolio.md`, die drei weiterhin offenen Entscheidungsdatensätze im geteilten Speicher und die Historiendateien anderer Sitzungen. Kein Code, kein Plan, keine Technologiefestlegung, kein Commit.

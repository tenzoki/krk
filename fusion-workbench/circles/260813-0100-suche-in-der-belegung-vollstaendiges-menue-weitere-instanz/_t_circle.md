# Jeder Befehl von KRK ist auf drei Wegen erreichbar, und eine weitere Instanz teilt die Ablage ohne sie zu zerstoeren

---
**Domain:** code
**Status:** active
**Filed by:** orchestrator (nach dem Spec des shaper)
**Active spec/plan:** shared/planning/260813-0053_*_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md
**Active session history:** shared/history/260813-0040-orchestrator-session.md

---

## Directive

Nach dieser Runde ist jeder Befehl von KRK auf drei Wegen erreichbar statt auf einem. Die Belegungsansicht wird durch Tippen durchsucht: jedes Zeichen hängt an einen Suchtext an, die Auswahl springt sofort auf den ersten Treffer, die Eingabetaste geht zum nächsten. Das Hauptmenü führt alle Funktionen der Belegung, gegliedert nach denselben neun Funktionsbereichen, die Belegungsansicht und Markdown-Ausgabe schon zeigen, jede mit ihrem Kürzel aus der Belegung und ausgegraut, wo sie gerade nicht wirkt. Und ein Tastenbefehl startet eine weitere Instanz von KRK, die sich Lesezeichen und Tastenbelegung mit der ersten teilt, ohne dass eine von beiden die Arbeit der anderen überschreibt.

Diese Runde setzt keine elfte Zeitzusage und fasst keine der zehn an.

## Grounding snapshot

Erhoben am 260813 am Baum, Stand `188b81a`. Der Spec traegt die vollstaendige Ausgangslage unter
`## Ausgangslage, am 260813 am Baum erhoben`; hier stehen die drei Befunde, die den Zuschnitt
tragen.

**Die Ausgrauung im Menue ist eine Korrektheitsbedingung und keine Politur.** Der Ereignisabgriff
reicht einen wegen des Fokus abgewiesenen Befehl unveraendert an AppKit weiter. Ein Menueeintrag
mit Kuerzel fuehrt ihn dann doch aus: mit dem Fokus im Editor bewegt ein Auf-Pfeil die Dateiliste
statt der Schreibmarke. Daraus wird die tragende Regel dieser Runde, eine Zulaessigkeitsfrage mit
zwei Fragern.

**Zwei Instanzen koennen eine Ablagedatei nicht nur ueberschreiben, sondern beschaedigen.**
`atomar::nachbarpfad` leitet den Namen der Nachbardatei fest ab und traegt bewusst keine
Laufnummer. Beide Prozesse benutzen dieselbe Nachbardatei, und das `rename` veroeffentlicht ein
Gemisch. Die Runde 6 faengt die Folge auf und verhindert die Ursache nicht.

**Das Menue fuehrt heute zehn Befehle**, nicht rund zwanzig: zwei im Anwendungsmenue, sechs unter
Bearbeiten, zwei unter Fenster. Die Belegung fuehrt 81 Funktionen.

## Dependencies

Keine auf einen anderen Circle. Die Runde baut auf dem Stand nach der Runde 6 auf und faellt
deren Entscheidungen nicht um.

Gebunden ist sie an vier offene Fragen im gemeinsamen Speicher, alle vom shaper am 260813-0053
angelegt, alle mit Kosten und Empfehlung. Die Runde ist ohne ihre Beantwortung zuschneidbar und
faehrt auf den Empfehlungen:

- `shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`
- `shared/decisions/260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`
- `shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`
- `shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`

## Turn log

(noch keiner)

## Closure note

(offen)

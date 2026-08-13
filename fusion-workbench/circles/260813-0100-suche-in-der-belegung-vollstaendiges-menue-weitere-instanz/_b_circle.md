# Jeder Befehl von KRK ist auf drei Wegen erreichbar, und eine weitere Instanz teilt die Ablage ohne sie zu zerstoeren

---
**Domain:** code
**Status:** bounded
**Filed by:** orchestrator (nach dem Spec des shaper)
**Active spec/plan:** circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/planning/260813-0205_*_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md (Spec: shared/planning/260813-0053_*_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md)
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

- Turn 1 (Sitzung 260813-0040): Commits ca66c39..a34bf17. Alle fuenfzehn Planschritte auf [DONE], der Baum gruen ueber 19 Ziele mit 1000 Proben. Coherence-Urteil: ok. Durchsichten: reviews/260813-0532-ontorev-belegungsdatei-weitere-instanz.md und reviews/260813-0540-coderev-turn-1-runde-7-rust-anteil.md, Bereich ca66c39..40b5fb0. 18 Defekte abgelegt, keiner kritisch oder hoch. Vor der Ausfuehrung haben drei Diagrammpruefungen ein Loch in der tragenden Regel, eine falsche Zaehlprobe und zwei falsche Verweismengen gefunden. Sitzungsprotokoll: shared/history/260813-0040-orchestrator-session.md

- Turn 2 (Sitzung 260813-0040): Commits a34bf17..dff167a. Ein Reparatur-Turn ohne Planschritte: achtzehn der zweiundzwanzig Durchsichtsbefunde behoben, 1003 Proben ueber 19 Ziele. Der schwerste war ein Loch im eigenen Mechanismus der Runde, der Messmodus schrieb die Sitzung ohne Sitzungsrecht; behoben am Typ. Drei Befunde teilten eine Ursache, und ihre Behebung hat die Bauform der Zaehlproben dieses Projekts geaendert: nach dem Gegenstand suchen statt nach dem Namen. Vier Datensaetze bleiben offen, einer davon gehoert dem Nutzer. Coherence-Urteil: ok. Sitzungsprotokoll: shared/history/260813-0040-orchestrator-session.md

## Closure note

**Beschraenkter Abschluss am 260813.** Alle fuenfzehn Planschritte sind gebaut, alle vier
Faehigkeiten stehen im Baum, und die Runde ist trotzdem nicht abgenommen. Der Grund ist
derselbe wie bei den sechs Runden davor und keine Haeufung von Fehlschlaegen: **der Abnahmelauf
verlangt KRK im Vordergrund und ist damit Nutzerarbeit.**

**Was der Artefakt traegt.** Die Belegungsansicht wird durch Tippen durchsucht, jedes Zeichen
haengt an, die Auswahl springt sofort, die Eingabetaste geht zum naechsten Treffer. Alle 82
Funktionen stehen im Menue, in neun Obermenues, jede mit ihrem Kuerzel und ausgegraut, wo sie
gerade nicht wirkt. `opt+cmd+n` startet eine weitere Instanz, und zwei Sperren ueber `flock`
verhindern, dass zwei Instanzen einander die Ablage zerlegen. 18 Commits, 1003 Proben ueber 19
Ziele, alle vier Abnahmekommandos auf Exit 0.

**Der Nebengewinn wiegt schwerer als eine der vier Faehigkeiten.** Der dritte Wunsch hat einen
Fehler ans Licht gebracht, den vorher niemand kannte: ein Menueeintrag mit Kuerzel fuehrte einen
Befehl aus, den die Fokuspruefung gerade abgewiesen hatte. Mit dem Fokus im Editor bewegte ein
Auf-Pfeil die Dateiliste statt der Schreibmarke. Das ist weg, und nebenbei kennt der
Ereignisabgriff den Editor nicht mehr — die Kopplung, vor der `CLAUDE.md` seit Runden warnt.

**Was diese Runde ueber sich selbst gelernt hat, und das ist der uebertragbare Teil.** Dreimal
hat eine Pruefung denselben Fehlertyp gefunden: **ein Text sagt mehr zu, als seine Probe haelt.**
Erst ein Loch in der tragenden Regel des Spec, das auf dem Weg ueber den Fokusvorbehalt nie
gepruefte wurde. Dann eine Zaehlprobe, die drei Aufrufstellen verlangte, wo der Entwurf zwei
ergab — und deren naheliegende Reparatur der Doppelbau gewesen waere, den die Runde beseitigen
sollte. Zuletzt die Behauptung, es gebe in Rust genau zwei Wege an eine fremde Funktion; zwei
uebersetzte Gegenbeispiele haben sie widerlegt. Die Bauform der Zaehlproben dieses Projekts ist
daraufhin geaendert: **nach dem Gegenstand suchen statt nach dem Namen**, und was keine Suche
fangen kann, steht am Doc-Kommentar statt im Namen der Probe.

**Drei Pruefungen vor der ersten Zeile Code** haben sich bezahlt gemacht: zwei am Spec, eine am
Plan. Ohne sie waere das Loch in der Zulaessigkeitsregel in fuenfzehn Planschritte eingegangen.

**Was am Buendel aussteht**, und damit bei dir: die Kriterien, die im Spec **(Buendel)** tragen,
darunter ob `Cmd+T` und `Cmd+R` die Schaltflaechen der F1-Ansicht erreichen. Dazu zwei bewusst
hingenommene Verluste gegenueber heute, beide abgeleitet und nicht gemessen: `esc` im Editor
bricht keine Zusammensetzung einer Eingabemethode mehr ab, und ein Klick in die Bereichsleiste
wirkt waehrend einer Umbenennung nicht.

**Warum hier Schluss ist und nicht nach dem naechsten Turn.** Jede Durchsicht dieser Runde hat
etwas gefunden, auch die dritte und die vierte. Die verbliebenen acht Datensaetze sind
saemtlich von derselben Art — eine Zusage ist weiter als ihre Wache — und keiner betrifft das
Verhalten. Sie sind benannt, begruendet und binden die naechste Runde. Weiterzureparieren
verspraeche eine Vollstaendigkeit, die dieselbe Runde dreimal als Trugschluss vorgefuehrt hat.

**Sitzungsprotokoll:** `shared/history/260813-0040-orchestrator-session.md`
**Abgleich:** `history/260813-0647-reconciliation.md`
**Durchsichten:** drei am Entwurf (`260813-0109`, `260813-0144`, `260813-0220`), vier an der
Ausfuehrung (`260813-0532`, `260813-0540`, `260813-0725`).

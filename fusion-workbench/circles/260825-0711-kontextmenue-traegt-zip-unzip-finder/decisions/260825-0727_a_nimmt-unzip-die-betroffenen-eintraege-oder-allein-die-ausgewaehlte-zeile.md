# Nimmt Unzip die betroffenen Einträge oder allein die ausgewählte Zeile?

---
**Domain:** code
**Filed by:** planner
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_t_circle.md` (Directive, Unzip-Teil); `crates/krk-ui/src/kommandos/operationen.rs:167` (`betroffene`); `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/decisions/260825-0711_*_woran-erkennt-unzip-dass-eine-datei-ein-zip-ist.md`

---

## Question

Für Zip hat der Nutzer die bestehende Regel `kommandos::operationen::betroffene` gewählt: die
markierten Einträge, ersatzweise die ausgewählte Zeile. Für Unzip sagt die Directive etwas
anderes, nämlich „die ausgewählte Datei, wenn sie ein Zip ist, sonst das eine Zip des angezeigten
Ordners". Solange nichts markiert ist, fallen beide Formulierungen zusammen, denn `betroffene`
weicht dann auf ebendiese ausgewählte Zeile aus. Sie fallen auseinander, sobald etwas markiert
ist: `betroffene` liefert dann die Markierung und ignoriert die Auswahl, während „die ausgewählte
Datei" die Markierung ignoriert.

Der Fall ist nicht ausgedacht. Wer drei Archive markiert und mit rechts auf eines davon klickt,
bekommt nach der einen Lesart drei Einträge und nach der anderen einen; und der Rechtsklick auf
eine markierte Zeile rückt die Auswahl ausdrücklich **nicht** nach
(`operationen::rechtsklick_zielzeile`), sodass die Auswahl in dieser Lage irgendwo anders stehen
kann als der Klick.

Die Frage muss vor dem Bau beantwortet sein, weil sie entscheidet, ob im Baum eine zweite
Auswahlregel neben `betroffene` entsteht. Genau das hat der Nutzer für Zip abgelehnt, und das
Kontextmenü trägt beide Befehle nebeneinander.

## Options

1. **Unzip nimmt `betroffene` und verlangt genau einen Eintrag** — sind mehrere betroffen,
   entpackt Unzip nichts und meldet es in der Statuszeile; ist genau einer betroffen und ein
   Archiv, ist er das Archiv; sonst gilt die Ersatzregel „das eine Zip des angezeigten Ordners".
   - Pro: Eine Auswahlregel für beide Befehle des Kontextmenüs, und es ist die bestehende. Bei
     leerer Markierung liefert sie wörtlich das, was die Directive beschreibt.
   - Contra: Wer drei Archive markiert hat und eines entpacken will, bekommt eine Meldung statt
     einer Handlung. Ein Mehrfach-Entpacken wäre der naheliegende Wunsch, und diese Möglichkeit
     weist ihn ab, statt ihn zu erfüllen.
2. **Unzip liest allein `Ordnermodell::auswahl_zeile` und sieht die Markierung nicht** — genau
   der Wortlaut der Directive.
   - Pro: Trifft die Directive ohne Auslegung. Eine Markierung stört das Entpacken nicht.
   - Contra: Eine zweite Auswahlregel im Baum, und die zwei Einträge desselben Menüs wirken auf
     verschiedene Mengen: Zip packt die drei markierten, Unzip entpackt das eine ausgewählte. Wer
     das nicht weiß, sieht es dem Menü nicht an. Dazu kann die Auswahl nach einem Rechtsklick auf
     eine markierte Zeile anderswo stehen als der Klick.
3. **Unzip nimmt `betroffene` und entpackt jedes Archiv darin, nacheinander** — drei markierte
   Archive ergeben drei Zielordner in einem Vorgang.
   - Pro: Eine Auswahlregel, und der Befehl erfüllt den Wunsch, statt ihn abzuweisen. Die
     Vorgangsmaschine läuft ohnehin Quelle für Quelle; mehrere Archive sind für sie derselbe
     Bauplan wie mehrere Kopierquellen.
   - Contra: Geht über die Directive hinaus, die von **dem** Archiv im Singular spricht. Der
     Zielordner-Konflikt aus dem Datensatz `260825-0711_*_was-tut-unzip-wenn-der-zielordner-schon-
     dasteht` stellt sich dann je Archiv und nicht einmal.

## Constraints

`betroffene` ist die eine Antwort auf „worauf wirkt ein Befehl" und hat heute sieben Abnehmer;
der Nutzer hat sie in Runde 1 ausdrücklich gewählt und den Gegenentwurf verworfen. Eine zweite
Regel daneben ist möglich, muss aber begründet sein, und die Begründung müsste tragen, warum zwei
Einträge desselben Menüs auf verschiedene Mengen wirken.

Gezählt werden bei `betroffene` allein die **sichtbaren** Einträge, in Sichtreihenfolge. Steht ein
Filtertext, sieht Unzip damit dieselbe Liste, die der Nutzer vor sich hat — und die Ersatzregel
„das eine Zip des angezeigten Ordners" ist aus demselben Grund über die sichtbaren Zeilen zu
rechnen und nicht über den ungefilterten Bestand.

## Recommendation

Möglichkeit 1. Sie hält die eine Auswahlregel, liefert im häufigen Fall (nichts markiert) wörtlich
das Verhalten der Directive, und ihr Preis ist eine Meldung in der Statuszeile statt einer
Handlung — also genau die Form, die die Directive für „nichts vorgefunden" ohnehin vorsieht.

Möglichkeit 3 ist die eigentlich reizvolle und bleibt der spätere Ausbau: sie kostet keinen neuen
Mechanismus, sondern nur die Erlaubnis, über die Directive hinauszugehen. Wer sie jetzt will,
sollte zugleich sagen, ob der Zielordner-Konflikt je Archiv gefragt wird oder einmal für alle.

---
Answered: shared/history/260824-2120-orchestrator-session.md:35 — Moeglichkeit 3, betroffene nehmen und jedes Archiv darin entpacken; der Zielordner-Konflikt wird je Archiv gefragt, mit dem Ankreuzfeld fuer alle weiteren.
Implemented:
Deferred:
Superseded by:
Retired:

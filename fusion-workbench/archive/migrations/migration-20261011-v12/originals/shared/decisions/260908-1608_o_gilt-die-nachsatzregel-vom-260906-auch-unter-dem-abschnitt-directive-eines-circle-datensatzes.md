# Gilt die Nachsatzregel vom 260906 auch unter dem Abschnitt `## Directive` eines Circle-Datensatzes?

---
**Domain:** Werkbankführung
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:**
`260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`
(die Regel, deren Reichweite hier zu bestimmen ist),
`260906-0509_*_fuenf-datensaetze-der-klasse-s-brauchen-einen-nachsatz-ausserhalb-von-circles-planning.md`
(der Befund, der die Frage stellt),
`260814-0637_*_die-directive-im-circle-datensatz-nennt-drei-sicherungsmomente-der-spec-vier.md`,
`260814-1002_*_die-directive-abweichung-steht-an-drei-stellen-des-circle-datensatzes-und-nicht-an-zwei.md`,
`260815-1047_*_die-directive-der-runde-10-und-ein-planschritt-schreiben-das-alte-leeren-weiter-fest.md`
(die drei Datensätze, die allein daran hängen)

---

## Question

Der Nutzer hat am 260906 entschieden, dass ein Agent eine falsche Angabe im Spec oder Plan einer
geschlossenen Runde als **Nachsatz** berichtigen darf. Die vierte Behebungsschleife hat damit 30
von 36 Datensätzen geschlossen. **Drei sind übrig, und bei allen dreien steht die falsche Angabe
unter `## Directive` oder unter `## Grounding snapshot` eines Circle-Datensatzes.**

Die Entscheidung vom 260906 spricht von Spec und Plan und sagt über die Directive nichts.
`agents/shaper.md` `## Scope` weist den Abschnitt `## Directive` dem Shaper zu, und nur im Modus
`portfolio-activation`. Das ist keine Frage des Pfades, sondern der Zuständigkeit, und niemand
hat sie beantwortet.

**Der Ausweg, den der Bestand heute nimmt, hat niemand gewählt.** Für `260815-1047` hat der
Orchestrator die Berichtigung ersatzweise in die Schließungsnotiz derselben Datei geschrieben,
weil er den Abschnitt nicht anfassen darf. Die Auskunft steht damit an einer Stelle, an der sie
niemand sucht, und der Datensatz bleibt offen. Für die zwei Datensätze der Runde 9 ist auch das
nicht geschehen; sie stehen seit dem 260814 unverändert.

**Die Frage ist jetzt fällig**, weil sie sonst je Datensatz einzeln beantwortet wird und die
drei sonst dauerhaft offen bleiben. Ihr Gegenstand ist in allen drei Fällen unstrittig und am
Baum nachgemessen.

## Options

1. **Die Regel vom 260906 gilt unverändert weiter, also auch unter `## Directive`.** Ein
   ausführender Agent hängt den datierten Nachsatz an die Stelle, der Wortlaut darüber bleibt
   Zeichen für Zeichen stehen.
   - Pro: eine Regel statt zweier, entscheidbar nach der Sache (eine falsche Angabe in einem
     freigegebenen Text) und nicht nach dem Abschnitt. Die drei Datensätze werden schließbar,
     ohne dass ein Shaper-Lauf angesetzt werden muss. Die Abnahmenotiz jener Runden behält
     ihren Bezug, weil nichts überschrieben wird.
   - Contra: `agents/shaper.md` gehört dem Rahmenwerk, und dieses Projekt kann es nicht ändern.
     Die Zuweisung des Abschnitts an den Shaper würde faktisch unterlaufen, auch wenn nichts
     im Bestandstext angetastet wird.
2. **Nur der Shaper darf es, im Modus `portfolio-activation`.** Die drei Datensätze bleiben
   offen, bis ein solcher Lauf angesetzt wird.
   - Pro: die Zuständigkeitsregel des Rahmenwerks bleibt unangetastet, und die Directive bleibt
     das, was sie sein soll — die Aussage, gegen die der Abschluss einer Runde gelesen wird.
   - Contra: die drei Runden sind geschlossen, und `portfolio-activation` ist der Modus für die
     Aktivierung eines vorgesehenen oder laufenden Circles. Für einen `_b_`-Datensatz gibt es
     diesen Lauf gar nicht mehr; die Möglichkeit ist damit womöglich leer, und die drei bleiben
     dauerhaft offen.
3. **Die Berichtigung geht in die Schließungsnotiz, und das wird zur Regel statt zum Ausweg.**
   Der Abschnitt `## Directive` bleibt unberührt; die Schließungsnotiz nennt jede seither
   überholte Aussage mit Datum und Grund. Der Orchestrator darf sie schreiben.
   - Pro: nichts am Rahmenwerk wird unterlaufen, die drei Datensätze werden schließbar, und der
     Weg ist im Bestand schon einmal gegangen worden.
   - Contra: die Auskunft steht dreißig bis achtzig Zeilen unter der falschen Angabe, und wer
     die Directive liest, liest die Schließungsnotiz nicht. Genau das hat `260815-1047`
     festgehalten, als es diesen Weg gehen musste, und deshalb ist es offen geblieben.

## Constraints

- Die Antwort muss nach dem **Ort** entscheidbar bleiben, wie die Ortsregel in `CLAUDE.md` und
  die Nachsatzregel vom 260906 es sind.
- Sie darf keine Runde nachträglich anders abgenommen aussehen lassen, als sie abgenommen wurde.
- Sie darf keinen Bestandstext überschreiben; das steht seit dem 260906 fest und wird hier nicht
  wieder aufgemacht.
- `agents/shaper.md` und `rules/circle-records.md` gehören dem Rahmenwerk und können von hier
  aus nicht geändert werden.

## Recommendation

Keine. Die Wahl steht zwischen einer Regel des Rahmenwerks und drei dauerhaft offenen
Datensätzen, und welche der beiden schwerer wiegt, ist eine Nutzerfrage. Was diese Frage
entscheidbar macht, ist eine Auskunft, die hier fehlt: **ob ein Shaper-Lauf im Modus
`portfolio-activation` an einem geschlossenen Circle-Datensatz überhaupt möglich ist.** Ist er
es nicht, fällt Möglichkeit 2 aus, und die Frage steht zwischen 1 und 3.

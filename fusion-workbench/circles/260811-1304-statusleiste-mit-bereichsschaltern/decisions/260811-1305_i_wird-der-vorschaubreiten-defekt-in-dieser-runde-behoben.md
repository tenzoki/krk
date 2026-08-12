# Wird der Vorschaubreiten-Defekt in dieser Runde behoben oder in einer eigenen davor?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `shared/issues/260811-1245_*_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`, `crates/krk-ui/src/fenstermodell.rs:519` (`breiten_uebernehmen`), `crates/krk-ui/src/appkit/aufteilung.rs`

---

## Question

Der Nutzer hat den Defekt am 260811-1240 gemeldet: wer das Vorschaufenster breiter zieht und danach in der Dateiliste navigiert, findet es auf seiner alten Breite. Der Entwurf dieses Circles nennt ihn selbst und sagt, er sei "vorher oder mit dieser Runde zu klären". Der Defektdatensatz begründet, warum die Reihenfolge zählt: eine proportionale Neuaufteilung auf einer Grundlage, die die Ziehbewegung des Nutzers nicht hält, verteilt die falschen Anteile.

Zu klären ist nicht das Ob, sondern das Wo: eine eigene kleine Runde vor dieser, oder die erste Fähigkeit dieser Runde.

## Options

1. **Erste Fähigkeit dieser Runde.** Der Defekt wird gemessen und behoben, bevor die proportionale Regel entsteht, aber im selben Circle.
   - Pros: Der Defekt und die neue Regel liegen in derselben Funktion. Wer beide nacheinander in einem Circle anfasst, schreibt `bereichsbreiten` und seine Proben einmal um statt zweimal.
   - Cons: Der Circle trägt eine Fähigkeit, die nichts Neues liefert, und der Nutzer wartet auf die Behebung, bis die ganze Runde steht.
   - **Folgen weiter unten:** Der Defekt zieht aus `shared/issues/` in diesen Circle um, sobald er aktiviert ist. Nach der Herkunftsregel bleibt er allerdings dort, wo er entstanden ist: er wurde ohne diesen Circle gemeldet. Er wird also zitiert und nicht verschoben, und die Runde schließt ihn an seinem Ort.

2. **Eigene kleine Runde davor.** Der Defekt bekommt einen eigenen Circle, wird gemessen, behoben und geschlossen; dieser Circle hängt an ihm.
   - Pros: Der Nutzer bekommt die Behebung früher, und sie ist unabhängig davon, wie die Frage nach der proportionalen Regel ausgeht. Der Messschritt, den der Defektdatensatz verlangt, steht dann für sich und wird nicht in eine größere Runde gezogen.
   - Cons: `bereichsbreiten` oder der Weg dorthin wird zweimal angefasst.
   - **Folgen weiter unten:** Dieser Circle bekommt eine harte Vorbedingung und kann erst danach aktiviert werden. Das Portfolio trägt einen weiteren vorgesehenen Circle.

3. **Nach der Behebung entscheiden.** Zuerst wird nur gemessen, welche der beiden im Defektdatensatz genannten Bruchstellen es ist; danach steht fest, wie groß die Behebung ist, und die Zuordnung folgt daraus.
   - Pros: Die Entscheidung fällt auf gemessener Grundlage statt auf einer Vermutung über den Umfang.
   - Cons: Die Messung selbst braucht KRK im Vordergrund und ist damit Nutzerarbeit, wie der Abnahmelauf.
   - **Folgen weiter unten:** Die Aktivierung dieses Circles wartet auf eine Handlung des Nutzers. Der Aufwand ist klein: eine Trennlinie ziehen, in der Liste navigieren und nachsehen, ob `Breiten::vorschau` in `session.toml` den neuen oder den alten Wert trägt.

## Constraints

- Der Defektdatensatz verlangt ausdrücklich, zuerst zu messen, welche der beiden Bruchstellen es ist, statt eine zu vermuten. Die beiden: entweder löst ein Lesevorgang in der Dateiliste eine Neuaufteilung aus, obwohl er weder Breite noch Sichtbarkeit ändert, oder die Ziehbewegung kommt im Fenstermodell nie an.
- Trifft die zweite Bruchstelle zu, war die Zusage aus C7 "Breiten überleben Beenden und Neustart" an dieser Stelle nie erfüllt, und der Umfang der Behebung ist ein anderer.
- Die Herkunftsregel lässt den Defekt in `shared/issues/`, wo er gemeldet wurde; er wird zitiert und nicht verschoben.

## Recommendation

**Möglichkeit 3, danach voraussichtlich Möglichkeit 1.** Die Messung kostet den Nutzer zwei Minuten und beantwortet die Frage, die der Defektdatensatz stellt. Fällt sie auf die erste Bruchstelle, also auf eine überflüssige Neuaufteilung beim Lesen, liegt die Behebung in derselben Maschinerie wie die neue Regel und gehört in dieselbe Runde. Fällt sie auf die zweite, also auf eine Ziehbewegung, die das Modell nie erreicht, ist die Behebung größer und eigenständiger, und eine eigene Runde davor wird plausibel.

Eine Empfehlung vor der Messung wäre eine Vermutung über den Umfang und keine Antwort.

---
Answered: `shared/issues/260811-1245_c_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`,
Abschnitt `## Behebung 260811-2130 (Commit 1ea5a3d)` — **die Lage hat geantwortet, nicht der
Nutzer.** Der Defekt ist am 260811 in der Runde 4 gemessen und behoben worden, also in einer
eigenen Runde vor diesem Circle; das ist im Ergebnis Moeglichkeit 2. Gemessen war es Bruchstelle 1:
`kommando_ausfuehren` rief `aufteilung_nachziehen()` nach jedem Befehl, bevor jemand die gezogene
Breite nachmass. Die Behebung ist `bildschirmbreiten_uebernehmen()` am Kopf von
`kommando_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:2048`, Funktion bei `:2577`), am Baum
gelesen im Abgleich
`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/history/260811-2157-reconciliation.md`.
Bruchstelle 2 trifft nicht zu, C7 war also nie gebrochen.

**Was das fuer die Aktivierung dieses Circles heisst:** die harte Vorbedingung ist weg, die
siebte Frage des Circles ist gegenstandslos, und die proportionale Regel entsteht auf einer
Grundlage, die die Ziehbewegung des Nutzers haelt. Ob dieser Datensatz damit auf `_i_` geht oder
als ueberholt gilt, entscheidet, wer den Circle aktiviert; der Marker steht deshalb auf
beantwortet und nicht auf umgesetzt.

**Ein Beifund aus der Behebung, der diesen Circle angeht:** `MINDESTGROESSE` (`fenster.rs`) steht
auf 780 Punkten, der Vierersatz mit dem Editor summiert sich auf 920. Zwischen 780 und 920 Punkten
Fensterbreite wird der Editor unter sein Mindestmass gedrueckt. Wer die Bereichsschalter baut,
trifft diese Zahl.
Implemented: 1ea5a3d (Runde 4, vor dieser Runde) — die Lage hat geantwortet. Dieser Circle hat die Behebung vorgefunden und nicht gebaut.
Deferred:
Superseded by:

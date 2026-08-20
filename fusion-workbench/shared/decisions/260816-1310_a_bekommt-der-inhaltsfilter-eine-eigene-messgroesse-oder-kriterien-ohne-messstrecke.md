# Bekommt der Inhaltsfilter eine eigene Messgröße, oder Kriterien ohne Messstrecke?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`, Abschnitt `### C8` (die zehn Zusagen); `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md:487-507` (die Runde, die statt einer elften Zahl zwei prüfbare Kriterien gesetzt hat); `crates/krk-bench/src/fixture.rs:22-31` und `:42` (die dünnbesetzten Prüfordner); `crates/krk-ui/src/messmodus.rs:820` (L1 misst zwanzig Pfeil-ab); `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md`

---

## Question

Der Inhaltsfilter führt eine Arbeit ein, die es in KRK bisher nicht gibt: das Lesen vieler Dateien auf der Bedienstrecke. Ob dafür eine Zahl zugesagt wird, ist zu entscheiden, und zwei am Baum geprüfte Befunde schneiden die Frage anders zu, als sie zunächst aussieht.

**Erster Befund: keine der zehn Zusagen deckt das Tippen.** L1 misst „Tastendruck bis die Auswahl im Dateifenster sichtbar umspringt", und der Messmodus setzt dafür zwanzig Pfeil-ab-Ereignisse ab (`messmodus.rs:820`, `// L1: zwanzig Pfeil ab im aktiven Dateifenster`). Ein getipptes Zeichen fällt dort nicht hinein. Der Namensfilter der Runde 10 ist deshalb schon ungemessen, und der Inhaltsfilter erbt diese Lage. Er fällt unter keine der zehn Zahlen, und die Frage lautet nicht „unter welche", sondern „ob eine elfte".

**Zweiter Befund: die vorhandene Messstrecke kann Inhalt nicht messen.** Die drei Prüfordner entstehen dünnbesetzt: je Datei werden 512 Bytes wirklich geschrieben, der Rest ist ein Loch (`fixture.rs:42`). Der Modulkopf warnt ausdrücklich davor, sie für eine Messung zu verwenden, die tatsächliche Bytes bewegt (`fixture.rs:22-31`, dort für L8 gesagt). Ein Inhaltsdurchlauf über den Ordner mit 100.000 Einträgen läse dort fast nichts und ergäbe eine Zahl, die mit dem Gebrauch nichts zu tun hat. Eine elfte Zusage braucht deshalb **zuerst einen vierten Prüfordner mit echtem Inhalt**, und der kostet Plattenplatz in der Größenordnung mehrerer Gigabyte sowie eine eigene Erzeugungsvorschrift, die bei gleicher Eingabe dieselbe Zusammensetzung liefert.

**Was das Projekt bisher getan hat**, und zwar zehn Runden lang: keine elfte Zahl. Die Runde 2 hat den Fall ausgeschrieben und statt einer Zahl zwei ohne Messstrecke prüfbare Kriterien gesetzt, mit der Begründung, eine Zusage, die eine Runde nicht messen kann, sei kein Abnahmekriterium, sondern ein Wunsch.

## Options

1. **Keine elfte Zahl; stattdessen Kriterien, die ohne Messstrecke prüfbar sind.** Die Form der Runde 2, angewandt auf den Inhaltsfilter: die Anwendung bleibt während des Lesens bedienbar, ein Tastendruck bricht das Laufende ab, und keine der zehn Zahlen wird geändert, gelockert oder umgedeutet.
   - Pro: dieselbe Antwort, die dieses Projekt zehnmal gegeben hat, und aus demselben Grund. Die Kriterien sind am laufenden Bündel prüfbar und brauchen keinen neuen Prüfordner. Der Abnahmelauf der Runde bleibt in der Größenordnung, die der Nutzer bisher gefahren hat.
   - Kontra: die einzige Zusage, die den Inhaltsfilter tatsächlich einschränkt, ist „bedienbar bleiben". Wie lange ein Durchlauf dauert, ist damit nirgends zugesagt, und eine spätere Verschlechterung fällt niemandem auf.

2. **Eine elfte Zusage, mit dem dafür nötigen vierten Prüfordner.** Eine Zahl für den Inhaltsdurchlauf über einen Ordner festgelegter Zusammensetzung, gemessen auf dem Referenzgerät.
   - Pro: die einzige Möglichkeit, unter der eine spätere Verschlechterung auffällt. Die Maxime „superschnell" bekäme für die neue Arbeit dieselbe Behandlung, die sie in der Runde 1 für die alte bekommen hat.
   - Kontra: ein vierter Prüfordner mit echtem Inhalt, eine Erweiterung der Messstrecke und ein längerer Abnahmelauf, der ohnehin Nutzerarbeit ist. Und die Zahl wäre gegen einen Sockel gesetzt, dessen zehn Zusagen seit dem 260810 nicht mehr nachgemessen sind, während sechs Runden dazwischenliegen.

3. **Eine elfte Zusage ohne eigenen Prüfordner, gemessen auf einem vom Nutzer benannten wirklichen Ordner.**
   - Pro: kein Plattenplatz für eine Kunstwelt, und gemessen wird an Dateien, die es wirklich gibt.
   - Kontra: bricht die Reproduzierbarkeit, auf der die Messbedingungen aus C8 ausdrücklich bestehen. Zwei Läufe messen zwei verschiedene Ordner, und die zwanzig Wiederholungen tragen dann nichts. Damit ist es keine Zusage, sondern eine Beobachtung.

4. **Die Frage wird zurückgestellt**, bis ein Abnahmelauf der zehn bestehenden Zusagen wieder gefahren ist.
   - Pro: die Reihenfolge stimmt: ein Sockel, dessen letzte Messung vor sechs Runden liegt, trägt keine elfte Zahl. Die Runde 2 hat mit derselben Begründung entschieden.
   - Kontra: der Inhaltsfilter wird dann ohne jede Leistungszusage gebaut und behalten, und die Rückstellung hat in diesem Projekt schon einmal dazu geführt, dass eine Sache der Sache nach entschieden war, ohne dass es jemand aufgeschrieben hätte.

## Constraints

- Eine Zusage, die diese Runde nicht messen kann, ist kein Abnahmekriterium. Die Regel steht im Spec der Runde 1 und ist von der Runde 2 angewandt worden.
- Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; kein Agent kann ihn fahren.
- Die zehn Zahlen aus C8 werden von dieser Runde nicht geändert, nicht gelockert und nicht umgedeutet. Das gilt unabhängig davon, wie diese Frage ausgeht.
- Ein vierter Prüfordner tritt neben drei bestehende und folgt derselben Erzeugungsvorschrift: gleiche Eingabe, gleiche Zusammensetzung.
- Der Messplatz liegt unter `~/Library/Caches/krk-messplatz`. Ein Prüfordner mit echtem Inhalt belegt dort Platz, der nicht im Temporärverzeichnis liegt und sich nicht selbst aufräumt.

## Recommendation

Möglichkeit 1, und die Begründung ist der zweite Befund oben. Solange die Messstrecke Inhalt nicht messen kann, wäre jede elfte Zahl entweder auf einem dünnbesetzten Ordner erhoben und damit falsch, oder sie verlangt zuerst einen vierten Prüfordner und macht damit den Bau der Messvorrichtung zum größeren Teil dieser Runde. Der Einwand gegen Möglichkeit 1 ist echt und soll im Spec stehen statt kleingeredet zu werden: die Dauer eines Inhaltsdurchlaufs ist danach nirgends zugesagt. Wer das ändern will, nimmt Möglichkeit 2 und weiß, dass der Prüfordner die eigentliche Arbeit ist.

---
Answered: `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md`, Abschnitt `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1` — Möglichkeit 1, keine elfte Zusage. Der Nutzer hat am 260816 entschieden, nachdem ihm die Kosten der Gegenvariante vorlagen, insbesondere der vierte Prüfordner mit echten Bytes. An die Stelle einer Zahl treten zwei ohne Messstrecke prüfbare Kriterien, in der Form, die die Runde 2 aufgestellt und die Runde 10 fortgeführt hat. Der Inhaltsdurchlauf ist im Spec als Gegenstand einer späteren Messrunde benannt, zusammen mit dem Befund über die dünnbesetzten Prüfordner.
Implemented:
Deferred:
Superseded by:

---
Abgleich 260820-2056 (reconciler, Baumstand `f5300f4`): **bleibt auf beantwortet, und der Grund ist
nicht Nachlässigkeit, sondern die Form der Antwort.** Am Baum nachgemessen ist die eine Aussage, die
sich messen lässt: `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u` liefert
unverändert L1 bis L10, also ist keine elfte Zahl gesetzt worden.

**`_i_` setzt der Marker trotzdem nicht, weil eine Antwort „wir setzen keine Zahl" keinen
Umsetzungscommit hat, den man zitieren könnte.** `rules/fusion-workbench-conventions.md`,
`## State Markers — decisions`, verlangt für `_i_` ausdrücklich die Zeile
`Implemented: <commit hash> or <path>:<line>`. Eine Abwesenheit hat keine Fundstelle. Die zwei
Ersatzkriterien, die an die Stelle der Zahl treten, sind daneben nur mittelbar über die Bauform des
Durchlaufs eingelöst, und das wäre Erschließung und kein Beleg.

**Dieselbe Lage tragen zwei weitere Datensätze dieses Baums**, und in einem steht sie seit dem
260812-2253 ausgeschrieben: `circles/260812-1000-…/decisions/260812-1000_a_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`
(Antwort: nichts ändern) und `circles/260814-1551-…/decisions/260814-1552_a_wie-kommt-der-nutzer-von-einem-tiefen-treffer-in-dessen-ordner.md`
(Antwort: die Frage löst sich mit der Bauform auf, kein dritter Weg). **Drei Datensätze stehen damit
dauerhaft auf `_a_`, ohne dass irgendetwas aussteht**, und eine Zählung offener Grundlage über
`_o_` + `_a_` meldet sie als offen. Als eigener Befund abgelegt:
`shared/issues/260820-2056_o_drei-beantwortete-datensaetze-koennen-nie-umgesetzt-werden-weil-ihre-antwort-eine-abwesenheit-ist.md`.

# Gilt das Ankreuzfeld „Deep" je Tab oder je Fenster?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/fenstermodell.rs:374` und `:590-598` (`Spaltensichtbarkeit`, der Halter der drei Spaltenschalter, fensterweit und in der Sitzung gespeichert); `crates/krk-ui/src/tabs.rs` (`Tabinhalt`, der Halter je Tab); `crates/krk-ui/src/appkit/bereichsleiste.rs:1-49` (die acht Ankreuzfelder)

---

## Question

Der Filter gehört dem Tab; das steht fest. Für den Schalter, der ihn in die Tiefe dehnt, steht es nicht fest, und die Bereichsleiste liefert kein Vorbild, das die Frage entschiede: ihre acht Felder sind alle fensterweit. Die fünf Bereichsschalter beschreiben die Fensterzeile, die drei Spaltenschalter liegen in `Spaltensichtbarkeit` am `Fenstermodell` und gelten für beide Dateifenster zugleich. „Deep" wäre das erste Feld der Leiste, dessen Gegenstand einem einzelnen Tab gehören könnte.

Die Frage ist sichtbar und nicht bloß eine Frage der Ablage. Sie entscheidet, was der Nutzer sieht, wenn er auf einen Tab wechselt, in dem ein Filtertext steht.

## Options

1. **Je Tab, neben dem Filtertext.** Jeder Tab führt seinen eigenen Stand; die Leiste zeigt den des sichtbaren Tabs und wird beim Tabwechsel nachgezogen.
   - Pro: Schalter und Filter liegen an derselben Stelle und wechseln gemeinsam. Ein Tab, in dem der Nutzer flach filtert, bleibt flach, auch wenn er im Nachbartab tief sucht. Der Nutzer sieht beim Tabwechsel genau die Liste, die er dort verlassen hat.
   - Kontra: die Leiste bekommt ihr erstes Feld mit zwei Bedeutungen je nach sichtbarem Tab, und die Zahl der Stände wächst mit der Zahl der Tabs. Der Stand gehört dann in `session.toml` je Tab oder wird beim Beenden verworfen, was eine eigene kleine Antwort verlangt.
2. **Je Fenster, wie die drei Spaltenschalter.** Ein Stand am `Fenstermodell`, gültig für beide Dateifenster und alle ihre Tabs.
   - Pro: die Leiste behält ihre Bauart, und alle neun Felder beschreiben dasselbe: den Zustand des Fensters. Ein Stand, eine Stelle in der Sitzung.
   - Kontra: ein Tabwechsel auf einen Tab mit stehendem Filtertext zeigt diesen unversehens tief gefiltert, obwohl der Nutzer dort nie „Deep" eingeschaltet hat. Bei „Deep" an stößt jeder Tabwechsel in einen gefilterten Tab einen Durchlauf an.
3. **Je Dateifenster.** Ein Stand je Seite, für alle Tabs dieser Seite.
   - Pro: liegt zwischen den beiden anderen und trennt wenigstens die beiden Seiten.
   - Kontra: löst die Überraschung beim Tabwechsel nicht, denn die tritt innerhalb einer Seite auf. Und sie führte einen dritten Gültigkeitsbereich in eine Leiste ein, die heute nur einen kennt.

## Constraints

- Ohne stehenden Filtertext ändert „Deep" nichts an der Liste. Der Unterschied zwischen den Möglichkeiten wird erst sichtbar, sobald mindestens zwei Tabs einen Filtertext tragen.
- Kein Schalter der Leiste nimmt den Ersthelferrang an, und das gilt für den neunten unverändert.
- `Bereichsleiste::zustaende_setzen` ist der eine Schreiber der angezeigten Stände. Jede Antwort läuft über ihn, keine daran vorbei.
- Ob der Stand die Sitzung übersteht, gehört zur Antwort. Die drei Spaltenschalter tun es, der Filtertext soll es nach der Directive nicht.

## Recommendation

Möglichkeit 1. Der Schalter beschreibt nicht das Fenster, sondern die Suche, und die Suche gehört dem Tab; ein Schalter, der weiter reicht als das, worauf er wirkt, erzeugt genau die Überraschung, die Möglichkeit 2 unter Kontra trägt. Der Preis ist ein Gültigkeitsbereich mehr in der Leiste, und er ist an einer Stelle zu zahlen, die ohnehin bei jedem Tabwechsel nachzieht. Wir empfehlen daneben, den Stand nicht in die Sitzung zu schreiben: er hängt an einem Filtertext, den die Sitzung selbst nicht behält.

---
Answered:
Implemented:
Deferred:
Superseded by:

`keyWindow` und `isEqual:` stehen nicht im Untergrenzen-Abschnitt von `anwendung.rs`

---

Schritt A2 des Plans bringt zwei neu angesprochene Methoden in
`crates/krk-ui/src/appkit/anwendung.rs`: `NSApplication::keyWindow` (`:2625`) und `isEqual:`
(`:2630`, `:2634`). Der Plan hält dazu fest:

> Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` bleibt richtig:
> `keyWindow`, `attachedSheet` und `isEqual:` stehen alle drei schon darin.

Im Abschnitt (`anwendung.rs:168-194`) steht namentlich allein `attachedSheet` (`:191`), und das
stand schon vor dieser Runde darin. `keyWindow` und `isEqual:` kommen dort nicht vor.

---

**Schwere:** niedrig. Auf dem Bauziel folgenlos, und die Aussage des Abschnitts ist nicht
falsch: beide Methoden fallen unter den Pauschalsatz „**Nichts in dieser Datei liegt ueber
15.0.** Alles Uebrige … traegt im SDK-Kopf gar keine Verfuegbarkeitsangabe und steht damit seit
10.0" (`anwendung.rs:189-194`).

**Warum es trotzdem zählt.** `CLAUDE.md` führt diesen Abschnitt als die **einzige**
Gegenmaßnahme gegen die fehlende Verfügbarkeitsprüfung von `objc2`: „Die Gegenmaßnahme ist eine
Gewohnheit und kein Werkzeug, und sie hält sich nicht von selbst." Eine Gewohnheit, die eine neu
angesprochene Methode nicht nennt, hält sich in genau dem Maß nicht, das dieser Satz beschreibt.
C6.4 verlangt für neue Dateien den Abschnitt und für jede genannte Zahl den Blick ins SDK;
für eine bestehende Datei, die eine Methode dazubekommt, sagt das Kriterium nichts — und
diese Lücke ist der Grund, warum die Stelle durchgegangen ist.

**Der Plan hat hier eine Prüfung durch eine Behauptung ersetzt.** „Stehen alle drei schon darin"
ist als Feststellung formuliert und trifft für zwei von dreien nicht zu. Der Ausführer hatte
damit keinen Anlass nachzusehen.

**Was zu tun ist**

Beide Namen in den Abschnitt aufnehmen. Am SDK nachgelesen gehört dazu: `NSApplication.h`
führt `keyWindow` ohne Verfügbarkeitsangabe, `isEqual:` steht an `NSObject` seit 10.0. Wer
den Pauschalsatz behalten will, nimmt sie in dessen Aufzählung auf, statt eine eigene Zeile
je Methode zu schreiben — dort stehen `attachedSheet` und fünf weitere schon so.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Berührt C6.4 und die offene Frage
  `shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`: eine Angabe
  von Hand, die zwei neu angesprochene Methoden nicht mitnimmt, ist ein zweiter Beleg für deren
  dritte Stufe. Der erste steht als
  `260813-1258_o_der-modulkopf-von-titelzusatz-laesst-die-bedingung-fuer-left-weg.md`.

# Was zeigen Git-Bereich, Ankreuzfeld und Dateiliste in einem Ordner ohne Repository?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 8); `crates/krk-ui/src/appkit/bereichsleiste.rs` (die zehn Ankreuzfelder); `crates/krk-ui/src/appkit/statuszeile.rs:96` (die Regel „ein ausgeblendeter Bereich bewirbt sich nicht")

---

## Question

KRK navigiert beliebige Ordner, und die meisten liegen in keinem Git-Baum. Der leere Fall ist damit der Normalfall und nicht der Sonderfall. Drei Anzeigen sind betroffen und hängen nicht aneinander, also sind es drei Antworten und nicht eine.

Die Kosten der Frage selbst sind kein Argument: `gix::discover` auf einem Pfad ohne Repository kostet gemessen 21 bis 82 µs, je nach Zahl der Ebenen bis zur Wurzel, also ein Zweihundertstel eines Bildes. Die Frage darf bei jedem Ordnerwechsel gestellt werden.

## Options

Für den **Git-Bereich**:

1. **Ein Satz an der Stelle des Inhalts**, etwa „Dieser Ordner liegt in keinem Git-Repository." Der Bereich bleibt sichtbar und behält seine Breite.
2. **Der Bereich blendet sich aus** und die übrigen bekommen den Platz. Das widerspricht der Bauart der Fensterzeile: die Sichtbarkeit eines Bereichs ist eine Größe des Nutzers, die er über die Bereichsleiste setzt, und `Fenstermodell::sichtbar_setzen` ist die eine Schreibstelle dafür. Ein Bereich, der sich nach dem Ordner selbst ausblendet, gäbe ihr einen zweiten Schreiber.

Für das **Ankreuzfeld der Marken**:

1. **Es bleibt eingeschaltet und wirkt nicht.** Wie die übrigen zehn Felder zeigt es eine Einstellung und keine Lage; die Einstellung gilt fensterweit, der Ordner wechselt darunter.
2. **Es wird ausgegraut, solange kein Repository dasteht.** Näher an der Wirklichkeit, aber es flackerte bei jedem Ordnerwechsel, und keines der zehn vorhandenen Felder tut das. Dazu käme die Frage, welches der beiden Dateifenster es entscheidet: die zwei letzten Felder („Deep" und „Content") hängen schon heute am **aktiven** Dateifenster und nicht am Fenster, und das ist der Sonderfall, den ihr Modulkopf ausdrücklich begründet.

Für die **Dateiliste**:

1. **Die Markenspalte steht und bleibt leer.** Die Spaltenbreite ist stabil, die Liste springt beim Ordnerwechsel nicht.
2. **Die Markenspalte wird eingezogen.** Spart Platz, lässt aber die ganze Liste bei jedem Wechsel zwischen einem Repository und einem gewöhnlichen Ordner umbrechen.

## Constraints

- Die Sichtbarkeit eines Bereichs hat genau einen Schreiber, `Fenstermodell::sichtbar_setzen`; jede Antwort, die einen zweiten einführt, bricht die Zusage, die dort im Doc-Kommentar steht.
- Die Statuszeile hat für dieselbe Lage schon eine Regel: „Ein ausgeblendeter Bereich bewirbt sich nicht." Sie zeigt nichts über einen Bereich, den der Nutzer nicht sieht. Der Git-Bereich sollte sich daran halten und nicht daneben.
- Ein Ordner **innerhalb** eines Repositorys ist nicht der leere Fall: `discover` findet den Baum auch aus einem Unterordner, und der Status ist dann über ein Pfadmuster auf den angezeigten Ordner zu beschränken. Der leere Fall ist allein der Pfad, unter dem bis zur Wurzel kein `.git` liegt.
- Ein Repository ohne Commit ist ein dritter Fall und nicht der leere: `head_name()` liefert dort den Branchnamen, `head_id()` scheitert mit `Unborn`, der Verlauf ist leer. Er braucht eine eigene Antwort im Git-Bereich.

## Recommendation

Wir empfehlen je Anzeige die erste Möglichkeit: ein Satz im Bereich, ein Ankreuzfeld, das eine Einstellung zeigt und keine Lage, und eine Markenspalte, die steht und leer bleibt. Alle drei halten die Anzeige beim Ordnerwechsel ruhig, und Ruhe beim Wechsel ist in einem Programm, dessen Zusagen in Einzelbildern gemessen werden, kein Geschmack, sondern eine Eigenschaft.

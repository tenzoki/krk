# Was tut ein Link im gerenderten Markdown, und bleibt die Vorschaufläche unauswählbar?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/appkit/vorschau.rs:574-575` (`setEditable(false)`, `setSelectable(false)` und die Begründung im Modulkopf); `crates/krk-ui/src/appkit/ereignisse.rs` (`ersthelfer_gehoert_appkit`); `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md` (der Betrachter, der Verweisen folgen soll); C9 der Runde 1, „Nur lokale Laufwerke"

---

## Question

Gerendertes Markdown zeigt Links als Verweise, ohne ihre Klammern. Damit stellt sich die Frage, die der rohe Text nicht stellte: was geschieht, wenn der Nutzer einen anklickt.

Die Vorschaufläche ist heute weder bearbeitbar noch auswählbar, und das ist keine Nachlässigkeit, sondern eine Bedingung. Der Modulkopf hält den Grund fest: eine auswählbare Textfläche nähme den Fokus als Textsystem, und der Ereignisabgriff reichte danach jede Taste an AppKit weiter, statt die Tabbefehle aus C1 auszuführen. Wer die Fläche auswählbar macht, verliert die Tastenbedienung der Vorschau-Tabs.

Ein anklickbarer Link braucht in AppKit keine auswählbare Fläche; eine Fläche mit `NSLinkAttributeName` liefert den Klick auch ohne Auswahl. Die Frage bleibt trotzdem gekoppelt, weil beide Antworten dieselbe Fläche betreffen und weil ein Nutzer, der einen Link anklicken kann, als Nächstes Text markieren will.

Eine dritte Sache hängt daran: **wohin ein Link führt.** Ein Verweis auf eine lokale Datei ist etwas anderes als eine Web-Adresse, und Letztere berührt zwei bestehende Festlegungen. C9 der Runde 1 lässt zum Systembrowser allein `http:` und `https:` durch und weist jedes andere Schema ab, damit KRK nicht über einen Umweg eine Serververbindung aufbaut. Der vorgesehene Circle `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` will Verweisen künftig **in KRK** folgen; diese Runde darf ihm das nicht vorwegnehmen, weil eine Web-Ansicht in Festlegung A ausdrücklich abgelehnt ist.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Links sind sichtbar, aber nicht anklickbar. Die Fläche bleibt unauswählbar.** Ein Verweis erscheint unterstrichen und eingefärbt, wie schon im Editor, und tut auf einen Klick nichts.
   - Folge: nichts ändert sich an Fokus, Tastenbedienung und Ereignisabgriff. C9 wird nicht berührt, und der Web-Betrachter findet die Frage unverbraucht vor.
   - Preis: ein Verweis, der aussieht wie ein Verweis und keiner ist. Das ist die Lage, die Nutzer am ehesten für einen Fehler halten.

2. **Web-Adressen gehen an den Systembrowser, Verweise auf lokale Dateien öffnen die Datei in der Vorschau. Die Fläche bleibt unauswählbar.**
   - Folge: der Link tut, was er verspricht, und der Weg ist gebaut: die Übergabe an den Systembrowser besteht seit C10 der Runde 1, das Öffnen einer Datei in der Vorschau seit C6. C9 bleibt eingehalten, weil weiterhin nur `http:` und `https:` durchgehen.
   - Preis: die Vorschau bekommt einen zweiten Weg, auf dem ihr Inhalt wechselt, neben der Auswahl im Dateifenster und der Zwischenablage. Das Halteverhalten aus C6 ist heute von einer einzigen Regel getragen, „jede Quelle schreibt in den aktiven Tab und in keinen anderen"; ein dritter Schreiber muss sich ihr fügen. Daneben nimmt diese Antwort dem Web-Betrachter einen Teil seiner ersten offenen Frage vorweg, nämlich welche Quellen eine Adresse setzen dürfen.

3. **Wie Möglichkeit 2, und die Fläche wird zusätzlich auswählbar, damit der Nutzer Text kopieren kann.**
   - Folge: die Vorschau verhält sich wie ein gewöhnlicher Betrachter.
   - Preis: die Tastenbedienung der Vorschau-Tabs fällt. Der Ereignisabgriff fragt nach der Nämlichkeit des Ersthelfers, und die Textfläche des Editors ist heute die eine angemeldete Ausnahme. Eine zweite bedienbare Textfläche müsste dort angemeldet werden, und der Vergleich sitzt beim Anwendungsdelegierten. Das ist ein Eingriff in eine Stelle, die dieses Projekt ausdrücklich schmal hält, für einen Zweck, den der Nutzer nicht verlangt hat.

## Constraints

- Die Vorschaufläche darf den Fokus nicht als Textsystem nehmen, solange die vier Tabbefehle aus C1 in ihr wirken sollen. Der Zusammenhang steht im Modulkopf von `crates/krk-ui/src/appkit/vorschau.rs`.
- C9 der Runde 1 bleibt, wo sie ist: zum System gehen allein `http:` und `https:`. Der Circle des Web-Betrachters hält ausdrücklich fest, dass ein eingebauter Betrachter diese Grenze nicht verschiebt; für diese Runde gilt dasselbe.
- Eine Web-Ansicht ist in Festlegung A abgelehnt. Ein Link darf in KRK keine Seite darstellen.
- Das Halteverhalten aus C6 hat genau eine Regel. Ein neuer Schreiber in die Vorschau nimmt keine Tabstelle entgegen und schreibt in den aktiven Tab.

## Recommendation

**Wir empfehlen Möglichkeit 1 für diese Runde.** Der Grund ist nicht der Aufwand, sondern die Reihenfolge: die Frage, welche Quellen eine Adresse setzen dürfen, ist die erste offene Frage des Web-Betrachter-Circles und dort mit drei aufeinander aufbauenden Möglichkeiten ausgearbeitet. Sie hier nebenbei zu beantworten, hieße jenen Circle um seine Klärungsrunde zu bringen.

Der Preis, ein Verweis, der nicht klickt, lässt sich mildern, ohne die Frage zu berühren: der Spec kann festlegen, dass ein Link im gerenderten Markdown die Farbe bekommt, aber nicht den Zeigefinger-Mauszeiger, damit er nicht mehr verspricht, als er hält.

**Möglichkeit 3 empfehlen wir nicht.** Sie kostet die Tastenbedienung der Vorschau-Tabs, und die ist eine abgenommene Zusage aus C1.


## Antwort 260812-1105

**Moeglichkeit 1.**

Ein Link im gerenderten Markdown wird **angezeigt und nicht angeklickt**, und die Vorschauflaeche
bleibt unauswaehlbar.

Der Grund ist die Reihenfolge, nicht der Aufwand: welche Quellen eine Adresse setzen duerfen, ist
die erste offene Frage des Web-Betrachter-Circles und dort mit drei aufeinander aufbauenden
Moeglichkeiten ausgearbeitet. Sie hier nebenbei zu beantworten hiesse, jenen Circle um seine
Klaerungsrunde zu bringen.

**Der Spec legt fest, dass ein Link die Farbe bekommt, aber nicht den Zeigefinger-Mauszeiger** —
so verspricht er nicht mehr, als er haelt.

Die Vorschauflaeche auswaehlbar zu machen ist abgelehnt: das kostet die Tastenbedienung der
Vorschau-Tabs, und die ist eine abgenommene Zusage aus C1 der Runde 2.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:

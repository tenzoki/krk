# Bedeutet der Akzentrahmen künftig den Fokus oder das aktive Dateifenster?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C9), `crates/krk-ui/src/appkit/aufteilung.rs:229-238` (`Aufteilung::aktives_markieren`, die gebaute Anzeige), `crates/krk-ui/src/kommandos/fokus.rs:55-90` (die fünf Fokuswerte), `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`

---

## Question

KRK zeichnet heute genau eine Markierung dieser Art, und sie hat eine Bedeutung. `Aufteilung::aktives_markieren` setzt die Rahmenfarbe eines `NSBox` je Dateifenster: `NSColor::controlAccentColor()` für das aktive, `NSColor::separatorColor()` für das andere, bei zwei Punkten Rahmenbreite. Der Doc-Kommentar begründet die Form ausdrücklich damit, dass ein Rahmen und kein Unterschied im Inhalt eindeutig bleibt, wenn beide Dateifenster denselben Ordner zeigen. Der Auslöser ist `Fensterzustand::aktiv` und nicht der Fokus: das aktive Dateifenster behält seinen Rahmen, während der Nutzer in der Lesezeichenleiste oder in der Vorschau arbeitet.

C9 verlangt vom selben Rahmen eine zweite Aussage. Der Nutzer soll sehen, welcher der fünf Bereiche seine Tasten annimmt, und die Anzeige soll für alle fünf dieselbe sein. Damit stehen zwei Aussagen auf einem Kanal:

- **"Hier kommen deine Tasten an."** Sie wechselt zwischen fünf Bereichen und ist die Frage, die C9 stellt.
- **"Aus diesem Dateifenster kopiert F5."** Sie wechselt zwischen zwei Bereichen und entscheidet jede Dateioperation aus C4 der Runde 1.

Beide Aussagen sind zu treffen, und beide werden gebraucht. Ein Nutzer, der im Editor tippt und F5 drückt, muss vorher wissen, aus welchem der beiden Dateifenster kopiert wird; ein Nutzer, der nicht weiß, wo seine Tasten ankommen, hat genau das Problem, gegen das C9 gerichtet ist.

Die Frage hält keinen Planschritt auf: der Spec trägt den Vorschlag des Shapers als Vorbelegung, und der Plan kann gegen ihn gebaut werden. Sie ist vor der Abnahme von C9 zu beantworten, weil sie das vierte und das fünfte Abnahmekriterium jener Fähigkeit bestimmt.

## Options

1. **Drei Zustände auf einem Kanal, der Fokus gewinnt** — der Bereich mit dem Fokus trägt die Akzentfarbe; das aktive Dateifenster ohne Fokus trägt eine zurückgetretene Form derselben Markierung; alles übrige trägt die Trennfarbe wie heute.
   - Pro: eine Anzeige, eine Stelle im Code, und sie folgt der Gewohnheit von macOS, das eine Auswahl bei verlorenem Fokus zurücktreten lässt, statt sie zu löschen. Beide Aussagen bleiben lesbar.
   - Contra: das aktive Dateifenster ist schwächer markiert als heute, sobald der Fokus woanders steht, und das ist gegenüber dem gebauten Stand eine Einbuße. Der Nutzer muss zwei Abstufungen derselben Farbe unterscheiden.

2. **Zwei Kanäle, jede Aussage bekommt ihren eigenen** — der Rahmen behält seine heutige Bedeutung, also das aktive Dateifenster, und der Fokus bekommt eine zweite, davon verschiedene Anzeige, etwa einen Streifen an der Kante des Bereichs oder eine abgesetzte Hintergrundfarbe seiner Kopfzeile.
   - Pro: keine Aussage verliert an Schärfe, und das aktive Dateifenster bleibt so markiert, wie der Nutzer es seit der Runde 1 kennt.
   - Contra: zwei Anzeigen, die im aktiven und fokussierten Dateifenster zugleich auftreten und dort dieselbe Fläche umranden. Die Maxime "supersimpel" spricht dagegen, und der Nutzer hat vier Zustände zu deuten statt drei.

3. **Der Rahmen bedeutet allein den Fokus, und das aktive Dateifenster wird anderswo genannt** — der Rahmen folgt dem Fokus in allen fünf Bereichen, ohne Abstufung, und welches Dateifenster das aktive ist, sagt die Statuszeile oder die Tableiste jenes Fensters.
   - Pro: die schärfste Anzeige für die Frage, die C9 stellt, ohne Abstufungen. Der Rahmen trägt genau eine Bedeutung.
   - Contra: die Angabe, die jede Dateioperation entscheidet, wandert aus dem Blickfeld in eine Zeile, auf die der Nutzer eigens sehen muss. Der Doc-Kommentar der gebauten Anzeige begründet den Rahmen gerade damit, dass ein Unterschied im Inhalt nicht eindeutig ist.

## Constraints

- Die Anzeige gilt für alle fünf Bereiche und entsteht an einer Stelle. Eine Anzeige, die nur den Editor bedient, ist keine Antwort auf C9.
- Zwei der fünf Bereiche haben keine Auswahl, deren Farbe umschlagen könnte: die Textanzeige der Vorschau lehnt Auswahl ausdrücklich ab (`crates/krk-ui/src/appkit/vorschau.rs:513`), und die Textfläche des Editors trägt eine Schreibmarke statt einer ausgewählten Zeile. Die Gewohnheit von macOS, die hervorgehobene gegen die zurückgetretene Auswahlfarbe zu setzen, trägt deshalb allein für drei von fünf.
- Welches Dateifenster das aktive ist, muss ohne einen zusätzlichen Handgriff erkennbar bleiben. Es entscheidet Kopieren, Verschieben, Löschen und Umbenennen aus C4 der Runde 1.
- KRK baut das Erscheinungsbild von Hell und Dunkel nicht selbst nach, sondern nimmt die Systemfarben. Die Begründung steht in `crates/krk-ui/src/appkit/leiste.rs:441` und im Modulkopf von `tableiste.rs`.
- Drei der fünf Bereiche tragen heute keinen Kasten: `Aufteilung` hält zwei `NSBox`, einen je Dateifenster (`crates/krk-ui/src/appkit/aufteilung.rs:134`). Jede der drei Möglichkeiten kostet dort Arbeit; keine ist dadurch billiger als die andere.
- Die Antwort bindet auch die Behebung von `issues/260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`. Solange `Anwendungsdelegierter::fokus` für eine Unteransicht eines Randbereichs `Dateifenster` antwortet, zeigt jede fokusgetriebene Anzeige dort den falschen Bereich.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie hält an einer Anzeige fest und trifft beide Aussagen, und die Abstufung ist keine Erfindung, sondern die Form, in der macOS seit Langem eine Auswahl zurücktreten lässt, die den Fokus verloren hat. Der Nutzer liest sie deshalb ohne Erklärung.

Die benannte Einbuße ist real und wir verkleinern sie nicht: das aktive Dateifenster ist mit dem Fokus im Editor schwächer markiert als heute. Sie wiegt weniger als der Preis der zweiten Möglichkeit, die zwei Anzeigen um dieselbe Fläche legt, und weniger als der Preis der dritten, die die Angabe aus dem Blickfeld nimmt, von der jede Dateioperation abhängt.

`inference:` Ob zwei Abstufungen derselben Akzentfarbe auf dem Referenzgerät und in beiden Erscheinungsbildern gut genug zu unterscheiden sind, ist nicht gemessen. Wer Möglichkeit 1 wählt und die Abstufung später als zu schwach empfindet, ändert eine Farbe und keine Regel.

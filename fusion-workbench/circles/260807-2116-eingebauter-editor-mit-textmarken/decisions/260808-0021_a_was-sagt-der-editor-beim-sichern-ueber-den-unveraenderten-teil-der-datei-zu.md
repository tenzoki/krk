# Was sagt der Editor beim Sichern über den Teil der Datei zu, den der Nutzer nicht angefasst hat?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md` (die Vorfrage, aus deren Antwort diese entsteht), `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C2 und C4), `crates/krk-ui/src/vorschaumodell.rs:522-527` (die bestehende Textregel der Vorschau)

---

## Question

Die Antwort vom 260808-0017 hat eine bindende Zusage gesetzt: kein Weg darf eine Datei beim Sichern verändern, die der Editor nicht vollständig und verlustfrei als Text gelesen hat. Diese Zusage regelt das **Lesen** vollständig. Über das **Zurückschreiben** des Teils, den der Nutzer gar nicht angefasst hat, sagt sie nichts, und genau dort entsteht der zweite Schaden derselben Art.

Drei Eigenschaften einer Textdatei überleben die Wandlung in eine Zeichenkette und wieder zurück nicht von selbst. Die Zeilenenden: eine Datei mit Windows-Zeilenenden trägt vor jedem Umbruch ein zusätzliches Byte, und ein Editor, der beim Sichern durchweg Unix-Zeilenenden schreibt, ändert jede Zeile der Datei. Der abschließende Zeilenumbruch: eine Datei ohne ihn bekommt einen, wenn der Editor beim Schreiben einen anhängt, oder verliert ihn, wenn er ihn beim Lesen abschneidet. Die Bytefolgenmarke am Dateianfang: `String::from_utf8` liefert sie als Zeichen `U+FEFF` am Anfang der Zeichenkette zurück, und ob sie beim Schreiben wieder dort landet, hängt daran, ob der Editor sie als Text behandelt oder als Rahmen.

Der Schaden ist sichtbar und nicht theoretisch. Wer eine Zeile in einer Datei mit Windows-Zeilenenden ändert und ein normalisierendes Sichern bekommt, hat danach eine Änderung in jeder Zeile der Datei. In einem versionierten Verzeichnis, und KRK bekommt in einer späteren Runde eine Git-Anbindung, ist das der Unterschied zwischen einer lesbaren Änderung und einer unbrauchbaren.

Die Frage gehört vor den Planschritt, der das Sichern baut. Sie hält keinen früheren Schritt auf.

## Options

1. **Die Datei behält ihre Form; der Editor merkt sie sich beim Lesen** — beim Öffnen hält der Editor fest, welche Zeilenenden die Datei trägt, ob sie mit einem Umbruch endet und ob sie eine Bytefolgenmarke führt. Beim Sichern schreibt er dieselbe Form zurück. Eine gemischte Datei, die beide Zeilenenden führt, bekommt die Form, die in ihr überwiegt.
   - Pro: das Sichern ändert genau die Stelle, die der Nutzer geändert hat, und keine andere. Die Zusage vom 260808-0017 wird auf das Schreiben durchgezogen statt auf halber Strecke aufzuhören.
   - Contra: drei Angaben mehr im Zustand des Editors, und die gemischte Datei braucht eine eigene Regel. Der Nutzer sieht die gemerkte Form nirgends und kann sie nicht ändern.

2. **KRK schreibt immer Unix-Zeilenenden, immer einen abschließenden Umbruch, nie eine Bytefolgenmarke** — eine Form für alles, unabhängig davon, was die Datei mitbrachte.
   - Pro: eine Regel, kein gemerkter Zustand, keine Sonderfälle. Auf einem Mac ist es die Form, die jede Werkzeugkette erwartet.
   - Contra: das Sichern ändert Zeilen, die der Nutzer nicht angefasst hat, und das ist derselbe Schaden, gegen den die Zusage vom 260808-0017 gerichtet ist, nur kleiner. Eine fremde Datei aus einem Windows-Projekt kommt verändert zurück.

3. **Der Editor weist Dateien ab, deren Form er nicht unverändert zurückschreiben kann** — angenommen werden allein Dateien mit Unix-Zeilenenden ohne Bytefolgenmarke; alles übrige meldet die Statuszeile mit dem Grund.
   - Pro: keine Form geht verloren, weil keine fremde Form hereinkommt. Die Regel ist in einem Satz erklärt.
   - Contra: die Größen- und Typprüfung aus C2 bekommt ein drittes Kriterium, das der Nutzer nicht erwartet, und eine gewöhnliche Textdatei wird abgewiesen, obwohl der Editor sie darstellen könnte.

## Constraints

- Die Zusage vom 260808-0017 bindet und steht über jeder der drei Möglichkeiten: kein Weg darf eine Datei beim Sichern verändern, die der Editor nicht vollständig und verlustfrei als Text gelesen hat.
- Die Textregel des Lesens steht bereits und wird nicht neu erfunden: gültiges UTF-8, wie die Vorschau es über `String::from_utf8` prüft (`crates/krk-ui/src/vorschaumodell.rs:522-527`). Diese Frage betrifft allein das Zurückschreiben.
- Ein abgewiesener oder veränderter Fall meldet seinen Grund in der Statuszeile aus C1 der Runde 1. Eine zweite Meldefläche entsteht nicht.
- Die Antwort bindet neben dem Sichern aus C4 auch das Ersetzen aus C5, weil ein Ersetzen über alle Treffer auf einen Zeilenumbruch treffen kann.
- Was der Editor beim Lesen wegwirft, kann er beim Sichern nicht zurückgeben. Eine Antwort, die das Merken erst beim Schreiben ansetzt, ist keine.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie zieht die bereits getroffene Zusage konsequent durch, statt sie auf das Lesen zu beschränken, und sie kostet drei Angaben in einem Zustand, den der Editor ohnehin führt. Der Preis der zweiten Möglichkeit ist derselbe Schaden in kleiner Form, und er trifft ausgerechnet den Fall, für den man einen eingebauten Editor am ehesten benutzt, nämlich eine fremde Datei, die man kurz anfasst.

Möglichkeit 3 empfehlen wir nicht. Sie tauscht eine Regel beim Schreiben gegen eine Absage beim Öffnen, und die Absage trifft Dateien, die der Nutzer als gewöhnlichen Text ansieht.

`inference:` Die gemischte Datei, die beide Zeilenenden führt, ist der einzige Fall, in dem Möglichkeit 1 eine eigene Entscheidung braucht. Sie ist selten, und die Regel "die überwiegende Form gewinnt" ist eine von mehreren vertretbaren; wer eine andere will, sagt es bei der Antwort.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"11. Sicherungsform" — Möglichkeit 2 gewählt: KRK schreibt beim Sichern immer Unix-Zeilenenden, immer einen abschließenden Umbruch und nie eine Bytefolgenmarke, unabhängig von der Form, die die Datei mitbrachte. Der Nutzer ist damit der Empfehlung dieses Datensatzes (Möglichkeit 1, die Datei behält ihre Form) nicht gefolgt. Der hier benannte Preis ist angenommen: das Sichern ändert Zeilen, die der Nutzer nicht angefasst hat, und eine fremde Datei aus einem Windows-Projekt kommt verändert zurück. Entschieden vom Nutzer am 260808-0043.

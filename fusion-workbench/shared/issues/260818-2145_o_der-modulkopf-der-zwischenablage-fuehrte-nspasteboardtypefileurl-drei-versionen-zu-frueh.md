Der Modulkopf der Zwischenablage führte `NSPasteboardTypeFileURL` drei Versionen zu früh
---
`crates/krk-ui/src/appkit/zwischenablage.rs` zählte in seinem Abschnitt `# Ab welchem
macOS die angesprochenen Klassen stehen` die Konstante `NSPasteboardTypeFileURL` unter
den Symbolen „seit 10.6". Das SDK sagt 10.13 (`NSPasteboard.h:39`,
`API_AVAILABLE(macos(10.13))`). Gefunden und berichtigt am 260818 während Schritt 6 der
Runde 13.
---
**Warum das trotz der Behebung einen Datensatz bekommt.**

Die Angabe ist folgenlos geblieben, weil die Untergrenze dieses Projekts bei macOS 15
liegt und 10.13 weit darunter. Der Befund betrifft nicht diese eine Zahl, sondern den
Mechanismus, der sie hält.

`objc2` führt keine Verfügbarkeitsangaben mit sich, und der Übersetzer hält die
Untergrenze deshalb nicht. Wer eine Methode anspricht, die nach macOS 15 hinzugekommen
ist, bekommt keine Warnung, sondern einen Absturz auf dem Referenzgerät. Der Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` in jeder Datei unter
`crates/krk-ui/src/appkit/` ist die **einzige** Gegenmaßnahme dagegen, und `CLAUDE.md`
sagt ausdrücklich, sie sei eine Gewohnheit und kein Werkzeug.

Dieser Fund zeigt, dass die Gewohnheit nicht nur ausfallen kann, sondern auch falsch
eintragen kann. Eine fehlende Angabe fällt bei der nächsten Durchsicht auf; eine falsche
sieht aus wie eine geprüfte. Der Unterschied ist wichtig, weil die Deckung dieses
Abschnitts schon zwischen dem 260811 und dem 260814 viermal falsch gezählt und
zwischenzeitlich auf fünf Dateien abgesunken war
(`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`).

**Die offene Frage dazu besteht bereits** und wird von diesem Fund gestützt:
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` führt drei
Stufen mit Kosten. Solange keine gewählt ist, ist eine falsche Zahl in einem Modulkopf
durch nichts zu entdecken außer durch einen Menschen oder einen Agenten, der zufällig
dieselbe Zeile im SDK aufschlägt.

**Zweiter Fund derselben Sitzung, hier nur vermerkt:** der Plan der Runde 13 legte in
Schritt 6 zwei Symbole auf eine Zeile und schrieb
`NSPasteboardURLReadingFileURLsOnlyKey` 10.13 zu. Richtig ist 10.6 (`NSPasteboard.h:146`).
Der Plan ist ein Datensatz und kein Baum, deshalb steht das hier und nicht als eigener
Eintrag.

---
Resolved: Die Zahl im Modulkopf steht seit dem 260818 auf 10.13, berichtigt in Schritt 6
der Runde 13. Der Datensatz bleibt offen für die Frage dahinter, wie eine falsche Angabe
künftig auffällt; die Antwort hängt an
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`.

---
Abgleich 260819-0057 (reconciler): **die Zahl am Baum stimmt, der Datensatz bleibt zu Recht
offen, und seine Form verdient eine Anmerkung.**

**Der Bestand.** `crates/krk-ui/src/appkit/zwischenablage.rs:136-139` sagt jetzt
„**`NSPasteboardTypeFileURL` steht seit 10.13** (`NSPasteboard.h:39`) und nicht seit 10.6, wie
diese Stelle bis zur Runde 13 sagte", und nennt daneben, dass die Angabe folgenlos geblieben
ist. Der zweite Fund im Rumpf oben ist ebenfalls am Baum nachgeprüft: `:129-131` führt
`NSPasteboardURLReadingFileURLsOnlyKey` unter den Symbolen seit 10.6, wie das SDK es sagt. Falsch
geblieben ist allein der Plan (Schritt 6), der die zwei Symbole auf eine Zeile legt und beiden
10.13 zuschreibt; der Plan ist mit diesem Abgleich auf `_c_` gesetzt und die Stelle darin
unberichtigt, weil sie beschreibt, was gefahren wurde.

**Warum der Datensatz offen bleibt.** Die Frage dahinter ist nicht die Zahl, sondern der
Mechanismus: wie fällt eine **falsche** Angabe künftig auf, wo eine fehlende bei der nächsten
Durchsicht auffiele. Sie hängt an
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` (offen, drei
Stufen mit Kosten) und wird von diesem Fund gestützt. Die Durchsicht des zweiten Turns hat
daneben jede Verfügbarkeitszahl der drei Modulköpfe gegen das SDK nachgelesen und alle richtig
gefunden — das misst die Gewohnheit an einem Tag und ersetzt den Mechanismus nicht.

**Anmerkung zur Form.** Dieser Datensatz trägt eine `Resolved:`-Zeile bei Marker `_o_`. Die
Konvention kennt diese Verbindung nicht: eine `Resolved:`-Zeile geht dort mit der Umbenennung
auf `_c_` einher. Der Grund hier ist erkennbar und richtig — der behobene Teil und der offene
Teil stehen in einem Datensatz —, aber der offene Teil ist der Sache nach eine Frage und keine
Störung, und er hat seinen eigenen Entscheidungsdatensatz schon. Wer den Datensatz aufräumt,
hat damit den sauberen Weg: `_c_` mit Verweis auf `260811-2050`, statt einer Mischform, die
jede Erhebung über `Resolved:`-Zeilen falsch zählt (vgl.
`shared/issues/260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md`).
Nicht in diesem Durchgang geändert, weil die Umbenennung eine Wertung über die offene Frage
wäre.

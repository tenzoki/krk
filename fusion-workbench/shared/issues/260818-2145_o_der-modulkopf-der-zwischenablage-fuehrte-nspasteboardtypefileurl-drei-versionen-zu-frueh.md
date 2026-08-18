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

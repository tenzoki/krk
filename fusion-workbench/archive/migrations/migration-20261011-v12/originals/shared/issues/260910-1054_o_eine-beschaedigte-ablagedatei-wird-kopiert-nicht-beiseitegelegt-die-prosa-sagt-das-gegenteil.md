Eine beschädigte Ablagedatei wird kopiert, nicht beiseitegelegt; die Prosa sagt das Gegenteil

---

Prosastellen unter `crates/krk-core/src/ablage/` beschreiben den Umgang mit einer
beschädigten Ablagedatei so, als werde sie beiseitegelegt und sei damit beim nächsten Lesen
weg. Der Code legt eine **Kopie** an; die beschädigte Datei bleibt liegen und wird beim
nächsten Start wieder gelesen und wieder als beschädigt befunden.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** vom Bau des Schrittes 1 der Runde
`260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`, der die Annahme beim Bauen
des Vergleichs geprüft hat und sie nicht bestätigen konnte.

## Was daran zählt

Die Annahme trägt eine Begründung: wer glaubt, die beschädigte Datei sei nach dem ersten
Lesen fort, hält den Ersatzweg für einmalig. Er ist es nicht. Für den Vergleich zwischen
Auslieferungsfassung und Nutzerdatei ist das gerade behandelt worden — eine Datei auf dem
Ersatzweg wird nicht verglichen, sonst meldete KRK jeden ausgelieferten Eintrag als
Neuerung —, aber die falsche Aussage steht weiter im Baum und trägt die nächste Annahme
genauso.

## Abnahme

Die Prosastellen sagen, was der Code tut: es wird eine Kopie angelegt, das Original bleibt
liegen. Erhoben mit einer Suche über `crates/krk-core/src/ablage/` nach den Wendungen für
das Beiseitelegen; welche Stellen es sind, sagt die Erhebung und nicht dieser Datensatz.

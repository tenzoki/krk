# Welchen Umfang von Markdown rendert die Vorschau?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/hervorhebung.rs` (Modulkopf: zwei Merkmalslisten, der Schnitt „wirkt auf die Auslegung oder nicht"); `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260810-0822_*_wie-die-formatansicht-ihre-auszeichnung-setzt-und-warum-an-zwei-orten.md`; `crates/krk-ui/src/vorschaumodell.rs:117` (`TEXTGRENZE`, 1 MB)

---

## Question

Festlegung A sagt, Markdown wird voll gerendert und die Auszeichnungszeichen verschwinden. Offen ist, wie weit „voll" reicht. Markdown ist keine feste Menge, und jede Zutat kostet unterschiedlich viel.

Der Editor der Runde 2 zeigt heute vier Wirkungen und behält dabei die Zeichen: Überschriften größer und fett, Listen eingerückt, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift. Diese vier sind die Untergrenze dessen, was die Vorschau zeigen muss, sonst sähe sie ärmer aus als der Editor, obwohl sie mehr können soll.

Darüber hinaus liegen, in wachsender Kosten: Betonung und starke Betonung, Zitatblöcke, Trennlinien, Tabellen, Bilder, Fußnoten und verschachtelte Listen. Die drei teuren sind Tabellen, Bilder und verschachtelte Listen. **Tabellen** brauchen eine Spaltenausrichtung, die eine `NSTextView` nur über Tabulatorpositionen kennt, und die Breite ist bei 160 Punkten Mindestbreite eng. **Bilder** bedeuten, dass die Vorschau beim Anzeigen einer Textdatei weitere Dateien von der Platte liest, was die Zusage L7 berührt und die Bildgrenze von 64 MB aus C6 in eine zweite Lage bringt. **Verschachtelte Listen** brauchen eine Einrücktiefe, die die vorhandene Auszeichnungsmechanik nicht kennt.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Die vier des Editors, ohne ihre Zeichen, plus Betonung.** Überschriften, Listen, Links, Quelltextblöcke, dazu kursiv und fett. Alles Weitere erscheint als der Text, der dasteht.
   - Folge: der Umfang ist genau der, den die Runde 2 schon herstellt, und der Unterschied zur Formatansicht des Editors ist allein das Verschwinden der Zeichen. Eine Tabelle erscheint als das Zeilenraster aus Strichen und Balken, das im Quelltext steht, und ist damit lesbar, wenn auch nicht ausgerichtet. Kein Lesevorgang auf weitere Dateien, damit keine Berührung mit L7 und keine mit der Bildgrenze.
   - Preis: der Nutzer, der „gerendert" wie in einem Betrachter meint, sieht bei Tabellen und Bildern den Unterschied. Bei einer README mit Tabelle ist das der auffälligste Fall.

2. **Möglichkeit 1 plus Tabellen und Zitatblöcke, ohne Bilder.** Tabellen bekommen Tabulatorpositionen, Zitate einen Einzug mit Randstrich.
   - Folge: die gängigen Bestandteile einer README erscheinen so, wie der Nutzer sie erwartet. Immer noch kein zweiter Lesevorgang und keine Berührung mit L7.
   - Preis: die Spaltenbreite einer Tabelle hängt an der Breite der Vorschau, und die ist der schmalste Bereich des Fensters. Bei 160 Punkten läuft jede Tabelle mit drei Spalten über, und was dann geschieht, wäre eine zusätzliche Regel. Der Zusammenhang mit dem Datensatz `260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md` ist unmittelbar.

3. **Voller Umfang einschließlich Bildern.** Ein `![...](pfad)` lädt die genannte Datei und zeigt sie in der Textfläche.
   - Folge: die Vorschau zeigt Markdown so, wie ein Betrachter es zeigt.
   - Preis: die Vorschau liest beim Anzeigen einer Textdatei weitere Dateien, und zwar so viele, wie das Dokument nennt. Das berührt L7 an genau der Stelle, an der Festlegung B sie gerade freigehalten hat: der Text ist erst sichtbar, wenn die Auslegung steht, und die steht erst, wenn die Bildgrößen bekannt sind. Daneben stellen sich drei Fragen neu, die C6 für Bilder bereits beantwortet hat: die Größengrenze von 64 MB, der Umgang mit einer nicht dekodierbaren Datei und der mit einer Adresse, die ins Netz zeigt. Die letzte berührt C9, „Nur lokale Laufwerke".

## Constraints

- Die Zusage L7 bleibt unangetastet (Festlegung B). Was den Text erst später sichtbar macht, ist ausgeschlossen; was ihn nachträglich einfärbt, ist erlaubt.
- Eine Auszeichnung, die auf die Auslegung wirkt, gehört in den Textspeicher, und nur was die Auslegung nicht ändert, wirkt als vorübergehendes Merkmal des Layoutverwalters. Der Schnitt stammt aus `NSLayoutManager.h:351` und ist in `crates/krk-ui/src/hervorhebung.rs` samt Zitat festgehalten. Er gilt in der Vorschau genauso.
- Die Vorschau zeigt Textdateien bis `TEXTGRENZE`, also 1 MB (`crates/krk-ui/src/vorschaumodell.rs:117`). Darüber fällt sie auf die Metadaten. Diese Runde ändert die Grenze nicht.
- Eine neue fremde Kiste braucht in der Wurzel-`Cargo.toml` den Satz, warum sie eingebunden ist, und darf auf dem Bauziel keinen C-Code bauen. `Cargo.lock` führt heute kein `cc` und außer `windows-sys` kein `-sys`-Paket.

## Recommendation

**Wir empfehlen Möglichkeit 1 für diese Runde**, mit einem ausdrücklichen Nachtrag im Spec, dass Tabellen als Quelltextraster erscheinen. Der Grund ist die Reihenfolge: Möglichkeit 2 hängt an der Breitenfrage, und die Breitenfrage hängt an der Rechnung der Runde 5, nach der die Vorschau bei 780 Punkten Fensterbreite rund 17 Punkte Luft hat. Solange diese Zahl nicht gemessen ist, ist eine Tabellenausrichtung eine Zusage auf ungeprüftem Grund.

**Möglichkeit 3 empfehlen wir nicht.** Sie stellt drei Fragen neu, die C6 für Bilder bereits beantwortet hat, und sie ist die einzige der drei, die L7 wirklich gefährdet. Wenn der Nutzer Bilder will, ist das eine eigene Runde und keine Zutat zu dieser.


## Antwort 260812-1105

**Moeglichkeit 1.**

Der Grundumfang, mit einem ausdruecklichen Nachtrag im Spec: **Tabellen erscheinen als
Quelltextraster** und nicht ausgerichtet.

Der Grund ist die Reihenfolge. Eine Tabellenausrichtung haengt an der Breitenfrage, und die haengt
an der Rechnung der Runde 5, nach der die Vorschau bei 780 Punkten Fensterbreite rund 17 Punkte
Luft hat. Solange diese Zahl ungemessen ist, waere eine Ausrichtung eine Zusage auf ungepruefter
Grundlage.

Eingebettete Bilder sind abgelehnt: sie stellen drei Fragen neu, die C6 fuer Bilder bereits
beantwortet hat, und sie sind der einzige der drei Umfaenge, der L7 wirklich gefaehrdet. Wenn der
Nutzer Bilder will, ist das eine eigene Runde und keine Zutat zu dieser.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:

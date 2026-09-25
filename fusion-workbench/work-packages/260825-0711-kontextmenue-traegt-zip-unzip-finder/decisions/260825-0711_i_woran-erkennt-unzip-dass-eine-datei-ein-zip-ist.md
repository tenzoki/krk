# Woran erkennt Unzip, dass eine Datei ein Zip ist?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_a_circle.md` (Directive, Unzip-Teil); `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260807-2147_*_welche-dateien-oeffnet-der-editor-ueberhaupt.md`

---

## Question

Unzip wirkt auf die ausgewählte Datei, wenn sie ein Zip ist, und sonst auf das eine Zip des angezeigten Ordners, wenn dort genau eines liegt. Beide Hälften dieser Regel setzen einen Test voraus, der eine Datei als Zip einordnet, und der Test entscheidet zugleich, was der Nutzer sieht: er bestimmt, ob eine Datei ohne Endung als Archiv angenommen wird, und ob eine falsch benannte Datei mit einer Meldung oder mit einem Fehlversuch endet. Die Frage muss vor dem Bau beantwortet sein, weil derselbe Test die Suche nach dem einen Zip im Ordner steuert und damit bestimmt, wann Unzip überhaupt etwas vorfindet.

## Options

1. **Nach der Endung `.zip`, ohne Rücksicht auf Groß- und Kleinschreibung** — die Datei heißt `.zip` oder `.ZIP`, also ist sie ein Archiv.
   - Pro: Billig, ohne Dateizugriff, und damit auch für die Suche im Ordner ohne Kosten, gleich wie viele Einträge er trägt. Deckt sich mit dem, was der Nutzer im Namen sieht.
   - Contra: Eine falsch benannte Datei wird angenommen und scheitert erst beim Entpacken; ein echtes Archiv ohne Endung wird nicht gefunden.
2. **Nach den ersten Bytes des Inhalts** — die Datei beginnt mit der Zip-Signatur `PK\x03\x04`.
   - Pro: Trifft die Sache statt den Namen; eine falsch benannte Datei fällt vorher auf.
   - Contra: Die Suche nach dem einen Zip im Ordner müsste jede Datei öffnen und lesen. Der Inhaltsfilter aus Runde 11 hält dafür genau einen Dateideskriptor und gibt ihn vor dem nächsten Kandidaten frei; derselbe Aufwand entstünde hier bei jedem Rechtsklick.
3. **Endung für die Suche, Inhalt für den gewählten Eintrag** — der Ordner wird nach `.zip` durchsucht, die tatsächlich gewählte Datei zusätzlich an ihren ersten Bytes geprüft.
   - Pro: Die teure Prüfung trifft genau eine Datei; eine falsch benannte Datei bekommt eine verständliche Meldung in der Statuszeile statt eines Fehlversuchs.
   - Contra: Zwei Regeln statt einer, und die Aussage „das ist ein Zip“ hat damit zwei Fassungen, die auseinanderlaufen können.

## Constraints

Dieses Projekt prüft den Typ einer Datei am offenen Deskriptor und nicht am Pfad: `krk_core::verzeichnis::sys::ohne_warten_oeffnen` öffnet mit `O_NONBLOCK`, der Aufrufer fragt `metadata()` am Deskriptor. Jede Möglichkeit, die den Inhalt liest, nimmt diesen Weg und keinen zweiten daneben. Der Test steht als eine Regel an einer Stelle, wie `verzeichnis::filter::traegt_ein_dateiname` es vormacht.

## Recommendation

Möglichkeit 1 für den ersten Bau. Der Nutzer wählt im Kontextmenü, was er im Dateifenster sieht, und dort steht der Name; ein Archiv, das nicht `.zip` heißt, ist in diesem Ordner ohnehin nicht als solches erkennbar. Möglichkeit 3 bleibt die Erweiterung, falls sich der Fehlversuch in der Praxis zeigt.

---
Answered: shared/history/260824-2120-orchestrator-session.md:32 — Moeglichkeit 1, Endung .zip ohne Ruecksicht auf Gross- und Kleinschreibung; die Inhaltspruefung bleibt spaeterer Ausbau.
Implemented: 423d5f2 — kontextmenue::ist_zipname prueft die Endung ohne Ruecksicht auf Gross- und Kleinschreibung und ohne Dateizugriff, als die eine Regel mit einer Stelle.
Deferred:
Superseded by:
Retired:

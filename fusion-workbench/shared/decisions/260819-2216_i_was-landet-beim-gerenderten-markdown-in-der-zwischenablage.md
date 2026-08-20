# Was landet beim gerenderten Markdown in der Zwischenablage: der gerenderte Text oder der Quelltext?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/decisions/260819-2216_*_wird-die-vorschauflaeche-auswaehlbar-und-was-genau-laesst-sich-auswaehlen.md`; `shared/decisions/260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md` (die Folgefrage, die aus dieser Antwort entstanden ist); `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C2); `crates/krk-ui/src/markdown.rs:187-203`; `crates/krk-ui/src/vorschaumodell.rs:211-219`

---

## Question

Bei gerendertem Markdown sind der Text, den der Nutzer sieht, und der Text, der in der Datei steht, zwei verschiedene Dinge. `# Überschrift` erscheint als große fette Zeile ohne Doppelkreuz, `[Text](Ziel)` als eingefärbter Verweis ohne Klammern und ohne Adresse. Markiert der Nutzer eine Stelle und kopiert sie, ist zu entscheiden, welcher der beiden Texte in der Zwischenablage landet.

Die Frage stellt sich allein für gerendertes Markdown. Bei rohem Text, eingefärbtem Quelltext, Metadaten, einem Hinweis und dem Text aus der Zwischenablage sind Anzeige und Quelle dasselbe.

## Options

1. **Der gerenderte Text, so wie er dasteht.**
   - Folge: Auswahl und Anzeige bleiben dieselbe Sache. AppKit legt ab, was markiert ist, und KRK setzt dafür nichts. Kein zweiter Textspeicher, keine Abbildung, kein Eingriff in den Kopierweg.
   - Preis: wer eine Markdown-Datei in der Vorschau liest und einen Absatz in eine andere Datei übernehmen will, verliert dabei jede Auszeichnung. Der Verweis verliert seine Adresse, und die ist nicht wiederherstellbar.

2. **Der Quelltext mit den Auszeichnungszeichen.**
   - Folge: der Nutzer kopiert das Markdown, das dasteht, und kann es woanders einfügen. Die Adresse eines Verweises fährt mit.
   - Preis: braucht einen zweiten Textspeicher neben dem gerenderten und eine eigene Abbildung von Auswahl auf Quelltextstellen. Schließt aus, dass Auswahl und Anzeige dieselbe Sache sind.

3. **Beides, über zwei Wege**: das Kopieren nimmt den gerenderten Text, ein zweiter Befehl den Quelltext.
   - Folge: der Nutzer wählt je Fall.
   - Preis: ein zweiter Kopierbefehl mit eigener Kombination, eigenem Menüeintrag und eigener Erklärung. Zwei Wege für eine Handlung, und der Nutzer muss vorher wissen, welchen er braucht.

## Constraints

- Es entsteht keine zweite Hülle um `NSPasteboard`.
- Die Abbildung muss im Durchgang entstehen, der ohnehin rendert; ein zweiter Durchgang über die Quelle liefe innerhalb der Endbedingung von L7.
- Die Stellen des gerenderten Textes zählen UTF-16-Einheiten, die Quellbereiche von `pulldown-cmark` zählen Bytes.

## Recommendation

**Wir haben Möglichkeit 1 empfohlen** und die Empfehlung mit dem Aufwand der zweiten begründet. Der Nutzer hat anders entschieden, mit der Kostenbeschreibung vor Augen.

## Antwort 260819-2210

**Möglichkeit 2.** Der Quelltext mit den Auszeichnungszeichen landet in der Zwischenablage. Der Nutzer hat die Kosten ausdrücklich angenommen: ein zweiter Textspeicher neben dem gerenderten, eine eigene Abbildung von Auswahl auf Quelltextstellen, und Auswahl und Anzeige sind nicht mehr dieselbe Sache.

**Eine Folge dieser Antwort stand in der Optionsbeschreibung nicht und ist als eigener Datensatz vorgelegt**: die Abbildung ist an den Rändern einer Auswahl nicht eindeutig. Sie liegt als `shared/decisions/260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md` beim Nutzer.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented: `13be459`, `91f8727`, `17dad8a` — `Gerendert` traegt `quellbezug: Arc<Quellbezug>` (`crates/krk-ui/src/markdown.rs:271`), `Quellbezug::quelltext` (`:335`) rechnet die Auswahl auf den Quellausschnitt, und `Vorschautext::auswahl_ablegen` (`crates/krk-ui/src/appkit/vorschau.rs:445-461`) legt ihn ueber die eine Huelle ab. Die Quelle wird nicht ein zweites Mal gelesen: sie kommt aus `self.quelle.to_owned()` in `Zerlegung::abschliessen` (`markdown.rs:1594`). Abgeglichen am 260820-0834.
Deferred:
Superseded by:

# Wie weit reicht die Suche in der Nähe einer Textmarke, und was gilt, wenn der gemerkte Text nicht mehr auffindbar ist?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` §"4. Textmarke", `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C6), `crates/krk-core/src/ablage/lesezeichen.rs:51` (`Lesezeichen::gueltig`)

---

## Question

Der Nutzer hat am 260807-2139 festgelegt, woran eine Textmarke hängt: an einer Zeilennummer plus dem Textinhalt jener Zeile als Prüfung. Der Sprung geht zur gemerkten Zeile, prüft den dort gemerkten Text und sucht bei Abweichung "in der Nähe".

"In der Nähe" trägt in dieser Form kein Abnahmekriterium. Zwei Größen fehlen, und beide bestimmen, ob eine Marke eine Änderung von außen überlebt, was der ganze Zweck der Festlegung ist.

Die erste ist die Reichweite. Sucht KRK zehn Zeilen um die gemerkte herum, überlebt eine Marke das Einfügen eines Absatzes darüber, nicht aber das Einfügen eines neuen Abschnitts von hundert Zeilen. Sucht KRK die ganze Datei, findet sie die Stelle fast immer, kann aber die falsche finden, denn eine Zeile wie `}` oder eine Leerzeile kommt hundertfach vor.

Die zweite ist der Fehlschlag. Wird der gemerkte Text nirgends gefunden, muss die Marke etwas tun, und die drei Möglichkeiten unterscheiden sich für den Nutzer deutlich: an die gemerkte Zeilennummer springen, gar nicht springen und sich als ungültig zeigen, oder springen und in der Statuszeile sagen, dass die Stelle nicht mehr stimmt.

Die zweite Größe bindet zugleich die gemeinsame Gültigkeitsprüfung der Leiste. `Lesezeichen::gueltig` prüft heute eine Sache, nämlich ob der Ordner noch da ist (`crates/krk-core/src/ablage/lesezeichen.rs:51`), und C5 der Runde 1 sagt zu, dass ein ungültiges Lesezeichen als solches markiert ist und die Auswahl den Grund nennt. Ob eine Textmarke ungültig heißt, wenn die Datei fehlt, oder auch dann, wenn der Text fehlt, entscheidet, was in der Leiste zu sehen ist.

## Options

1. **Feste Reichweite, Fehlschlag springt trotzdem** — KRK sucht den gemerkten Text in einem festen Fenster um die gemerkte Zeile, etwa fünfzig Zeilen in beide Richtungen. Findet es ihn nicht, springt es an die gemerkte Zeilennummer und meldet in der Statuszeile, dass die Stelle sich geändert hat. Ungültig heißt allein: die Datei fehlt.
   - Pro: eine Zahl, eine Regel, und die Marke führt immer irgendwohin. Die Gültigkeitsprüfung bleibt so einfach wie heute, nämlich eine Frage an das Dateisystem, die ohne Lesen der Datei zu beantworten ist.
   - Contra: die Zahl ist gegriffen. Bei einer Datei, die um mehr als fünfzig Zeilen gewachsen ist, landet der Nutzer irgendwo, mit einem Hinweis, den er leicht übersieht.

2. **Ganze Datei, erster Treffer, Fehlschlag springt trotzdem** — gesucht wird in der ganzen Datei, und zwar von der gemerkten Zeile aus nach außen, sodass der nächstgelegene Treffer gewinnt. Der Fehlschlag verhält sich wie in Möglichkeit 1.
   - Pro: keine Zahl zu wählen und zu begründen. Eine verschobene Stelle wird gefunden, gleich wie weit sie gewandert ist.
   - Contra: bei einer Zeile ohne Eigenart, etwa `}` oder einer leeren Zeile, findet die Suche die nächstgelegene und nicht die gemeinte. Das ist kein Fehler der Umsetzung, sondern eine Grenze der Regel selbst: der gemerkte Zeileninhalt ist nicht eindeutig, und keine Reichweite macht ihn eindeutig.

3. **Ganze Datei, und die Marke merkt sich mehr als eine Zeile** — gemerkt werden die Zeile und ihre beiden Nachbarn, gesucht wird die Dreiergruppe in der ganzen Datei. Der Fehlschlag verhält sich wie in Möglichkeit 1.
   - Pro: löst das Eindeutigkeitsproblem aus Möglichkeit 2 weitgehend, denn drei aufeinanderfolgende Zeilen wiederholen sich in einer Datei selten.
   - Contra: `bookmarks.toml` wird für jede Marke dreimal so groß, und die Datei ist nach dem Vorbild der übrigen drei Ablagedateien von Hand lesbar gedacht. Die Regel ist zudem schwerer zu erklären als beide anderen.

## Constraints

- Die Festlegung des Nutzers vom 260807-2139 steht: Zeilennummer plus Textinhalt, Sprung zur Zeile, Prüfung, bei Abweichung Suche in der Nähe. Zur Debatte steht allein, was "in der Nähe" und was der Fehlschlag heißt.
- Die Gültigkeitsprüfung der Leiste ist gemeinsam für Ordnermarken und Textmarken. Sie wird bei jedem Neuaufbau der Liste gestellt und nach jedem Ein- und Aushängen eines Datenträgers, nicht bei jedem Zeichendurchgang. Eine Antwort, die dafür jede gemerkte Datei öffnen und lesen muss, macht aus einer Frage an das Dateisystem einen Lesevorgang je Marke.
- `bookmarks.toml` bleibt von Hand lesbar, wie die drei übrigen Ablagedateien.
- Die Zeitzusagen aus C8 der Runde 1 bleiben unberührt; diese Runde setzt keine eigene, und die Marke darf keine der zehn bestehenden berühren.

## Recommendation

Wir empfehlen Möglichkeit 1, und zwar wegen der Randbedingung zur Gültigkeitsprüfung, nicht wegen der Trefferquote. Möglichkeit 2 und 3 finden mehr, aber beide beantworten die Frage "ist diese Marke gültig" erst nach dem Lesen der Datei, und die Leiste stellt diese Frage für jede Marke bei jedem Neuaufbau ihrer Liste. Möglichkeit 1 trennt beides sauber: gültig ist eine Frage an das Dateisystem, das Wiederfinden der Stelle geschieht beim Sprung und nur dort.

Zur Zahl selbst: fünfzig Zeilen ist ein Vorschlag, keine gemessene Größe. `inference:` Sie deckt die häufige Änderung ab, nämlich einen eingefügten oder gelöschten Abschnitt oberhalb der Marke, und verfehlt die seltene, nämlich eine umgebaute Datei. Wer die Zahl ändern will, ändert eine Konstante und keine Regel.

Ein Punkt ist unabhängig von der Wahl und gehört so oder so festgehalten: **der gemerkte Zeileninhalt ist keine eindeutige Kennung.** Eine Marke auf einer Zeile, die in der Datei mehrfach vorkommt, kann nach einer Änderung von außen nicht zuverlässig wiedergefunden werden, und keine der drei Möglichkeiten ändert daran etwas. Der Spec schreibt das als Grenze der Fähigkeit aus, statt sie durch eine größere Reichweite zu verdecken.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"10. Suche in der Nähe" — Möglichkeit 1 gewählt: festes Fenster von etwa fünfzig Zeilen um die gemerkte Zeile; wird der Text nicht gefunden, springt die Marke trotzdem an die gemerkte Zeilennummer und meldet die Abweichung in der Statuszeile. Ungültig heißt allein: die Datei fehlt. Tragender Grund ist die gemeinsame Gültigkeitsprüfung der Leiste, die damit eine Frage an das Dateisystem bleibt statt ein Lesevorgang je Marke. inference: fünfzig Zeilen ist ein Vorschlag, keine gemessene Größe. Als Grenze der Fähigkeit festgehalten: der gemerkte Zeileninhalt ist keine eindeutige Kennung. Entschieden vom Nutzer am 260808-0017.
Implemented: `6a9a872` und `0ad7f29` — `crates/krk-core/src/text/marke.rs:68` führt `NAHFENSTER = 50`, `:154` fährt das feste Fenster in beide Richtungen, und bei Fehlschlag springt die Marke an die gemerkte Nummer. „Ungültig heißt allein: die Datei fehlt" steht als reine Dateisystemfrage ohne Lesevorgang in `crates/krk-core/src/ablage/lesezeichen.rs:198` (`datei.is_file()`). Die Meldung der Abweichung geht in die Statuszeile über `crates/krk-ui/src/appkit/editor.rs:414`. Planschritt S12 trägt `[DONE]`. Nachgeprüft im Abgleich am 260810.

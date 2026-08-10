# Welche Dateien öffnet der Editor überhaupt, nach Typ und nach Größe?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C2 und C3), `shared/decisions/260802-0842_*_editor-formatansicht-je-dateityp.md`, `crates/krk-ui/src/vorschaumodell.rs:81` (die Textgrenze der Vorschau)

---

## Question

Die Directive nennt Text, Code und Markdown. Sie sagt nicht, was geschieht, wenn der Nutzer F4 auf einem Bild, einem Archiv, einer ausführbaren Datei oder einem Ordner drückt, und sie nennt keine Größengrenze. Beides muss vor dem ersten Planschritt feststehen, denn beides ist ein Abnahmekriterium von C2 und beides birgt einen Schaden, den kein späterer Schritt zurücknimmt.

Der Schaden ist konkret. Ein Editor, der eine Binärdatei als Text einliest, ersetzt beim Sichern jede Bytefolge, die keine gültige UTF-8-Zeichenkette ist, durch ein Ersatzzeichen. Die Datei ist danach zerstört, und der Nutzer hat nichts getan als F4 zu drücken. Bei der Größe steht ein zweiter Schaden: das Referenzgerät hat 16 GB Arbeitsspeicher, und eine Protokolldatei von mehreren Gigabyte vollständig einzulesen hält die Anwendung an.

Die Vorschau aus C6 hat für dieselbe Frage bereits eine Antwort: Text bis 1 MB als Inhalt, Bilder bis 64 MB als Bild, alles übrige als Metadaten. Die Frage hier ist, ob der Editor diese Antwort übernimmt oder eine eigene braucht, denn Ansehen und Bearbeiten sind nicht dieselbe Handlung. Wer eine Datei von 4 MB nur ansehen will, ist mit den Metadaten schlecht bedient; wer sie bearbeiten will, umso mehr.

## Options

1. **Der Editor erbt die Regel der Vorschau und weist alles übrige ab** — Text bis 1 MB wird geöffnet, alles andere meldet die Statuszeile mit dem Grund und öffnet nichts.
   - Pro: eine Regel für beide Flächen, eine Grenze, eine Zahl. Kein Weg, auf dem eine Binärdatei in einen Editor gerät.
   - Contra: 1 MB ist für einen Editor knapp. Eine Protokolldatei oder ein Datenauszug von wenigen MB ist genau der Fall, für den man einen eingebauten Editor haben will, und der Nutzer bekäme dafür eine Absage.

2. **Eigene, höhere Grenze für den Editor, gleiche Typregel** — der Editor öffnet Textdateien bis zu einer eigenen, höheren Grenze, etwa 16 MB, und weist alles Nichttextliche ab.
   - Pro: trifft die Fälle, für die ein eingebauter Editor da ist, ohne den Speicher zu füllen. Die Typregel bleibt dieselbe wie in der Vorschau.
   - Contra: zwei Zahlen für dieselbe Frage in zwei Flächen. Der Nutzer sieht eine Datei in der Vorschau als Metadaten und kann sie im Editor öffnen, was zunächst widersprüchlich wirkt.

3. **Der Editor öffnet jede Datei, zeigt Nichttextliches aber schreibgeschützt** — die Rohansicht zeigt die Bytes, das Sichern ist gesperrt, solange die Datei nicht als Text gelesen werden konnte.
   - Pro: kein Befehl, der ins Leere geht. F4 tut immer etwas.
   - Contra: eine zweite Sorte Editorzustand mit eigener Regel für das Sichern, also genau der Sonderfall, den die Maxime "supersimpel" ausschließt. Der Nutzen ist gering, denn eine Binärdatei in einem Textfenster liest niemand.

## Constraints

- Kein Weg darf eine Datei beim Sichern verändern, die der Editor nicht vollständig und verlustfrei als Text gelesen hat. Diese Grenze gilt unabhängig davon, welche der drei Möglichkeiten gewählt wird.
- Ein abgewiesenes F4 meldet den Grund. Die Statuszeile aus C1 der Runde 1 ist der dafür vorgesehene Ort und trägt fünf Ränge; eine zweite Meldefläche entsteht nicht.
- Die Antwort bindet zugleich den Übergang aus der Vorschau: dort steht eine Datei bereits fest, und der Übergang muss dieselbe Prüfung anlegen wie F4, sonst gibt es zwei Wege mit zwei Regeln.
- Der Ordner ist der eine Fall, der sicher abgewiesen wird: er hat keinen Inhalt, den ein Texteditor zeigen könnte. Er braucht in keiner der drei Möglichkeiten eine eigene Regel.

## Recommendation

Wir empfehlen Möglichkeit 2. Die 1 MB der Vorschau sind für das Ansehen gesetzt worden, nicht für das Bearbeiten, und sie mit derselben Begründung auf den Editor zu übertragen wäre eine Übernahme ohne Prüfung. Der Preis der zweiten Zahl ist gering, solange beide Zahlen dieselbe Regel tragen, nämlich eine Obergrenze für das vollständige Einlesen in den Arbeitsspeicher; verschieden ist allein, wie viel die jeweilige Handlung rechtfertigt.

Die genaue Zahl ist zweitrangig gegenüber der Frage, ob es eine eigene gibt. 16 MB ist ein Vorschlag und keine gemessene Größe: er liegt weit unter dem, was das Referenzgerät verkraftet, und weit über den Dateien, die man von Hand bearbeitet. `speculation:` Eine Messung, ab welcher Dateigröße das Öffnen im Editor spürbar wird, gibt es nicht, und sie wäre ohne den ausgeklammerten Abnahmelauf auch nicht abzunehmen.

Möglichkeit 3 empfehlen wir nicht. Sie erkauft einen Befehl, der nie ins Leere geht, mit einem zweiten Editorzustand samt eigener Sicherungsregel, und das Ergebnis liest niemand.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"7. Welche Dateien der Editor öffnet" — Möglichkeit 2 gewählt: eigene höhere Grenze für den Editor, etwa 16 MB, gleiche Typregel wie die Vorschau. Alles Nichttextliche und alles Größere wird abgewiesen, mit Grund in der Statuszeile; der Übergang aus der Vorschau legt dieselbe Prüfung an wie F4. Zwei Zahlen für dieselbe Frage sind angenommen (Vorschau 1 MB, Editor 16 MB), beide tragen dieselbe Regel. speculation: die 16 MB sind ein Vorschlag und keine gemessene Größe. Bindend unabhängig von der Zahl: kein Weg darf eine Datei beim Sichern verändern, die der Editor nicht vollständig und verlustfrei als Text gelesen hat. Entschieden vom Nutzer am 260808-0017.
Implemented: `ff6dd25` — `crates/krk-core/src/text/datei.rs:136` führt `EDITORGRENZE = 16 * 1024 * 1024`, `:153` sichert beim Übersetzen zu, dass sie über der Vorschaugrenze liegt. `:167` führt `Abweisung` mit drei Werten ohne Auffangzweig, darunter `NichtAlsTextLesbar`; das ist die bindende Zusage, dass keine Datei beim Sichern verändert wird, die nicht verlustfrei als Text gelesen wurde. Die Prüfung läuft vor dem Lesen und liest höchstens `EDITORGRENZE + 1` Bytes. Es gibt genau **eine** Aufrufstelle im Programm, `crates/krk-ui/src/editormodell.rs:456`; der Übergang aus der Vorschau geht denselben Weg (`crates/krk-ui/src/appkit/anwendung.rs:3333`). Planschritte S10 und S23 tragen `[DONE]`. Nachgeprüft im Abgleich am 260810.

Zwei offene Defekte beschreiben Grenzen dieser Umsetzung und nicht eine fehlende Umsetzung: `issues/260809-1610_*_die-zusicherung-editorgrenze-groesser-textgrenze-laesst-sich-in-krk-core-nur-halb-schreiben.md` und `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`.

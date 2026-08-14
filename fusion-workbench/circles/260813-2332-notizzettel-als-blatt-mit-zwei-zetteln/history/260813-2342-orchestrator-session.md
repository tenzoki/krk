# Orchestrator-Sitzung — 260813-2342

**Directive:** Ein Notizzettel als Blatt am Hauptfenster: zwei Zettel als anklickbare Tabs, nackte Textfläche, gesichert bei Tabwechsel, Schließen und Beenden, geholt mit `f2` oder `cmd+k`, geschlossen mit `Esc`, zwei einzelne Dateien im Ablageordner.
**Modus:** (Phase 0 offen)
**Status:** Läuft

## Aufnahme beim Start

| Größe | Wert |
|---|---|
| Aktiver Circle | 260813-2332-notizzettel-als-blatt-mit-zwei-zetteln (aktiviert 23:41 über /fusion:next mit Namen) |
| git HEAD | 6d05bef |
| Turn-Budget | 5 |
| Erkannte Domäne | code (132 Quelldateien, 11 Datendateien, git-ls-files) |
| Offene Defekte | 0 im Circle, 10 in shared |
| Offene Fragen | 0 im Circle, 7 in shared |
| Offene Pläne | 1 in shared |
| Guard | haltActive: false |
| Circles | 1 aktiv, 1 vorgesehen, 7 beschränkt, 1 kohärent geschlossen |
| Arbeitswarteschlange | keine tasklist.md |

## Was diese Runde vorfindet

Sieben Klärungsfragen sind vor der Anlage des Circles beantwortet worden, in zwei Runden des shapers. Vor dem Spec steht damit keine offene Frage. Die Grounding-Aufnahme des Circle-Datensatzes trägt drei benannte Folgen — Absturzverlust, Überschreibgefahr bei zwei Instanzen, und die Verträglichkeit von nackter Fläche und Blattform — sowie drei Funde des shapers am Baum.

Vorgängersitzung: shared/history/260813-1006 gibt es nicht; die achte Runde lief unter circles/260813-0939-…/history/260813-1006-orchestrator-session.md und ist kohärent geschlossen.

## Drei Antworten am Spec-Tor (Nutzer, 260814-0005)

**`shift+cmd+w` bei stehendem Zettel: sichern, dann schließen.** Ein vierter Sicherungsmoment neben Tabwechsel, Schließen und Beenden. Der Grund ist die Logik der anderen drei: kein Weg aus dem Zettel heraus verliert Text. Die Ausnahmeliste bleibt unangetastet — `fenster_schliessen` steht seit dem 260813-1125 ausdrücklich darauf, und der Entscheid dazu wird nicht gekippt.

**Unlesbare Zetteldatei: beiseitelegen und mit einem leeren Zettel weiterarbeiten.** Möglichkeit 3 des Datensatzes, die Empfehlung des shapers. Es ist die Antwort, die dieses Projekt für `keymap.toml` und `settings.toml` schon gegeben hat: ein Tippfehler nimmt dem Nutzer die Datei nicht weg. Kein zweiter Zustand am Zettel, keine Sperre, keine Ausnahme im Sicherungsweg; der Preis ist ein sechster Aufrufer von `beiseite_legen` und eine Datei mehr im Ablageordner.

**Spec: erst nachziehen, dann freigeben.** Der shaper arbeitet die beiden Antworten ein, ergänzt das fehlende Abnahmekriterium für `shift+cmd+w` in C1 und berichtigt die beiden Bilder, die sich an der Stelle widersprechen, an der der dritte Sicherungsmoment hängt.

## Die Diagrammprüfung des Spec

Urteil `acceptable` (`reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`). Bild 1: zwölf Knoten, elf Kanten, kein Zyklus, kein Gott-Knoten. Bild 2: drei Zustände, zehn Übergänge; die drei Zyklen sind in einem Zustandsautomaten die Sache selbst. Beide mit mermaid-cli 11.16.0 nach SVG gerendert.

Fünf Befunde, zwei mittel. Der schwerere ist kein Zeichenfehler, sondern eine Lücke im Spec: `shift+cmd+w` kommt über die Ausnahmeliste bei stehendem Blatt durch und ruft `performClose` am Hauptfenster, und kein Kriterium in C1 deckte den Fall. Am Baum geprüft, nicht erschlossen.

**Ein Muster, das die Prüfung benennt:** dieselbe unvollständige Fallunterscheidung — eine Entscheidungsraute mit nur einem Ausgang — ist zum dritten Mal gezeichnet worden, und die zwei früheren Beanstandungen sind nie behoben worden (offener Datensatz `260813-1345_o_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-…` im Circle der achten Runde).

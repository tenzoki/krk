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

## Plan-Tor (Nutzer, 260814-0715)

**Plan freigegeben.** `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, sechzehn Schritte in sechs Strängen. Mit der Freigabe angenommen sind die zwei Namenswahlen des planners: die Dateien heißen `note-1.txt` und `note-2.txt` (englisch wie die vier bestehenden Ablagedateien), die Tabs „Zettel 1" und „Zettel 2" (deutsch, weil der Nutzer sie liest).

**Die zwei geweiteten Signaturen in `krk-core` sind genehmigt.** `atomar::schreiben` und `Zugang::beiseite_legen` nehmen künftig einen Leser statt einer Zeichenkette; fünf Aufrufstellen ziehen mit. Der Grund ist keine Bequemlichkeit: beide unlesbaren Fälle des Zettels tragen keinen `&str`, und eine Datei über `EDITORGRENZE` darf nicht in den Speicher. Die Alternative, den Zettel an `atomar::schreiben` vorbeischreiben zu lassen, ist verworfen — sie hätte eine zweite atomar schreibende Stelle angelegt, genau das, was die Probe `nur_benannte_dateien_erreichen_das_atomare_schreiben` verhindern soll.

**Die Abschaltung der Textautomatiken wird über eine Zählprobe am Baum abgesichert**, Möglichkeit 2 des Datensatzes: jede Datei mit `setEditable(true)` muss auch `automatiken_abschalten` nennen. Der blinde Fleck ist benannt und nicht verschwiegen — eine Fläche, die ihre Bearbeitbarkeit anders schreibt, entgeht der Nadel. Der eigene Typ um die bearbeitbare Fläche (Möglichkeit 3) ist als der teuerste verworfen.

## Die Diagrammprüfung des Plans

Urteil `acceptable` (`reviews/260814-0711-conceptrev-plan-notizzettel-als-blatt-mit-zwei-zetteln.md`). **Der Fehler ist nicht zum vierten Mal passiert:** die einzige Entscheidungsraute des Plans trägt vier Ausgänge, und der Absatz darunter behauptet die Vollständigkeit ausdrücklich. Der Arbeitsgraph deckt sich Kante für Kante mit den Abhängigkeitszeilen der Schrittliste — sechzehn Schritte, neunzehn Kanten, kein Zyklus.

Zwei Befunde, beide Auszählungen. Der eine liegt beim Spec: er zeichnet auf dem `Esc`-Weg erst das Sichern und dann das Schließen, der Plan hängt es hinter das Schließen; belegt ist die Fassung des Plans. Kein Abnahmekriterium bindet die Reihenfolge, der Nutzer hat auf einen weiteren shaper-Lauf verzichtet. **Der Spec bleibt damit an dieser Stelle ungenau, und das ist eine bewusste Entscheidung, keine Lücke.**

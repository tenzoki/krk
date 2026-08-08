# Shaper — Spec-Überarbeitung nach den sechs Festlegungen (260808-0021)

**Auftrag:** Den Spec `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` überarbeiten. Sechs beantwortete Fragen einarbeiten, die drei Befunde der Diagrammprüfung beheben, Status auf fertig zur Abnahme setzen.
**Modus:** in-circle, ohne Rückfragewerkzeug
**Status:** Complete

## Was eingearbeitet wurde

Die fünf Datensätze unter `decisions/` stehen seit dem 260808-0017 auf `_a_`, dazu eine sechste Antwort, die sich aus der ersten ergab. Alle sechs sind gelesen, ebenso `history/260807-2139-orchestrator-session.md` §"Die sechs Festlegungen der Spec-Runde" und der Prüfbericht `reviews/260807-2202-conceptrev-spec-eingebauter-editor-mit-textmarken.md`.

**C3, Formatansicht.** Die Syntaxhervorhebung kommt aus einer fertigen Rust-Kiste; welche, entscheidet der Planner. Sie wird die fünfte fremde Kiste mit Wirkung auf die Anwendung und bekommt eine geschriebene Begründung in `Cargo.toml`, wie die vier bestehenden (geprüft: `serde`, `toml`, `icu_collator`, `signal-hook` und die `objc2`-Familie tragen dort je einen Begründungsblock). Die einklappbaren Blöcke sind aus der Runde herausgenommen und stehen jetzt unter `## Ausdrücklich außerhalb dieser Runde`. Zwei Abnahmekriterien sind hinzugekommen, die vorher nirgends standen: der Rückfall auf die Textdarstellung bei einer unbekannten Sprache, und die Lesbarkeit in Hell und Dunkel. Die zweite ist eine Ableitung aus `crates/krk-ui/src/appkit/leiste.rs:441` und dem Modulkopf von `tableiste.rs`, wo zweimal begründet steht, dass KRK das Erscheinungsbild nicht selbst nachbaut.

**C2, welche Dateien der Editor öffnet.** Rund 16 MB, nur Text. Die Prüfung steht jetzt im Titel der Fähigkeit und trägt sieben Abnahmekriterien statt zweier. Was "als Text lesbar" heißt, ist am gebauten Code abgelesen statt neu definiert: die Vorschau wandelt über `String::from_utf8` (`crates/krk-ui/src/vorschaumodell.rs:522-527`), und der Editor legt dieselbe Regel an. Die Größenprüfung steht vor dem Lesen, so wie `TEXTGRENZE` und `BILDGRENZE` es in derselben Datei halten.

**C4, Nachfrage bei der Sitzungssicherung.** Der dritte Anlass fällt mit dem zweiten zusammen. Aus fünf Anlässen, von denen einer offen war, sind vier feste geworden. Zwei Abnahmekriterien sind hinzugekommen: dass die getaktete Sicherung nichts fragt, und dass die Sitzung festhält, welche Datei offen ist, ohne den ungesicherten Stand mitzutragen.

**C6, Textmarke.** Nur eine Stelle, kein Bereich. Suchfenster rund fünfzig Zeilen, Fehlschlag springt trotzdem und meldet, ungültig heißt allein: die Datei fehlt. Fünf Abnahmekriterien sind hinzugekommen, darunter zwei Ableitungen, die als solche gekennzeichnet sind: der nächstliegende Treffer gewinnt bei Mehrfachvorkommen im Fenster, und eine Änderung im Editor selbst zieht keine Marke nach.

## Die drei Diagrammbefunde

**Befund 1, Diagramm 2 mischte drei Gegenstände.** Aufgeteilt in ein `erDiagram` für die Ablageform und einen `flowchart` für den Sprung. Der Flowchart hat dabei einen Zweig bekommen, der vorher fehlte: die Prüfung aus C2 greift auch beim Sprung auf eine Marke, und "abgewiesen" ist etwas anderes als "ungültig".

**Befund 2, C4 trug kein Diagramm.** Ein `stateDiagram-v2` über vier Zustände ist hinzugekommen, im Abschnitt `## Aufbau dieser Runde` neben den übrigen. Aus dem Zustand `Nachfrage steht` führen genau drei Kanten heraus, so viele wie das Blatt Wahlmöglichkeiten anbietet.

**Befund 3, die zwei Kanten aus `Editor` waren unbedingt gezeichnet.** Beide tragen jetzt einen Vorbehalt im Kantenlabel, dazu eine Notiz am Zustand, die auf C4 verweist. Befund 4 (kosmetisch, dieselbe Auslösung zweimal verschieden beschriftet) ist mit erledigt.

Alle fünf Diagramme des überarbeiteten Specs sind mit `@mermaid-js/mermaid-cli` 11.16.0 gegengelesen und parsen.

## Was neu offen ist

Ein Datensatz: `decisions/260808-0021_o_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`. Die bindende Zusage vom 260808-0017 regelt das Lesen vollständig und sagt über das Zurückschreiben des unangetasteten Teils nichts. Betroffen sind Zeilenenden, der abschließende Zeilenumbruch und eine Bytefolgenmarke am Dateianfang. Drei Möglichkeiten, Empfehlung ist die erste. Die Frage hält keinen Planschritt auf und bindet den Schritt, der das Sichern baut.

Ein Defekt: `issues/260808-0021_o_die-fuenf-beantworteten-datensaetze-tragen-zwei-answered-zeilen-und-einen-veralteten-kopf.md`. Alle fünf am 260808-0017 beantworteten Datensätze führen einen leeren Vorlagenblock über der gefüllten `Answered:`-Zeile und tragen im Kopf weiterhin `**Status:** open`.

## Was der Shaper nicht angefasst hat

Der Circle-Datensatz `_t_circle.md`. Die zu streichende Stelle ist im Spec unter `## Abgleich mit der Circle-Directive` benannt und dem Nutzer im Bericht genannt: Zeile 14, die drei Wörter " und Textbereiche". Zwei weitere Stellen im `## Grounding snapshot` (Zeile 85 und Zeile 126) sagen dasselbe und halten den Kenntnisstand vom 260807-2116 fest; ob sie mitgezogen werden, entscheidet der Nutzer.

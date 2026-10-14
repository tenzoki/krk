# Gilt die Quelltextzusage auch für das Ziehen einer Auswahl und für die Dienste des Systems?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `shared/decisions/260819-2216_*_was-landet-beim-gerenderten-markdown-in-der-zwischenablage.md`; `shared/decisions/260819-2216_*_welches-kontextmenue-zeigt-die-auswaehlbare-vorschau.md`; `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C2.12); `crates/krk-ui/src/appkit/zwischenablage.rs` (die eine Hülle um `NSPasteboard`)

---

## Question

Eine auswählbare Textansicht gibt ihren markierten Text auf mehr als einem Weg heraus. Neben `cmd+c` und dem Menüeintrag „Kopieren" stehen das **Ziehen der Auswahl mit der Maus** in ein anderes Programm und die **Dienste des Systems**, die im Kontextmenü unter „Dienste" stehen und mit dem markierten Text arbeiten. Der Nutzer hat für die Zwischenablage entschieden, dass bei gerendertem Markdown der Quelltext herausgeht. Zu entscheiden ist, ob dieselbe Zusage für die übrigen Wege gilt.

Die Frage ist nicht bloß Vollständigkeit. Fällt die Antwort auf „nur die Zwischenablage", dann gibt dieselbe Fläche denselben Text auf zwei Wegen verschieden heraus, und der Nutzer sieht dem Weg nicht an, welchen er gerade nimmt.

## Options

1. **Eine Stelle für alle Ausgabewege.** Was die Vorschau aus einer Auswahl herausgibt, entsteht an genau einer Stelle, und jeder Weg fragt sie.
   - Folge: der Nutzer bekommt überall dasselbe. Beim Ziehen und bei einem Dienst gilt dieselbe Zusage wie beim Kopieren.
   - Preis: die Stelle muss so tief liegen, dass alle Wege durch sie gehen. In AppKit ist das die Methode, mit der eine Textansicht ihre Auswahl auf eine Ablage schreibt; ob sie wirklich alle Wege trägt, ist am Bündel nachzusehen und nicht aus der Dokumentation zu erschließen.

2. **Nur die Zwischenablage.** Kopieren gibt den Quelltext heraus, Ziehen und Dienste geben den gerenderten Text.
   - Folge: der Eingriff bleibt auf den Kopierweg beschränkt und ist mit Sicherheit dort, wo er sein soll.
   - Preis: zwei Antworten auf eine Frage, und der Unterschied ist am Bündel nicht zu erkennen. Wer den Text mit der Maus in einen Editor zieht, verliert die Auszeichnungen, die er beim Kopieren bekommen hätte.

3. **Das Ziehen abschalten.** Die Auswahl lässt sich nicht aus der Vorschau ziehen; Dienste bleiben, wie AppKit sie liefert.
   - Folge: ein Weg weniger, über den etwas Falsches herausgehen kann.
   - Preis: eine Bedienung, die der Nutzer von einer Textansicht kennt, fehlt ohne sichtbaren Grund. Für die Dienste löst die Möglichkeit nichts.

## Constraints

- Es entsteht keine zweite Hülle um `NSPasteboard`.
- Für die fünf übrigen Inhalte der Vorschau, in denen Anzeige und Quelle dasselbe sind, ist die Frage gegenstandslos: dort gibt jeder Weg dieselben Zeichen heraus.

## Recommendation

**Wir empfehlen Möglichkeit 1**, mit einem Vorbehalt: ob eine einzige Stelle wirklich alle Wege trägt, ist am laufenden Bündel zu prüfen. Trägt sie es nicht, ist Möglichkeit 2 die ehrlichere Antwort, und dann gehört der Unterschied in die Abnahmekriterien und nicht in eine Fußnote.

## Antwort 260819-2242

**Möglichkeit a.** Eine Stelle für alle Ausgabewege.

Zwischenablage, Ziehen mit der Maus und die System-Dienste liefern denselben Quelltext. Eine Regel, ein Ort im Code. Der Vorbehalt gehört dazu: ob eine Stelle wirklich alle Wege trägt, ist am gebauten Bündel zu prüfen und nicht an einer Probe.

## Abgleich 260820-0834 — der Marker bleibt auf beantwortet

**Die eine Stelle steht, dass sie alle Wege traegt, ist nicht gemessen.** Gebaut ist genau,
was die Antwort verlangt: `Vorschautext::auswahl_ablegen`, die Ueberschreibung von
`writeSelectionToPasteboard:types:` (`crates/krk-ui/src/appkit/vorschau.rs:445-461`), ist die
einzige Abfangstelle im Baum, und die Zaehlprobe
`die_abfangstelle_steht_im_baum_genau_einmal` (`vorschau.rs:1765`) haelt das fest.

**Zwei Gruende halten den Marker trotzdem auf `_a_`:**

- Der Plan der Runde sagt es selbst. Seine Tabelle `## Welcher Schritt welchen Datensatz
  realisiert` traegt fuer diesen Datensatz **erst nach der Buendelabnahme von C2.12**. Die
  Abnahme ist Nutzerarbeit und nicht gefahren.
- Der offene Durchsichtsbefund
  `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/issues/260820-0733_o_die-abfangstelle-verwirft-die-geforderten-sorten-und-leert-jede-gereichte-ablage.md`
  misst am Baum, dass die Stelle den Parameter `sorten` im Markdown-Zweig nicht liest und
  `text_auf_ablage_schreiben` unbedingt `clearContents()` ruft. Fuer die Zwischenablage des
  Nutzers ist beides richtig; fuer eine hereingereichte Ablage — die eines Ziehvorgangs oder
  eines Dienstes — ist es ungeprueft. Der Befund ist am 260820-0834 gegen `05cb614` nachgelesen
  und trifft unveraendert zu; jene Behebung hat allein `markdown.rs` angefasst.

Fuer die Zwischenablage ist die Zusage eingeloest. Fuer die zwei uebrigen Wege, um die dieser
Datensatz allein geht, ist sie es nicht. `_i_` waere hier die unehrlichere Auskunft.


---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented:
Deferred:
Superseded by:

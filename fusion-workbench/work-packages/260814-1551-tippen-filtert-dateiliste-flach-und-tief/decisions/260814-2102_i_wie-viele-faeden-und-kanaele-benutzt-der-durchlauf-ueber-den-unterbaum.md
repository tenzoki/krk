# Wie viele Fäden und wie viele Kanäle benutzt der Durchlauf über den Unterbaum?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md` (Abschnitt `## Offen für den Planner`, dritter Punkt, und C3.1); `crates/krk-core/src/verzeichnis/leser.rs:100-160` (die Hülle, die der Durchlauf ein zweites Mal baut); `reviews/260814-1840-conceptrev-tippen-filtert-dateiliste-flach-und-tief.md` (Befund B3, der die Frage aus dem zweiten Bild des Spec herausgenommen hat)

---

## Question

Der Spec hält sich bei der Bauart des Durchlaufs absichtlich heraus. C3.1 verlangt allein, dass er nicht auf dem Hauptfaden läuft, gestückelt liefert und dieselbe Bauart benutzt wie der vorhandene Lesevorgang; das zweite Bild sagt nach der Nachbesserung vom 260814-1852 nichts mehr über Fäden. Die Frage ist damit dem Planner überlassen, und sie ist zu beantworten, bevor jemand `krk-core/src/verzeichnis/durchlauf.rs` schreibt: die Zahl der Fäden entscheidet, ob C3.6 („Je Tab läuft nie mehr als einer") eine Lesart braucht, und die Kanaltiefe entscheidet, ob der Arbeitsfaden nach jedem Befund bis zum nächsten Takt blockiert.

Dieser Datensatz entsteht beantwortet, weil der Spec die Frage ausdrücklich an den Planner gerichtet hat und der Plan sie im selben Zug beantwortet. Er steht als eigener Datensatz und nicht als Absatz im Plan, weil die Antwort eine Bauart festlegt, die über diese Runde hinaus bindet: wer später einen zweiten nebenläufigen Leser baut, findet hier die Begründung und nicht in einem Planschritt, der mit der Runde schließt.

## Options

1. **Ein Faden je zu entscheidendem Ordner, ein Kanal je Faden.** Jeder Unterordner des angezeigten Ordners bekommt seinen eigenen Arbeitsfaden.
   - Pro: die Ordner werden nebenläufig entschieden; ein großer Unterbaum verzögert die anderen nicht.
   - Kontra: ein angezeigter Ordner mit zweihundert Unterordnern erzeugt zweihundert Fäden und zweihundert Kanäle. C3.6 bräuchte dann eine Lesart, in der „einer" Durchläufe zählt und keine Fäden. Die Zuordnung der Befunde bräuchte eine Verwaltung, die es heute nicht gibt.

2. **Ein Faden je Tab, ein Kanal je Tab, die Auftragsliste beim Start vollständig übergeben.** Der Faden arbeitet die Ordner nacheinander ab und schickt je entschiedenem Ordner einen Befund.
   - Pro: C3.6 zählt Fäden, Kanäle und Durchläufe zugleich und braucht keine Lesart. Die Hülle ist die von `Lesevorgang`, Zeile für Zeile. Der Abbruch hat dieselben zwei Wege wie dort. Die Befunde tragen den Eintragsindex und brauchen keine Zuordnungsverwaltung.
   - Kontra: die Ordner werden nacheinander entschieden; ein Ordner mit großem Unterbaum ohne Treffer verzögert die nach ihm. Der Durchlauf kann erst beginnen, wenn der angezeigte Ordner fertig gelesen ist, sonst müsste er Aufträge nachgereicht bekommen.

3. **Ein Faden je Tab mit einem zweiten Kanal in die Gegenrichtung**, über den der Hauptfaden Aufträge nachreicht, während der angezeigte Ordner noch gelesen wird.
   - Pro: der Durchlauf beginnt, sobald der erste Stapel des Lesevorgangs da ist, und wartet nicht auf dessen Ende.
   - Kontra: ein zweiter Kanal, ein Wachstumsfall und eine Blockierbedingung mehr. Der Gewinn ist die Dauer des Lesevorgangs, auf dem Ordner mit 100.000 Einträgen rund 800 ms; für gewöhnliche Ordner liegt sie unter einem Takt.

## Constraints

- C3.1: keine zweite Lesemechanik neben dem vorhandenen Lesevorgang.
- C3.4: der Abbruch greift innerhalb von zwei Stapeln und wird an der Stapelgrenze geprüft, nicht beim Absteigen. Ein Ordner ohne Unterordner ist davon nicht ausgenommen.
- C3.6: je Tab läuft nie mehr als einer.
- C3.11 und C3.12: die Liste wächst während des Durchlaufs, und KRK hält nicht an.
- Der Hauptfaden räumt die Kanäle im vorhandenen Einzugstakt von 1/60 s leer und darf dabei nicht warten.

## Recommendation

Möglichkeit 2, und so fährt der Plan. Die entscheidende Eigenschaft ist, dass C3.6 unter ihr keine Lesart braucht: „einer" zählt Fäden, Kanäle und Durchläufe zugleich. Möglichkeit 1 kauft Nebenläufigkeit mit einer Fadenzahl, die der angezeigte Ordner bestimmt, und das ist kein Preis, den ein Kriterium verlangt. Möglichkeit 3 kauft die Dauer eines Lesevorgangs mit einem zweiten Kanal und einem Wachstumsfall; sie bleibt erreichbar, falls die Verzögerung auf großen Ordnern je auffällt.

**Die Kanaltiefe ist 1.024 Befunde, und die Zahl ist dieselbe wie beim Lesevorgang bei anderer Einheit.** Dort hält der Kanal einen Stapel, und der Grund ist der Speicher: ein tieferer Kanal hielte den Bestand eines Ordners mit 100.000 Einträgen ein zweites Mal. Ein Befund ist ein Eintragsindex und ein Wahrheitswert, also acht Byte; 1.024 davon kosten acht Kilobyte. Mit der Tiefe 1 blockierte der Arbeitsfaden nach jedem einzelnen Befund bis zum nächsten Takt, also bis zu 16 ms je entschiedenem Ordner, und ein Ordner mit zweihundert flach liegenden Treffern brauchte drei Sekunden für eine Arbeit von Millisekunden. Die Zusage aus C3.4 hängt nicht an dieser Tiefe, sondern an der Schleife im Arbeitsfaden, die das Abbruchkennzeichen an jeder Stapelgrenze liest.

---
Answered: `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Abschnitt `## Der Durchlauf: ein Faden je Tab, ein Kanal je Tab` und Schritt F1 — Möglichkeit 2, ein Arbeitsfaden je Tab, ein `sync_channel` mit 1.024 Plätzen je Tab, die Auftragsliste beim Start vollständig übergeben, Beginn erst nach dem Abschluss des Lesevorgangs des angezeigten Ordners.
Implemented:
Deferred:
Superseded by:

---
Implemented: `2cdd299` — `Durchlauf::neu` legt genau einen `sync_channel(STAPELGROESSE)` an und startet genau einen benannten Faden je Aufruf (`crates/krk-core/src/verzeichnis/durchlauf.rs:262-267`); `STAPELGROESSE` ist 1.024 (`crates/krk-core/src/verzeichnis/leser.rs:50`), also die im Datensatz vorgerechnete Kanaltiefe. Die Auftragsliste wird beim Start vollständig übergeben. Je Tab entsteht ein Durchlauf (`crates/krk-ui/src/tabs.rs:914`), damit ein Faden und ein Kanal je Tab — Möglichkeit 2 wie beantwortet. Abgeglichen am 260820-2056 gegen `f5300f4`.

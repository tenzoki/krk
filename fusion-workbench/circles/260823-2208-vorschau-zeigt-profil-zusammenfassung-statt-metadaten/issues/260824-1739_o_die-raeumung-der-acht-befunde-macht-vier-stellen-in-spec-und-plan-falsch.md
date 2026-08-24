Die Räumung der acht Befunde macht vier Stellen in Spec und Plan falsch

---

Das Räumen der acht Befunde der Durchsicht `reviews/260824-1700-ontorev-…` hat
`resources/default-readers.toml` an zwei Stellen inhaltlich geändert: das Profil „ein
Defektspeicher" trägt fünf Zeilen statt drei, und das Feldmuster der Zeile „Sitzung" ist ersetzt.
Vier Stellen in Spec und Plan beschreiben weiter den Stand davor. **Sie stehen in Dateien, die dem
`analyst` gehören, und sind deshalb nicht mitgeräumt worden.**

---

**Gemessen am 260824-1739 an der geänderten Datei.**

| Stelle | steht dort | ist |
|---|---|---|
| `planning/260824-0613_o_spec-…:280` (C5.4) | „erscheinen die Zahl der offenen, die Zahl der geschlossenen und die zehn jüngsten Titel" | fünf Zeilen: Datensätze, Offen, Geschlossen, Zurückgestellt, die jüngsten zehn |
| `planning/260824-0640_o_plan-…:235` (Tabelle `### Die fünf mitgelieferten Profile`) | „zwei Zählungen (offen, geschlossen), jüngste zehn" | vier Zählungen und die jüngsten zehn |
| `planning/260824-0640_o_plan-…:327` (Schritt 7, `Changes`) | `(?s)## Current\n\s*(.+?)\n` als das Muster auf `^orchestrator-live\.md$` | `## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)` |
| `planning/260824-0640_o_plan-…:229` | „Es sind seit der Antwort vom 260824-1505 achtzehn und nicht zwölf, je neun im gemeinsamen Speicher und im Circle" | 99 Ordner, neun Speichernamen unter `shared/` und fünf je Runde |

**Die vierte ist nicht neu, sondern dieselbe Rechnung, die der Befund
`issues/260824-1651_*_der-kopf-des-speicherprofils-nennt-achtzehn-orte-…` in der TOML-Datei
berichtigt hat.** Der Kommentarkopf der Auslieferungsfassung und diese Planzeile sagten dasselbe,
und geräumt ist bislang nur der Kommentar. Gemessen mit `regex` 1.13.1 gegen alle 154
Verzeichnisse unter `fusion-workbench/`: 99 Treffer, davon 9 unter `shared/` und 90 in den achtzehn
Runden. `backlog`, `consult`, `investigations` und `memos` kommen in einer Runde nicht vor
(`rules/fusion-workbench-conventions.md:78`, gemessen null Vorkommen).

**Eine Zahl in C5.4 ist unberührt und bleibt richtig:** „54 offene von 82" für `shared/issues`
stimmt am 260824-1739 nach wie vor, nachgezählt 54 offen, 27 geschlossen, 1 zurückgestellt, 82
zusammen. Falsch ist allein die Aufzählung der Zeilen daneben.

**Zur dritten Stelle gehört eine Formfrage, die dieser Datensatz nicht entscheidet.** Der Plan hält
den Wortlaut eines Schrittes bewusst auf dem gebauten Stand fest und trägt spätere Änderungen als
Notiz nach; `planning/260824-0640_o_plan-…:245` sagt das für Schritt 7 ausdrücklich. Ob Schritt 7
also umgeschrieben wird oder eine Notiz bekommt, gehört dem `analyst`.

Gefunden beim Räumen der acht Befunde, `history/260824-1739-ontocoder-die-acht-befunde-der-auslieferungsfassung.md`.

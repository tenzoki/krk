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

---
Resolved: Alle vier Stellen sind am 260824-1751 nachgezogen, jede in der Form, die dieser Circle
für ihre Datei festgelegt hat. Im **Spec** steht die Berichtigung neben dem freigegebenen Wortlaut,
im **Plan** ist der Text ersetzt und die alte Fassung in der Klammer aufgehoben.

| Stelle | was jetzt dasteht |
|---|---|
| `planning/260824-0613_o_spec-…:280` (C5.4) | Wortlaut unverändert, dahinter der Verweis „Am 260824-1751 berichtigt". Die Berichtigung steht als `:294` unter der Kriterienliste von C5 und schreibt die fünf gebauten Zeilen aus, mitsamt dem Grund des Nutzers und der Messung 622 = 178 + 440 + 4 + 0. |
| `planning/260824-0640_o_plan-…:229` | „Das Speicherprofil zählt seit der Antwort vom 260824-1505 neun Speichernamen auf und nicht sechs", dazu die fünf, die in einer Runde stehen können, und die 99 getroffenen Ordner als Stand und nicht als Zusage. Die alte Fassung steht in der Klammer. |
| `planning/260824-0640_o_plan-…:235` (Profiltabelle) | „vier Zählungen (Datensätze, offen, geschlossen, zurückgestellt), jüngste zehn", mit der alten Fassung in der Klammer, nach dem Vorbild der Zeile „Ein Speicher" darüber. |
| `planning/260824-0640_o_plan-…:331` (Schritt 7) | **nicht umgeschrieben.** Der Schritt steht auf `[DONE]`, und sein Wortlaut hält den gebauten Stand vom 260824-1313 fest. Er trägt jetzt einen Nachtrag, der beide nach dem Bau ersetzten Angaben benennt und den Ersatz Schritt 14 und der Räumung der Durchsicht zuschreibt. |

**Die Formfrage aus diesem Datensatz ist damit beantwortet**, und zwar gegen das Umschreiben.
Schritt 7 hat gebaut, was er vorschreibt; eine Anweisung, die die heutige Datei nicht erfüllt,
entstünde erst durch das Umschreiben. Dieselbe Überlegung steht seit dem 260824-1508 in
`planning/260824-0640_o_plan-…:245` für die vierte Zustandszeile.

**Der Nachtrag hält beim Sitzungsmuster ausdrücklich fest, dass der Vorschlag aus
`issues/260824-1650_*_…` nicht übernommen ist**, weil er gemessen einen dritten falschen Ausgang
hatte, und nennt den Preis des gebauten Ausdrucks: `regex` kennt kein `(?!…)`, also wird eine
Wertzeile, die selbst mit `#` beginnt, nicht gelesen.

**Eine fünfte Stelle ist beim Lesen dazugekommen und mitgenommen worden**, dieselbe Räumung und
derselbe Planschritt: `planning/260824-0640_o_plan-…:327` schreibt für das Rundenprofil „den
jüngsten zehn auf `ordner = "history"`", und die Zeile trägt seit `942172b` zusätzlich
`muster = '\.md$'` (`issues/260824-1654_*_die-verlaufszeile-des-rundenprofils-traegt-kein-muster-…`).
Sie steht als zweiter Punkt im selben Nachtrag und hat keinen eigenen Datensatz bekommen.

**Eine sechste ist derselbe Commit, aber kein Bauauftrag.** Die Zahl „54 offene von 82" in C5.4
stimmte am 260824-1739 noch, wie dieser Datensatz oben festhält. Am 260824-1751 sind es 55 offene
von 83, weil `942172b` selbst mit
`shared/issues/260824-1745_o_ein-commit-des-orchestrators-nimmt-die-git-mv-…` einen weiteren
offenen Datensatz angelegt hat. Der Wortlaut bleibt stehen: der Plan führt diese Zahlen
ausdrücklich als Stände vom 260824, die sich mit jeder Sitzung ändern und in keiner Probe stehen
(`## Nutzerarbeit` Punkt 7, `## Testing Strategy`). Der Spec sagt es jetzt bei C5.4 dazu.

**Was bewusst stehen bleibt.** Die Beschreibung von B4 im Spec (`:192`, „Der Wert ist „ja" oder
„nein"") nennt den dritten Ausgang nicht, den der Kommentar der Auslieferungsfassung mit
`issues/260824-1653_*_…` bekommen hat. Sie beschreibt keinen überholten Stand: der Baustein hat
sich nicht geändert, und C3.12 deckt den Platzhalter für alle vier Bausteine ab. Ebenso bleiben die
Zahlen in `planning/260824-0640_o_plan-…` unter `## Nutzerarbeit` und `## Testing Strategy` stehen,
die sich dort selbst als Stände vom 260824 ausweisen.

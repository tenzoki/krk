Die Zeile „Sitzung" liefert bei leerem `## Current`-Abschnitt die nächste Überschrift statt des Platzhalters

---

Das Feldmuster `(?s)## Current\n\s*(.+?)\n` (`resources/default-readers.toml:193`) überspringt mit
seinem gierigen `\s*` die Leerzeile eines leeren `## Current`-Abschnitts und fängt die nächste
Überschrift. Die Zusammenfassung der Werkbankwurzel zeigt dann „Sitzung: ## This Turn". Der
Kommentarkopf desselben Profils (`:168-170`) und C5.8 sagen für genau diesen Fall den Platzhalter
zu.

---

**Gemessen am 260824-1650 mit `regex` 1.13.1**, derselben Fassung, die `Cargo.lock` führt, in einem
Wegwerfprogramm außerhalb des Baumes. Fünf Gestalten von `orchestrator-live.md` gegen das
ausgelieferte Muster und gegen einen Vorschlag:

| Gestalt | ausgeliefert `(?s)## Current\n\s*(.+?)\n` | Vorschlag `## Current\n[^\S\n]*([^\n]+)` |
|---|---|---|
| die echte Datei dieser Werkbank | `[RUNNING] coderev -> Durchsicht abe1a31..HEAD, …` | dasselbe |
| Vorlage aus `/fusion:setup`, `## Current` als letzter Abschnitt, mit Zeilenende | `[SETUP] orchestrator -> New session starting...` | dasselbe |
| dieselbe Vorlage **ohne** Zeilenende am Dateiende | **kein Treffer** (Platzhalter, obwohl der Wert dasteht) | `[SETUP] orchestrator -> New session starting...` |
| `## Current` leer, `## This Turn` folgt | **`## This Turn`** | kein Treffer (Platzhalter) |
| Überschrift umbenannt | kein Treffer (Platzhalter) | kein Treffer (Platzhalter) |

Der Vorschlag trägt genau eine Fanggruppe und hält damit C3.10.

**Zwei Ausgänge sind falsch, und beide auf dieselbe Weise verkehrt herum.** Der leere Abschnitt
liefert einen Wert, wo der Platzhalter zugesagt ist; der fehlende Zeilenumbruch am Dateiende
liefert den Platzhalter, wo ein Wert dasteht. Der erste ist der schwerere: „## This Turn" sieht
aus wie eine Auskunft über die Sitzung und ist keine.

**Wie oft die zwei Gestalten wirklich vorkommen, ist nicht gemessen.** `orchestrator-live.md`
steht in `.gitignore:16`, also gibt es keine Aufzeichnung früherer Stände in diesem Baum. Beide
Gestalten sind aus den ausgelieferten Vorlagen abgeleitet: `skills/setup/SKILL.md:107-114` und
`skills/next/SKILL.md:234-241` schreiben `## Current` als **letzten** Abschnitt der Datei, und
keine der beiden Vorlagen sagt, ob die Datei auf ein Zeilenende endet.

**Das `(?s)` trägt an dieser Stelle nichts.** Vor einem `\n` hinter einer nicht gierigen `.+?`
endet der Treffer ohnehin an der ersten Zeilengrenze; die Angabe steht als Beispiel in einer Datei,
die dem Nutzer unter `:112-117` beibringt, `(?s)` heiße „über Zeilengrenzen hinweg greifen". Wer
sie fallen lässt, ändert am gemessenen Verhalten nichts.

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

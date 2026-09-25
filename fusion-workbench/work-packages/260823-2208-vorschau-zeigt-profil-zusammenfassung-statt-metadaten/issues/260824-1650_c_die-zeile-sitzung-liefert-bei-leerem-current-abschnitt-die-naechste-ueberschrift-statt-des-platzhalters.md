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

---
Resolved: `resources/default-readers.toml:210` trägt jetzt
`## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)`, eine Fanggruppe, und der Kommentarkopf des
Profils (`:178-187`) schreibt das Verhalten aus.

**Der Vorschlag dieses Datensatzes ist nicht übernommen worden, weil er gemessen einen dritten
falschen Ausgang hat.** Am 260824-1739 mit `regex` 1.13.1 in einem Wegwerfprogramm außerhalb des
Baumes gegen neun Gestalten von `orchestrator-live.md` gerechnet, den fünf dieses Datensatzes und
vier weiteren: der Vorlage aus `skills/next/SKILL.md:234-241`, einem leeren `## Current` mit
Leerzeile am Dateiende, einem leeren `## Current` ohne alles dahinter — und der Gestalt, die
`crates/krk-core/tests/leseprofil.rs` in `werkbankwurzel` schreibt, nämlich eine Leerzeile unter
`## Current` und **dann** der Wert. Auf die letzte liefert der Vorschlag den Platzhalter, obwohl
der Wert dasteht.

Nachgewiesen und nicht abgeleitet: mit dem Vorschlag in der Datei fällt
`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` mit
`left: … Nicht …  right: … Text("Schritt 12, die Zaehlproben") …`
(`crates/krk-core/tests/leseprofil.rs:2206`). Die Probe gehört dem `coder` und ist unberührt
geblieben.

Das übernommene Muster nimmt die erste Zeile unter `## Current`, die etwas trägt, überspringt
dabei Leerzeilen und hält vor der nächsten Überschrift an. Alle neun Gestalten gemessen: die
echte Datei dieser Werkbank, beide Vorlagen und die Vorlage ohne Zeilenende am Dateiende liefern
ihren Wert, die drei leeren Abschnitte und die umbenannte Überschrift den Platzhalter, die
Gestalt des Prüfordners ihren Wert. `(?s)` ist gefallen; es trug an dieser Stelle nichts.

**Ein Rest bleibt und ist der Preis der Abgrenzung ohne Vorausschau:** die Kiste `regex` kennt
kein `(?!…)`, also trennt das erste Zeichen der Fanggruppe (`[^#\n]`) die Überschrift vom Wert.
Eine Zeile unter `## Current`, die selbst mit `#` beginnt, wird damit nicht gelesen. Die
Kommentarzeile `:184-186` sagt es dem Nutzer.

`make check` grün, Exit 0.

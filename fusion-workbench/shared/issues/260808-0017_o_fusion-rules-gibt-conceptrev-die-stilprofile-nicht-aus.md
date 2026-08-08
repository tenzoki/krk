fusion-rules gibt conceptrev die Stilprofile nicht aus

---

`"$FUSION_PLUGIN_ROOT/bin/fusion-rules" conceptrev` gibt die beiden Stilprofile
des Projekts **nicht** aus, obwohl beide vorhanden sind:

- `fusion-workbench/stilwerk/chat-voice-de.yaml`
- `fusion-workbench/stilwerk/default-voice-de.yaml`

Zum Vergleich: `fusion-rules orchestrator` gibt am 260807-1934 beide Pfade aus,
an den Positionen 8 und 9 seiner Ausgabe. Das Projekt deklariert
`**Language:** de` in `CLAUDE.md`, die Auflösung greift also.

---

## Wie es aufgefallen ist

Der `conceptrev` hat es am 260807-2202 selbst gemeldet, am Ende seines Berichts
zur Diagrammprüfung des Editor-Spec: er habe die Profile nicht über
`fusion-rules` bekommen, sie stattdessen direkt gelesen und angewandt.

## Warum das mehr ist als ein fehlender Pfad

`rules/agent-setup.md` Abschnitt `## Voice profiles` sagt: „If `fusion-rules`
emitted a `chat-voice-*.yaml` path (**it does for every agent**), read it and
apply it." Für `conceptrev` trifft die Klammer nicht zu.

Der Agent, der die Lücke nicht bemerkt, schreibt seinen Bericht ohne das
Chat-Profil. Er merkt es nicht, denn die Setup-Anweisung sagt ihm, er solle
lesen, was ausgegeben wird, und für ihn wird nichts ausgegeben. Dass dieser Lauf
es gemerkt hat, ist kein Verlass für den nächsten.

`rules/fusion-workbench-conventions.md` Abschnitt `## Project language` legt
fest, dass `bin/fusion-rules` den Chat-Profil-Pfad **für jeden Agenten** ausgibt
und den Schreibprofil-Pfad nur für Langform-Agenten. Ob `conceptrev` als
Langform-Agent gilt, ist eine eigene Frage; dass er das Chat-Profil bekommen
muss, ist es nicht.

## Zuständigkeit

**Das ist ein Defekt im fusion-Plugin, nicht in KRK.** Er liegt in der
Musterzuordnung von `bin/fusion-rules` oder in
`rules/context-manifest.yaml`. Er gehört hierher, weil `$OUT_ISSUE` ihn hierher
auflöst und er hier aufgefallen ist; behoben wird er im Plugin.

`inference:` Ungeprüft ist, ob weitere Agenten dieselbe Lücke haben. Wer den
Defekt anfasst, sollte `fusion-rules` für alle sechzehn Agentennamen aufrufen
und die Ausgaben auf `chat-voice-` durchsehen, statt nur `conceptrev` zu
reparieren.

**Aufgefallen bei:** der Diagrammprüfung des Editor-Spec am 260807-2202,
gemeldet vom `conceptrev` selbst.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/reviews/260807-2202-conceptrev-spec-eingebauter-editor-mit-textmarken.md`

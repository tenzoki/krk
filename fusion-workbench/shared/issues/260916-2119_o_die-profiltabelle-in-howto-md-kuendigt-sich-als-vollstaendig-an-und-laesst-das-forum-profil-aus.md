Die Profiltabelle in HowTo.md kündigt sich als vollständig an und lässt das Forum-Profil aus

---

Die Tabelle unter `## Die mitgelieferten Leseprofile` in `HowTo.md` wird mit dem Satz „Für eine fusion-Werkbank sind es diese" eingeleitet und führt acht Zeilen. `resources/default-readers.toml` führt neun Profile für eine fusion-Werkbank. Die Einleitung sagt also „diese" und meint nicht alle.

---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

**Es fehlt `fusion-Werkbank: der Forum-Speicher`.** Das Profil greift auf `pfad = 'fusion-workbench/(shared|circles/[^/]+)/forum$'` und führt die drei Zeilen `Nachrichten`, `Zuletzt angekommen` und `Die jüngsten zehn`. In der Tabelle steht es nicht.

**Das ist eine Lücke und keine falsche Zeile**, und deshalb ein eigener Datensatz und keine Nacharbeit an `260916-2021_*_zwei-zeilen-der-profiltabelle-in-howto-md-versprechen-angaben-die-kein-profil-mehr-fuehrt.md`: dort war jede genannte Zeile falsch, hier ist jede genannte Zeile richtig und eine ungenannt.

**Zu entscheiden ist mit der Behebung eines mit:** ob die Tabelle künftig vollständig ist und dann bei jedem neuen Profil nachgezogen werden muss, oder ob die Einleitung sagt, dass sie eine Auswahl zeigt, und auf `grep -c '^\[\[profil\]\]' resources/default-readers.toml` verweist, wie sie es für die Gesamtzahl schon tut. Die Anleitung reist im Releasepaket mit; eine Tabelle, die Vollständigkeit behauptet und sie nicht hält, ist die schlechtere der beiden Lagen.

**Wie sich der Befund erheben lässt.** Die Profilnamen zählt
`grep -n '^name = "fusion-Werkbank' resources/default-readers.toml`,
die Tabellenzeilen die Tabelle unter `## Die mitgelieferten Leseprofile`.

**Abnahme.** Entweder nennt die Tabelle jedes Profil, das die Profildatei für eine fusion-Werkbank führt, oder ihre Einleitung sagt ausdrücklich, dass sie eine Auswahl zeigt. Der `flight`-Teil der Datei wird dabei mitbetrachtet: er führt vier weitere Profile, die die Tabelle ebenfalls nicht nennt und nach ihrer eigenen Ankündigung auch nicht nennen muss.

**Gefunden am 260916** bei der Durchsicht der übrigen Tabellenzeilen im Zuge von `260916-2021`.

**Cross-references:** 260916-2021_*_zwei-zeilen-der-profiltabelle-in-howto-md-versprechen-angaben-die-kein-profil-mehr-fuehrt.md

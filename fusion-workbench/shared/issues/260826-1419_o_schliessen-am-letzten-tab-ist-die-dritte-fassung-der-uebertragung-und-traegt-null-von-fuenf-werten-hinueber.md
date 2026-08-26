`Tabliste::schliessen` am letzten Tab ist die dritte Fassung der Übertragung und trägt null von fünf Werten hinüber

---

Drei Stellen in `crates/krk-ui/src/tabs.rs` setzen einen frischen `Tabinhalt` an die Stelle eines
alten, und jede rettet vorher eine andere Teilmenge aus dessen `Ordnermodell`:

| Stelle | Sortierung | Verstecke | Deep | Content | Filtertext |
|---|---|---|---|---|---|
| `ordner_setzen` (`tabs.rs:653-681`) | ja | ja | ja | ja | ja |
| `verdeckten_tab_setzen` (`:485-495`) | ja | ja | nein | nein | nein |
| `schliessen`, letzter Tab (`:560-571`) | **nein** | **nein** | nein | nein | nein |

Die dritte Zeile ist neu gegenüber
`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/issues/260815-0020_*_verdeckten-tab-setzen-baut-denselben-frischen-tabinhalt-…md`,
das die ersten beiden führt (dort noch „zwei von vier"; seit dem Inhaltsfilter sind es zwei von
fünf). Der Doc-Kommentar von `ordner_setzen` (`:613-625`) nennt das Schliessen des letzten Tabs
als einen der Wege, auf denen der **Filtertext** fällt. Dass dort auch die Sortierung und der
Schalter für versteckte Einträge auf den Auslieferungswert zurückspringen, sagt weder er noch der
Doc-Kommentar von `schliessen` („zeigt den Standardordner"). Wer nach Grösse absteigend sortiert
und `cmd+w` auf dem letzten Tab drückt, bekommt den Benutzerordner nach Name aufsteigend, ohne
Meldung.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/tabs.rs` (`schliessen`, `verdeckten_tab_setzen`, `ordner_setzen`)
**Baumstand:** `ca8072d`

## Warum nicht nur ein Nachtrag am alten Datensatz

Der alte Datensatz stellt eine Entwurfsfrage über zwei Fassungen; die dritte macht die Antwort
dringlicher, denn sie ist die Fassung, die am wenigsten trägt und am wenigsten dokumentiert ist.
Der Nutzer hat am 260815-1055 für den Filtertext entschieden, dass die Aufzählung der Löschwege
offen ist und das Verhalten bleibt (`shared/issues/260815-1047_*`). Für Sortierung und Verstecke
gibt es keinen solchen Entscheid; `ordner_setzen` und `verdeckten_tab_setzen` tragen beide, und
C1 der Runde 1 sagt zum letzten Tab nur „zeigt den Standardordner".

## Zwei Wege

1. **Eine Übertragungsfunktion** `Tabinhalt::nachfolger(&self, ordner, auswahl) -> Tabinhalt`,
   die alle fünf Werte trägt, und alle drei Stellen rufen sie; wer eine Fassung anders will,
   nimmt danach ausdrücklich etwas weg. Das ist der erste Ausweg des alten Datensatzes, auf drei
   Rufer erweitert.
2. **Den Zustand festschreiben:** der Doc-Kommentar von `schliessen` nennt, was fällt, und eine
   Probe hält es.

Die Zählprobe, die den Unterschied fängt, fehlt heute in allen drei Fällen.

Ein Commit des Orchestrators nimmt die `git mv`-Umbenennungen eines laufenden Agenten mit

---

Am 260824-2100 hat der Commit `79209c8` („docs(core): der Modulkopf zieht auf die Fassung des
Wertes nach") **fünf Umbenennungen mitgenommen, die ihm nicht gehören**. Seine Staging-Liste
nannte drei Pfade; im Commit stehen acht Dateien.

Die fünf sind Defektdatensätze, die ein zu diesem Zeitpunkt **noch laufender** `ontocoder`
mit `git mv` von `_o_` auf `_c_` gebracht hatte:

    260824-1651_c_der-kopf-des-speicherprofils-nennt-achtzehn-orte-…
    260824-1652_c_ein-abschliessender-schraegstrich-in-ordner-…
    260824-1653_c_zwei-bausteinbeschreibungen-sagen-weniger-…
    260824-1654_c_die-verlaufszeile-des-rundenprofils-traegt-kein-muster-…
    260824-1656_c_der-kopf-der-auslieferungsfassung-braucht-das-wort-ablage-…

---

## Die Mechanik

`git mv` ist kein reiner Dateisystemvorgang: es **staged** die Umbenennung im Index. Der Index
ist für das ganze Arbeitsverzeichnis einer, also teilen ihn Orchestrator und jeder Sub-Agent.
`git commit` ohne Pfadangabe committet den **ganzen** Index und nicht die Pfade, die das
vorangegangene `git add` genannt hat.

Damit greift die Staging-Regel des Orchestrators hier nicht. Sie verhindert, dass er **zu viel
auswählt** — jeder Pfad ausgeschrieben, kein `-A`, kein Verzeichnis, kein Glob. Sie kann nicht
verhindern, dass ein anderer etwas in den Index legt, das der Commit dann einsammelt.

Die Commit-Sperre (`bin/fusion-commit-lock`) greift ebenfalls nicht. Sie serialisiert
**Committer** gegeneinander; der `ontocoder` committet nicht und nimmt sie deshalb nie. Sein
`git mv` läuft außerhalb jeder Sperre.

## Warum es zählt, und warum wenig

**Wenig:** nichts ist verloren, nichts überschrieben, kein Inhalt falsch. Die fünf Datensätze
tragen ihren richtigen Namen, und ihre `Resolved:`-Zeilen — nach dem `git mv` geschrieben und
deshalb noch unversioniert — kommen mit dem nächsten Commit nach.

**Trotzdem zählt es:** ein Datensatz ist über zwei Commits gespalten, deren erster in seiner
Nachricht kein Wort von ihm sagt. Wer die Geschichte eines Befunds liest, findet seine
Schließung in einem Commit über einen Modulkommentar. Und die Aussage „diese Staging-Liste
sagt, was im Commit steht" ist für `79209c8` falsch, während der ganze Mechanismus darauf
gebaut ist, dass sie stimmt.

Bei einem gleichzeitig laufenden **Coder** wäre der Schaden größer: dessen halbfertige
Quelldatei stünde im Index, sobald er sie mit `git mv` verschiebt.

## Was der Orchestrator falsch gemacht hat

Er hat einen Agenten committet, während ein anderer Datensätze umbenennt. Die zwei Aufträge
waren nach **Dateien** sauber getrennt — `crates/` gegen `resources/` —, und das genügt nicht:
getrennt sein müssen sie auch im **Index**.

## Möglichkeiten

1. **Nicht committen, solange ein Agent läuft.** Einfach und teuer: der Orchestrator verliert
   die Möglichkeit, fertige Arbeit einzuchecken, während der Nachbar noch rechnet.
2. **`git commit -o <pfad>…`** — die Pfadform von `git commit` committet nur die genannten
   Pfade und lässt den übrigen Index stehen. Die Staging-Liste wäre dann wirklich der
   Commitinhalt. Zu prüfen: wie sie sich mit einer Umbenennung verträgt, die zwei Pfade hat.
3. **Sub-Agenten `git mv` verbieten** und stattdessen `mv` nehmen lassen; der Orchestrator
   staged die Umbenennung beim Commit. Kostet die Rename-Erkennung nichts, weil er beide Namen
   ohnehin aufschreibt.
4. **Die Commit-Sperre auf `git mv` ausweiten**, also jeden Indexschreiber sie nehmen lassen.
   Am gründlichsten und am teuersten.

**Domain:** code
**Gefunden:** Orchestrator, an der Staging-Prüfung nach `79209c8`

---
Resolved:

---
**Nachgeprüft beim Abgleich zum Abschluss der Runde 16, 260824-1852: die Beschreibung stimmt
Stelle für Stelle.** `git show --name-status 79209c8` führt acht Dateien: die geänderte
`crates/krk-core/src/leseprofil/bausteine.rs`, die neue Verlaufsdatei und sechs Umbenennungen.
Fünf der sechs sind die hier aufgezählten Defektdatensätze; die sechste,
`260824-1722_*_der-modulkopf-der-bausteine-…`, gehört dem Commit und ist in seiner Zeile
`Source:` genannt. Der Datensatz bleibt offen: keine der vier Möglichkeiten ist gewählt.

**Der Abgleich hat aus dem Befund eine Vorkehrung gezogen** und die zehn Markerwechsel dieser
Sitzung mit `mv` statt `git mv` gefahren, damit der Index unberührt bleibt. Das ist Möglichkeit 3
in der Praxis eines einzelnen Laufs und keine Entscheidung über die Regel.

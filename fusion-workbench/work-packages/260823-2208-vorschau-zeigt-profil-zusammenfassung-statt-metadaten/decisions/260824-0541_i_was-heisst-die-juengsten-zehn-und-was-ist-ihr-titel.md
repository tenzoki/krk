# Was heißt „die jüngsten zehn", und was ist ihr Titel?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`, `crates/krk-core/src/verzeichnis/durchlauf.rs`, `crates/krk-bench/src/messen.rs` (Zusage L7)

---

## Question

Der Baustein „jüngste N mit Titel" kommt in vier der sechs skizzierten Zusammenfassungen vor, und zwei seiner Wörter sind unbestimmt. „Jüngste" kann die Reihenfolge der Dateinamen meinen oder das Änderungsdatum; „Titel" kann der Dateiname sein oder die erste Überschriftenzeile im Inhalt. Die vier Verbindungen aus beidem liefern sichtbar verschiedene Anzeigen, und sie kosten verschieden viel: der Dateiname steht nach dem Lesen des Ordners bereits da, die Überschriftenzeile verlangt, zehn Dateien zu öffnen und anzulesen. Die Antwort gehört vor die Abnahmekriterien, weil sie den Wortlaut jedes Kriteriums über diesen Baustein bestimmt, und sie berührt die Zeitzusage L7, in deren Endbedingung die ganze Zusammenfassung fällt.

## Options

1. **Nach Dateiname absteigend, Titel ist der Dateiname** — Sortiert wird über den Namen, angezeigt wird der Name ohne Endung.
   - Pros: Keine Datei wird geöffnet. Für die fusion-workbench ist die Reihenfolge trotzdem zeitlich, weil jeder Datensatzname mit `YYMMDD-HHMM` beginnt. Der Zustandsmarker im Namen bleibt sichtbar, also sieht der Nutzer einem Eintrag an, ob er offen oder geschlossen ist.
   - Cons: In einem Ordner ohne diese Namenskonvention ist „jüngste" alphabetisch und damit falsch. Die Anzeige trägt Zeitstempel, Marker und Bindestriche statt lesbarer Sätze.
2. **Nach Änderungsdatum, Titel ist die erste Überschriftenzeile** — Sortiert wird über den Zeitstempel des Dateisystems, angezeigt wird die erste Überschrift der Datei, und wo es keine gibt, der Dateiname.
   - Pros: Liest sich als Satzliste und nicht als Dateiliste, in jedem Ordner und ohne Namenskonvention. „Jüngste" stimmt auch dort, wo die Namen nichts über die Zeit sagen.
   - Cons: Zehn Dateien werden je Zusammenfassung geöffnet und angelesen, also arbeitet die Anzeige innerhalb der Zusage L7 mehr als in Möglichkeit 1. Ein nachträglich bearbeiteter alter Datensatz rutscht an die Spitze, und der Zustandsmarker verschwindet aus der Anzeige.
3. **Beides je Profil wählbar** — Die Profilzeile nennt, wonach sortiert und was angezeigt wird.
   - Pros: Der Nutzer entscheidet je Ort, und beide Anzeigen sind erreichbar.
   - Cons: Der Baustein trägt zwei Schalter statt keiner, die Definitionsdatei wird länger, und die Runde baut beide Wege statt eines. Ein fester Bausteinsatz mit Schaltern ist ein Schritt zurück in Richtung der Ausdruckssprache, die der Nutzer abgelehnt hat.

## Constraints

Die Zusammenfassung fällt in die Endbedingung der Zeitzusage L7, die 100 ms zusagt; L7 steht seit dem 260819-2242 ohnehin auf den Gegenständen der späteren Messrunde. Gelesen wird über die vorhandene Maschinerie, also `krk-core/src/verzeichnis/durchlauf.rs` für den Ordner und `text::datei::bis_zur_grenze_lesen` für den Inhalt; ein zweiter Leseweg daneben entsteht nicht. Der Beispielfall der Runde ist die fusion-workbench, deren Dateinamen sämtlich mit `YYMMDD-HHMM` beginnen.

## Recommendation

Möglichkeit 1. Der Beispielfall trägt seine Zeit im Namen, also liefert die billigste Fassung dort dasselbe Ergebnis wie die teure, und sie zeigt den Zustandsmarker mit, den Möglichkeit 2 unterschlägt. Der Preis ist benannt: in einem Ordner ohne diese Namenskonvention ist die Reihenfolge alphabetisch, und wer das ändern will, stellt die Frage als eigene Runde.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:74 — Nach Änderungsdatum, Titel ist die erste Überschriftenzeile (Möglichkeit 2).
Answered (Nachtrag 260824-0609): circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:88 — die Titelhälfte dieser Antwort ist berichtigt; maßgeblich ist circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0600_a_der-titel-aus-der-ueberschriftenzeile-erreicht-keinen-einzigen-defektdatensatz.md. Die Sortierung nach Änderungsdatum bleibt wie hier entschieden.
Implemented: 260824-1849, Commit `abe1a31`, Schritt 6 des Plans — die Sortierhälfte dieser Antwort. `crates/krk-core/src/leseprofil/bausteine.rs:363` sortiert absteigend nach `Eintrag::geaendert` und bei gleichem Zeitpunkt aufsteigend nach dem Namen, damit die Reihenfolge bestimmt ist. Die Titelhälfte ist am 260824-0609 berichtigt und in `260824-0600_*_der-titel-aus-der-ueberschriftenzeile-…` umgesetzt. Belegt durch `crates/krk-core/tests/leseprofil.rs::die_juengsten_stehen_nach_aenderungsdatum_und_tragen_ihre_titel`.

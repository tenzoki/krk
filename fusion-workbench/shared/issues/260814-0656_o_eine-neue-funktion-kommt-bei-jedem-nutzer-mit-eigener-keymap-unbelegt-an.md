Eine neue Funktion kommt bei jedem Nutzer mit eigener `keymap.toml` unbelegt an

---

Wer seit der Runde 7 einmal eine Taste in der Belegungsansicht zugewiesen hat, hat eine `keymap.toml` unter `~/Library/Application Support/KRK/`. Für ihn kommt jede Funktion, die eine spätere Runde neu ausliefert, **ohne ihre Kombinationen** an: `Belegung::bauen` (`crates/krk-core/src/tasten/belegung.rs:1252-1267`) fügt eine Funktion, die die Nutzerdatei nicht nennt, mit `tasten: Vec::new()` hinzu. Die ausgelieferte Belegung aus `resources/default-keymap.toml` ist dabei allein der Wortschatz und nicht die Quelle der Tasten.

Die Folge trifft die Abnahme jeder Runde, die eine Funktion hinzufügt. Für die neunte Runde heißt sie konkret: `f2` und `cmd+k` öffnen den Notizzettel auf dem Gerät des Nutzers nicht, obwohl der Code stimmt und alle Proben grün sind. Betroffen sind rückwirkend auch die Runden 7 und 8 mit ihren neuen Funktionen.

---

**Der Kommentar an der Stelle nennt den Grund, und der Grund ist gut:** „Funktionen, die die Nutzerdatei nicht nennt, treten unbelegt hinzu. Die Belegungsansicht führt damit weiter jede Funktion, und der Nutzer kann eine, die er gelöscht hat, wieder erreichbar machen." Das ist richtig für eine Funktion, die der Nutzer bewusst entbelegt hat.

**Der Mechanismus kann die zwei Fälle aber nicht trennen.** „Der Nutzer hat die Tasten dieser Funktion gelöscht" und „diese Funktion gab es noch nicht, als der Nutzer zuletzt gesichert hat" sehen in der Datei gleich aus: in beiden Fällen fehlt der Eintrag. Aus den Eingaben, die `bauen` hat, ist die Frage nicht zu entscheiden, und die heutige Antwort ist eine Näherung, die im zweiten Fall falsch liegt.

Entscheidbar wird sie mit anderen Eingaben. Drei Wege sind denkbar und keiner ist hier gewählt:

1. Der Nutzer entbelegt eine Funktion künftig **ausdrücklich**, also mit einem Eintrag `tasten = []` in seiner Datei. Ein fehlender Eintrag heißt danach „kenne ich nicht" und bekommt die Auslieferungstasten. Kostet eine Wandlung für bestehende Dateien: eine alte `keymap.toml` nennt entbelegte Funktionen nicht.
2. Die Datei trägt einen Stand des Wortschatzes, gegen den `bauen` vergleicht. Was seither dazugekommen ist, kommt mit seinen Tasten.
3. Es bleibt, wie es ist, und jede Runde nennt die Folge in ihrer Nutzerliste. Kostet nichts und lässt jede neue Funktion beim Nutzer erst einmal tot ankommen.

Ein Konflikt ist bei 1 und 2 möglich: die neue Auslieferungskombination kann beim Nutzer schon vergeben sein. `Belegung::bauen` meldet ihn heute über `Belegungsfehler::Konflikt`, und das führte zum Auslieferungszustand — für eine hinzugefügte Funktion die schlechteste aller Antworten. Wer den Weg 1 oder 2 geht, beantwortet die Konfliktfrage mit.

**Für die neunte Runde reicht ein Handgriff, und er steht im Plan:** vor dem Abnahmelauf die eigene `keymap.toml` zur Seite legen oder `f2` in der Belegungsansicht von Hand zuweisen (`circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, Abschnitt „Nutzerarbeit").

Gefunden beim Planen der neunten Runde, am Baum geprüft und nicht aus der Prosa übernommen. Der Defekt steht im gemeinsamen Speicher und nicht im Circle, weil er nicht aus dessen Directive entstanden ist: er betrifft jede Runde, die eine Funktion hinzufügt, und drei haben es schon getan.

**Filed by:** planner

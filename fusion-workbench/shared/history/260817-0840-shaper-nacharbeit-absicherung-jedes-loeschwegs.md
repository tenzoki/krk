# Nacharbeit am Spec „Absicherung jedes Löschwegs"

**Datum:** 2026-08-17
**Agent:** shaper (user-direct, Nacharbeit nach der Abnahme)
**Status:** Complete
**Gegenstand:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`

## Anlass

Der Nutzer hat den Spec am 260817 abgenommen und dabei die drei offenen Fragen beantwortet. Eine der drei Antworten widerspricht dem, was der Spec bis dahin schrieb. Diese Sitzung beantwortet die Datensätze und zieht den Spec nach; eine zweite Fassung entsteht nicht.

## Die drei Antworten

1. **Gespeicherte `keymap.toml` mit der entfallenen Kennung:** Möglichkeit 1, es bleibt wie heute. Die Datei wird als Ganzes verworfen, die Auslieferungsbelegung greift, die Statuszeile nennt sie. Der Nutzer nimmt den Verlust seiner eigenen Belegung in Kauf.
2. **`f8`:** Möglichkeit 1, die Empfehlung des Shapers. `f8` zeigt künftig auf „In den Papierkorb räumen"; die Norton-Reihe behält ihre Löschtaste. Zu `opt+cmd+delete` hat der Nutzer nichts gesagt; die Frage ist ohne ihn entscheidbar und im Datensatz begründet beantwortet: die Kombination bleibt unbelegt.
3. **Reichweite der `.git`-Prüfung:** Möglichkeit 2, die Prüfung sieht auch aufwärts. Der Nutzer hat damit seine Festlegung der zweiten Klärungsrunde umgedreht und die Empfehlung des Shapers ausdrücklich verworfen. Der Einwand steht als benannte Folge im Spec.

## Was geschrieben wurde

**Drei Datensätze beantwortet**, jeder mit einem Abschnitt `## Antwort des Nutzers`, einer Zeile `Answered:`, `**Status:** answered` im Kopf und der Umbenennung `_o_` → `_a_`:

- `shared/decisions/260817-0536_a_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md`
- `shared/decisions/260817-0536_a_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md`
- `shared/decisions/260817-0536_a_sieht-die-git-pruefung-nur-den-ordner-selbst-oder-auch-aufwaerts.md`

**Ein Nachtrag im tragenden Datensatz.** `260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-…` hielt in seinem Abschnitt `## Antwort des Nutzers` die enge Form der `.git`-Prüfung als Festlegung der zweiten Klärungsrunde. Der Satz bleibt als Aufzeichnung stehen; ein Nachtrag am Ende des Abschnitts nennt die Umkehrung und verweist auf den neuen Datensatz. Ohne ihn stünden zwei bindende Datensätze im Widerspruch.

**Der Spec ist an neunzehn Stellen nachgezogen.** Im Kopf eine neue Zeile `**Nachgezogen am 260817**`. Im Abschnitt `## Was der Nutzer entschieden hat`: die Einleitung, die Festlegung zum Wegfall des endgültigen Löschens ohne die beiden Kombinationen, die umgedrehte Git-Festlegung und zwei neue Absätze zu `f8` und zur `keymap.toml`. In C3: die fünfte Zeile der Auslösertabelle, das Abnahmekriterium zum Git-Auslöser, jetzt zwei statt eines, zwei neue Festlegungen zur Reichweite und zur Quelle des Aufwärtsgangs, und der Block `**Was die Reichweite der Git-Prüfung kostet**`. In C5: der Belegungspunkt zu `default-keymap.toml`, ein neues Kriterium zu den drei Kombinationen von `in_papierkorb` und das aufgeteilte Kriterium zu `f8` und `opt+cmd+delete`. Dazu der neu gerechnete Kalibrierungsabschnitt, der Kostensatz im Abschnitt zu den zehn Zeitzusagen, der Abgrenzungspunkt zum eigenen Quellbaum, ein neuer Punkt unter `## Offen für den Planner` zum Aufwärtsgang, der Punkt zu den Fragern nach dem Benutzerverzeichnis, jetzt vier statt drei, und der Abschnitt `## Ausstehende Nutzerentscheidungen`, der danach leer ist und die drei Fragen als beantwortet mit Verweis führt.

## Die neu gerechnete Kalibrierung

Die frühere Fassung sagte, beim Vorfall vom 260817-0344 hätte keiner der fünf Zieltests angeschlagen. Mit der aufwärts sehenden Prüfung schlägt einer an: `/Users/k1/Projects/productive/krk/fusion-workbench/shared` liegt im Arbeitsbaum unter `/Users/k1/Projects/productive/krk`, nachgeprüft mit `git rev-parse --show-toplevel`. Vier Zieltests schweigen weiterhin. Getroffen hätten den Fall damit drei Prüfungen dieser Runde: der Git-Auslöser, die Umfangsschwelle und die unbedingte Rückfrage.

Der Schluss der ersten Fassung bleibt stehen, und die Prüfung hat ihn geschärft. Verhindert hätte den Vorfall, dass überhaupt ein Blatt erscheint und „Abbrechen" darin vorbelegt ist. Die laute Form fügt ein Warnzeichen und einen Grund hinzu, und beides wirkt nur, solange es nicht der Normalfall ist; im Quellbaum, in dem der Vorfall geschah, ist es nach der neuen Reichweite der Normalfall. Der Zugewinn liegt deshalb nicht bei diesem Nutzer in diesem Baum, sondern bei einem Vorgang in einem fremden Projekt.

## Keine neue Frage

Bei Antwort 2 ist keine neue Nutzerfrage aufgegangen. `opt+cmd+delete` war aus dem Baum und aus der Aktenlage entscheidbar; die Begründung steht im Datensatz und nicht als offene Frage daneben.

## Nicht angefasst

Kein Circle angelegt, kein Circle-Marker umbenannt, der Spec nicht verschoben. Kein Code, keine Belegungsdatei, keine anderen Datensätze.

# Bekommt f8 den Papierkorb, nachdem das endgültige Löschen weggefallen ist?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (C1, C5, C6), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C3), `shared/decisions/260802-0842_i_f-tasten-unter-macos-systembelegung.md`

---

## Question

Mit dem Wegfall des endgültigen Löschens werden `f8` und `opt+cmd+delete` frei. Der Nutzer hat das am 260817 so gesagt, und für `opt+cmd+delete` ist die Sache damit klar: die Kombination trägt im Finder die Bedeutung „sofort löschen", und KRK hat diese Bedeutung nicht mehr.

Für `f8` steht eine Zusage der Runde 1 dagegen, die mit dem Wegfall bricht. C3 sagt zu, dass jede Funktion der Norton-Reihe auf zwei Wegen erreichbar ist, über die Funktionstaste und über ein Cmd-Kürzel, und die Norton-Zuordnung lautet dort: „F3 Vorschau anzeigen, F5 Kopieren, F6 Verschieben, F7 Ordner anlegen, F8 endgültig löschen". In Norton Commander und in Total Commander ist F8 die Löschtaste. Fällt sie ersatzlos, hat die Norton-Reihe in KRK keine Löschtaste mehr, und ein Nutzer aus dieser Tradition greift ins Leere.

Die Frage ist damit nicht, ob eine Kombination frei wird, sondern ob die Norton-Reihe ihre Löschtaste behält, indem `f8` auf den verbliebenen Löschweg zeigt.

## Options

1. **`f8` zeigt auf „In den Papierkorb räumen"** — die Funktion trägt danach drei Kombinationen: `delete`, `cmd+delete` und `f8`.
   - Pro: die Norton-Reihe behält ihre Löschtaste, und die Zusage aus C3 der Runde 1 bleibt in ihrer Substanz erhalten. F8 bedeutet in beiden Vorbildern „löschen", und KRK hat nur noch eine Art zu löschen.
   - Contra: `f8` hieße jetzt Papierkorb statt endgültig. Wer die alte Bedeutung im Finger hat, löst einen anderen Befehl aus als früher, und dieser Befehl ist der harmlosere, aber nicht derselbe. Der Übergang ist durch die neue Rückfrage abgefedert.

2. **`f8` bleibt frei** — die wörtliche Lesart der Nutzeraussage vom 260817.
   - Pro: keine Umdeutung einer eingeübten Taste. Die Kombination steht für eine spätere Runde bereit.
   - Contra: die Norton-Reihe verliert ihre Löschtaste, und das Abnahmekriterium aus C3 der Runde 1 wird an dieser Stelle nicht nachgezogen, sondern aufgegeben. Das gehört dann ausdrücklich in den Spec und nicht als stille Folge in einen Diff.

3. **`f8` bleibt frei, und C3 wird ausdrücklich eingeschränkt** — die Norton-Reihe umfasst danach fünf statt sechs Funktionen.
   - Pro: ehrlich in der Buchführung, ohne eine Taste umzudeuten.
   - Contra: dieselbe Lücke wie Möglichkeit 2, nur mit Papier davor. Der Nutzer, der F8 drückt, merkt vom Spec nichts.

## Constraints

- `f8` und ein nacktes F8 sind für KRK nicht unterscheidbar; KRK belegt den Tastencode und nicht die Fingerhaltung.
- `opt+cmd+delete` bleibt nach der Antwort des Nutzers frei und ist von dieser Frage nicht berührt.
- Die ausgelieferte Belegung steht allein in `resources/default-keymap.toml`; Menü, Belegungsansicht und Markdown-Ausgabe folgen ihr.
- Jede Belegung ist frei konfigurierbar; die Frage betrifft die Auslieferung.

## Recommendation

Möglichkeit 1. Die Norton-Reihe ist eine ausdrückliche Zusage der Runde 1, F8 bedeutet in ihren beiden Vorbildern „löschen", und KRK hat nach dieser Runde genau eine Art zu löschen. Die Umdeutung wird dadurch gemildert, dass der neue Befehl vor dem Räumen fragt: wer F8 aus alter Gewohnheit drückt, sieht eine Rückfrage und nicht einen ausgeführten Löschvorgang. Die Abwägung ist eine Empfehlung und keine geprüfte Aussage.

## Antwort des Nutzers

**Am 260817, bei der Abnahme des Specs: Möglichkeit 1.** `f8` zeigt künftig auf „In den Papierkorb räumen". Die Funktion trägt danach drei Kombinationen, `delete`, `cmd+delete` und `f8`. Die Norton-Reihe behält ihre Löschtaste, und die Zwei-Wege-Zusage aus C3 der Runde 1 bleibt an dieser Stelle gewahrt.

**Zu `opt+cmd+delete` hat der Nutzer nichts gesagt, und die Frage bleibt ohne ihn entscheidbar.** Die Kombination bleibt unbelegt. Zwei Gründe tragen das, und beide standen schon vor dieser Antwort fest. Der Nutzer hat am 260817 gesagt, dass mit dem Wegfall des endgültigen Löschens beide Kombinationen frei werden; die Nachfrage betraf allein `f8`, und eine Antwort auf sie hebt die Aussage über die zweite Kombination nicht auf. Dazu trägt `opt+cmd+delete` im Finder die Bedeutung „sofort löschen", und diese Bedeutung hat KRK nach dieser Runde nicht mehr. Eine dritte Kombination auf denselben Papierkorbbefehl zu legen wäre eine Zutat ohne Anlass. Die Kombination steht damit einer späteren Runde zur Verfügung.

---
Answered: `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, Abschnitt `## Was der Nutzer entschieden hat` und C5 — Möglichkeit 1: `f8` zeigt auf „In den Papierkorb räumen", `opt+cmd+delete` bleibt unbelegt.
Implemented: `82707ef` — Möglichkeit 1 steht in `resources/default-keymap.toml`: `in_papierkorb` trägt dort `tasten = ["delete", "cmd+delete", "f8"]`, der Eintrag `endgueltig_loeschen` ist ganz gefallen, und `opt+cmd+delete` steht in keiner Tastenliste mehr. Der Kopf der Datei nennt 84 Funktionen mit 89 Kombinationen und als bindenden Datensatz nicht mehr den überholten Löschentscheid vom 260802, sondern `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`. Das ist Schritt 13 des Plans `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`; seine Voraussetzung, Schritt 12, trägt derselbe Commit, weil der Baum zwischen beiden in jeder Reihenfolge rot wäre. Bewegt in Schritt 16 desselben Plans.
Deferred:
Superseded by:

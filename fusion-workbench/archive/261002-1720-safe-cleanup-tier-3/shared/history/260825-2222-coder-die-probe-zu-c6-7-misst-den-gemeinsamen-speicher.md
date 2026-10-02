# Die Probe zu C6.7 misst den gemeinsamen Speicher

**Agent:** coder
**Datum:** 2026-08-25, 22:05 bis 22:22
**Aufgabe:** R-4, Runde 2 der Sitzung zur Runde 18; Befund M3 der Durchsicht
`fusion-workbench/shared/reviews/260825-2127-coderev-runde-18-vorschau-vertieft-und-zwei-fehler.md`,
Datensatz `shared/issues/260825-2127_c_die-probe-zu-c6-7-misst-nicht-mehr-das-groesste-mitgelieferte-profil.md`
**Status:** Complete

## Was entstanden ist

Eine Datei geändert: `crates/krk-core/tests/leseprofil.rs`.

1. **Die C6.7-Probe hat einen dritten Fall.** `die_zwei_groessten_…` heißt jetzt
   `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` (`:2967`). Nach Runde
   und Wurzel misst sie das Profil `fusion-Werkbank: der gemeinsame Speicher` über
   `zusammenfassen_gezaehlt` gegen die eingebettete Auslieferungsfassung und hält die Zahlen der
   Kostenmessung `shared/analyses/260825-2107-…` auf die Zahl genau: **10 Leseläufe, 0
   Öffnungen, Abstand 2 zu `HOECHSTENS_LESELAEUFE`.** Ausweis, dass das richtige Profil gegriffen
   hat, ist wie bei den zwei Fällen davor die Beschriftungsliste, hier gegen die Zeilen des
   Profils gehalten statt gegen eine zweite Aufzählung in der Probe. Dazu: keine Zeile liefert
   den Platzhalter, jede Zählung sieht den einen Datensatz.
2. **Die Zahl der Unterordner kommt aus der Profildatei.** `genannte_orte` (`:2888`) sammelt die
   verschiedenen Orte aller Zeilen des Profils; `gemeinsamer_speicher` (`:2918`) legt genau
   diese als Unterordner unter `fusion-workbench/shared` an, weil das Pfadmuster des Profils
   darauf trifft. Die Probe hält daneben `orte.len() == 10` und `Leseläufe == orte.len()`:
   die Regel „ein Ort, ein Leselauf" steht ausgeschrieben, und ein Ort mit Platzhalter hielte
   die Probe an, weil die Regel für ihn nicht gilt.
3. **Die Gegenprobe ist eine eigene, dauerhaft grüne Probe**:
   `ein_elfter_unterspeicher_kostet_einen_elften_leselauf` (`:3140`). Sie fügt einer Kopie von
   `AUSLIEFERUNGSTEXT` hinter dem Pfadmuster des Speicherprofils eine elfte Zeile auf den Ort
   `elfter` ein, prüft, dass der Anker genau einmal vorkommt, und misst 11 Läufe und einen Lauf
   Abstand. Damit ist belegt, dass die Messung einen elften Ort sieht; an der wirklichen Datei
   würden `orte.len() == 10` und `(10, 0)` der Hauptprobe rot. `resources/default-readers.toml`
   ist nicht angefasst — ein Ontocoder arbeitet darin; gelesen habe ich den festen Stand über
   `git show HEAD:resources/default-readers.toml`.
4. Überschrift und Kopf der Probe sagen jetzt, dass „das größte Profil" an der Frage hängt:
   nach Öffnungen die Runde (11), nach Leseläufen der Speicher (10 von 12), und dass die Zahlen
   aus der Kostenmessung vom 260825-2107 stammen. Die Tabelle im Kopf hat eine dritte Zeile.

## Zwei Stolpersteine beim Bauen

- `Zusammenfassung::name()` ist der Name des **Ordners** („shared"), nicht des Profils. Die
  erste Fassung hielt ihn gegen den Profilnamen und war rot; die Beschriftungsliste ist der
  Ausweis, so wie es die zwei Fälle davor auch halten.
- Die eingefügte elfte Zeile steht **vor** den zwanzig bestehenden, nicht dahinter, weil sie
  unmittelbar hinter `pfad = …` eingesetzt wird; die Gegenprobe prüft deshalb die erste
  Beschriftung und nicht die letzte.

## Was ausdrücklich nicht angefasst wurde

`resources/default-readers.toml`, `crates/krk-core/src/`, `crates/krk-core/tests/operation.rs`,
`crates/krk-ui/`, `Cargo.toml`. Die Verweise auf den alten Probennamen in Verläufen, Defekten
und dem Plan der Runde 16 bleiben als Aufzeichnung ihres Standes stehen.

## Abnahme

`Verification: make check — exit 0` (mit `PATH="$HOME/.cargo/bin:$PATH"`; `cargo fmt -p
krk-core` vor dem Lauf, `cargo fmt --all --check` schreibt nichts).

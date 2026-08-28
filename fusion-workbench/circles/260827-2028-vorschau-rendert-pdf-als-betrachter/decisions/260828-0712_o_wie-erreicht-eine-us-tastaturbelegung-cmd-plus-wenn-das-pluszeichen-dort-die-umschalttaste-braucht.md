# Wie erreicht eine US-Tastaturbelegung `cmd+plus`, wenn das Pluszeichen dort die Umschalttaste braucht?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/planning/260828-0649_*_spec-vorschau-rendert-pdf-als-betrachter.md` (C3.1, C3.2, der Absatz „`+` und `-` stehen nicht im Tastenalphabet"); `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/planning/260828-0712_*_plan-vorschau-rendert-pdf-als-betrachter.md` (Entscheidung 3, `**Decidability:**`); `crates/krk-ui/src/appkit/ereignisse.rs:742-745` (`gemeldetes_zeichen`); `crates/krk-core/src/tasten/parser.rs:154-162` (`Tastenkennung`); `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md` (die Regel, die hier an ihre Grenze kommt)

---

## Question

Der Ereignisabgriff liest zu jedem Tastendruck das Zeichen, das die Taste **ohne Zusatztasten** meldet (`charactersByApplyingModifiers:` mit leerer Maske, `ereignisse.rs:743`), und schlägt Buchstaben und Ziffern über dieses Zeichen nach. Der Plan der Runde 20 nimmt `plus` und `minus` in dieselbe Sorte auf. Auf der deutschen Belegung des Referenzgeräts meldet die Taste rechts neben `ü` ohne Zusatztaste ein `+`, und `cmd+plus` trifft. Auf der US-Belegung trägt keine Taste ohne Zusatztaste ein `+`: das Zeichen liegt auf `shift+=`, die Taste meldet ohne Zusatztasten ein `=`, und `=` steht nicht im Alphabet. Der Abgriff sieht dort `shift+cmd+=` als unbelegt und reicht ihn an AppKit weiter.

C3.2 des Specs verlangt, dass auf einer deutschen **und** einer US-amerikanischen Belegung jeweils die Taste mit der Beschriftung `+` wirkt. Für `-` gilt das auf beiden Belegungen (beide tragen es ohne Umschalttaste), für `+` allein auf der deutschen. Die Frage ist aus den Eingaben, die der Abgriff heute liest, nicht entscheidbar: „welche Taste trägt die Beschriftung `+`" ist auf der US-Belegung eine Frage nach dem Zeichen **mit** Umschalttaste, und genau die liest er nicht.

## Options

1. **Die Zusage bleibt beim Zeichen ohne Zusatztaste, und C3.2 wird auf diese Regel verengt.** `cmd+plus` heißt: die Taste, die ohne Zusatztaste ein `+` erzeugt. Auf der deutschen Belegung ist das die beschriftete Taste, auf der US-Belegung der Zehnerblock, sofern die Tastatur einen hat.
   - Folge: keine Änderung am Abgriff; die Regel aus dem Datensatz `260808-0140` bleibt eine Regel. Der Plan baut genau das.
   - Preis: ein Nutzer mit US-Belegung ohne Zehnerblock erreicht das Vergrößern über die Tastatur nicht, nur über den Menüeintrag und die Trackpad-Geste (Festlegung A4). Ob AppKit den Menüeintrag mit Kürzel `+` bei `shift+cmd+=` selbst auslöst, ist nicht gemessen (`inference:` AppKit vergleicht Menükürzel gegen `charactersIgnoringModifiers`, das bei gehaltener Umschalttaste `+` meldet; am Bündel nachzusehen).

2. **Der Abgriff liest ein zweites Zeichen, sobald das erste nicht trifft.** Findet der Nachschlag das Zeichen ohne Zusatztaste nicht im Alphabet und ist die Umschalttaste gehalten, fragt er `charactersByApplyingModifiers:` ein zweites Mal mit der Umschaltmaske und schlägt mit dem Ergebnis und einer Maske **ohne** das Umschaltbit nach. `shift+cmd+=` wird auf der US-Belegung zu `cmd+plus`; `shift+cmd+1` bleibt `shift+cmd+1`, weil die `1` im Alphabet steht und der zweite Weg gar nicht erreicht wird.
   - Folge: C3.2 hält auf beiden Belegungen wie geschrieben.
   - Preis: ein zweiter Fremdaufruf auf dem Tastendruckpfad, an dem L1 hängt, wenn auch nur im Fehlschlagfall des ersten; und eine Regel, die die Umschalttaste je nach Belegung einmal als Zusatztaste und einmal als Teil des Zeichens liest. `Kombination::aus_tastendruck` in der Belegungsansicht müsste dieselbe zweite Lesung tragen, sonst schriebe ein US-Nutzer beim Zuweisen `shift+cmd+=` in seine `keymap.toml` und fände es nie wieder.

3. **`equal` kommt als dritter Name ins Alphabet, und die Auslieferungsbelegung legt `cmd+plus` und `shift+cmd+equal` nebeneinander auf dieselbe Funktion.**
   - Folge: C3.2 hält auf der US-Belegung über eine zweite Kombination derselben Zeile.
   - Preis: `=` ist ein Satzzeichen, dessen Stelle je Belegung wandert (auf der deutschen liegt es auf `shift+0`); genau die Sorte, die `parser::TASTEN` ausdrücklich ausschließt. Ein deutscher Nutzer sähe in der Belegung eine Kombination, die auf seiner Tastatur `shift+cmd+0` heißt, und `make menue` zeigte zwei Kürzel für einen Eintrag.

## Constraints

- Die Konflikterkennung kennt keine Bereiche; jede zusätzliche Kombination muss in der ganzen Belegung frei sein (`belegung.rs`, „Der Zusteller, und was er für den Konflikt bedeutet").
- Der Tastendruckpfad trägt L1; ein zweiter Fremdaufruf darauf braucht eine Messung, bevor er als kostenlos gilt.
- Das Referenzgerät hat eine deutsche Belegung; keine US-Belegung ist in diesem Projekt je gemessen worden.

## Recommendation

Wir empfehlen Möglichkeit 1 und bauen sie im Plan, vorbehaltlich der Antwort: sie kostet keine Zeile am Abgriff, hält die Regel aus `260808-0140` unverändert und trifft das Referenzgerät vollständig. C3.2 wird dabei um die US-Hälfte kleiner, und der Spec sagt heute mehr zu, als der Mechanismus entscheiden kann; der Nutzer sollte das wissen, bevor der Abnahmelauf es zeigt. Fällt die Antwort auf Möglichkeit 2, wächst der Plan um einen Schritt am Abgriff und an `Kombination::aus_tastendruck`, und C3.2 bekommt eine Probe über beide Lesungen.

---
Abgleich 260828-1044: weiterhin offen. Gesucht in `planning/`, `analyses/` dieses Circles und in `shared/decisions/`; keine Antwort. Der Baum entscheidet die Taste über das gemeldete Zeichen ohne Zusatztaste (`crates/krk-core/src/tasten/parser.rs:211-222`, `zeichen_des_namens`); die US-Hälfte von C3.2 bleibt damit ungebaut, und der Plan nennt das keine Vorbedingung (`## Where this Circle stops`, Klausel 9).

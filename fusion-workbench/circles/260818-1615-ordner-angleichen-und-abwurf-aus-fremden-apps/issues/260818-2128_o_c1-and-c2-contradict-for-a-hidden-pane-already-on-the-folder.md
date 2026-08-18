C1 and C2 of the spec contradict each other for a hidden target pane already on the folder, and the built command leaves it hidden

---

Two acceptance criteria of
`shared/planning/260818-1510_o_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md`
cover the same lage and demand different outcomes:

- **C1, fifth criterion:** "Zeigt das andere Dateifenster in seinem sichtbaren Tab bereits
  denselben Ordner, geschieht nichts, und die Statuszeile sagt es."
- **C2, first criterion:** "Ist das andere Dateifenster ausgeblendet und ist das Fenster breit
  genug, blendet der Befehl es ein und stellt es auf den Ordner. Beides geschieht in einem Zug,
  ohne einen zweiten Tastendruck."

Neither carries an exception for the other. When the target pane is **hidden** and its visible
tab already holds the folder, both apply and they disagree. The spec's own flowchart under
"Der Weg des Tastenbefehls" puts the equality question ahead of the visibility question, so it
resolves the conflict in C1's favour without saying that it is resolving one.

`Anwendungsdelegierter::ordner_angleichen` (`crates/krk-ui/src/appkit/anwendung.rs:3325-3352`)
follows the flowchart: the equality branch returns before `Bereich::von_seite(ziel)` is even
computed. The doc comment names the consequence and calls it deliberate. **The built behaviour
is therefore: the pane stays hidden, and the status line of the triggering pane says "das andere
Dateifenster zeigt diesen Ordner bereits" — about a pane that is not on screen and, being
hidden, shows nothing.** C2's first criterion is literally unmet in that lage.

---

**Severity:** Medium. No data is at risk and no build stops. The user presses a key whose whole
purpose is to put the other pane in view, sees nothing change, and is told the pane already
shows what he cannot see. The acceptance run is user work, so this will surface there unless it
is settled first.
**Found by:** coderev, reading the two criteria against each other and against the built body.
**Affects:** `shared/planning/260818-1510_o_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md`
(C1 fifth criterion against C2 first criterion, and the flowchart under "Der Weg des
Tastenbefehls"); `crates/krk-ui/src/appkit/anwendung.rs:3325-3332`
**Tree state:** `71413c3`
**Domain:** code

## Why this is filed and not left to the acceptance run

The code is not wrong against the record it was given — it follows the flowchart, and the doc
comment says so in as many words. What is wrong is the record: it holds two criteria that cannot
both be met, and it hides the choice in the order of a diagram rather than stating it. A reader
checking C2 off at the acceptance run will find it unmet and have no way to tell whether that is
a defect or the intended reading.

## The two candidate resolutions, and what each costs

1. **Keep the order as built** and amend C2's first criterion to except the equal-folder case.
   Cost: the command has one lage in which it visibly does nothing, and the message names an
   invisible pane. Mitigable by wording the message for that case — "steht schon dort, bleibt
   aber ausgeblendet" is at least true.
2. **Move the visibility handling ahead of the equality check.** The hidden pane is revealed,
   then the equality check suppresses the redundant read, and both criteria hold as written.
   Cost: the equality branch stops being a pure no-op — it can now change the layout — so C1's
   "geschieht nichts" would need the same amendment in the other direction.

The choice is the user's. It is filed as a defect rather than a decision record because what is
wrong is a contradiction in the binding text, not an open design fork; either resolution is a
one-branch edit plus one criterion reworded.

**Filed by:** coderev

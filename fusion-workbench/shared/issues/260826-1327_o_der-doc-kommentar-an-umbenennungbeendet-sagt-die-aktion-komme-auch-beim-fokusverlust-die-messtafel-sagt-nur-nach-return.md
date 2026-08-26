Der Doc-Kommentar an umbenennungBeendet: sagt, die Aktion komme auch beim Fokusverlust; die Messtafel sagt: nur nach Return

---

Die Aktion `umbenennungBeendet:` des Delegierten traegt einen Doc-Kommentar aus der Zeit vor der Messung
vom 260816: "AppKit schickt sie, wenn die Bearbeitung mit Return endet oder die Zelle den Fokus
verliert". Die Messtafel am `Namensfeld` und der Kopf von `umbenennung_beenden` in derselben Datei sagen
das Gegenteil: die Aktion kommt allein nach Return, ein Fokusverlust schickt keine.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/tabelle.rs:3845-3852`: "AppKit schickt sie, wenn die Bearbeitung mit Return
  endet oder die Zelle den Fokus verliert, und ausdruecklich **nicht** nach Escape."
- `:2565-2575` (`umbenennung_beenden`): "**Gerufen aus der Aktion des Feldes, und die kommt allein von
  Return.** … Jedes uebrige Ende — der Klick daneben, der Fokuswechsel … — schickt **keine** Aktion".
- `:4338-4346`: die Messtafel; Fokusverlust → `textDidEndEditing:` ohne Aktion. `:4354-4355`: "Die Aktion
  kommt nur nach Return".

Genau diese Halbwahrheit hat `:2576-2579` schon einmal aus `umbenennung_beenden` entfernt ("die zweite
Haelfte davon war falsch"); die Kopie am Delegierten blieb stehen.

## Umfang

`krk-ui`, `appkit/tabelle.rs`.

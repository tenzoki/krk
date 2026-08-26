Das Kürzelzeichen der Schaltflächen der Belegungsansicht steht an der Aufrufstelle, und die Anzeige in der Tafel

---

`crates/krk-ui/src/appkit/belegungsansicht.rs:172-207` (`SCHALTFLAECHEN`) führt je Schaltfläche `titel`,
`mit_befehl` und `anzeige` („Cmd+T", „Cmd+R", „Cmd+Eingabe") und verspricht „Eine Quelle, zwei
Abnehmer … Ein Satz, der eine andere Taste nennt als die Schaltfläche trägt, kann so nicht
entstehen" (`:173-177`). Das Zeichen der Taste steht aber nicht in der Tafel: `taste_setzen`
(`:618-623`) bekommt es als drittes Argument, und die Rufer schreiben es von Hand,
`ns_string!("t")` (`:712`) und `ns_string!("r")` (`:728`). Wer „Zuweisen" auf Cmd+Z legt und
`anzeige` nicht nachzieht, bekommt einen Satz, der Cmd+T nennt, und eine Schaltfläche, die auf Z
hört; die Probe `die_erlaeuterung_nennt_die_drei_kuerzel_und_die_suche` (`:836-856`) prüft den
Satz gegen die Tafel und sieht die Aufrufstelle nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/belegungsansicht.rs`

Heute stimmen beide überein; der Befund ist die Lücke zwischen Zusage und Bauart. Weg: ein Feld
`zeichen: &'static str` in `Schaltflaechentaste`, `taste_setzen` liest es aus der Angabe, und die
bestehende Probe kann `anzeige` gegen `zeichen` halten (`"Cmd+" + zeichen.to_uppercase()`).
Daneben: `:142-143` sagt „den neun Bereichsueberschriften"; `Funktionsbereich` trägt heute neun
Werte, die Zahl stimmt, ist aber dieselbe Sorte Prosazahl, die `:148-151` in derselben Datei schon
einmal falsch geworden ist.

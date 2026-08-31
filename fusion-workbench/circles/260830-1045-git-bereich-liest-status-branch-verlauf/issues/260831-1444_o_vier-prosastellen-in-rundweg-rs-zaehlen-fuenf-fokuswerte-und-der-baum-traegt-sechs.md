Vier Prosastellen in `rundweg.rs` zählen fünf Fokuswerte, und der Baum trägt sechs

---
`crates/krk-ui/src/kommandos/rundweg.rs` steht in der Dateiliste von Schritt 11 der Runde 23 (Nachzug der Zählaussagen über Bereiche und Fokuswerte) und ist dort angefasst worden: die Tafel, ihr Probenname und `JEDER_FOKUS` sind auf sechs gezogen. Vier Prosastellen derselben Datei sind stehen geblieben und nennen weiter fünf:

- `:24` — Überschrift `# Eine Groesse, fuenf Werte`
- `:26` — „Die Regel hängt an nichts als am [`Fokus`]. Drei seiner **fünf** Werte tragen einen Ausgang, **zwei** tragen keinen" — beide Zahlen sind falsch; es sind drei von sechs, und drei tragen keinen (`Leiste`, `Git`, `Anderswo`)
- `:65` — „[`rundweg`] antwortet trotzdem für alle **fünf** Werte"
- `:160` — „eine zweite Liste derselben **fünf** Werte", unmittelbar über `const JEDER_FOKUS: [Fokus; 6]`

Die letzte steht eine Zeile über der Konstante, die sie beschreibt, und widerspricht ihr damit im selben Blick.

Das Erhebungsmuster aus C9.4 hätte drei der vier gefunden (`fuenf Fokuswert` trifft nicht, `fuenf Werte` schon); Schritt 11 hat sein Muster laut Auftrag um `fuenf Werten` erweitert, nicht um `fuenf Werte`.

**Abnahmetest:** `grep -nE 'fuenf (Werte|Fokuswert)' crates/krk-ui/src/kommandos/rundweg.rs` liefert nichts, und die Aussage über die Zahl der Werte ohne Ausgang stimmt mit `rundweg` überein.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23 durch Wiederholung der Erhebung aus C9.4 mit erweitertem Muster. Verwandt: `260831-1212_*_die-zaehlaussagen-ueber-spalten-und-schalter-stehen-in-sieben-dateien-die-schritt-12-nicht-fuehrt.md` (derselbe Befundtyp, andere Hälfte der Erhebung).

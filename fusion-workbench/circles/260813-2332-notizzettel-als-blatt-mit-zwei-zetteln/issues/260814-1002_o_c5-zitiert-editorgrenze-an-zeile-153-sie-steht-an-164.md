C5 zitiert EDITORGRENZE an datei.rs:153; sie steht an :164

---

Das achte Kriterium der ersten Liste von C5 sagt: „Der Baum führt `EDITORGRENZE` nach dieser
Runde weiterhin an genau einer Stelle (`crates/krk-core/src/text/datei.rs:153`); eine zweite
Zahl für dieselbe Sache entsteht nicht."

Die Zusage hält, die Fundstelle nicht. `pub const EDITORGRENZE: u64 = 16 * 1024 * 1024;` steht
am 260814-1002 an `crates/krk-core/src/text/datei.rs:164`. An `:153` steht eine Zeile aus dem
Doc-Kommentar darüber, der Verweis auf den Nutzerentscheid vom 260808-0017.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten, und die Aussage selbst stimmt: über alle
`crates/` hinweg steht die Zahl genau einmal, und jede weitere Fundstelle liegt in
`crates/krk-core/tests/text.rs` und `tests/textkopien.rs`, wo sie als `datei::EDITORGRENZE`
gelesen und nicht wiederholt wird.

**Die Abweichung stammt aus dieser Runde selbst.** Schritt 1 hat `crates/krk-core/src/text/datei.rs`
um `Textstand`, `Unlesbarkeit` und `lesen` erweitert; die Datei ist um 232 Zeilen gewachsen,
und die Konstante ist dabei um elf Zeilen nach unten gerutscht. Das Kriterium ist am
260813-2348 geschrieben und beim Nachtrag vom 260814-0628 nicht nachgezogen worden.

**Warum es trotzdem aufgeschrieben ist.** Eine Zeilennummer in einem Abnahmekriterium ist die
Anweisung, wo nachzusehen ist. Wer der Nummer folgt, findet die Konstante nicht und liest
stattdessen einen Absatz über eine andere Entscheidung — bei einem Kriterium, dessen ganzer
Inhalt „genau eine Stelle" lautet, ist das die schlechteste Stelle für einen Fehlgriff. Die
Runden 7 und 8 haben Befunde derselben Bauart abgelegt, und dieses Projekt zieht Zahlen in
Prosa aus genau diesem Grund einzeln nach.

**Was zu tun ist.** Im Kriterium `:153` auf `:164` ziehen, oder die Nummer weglassen und
allein die Datei nennen — die Konstante ist über ihren Namen eindeutig zu finden, und eine
Zahl, die bei jeder Änderung der Datei mitwächst, veraltet wieder.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Gemessen mit `grep -rn 'EDITORGRENZE' crates/` am Stand `79dab20`.

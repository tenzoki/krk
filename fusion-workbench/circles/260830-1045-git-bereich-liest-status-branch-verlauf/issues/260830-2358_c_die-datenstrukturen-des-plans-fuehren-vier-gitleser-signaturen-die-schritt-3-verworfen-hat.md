Die Datenstrukturen des Plans führen vier `Gitleser`-Signaturen, die Schritt 3 verworfen hat

---

`## Data Structures` des Plans der Runde 23 schreibt den `Gitleser` mit vier Signaturen aus, die
der Baum seit `1d84f2b` nicht mehr trägt. Schritt 3 ist von ihnen abgewichen, hat den Grund
gemessen und in seinem History-Eintrag belegt (`260830-1620-coder-schritt-3-gix-und-der-gitleser.md`,
Abschnitt „`crates/krk-core/src/git/leser.rs`"): unter `ulimit -n 64` scheitert `gix::discover` an
einem echten Repository, und eine zweiwertige Antwort müsste das als „kein Repository" ausgeben —
genau die Verwechslung, die C7.8 verbietet. Der Plan selbst ist nicht nachgezogen.

| `## Data Structures` (`:517-521`) | `crates/krk-core/src/git/leser.rs` |
|---|---|
| `pub fn oeffnen(ordner: &Path) -> Option<Self>` | `pub fn oeffnen(ordner: &Path) -> Oeffnung` (`:143`) |
| `pub fn kopf(&self) -> Kopf` | `pub fn kopf(&self) -> Option<Kopf>` (`:161`) |
| `pub fn verlauf(&self, ab, zahl) -> Vec<Commit>` | `-> Option<Vec<Commit>>` (`:198`) |
| `pub fn marken(&self, ordner) -> Vec<(String, Marke)>` | `-> Option<Vec<(String, Marke)>>` (`:255`) |

**Warum das mehr ist als eine veraltete Zeile.** Die Schritte 5, 6 und 7 lesen genau diesen
Abschnitt für ihre eigenen Signaturen; wer ihn als Vorlage nimmt, baut den Rufer, den der
Übersetzer danach ablehnt, oder — schlimmer — zieht `Unentschieden` und `KeinRepository`
zusammen. Der Schritt 4 ist auf der gebauten Fassung gefahren und hat die Regel im Modulkopf von
`crates/krk-core/src/git/lauf.rs` ausgeschrieben; der Plan sagt weiter das Gegenteil.

Zwei kleinere Abweichungen desselben Abschnitts stehen daneben und sind nicht der Gegenstand
dieses Datensatzes: `pub struct Gitleser;` steht dort ohne Feld, und `Gitlauf::starten` steht ohne
`#[must_use]`, das der Baum trägt.

**Abnahme:** die vier Zeilen in `## Data Structures` nennen die Rückgabewerte, die
`crates/krk-core/src/git/leser.rs` trägt, oder der Abschnitt nennt die Abweichung samt dem
History-Eintrag, der sie begründet.

**Resolved:** 260831, beim Bau von Schritt 5. `## Data Structures` des Plans nennt jetzt die vier
Rückgabewerte, die `crates/krk-core/src/git/leser.rs` trägt — `Oeffnung` samt ihren drei Werten und
das `Option` an `kopf`, `verlauf` und `marken` —, und ein Absatz unter dem Codeblock schreibt die
Abweichung samt der Messung und dem History-Eintrag aus, der sie begründet. Die zwei kleineren
Abweichungen sind mitgezogen: `Gitleser` steht mit seinem Feld, `Gitlauf::starten` mit
`#[must_use]`. Damit ist die Abnahme dieses Datensatzes auf beiden Wegen erfüllt.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
Gefunden beim Bau von Schritt 4, der auf allen vier Signaturen aufsetzt.

Drei Signaturen in den Datenstrukturen des Plans sind mit den Durchsichtsbefunden wieder veraltet

---
`## Data Structures` des Plans der Runde 23 (`260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md:517-522`) nennt drei Formen, die der Baum seit dem 260831 nicht mehr trägt:

| Plan | Baum |
|---|---|
| `pub fn verlauf(&self, ab: Option<ObjectId>, zahl: usize)` | `pub fn verlauf(&self, bereits: usize, zahl: usize)` (`crates/krk-core/src/git/leser.rs`) |
| `pub fn marken(&self, ordner: &Path)` | `pub fn marken(&self, ordner: &Path, abbruch: &AtomicBool)` (dieselbe Datei) |
| `pub enum Gitfrage { Ganz, WeitererVerlauf { ab: ObjectId } }` | `WeitererVerlauf { bereits: usize }` (`crates/krk-core/src/git/lauf.rs`) |

Die ersten beiden Zeilen der rechten Spalte kommen aus der Behebung zweier Durchsichtsbefunde: `260831-1444_*_der-nachschlag-des-verlaufs-setzt-am-letzten-commit-an-und-verliert-jeden-nebenzweig.md` hat den Nachschlag auf eine Zahl umgestellt, `260831-1444_*_ein-abgebrochener-gitlauf-laeuft-weiter-und-a10-gilt-nur-dem-halter-und-nicht-dem-faden.md` das Abbruchkennzeichen hereingereicht.

Dasselbe ist an demselben Abschnitt schon einmal aufgelaufen und dort begründet, warum es mehr ist als eine veraltete Zeile: `260830-2358_*_die-datenstrukturen-des-plans-fuehren-vier-gitleser-signaturen-die-schritt-3-verworfen-hat.md`.

**Abnahmetest:** die drei Zeilen nennen die Formen, die `crates/krk-core/src/git/leser.rs` und `crates/krk-core/src/git/lauf.rs` tragen, oder der Absatz unter dem Codeblock nennt die Abweichung samt dem Datensatz, der sie verursacht hat.

**Resolved:** 260831. `## Data Structures` des Plans der Runde 23 nennt die drei Formen, die der Baum trägt: `verlauf(&self, bereits: usize, zahl: usize)`, `marken(&self, ordner: &Path, abbruch: &AtomicBool)` und `Gitfrage::WeitererVerlauf { bereits: usize }`. Ein Absatz unter dem Codeblock nennt beide Abweichungen samt dem Datensatz, der sie verursacht hat — `260831-1444_*_der-nachschlag-des-verlaufs-setzt-am-letzten-commit-an-und-verliert-jeden-nebenzweig.md` für die Zahl statt des Commits, `260831-1444_*_ein-abgebrochener-gitlauf-laeuft-weiter-und-a10-gilt-nur-dem-halter-und-nicht-dem-faden.md` für das Abbruchkennzeichen — und steht neben dem Absatz, den die erste Runde desselben Befunds hinterlassen hat. Die Abnahme ist damit auf beiden Wegen erfüllt.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden beim Beheben der drei Codebefunde der Durchsicht; die zweite und dritte Zeile sind von dieser Arbeit selbst verursacht.

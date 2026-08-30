# Coder-Sitzung: Schritt 5 der Runde 23, der Gitbefund im Ordnermodell

**Date:** 2026-08-31
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 5
**HEAD:** `437fd69` (nicht committet; der Orchestrator committet)

## Was gebaut wurde

### `crates/krk-core/src/verzeichnis/modell.rs` — die einzige angefasste Codedatei

`gitmarke: Vec<Option<Marke>>` parallel zu `eintraege`, in der Bauart von `markiert`, `befund`
und `grund`. `Ordnermodell::neu` legt ihn leer an, `anhaengen` hängt je Eintrag ein `None` an,
`ersatz_einloesen` leert ihn mit den drei anderen, und `befund_zuruecksetzen` fasst ihn **nicht**
an. Das ist die ganze Ungültigkeitsregel: die Marke fällt mit dem Bestand und mit nichts sonst.

`#[must_use] pub fn gitmarken_setzen(&mut self, generation: u64, marken: &[(String, Marke)]) -> bool`
weist ab, solange `generation != self.generation` oder `ersatz_ausstehend` steht; sonst baut es
einmal eine `HashMap<&str, usize>` über den Bestand, trägt die gefundenen Namen ein und liefert,
ob etwas eingetragen wurde. `#[must_use] pub fn gitmarke(&self, eintragsindex: u32) -> Option<Marke>`
als Leseseite, mit `None` für jeden Index außerhalb des Bestands — dieselbe Antwort, die
`Ordnermodell::befund` dort mit `Unentschieden` gibt.

**Der Doc-Kommentar nennt beide Unterschiede zu `befunde_setzen` unter einer eigenen Überschrift**
(`# Zwei Unterschiede zu Ordnermodell::befunde_setzen`): die Zuordnung über den Namen und die
daraus folgende Generationsprüfung, und dass die Sicht nicht neu aufgebaut wird. Zwei
nebeneinanderstehende Setzer, die sich in zwei Punkten unterscheiden, laufen sonst zusammen.

Der Modulkopf bekommt den Abschnitt `# Zwei Befundvektoren, zwei Ungueltigkeitsregeln` mit einer
Tafel über die vier Anlässe (Filtertext getippt, „Content" umgelegt, Verstecke umgelegt,
Ordnerwechsel) und dem Satz, warum ein gemeinsamer Vektor die eine Frage mit der anderen
wegwürfe. Die Doc-Kommentare von `ersatz_einloesen` und `befund_zuruecksetzen` ziehen nach: das
erste zählt jetzt vier Vektoren statt dreier, das zweite sagt ausdrücklich, dass es `gitmarke`
nicht anfasst.

### Eine Stelle, an der der Bau über den Plantext hinausgeht

`gitmarke()` trägt `#[must_use]`, obwohl der Auftragstext es nur für `gitmarken_setzen`
ausdrücklich verlangt. `## Data Structures` des Plans führt es an beiden, und die Regel des
Projekts gilt jedem Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe; ein Getter, dessen
Wert fällt, ist eine tote Anweisung. Der Nachbar `befund()` trägt es nicht — das ist eine
Ungleichheit im Baum, aber sie zu übernehmen hieße, die Regel für einen neuen Rückgabewert zu
brechen, und nicht umgekehrt.

## Die Proben

Sechs, alle im `#[cfg(test)]`-Modul neben dem Code. **Keine ruft `git`**: sie reichen die Marken
von Hand herein, so wie der Gitlauf sie über seinen Kanal liefert. Die Läufe gegen ein angelegtes
Repository stehen in `crates/krk-core/tests/git.rs` aus Schritt 3 und 4; die Zählprüfung der
Runde 8 bleibt damit unberührt, und eine vierte Prüfordner-Fassung entsteht nicht.

| Probe | Kriterium |
|---|---|
| `die_fuenf_marken_stehen_an_ihren_eintraegen_und_der_unveraenderte_traegt_keine` | C5.3 (Modellhälfte), A11 |
| `ein_befund_mit_fremder_generation_traegt_keine_marke_ein` | C7.5 |
| `ein_befund_waehrend_des_vorgemerkten_ersatzes_schreibt_nichts_in_den_alten_bestand` | C7.4 |
| `ein_tippen_wirft_nur_den_filterbefund_weg_ein_ordnerwechsel_beide` | C7.6, Bedingung 5 |
| `ein_unbekannter_name_wird_verworfen_ohne_die_uebrigen_zu_verhindern` | die Verwerfungszusage aus Schritt 5 |
| `gitmarken_setzen_baut_die_sicht_nicht_neu_auf` | Entscheidung 4, zweiter Unterschied |

**Die letzte hängt an der einen Lage, in der der Unterschied überhaupt sichtbar ist.** Während
eines Lesevorgangs steht die Sicht in Lesereihenfolge; erst `abschliessen` sortiert sie. Ein
Setzer, der `sicht_neu_aufbauen` riefe, sortierte dort vorzeitig um. Die Probe misst das an zwei
Einträgen in umgekehrter Reihenfolge und hält daneben eine **Gegenprobe** mit `befunde_setzen`, die
genau diese Umsortierung auslöst — ohne sie hinge die Zusage an der Annahme, ein Neuaufbau wäre an
dieser Stelle überhaupt zu sehen.

`ein_befund_waehrend_des_vorgemerkten_ersatzes_…` prüft den härteren Fall: der Befund nennt die
**neue** Generation und der alte Bestand führt dieselben Namen. Allein `ersatz_ausstehend` hält
ihn ab. Nach dem eingelösten Ersatz nimmt dasselbe Modell denselben Befund an.

## Der Plan und der Defekt

`## Data Structures` ist auf den gebauten Stand gezogen: `Oeffnung` samt ihren drei Werten,
`oeffnen -> Oeffnung`, `Option` an `kopf`, `verlauf` und `marken`, `Gitleser` mit seinem Feld,
`#[must_use]` an `Gitlauf::starten`. Darunter steht ein Absatz, der die Abweichung samt der
Messung unter `ulimit -n 64` und dem History-Eintrag von Schritt 3 begründet — der Abschnitt
allein hätte den Schritten 6 und 7 sonst dieselbe Falle gestellt.

`260830-2358_*_die-datenstrukturen-des-plans-fuehren-vier-gitleser-signaturen-die-schritt-3-verworfen-hat.md`
ist mit einer `Resolved:`-Zeile geschlossen und von `_o_` auf `_c_` umbenannt.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, 16 Proben in `verzeichnis::modell`, davon die
sechs neuen.

Kein `git stash`, kein `git checkout .`, kein `git reset --hard`, kein `git clean`, kein
`git restore .`. Nicht committet.

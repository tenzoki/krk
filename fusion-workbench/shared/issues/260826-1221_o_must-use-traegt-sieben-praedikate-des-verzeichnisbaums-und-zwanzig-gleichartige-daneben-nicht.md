`#[must_use]` traegt sieben Praedikate des Verzeichnisbaums und zwanzig gleichartige daneben nicht

---

CLAUDE.md fuehrt die Regel als bindend: „Ein Rueckgabewert, dessen stilles Fallenlassen
unbemerkt bliebe, bekommt in diesem Projekt `#[must_use]`", entschieden vom Nutzer am
260811-2140. Unter `crates/krk-core/src/verzeichnis/` tragen es sieben Stellen. Rund zwanzig
Funktionen derselben Bauart — reine Praedikate und Leser ohne Nebenwirkung — tragen es nicht,
und zwei der Paare stehen so dicht beieinander, dass die Auslassung nicht als Abwaegung zu
lesen ist.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Affected:** `crates/krk-core/src/verzeichnis/modell.rs`, `filter.rs`, `leser.rs`,
`eintrag.rs`, `kollation.rs`, `sortierung.rs`, `mod.rs`
**Tree state:** `004ff72`
**Domain:** code

## Die zwei Paare, an denen es sichtbar wird

**Erstes Paar, zwanzig Zeilen auseinander in derselben `impl`:**

```rust
// crates/krk-core/src/verzeichnis/modell.rs:982-984
pub fn tief(&self) -> bool { self.tief }          // ohne

// crates/krk-core/src/verzeichnis/modell.rs:1005-1008
#[must_use]
pub fn inhalt(&self) -> bool { self.inhalt }      // mit
```

Es sind die beiden Ankreuzfelder desselben Filters, dieselbe Signatur, dieselbe Zeile Rumpf.

**Zweites Paar, in `filter.rs`, dessen eigener Modulkopf von „den drei Regeln des Filters"
spricht (`filter.rs:1-2`):**

| Regel | Zeile | `#[must_use]` |
|---|---|---|
| `traegt_ein_dateiname` | `filter.rs:90` | nein |
| `traegt_die_folge` | `filter.rs:122` | nein |
| `inhaltsschwelle` | `filter.rs:156` | **ja** |

Die Begruendung bei `inhaltsschwelle` (`filter.rs:154-155`) trifft die beiden anderen
wortgleich: „weil der Aufruf nichts tut ausser zu antworten: wer den Wert fallen laesst, hat
ihn umsonst geholt, und still."

## Die uebrige Erhebung

Getragen (7): `verweisziel::bestimmen`, `arbeitsbaum::{traegt,liegt_in,beruehrt_einen}_...`,
`filter::inhaltsschwelle`, `inhalt::traegt_der_inhalt`, `umfang::zaehlen`,
`sys::ist_deskriptormangel`, `Sperrversuch`, `Ortszeit`, `Loeschzielbefund::{ist_warnwuerdig,oder}`,
`Ordnermodell::{steht_wegen_des_inhalts,auftraege,bestand,letztes_zeichen_weg,inhalt,inhalt_wirkt}`.

Nicht getragen, gleiche Bauart:

- `modell.rs`: `tief`, `filter_steht`, `verstecke_ausgeblendet`, `ist_markiert`, `befund`,
  `generation`, `gehoert_dazu`, `ersetzt_beim_naechsten_stapel`, `markierungsstand`, `auswahl`,
  `auswahl_zeile`, `zeile_von`, `eintragsindex`, `zeilenzahl`, `index_von_namen`, `sortierung`,
  `filtertext`, `filter_klein`, `zeile`, `Markierungsstand::ist_leer`
- `filter.rs`: `traegt_ein_dateiname`, `traegt_die_folge`
- `leser.rs`: `Abschluss::ist_abgebrochen`, `Abschluss::ist_vollstaendig`, `meldungen`
- `eintrag.rs`: `endung`, `ist_ordner`, `ist_verknuepfung`
- `kollation.rs`: `schluessel`
- `sortierung.rs`: `Richtung::umgekehrt`, `Sortierung::{neu,alle,vergleiche}`
- `mod.rs`: `aufwaerts`
- `durchlauf.rs`: `Durchlauf::{befunde,zu_gross}`

## Warum das kein blosser Ordnungswunsch ist

Der Bau faengt die Auslassung nicht: `unused_must_use` ist erst unter `-D warnings` ein Fehler,
und ohne das Attribut gibt es gar keine Warnung, die verschaerft werden koennte. CLAUDE.md
haelt genau das fest. Der Baum hat den Fall schon einmal gehabt
(`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/issues/260820-0739_o_text-schreiben-hat-sein-must-use-bei-der-aufteilung-nicht-mitbekommen.md`,
offen) und dort als Defekt gefuehrt.

## Richtung

Nicht pauschal alles anhaengen. Zu klaeren ist zuerst, ob die Regel „jeden reinen Leser" meint
oder „jeden Wert, dessen Verlust eine Zusage bricht" — die getragenen sieben sind
ueberwiegend die zweite Sorte, und die Doc-Kommentare bei `filter::inhaltsschwelle` und
`Ordnermodell::inhalt` argumentieren mit der ersten. Solange beide Lesarten nebeneinander im
Baum stehen, waechst die Liste oben mit jeder Runde weiter. Der billige erste Schritt ist das
Paar `tief`/`inhalt`: dort ist keine Lesart denkbar, unter der die zwei verschieden ausgehen.

Also seen: 260826-1221 by coderev — dieselbe Luecke in der Vorgangsmaschine und im Stapelumbenennen: `shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-und-des-stapelumbenennens.md`. Die zwei Datensaetze sind derselbe Befund an zwei Umfaengen und keine Doppelung.

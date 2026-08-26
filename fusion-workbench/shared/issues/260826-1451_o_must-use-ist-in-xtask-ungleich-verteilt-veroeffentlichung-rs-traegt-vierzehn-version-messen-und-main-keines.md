`#[must_use]` ist in `xtask` ungleich verteilt: `veroeffentlichung.rs` trägt vierzehn, `version.rs`, `messen.rs` und `main.rs` keines
---
Rund 23 reine Antworten der Kiste tragen das Attribut nicht, das `CLAUDE.md` seit dem 260811 verlangt; eine Datei trägt es an jeder.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/*.rs`

## Befund

Zählung per `grep -c '#\[must_use\]' xtask/src/*.rs`: `veroeffentlichung.rs` 14, `git.rs` 6, `release.rs` 2, `beglaubigung.rs` 2, `bundle.rs` 1, `sign.rs` 1, `version.rs` 0, `messen.rs` 0, `main.rs` 0.

Reine Antworten ohne Attribut, außerhalb der Proben:

| Datei | Funktionen |
|---|---|
| `git.rs` | `tag_steht` (`:779`), `geaenderte_dateien` (`:767`), `aufsichtsbefund` (`:540`), `gewaltbefund` (`:621`), `stellungsbefund` (`:603`), `Gestalt::befund` (`:167`), `tagnamenbefund` (`:208`), `steuerzeichenbefund` (`:220`), `verwandte_marke` (`:683`), `kurze_marke` (`:698`) |
| `sign.rs` | `developer_id_namen` (`:130`), `aus_umgebung` (`:254`), `abschnitt_der_treffer` (`:316`), `enthaelt_identitaet` (`:363`), `gueltige_namen` (`:374`), `eintragsname` (`:392`), `anleitung` (`:413`) |
| `bundle.rs` | `cargo` (`:278`), `wurzel` (`:299`), `plist_zeichenkette` (`:465`), `zielpfad` (`:522`), `Vorlage::binaer_im_buendel` (`:260`) |
| `release.rs` | `verletzt_grenze` (`:492`), `nennt_objc2_pfad` (`:510`), `ist_bezeichnerzeichen` (`:535`), `ist_objc2_use` (`:555`), `sichtbarkeit_abstreifen` (`:576`) |
| `version.rs` | `zuruecknehmen` (`:272`), `arbeitsbaum_meldung` (`:388`), `wertspanne` (`:515`), `eintragsmeldung` (`:531`) |

Heute lässt kein Rufer eine davon fallen; der Befund ist die Regel aus `CLAUDE.md` („Ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`"), und `veroeffentlichung.rs` zeigt, wie die Kiste sie meint. Dieselbe Gestalt wie die acht `must-use`-Datensätze dieser Sitzung (`260826-1221`, `-1223`, `-1225`, `-1305`, `-1325`, `-1327`, `-1335`, `-1417`, `-1421`).

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L7

# Q4: die offenen Befunde in `krk-ui/src/` ausserhalb von `appkit/`

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Die offenen Defektdatensaetze abarbeiten, deren Behebung in `crates/krk-ui/src/` landet, aber
nicht unter `appkit/` und nicht in `crates/krk-core/`. Jeden gegen den heutigen Baum lesen,
bevor er behoben wird. Nicht committen. HEAD war `fe5fe9c`.

## Die Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

189 offene Datensaetze. Davon zitieren 76 eine Datei unter `crates/krk-ui/src/`; 39 davon
zitieren **keine** unter `appkit/`, und 29 dieser 39 zitieren daneben auch keine unter
`crates/krk-core/`. Alle 39 sind gelesen worden, die zehn mit Kernbezug ebenfalls, weil ein
blosses Zitat des Kerns die Behebung noch nicht dorthin verlegt.

## Behoben

| Datensatz | Datei | Was |
|---|---|---|
| `260907-1422_*` | `messmodus.rs` | `messung_unmoeglich` traegt keinen `_`-Zweig mehr |
| `260826-1418_*` | `editormodell.rs` | eine gehaltene Datei ohne Stempel gilt als geaendert |
| `260812-2133_*` | `markdown.rs` | `merkzeichen_einloesen` ohne inneren Durchlauf |
| `260826-1442_*` (apply) | `hervorhebung.rs` | beide Abbrueche der Kiste lassen den Stand fallen |
| `260826-1442_*` (Messstelle) | `hervorhebung.rs` | `#[ignore]`-Messstelle fuer C3 |
| `260826-1442_*` (Pruefordner) | `pruefordner.rs`, `kommandos/pfadeingabe.rs` | zweistufiges Abraeumen wie im Kern |
| `260826-1442_*` (ordner_a) | `messmodus.rs` | `pruefen` weist die vier stillen Rueckfaelle ab |
| `260825-1425_*` (Bytelaenge) | `kommandos/kontextmenue.rs` | Ordnung nach der gefalteten Laenge |
| `260814-0912_*` | `belegungsausgabe.rs` | die letzte der neun Stellen, jetzt ohne Zahl |
| `260812-2134_*` | `markdown.rs` | die Ungleichheit steht am Doc-Kommentar und unter einer Probe |
| `260813-0416_*` | — | die Sperre ist gefallen: `260813-0430_*` steht auf `_i_` |
| `260907-1226_*` | — | am Baum steht nichts mehr aus |

Die Einzelheiten stehen je im `Resolved:`-Nachsatz des Datensatzes und werden hier nicht
wiederholt.

## Gemessen statt behauptet

- `merkzeichen_einloesen`, `--release`, Bestzeit aus sieben Laeufen ueber `"- " x Tiefe`:
  12 kB von 161,6 ms auf 42,6 ms, 20 kB von 408,0 ms auf 120,1 ms, 2 kB von 2,60 ms auf
  0,79 ms. Die Grenze von L7 wandert von rund 12 kB auf rund 19 kB zurueck.
- Die neue Messstelle der Einfaerbung, `--release`, `appkit/anwendung.rs` (546 429 Bytes):
  voller Durchgang 1,897 s (0,29 MB/s), ein Anschlag in der Mitte 8,7 ms, 329 Haltepunkte.
  Die 0,3 MB/s des Modulkopfs halten.
- Zwei neue Proben sind mutationsgeprueft:
  `pruefordner::tests::ein_unterordner_ohne_rechte_haelt_das_abraeumen_nicht_auf` und
  `drei_kelvinzeichen_kehren_die_ordnung_des_entpackschnitts_nicht_um`.

## Offen gelassen, mit Grund

- `260812-0700_*` (Breitenschritt), `260812-1805_*` (YAML-Front-Matter), `260816-2144_*`
  (Leertaste), `260826-1417_*` (16 MB nur am Eingang), `260826-1419_*` (letzter Tab),
  `260816-1710_*` (Rueckwechsel), `260815-0230_*` (`zeile_traegt`), `260825-1425_*`
  (Normalform), `260828-0744_*` (C6): jeder verlangt eine Wahl, die der Datensatz offen
  laesst.
- `260826-1442_*` (`gleicher_ordner`): der Vorschlag haelt die aufgeloesten Ordner beim Setzen
  des Stroms, und der liegt in `appkit/`. Die Kosten sind daneben ungemessen.
- `260813-0715_*`, `260813-0716_*`, `260813-0720_*`, `260813-1110_*`, `260818-0415_*`,
  `260826-1221_*`, `260826-1223_*`, `260826-1933_*`, `260905-0406_*`: die Behebung landet in
  `crates/krk-core/` oder in `xtask/`.
- `260813-0420_*`, `260831-1355_*`: Domaene Daten, `resources/default-keymap.toml`.
- `260817-1122_*`, `260907-2350_*`, `260820-2056_*`: die Behebung ist eine Aenderung an
  Werkbank-Datensaetzen und nicht am Code; bei `260907-2350_*` sind beide Ziele terminale
  Datensaetze, und ob deren Rumpf berichtigt oder ein Nachsatz angehaengt wird, ist die
  offene Frage.
- `260823-1210_*`: braucht den naechsten roten Lauf mit erhaltener Ausgabe.
- `260818-0414_*`: „Nothing to change in the code"; die Schliessung gehoert der
  Abschlussnotiz der Runde.

## Neu abgelegt

`260908-0719_*_zwei-saetze-zur-kuerzelregel-fehlen-in-default-keymap-toml-seit-die-frage-dahinter-beantwortet-ist.md`
im Circle der Runde 7: die zwei vom `ontorev` empfohlenen Saetze in
`resources/default-keymap.toml` sind seit dem 260906-2147 schreibbar und stehen nicht da.

## Abnahme

Am 260908 gefahren, alle fuenf:

```
cargo build --workspace                              Exit 0
cargo test --workspace                               Exit 0  (25 Probenlaeufe, keiner rot)
cargo test -p krk-ui                                 Exit 0  (932 bestanden, 1 uebersprungen)
cargo clippy --workspace --all-targets -- -D warnings Exit 0
cargo fmt --all --check                              zwei fremde Dateien, keine aus dieser Bahn
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  Exit 0
```

Der eine uebersprungene Lauf ist die neue Messstelle der Einfaerbung; sie traegt `#[ignore]`
mit Begruendung.

## Fremde Bahnen

Waehrend dieses Durchgangs waren `crates/krk-ui/src/appkit/`, `crates/krk-core/` und
`xtask/`+`krk-bench/` von drei weiteren Laeufen in Arbeit. Zweimal ist ein Lauf an ihren
Aenderungen rot geworden, beide Male ausserhalb dieser Bahn:
`appkit::editor::tests::der_verwalter_gibt_den_block_…` (der Probenname hat sich zwischen zwei
Laeufen geaendert, also war die Datei gerade im Umbau) und `clippy::double_must_use` an
`appkit/blaetter/mod.rs:861`. `cargo fmt --all --check` meldet vier Dateien, keine davon aus
dieser Bahn; die zwei eigenen sind formatiert.

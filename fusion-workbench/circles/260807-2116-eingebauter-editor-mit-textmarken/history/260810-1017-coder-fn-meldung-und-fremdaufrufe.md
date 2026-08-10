# coder — Fehlermeldung zur fn-Taste und die Aufzählung der Fremdaufrufe

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Bearbeitet:** `crates/krk-core/src/tasten/parser.rs`, `crates/krk-core/src/lib.rs`, `crates/krk-core/src/verzeichnis/mod.rs`

## Auftrag

Zwei offene Defekte in `crates/krk-core`, beide Aussagen über den eigenen Bestand, die nicht stimmen:

- `260810-0935` — die Fehlermeldung zu `fn+f3` behauptet "KRK belegt den Tastencode" als allgemeine Regel.
- `260810-0955` — zwei Dateiköpfe zählen die Fremdaufrufe des Kerns auf und nennen drei.

Dateigrenze: die drei oben genannten Dateien. `crates/krk-core/src/text/**`, `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-ui/**`, `resources/**` und das Plandokument nicht anfassen, weil dort parallel andere Agenten arbeiten.

## Was getan wurde

**`260810-0935`, eine Wortänderung in `parser.rs`.** Der Zweig `FnAlsZusatztaste` in `fmt::Display for Schreibfehler` sagt jetzt "Funktionstasten schlaegt KRK ueber den Tastencode nach" statt "KRK belegt den Tastencode", und das Rückbezugswort "denselben" ist zu "denselben Tastencode" ausgeschrieben, weil der neue erste Halbsatz den Tastencode in anderer Satzrolle führt. Der Wortlaut ist mit den drei bereits richtiggestellten Stellen abgeglichen: dem Dateikopf von `resources/default-keymap.toml` Zeile 42, dem Block bei Zeile 484 derselben Datei und dem Modulkopf von `parser.rs`. Von der Keymap-Fassung weicht die Meldung in einem Wort ab, "fn" statt "fn-Taste", weil ihr eigener erster Halbsatz die Taste unmittelbar davor "fn" nennt.

Zusammensetzung geprüft und nicht nur gelesen: die Meldung steht über zwei Fortsetzungsstriche auf drei Quellzeilen. Derselbe String durch `rustc` gejagt ergibt eine Zeile ohne Doppelleerzeichen und ohne fehlendes Leerzeichen an den Nahtstellen.

**`260810-0955`, zwei Modulköpfe, und die Zahl ist nicht vier.** Am Bestand ermittelt: `krk-core` hat drei `unsafe extern "C"`-Blöcke, alle in `verzeichnis/sys.rs`, und sie binden **acht** Funktionen — `getattrlistbulk`, `copyfile`, die vier `copyfile_state_*`, `renamex_np` und `fcntl`. Alle acht werden auch gerufen, die vier Helfer in `datei_kopieren`, `mit_zustand_kopieren`, `wurde_geklont` und im Statusrückruf. Vier ist die Zahl der Schnittstellen.

`lib.rs` und `verzeichnis/mod.rs` nennen deshalb jetzt beide Zahlen in derselben Formulierung: "vier Schnittstellen und acht gebundene Funktionen, denn `copyfile(3)` braucht seine vier `copyfile_state_*`-Helfer". Nur die Schnittstellenzahl zu nennen hätte den gemeldeten Defekt in kleinerem Maßstab wiederholt. `mod.rs` hat daneben den Anlass der `fcntl`-Bindung bekommen und den Folgesatz "aus dem ersten der drei Aufrufe" mitgezogen; `lib.rs` hat seinen Schlusssatz berichtigt, der Schritt 15 als letzten Zuwachs benannte.

## Abnahme

```
cargo build --workspace                  → exit 0
cargo test --workspace                   → exit 0   (15 Testziele, 730 bestanden, 1 ausgelassen, 0 gescheitert)
cargo clippy --workspace --all-targets   → exit 0   (keine Warnung)
cargo fmt -p krk-core -- --check         → exit 0
```

`cargo fmt` auf `krk-core` beschränkt statt `--all`, wie im Auftrag verlangt, weil an `krk-ui` zur selben Zeit zwei andere Agenten arbeiten.

Nachgesucht ist über den Baum, ob eine der beiden Behauptungen anderswo weiterlebt. `belegt den Tastencode` kommt unter `crates/` nicht mehr vor; die verbleibenden Treffer stehen in Datensätzen und Sitzungsberichten sowie im geschlossenen Spec der Runde 1, wo `260810-0935` sie ausdrücklich als keinen Defekt einordnet. Die Aufzählung der Fremdaufrufe ist über `Fremdaufruf`, `Systemaufrufe`, `drei Aufrufe` und `renamex_np` gesucht, über `crates/`, `README.md` und `Cargo.toml`: keine dritte Stelle.

## Neu gefundener Defekt

`260810-1017_o_der-kopf-von-sys-rs-nennt-vier-fremdaufrufe-und-bindet-acht-funktionen.md`, Schwere Low. Der Modulkopf von `verzeichnis/sys.rs` nennt "die vier Fremdaufrufe" und führt vier Zeilen im Diagramm, verschweigt also dieselben vier `copyfile_state_*`-Bindungen — derselbe Defekt wie `260810-0955`, eine Ebene tiefer. Nicht mitbehoben, weil `sys.rs` außerhalb der Dateigrenze lag. Der Datensatz führt die acht Bindungen mit Zeilennummer und Aufrufstelle und empfiehlt die Formulierung, die die beiden nachgezogenen Köpfe jetzt tragen.

## Nicht getan

Kein Commit: der Orchestrator committet nach Abschluss des Arbeitspakets. Die Marker `_o_` → `_c_` an den beiden behobenen Datensätzen benennt der Nutzer um.

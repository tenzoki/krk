# Die Aufzählung der Fremdaufrufe nennt drei, und es sind vier

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, beim Umbau von `oeffnen` auf den Deskriptor
**Betroffen:** `crates/krk-core/src/lib.rs:11`, `crates/krk-core/src/verzeichnis/mod.rs:11-13`
**Cross-references:** `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`

---

## Der Befund

`verzeichnis/sys.rs` bindet seit dem 260810 einen vierten Fremdaufruf,
`fcntl(2)`, für `ohne_warten_oeffnen`. Zwei Dateiköpfe außerhalb dieses Moduls
zählen die Aufrufe namentlich auf und stehen seither auf drei:

- `crates/krk-core/src/lib.rs:11` — "das Modul `verzeichnis::sys` bindet die
  Systemaufrufe `getattrlistbulk`, `copyfile` und `renamex_np`".
- `crates/krk-core/src/verzeichnis/mod.rs:11-13` — "[`sys`] ist die einzige
  Stelle im Kern mit einem Fremdaufruf und bindet `getattrlistbulk(2)` für das
  Lesen sowie, seit Schritt 15, `copyfile(3)` und `renamex_np(2)` für die
  Operationsmaschine."

Der Modulkopf von `sys.rs` selbst ist nachgezogen: er sagt "die vier
Fremdaufrufe" und führt `fcntl(2)` in seinem Diagramm.

## Warum das zählt

Die beiden Sätze sind keine Prosa, sondern die Stelle, an der ein Leser nachsieht,
welche Fremdbindungen der Kern hat und wo die eine Ausnahme von
`#![deny(unsafe_code)]` liegt. Eine Aufzählung, die eine Bindung verschweigt,
verschweigt genau das, was sie aufzuzählen behauptet. Der Satz in `mod.rs` trägt
außerdem die Aussage "die einzige Stelle im Kern mit einem Fremdaufruf", und die
bleibt wahr — falsch ist allein die Liste dahinter.

## Was zu tun ist

`fcntl(2)` in beiden Aufzählungen mitnennen, in `mod.rs` mit dem Anlass
(`ohne_warten_oeffnen` für `text::datei::oeffnen`). Kein Verhalten, nur zwei
Dateiköpfe.

## Warum es nicht mitgekommen ist

Beide Dateien lagen außerhalb der Dateigrenze des Arbeitspakets, das die vierte
Bindung angelegt hat. Die Grenze war ausdrücklich gesetzt, weil parallel andere
Agenten im Baum arbeiten.

---
Resolved: Beide Aufzählungen sind nachgezogen, und sie nennen jetzt zwei Zahlen statt einer. Geändert sind ausschließlich `crates/krk-core/src/lib.rs` (Zeilen 9-17) und `crates/krk-core/src/verzeichnis/mod.rs` (Zeilen 11-18), in beiden Fällen nur der Modulkopf; keine Zeile Code, kein Modulbaum, keine `pub use`-Zeile.

**Die Zahl ist am Bestand ermittelt und nicht aus diesem Datensatz übernommen — und sie ist nicht vier.** `krk-core` hat drei `unsafe extern "C"`-Blöcke, alle drei in `verzeichnis/sys.rs`, und sie binden zusammen **acht** Funktionen: `getattrlistbulk` (Zeile 109), `copyfile`, `copyfile_state_alloc`, `copyfile_state_free`, `copyfile_state_set`, `copyfile_state_get`, `renamex_np` (Zeilen 395-415) und `fcntl` (Zeile 684). Jede der acht wird auch gerufen, nicht nur deklariert: die vier `copyfile_state_*` in `mit_zustand_kopieren`, `wurde_geklont` und im Statusrückruf (Zeilen 504, 538, 547, 567, 575, 623). Vier ist die Zahl der **Schnittstellen** — `getattrlistbulk(2)`, `copyfile(3)`, `renamex_np(2)`, `fcntl(2)` —, und die vier `copyfile_state_*` sind Zubehör von `copyfile(3)`: ohne sie lässt sich weder der Fortschrittsrückruf setzen noch die Zahl der kopierten Bytes abfragen.

Beide Köpfe sagen deshalb jetzt beides, in derselben Formulierung: "Das sind vier Schnittstellen und acht gebundene Funktionen, denn `copyfile(3)` braucht seine vier `copyfile_state_*`-Helfer." Nur die Schnittstellenzahl zu nennen hätte den gemeldeten Defekt in kleinerem Maßstab wiederholt, denn genau die vier Helfer sind die Bindungen, die eine Aufzählung mit einer einzigen Zahl verschweigt; nur die Symbolzahl zu nennen hätte den Kopf von `sys.rs` und sein Diagramm ohne Not in Widerspruch dazu gestellt.

`mod.rs` hat daneben den Anlass bekommen, wie im Datensatz verlangt: `fcntl(2)` steht dort "seit dem Defekt `260809-1652` ... fuer `ohne_warten_oeffnen`, den Eingang von `text::datei::oeffnen`". Der Satz "die einzige Stelle im Kern mit einem Fremdaufruf" steht unverändert; er ist am Bestand nachgeprüft und trägt weiter, denn alle drei `extern`-Blöcke liegen in `sys.rs`. Mitgezogen ist der Folgesatz "aus dem ersten der drei Aufrufe", der jetzt "aus der ersten der vier Schnittstellen" lautet — er wäre sonst die dritte Stelle mit der alten Zahl gewesen.

In `lib.rs` ist zusätzlich der Schlusssatz berichtigt. Er sagte "und mit Schritt 15 ist es das geblieben" und benannte damit den letzten Zuwachs falsch; er sagt jetzt, dass Schritt 15 `copyfile` und `renamex_np` und der Defekt `260809-1652` `fcntl` hinzugebracht haben. Das ist dieselbe Herkunftsangabe, die der Kopf von `sys.rs` seit dem 260810 führt.

Der Baum ist auf weitere Stellen mit dieser Aufzählung durchsucht (`Fremdaufruf`, `Systemaufrufe`, `drei Aufrufe`, `renamex_np`), über `crates/`, `README.md` und `Cargo.toml`. Es gibt keine dritte: die Treffer in `krk-ui` und in `Cargo.toml` sprechen von AppKit-Aufrufen und vom paarweisen Vergleich in `icu_collator`, nicht von den Bindungen des Kerns.

Verification: `cargo build --workspace` → exit 0, `cargo test --workspace` → exit 0 (15 Testziele, 730 bestandene Proben, eine ausgelassene, keine gescheiterte), `cargo clippy --workspace --all-targets` → exit 0 ohne Warnung, `cargo fmt -p krk-core -- --check` → exit 0. Auf `krk-core` beschränkt statt `--all`, weil an `krk-ui` zur selben Zeit andere Agenten arbeiten.

Ein neuer Defekt ist bei dieser Arbeit gefunden und nicht mitbehoben worden, weil `verzeichnis/sys.rs` außerhalb der Schreibgrenze lag: dessen eigener Modulkopf nennt "die vier Fremdaufrufe" und führt vier Zeilen im Diagramm, verschweigt also dieselben vier `copyfile_state_*`-Bindungen. Geführt als `issues/260810-1017_o_der-kopf-von-sys-rs-nennt-vier-fremdaufrufe-und-bindet-acht-funktionen.md`, Schwere Low.

Die Umbenennung des Markers `_o_` → `_c_` macht der Nutzer.

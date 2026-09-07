# K16: ein Prüflauf hält die macOS-Untergrenzen im Modulkopf

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Den Entscheid `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` umsetzen:
Möglichkeit 1 (der Abschnitt steht da) und Möglichkeit 2 (jede angesprochene Klasse ist
darin genannt), Möglichkeit 3 (die Zahlen prüfen) nicht. Der Prüflauf darf nicht so
heißen, als prüfe er mehr, als er hält.

## Die Erhebung

42 `.rs`-Dateien unter `crates/krk-ui/src/appkit/`, 33 auf der obersten Ebene und 11 unter
`blaetter/`. Erhoben mit
`find crates/krk-ui/src/appkit -name '*.rs' | wc -l` und
`grep -rL '# Ab welchem macOS die angesprochenen Klassen stehen' --include='*.rs' crates/krk-ui/src/appkit/`.

40 trugen den Abschnitt, zwei nicht: `koordinaten.rs` und `mod.rs`. Beide holen keinen
Namen aus einer `objc2_`-Kiste herein — die Ausnahme ist damit eine Eigenschaft und keine
Liste, und sie steht nirgends als Aufzählung.

**Möglichkeit 1 war am Bestand also schon erfüllt.** Möglichkeit 2 nicht: 196 Namen aus 66
verschiedenen, verteilt über 27 der 42 Dateien, kamen über eine `use`-Zeile herein, ohne
im Abschnitt genannt zu sein. Der Befund steht in
`260907-2256_*_die-untergrenzen-abschnitte-nannten-196-hereingeholte-namen-nicht-und-27-von-42-appkit-dateien-waren-betroffen.md`.

## Warum nicht nach Klassen gefragt wird

Ob ein hereingeholter Name eine Objective-C-Klasse benennt, ist am Quelltext nicht
entscheidbar: `NSPoint` und `NSPasteboard` sehen gleich aus, das eine ist eine C-Struktur
und das andere eine Klasse, und die `use`-Zeile trennt sie nicht. Der Entscheid
veranschlagt für diesen Unterschied eine Ausnahmeliste; die Messung zeigte, dass sie
größer geworden wäre als dort angenommen — nicht nur Aufzählungstypen, sondern auch
C-Strukturen, Ganzzahltypen, Protokolle, Kategorien, Konstanten und zwei reine
Rust-Werkzeuge der Kiste.

Statt die Frage zu nähern, ist die Frage gewechselt: gefordert ist, dass **jeder** Name im
Abschnitt steht, den eine `use`-Zeile auf oberster Ebene aus einer Kiste mit dem
Namensanfang `objc2_` hereinholt. Das ist am Baum entscheidbar, es ist eine Obermenge
dessen, was Möglichkeit 2 verlangt, und es braucht keine Ausnahmeliste. Die Kernkiste
`objc2` fällt am Kistennamen heraus und nicht an einem Eintrag.

## Was gebaut ist

`crates/krk-core/tests/baum.rs`, angehängt: die Konstante `UNTERGRENZEN_UEBERSCHRIFT`
(zusammengesetzt mit `concat!`), die Helfer `appkit_dateien`, `modulkopf`,
`untergrenzen_abschnitt`, `frameworknamen` und `steht_als_wort` und die zwei Proben
`jede_appkit_datei_mit_frameworkimport_traegt_den_untergrenzen_abschnitt` und
`jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt`.

Dazu 27 ergänzte Modulköpfe unter `crates/krk-ui/src/appkit/`, jede Angabe am Xcode-SDK
gelesen. Die Aufzählungen sind an ihrer eigenen schließenden Klammer gelesen, wie
`## Constraints` des Entscheids es verlangt.

## Was der Prüflauf nicht hält

Die Richtigkeit der Zahl. Er sieht, dass ein Name im Abschnitt steht, nicht ob die
macOS-Fassung daneben stimmt. Das steht an beiden Proben, an `frameworknamen` und in den
Probennamen selbst, die von `steht` sprechen und nicht von `stimmt`.

Die Wortsuche selbst hat unterwegs eine eigene Lücke gehabt: eine Belegangabe wie
`NSAlert.h:22` erfüllte sie für die Klasse `NSAlert`. Aufgefallen ist das an der
Gegenprobe, behoben ist es, und die zwei dadurch verdeckten Lücken in `betrachter.rs` und
`editor.rs` sind geschlossen
(`260907-2307_*_eine-wortsuche-ueber-den-untergrenzen-abschnitt-nahm-die-kopfzeilenangabe-ns-alert-h-als-nennung-von-ns-alert.md`).

Daneben zwei Formen, die er nicht sieht: eine eingerückte `use`-Zeile und einen voll
ausgeschriebenen Pfad im Rumpf. Beide sind gemessen und in
`260907-2258_*_deckt-der-untergrenzen-prueflauf-auch-eingerueckte-use-zeilen-und-voll-ausgeschriebene-pfade.md`
zur Entscheidung vorgelegt; die drei Namen, die dabei durchfielen, sind von Hand in
`editor.rs` nachgetragen, mit dem Vermerk, dass ein Mensch sie nachgetragen hat.

## Abnahme

`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` und
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — alle fünf grün.

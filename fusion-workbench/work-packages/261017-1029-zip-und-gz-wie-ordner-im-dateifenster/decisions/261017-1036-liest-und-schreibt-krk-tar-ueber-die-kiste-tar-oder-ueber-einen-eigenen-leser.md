# Liest und schreibt KRK tar über die Kiste `tar` oder über einen eigenen Leser und Schreiber?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261017-1029-zip-und-gz-wie-ordner-im-dateifenster.md, 261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md, 260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md

---

## Question

Der Nutzer verlangt neben Zip auch gz, und die Directive nennt `.tar.gz` und `.tgz`, „soweit ohne C-Bibliothek machbar“. Den gzip-Strom hat KRK schon: `flate2` steht mit dem reinen Rust-Verfahren `miniz_oxide` im Baum. Für das tar-Format darin steht nichts im Baum. Die Wahl ist projektweit, weil jede Kiste eine Begründung in der Wurzel-`Cargo.toml` und einen `cargo tree`-Beleg je Mac-Ziel schuldet. Sie muss vor dem ersten Schritt des Plans fallen, der Archive liest.

## Options

1. **Die Kiste `tar` ohne Vorgabemerkmale** (`default-features = false`, also ohne `xattr`), festgenagelt auf eine genaue Fassung.
   - Pros: Sie liest und schreibt ustar, die PAX-Köpfe und die GNU-Langnamen. Sie prüft die Kopfsumme und baut lange Namen beim Schreiben selbst. Nach ihrer Beschreibung bringt sie auf macOS allein `filetime` und `libc` mit, und beide stehen schon über `gix` und `objc2` im Baum (`inference:` aus der Kistenbeschreibung, belegt wird es in Schritt 1 mit `cargo tree` je Mac-Ziel). Ohne `xattr` kommt kein `rustix` herein.
   - Cons: Eine weitere fremde Kiste. Ihr Merkmal `xattr` muss ausgeschaltet bleiben, und die Zeile in der Wurzel-`Cargo.toml` muss das begründen.
2. **Ein eigener Leser und Schreiber in `krk-core`** für 512-Byte-Köpfe mit ustar, PAX (`path`, `linkpath`, `size`, `mtime`) und GNU-Langnamen (`L`, `K`).
   - Pros: Keine neue Kiste.
   - Cons: Ein Formatparser für fremde Dateien, den KRK selbst pflegt, mit den bekannten Fallen: Oktalfelder, Basis-256-Größen, Kopfsumme, PAX-Vorrang und das Auffüllen auf 512 Bytes. Jede Lücke ist ein Fehler an fremden Archiven, und jeden davon findet erst der Nutzer.
3. **Kein tar in diesem Paket**, nur `.zip` und `.gz` als einzelne Datei.
   - Pros: Am kleinsten.
   - Cons: Die Directive nennt `.tar.gz` und `.tgz` ausdrücklich, soweit ohne C machbar, und Möglichkeit 1 macht es ohne C machbar.

## Constraints

- Auf `aarch64-apple-darwin` und `x86_64-apple-darwin` kommt weder `cc` noch ein Paket mit einem Namen auf `-sys` in den Baum (`cargo tree --target <ziel> -e normal,build`).
- `#![deny(unsafe_code)]` an jeder Kistenwurzel bleibt; eine fremde Kiste darf intern `unsafe` tragen, KRKs Code nicht.

## Recommendation

Möglichkeit 1, mit der Fassung festgenagelt wie `gix` und mit der Begründung in der Wurzel-`Cargo.toml`. Bringt `cargo tree` in Schritt 1 entgegen der Kistenbeschreibung ein weiteres Paket mit, das `cc` oder einen Namen auf `-sys` trägt, gilt Möglichkeit 1 als nicht erfüllbar. Dann hält der Schritt an und legt die Frage mit dem Beleg neu vor; er weicht nicht still auf Möglichkeit 2 aus.

---
Implemented: Cargo.toml (Begruendung tar), crates/krk-core/src/archiv/verzeichnis.rs — Kiste tar ohne Vorgabemerkmale, kein cc und kein Paket mit Namen auf -sys

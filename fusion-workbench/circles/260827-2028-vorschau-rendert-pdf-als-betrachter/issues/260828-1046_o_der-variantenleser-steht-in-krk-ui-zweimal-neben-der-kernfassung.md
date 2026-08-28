Der Variantenleser steht in `krk-ui` zweimal neben der Kernfassung

---

Drei Stellen lesen die Varianten einer Aufzählung ohne Daten aus dem Quelltext, mit derselben Lesart („ab `pub enum X {`, bis `}`, ohne Leer-, Kommentar- und Attributzeilen, bis zum Komma"):

- `crates/krk-core/tests/gemeinsam/mod.rs:411`, `varianten_der_aufzaehlung`, die anerkannte Kernfassung.
- `crates/krk-ui/src/appkit/betrachter.rs:662-675`, `varianten` im Prüfmodul (Runde 20, Schritt 6), mit dem Kommentar, dass die Kernfassung diese Kiste nicht erreicht.
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs:484-494`, eingebettet in `jeder_wirkungsbereich_hat_einen_stellvertreter` (Runde 20, Schritt 2), mit demselben Kommentar.

Dass `krk-ui` die Kernfassung nicht erreicht, stimmt: die Kiste hat kein Bibliotheksziel, und `tests/gemeinsam` gehört den Probenzielen des Kerns (CLAUDE.md, „`krk-ui` hat kein Bibliotheksziel"). Dass innerhalb von `krk-ui` zwei Abschriften nebeneinander stehen, folgt daraus nicht: `crate::quellbaum` (`crates/krk-ui/src/quellbaum.rs`) ist das Modul, über das die Proben der Kiste den Quelltext lesen, und beide Stellen rufen dort schon `quelldateien`. Daneben steht in `betrachter.rs:678-682` ein `codezeilen`-Filter, den `vorschau.rs` in zwei Proben (`set_hidden_steht_in_dieser_datei_allein_in_flaeche_zeigen`, `der_betrachter_wird_allein_in_pdf_zeigen_gebaut`) als Schließung nachbaut.

Zwei Leser derselben Lesart driften: wer einen um Varianten mit Daten oder um `#[doc]`-Zeilen erweitert, erweitert den anderen nicht, und eine Probe sieht dann Werte, die die andere nicht sieht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Betroffen:** `crates/krk-ui/src/quellbaum.rs` (neue Funktion), `crates/krk-ui/src/appkit/betrachter.rs` (Prüfmodul), `crates/krk-ui/src/kommandos/zulaessigkeit.rs` (Prüfmodul); die Kernfassung bleibt
**Schwere:** Low (Probencode; kein Verhalten des Bündels)

Fix: `pub fn varianten(inhalt: &str, name: &str) -> Vec<String>` und `pub fn codezeilen(inhalt: &str) -> impl Iterator<Item = &str>` in `quellbaum.rs`, mit dem Hinweis auf die Kernfassung und darauf, dass beide dieselbe Lesart tragen sollen; die zwei Proben in `betrachter.rs` und `zulaessigkeit.rs` und die zwei Schließungen in `vorschau.rs` rufen sie. Eine Quellbaumprobe, die zählt, dass `pub enum` als Suchnadel in `krk-ui/src` allein in `quellbaum.rs` steht, hält die Zahl danach bei eins.

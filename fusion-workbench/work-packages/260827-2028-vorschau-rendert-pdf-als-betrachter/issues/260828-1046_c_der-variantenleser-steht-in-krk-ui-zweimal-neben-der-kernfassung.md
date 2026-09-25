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

---

## Abgleich 260908, und die Behebung

**Der Befund bestand unveraendert, und der Nebenbefund war groesser als beschrieben.** Beide
Abschriften des Variantenlesers standen da. Der `codezeilen`-Filter stand nicht zweimal,
sondern **neunmal**: einmal in `betrachter.rs`, **siebenmal** in `vorschau.rs` (nicht zwei,
wie der Datensatz sagt: `die_zwei_schalter_stehen_je_an_genau_einer_stelle_und_dort`,
`der_quellbezug_wird_an_genau_zwei_stellen_gesetzt`,
`die_abfangstelle_steht_im_baum_genau_einmal` mit zwei Stellen,
`die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal`,
`set_hidden_steht_in_dieser_datei_allein_in_flaeche_zeigen` und
`der_betrachter_wird_allein_in_pdf_zeigen_gebaut`) und einmal in `aufrufstellen` in
`quellbaum.rs` selbst.

Gebaut ist der Vorschlag des Datensatzes:

- **`pub(crate) fn varianten(inhalt: &str, name: &str) -> Vec<String>`** und
  **`pub(crate) fn codezeilen(inhalt: &str) -> impl Iterator<Item = &str>`** stehen in
  `crates/krk-ui/src/quellbaum.rs`. Beide tragen am Doc-Kommentar den Hinweis auf die
  anerkannte Kernfassung `varianten_der_aufzaehlung`, den Grund, aus dem sie diese Kiste
  nicht erreicht, und die Zusage, dass beide dieselbe Lesart tragen sollen. Die Nadel
  `pub enum` steht nach der Regel des Modulkopfs **zusammengesetzt** da.
- **Alle neun Stellen rufen sie**: `betrachter.rs` (Variantenleser und Filter),
  `kommandos/zulaessigkeit.rs` (Variantenleser), `vorschau.rs` (sieben Filterstellen) und
  `aufrufstellen` selbst. Die zwei Schliessungen in `vorschau.rs`, die vorher `codezeilen`
  hiessen, heissen jetzt `treffer` und zaehlen die Codezeilen mit der Nadel, statt den Filter
  nachzubauen.
- **Eine Quellbaumprobe haelt die Zahl bei eins**:
  `der_aufzaehlungsleser_steht_in_dieser_kiste_genau_einmal` sucht den **Gegenstand** — den
  Abbruch am schliessenden `}` einer Zeile — und nicht den Namen, benennt ihre zwei
  verbleibenden Blindheiten und nimmt die Kernfassung ausdruecklich aus.

Resolved: 260908 — `crates/krk-ui/src/quellbaum.rs` (`varianten`, `codezeilen`, die neue
Zaehlprobe), `crates/krk-ui/src/appkit/betrachter.rs`,
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, `crates/krk-ui/src/appkit/vorschau.rs`.

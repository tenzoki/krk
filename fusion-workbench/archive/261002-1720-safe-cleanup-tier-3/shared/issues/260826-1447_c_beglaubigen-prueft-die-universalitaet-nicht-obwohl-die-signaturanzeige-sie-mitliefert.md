`beglaubigen` prüft die Universalität nicht, obwohl die Signaturanzeige sie mitliefert
---
Der Nur-Beglaubigungsweg spricht vom „universellen" Bündel, prüft aber nur Developer-ID und `runtime`. Die Zeile `Format=… Mach-O universal (x86_64 arm64)` steht in derselben Ausgabe.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/beglaubigung.rs`

## Befund

`signaturstand_pruefen` (`beglaubigung.rs:231-268`) stellt zwei Fragen an die Ausgabe von `codesign --display --verbose=2`. Dieselbe Ausgabe trägt die Zeile `Format=app bundle with Mach-O universal (x86_64 arm64)` (`:431`, `:456`, `:480` in den aufgezeichneten Proben). Die Universalität wird nicht gefragt. `README.md:302-303` und der Modulkopf `:12-14` beschreiben das Bündel als universell.

Ein Bündel aus `cargo xtask bundle` mit `KRK_SIGN_IDENTITY=<Developer-ID>` ist nicht universell (`sign.rs:175-180`); mit `codesign --options runtime` von Hand nachsigniert bestünde es beide Fragen und ginge bei Apple ein. Konstruiert, aber billig.

## Abhilfe

Eine dritte Frage `traegt_beide_architekturen` nach dem Muster von `traegt_gehaertete_laufzeitumgebung` (`:299-306`), gegen `release::ARCHITEKTUREN`.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L3

---
Resolved: Die dritte Frage steht, gebaut nach dem Muster von
`traegt_gehaertete_laufzeitumgebung`.

`traegt_beide_architekturen` (`xtask/src/beglaubigung.rs`) liest die Klammer der Zeile `Format=`
und hält jeden Namen aus `release::ARCHITEKTUREN` als ganzes Wort dagegen. Die Konstante ist dafür
`pub(crate)` geworden; eine zweite Namensliste in `beglaubigung.rs` wäre die zweite Wahrheit
darüber, welche Architekturen KRK trägt.

**Gefragt sind die Namen und nicht das Wort `universal`.** Eine dünne Binärdatei nennt genau eine
Architektur, gleich wie `codesign` die Bauform benennt; die Namensliste ist damit die schärfere
Frage und hängt nicht an einem Wortlaut, den ein späteres `codesign` ändern könnte. Steht hinter
`Format=` keine Klammer, gilt das Bündel als nicht universell — dieselbe Richtung wie bei der
gehärteten Laufzeitumgebung, weil ein Raten zur bequemen Seite eine Einreichung auf gut Glück
wäre.

Drei Proben halten es:

- `ein_duennes_buendel_haelt_die_beglaubigung_an` gegen die neue Aufzeichnung `NUR_ARM` — aus
  `OHNE_HAERTUNG` abgeleitet, mit gesetzter Härtung und `Format=app bundle with Mach-O thin
  (arm64)`, also **allein** ohne Universalität. Genau der im Befund beschriebene Fall. Sie prüft
  auch, dass die Meldung die zwei anderen Befunde nicht zu Unrecht mitführt.
- `die_bauform_wird_an_den_architekturnamen_gemessen` über vier Fälle: die aufgezeichnete Anzeige
  des ausgelieferten Bündels, die dünne, eine `Format=`-Zeile ohne Klammer und eine leere Anzeige.
- `das_ausgelieferte_buendel_ist_beglaubigungsfaehig` läuft unverändert grün; die aufgezeichnete
  Anzeige vom 260820 trägt beide Architekturen.

Nachgezogen sind die Prosastellen in der Grenze dieses Durchgangs: der Modulkopf von
`beglaubigung.rs` (dritter Punkt, ohne Zahl — welche Fragen es sind, sagt `signaturstand_pruefen`),
der Doc-Kommentar von `signaturstand_pruefen` selbst, die Erfolgsmeldung in `ausfuehren` und der
Hilfetext in `xtask/src/main.rs`.

Die `README.md` führt unter „Nur beglaubigen" weiter „Geprüft wird zweierlei" mit einer
zweizeiligen Tabelle und lag ausserhalb der Grenze; der Nachtrag ist als
`260906-0008_o_readme-und-claude-md-fuehren-station-1-mit-drei-bedingungen-und-die-beglaubigung-mit-zwei-pruefungen.md`
abgelegt.

Geprüft: `cargo test -p xtask` (164 Proben, Exit 0), `cargo clippy -p xtask -p krk-bench
--all-targets -- -D warnings` (Exit 0), `cargo fmt -p xtask -p krk-bench -- --check` (Exit 0).
Baumstand `ba0c6bd`.

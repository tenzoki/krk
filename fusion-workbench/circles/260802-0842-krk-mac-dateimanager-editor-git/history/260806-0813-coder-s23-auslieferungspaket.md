# S23: Auslieferungspaket (`cargo xtask release`)

**Agent:** coder
**Datum:** 260806-0813
**Status:** Complete
**Auftrag:** Planschritt `#### 23.` aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

## Was umgesetzt wurde

- `xtask/src/release.rs` (neu): der Unterbefehl `cargo xtask release` in sechs
  Stationen — AppKit-Grenzprüfung, Übersetzung beider Ziele, `lipo -create`
  mit sofortiger `-archs`-Prüfung, Bündelmontage über `bundle::Vorlage`,
  Signierung mit gehärteter Laufzeitumgebung, Beglaubigung über
  `xcrun notarytool submit --wait` und `xcrun stapler staple`. Die
  Voraussetzungen der Beglaubigung (vollständiges Xcode, Schlüsselbundprofil
  in `KRK_NOTARY_PROFILE`) werden bewusst erst nach der Signierung geprüft,
  damit das signierte Bündel bei einem Abbruch liegen bleibt — der Abnahmeweg,
  den der Plan für ein Gerät ohne Entwicklerkonto vorschreibt.
- `xtask/src/bundle.rs`: Montage von Übersetzung und Signierung getrennt
  (`Vorlage`, `vorbereiten`, `zusammensetzen`), `uebersetzen` um ein
  optionales Ziel-Tripel erweitert, `zielpfad` als eine Herleitung des
  Cargo-Ausgabepfads. `bauen()` (für `bundle` und `messen`) verhält sich
  unverändert; ein zweiter Bündelbauer ist nicht entstanden.
- `xtask/src/sign.rs`: `bestimmen_fuer_release()` — dieselben drei Stufen,
  die zweite sucht nach dem Namensanfang `Developer ID Application` statt
  nach `KRK Entwicklung`; `signieren_gehaertet()` signiert mit
  `--options runtime --timestamp`. Die Suche und `signieren` teilen sich die
  vorhandenen Hilfsfunktionen.
- `xtask/src/main.rs`: `mod release;`, Unterbefehlsauswahl, Hilfetext.
- `README.md`: neuer Abschnitt `## Auslieferung` mit den sechs Stationen,
  der Grenzprüfungs-Vorschrift, `store-credentials` und dem Verhalten ohne
  Entwicklerkonto.
- Die AppKit-Grenzprüfung läuft in Rust, zeilenanfangsverankert wie die
  Vorschrift (`^[[:space:]]*use +objc2`); die sechs Modulkommentare der Form
  "keine `use objc2`-Zeile" fallen durch, per Unit-Test belegt.

## Abnahme

- `lipo -archs target/KRK.app/Contents/MacOS/krk` → `x86_64 arm64`.
- `codesign -dv --verbose=4 target/KRK.app` → `flags=0x10000(runtime)`,
  gesicherter Zeitstempel, `codesign --verify --strict` Rückgabewert 0.
- Grenzprüfung: das Grep aus dem Abnahmekriterium gibt keine Zeile aus
  (Rückgabewert 1 der Pipe); der Lauf von `cargo xtask release` meldet
  dasselbe.
- Beglaubigungsteil: bricht auf diesem Gerät benennend ab — `notarytool` und
  `stapler` sind vorhanden (vollständiges Xcode installiert), es fehlt das
  Schlüsselbundprofil des Entwicklerkontos; die Meldung nennt
  `KRK_NOTARY_PROFILE`, das `store-credentials`-Kommando und den Ablageort
  des signierten Bündels. Damit ist der Schritt nach der Sonderregel des
  Plans abgenommen.
- Signiert wurde über Stufe 3 mit `Apple Development: Kai Stalmann`
  (einzige gültige Identität, keine Developer-ID vorhanden); der Lauf sagt
  per Hinweis dazu, dass die Beglaubigung diese Signatur nicht annähme.
- `make check`: alle vier Kommandos grün (build, test mit 29 xtask-Tests,
  clippy `-D warnings`, fmt).

## Defekt 260803-1530

`issues/260803-1530_c_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md`
trägt bereits `_c_` samt `Resolved:`-Notiz (Nutzerentscheidung vom
260805-0000, Einarbeitung in den Plan). Die Umsetzung in Code ist mit diesem
Schritt erfolgt; am Marker war nichts zu tun.

## Nicht angefasst

Plan-Marker von S23 ([IN PROGRESS] → [DONE]) und Commits: laut Auftrag Sache
des Orchestrators. Kein Makefile-Ziel `release` angelegt, weil die
Dateiliste des Planschritts das Makefile nicht nennt.

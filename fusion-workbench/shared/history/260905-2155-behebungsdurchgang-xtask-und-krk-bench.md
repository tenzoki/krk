# Behebungsdurchgang: achtzehn Defekte in `xtask/` und `crates/krk-bench/`

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Datum:** 260905-2155
**Baumstand bei Beginn:** `28c4a47`
**Grenze:** ausschließlich `xtask/` und `crates/krk-bench/`; zwei weitere Agenten arbeiteten
zeitgleich in `crates/krk-core/` und `crates/krk-ui/`.

## Verifikation

```
cargo test -p xtask -p krk-bench            → exit 0 (161 + 65 Proben)
cargo clippy -p xtask -p krk-bench --all-targets -- -D warnings → exit 0
cargo fmt -p xtask -p krk-bench -- --check  → exit 0
```

`make check` ist bewusst nicht gefahren: es prüft den ganzen Arbeitsbereich und wäre an den
Dateien der parallel laufenden Agenten abgebrochen. Keine Auslieferung angestoßen, kein
`cargo xtask bundle`.

## Geschlossen (sechzehn)

In `xtask/`:

- `260826-1451` — `#[must_use]` an 31 reinen Antworten in `git.rs`, `sign.rs`, `bundle.rs`,
  `release.rs`, `version.rs`.
- `260826-1450` — `#![deny(unsafe_code)]` an `xtask/src/main.rs`.
- `260826-1453` — die zwei Prüfhelfer mit dem absoluten Pfad des Referenzgeräts sind durch
  `bundle::pruefbuendel()` ersetzt.
- `260815-1715` — der Aufrufkommentar in `main.rs` sagt jetzt „Name" statt „Art".
- `260815-1716` — die Zusicherung läuft gegen `crate::HILFE` statt gegen `include_str!("main.rs")`;
  der Kommentar sagt, was er hält.
- `260826-1448` — die zwei Prosastellen zu `iconutil` sagen, was der Aufruf tut; `messen.rs` ruft
  `bundle::cargo()`.
- `260826-1446` — `KRK_SIGN_IDENTITY=-` wird benennend abgewiesen.
- `260826-1442` — beide späten Abbruchzweige der Station 7 nennen `./certify-only.sh <zahl>`.
- `260826-1452` — `sammeln` bricht bei einem Lesefehler ab und lässt Werkbank und `spikes/` außen
  vor.

In `crates/krk-bench/`:

- `260826-1304` — `rate_ueber_runden` hält die je Runde gemeldete Bildwiederholrate gegeneinander.
- `260826-1303` — der Perzentil-Zweig trägt dieselbe Wache wie der Anteils-Zweig.
- `260826-1308` — `Zeitmarkenwaechter` in `fixture.rs`, Bauform des `Messplanwaechter`.
- `260826-1307` — alle drei Berichtsschreiber gehen über `bericht::ohne_ueberschreiben`.
- `260826-1309` — die Probe nimmt einen `Wegwerfordner` statt eines festen Namens im echten
  Temporärverzeichnis.
- `260826-2153` — die Abhilfe nennt den Startwert aus dem Steckbrief.
- `260826-2154` — `Durchstich::fahren` hält seine zwei Prüfordner vorab.

## Offen geblieben (zwei), mit Teilarbeit

- `260826-1305` (`#[must_use]` in `krk-bench`). Umgesetzt sind der `Sitzungswaechter` und die zwei
  Gate-Urteile `Durchstichergebnis::bestanden` und `Gesamtergebnis::bestanden`. Der
  `Wegwerfordner` verlangt nach dem Wortlaut des Datensatzes eine Entscheidung über alle drei
  Prüfordner-Fassungen, und zwei davon liegen in `krk-core` und `krk-ui`. Als Entscheidungsfrage
  abgelegt:
  `260905-2155_*_bekommen-die-drei-pruefordner-fassungen-must-use-oder-keine.md`.
- `260826-2155` (Prüfordner B und der L6-Unterordner). Der falsche Kommentar in `bericht.rs` ist
  berichtigt: B wird von der Anwendung gelesen, von `krk-bench` nicht. Welcher der zwei Wege des
  Datensatzes gilt, nennt er selbst eine Nutzerentscheidung; als solche abgelegt:
  `260905-2155_*_bekommen-pruefordner-b-und-der-l6-unterordner-die-zweite-haelfte-der-deckung.md`.

## Neue Datensätze

- `260905-2155_*_claude-md-nennt-drei-kisten-mit-deny-unsafe-code-seit-heute-tragen-es-vier.md`
  (Defekt; die zweite Hälfte der Abhilfe von `260826-1450`, außerhalb der Grenze dieses Durchgangs)
- `260905-2155_*_bekommen-die-drei-pruefordner-fassungen-must-use-oder-keine.md` (Entscheidung)
- `260905-2155_*_bekommen-pruefordner-b-und-der-l6-unterordner-die-zweite-haelfte-der-deckung.md`
  (Entscheidung)

## Zahlen: ersetzt, verankert, berichtigt

Die tragende Vorgabe war, eine Zahl möglichst durch den Zeiger auf die Stelle zu ersetzen, die die
Auskunft trägt, sonst durch eine Probe zu verankern.

**Ersetzt** (Zahl weg, Zeiger hin): der Doc-Kommentar von `bundle::cargo()` sagte „Beide inneren
Aufrufe lesen ihn hier" und nennt jetzt die Regel plus `grep -rn 'bundle::cargo()' xtask/src`. Die
zwei Doc-Kommentare der `Durchstich`-Felder nannten „10.000" und „100.000" und zeigen jetzt auf
`EINTRAEGE_A` und `EINTRAEGE_GROSS`.

**Verankert** (Zahl bleibt, eine Probe wird rot, wenn der Baum weiterzieht):
`die_umgebungsvariable_cargo_wird_an_genau_einer_stelle_gelesen` (`bundle.rs`) hält die eine
Lesestelle von `CARGO`; das war die Zusage, die `messen.rs` unbemerkt gebrochen hatte.
`jeder_abbruch_der_station_sieben_nennt_die_wiederaufnahme` hält alle drei Abbruchzweige.

**Berichtigt ohne Verankerung** — eine Stelle: der Kommentar über Prüfordner B in
`bericht.rs`. Er ist Prosa über das Zusammenspiel von Messplan und Anwendung; eine Probe könnte
höchstens messen, dass der Messplan B als Tab in beide Dateifenster schreibt, und das ist bereits
in `plan_in_verzeichnis_schreiben` und seiner Probe belegt. Der Satz selbst bleibt unverankert.

## Was nicht angefasst wurde

- `260821-1221` (Suchpfad-Regel für fremde Werkzeuge) — der `iconutil`-Befund läuft hinein; die
  Prosa ist auf den Ist-Zustand gestellt, ohne die allgemeine Regel zu entscheiden.
- `260826-1302` (die vierte Prüfordner-Fassung `Wegwerfwurzel` in `xtask/src/release.rs`) — offene
  Nutzerfrage, nicht berührt.
- `CLAUDE.md`, `README.md`, die Wurzel-`Cargo.toml`, `crates/krk-core/`, `crates/krk-ui/` — außerhalb
  der Grenze.

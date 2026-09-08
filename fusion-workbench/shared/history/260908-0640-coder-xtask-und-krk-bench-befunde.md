# Sechs Befunde der Bahn `xtask`/`krk-bench` geschlossen, zwei Fragen gestellt

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Auftrag

Q5: die offenen Defektdatensätze, deren Behebung in `xtask/` oder `crates/krk-bench/` landet
und weder `crates/krk-core/` noch `crates/krk-ui/` anfasst. Drei weitere Bahnen liefen
gleichzeitig an den beiden anderen Kisten. HEAD bei Beginn `fe5fe9c`.

## Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

189 offene Defektdatensätze im ganzen Bestand. Davon nennen 17 einen Pfad unter `xtask/src`,
`crates/krk-bench`, `release.sh` oder `certify-only.sh`
(`grep -lE 'xtask/src|crates/krk-bench|release\.sh|certify-only\.sh'`); ein weiterer Durchlauf
über das weitere Muster `xtask|krk-bench|Makefile` brachte zehn zusätzliche Namen, alle mit
einer Behebung in `crates/`. Jeder der 17 ist einzeln gegen den heutigen Baum gelesen worden.

## Die zehn Zusagen

`grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u` liefert vorher und nachher
zeichengleich `"L1" "L10" "L2" "L3" "L4" "L5" "L6" "L7" "L8" "L9"`. Keine elfte Zahl, keine
geänderte Endbedingung; `crates/krk-bench/` ist in diesem Durchgang nicht angefasst worden.

## Behoben und geschlossen

- **`260815-1447`** — `jedes_ziel_tripel_bekommt_einen_namen_aus_den_architekturen`
  (`xtask/src/release.rs`) fragt nach der Stelle statt nach der Mitgliedschaft:
  `ZIELE.into_iter().zip(ARCHITEKTUREN)` und `assert_eq!`. Fängt jedes Vertauschen an jeder
  Stelle.
- **`260815-1446`** — der Zweigkörper des Unterbefehls `bundle` steht als benannte Funktion
  `bundle_fahren` (`xtask/src/main.rs`); `allein_der_unterbefehl_bundle_gibt_den_hinweis_aus`
  (`xtask/src/sign.rs`) läuft über jede `.rs`-Datei unter `xtask/src` statt über drei benannte
  und hält, dass die eine Fundstelle im Rumpf dieser Funktion liegt.
- **`260821-2105`** — die Meldung des gescheiterten Anlegens ist als reine Funktion
  `anlegen_gescheitert_meldung` (`xtask/src/veroeffentlichung.rs`) herausgezogen und trennt
  ihre zwei Lagen: vorübergehend gegen stehend, je mit ihrem Handgriff. Die Maschine
  entscheidet zwischen ihnen bewusst nicht — der Rückgabewert trennt sie nicht, und der
  Wortlaut von `gh` ist in diesem Modul keine zulässige Grundlage. Mitbehoben ist der Nachtrag
  vom 260826-1440: keine Meldung sagt mehr „Derselbe Aufruf", alle nennen
  `cargo xtask veroeffentlichen <zahl>`.
- **`260906-0008`** — `README.md` nennt die vierte Frage von Station 1 und stellt die
  Beglaubigungstabelle auf die Frage um, die der Code führt, statt auf eine Zahl von
  Teilfragen. `CLAUDE.md` ist Zeile für Zeile gegen den Baum gehalten und bleibt unverändert
  richtig.
- **`260821-1221`** — ohne Codeänderung geschlossen: `grep -rn "sieben Stationen"` über den
  Quellbaum liefert keinen Treffer, und
  `der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` hält es.
- **`260907-2350`** — beide genannten Datensätze tragen einen Nachtrag, der die L7-Zeile für
  den Ordnersprung neben L4, L5 und L6 stellt und für die volle Liste auf
  `DECKUNG_DER_ORDNER` (`crates/krk-bench/src/bericht.rs`) zeigt.

## Offen gelassen, mit Grund

- **`260823-1439`** — die zwei Zeilenzitate in `xtask/` sind auf Datei plus Name gezogen (beide
  waren falsch, die drei Zahlen des Nachtrags 260826-1440 inzwischen ebenfalls). Die drei
  übrigen Zitate stehen in `krk-core` und `krk-ui`, außerhalb dieser Bahn. Nachtrag im
  Datensatz.
- **`260813-0026`** — verlangt eine Wahl zwischen drei Zuschnitten, die alle ändern, was ein
  gewöhnlicher Tastendruck tut. Nicht geraten; als Frage abgelegt.
- **`260823-1210`** — ein einzelner roter `make check` ohne erhaltene Ausgabe ist nicht
  diagnostizierbar; zu tun bleibt, den nächsten mit Ausgabe festzuhalten.
- **`260823-1651`** — Zielkollision zwischen „der Tag steht auf HEAD" und „jede Sitzung
  schreibt ihr Ende"; drei Wege, keiner gewählt, und die Wahl ist keine Codefrage dieser Bahn.

Außerhalb der Bahn und deshalb unangetastet: die zwei Befunde an der Spannenstrecke und an
`Messplan::ordner_a` (beide `crates/krk-ui/src/messmodus.rs`), die fehlende Messstelle der
Syntaxhervorhebung, die zwei Datensätze zur vierten `Wegwerfordner`-Fassung, die C4.6-Nadel,
`Abschluss::ist_abgebrochen`, die flight-Profil-Zahlen und das `deny(unsafe_code)` der
Probenziele — alle mit einer Behebung unter `crates/`.

## Neu abgelegt

- `260908-0640_*_wie-wird-das-beglaubigte-buendel-vor-dem-naechsten-entwicklungsbau-geschuetzt.md`
  (Entscheidung) — die Wahl aus `260813-0026`, mit einer Korrektur: Zuschnitt 2 braucht kein
  `xcrun stapler validate` und damit kein Netz mehr, seit `traegt_angeheftetes_ticket` da ist.
- `260908-0645_*_die-vorabfrage-an-station-1-meldet-bei-verschwundenem-gh-gepackt-und-geschoben-obwohl-nichts-geschehen-ist.md`
  (Defekt) — beim Beheben von `260821-2105` gefunden: `release_steht` hat zwei Rufer mit
  verschiedenem Stand des Laufs und gibt beiden die späte Meldung.

## Abnahme

`cargo build --workspace` grün. `cargo test -p xtask`: 170 grün, ein Fehlschlag aus fremder
Bahn — `werkbank::tests::jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk`
nennt `260907-2046_c_…`, einen Datensatz eines gleichzeitig laufenden Kuratorendurchgangs ohne
Zeile `Resolved:`. `cargo test -p krk-bench`: 67 grün. `cargo clippy -p xtask --all-targets --
-D warnings` ohne Befund. `cargo fmt --all --check` meldet allein Stellen unter
`crates/krk-ui/`, keine in dieser Bahn. `RUSTDOCFLAGS="-D warnings" cargo doc -p xtask
--no-deps` grün.

`cargo xtask bundle` ist nicht gefahren: es verlangt eine Signaturidentität, und dieser
Durchgang ändert am Bündelbau nichts.

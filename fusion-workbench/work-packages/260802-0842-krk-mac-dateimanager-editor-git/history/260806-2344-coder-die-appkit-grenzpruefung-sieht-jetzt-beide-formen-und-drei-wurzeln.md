# Die AppKit-Grenzprüfung sieht jetzt beide Formen und drei Wurzeln — 260806-2344

**Absender:** coder
**Domäne:** code
**Auftrag:** D3 aus Orchestrator-Turn 25
**Defekt:** `issues/260806-1333_c_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md`
**Codestand vor der Umsetzung:** `181ff50`
**Status:** Complete

---

## Ergebnis in einem Satz

Die Grenzprüfung in `xtask/src/release.rs` erkennt neben der `use`-Zeile jetzt auch den ausgeschriebenen `objc2::`-Pfad und begeht alle drei Quellwurzeln des Workspace; sie läuft am echten Baum grün, und `make check` ebenso.

---

## Was geändert wurde

| Stelle | Was |
|---|---|
| `xtask/src/release.rs:5-14` | Station 1 im Modulkopf: beide Formen, drei Quellwurzeln, zweiter Defektverweis |
| `xtask/src/release.rs:50-63` | neue Liste `GRENZWURZELN` — drei Paare aus Quellwurzel und ausgenommenem Teilbaum |
| `xtask/src/release.rs:105-145` | `appkit_grenze_pruefen` läuft über die Liste statt über eine fest verdrahtete Wurzel; beide Meldungen nennen jetzt beide Formen |
| `xtask/src/release.rs:147-183` | `dateien_pruefen` nimmt `Option<&Path>` als Ausnahme und fragt `verletzt_grenze` statt `ist_objc2_use` |
| `xtask/src/release.rs:185-213` | `verletzt_grenze` samt der Begründung der Kommentarregel |
| `xtask/src/release.rs:215-252` | `nennt_objc2_pfad` und `ist_bezeichnerzeichen` |
| `xtask/src/release.rs:562-640` | vier neue Prüfungen |
| `xtask/src/bundle.rs:187` | `wurzel()` ist `pub(crate)`, damit die Baumprüfung die Projektwurzel nicht ein zweites Mal ableitet |

`ist_objc2_use` ist unverändert. Der Plan: `planning/260802-1428_o_plan-navigator-geruest-runde-1.md:1151` (die falsch gewordene Zusage „ohne `use`-Zeile kommt keine Bindung zustande" ist ersetzt), `:1153` (neuer Absatz zum Defekt), `:1154` (Abnahmekriterium: beide Greps, drei Wurzeln, `cargo test -p xtask`). Marker der Plandatei bleibt `_o_`.

---

## Die Entscheidung zum Kommentarfall

**Kein Zustandsautomat. Die Regel lautet: eine Zeile, deren erstes nicht-leeres Zeichen ein `/` ist, wird nicht gelesen.** Drei Gründe, alle nachgesehen statt vermutet.

Erstens treffen die zwölf Kommentarzeilen des Baums, auf denen die Prüfung nicht anschlagen darf, sämtlich diese Form: sie stehen als `//!` in Spalte 1. Keine steht hinter Code, keine ist eingerückt.

Zweitens gibt es in `crates/` keinen einzigen Blockkommentar (`grep -rn '/\*' crates --include='*.rs'` liefert nichts). Ein Automat für `/* */` wäre Code gegen einen Fall, den es nicht gibt.

Drittens fällt die verbleibende Lücke — ein nachgestellter Kommentar hinter Code, der `objc2::` nennt — zur sicheren Seite: sie meldet einen Verstoß zu viel, nicht einen zu wenig, und ein Umformulieren räumt sie aus. Ein halber Rust-Zerteiler könnte umgekehrt scheitern, und dann schweigt das Tor. Die Begründung steht am Programmtext über `verletzt_grenze`.

**`crates/krk-core/src` ist dazugekommen.** Der Defekt stellte es frei. Zwei Gründe dafür: die Prüfung kostet einen Verzeichnisdurchgang über sechzehn Dateien, und die Grenze ist danach an einer Stelle lesbar statt über zwei Abnahmekriterien verteilt. Das Kriterium von S15 bleibt daneben stehen und belegt dasselbe ein zweites Mal über die Abhängigkeiten der Kiste; ein zweiter Träger für eine Zusage, die `#![deny(unsafe_code)]` gerade nicht trägt, ist kein Schaden.

---

## Die Prüfungen

`ein_ausgeschriebener_objc2_pfad_ist_ein_verstoss` — die erste Lücke. Der erste Prüffall ist `crates/krk-ui/src/appkit/anwendung.rs:575` wörtlich, also die Zeile, die der Defekt als Beleg nennt.

`die_kommentarzeilen_des_baums_sind_kein_verstoss` — die zwölf Kommentarzeilen wörtlich, zehn aus `krk-ui` außerhalb von `appkit/` und zwei aus `krk-core`, dazu zwei erfundene Kommentare, die den Pfad ausschreiben und die die zwölf heute noch nicht abdecken.

`zeilen_ohne_objc2_sind_kein_verstoss` — die Abgrenzung, darunter `meinobjc2::rufen()`, ein fremder Name, der nur auf `objc2` endet.

`die_grenzpruefung_laeuft_am_baum_gruen` — die zweite Lücke und zugleich die Abnahme: `appkit_grenze_pruefen` läuft über den echten Baum. Das ist eine Zutat über den Auftrag hinaus und begründet: die Prüfung hing bisher allein an `cargo xtask release`, und das verlangt eine Signaturidentität und zwei Übersetzungsläufe. So läuft sie bei jedem `make check` mit und meldet einen Verstoß am Tag, an dem er entsteht, statt am Tag der Auslieferung.

**Gegenprobe, dass die neue Wurzel wirklich begangen wird.** Eine Zeile `fn probe() { objc2::rc::Weak::from_retained(); }` in `crates/krk-bench/src/bericht.rs` lässt `die_grenzpruefung_laeuft_am_baum_gruen` scheitern, und die Meldung nennt die Datei. Danach zurückgenommen; der Arbeitsbaum trägt sie nicht.

---

## Abnahme

`make check` grün — `cargo build --workspace`, `cargo test --workspace` (501 Prüfungen, davon 34 in `xtask`), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`. Die Ausgabe endet mit „alle vier gruen".

Die Prüfung am echten Baum: `release::tests::die_grenzpruefung_laeuft_am_baum_gruen ... ok`.

Die beiden Greps aus dem neuen Abnahmekriterium, je über `crates/krk-ui/src crates/krk-core/src crates/krk-bench/src` mit `| grep -v '^crates/krk-ui/src/appkit/'`, geben beide keine Zeile aus. Die Gegenprobe ohne den Ausschluss findet die Pfad-Form in `appkit/zwischenablage.rs`, `appkit/terminal.rs` und `appkit/volumes.rs` — die Suche greift also, sie hat nur außerhalb nichts zu finden.

---

## Was dabei auffiel und nicht zum Auftrag gehört

**Das Abnahmekriterium und der Code sind zwei Formulierungen derselben Vorschrift, und sie sind nicht Zeichen für Zeichen dasselbe.** Der Grep der Pfad-Form (`'^[[:space:]]*[^/[:space:]].*[^[:alnum:]_]objc2[[:alnum:]_]*::'`) verlangt vor `objc2` ein Zeichen, das kein Bezeichnerzeichen ist, und übersieht deshalb den Fall, dass die Zeile ohne jede Einrückung mit `objc2::` beginnt. Der Rust-Code fängt ihn. In Rust steht ein solcher Ausdruck immer in einem Funktionsrumpf und damit eingerückt, und unverankert wäre der Grep wieder in der Kommentarfalle. Verbindlich ist der Code; der Grep ist die von Hand nachvollziehbare Fassung. Kein Defekt gemeldet, weil die schwächere Fassung nur mehr durchlässt, nicht mehr meldet.

**Der Defektdatensatz lag als `_p_` und war nie committet.** Die Umbenennung auf `_c_` lief deshalb über ein gewöhnliches `mv`; die `_o_`-Fassung steht weiterhin als `D` im Arbeitsbaum.

**`bundle::wurzel()` ist jetzt `pub(crate)`.** Die Alternative wäre gewesen, `env!("CARGO_MANIFEST_DIR")` im Prüfmodul von `release.rs` ein zweites Mal auszuwerten — dieselbe Ableitung an zwei Stellen. Der Sichtbarkeitswechsel ist die kleinere Änderung, führt aber dazu, dass die Projektwurzel jetzt aus jedem Modul von `xtask` erreichbar ist.

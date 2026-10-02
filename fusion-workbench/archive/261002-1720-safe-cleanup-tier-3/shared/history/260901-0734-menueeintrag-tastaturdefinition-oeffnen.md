# Der Menüeintrag „Tastaturdefinition öffnen"

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Datum:** 260901-0734
**Auftrag:** Nutzerauftrag nach dem Abschluss der Runde 23, kein Circle aktiv

---

## Was gebaut wurde

Ein Kommando `BelegungsdateiAnsehen` mit der Kennung `belegungsdatei_ansehen`, das
`~/Library/Application Support/KRK/keymap.toml` in das Vorschaufenster stellt, den Fokus
dorthin holt und beim Öffnen vor den zwei Schreibern der Datei warnt. Ohne Tastenkombination,
erreichbar über das Hauptmenü, wie die vier Spaltenschalter.

## Angefasste Dateien

- `crates/krk-core/src/tasten/belegung.rs` — Variante, Zeile in `KENNUNGEN` (85 → 86),
  Zweig in `Kommando::wirkungsbereich` (`Ueberall`)
- `crates/krk-core/tests/belegung.rs` — `OHNE_KOMBINATION_AB_WERK` (6 → 7),
  neue Probe `die_belegungsdatei_ist_aus_jedem_fokus_zu_bekommen`
- `crates/krk-ui/src/belegungsmodell.rs` — Zweig in `bereich_des_kommandos`
  (`Funktionsbereich::Anwendung`), neue Probe
  `die_belegungsdatei_steht_bei_der_anwendung_und_nicht_bei_der_vorschau`
- `crates/krk-ui/src/belegungsausgabe.rs` — die zweite, unabhängig geführte Liste der ab
  Werk unbelegten Funktionen
- `crates/krk-ui/src/kommandos/operationen.rs` — die drei Sätze
  `belegungsdatei_hat_zwei_schreiber`, `keine_belegungsdatei`,
  `belegungsdatei_ohne_ablageordner` und drei Proben dazu
- `crates/krk-ui/src/appkit/anwendung.rs` — `ablagepfad`, `belegungsdatei_ansehen`,
  Ausführungszweig in `kommando_ausfuehren`

## Die vier Pflichtstellen

Die drei aus `CLAUDE.md` und die vierte, die dort keine Pflichtstelle heißt:

1. `Kommando::wirkungsbereich` → `Wirkungsbereich::Ueberall`
2. `belegungsmodell::bereich_des_kommandos` → `Funktionsbereich::Anwendung`
3. `Kommando::KENNUNGEN` → `(Kommando::BelegungsdateiAnsehen, "belegungsdatei_ansehen")`,
   Längenangabe von 85 auf 86
4. Der Ausführungszweig in `Anwendungsdelegierter::kommando_ausfuehren`, vor dem
   Auffangzweig auf `bereichskommando`

## Die Entscheidungen und ihre Begründung

**`Wirkungsbereich::Ueberall` und nicht `Vorschau`.** Der Befehl *holt* die Vorschau hervor
und den Fokus hinein; ein Vorbehalt auf das Vorschaufenster verlangte genau den Zustand, den
er selbst herstellt, und der Nutzer bekäme seine Datei aus dem Dateifenster heraus nie zu
sehen. Dieselbe Erwägung trägt `Kommando::FokusVorschau`.

**`Funktionsbereich::Anwendung` und nicht `Vorschau`.** Die Gliederung fragt nach der Gegend
der Anwendung und nicht nach dem Mechanismus (Modulkopf von `belegungsmodell`). Wer seine
Tastaturdefinition sucht, sucht sie neben der Belegungsansicht und der Markdown-Ausgabe. Bei
`ZwischenablageAnsehen` fällt dieselbe Frage anders aus, weil die Zwischenablage keine eigene
Gegend hat.

**Der Fokus geht mit in die Vorschau.** `cmd+e` trägt `Wirkungsbereich::Dateibereiche` und
bedeutet in der Dateiliste den ausgewählten Eintrag, in der Vorschau die angezeigte Datei.
Ohne den Fokuswechsel öffnete der Rundweg die falsche Datei im Editor, und die Zusage des
Auftrags („so, dass er von dort mit `cmd+e` in den Editor wechseln kann") trüge nicht.
Gebaut über `fokus_holen`, den einen Weg jedes Fokusbefehls: einblenden, dann Fokus setzen.
Ausgeblendet wird nie — dafür bleibt `f3`.

**Drei Ausgänge statt zweier.** Ohne Ablageordner gibt es keinen Ort, an dem die Datei stehen
könnte; mit Ordner und ohne Datei ist es der erste Start, und sie entsteht mit der ersten
gesicherten Umbelegung. Beide antworten mit einem eigenen Satz und lassen die Vorschau
unberührt; dort stünde sonst „liess sich nicht lesen: No such file or directory", was die
Lage nennt und nicht erklärt. Ein `Err` von `try_exists` zählt zum dritten Ausgang: die
Vorschau antwortet dann auf ihrem Weg, statt dass diese Stelle behauptet, was sie nicht weiß.

**`ablagepfad` neben `unter_der_sperre`.** `Ablage::pfad` rechnet einen Pfad und liest nichts;
ein Durchgang dafür nähme die Schreibsperre und wartete auf dem Hauptfaden auf eine zweite
Instanz — für eine Zeichenkette.

## Der Hinweis auf die zwei Schreiber

Ein Satz in der Statuszeile, Rang `Befehlsantwort`, gesetzt beim Öffnen:

> keymap.toml hat zwei Schreiber: eine Änderung von Hand wirkt erst beim nächsten Start,
> und die Belegungsansicht (F1) überschreibt sie beim Verlassen

Der Dateiname kommt aus `Datei::Belegung` und steht in keinem der drei Sätze als
Zeichenkette. Die weitergehende Frage — Neuladen im Betrieb, Sperre, Warnung beim
Überschreiben — ist gefilt als
`260901-0734_*_haelt-krk-die-belegungsdatei-gegen-ihren-zweiten-schreiber-oder-bleibt-es-beim-hinweis.md`.

## Nicht angefasst

`resources/default-keymap.toml`. Der Eintrag dort ist der Schritt des `ontocoder` und kommt
danach. Er gehört hinter `belegung_ansehen` in den Abschnitt „C3: die Belegungsansicht",
mit `tasten = []` und dem Namen „Tastaturdefinition öffnen"; die Zeile `# Ausgeliefert sind
91 Funktionen …` im Dateikopf steigt dabei auf 92.

## Verifikation

`make check` → Exit 2. Vier rote Proben, alle vier durch den fehlenden Eintrag in
`resources/default-keymap.toml` erklärt:

- `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
- `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` (`krk-core`, `tests/belegung.rs`)
- `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
- `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`

**Gemessen und nicht abgeleitet:** mit einem probeweise eingetragenen `[[funktion]]`-Block
liefen alle 24 Prüfziele grün (`cargo test --workspace --no-fail-fast`), danach ist die Datei
aus einer Sicherung zurückgestellt und über `shasum -a 256` als bytegleich nachgewiesen
(`152d1d16a5150c092bdf063ccb82929afc99c8e49d7ed07df094e8830e565288`). Außer den vier
genannten ist nichts rot.

`cargo build --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all
--check` sind grün.

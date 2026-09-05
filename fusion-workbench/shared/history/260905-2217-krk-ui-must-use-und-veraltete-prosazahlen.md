# krk-ui: `#[must_use]` nachgezogen und veraltete Prosaangaben ersetzt

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baumstand:** HEAD `28c4a47`, Arbeitsbaum mit zwei weiteren Agenten in `krk-core`, `krk-bench` und `xtask`
**Grenze:** ausschliesslich `crates/krk-ui/`

## Auftrag

Einundzwanzig offene Defektdatensaetze aus `shared/issues/`, alle in `crates/krk-ui/`:
sechs zu fehlenden `#[must_use]`-Marken, fuenfzehn zu Zahlen- und Beschreibungsangaben in
Modulkoepfen und Doc-Kommentaren, die der Baum nicht mehr traegt.

## Verifikation

```
cargo test -p krk-ui                                   -> exit 0 (902 + 5 Proben)
cargo clippy -p krk-ui --all-targets -- -D warnings    -> exit 0
cargo fmt -p krk-ui -- --check                         -> exit 0
```

Daneben, als Gegenprobe fuer die Doc-Verweise:
`cargo doc -p krk-ui --no-deps --document-private-items` meldet 51 tote Verweise, dieselbe
Zahl wie vor der Sitzung; keiner der neu geschriebenen Verweise ist darunter.

## Gruppe A: die Marken

Alle sechs Datensaetze geschlossen. Der Lauf unter `-D warnings` ist dabei die eigentliche
Pruefung gewesen und hat neun Rufer gefunden, die die Antwort still fallen liessen:

- `messhandlung` im Messmodus, zweimal auf `DateifensterQuelle::kommando_ausfuehren` — genau
  die zwei Stellen, die `260826-1327` benannt hat. `let _ =` ist dort richtig, weil beide
  Kommandos im `true`-Schwanz von `kommando_ausfuehren` landen.
- `Leistenmodell::orte_setzen`, `Editorbereich::ansicht_umschalten` und die drei Tabzweige von
  `Vorschaufenster::kommando_ausfuehren`.
- 23 Aufrufe in Pruefmodulen (`editormodell.rs`, `leistenmodell.rs`, `tabs.rs`).

Zwei Marken sind wieder gefallen, weil `clippy::double_must_use` sie als Wiederholung einer
Marke am Rueckgabetyp zaehlt (`Tabliste::auswahl_auf_namen`, `Tabliste::einziehen`); zwei
weitere haben statt der nackten Marke einen Meldungstext bekommen
(`Blattgriff::abbruchweg`, `Regelfelder::regel`).

**Nicht angefasst:** `zwischenablage::text_schreiben` (gehoert zu `260820-0739`) und
`Blatt::zeigen` mit seinem `let _griff =` — die Bindung faengt die Marke ab, und was dort
fehlt, ist der Griff selbst, was `260826-1325_*_esc-im-stapel-umbenennen-blatt-…` fuehrt.

## Gruppe B: die Angaben

Fuenfzehn Datensaetze geschlossen. Der Zug war, wo moeglich, der von `CLAUDE.md`: statt die
Zahl zu berichtigen, das Zaehlkommando oder den Zeiger auf die Quelle hinschreiben. Das ist
bei jeder Stelle gelungen; **keine Zahl ist berichtigt und unverankert stehen geblieben.**

Zwei Befunde bestanden am heutigen Baum nicht mehr und sind ohne Codeaenderung geschlossen:
die drei „vier Bereiche" in `anwendung.rs` (der Nachzugsschritt der Runde 23 hat sie erledigt)
und die neun Ankreuzfelder in `appkit/mod.rs`. Ein dritter, der Modulkopf der Zwischenablage,
war am Code schon berichtigt; seine offene Haelfte ist eine Frage und haengt an
`decisions/260811-2050_*`, weshalb der Datensatz auf dem Weg geschlossen ist, den sein eigener
Abgleich vom 260819-0057 als den sauberen benennt.

Drei Befunde waren beim Nachmessen groesser oder anders, als der Datensatz sagte, und sind in
ihrer wirklichen Groesse behoben:

- `260826-1419` nennt zwei Module, die im Ueberblick von `appkit/mod.rs` fehlen; es sind
  fuenf (`abwurf`, `weitereinstanz`, `git`, `leiste`, `koordinaten`). Der Ueberblick ist jetzt
  als Zeichnung der Wertefluesse gekennzeichnet und nicht als Modulverzeichnis; `git` und
  `weitereinstanz` haben im Fliesstext dahinter einen eigenen Absatz bekommen.
- `260826-1418` beschreibt den zweiten Absatz als „falscher Mechanismus". Er war doppelt
  falsch: mit dem Fokus in der Leiste weist nicht der Fokusvorbehalt `alle_markieren` ab,
  sondern der Bestandteil (3) der Zulaessigkeitsregel, denn `Kommando::AlleMarkieren` traegt
  `Wirkungsbereich::Dateifenster` und eine `NSTableView` stellt dem Vorbehalt gar keinen
  Ersthelfer.
- `260831-1212` schlaegt als Ersatztext vor, die Probe
  `die_tafel_nennt_jeden_befehl_genau_einmal` halte die Vollstaendigkeit von
  `Kontextbefehl::ALLE`. Sie tut es nicht: sie laeuft ueber `ALLE` und haelt die Tafel
  dagegen. Der neue Kommentar nennt stattdessen die drei vollstaendigen
  Fallunterscheidungen, die der Uebersetzer haelt, sagt ausdruecklich, dass an `ALLE` selbst
  nichts haengt, und verweist fuer die Frage nach der Bauform auf `260826-1811_*`.

Eine echte Codeaenderung ist dabei mitgelaufen, weil `260826-1327` sie verlangt:
`Editorbereich::bauen` und `::schliessen` schreiben die drei Zeilen von `stand_erneuern` nicht
mehr von Hand hin, sondern rufen es.

## Neuer Datensatz

`260905-2217_o_cargo-doc-meldet-51-tote-verweise-in-krk-ui-und-make-check-faehrt-cargo-doc-nicht.md`.
Gefunden beim Beheben von `260826-1327_*_zwei-tote-verweise-…`, das zwei dieser Verweise
namentlich fuehrt; die Luecke im Abnahmelauf, die es nebenbei nennt, hatte keinen eigenen
Datensatz und ist um 49 Stellen groesser als jener vermuten laesst.

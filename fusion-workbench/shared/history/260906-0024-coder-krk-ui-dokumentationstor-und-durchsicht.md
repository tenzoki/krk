# Das Dokumentationstor von `krk-ui` und sieben Befunde der Durchsicht

**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `ba0c6bd`
**Umfang:** ausschliesslich `crates/krk-ui/` und `fusion-workbench/shared/issues/`

## Verification

```
cargo test   -p krk-ui                                  -> exit 0 (903 + 5 bestanden)
cargo clippy -p krk-ui --all-targets -- -D warnings     -> exit 0
cargo fmt    -p krk-ui -- --check                       -> exit 0
RUSTDOCFLAGS="-D warnings" cargo doc -p krk-ui --no-deps -> exit 0
```

## Aufgabe 1: das Dokumentationstor

**Vorher 85 Warnungen, nachher 0.** Gezaehlt mit

```sh
export PATH="$HOME/.cargo/bin:$PATH"
touch crates/krk-ui/src/main.rs
cargo doc -p krk-ui --no-deps 2>&1 | grep -c '^warning:'
```

(die Summenzeile abgezogen). Die 85 verteilten sich auf drei Arten und nicht
auf eine:

| Art | Zahl |
|---|---|
| `unresolved link to …` | 51 |
| `redundant explicit link target` | 30 |
| `… is both a function and a module` | 4 |

Die vierte Art, `public documentation for … links to private item`, kommt in
`krk-ui` **nicht** vor und kann es nicht: die Kiste hat kein Bibliotheksziel und
keine oeffentliche Flaeche, an der rustdoc die Meldung stellen koennte.

### Was die 51 unaufgeloesten Verweise wirklich waren

Nachgemessen und nicht angenommen: `cargo doc --document-private-items` loest
sie **nicht** auf. `crate::appkit::fsevents` ist von `crate::auffrischung` aus
kein gueltiger Pfad, weil `mod fsevents;` in `appkit/mod.rs` privat ist. Ein
`#![allow(rustdoc::private_intra_doc_links)]` haette in dieser Kiste keine
einzige Meldung gedeckt. Die Gruppe ist damit die, fuer die
`shared/decisions/260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`
Option 2 empfiehlt, und `appkit/mod.rs` schreibt kistenuebergreifende Verweise
seit jeher so: Backticks ohne eckige Klammern.

Behandlung nach Gruppe:

- **Privates Nachbarmodul** (`crate::appkit::…`, `super::appkit::menue`,
  `super::editor::textflaeche_bauen`): zu Fliesstext.
- **Probennamen** (`tests::…`, `Fokus::ALLE`, `formatieren`): zu Fliesstext,
  denn `#[cfg(test)]`-Code steht im Doc-Bau nicht.
- **Fremdname ohne Import** (`io::ErrorKind::AlreadyExists`, `fmt::Display`,
  `Kommando`, `Wirkungsbereich::beschriftung`, `Ordnermodell`, `Tabliste`):
  auf den vollen Pfad gesetzt, der Anzeigetext bleibt kurz.
- **`NSFontDescriptor`**: zu Fliesstext, wie derselbe Name an drei anderen
  Stellen derselben Datei.
- **Falscher Name** (`Self::sitzung_laden` an `AnwendungsIvars`,
  `DateifensterQuelle::auswahl_verschieben`,
  `crate::editormodell::EditorModell::bearbeiten`): auf den Namen gesetzt, den
  es gibt (`Anwendungsdelegierter::sitzung_laden`, `auswahl_bewegen`,
  `Editormodell`).
- **Ueberfluessiges Ziel**: `[`X`](X)` zu `[`X`]`, mechanisch ueber die
  Spaltenangabe der Meldung.
- **Mehrdeutig**: `fn@` davor; alle vier meinten die Funktion.
- **Verstuemmelt**: `spalten.rs` trug
  ``[`Kommando::SpalteMarkeUmschalten`]**(krk_core::tasten::Kommando)`` — das
  `**` der Fettschrift stand zwischen Text und Ziel.

## Aufgabe 2: sieben Befunde der Durchsicht vom 260826

Datei- statt datensatzweise gearbeitet, wie beauftragt.

1. `260826-1442_*_die-liste-der-gemaechlichen-arten-in-auffrischung-rs-…` —
   `die_gemaechlichen()` traegt fuenf statt drei Arten; neue Zaehlprobe
   `die_liste_der_gemaechlichen_deckt_jede_art_ausser_dem_stapel_umbenennen`
   liest die Varianten von `pub enum Art` aus dem Quelltext von `krk-core`.
   Die Probe ist gegen ihren Zweck geprueft: mit entferntem `Art::Entpacken`
   bricht sie ab.
2. `260826-1327_*_der-doc-kommentar-an-umbenennungbeendet-…` — Doc berichtigt.
3. `260826-1327_*_die-doc-der-abwurfmeldungs-tafel-…` — Doc-Block an seine
   Probe verschoben.
4. `260826-1442_*_die-fortschreibungsprobe-…` — `fall: &str` als fuenfter
   Parameter, `let _ = name;` gefallen, jeder Rufer bekommt einen Fallnamen.
5. `260826-1334_*_frei-zeigen-…` — `selectText(None)` in `frei_zeigen`.
6. `260826-1423_*_das-kuerzelzeichen-der-schaltflaechen-…` — neue Aufzaehlung
   `Schaltflaechenzeichen`; die Anzeigeform wird gerechnet statt daneben
   geschrieben, das Feld `anzeige` faellt.
7. `260826-1442_*_die-kachelungsprobe-…` — sechs Beispiele dazu (16 statt 10),
   und der Anspruch "Beweis der Totalitaet" zurueckgenommen.
8. `260826-1422_*_der-probenhelfer-liste-…` — `std::env::temp_dir()` in
   `tabs.rs` von 4 auf 0; neuer Bauplatz `vorhandene_ordner(namen)` ueber
   `crate::pruefordner::Pruefordner`.

## Zwei untertreibende Datensaetze, gemessen

- `260905-2217` nennt 51 tote Verweise. Richtig fuer sein Kommando
  (`grep -c 'unresolved link'`), aber `krk-ui` trug 85 Doc-Warnungen; die
  anderen 34 haetten das Tor genauso rot gehalten.
- `260826-1423` schreibt am Ende: "`Funktionsbereich` traegt heute neun Werte,
  die Zahl stimmt". Sie stimmte beim Filen nicht.
  `awk '/pub enum Funktionsbereich/,/^}/' crates/krk-ui/src/belegungsmodell.rs`
  liefert zehn; `Git` ist in der Git-Runde dazugekommen.
- `260826-1422` nennt zwei Helfer mit Griff ins echte Temporaerverzeichnis. Es
  waren vier.

## Zahlen, die stehen geblieben sind

- Die Laengenangabe von `KACHELBEISPIELE` (`[&str; 16]`) und von
  `SCHALTFLAECHEN` (`[Schaltflaechentaste; 3]`): der Uebersetzer haelt sie.
- `[Fokus; 6]`, `[Art; …]` und die uebrigen Feldlaengen: unberuehrt.
- Ersetzt sind: "drei Arten" in `auffrischung.rs` (durch die Zaehlprobe), "Zehn
  Faelle" in `markdown.rs` (durch den Zeiger auf die Feldlaenge), "neun
  Bereichsueberschriften" in `belegungsansicht.rs` (durch das `awk`-Kommando).

## Nicht angefasst

Die uebrigen offenen Datensaetze der Kampagne, die `krk-ui` zitieren, stehen
unveraendert auf `_o_`. Die Torfrage aus `260905-2217` — ob `cargo doc` als
fuenftes Kommando in `make check` kommt — ist eine Nutzerentscheidung und liegt
in `shared/decisions/260905-2336_*`.

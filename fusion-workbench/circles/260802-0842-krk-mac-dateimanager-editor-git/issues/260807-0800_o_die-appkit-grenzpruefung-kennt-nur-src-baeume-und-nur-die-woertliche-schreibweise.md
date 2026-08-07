Die AppKit-Grenzprüfung kennt nur die drei src-Bäume und nur die wörtliche Schreibweise objc2

---

`4db66ed` hat die Prüfung um die ausgeschriebene Pfadform und um zwei
Quellwurzeln erweitert. Zwei Wege, auf denen eine `objc2`-Kiste außerhalb von
`crates/krk-ui/src/appkit/` in den Bau kommt, sieht sie weiterhin nicht. Beide
sind heute nicht begangen; die Prüfung ist aber ein Tor, und ein Tor wird an
dem gemessen, was es durchlässt.

---

## Weg 1: eine Quelldatei außerhalb von `<kiste>/src`

`xtask/src/release.rs:57-61`:

```rust
const GRENZWURZELN: [(&str, Option<&str>); 3] = [
    ("crates/krk-ui/src", Some("appkit")),
    ("crates/krk-core/src", None),
    ("crates/krk-bench/src", None),
];
```

Cargo übersetzt je Kiste außer `src/` auch `tests/`, `benches/`, `examples/`
und `build.rs`. `crates/krk-ui/` hat davon heute keines (geprüft: dort liegen
nur `Cargo.toml` und `src`), und `crates/krk-core/tests/` gibt es, kann aber
keine `objc2`-Kiste übersetzen, weil `krk-core` sie nicht als Abhängigkeit
führt (`crates/krk-core/Cargo.toml`).

Für `krk-ui` gilt das nicht: die Kiste führt fünf `objc2`-Abhängigkeiten
(`crates/krk-ui/Cargo.toml:25-37`). Ein `crates/krk-ui/tests/…rs`, das eine
davon nennt, ginge grün durch die Prüfung, und die Meldung des Werkzeugs sagte
trotzdem "keine `objc2`-Kiste außerhalb von crates/krk-ui/src/appkit/".

## Weg 2: die Kiste unter einem anderen Namen

Beide Suchen (`ist_objc2_use` und `nennt_objc2_pfad`,
`xtask/src/release.rs:225-243` und `:270`) suchen die Zeichenfolge `objc2` im
Quelltext. Cargo lässt eine Abhängigkeit umbenennen:

```toml
appkit = { package = "objc2-app-kit", version = "…" }
```

Danach ist `use appkit::NSView;` gültiges Rust in jeder Datei von `krk-ui`, und
keine der beiden Suchen schlägt an. Dasselbe leistet `extern crate objc2 as ak;`
innerhalb einer Datei: kein `::` hinter dem Bezeichner, also kein Treffer in
`nennt_objc2_pfad`, und `ist_objc2_use` verlangt `use`.

Die `Cargo.toml`-Dateien liest die Prüfung nicht; sie geht nur über `.rs`
(`xtask/src/release.rs:172-174`).

## Was die Prüfung bereits richtig macht

Nachgeprüft und in Ordnung: die zwölf Kommentarzeilen des Baums, auf denen die
Prüfung nicht anschlagen darf, stehen alle als `//!` in Spalte 1 und fallen
durch die Kommentarregel (`grep -rn "objc2"` über die drei Quellwurzeln ohne
`appkit/` liefert genau diese zwölf). Blockkommentare gibt es unter `crates/`
keinen einzigen. Die Begründung im Kopf von `verletzt_grenze` hält also, soweit
sie reicht.

## Denkbarer Weg

Für Weg 1: `GRENZWURZELN` je Kiste um die übrigen Quellbäume ergänzen, die
Cargo übersetzt, und fehlende Bäume überspringen statt daran zu scheitern —
`dateien_pruefen` bricht heute mit `Abbruch::Lauf` ab, wenn ein Ordner nicht
lesbar ist (`xtask/src/release.rs:157-159`), ein nicht vorhandenes `tests/`
träfe also die Fehlerbahn.

Für Weg 2: eine Zeile je `Cargo.toml` des Workspace, die einen
`package = "objc2…"`-Eintrag unter fremdem Schlüssel meldet. Ob der Aufwand
lohnt, ist eine Abwägung gegen die Maxime "supersimpel"; er ist gering, und die
Prüfung läuft seit `4db66ed` bei jedem `make check` mit
(`die_grenzpruefung_laeuft_am_baum_gruen`).

## Dringlichkeit

Gering. Beide Wege sind heute nicht begangen, keine Kiste ist umbenannt, und
`crates/krk-ui/` hat keinen zweiten Quellbaum. Der Wert liegt darin, dass das
Tor auch dann hält, wenn jemand einen davon anlegt.

**Betrifft:** `xtask/src/release.rs`.

**Aufgefallen bei:** der inkrementellen Durchsicht nach Turn 25 der Sitzung
260806-2257, Diff `f9a0462..HEAD`, Commit `4db66ed`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1333_c_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-0834_c_die-appkit-grenzpruefung-uebersieht-pub-use-und-use-mit-fuehrendem-doppelpunkt.md`

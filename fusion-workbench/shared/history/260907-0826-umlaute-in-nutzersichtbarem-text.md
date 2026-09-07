# Nutzersichtbarer deutscher Text bekommt Umlaute

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Auftrag K4. Grundlage ist die Antwort des Nutzers vom 260907 in
`260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`:
Umlaute in allem, was ein Mensch liest; die Umschrift bleibt fuer Kommentare und Bezeichner.
Ausgangsstand `0115cf5`.

## Die Erhebung

Der vorbereitende Lauf schaetzte „rund fuenfzehn Zeichenketten". Die eigene Erhebung geht
ueber alle vier Kisten und arbeitet in drei Sieben, weil ein Wortmuster allein hier nichts
taugt: `traegt` ist im Doc-Kommentar richtig und in der Meldung falsch.

1. Ein Scanner ueber den Quelltext zieht **jedes Stringliteral** samt Zeilennummer heraus
   (`"…"` und `r#"…"#`, Zeilen- und Blockkommentare uebersprungen) und laesst die
   `#[cfg(test)]`-Module und `tests/` weg. Die Modulgrenze faellt an der Zeile, die genau `}`
   im Einzug des `mod` traegt — die erste Fassung zaehlte geschweifte Klammern und wurde von
   `format!("{}")` in die Irre gefuehrt, was 38 Stellen falsch ausschloss und 15 in
   `appkit/editor.rs` erst im zweiten Lauf sichtbar machte.
2. Ein Digraphfilter (`ae|oe|ue|ss` im Wortinneren) und danach eine von Hand durchgesehene
   Stammliste, die `neue`, `Quelle`, `Fassung`, `muss`, `dass` und ihresgleichen ausschliesst.
3. Eine Einordnung nach dem Konstrukt: `#[must_use]`, `assert`, `debug_assert`, `panic` und
   `expect` sind Diagnostik fuer den Entwickler und keine Meldung an den Nutzer.

Kommando (der Scanner steht in der Sitzung, nicht im Baum):

```sh
python3 scan.py --prod crates xtask | python3 umschrift.py | python3 tag.py
```

Ergebnis am Stand `0115cf5`: 526 Literale mit einem Digraphen, davon 338 mit echter Umschrift
nach der Stammliste. Verteilung: `xtask` 123, `crates/krk-bench` 107, `crates/krk-ui` 61
(davon 19 in `messmodus.rs`), `crates/krk-core` 47.

**Beide bekannten Fallen sind geprueft.** Ueber einen Zeilenumbruch verteilte Woerter: der
Scanner liest die Fortsetzung mit, und ein `grep -rnE '[a-zA-ZaeoeueAeOeUess]\\$'` ueber
`krk-core/src` und `krk-ui/src` findet ausserhalb einer Markdown-Probe keine einzige Stelle,
an der ein Wort geteilt ist. Ueber `format!` zusammengesetzte Meldungen: die Bruchstuecke
(`Grund::beschreibung`, `Ersatz::satzteil`, `eintraege_text`, `profilmeldung`,
`zeilenmeldung`) sind einzeln erfasst und einzeln geprueft.

## Was geaendert ist

61 Zeichenketten in `krk-core` und `krk-ui`, die durch KRKs Oberflaeche gehen — Statuszeile,
Blaetter, Abschlussliste eines Vorgangs, Vorschau. Danach steht `krk-core` bei 7 und `krk-ui`
bei 40 verbliebenen Stellen, und jede davon ist Diagnostik, ein Bezeichner in einem
Formatplatzhalter, die maschinenlesbare Ausgabe von `--menue-protokoll` oder eine
`eprintln!`-Zeile.

Acht Proben prueften auf den alten Wortlaut und sind mitgezogen: vier in
`crates/krk-core/tests/ablage.rs`, drei in `crates/krk-core/tests/operation.rs`, je eine in
`crates/krk-ui/src/appkit/weitereinstanz.rs` und
`crates/krk-ui/src/kommandos/operationen.rs`. Eine Probe, die grün geblieben waere, obwohl ihr
Wortlaut fiel, gibt es nicht: die Proben in `crates/krk-core/tests/leseprofil.rs` pruefen auf
Wortbestandteile, die die Aenderung nicht anfasst (`Fanggruppen`, `Platzhalter`,
`Ortsangabe`). `Ortsmangel::grund` traegt ueberhaupt keine Probe ausserhalb seiner Datei — ein
Zustand von vorher, den dieser Auftrag nicht erzeugt hat.

## Was bewusst steht bleibt

- Diagnostik des Uebersetzers und der Proben (`#[must_use]`, `assert`, `debug_assert`,
  `panic`, `expect`). Der Vorgaengerdatensatz nimmt Kommentare ausdruecklich aus, und diese
  Texte sind Kommentar in Attributform.
- Die Terminalausgabe von `xtask`, `krk-bench`, `--messmodus` und den
  `eprintln!("krk: …")`-Zeilen. Der Vorgaengerdatensatz begruendet die gemeinsame Regel fuer
  `krk-core` und `krk-ui` mit der geteilten Statuszeile, und diese Begruendung traegt fuer ein
  Terminal nicht. Als eigene Frage abgelegt.
- Die maschinenlesbare Ausgabe `--menue-protokoll` (`appkit/menue.rs`).
- TOML-Schluessel, die in einer Meldung zitiert werden: `BAUSTEINNAMEN`
  (`"zaehlung, juengste, feld, vorhandensein"`) und `"juengste mit anzahl = 0 …"`. Wer sie
  umschreibt, nennt dem Nutzer einen Schluessel, den seine `readers.toml` nicht kennt.
- Bezeichner in Formatplatzhaltern (`{groesse}`, `{HOECHSTENS_EINTRAEGE}`, `{erlaeuterung}`,
  `{wofuer}`, `{uebertragen}`).
- `resources/default-keymap.toml` und `resources/default-readers.toml` tragen in ihren
  nutzersichtbaren Werten (`name`, `beschriftung`) keine Umschrift; geprueft, nichts zu tun,
  kein Auftrag fuer den `ontocoder`.

## Abnahme

Alle fuenf Kommandos am Ende des Durchgangs gefahren, jedes mit Exit 0:

```
cargo build --workspace                                   -> exit 0
cargo test --workspace                                    -> exit 0
cargo clippy --workspace --all-targets -- -D warnings     -> exit 0
cargo fmt --all --check                                   -> exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps -> exit 0
```

Nicht committet; der Orchestrator committet.

## Datensaetze

- `260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`
  von `_a_` auf `_i_`, mit `Implemented:`-Zeile.
- Neu: `260907-0826_*_gilt-die-umlautregel-auch-fuer-die-terminalausgabe-von-xtask-krk-bench-und-messmodus.md`
- Neu: `260907-0826_*_wie-wird-die-naht-zwischen-umlaut-und-umschrift-gehalten-jetzt-da-sie-eine-regel-ist.md`
  — die Antwort auf Anforderung 4: eine Probe kann die Sichtbarkeit einer Zeichenkette aus dem
  Quelltext nicht entscheiden, und die Naeherung, die ohne Aufzaehlung zu haben waere, faengt
  den haeufigeren Fall nicht. Statt einer halben Loesung im Baum steht die Frage als Datensatz.

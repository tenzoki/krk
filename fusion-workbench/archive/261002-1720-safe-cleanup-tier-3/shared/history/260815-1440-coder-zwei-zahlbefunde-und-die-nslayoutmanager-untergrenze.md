# Zwei Zahlbefunde und die NSLayoutManager-Untergrenze

**Status:** Complete
**Agent:** coder
**Anlass:** zwei Defektdatensätze im gemeinsamen Speicher,
`shared/issues/260813-1345_p_fuenf-stellen-nennen-79-funktionen-und-73-mit-kommando-die-belegung-fuehrt-82-und-76.md`
und
`shared/issues/260812-1558_p_zwei-modulkoepfe-nennen-fuer-nslayoutmanager-macos-10-0-das-sdk-sagt-10-7.md`;
Nutzerlinie vom 260815-1418: die Zahlen frisch nachzählen, den Datensatz nicht
als Quelle nehmen, beide Aufgaben getrennt halten

---

## Was geändert wurde

Reine Doc-Kommentararbeit in drei Dateien. Kein ausführbarer Code, keine
Signatur, keine Probe, keine `.toml`. Kein Commit.

## Aufgabe A — die Funktionszahlen stehen auf 84 und 78

Der Datensatz nannte 82 und 76, den Stand vom 260813. Am 260815 selbst
nachgezählt, drei Zählungen, die sich gegenseitig stützen:

```
grep -c '^id = ' resources/default-keymap.toml            -> 84
grep -c '^gehalten_von = ' resources/default-keymap.toml  ->  6
awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs \
  | grep -cE '^    [A-ZÄÖÜ]'                              -> 78
```

84 Funktionen minus die sechs zugestellten Textbefehle ergibt 78 mit Kommando,
und die Aufzählung `Kommando` trägt genau 78 Varianten. Gesetzt ist also
79 → 84 und 73 → 78, nicht die 82 und 76 des Datensatzes.

Der Datensatz nennt fünf Stellen; es sind neun Zeilen in zwei Dateien. Alle neun
führen die Zahl in dieser Bedeutung, keine ist eine Zeilennummer oder eine Größe.
Nach der Änderung findet `grep '79\|73'` in beiden Dateien nichts mehr.

`crates/krk-ui/src/appkit/menue.rs`, Zeilen 128, 799, 867. Der Satzbau geht mit:
„Fuer 73 der 79 Funktionen" heißt jetzt „Fuer 78 der 84 Funktionen".

`crates/krk-ui/src/belegungsausgabe.rs`, Zeilen 45, 48, 56, 256, 725, 726.
Zeile 56 ist eine Tabellenzelle des Modulkopfs, Zeile 256 ein Kommentar im Rumpf
von `wirkung`, die übrigen Modulkopf und Probenkommentar.

Die Sechs bleibt überall stehen: sie stimmt weiter.

## Aufgabe B — NSLayoutManager steht seit 10.7

Am SDK selbst nachgelesen, nicht dem Datensatz geglaubt:
`MacOSX.sdk/…/AppKit.framework/Headers/NSLayoutManager.h:65` trägt
`API_AVAILABLE(macos(10.7), ios(7.0), tvos(9.0))`.

**Nur eine der beiden genannten Dateien trug den Fehler noch.**
`crates/krk-ui/src/appkit/editor.rs` ist bereits berichtigt und im Baum: sein
Modulkopf führt `NSLayoutManager` seit dem 260812 in einem eigenen fetten Absatz
mit der 10.7 und nennt den Defektdatensatz dazu. Angefasst wurde die Datei
deshalb nicht.

`crates/krk-ui/src/appkit/nummernspalte.rs`, Zeilen 83–86: die Klasse stand in
der Aufzählung der seit 10.0 verfügbaren und trug damit keine eigene Angabe.
Vorher:

```
//! `NSRulerView`, `NSLayoutManager`, `NSTextContainer`, `NSTextStorage` und
//! `NSClipView` stehen seit macOS 10.0 zur Verfuegung; das Buendel zielt auf
//! 15.0 (`.cargo/config.toml`). …
```

Nachher:

```
//! `NSRulerView`, `NSTextContainer`, `NSTextStorage` und `NSClipView` stehen
//! seit macOS 10.0 zur Verfuegung, `NSLayoutManager` traegt im SDK
//! `macos(10.7)` (`NSLayoutManager.h:65`, am SDK gelesen); das Buendel zielt
//! auf 15.0 (`.cargo/config.toml`). …
```

Die Schreibweise ist die von `crates/krk-ui/src/appkit/textmerkmale.rs`
übernommen, das die 10.7 schon führt und laut Auftrag nicht angefasst wurde.

## Was daneben auffiel und nicht angefasst wurde

`crates/krk-ui/src/appkit/textmerkmale.rs:65` sagt im Präsens, die 10.0 sei
das, „die zwei andere Modulkoepfe dieses Verzeichnisses fuer dieselbe Klasse
nennen". Nach dieser Runde nennt sie keiner mehr. `appkit/vorschau.rs:138` sagt
dasselbe im Präteritum („wie zwei aeltere Modulkoepfe … es nannten") und bleibt
richtig. Beide Dateien standen außerhalb des Auftrags.

Die beiden Defektdatensätze tragen weiter den Marker `_p_`; ihr Nachziehen war
nicht Teil des Auftrags. Der Datensatz zu Aufgabe A nennt in seinem Rumpf noch
die überholten Werte 82 und 76 und fünf statt neun Stellen.

## Prüfung

```
export PATH="$HOME/.cargo/bin:$PATH"
cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo build --workspace
```

Exit 0. Einmal gefahren, am Ende, nach beiden Aufgaben. Clippy meldete nichts.

---

## Nachtrag vom 260815-1420 — der Satz in `textmerkmale.rs`

Nutzerlinie: der Befund aus dem Abschnitt „Was daneben auffiel" ist von der
Änderung an `nummernspalte.rs` erzeugt worden und gehört deshalb in dieselbe
Runde, nicht in einen neuen Defektdatensatz. Auftrag: nur
`crates/krk-ui/src/appkit/textmerkmale.rs`, nur der Modulkopf, kein Commit.
`vorschau.rs` ausdrücklich ausgenommen, weil es im Präteritum steht und
richtig geblieben ist.

Vorher (Zeilen 64 bis 67):

```
//! zur Verfuegung. `NSLayoutManager` traegt im SDK `macos(10.7)`
//! (`NSLayoutManager.h:65`) und nicht die 10.0, die zwei andere Modulkoepfe
//! dieses Verzeichnisses fuer dieselbe Klasse nennen; die Zahl hier ist am SDK
//! gelesen.
```

Nachher (Zeilen 64 bis 68):

```
//! zur Verfuegung. `NSLayoutManager` nicht: die Klasse traegt im SDK
//! `macos(10.7)` (`NSLayoutManager.h:65`, am SDK gelesen). Sie steht in dieser
//! Aufzaehlung zwischen lauter 10.0ern, und die Reihe fortzuschreiben statt
//! nachzusehen hat in diesem Verzeichnis schon einmal die falsche Zahl
//! erzeugt.
```

Der Satz zählt keine fremden Dateien mehr. Was er stattdessen sagt, ist der
Grund, aus dem die 10.0 naheliegt: `NSLayoutManager` steht in dieser
Aufzählung zwischen lauter Klassen, die tatsächlich seit 10.0 stehen, und wer
die Reihe fortschreibt statt im SDK nachzusehen, trägt sie für sie mit ein.
Die Warnung bleibt im Präteritum erhalten („hat … schon einmal die falsche
Zahl erzeugt") und stimmt damit auch dann noch, wenn niemand mehr weiß,
welche Köpfe es einmal waren.

## Prüfung des Nachtrags

```
export PATH="$HOME/.cargo/bin:$PATH"
cargo fmt --all --check && cargo build --workspace
```

Exit 0. Ein Lauf, kein paralleler zweiter Bau.

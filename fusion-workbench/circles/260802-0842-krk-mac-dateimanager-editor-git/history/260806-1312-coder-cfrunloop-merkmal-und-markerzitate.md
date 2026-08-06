# coder — Das Merkmal `CFRunLoop` und die Markerzitate im Quelltext

**Status:** Complete
**Zeitpunkt:** 260806-1312
**Ausführender:** `coder`
**Auftrag:** Zwei kleine offene Defekte beheben. Erstens das Merkmal `CFRunLoop` aus
`Cargo.toml` entfernen, nachdem seine Nutzerlosigkeit belegt ist. Zweitens das
Markerzitat im Modulkopf von `belegungsmodell.rs` nachziehen und dabei den ganzen
Bestand auf eine Schreibweise bringen, die nicht wieder veraltet. Nicht committen,
nichts in den Defektdateien vermerken, das Plandokument nicht anfassen.

---

## Ergebnis je Defekt

| # | Defekt | Ausgang |
|---|--------|---------|
| 1 | Merkmal `CFRunLoop` in `Cargo.toml` ohne Nutzer (`260805-0905`) | behoben, Nutzerlosigkeit belegt |
| 2 | Modulkopf zitiert einen Issue-Pfad mit überholtem Marker (`260806-1123`) | behoben, dazu 61 weitere Zitate derselben Bauart |

`make check` ist grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check` und `cargo clippy --workspace --all-targets -- -D warnings`
laufen ohne Befund durch. Weder `Cargo.lock` noch eine Datei außerhalb von `Cargo.toml`
und `crates/` ist berührt.

---

## 1. Das Merkmal `CFRunLoop`

### Der Beleg für die Nutzerlosigkeit

Der Datensatz führt bereits `grep -rn CFRunLoop crates/ --include='*.rs'` ohne Treffer
an. Das allein trägt die Aussage nicht: es zeigt nur, dass kein Modul den Namen
schreibt, und lässt offen, ob eine Abhängigkeitskette oder eine `cfg`-Bedingung das
Merkmal doch zieht. Drei Prüfungen schließen das:

**Kein Modul nennt einen Typ aus dem Merkmal.** Die einzige Zeile im ganzen Baum, die
aus `objc2-core-foundation` einführt, ist `crates/krk-ui/src/appkit/fsevents.rs:89`:

```rust
use objc2_core_foundation::{CFArray, CFIndex, CFRetained, CFString};
```

`CFIndex` und `CFRetained` liegen im Rumpf der Kiste und hängen an keinem der
Typmerkmale; `CFArray` und `CFString` bleiben in der Liste. Die übrigen Treffer auf
`RunLoop` im Baum sind `NSRunLoop` und `NSRunLoopCommonModes` in `vorschau.rs`,
`bildtakt.rs`, `tabelle.rs` und `anwendung.rs`. Beide kommen aus `objc2-foundation`
und nicht aus `objc2-core-foundation`; sie sind von diesem Merkmal unabhängig.

**Keine `cfg`-Bedingung schaltet etwas dazu.** `grep -rn 'cfg(feature\|cfg(any\|cfg!('`
über `crates/` und `xtask/` findet genau einen Treffer, `cfg!(debug_assertions)` in
`crates/krk-bench/src/bericht.rs:73`. Merkmalsabhängigen Code gibt es im Vorhaben
nicht.

**Der Merkmalsgraph nennt vor der Änderung zwei Einschalter, danach einen.**
`cargo tree -e features -i objc2-core-foundation --workspace`, vorher:

```
├── objc2-core-foundation feature "CFRunLoop"
│   ├── krk-ui v0.1.0 (/Users/k1/Projects/productive/krk/crates/krk-ui) (*)
│   └── objc2-foundation v0.3.2 (*)
```

nachher:

```
├── objc2-core-foundation feature "CFRunLoop"
│   └── objc2-foundation v0.3.2 (*)
```

### Was daraus folgt, und was der Datensatz zu hoch ansetzt

Der Datensatz schreibt, das Merkmal ziehe "ein paar Typen mehr in die Übersetzung und
sonst nichts". Der Graph zeigt, dass es nicht einmal das tut. `objc2-foundation`
schaltet `CFRunLoop` von sich aus ein, und KRK führt diese Kiste mit ihren
Vorgabemerkmalen. Die Merkmalsvereinigung von Cargo lässt das Merkmal also
eingeschaltet, egal was `Cargo.toml` im Projektwurzelverzeichnis dazu sagt. Der Gewinn
der Änderung liegt allein dort, wo der Datensatz ihn unter "Warum es zählt" auch
verortet: der Kommentar behauptete eine Verwendung, die es nicht gibt, und schickte
den nächsten Leser auf die Suche.

Belegt ist das nebenbei durch `Cargo.lock`: die Datei ist nach der Änderung unverändert
(`git status --porcelain Cargo.lock` schweigt).

### Die Änderung

`Cargo.toml` verliert die Zeile `"CFRunLoop",` aus der Merkmalsliste von
`objc2-core-foundation`. Der Kommentar darüber begründet jetzt nur noch `CFArray` und
`CFString` und trägt in einem zweiten Absatz nach, dass `CFRunLoop` bis zum 260806 dort
stand, weshalb es weggefallen ist, und dass die Übersetzung sich dadurch nicht ändert.
Der Nachtrag steht dort, damit niemand das Merkmal später wieder einträgt, weil er den
Wegfall für ein Versehen hält.

Nicht angefasst ist `crates/krk-ui/Cargo.toml:41`. Die Zeile nennt `CFRunLoopSource` als
den Weg, den KRK für den Weckruf des Vermittlerfadens **nicht** gegangen ist. Sie
behauptet keine Verwendung, sondern begründet eine verworfene Möglichkeit, und bleibt
richtig.

---

## 2. Die Markerzitate im Quelltext

### Der Befund ist größer als die eine Zeile

Der Datensatz meldet `crates/krk-ui/src/belegungsmodell.rs:27` und benennt die Ursache
selbst: ein Zustandsmarker wandert mit dem Bearbeitungsstand (`_o_` → `_p_` → `_c_`),
also ist jedes in Code einzementierte Markerzitat ab dem nächsten Übergang falsch. Die
Ursache trifft nicht eine Zeile, sondern jede Zeile dieser Bauart. Der Bestand:

```
$ grep -rnE '[0-9]{6}-[0-9]{4}_[opcaidstb]_' --include='*.rs' crates/ xtask/ | wc -l
      62
```

62 Zitate in 32 Dateien, davon eines das gemeldete. Nur die eine Zeile zu korrigieren
hätte 61 Zeitbomben derselben Art stehen gelassen.

### Die gewählte Schreibweise

Der Datensatz stellt zwei Wege zur Wahl, den Marker auf `_c_` zu aktualisieren oder ihn
wegzulassen, und merkt an, dass nur der zweite nicht wieder veraltet. Gewählt ist die
**Platzhalterform** `260806-1054_*_belegungsansicht-gruppiert-nach-funktionsbereich.md`,
also der Stern an der Stelle des Markers. Sie ist im Vorhaben nicht neu: Spec und Plan
schreiben Pfade seit jeher so, mit 209 Vorkommen im Workbench, und ein Zitat in dieser
Form findet seine Datei über `ls` oder eine Ergänzung der Kommandozeile unmittelbar.
Den Marker ersatzlos zu streichen wäre die zweite Möglichkeit gewesen; sie hätte einen
Pfad ergeben, der so auf keiner Platte steht und den auch keine Vervollständigung
findet.

### Die Änderung

Alle 62 Zitate sind auf die Platzhalterform gezogen, mechanisch über
`(\d{6}-\d{4})_[opcaidstb]_` → `\1_*_`. Verteilung:

| Datei | Zitate |
|---|---|
| `crates/krk-core/tests/belegung.rs` | 7 |
| `crates/krk-ui/src/appkit/anwendung.rs` | 7 |
| `crates/krk-ui/src/appkit/menue.rs` | 7 |
| `crates/krk-ui/src/kommandos/operationen.rs` | 4 |
| `crates/krk-bench/src/messen.rs` | 3 |
| `crates/krk-ui/src/appkit/statuszeile.rs` | 3 |
| `crates/krk-core/src/operation/umbenennen.rs`, `crates/krk-core/src/stapelumbenennen/mod.rs`, `crates/krk-core/src/verzeichnis/leser.rs`, `crates/krk-ui/src/appkit/fsevents.rs`, `crates/krk-ui/src/appkit/tabelle.rs` | je 2 |
| 21 weitere Dateien | je 1 |

Drei Punkte zur Abgrenzung:

**Das Muster verlangt den Zeitstempel vor dem Marker** und greift deshalb nicht auf
Bezeichner, die zufällig so aussehen: `als_c_pfad` in `verzeichnis/sys.rs` und
`cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt` in `tests/belegung.rs` sind
unberührt. Nachgeprüft am Diff: er enthält ausschließlich Kommentar- und Zitatzeilen.

**Drei der 62 stehen in einer Zeichenkette, nicht in einem Kommentar.** Es ist der
Begleittext, den `crates/krk-bench/src/messen.rs` in den Messbericht schreibt; einer
davon zitiert `260803-1755_*_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`. Sie sind
mitgezogen, weil ein Bericht denselben toten Pfad trägt wie ein Kommentar, sobald der
Datensatz seinen Stand wechselt.

**Zitate außerhalb von `.rs` sind stehen geblieben.** `resources/default-keymap.toml`
und `resources/default-settings.toml` führen zusammen 14 weitere (11 und 2 Zeilen,
eine davon mit zwei Zitaten); sie sind
Datendateien und liegen beim `ontocoder`, nicht beim `coder`. Der Auftrag hat sie
ausdrücklich ausgenommen. Sie tragen denselben Defekt und brauchen denselben Zug; ein
eigener Datensatz dafür ist nicht angelegt, weil das Anlegen und Vermerken beim
Auftraggeber liegt.

---

## Geänderte Dateien

`Cargo.toml` sowie 32 Dateien unter `crates/`, zusammen 74 eingefügte und 68 entfernte
Zeilen, sämtlich Kommentar- und Zitatzeilen. Kein Verhalten des Programms ist berührt,
keine `#![deny(unsafe_code)]`-Grenze angetastet, keine Tracking-Datei außer diesem
Eintrag geschrieben.

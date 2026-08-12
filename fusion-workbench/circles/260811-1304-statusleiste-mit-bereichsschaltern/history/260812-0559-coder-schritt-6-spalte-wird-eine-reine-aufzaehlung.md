# Coder, Schritt 6: `Spalte` wird eine reine Aufzählung

**Datum:** 260812-0559
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 6
**Abnahme:** `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p krk-core` — alle vier
Exit 0. `make check` ist für diesen Schritt nicht erreichbar; der Grund liegt außerhalb
(`issues/260812-0548_o_make-check-bleibt-auch-nach-schritt-5-rot-….md`).

## Auftrag

Der mechanische Umzug ohne Verhaltensänderung: `Spalte` verlässt
`crates/krk-ui/src/appkit/tabelle.rs` und wird zu einem eigenen Modul neben `appkit`, damit die
Bereichsleiste aus Schritt 8 die Aufzählung lesen kann, ohne die Tabelle zu brauchen. Nicht
Schritt 7, nicht committen.

Der Baum war beim Beginn rot, mit 28 Fehlschlägen in `krk-ui`, jeder mit
`spalte_groesse_umschalten` in der Meldung. Er ist es am Ende unverändert: dieselbe Zahl,
dieselben Namen, dieselbe Ursache. Schritt 7 baut die drei Kommandos.

## Was entstanden ist

**`crates/krk-ui/src/spalten.rs` (neu)**

- `pub enum Spalte` mit den vier Werten und ihren Dokumentationskommentaren, unverändert
  übernommen; der lange Kommentar an `Typ` mit den zwei Entscheiden von 260806 ist mitgezogen
  und nennt `Schluessel::Typ` jetzt über den vollen Pfad, weil die Kiste `krk_core` hier nicht
  in den Namensraum gezogen wird.
- `Spalte::ALLE` und `Spalte::beschreibbar` sind mitgezogen und heißen `pub`.
- Neu ist `Spalte::beschriftung(self) -> &'static str`: der kurze Name, den ein Schalter der
  18 Punkte hohen Bereichsleiste trägt. Drei der vier Namen sind zugleich die Spaltenüberschrift
  der Tabelle. `Geaendert` weicht ab — Schalter "Datum", Überschrift "Änderungsdatum" —, und der
  Kommentar sagt, dass das gewollt ist und warum: "Datum" ist der Name, den der Nutzer dem
  Schalter gegeben hat, und über der Spalte stünde er zu knapp, weil dort auch die Uhrzeit steht.
- **Der Modulkopf sagt zweierlei**: keine Zeile AppKit, und `Spalte` ist eine vollständige
  Fallunterscheidung ohne Auffangzweig, die alle vier Werte behält. Der Grund steht dabei — ein
  `_ =>` machte aus einer fünften Spalte still eine linksbündige, unbeschreibbare Namenlose.
- Zwei Prüfungen ohne AppKit: jede Beschriftung ist nicht leer und von jeder anderen
  verschieden; beschreibbar ist genau die Namensspalte.

**`crates/krk-ui/src/main.rs`**

- `mod spalten;` zwischen `pruefordner` und `tabs`.
- Der Modulkopf zählt jetzt dreizehn Module neben `appkit` statt zwölf und beschreibt das neue
  in einem Satz, samt dem Grund für seinen Ort: zwei Leser hängen an der Aufzählung, die Tabelle
  und die Bereichsleiste, und der zweite braucht die Tabelle nicht.

**`crates/krk-ui/src/appkit/tabelle.rs`**

- Die Aufzählung und ihr `impl` sind weg; an ihre Stelle treten fünf freie Funktionen über
  `Spalte`: `kennung`, `titel`, `breiten`, `ausrichtung`, `aus_kennung`. Dasselbe Muster wie
  `aufteilung::sichtbar_im` und `aufteilung::rahmenfarbe`. Der Kommentar über dem Block nennt
  das Muster und den Grund, warum es freie Funktionen sind.
- `ausrichtung` hat seinen Auffangzweig `_ => NSTextAlignment::Left` verloren und schreibt die
  drei linksbündigen Spalten aus. Das ist die einzige Stelle, an der sich am Programmtext mehr
  als der Aufrufweg geändert hat, und sie zieht die Datei an die Regel heran, die der neue
  Modulkopf aufschreibt: eine fünfte Spalte hält den Bau an.
- `aus_kennung` nimmt seinen Parameter als `gesucht`. Hieße er `kennung`, verdeckte er im
  eigenen Rumpf die gleichnamige Funktion; der Kommentar sagt das.
- **`titel` liefert `Retained<NSString>` und nicht `&'static NSString`, und das ist die eine
  Abweichung vom Plan.** Beides zugleich war nicht zu haben: `ns_string!` verlangt sein Literal
  an Ort und Stelle, also stünden die drei aus `beschriftung` übernommenen Texte ein zweites Mal
  im Programmtext, und genau das soll die Ableitung verhindern. Der Plan verlangt die Ableitung
  ausdrücklich, die Lebensdauer nur nebenbei; die Ableitung hat gewonnen. Einziger Aufrufer ist
  `spaltenkopf`, der die Zeichenkette achtmal baut, beim Aufbau der vier Spalten der beiden
  Dateifenster. Der Kommentar an `titel` schreibt beides aus.
- Die beiden bestehenden Prüfungen am Ende der Datei **bleiben**, weil beide AppKit nennen
  (`ns_string!`, Kennung und Überschrift). Sie rufen die freien Funktionen statt der Methoden.
- Die Aufrufstellen sind nachgezogen: `zellenansicht`, `feld`, `spaltenkopf` und die Schleife im
  Aufbau. `beschreibbar` bleibt eine Methode und wird weiter als solche gerufen, auch in der
  konstanten Rechnung `NAMENSSPALTE`, die dadurch `const fn` bleibt.

## Was nicht geschehen ist

- **Kein Schritt 7.** Weder `Spaltensichtbarkeit`, noch `spalte_sichtbar_in`, noch die drei
  Kommandos, die die 28 Fehlschläge auflösen.
- Kein Commit; der Orchestrator trägt ein.
- Keine Verhaltensänderung. Die Tabelle baut dieselben vier Spalten mit denselben Kennungen,
  Überschriften, Breiten und Ausrichtungen wie zuvor.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | Exit 0 |
| `cargo fmt --all --check` | Exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| `cargo test -p krk-core` | Exit 0, 331 Proben |
| `cargo test -p krk-ui` | Exit 101, 347 bestanden, **28 gescheitert** |

Die 28 sind die 28 aus dem Befund zu Schritt 5: dieselbe Zahl wie vor diesem Schritt, und jede
einzelne nennt `spalte_groesse_umschalten` in ihrer Meldung. Keine nennt `Spalte`, `spalten.rs`
oder `tabelle.rs`. Die vier Proben um die Spalten laufen: die beiden neuen in `spalten::tests`
und die beiden gebliebenen in `appkit::tabelle::tests`.

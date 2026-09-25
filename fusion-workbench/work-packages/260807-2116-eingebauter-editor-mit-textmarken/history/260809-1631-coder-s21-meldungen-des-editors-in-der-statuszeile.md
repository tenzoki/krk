# S21: Die Meldungen des Editors in der Statuszeile

**Agent:** coder
**Datum:** 260809-1631
**Status:** Complete
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 21

## Was gebaut ist

Der Editor bekommt keine eigene Meldezeile. Was er zu sagen hat, geht als Wert nach
oben und landet auf Rang 1 der Statuszeile des aktiven Dateifensters, über denselben
`antwort_zeigen`, den Leiste und Vorschau seit der Runde 1 nehmen.

```text
  krk_core::text::datei::oeffnen ──Err(Abweisung)──┐
                                                   │
  krk_core::text::marke::wiederfinden ──Fund───────┤
                                                   v
                                          Editormeldung          appkit/editor.rs
                                                   │  .text()
                                                   v
                              editormeldung_zeigen ──> antwort_zeigen(aktiv, …)
                                                                   appkit/anwendung.rs
                                                   │
                                                   v
                              statuszeile::zeile, Rang 1 von fünf   unverändert
```

**Zwei Meldungen, beide aus einer schon gebauten Quelle.**

- `Editormeldung::Abgewiesen(Abweisung)` — die drei Abweisungsgründe aus S10, über
  `Abweisung::meldung` durchgereicht und nicht neu formuliert. Der Datensatz
  `260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md` verlangt, dass „zu
  groß" von „nicht als Text lesbar" zu unterscheiden ist; die Unterscheidung steht in
  `krk-core` und wird hier nicht ein zweites Mal getroffen.
- `Editormeldung::MarkenstelleGeaendert { zeile }` — der Fall `Fund::NichtGefunden`
  aus S12. Der Datensatz
  `260807-2147_a_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md` sagt: die
  Marke springt trotzdem an die gemerkte Zeilennummer und meldet, dass die Stelle sich
  geändert hat. Der Satz nennt die Zeile, an die sie geführt hat.

`Editormeldung::markenstelle` trägt die vollständige Fallunterscheidung über `Fund`:
getroffen und verschoben melden nichts, weil beide an der richtigen Stelle landen.
`Editormeldung::text` ist vollständig und ohne Auffangzweig; eine siebte Meldung hält
den Bau an und erzwingt ihren Satz.

**Keine zweite Meldefläche.** `statuszeile.rs` trägt keine geänderte Zeile. `zeile`
hat weiterhin fünf Parameter, fünf Ränge und dieselbe Vorrangregel; eine sechste
Quelle entsteht nicht.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/editor.rs` — `Editormeldung` samt `markenstelle`, `text`
  und zwei Proben; ein Absatz im Modulkopf über den Meldeweg.
- `crates/krk-ui/src/appkit/anwendung.rs` — `Anwendungsdelegierter::editormeldung_zeigen`
  neben `antwort_zeigen`, und der Import von `Editormeldung`.

Unberührt: `crates/krk-core/`, `resources/default-keymap.toml`,
`crates/krk-ui/src/appkit/menue.rs`, `crates/krk-ui/src/appkit/aufteilung.rs`,
`crates/krk-ui/src/appkit/statuszeile.rs`. Die beiden Zweige `Fokus::Editor => false`
in `anwendung.rs` stehen unverändert; sie gehören S17.

## Abnahme

| Kommando | Ergebnis |
| --- | --- |
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, darunter die beiden neuen Proben |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |

Die beiden Kriterien des Schrittes:

- `grep -c 'Statuszeile' crates/krk-ui/src/appkit/editor.rs` → **0**. Der Editor kennt
  die Zeile nicht, auch nicht in seiner Prosa; der Modulkopf spricht von „der einen
  Meldefläche des Fensters aus C1".
- `git diff --stat crates/krk-ui/src/appkit/statuszeile.rs` → leer. Keine sechste
  Quelle in `statuszeile::zeile`.

Die `unsafe_code`-Grenze ist nicht gewachsen:
`grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` nennt
weiterhin genau `appkit/mod.rs`.

## Zwei Vermerke

**Der Meldeweg steht ohne Auslöser da.** Die Auslöser sind S22 (F4), S23 (Übergang aus
der Vorschau) und S39 (Sprung auf eine Textmarke) und gehörten nicht zu diesem Schritt.
Deshalb tragen `Editormeldung` und `editormeldung_zeigen` je ein
`#[allow(dead_code)]`. Gemessen am 260809 mit entfernten Zeilen:
`cargo clippy --workspace --all-targets` meldet drei Fundstellen toten Werts, und der
Arbeitsbereich stünde rot, weil `make lint` mit `-D warnings` fährt. Ablösender Schritt
ist **S22**, der erste Auslöser; beide Kommentare nennen ihn, wie es
`issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`
verlangt.

**Ein Markensprung kann zwei Meldungen zugleich haben.** `Markensprung` führt
`fund` und `sprung.lage` als zwei verschiedene Auskünfte, und `marke.rs:93-98` verlangt
vom Aufrufer, beides zu melden. Rang 1 trägt einen Text. Dieser Schritt baut die erste
Hälfte; die zweite ist die Meldung der Zeilenlage aus C5 und kommt mit S35. Der
zusammengesetzte Fall ist als
`issues/260809-1631_o_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`
festgehalten und bei S39 zu entscheiden; der Vorschlag darin ist ein Satz für den
zusammengesetzten Fall statt einer zweiten Vorrangregel neben der bestehenden.

## Was ein Agent nicht prüfen kann

Nichts an diesem Schritt ist `Nutzerarbeit`. Der Meldeweg ist am laufenden Bündel erst
sichtbar, wenn ein Auslöser ihn geht; die erste Sichtprüfung fällt mit S22 an, dessen
Abnahmekriterium sie ohnehin führt.

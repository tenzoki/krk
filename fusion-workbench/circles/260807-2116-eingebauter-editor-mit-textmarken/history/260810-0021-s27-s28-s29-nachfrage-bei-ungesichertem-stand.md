# S27, S28 und S29: die Nachfrage bei ungesichertem Stand

**Status:** Complete
**Agent:** coder
**Datum:** 260810-0021
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritte 27, 28, 29

---

## Was gebaut ist

Die Nachfrage aus C4 steht an allen vier Anlässen: beim Schließen des Editors (`opt+cmd+e`), beim Aufnehmen einer anderen Datei (F4), beim Einblenden der Vorschau (`f3`, `cmd+y`, `shift+cmd+y`) und beim Beenden der Anwendung. Sie hat drei Wahlmöglichkeiten — sichern, verwerfen, abbrechen — und ein gescheitertes Sichern lässt den Anlass unterbleiben, statt den Stand mitzunehmen.

Die getaktete Sitzungssicherung fragt weiterhin nichts und trägt den ungesicherten Stand nicht mit; `applicationWillTerminate:` ist unverändert geblieben und läuft nach der Zustimmung.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-ui/src/appkit/blaetter/ungesichert.rs` | neu: das Blatt und seine dreiwertige `Antwort` |
| `crates/krk-ui/src/appkit/blaetter/mod.rs` | Modulkopf auf sieben Blätter, `pub mod ungesichert` |
| `crates/krk-ui/src/editormodell.rs` | `Ladeausgang::Zurueckgehalten`, das Zurückhalten und seine zwei Ausgänge, fünf neue Proben |
| `crates/krk-ui/src/appkit/editor.rs` | `hat_ungesicherten_stand`, `schliessen`, die beiden Wege des Zurückgehaltenen |
| `crates/krk-ui/src/appkit/anwendung.rs` | `Anlass`, die eine Nachfrage, die vier Anlässe, `applicationShouldTerminate:` |

## Drei Entscheidungen, die vom Schnitt des Plans abweichen

**1. Eine Aufrufstelle des Blattes statt vier.** Die vier Anlässe teilen sich den schwachen Griff auf den Delegierten, `offenes_blatt`, die dreiwertige Antwort und die Behandlung des gescheiterten Sicherns. Viermal aufgeschrieben wären das vier Stellen, an denen das zehnte Abnahmekriterium von C4 zu halten oder zu brechen wäre. Gebaut ist `nachfrage_zeigen` als die eine Stelle, dazu die Aufzählung `Anlass` mit zwei vollständigen Fallunterscheidungen ohne Auffangzweig. Kein Feld hält einen Anlass über den Rückruf hinaus: der Wert reist in der Schließung mit. Das Abnahmekriterium des Plans zählt `grep -c 'ungesichert::zeigen'` gleich vier; gemessen ist 1.

**2. Das Zurückhalten im Modell.** S28 setzt voraus, dass die Aufrufstelle vor `datei_oeffnen` fragt. Seit S24 läuft die Prüfung aber auf dem Arbeitsfaden, und wer vor dem Ruf fragt, fragt vor der Prüfung — der Nutzer bekäme die Nachfrage auch für einen Ordner, den der Editor ohnehin abweist. Genau das verbietet das elfte Abnahmekriterium von C2. `Editormodell` hält deshalb die gelesene Datei zurück, wenn ungesicherter Stand offensteht, und meldet `Ladeausgang::Zurueckgehalten`; zwei Ausgänge verbrauchen sie. Der Gewinn: F4, der Übergang aus der Vorschau (S23) und der Sprung auf eine Textmarke aus C6 erben die Regel, ohne sie zu kennen.

**3. `shift+cmd+y` ist mitgenommen.** Der Fokusbefehl holt seit dem Nutzerentscheid vom 260807 einen ausgeblendeten Bereich hervor und verdrängt den Editor damit genauso wie `f3`. Ihn auszulassen hieße, denselben Verlust auf dem einen Weg abzufragen und auf dem anderen nicht.

## Was die Abnahme sagt

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden alle mit 0. `grep -c 'NSAlert'` auf dem neuen Blatt liefert 0; die Grenze `#![allow(unsafe_code)]` nennt weiterhin allein `appkit/mod.rs`.

Fünf neue Proben in `editormodell.rs` decken ab: eine gelesene Datei wird bei ungesichertem Stand zurückgehalten, die Übernahme geht denselben Weg wie jedes Öffnen, ein Abbruch lässt den Stand vollständig stehen, eine abgewiesene Datei wird **nicht** zurückgehalten (die Reihenfolge aus C2), und das Schließen lässt eine wartende Datei fallen. Die Probe `eine_andere_datei_wird_weiterhin_gelesen` hat ihre Aussage gewechselt: sie hielt bis hierher fest, dass F4 den getippten Stand kommentarlos ersetzt.

## Angelegter Datensatz

`decisions/260810-0021_o_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md` — der dritte Anlass ist aus einer Annahme über den Code entstanden, die der Code nicht trägt: ein verdrängter Editor verliert seinen Stand nicht, und „Verwerfen" verwirft dort nichts. Gebaut ist der Weg des Plans, weil er der einzige von dreien ist, der nichts verlieren kann. Der Datensatz hält keinen Schritt auf.

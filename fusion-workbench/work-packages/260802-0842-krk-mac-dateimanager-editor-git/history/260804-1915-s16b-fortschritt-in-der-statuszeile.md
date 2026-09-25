# Schritt 16b: Der Fortschritt verlässt das Blatt und geht in die Statuszeile (C4, L8)

---
**Agent:** coder
**Status:** Complete
**Datum:** 260804-1915
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 16b.`
**Spec:** derselbe Circle, `planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitte `### C4` und `### C8` (L8)
**Bindend:** derselbe Circle, `decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`

---

## Was entstanden ist

Das Fortschrittsblatt ist weg. Der Stand einer laufenden Dateioperation steht jetzt in der Statuszeile des Dateifensters, das die Operation begonnen hat, und das Fenster bleibt währenddessen bedienbar.

### Entfallene Datei

| Datei | Grund |
|---|---|
| `crates/krk-ui/src/appkit/blaetter/fortschritt.rs` | Ersatzlos. `grep -rn 'fortschritt' crates/krk-ui/src/appkit/blaetter/` liefert keinen Treffer. |

### Erweiterte Dateien

| Datei | Änderung |
|---|---|
| `crates/krk-ui/src/appkit/statuszeile.rs` | `Statuszeile::zeigen` nimmt die Art mit auf: `Art::Fehler` färbt rot wie bisher, `Art::Vorgang` bekommt die gewöhnliche Textfarbe. Eine Funktion, zwei Werte. |
| `crates/krk-ui/src/appkit/tabelle.rs` | Das dritte Feld `vorgangsanzeige` in `QuelleIvars`, die Rangfolge in `meldung_anzeigen`, die beiden Eingänge `vorgang_zeigen` und `vorgang_beenden`. `meldung_zeigen` setzt die Zeile nicht mehr selbst, sondern schreibt sein Feld und lässt `meldung_anzeigen` zeichnen. |
| `crates/krk-ui/src/kommandos/operationen.rs` | `BLATTVERZUG` → `ANZEIGEVERZUG`, `blatt_faellig` → `anzeige_faellig`, `standtext` → `vorgangszeile` (einzeilig, mit dem Abbruchhinweis), dazu `abbruchzeile` und `schon_ein_vorgang`. `waehrend_blatt_erlaubt` behält den Namen und die Umsetzung; sein Kommentar sagt, was seit hier nicht mehr dazugehört. |
| `crates/krk-ui/src/appkit/anwendung.rs` | `Vorgang` trägt die Fensterseite statt des Blattes; `konflikt_steht` entfällt; `kommando_ausfuehren` sperrt nur noch bei stehendem Blatt; `auftrag_stellen` weist einen zweiten Vorgang ab; `fortschritt_zeigen`, `abbrechen` und `vorgang_beenden` schreiben in die Zeile der gemerkten Fensterseite. |
| `crates/krk-ui/src/appkit/blaetter/mod.rs` | `pub mod fortschritt;` entfällt. Der Modulkopf und drei Doku-Absätze nennen die drei verbliebenen Blätter statt der vier. Die Hülle selbst ist unverändert. |

`resources/default-keymap.toml` ist nur gelesen worden. `abbrechen` liegt dort seit S9 auf `esc` und ist hier nicht ein zweites Mal festgelegt.

## Die drei Quellen der Statuszeile

```text
meldung_anzeigen  ──> laufender Vorgang an diesem Fenster?
                         │ja ──> Stand des Vorgangs, "Esc bricht ab"
                         │nein
                      Fenstermeldung?
                         │ja ──> Auswurf, beschädigte Ablagedatei, Abschluss
                         │nein
                      Tabmeldung des sichtbaren Tabs, sonst leere Zeile
```

**Genau eine Stelle entscheidet.** `grep -rn 'statuszeile\.zeigen' crates/krk-ui/src/` nennt zwei Zeilen, `crates/krk-ui/src/appkit/tabelle.rs:1006` und `:1017`; beide stehen in derselben Funktion `meldung_anzeigen`, und keine andere Funktion ruft `Statuszeile::zeigen`. Vorher gab es eine zweite Stelle: `meldung_zeigen` schrieb die Zeile selbst und wäre damit an der Rangfolge vorbeigegangen.

Die Vorgangsanzeige bekommt ein eigenes Feld und teilt sich keines mit der Fenstermeldung, weil ihre Lebensdauern die entgegengesetzten sind. `fenstermeldung_loeschen` läuft bei jedem echten Ordner- und Tabwechsel; die Vorgangsanzeige rührt es nicht an. Gemessen: der Wechsel von Tab 0 auf Tab 1 während einer Kopie lässt den Fortschritt stehen.

## Die sieben Abnahmepunkte

Gemessen am laufenden Bündel `target/KRK.app` am 260804-1915. Prüfdaten unter `/tmp/krk-s16b` auf demselben APFS-Datenträger: ein Prüfordner mit 5.000 Einträgen, einer mit 30.000, ein Ordner mit drei kleinen Dateien, ein leerer Zielordner. Hinterher entfernt.

| Punkt | Ergebnis |
|---|---|
| 1. Während der Kopie von 5.000 Einträgen steht kein Blatt, und die Statuszeile des Quellfensters zeigt den Fortschritt | **Erfüllt.** `NSWindow.attachedSheet` ist leer; die Zeile links liest `Kopieren: 1.818 Einträge, 3,5 GB, eine ausgewählte Position · beleg-2794b00b-002733 · Esc bricht ab`. Dreimal gleich. |
| 2. Navigation, Markierung und Tabwechsel sind bedienbar, und `esc` bricht ab | **Erfüllt.** Während der Kopie von 30.000 Einträgen: drei Pfeiltasten bewegen die Auswahl von Zeile 2 auf Zeile 5; zwei Leertasten markieren zwei Einträge (`betroffene_eintraege().zahl() == 2`); `ctrl+tab` wechselt den Tab von 0 auf 1; `tab` macht das rechte Dateifenster zum aktiven. Die Vorgangsanzeige übersteht beide Wechsel. `esc` beendet die Kopie; die Zeile liest unmittelbar danach `Kopieren abgebrochen: 1.859 Einträge, 3,6 GB (eine ausgewählte Position) übertragen`. Dreimal gleich. |
| 3. Eine Kopie von 3 kleinen Dateien lässt keine Zeile aufblitzen | **Erfüllt.** In 1.500 ms nach dem Tastendruck, an jeder Bildgrenze abgelesen, stand genau ein Text in der Zeile: `Kopieren fertig: 4 Einträge, 18 Bytes (eine ausgewählte Position)`. Keine Zeile mit `Esc bricht ab`. Dreimal gleich. |
| 4. `cargo xtask messen` weist L8 mit dem 95. Perzentil unter 200 ms aus | **Nicht erfüllt, als Defekt gemeldet.** Den Unterbefehl gibt es nicht. Die Zahl selbst hält mit Reserve, siehe unten. Defekt: `issues/260804-1915_o_das-abnahmekriterium-von-s16b-nennt-cargo-xtask-messen-das-es-nicht-gibt.md`. |
| 5. Nach dem Ende steht der Abschlusstext in derselben Zeile und überlebt die Auffrischung | **Erfüllt.** Unmittelbar nach dem Ende, 1,5 s später und noch einmal 1,5 s später steht derselbe Text. Die Auffrischung über `ordner_neu_lesen` räumt ihn nicht weg, weil sie die Fenstermeldung ausdrücklich nicht löscht. Dreimal gleich. |
| 6. Wird während einer laufenden Kopie ein Datenträger ausgeworfen, erscheint die Auswurfmeldung, sobald der Fortschritt endet | **Nicht erfüllt, als Defekt gemeldet.** Während der Kopie steht sie richtigerweise hinten an. Nach dem Ende erscheint sie nicht: der Abschlusstext schreibt in dasselbe Feld und überschreibt sie. Defekt: `issues/260804-1915_o_der-abschlusstext-ueberschreibt-die-verdraengte-fenstermeldung.md`. |
| 7. Ein zweiter Operationsbefehl meldet sich in der Zeile und startet nichts | **Zur Hälfte erfüllt, als Defekt gemeldet.** Gestartet wird nichts, in beiden Fällen. Sichtbar ist die Meldung nur, wenn der zweite Befehl aus dem **anderen** Dateifenster kommt: dort steht `es läuft bereits eine Operation: Kopieren`. Kommt er aus dem Fenster des Vorgangs, verdrängt ihn dessen eigene Anzeige. Defekt: `issues/260804-1915_o_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`. |

Dazu die beiden Prüfungen, die das Abnahmekriterium daneben nennt:

- `grep -rn 'fortschritt' crates/krk-ui/src/appkit/blaetter/` liefert keinen Treffer.
- Der Diff zeigt genau eine Stelle, die entscheidet, was in der Statuszeile steht, siehe oben.

## Wie L8 gemessen wurde

**Nicht mit `cargo xtask messen`, weil es das nicht gibt.** `xtask` kennt genau einen Unterbefehl, `bundle`; die kopflose Strecke `krk-bench messen` misst das Lesen eines Ordners, und `krk-bench durchstich` deckt L1, L2, L3, L4 und L10 ab. `crates/krk-bench/` ist für diesen Schritt gesperrt, und der Messmodus in der Anwendung wächst erst mit S21.

Gemessen wurde stattdessen wie in S8, über den Weg, den auch L1 geht: eine vorübergehende Sonde in `crates/krk-ui/src/appkit/anwendung.rs` hängt denselben `CADisplayLink` ein, den `bildtakt::Zeichenende` einhängt, nimmt den Zeitstempel unmittelbar vor dem Einreihen des F5-Ereignisses über `NSApplication.postEvent:atStart:` und stoppt an der ersten Bildgrenze, an der der Fortschritt in der Statuszeile steht. 20 Läufe, je ein Kaltstart des Bündels, Kopie eines Prüfordners mit 5.000 Einträgen auf denselben APFS-Datenträger:

| Kennzahl | Wert |
|---|---|
| kleinster Wert | 154,5 ms |
| Median | 164,7 ms |
| 95. Perzentil | 168,9 ms |
| größter Wert | 169,0 ms |

Die Zusage von 200 ms hält mit rund 31 ms Reserve. Die Rechnung aus dem Entscheidungsdatensatz — 150 ms Verzug plus rund 17 ms bis zum nächsten Zeichendurchgang, zusammen rund 170 ms — trifft die Messung.

**Was das nicht belegt.** Wann der erste Bildpunkt der Zeile physisch auf dem Schirm steht, ist aus dem eigenen Prozess heraus nicht feststellbar; gemessen ist die Spanne bis zur ersten Bildgrenze nach der Änderung, dieselbe Näherung, die L1, L5, L6 und L7 benutzen. Und: dass eine körperlich gedrückte F5-Taste dieselben Ereignisse erzeugt wie das eingereihte, ist hier nicht gemessen, sondern aus der Messung vom 260802-1137 übernommen.

## Was aufgefallen ist

**Drei Defekte abgelegt**, alle drei in diesem Circle:

- `260804-1915_o_das-abnahmekriterium-von-s16b-nennt-cargo-xtask-messen-das-es-nicht-gibt.md`
- `260804-1915_o_der-abschlusstext-ueberschreibt-die-verdraengte-fenstermeldung.md`
- `260804-1915_o_der-zweite-operationsbefehl-meldet-sich-im-fenster-des-vorgangs-unsichtbar.md`

Die letzten beiden liegen dicht beieinander: eine einzeilige Zeile trägt einen Text, und der Plan verspricht ihr an zwei Stellen zwei. Beide Doku-Kommentare im Code sagen jetzt, was gemessen ist, und nennen den Defekt; keiner behauptet die Zusage, die nicht hält.

**Geschlossen**, wie der Plan es vorsieht:

- `260804-1814_c_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md`
- `260804-1814_c_ein-modales-blatt-widerspricht-der-zusage-dass-die-oberflaeche-bedienbar-bleibt.md`

**Nichts spricht gegen die Bedienbarkeit während einer Operation.** Delete wirkt im Dateifenster hinter dem laufenden Vorgang; das ist die Folge, die C4 ausgeschrieben hat, und es ist keine Sperre dafür gebaut. Beim Umbau ist kein Grund aufgetaucht, der das gefährlicher machte als gedacht. Zwei Beobachtungen, die dazu gehören und beide harmlos sind: eine gelöschte Datei, die die laufende Kopie noch anfassen wollte, landet über den vorhandenen Weg auf der Liste der übersprungenen Einträge; und die Markierungen des Quellfensters fallen mit der Auffrischung am Ende der Operation weg, weil der Ordner neu gelesen wird — das war vorher genauso, nur konnte der Nutzer während der Operation nicht markieren.

**Drei Namen, die gelogen hätten.** `BLATTVERZUG` und `blatt_faellig` heißen jetzt `ANZEIGEVERZUG` und `anzeige_faellig`, und die zugehörige Prüfung heißt `die_vorgangsanzeige_erscheint_erst_nach_150_ms`. Das Blatt, das sie benannten, gibt es nicht mehr. `waehrend_blatt_erlaubt` behält seinen Namen, wie der Plan es sagt.

**Ein Feld ist ersatzlos entfallen, das der Plan nicht nennt.** `Vorgang::konflikt_steht` verhinderte, dass ein Fortschrittsblatt hinter der Konfliktfrage aufging: an einem Fenster steht genau ein Blatt. Eine Zeile am Fuß ist kein Blatt, der Grund ist mit dem Blatt verschwunden, und ein Zustand, der nichts mehr trägt, ist der Sonderfall, den "supersimpel" ausschließt.

## Die Sonde

`osascript` darf in dieser Sitzung keine Tastatureingaben senden. Die Sonde war über die Umgebungsvariable `KRK_S16B_SONDE` geschaltet, lag ausschließlich in `crates/krk-ui/src/appkit/anwendung.rs` und bestand aus einem Zustandsautomaten auf einem 5-ms-Zeitgeber, einem `CADisplayLink` und einem Sender für synthetische Tastenereignisse. Sie ist **vollständig zurückgenommen**: `grep -rniE 'S16B_SONDE|KRK_S16|sonde_|sondentakt|sondenbild|VORUEBERGEHENDE' crates/ xtask/ resources/` liefert null Treffer.

Ein Umweg, der Zeit gekostet hat und für die nächste Sonde festgehalten sei: ein über `NSEvent.keyEventWithType_…` gebautes Ereignis mit einem Nullzeichen als `characters` erreicht den lokalen Ereignisabgriff nicht. Es entsteht ohne Fehlermeldung und verschwindet. Jede Taste braucht ihr wirkliches Zeichen, die Funktionstasten dazu die Marke `function`.

## Abnahme

```
cargo fmt --all -- --check                0
cargo build --workspace                   0, keine Warnung
cargo clippy --workspace --all-targets    0, keine Warnung
cargo test --workspace                    0, 306 Prüfungen, keine gescheitert
cargo xtask bundle                        0, signiert
```

Drei Prüfungen sind in `crates/krk-ui/src/kommandos/operationen.rs` dazugekommen: dass die Vorgangszeile den Abbruch nennt und einzeilig bleibt, dass sie beide Zahlen und den Namen des Eintrags trägt, und dass die Meldung über den zweiten Operationsbefehl die laufende Art benennt.

Prüfdaten unter `/tmp/krk-s16b` sind entfernt. Gelöscht wurde nichts, was diese Sitzung nicht selbst angelegt hat. **Eine Nebenwirkung bleibt:** die 26 Läufe des Bündels haben `~/Library/Application Support/KRK/session.toml` fortgeschrieben, weil eine Anwendung außerhalb des Messmodus ihre Sitzung schreibt. Das linke Dateifenster steht dort jetzt auf einem Ordner, den es nicht mehr gibt; KRK weicht beim nächsten Start auf das Benutzerverzeichnis aus.

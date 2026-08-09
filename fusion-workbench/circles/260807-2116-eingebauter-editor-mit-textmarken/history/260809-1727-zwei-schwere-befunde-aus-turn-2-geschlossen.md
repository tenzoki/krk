# Zwei schwere Befunde aus der Durchsicht von Turn 2 geschlossen

**Agent:** coder
**Status:** Complete
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Quelle:** `reviews/260809-1700-coderev-turn-2-der-editor-runde.md`

---

## Was geschlossen wurde

`260809-1644` (Textfläche ohne `allowsUndo`) und `260809-1646` (gehaltener Stand
ohne Normalisierung an seinen Eingängen), beide auf `_c_` mit Abschlussnotiz.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/editor.rs` — `setAllowsUndo(true)` in
  `textflaeche_bauen`; Doc-Absatz an `stand_einsetzen`, der auf den
  abgetrennten Defekt zum Rückgängigstapel zeigt.
- `crates/krk-ui/src/appkit/menue.rs` — der eine Prosasatz zur Begründung von
  `undo:`/`redo:` berichtigt.
- `crates/krk-ui/src/editormodell.rs` — `bearbeiten` wandelt seinen Stand; neuer
  privater Eingang `ersetzung_vorbereiten` für beide Ersetzungswege; Modulkopf
  um den Abschnitt "Die zwei Eingänge für fremden Text" ergänzt; drei Proben.

Nicht angefasst: `crates/krk-core/`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/ereignisse.rs`.

## Aus drei Eingängen wurden zwei, nicht einer

Die drei Stellen sind zwei verschiedene Sachen. `bearbeiten` nimmt einen ganzen
Stand aus der `NSTextView`; die beiden Ersetzungswege nehmen einen kurzen
Ersatztext aus einem Eingabefeld. Die beiden letzten teilen sich jetzt
`Editormodell::ersetzung_vorbereiten`, das zugleich den `gesucht`-Klon und die
Suchlauf-Prüfung aufnimmt, die vorher in beiden ausgeschrieben standen.

Auf **einen** Eingang gehen sie nicht, und der Grund ist gemessen statt
vermutet. Die Wandlung an die Zuweisung `self.stand = …` zu setzen, wie der
Defekt vorschlug, bricht `treffer_ersetzen`: `suche::einen_ersetzen` liefert den
nächsten Treffer als Byteversatz in den Stand, den es eben gebildet hat, und
eine Wandlung danach verschiebt jeden Versatz dahinter. Gegenprobe am
260809-1725 mit eingesetzter Bauform: `ein_ersatztext_mit_crlf_kommt_in_gehaltener_form_an`
fällt mit `left: None, right: Some(9)`. Der Durchgang bliebe kommentarlos
stehen. Die Gegenprobe ist danach zurückgenommen worden; der Grund steht im
Modulkopf von `editormodell.rs` und in der Abschlussnotiz des Defekts.

## Wie belegt ist, dass ein CRLF nicht mehr auf die Platte kommt

Drei neue Proben in `editormodell.rs`:

- `ein_eingefuegtes_crlf_landet_nicht_auf_der_platte` — gibt über `bearbeiten`
  einen Stand mit `\r\n` herein, sichert, liest die Datei von der Platte zurück
  und prüft sie auf `\r`. Das ist das Ende, an dem die Entscheidung des Nutzers
  vom 260808-0043 zählt.
- `ein_ersatztext_mit_crlf_kommt_in_gehaltener_form_an` — der eine
  Ersetzungsweg, zugleich die Probe, die die Reihenfolge festhält.
- `das_sammelersetzen_wandelt_seinen_ersatztext_ebenfalls` — der andere.

## Die vier Abnahmekommandos

Grün, aber in einem abgetrennten Baum gemessen. Im Arbeitsbereich hat ein
parallel laufender Schritt `crates/krk-core/src/tasten/parser.rs` in einem
Zwischenstand hinterlassen, der nicht übersetzt (`E0599 kennung`, `E0560
zeichen`); die Datei ist für jenen Schritt reserviert. Gemessen wurde deshalb
auf `HEAD` plus allein den drei Dateien dieses Schrittes:

```
cargo build --workspace           Finished
cargo test --workspace            15 Läufe, alle ok, 0 failed
cargo clippy --workspace --all-targets -- -D warnings   Finished
cargo fmt --all --check           sauber
```

Vor dem fremden Zwischenstand liefen dieselben vier Kommandos auch im
Arbeitsbereich selbst grün.

## Abgetrennt und offen geblieben

- `issues/260809-1727_o_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`
  — `setString:` schreibt an der Rückgängigverwaltung vorbei; seit `allowsUndo`
  an ist, kann ein Stapel der vorigen Datei stehenbleiben. Heute unerreichbar,
  weil `stand_einsetzen` nur einen Aufrufer hat. Braucht möglicherweise eine
  Zeile in `crates/krk-ui/Cargo.toml` für `NSUndoManager`, und die lag außerhalb
  des Umfangs.
- `issues/260809-1728_o_der-modulkopf-von-datei-rs-nennt-den-groesseren-der-beiden-eingaenge-nicht.md`
  — Prosa-Berichtigung in `krk-core`, reserviert.

## Der Entscheidungsdatensatz bleibt auf "beantwortet"

`decisions/260808-0021_a_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`
ist **nicht** auf `_i_` gezogen worden. Die Zusage hält jetzt am Modell, aber
der Sicherungsbefehl selbst kommt erst mit S25; solange kein Nutzer sichern
kann, ist die Antwort nicht in der laufenden Anwendung eingelöst. Der Schritt,
der S25 baut, zieht den Datensatz und zitiert dabei seinen Commit.

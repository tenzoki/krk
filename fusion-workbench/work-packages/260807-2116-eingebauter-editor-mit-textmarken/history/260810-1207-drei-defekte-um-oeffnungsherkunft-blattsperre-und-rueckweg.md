# Drei Defekte um die Öffnungsherkunft, die Blattsperre und den Rückweg aus der Textfläche

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Datum:** 260810-1207

---

## Auftrag

Die letzten drei offenen Defekte des Editor-Circles behandeln, deren Behebung in
`crates/krk-ui/src/appkit/` liegt. Dateigrenze: `appkit/editor.rs`,
`appkit/anwendung.rs` und, falls Punkt 2 es verlangt, die Stelle der fünf
Blätter. `krk-core/**`, `editormodell.rs`, `resources/**` und das Plandokument
nicht anfassen. Die vier Instanzproben in `editor.rs`, die
`MainThreadMarker::new_unchecked` benutzen, stehen lassen.

## Was getan wurde

### 1. `260810-1028` — die Herkunft eines Öffnens (behoben)

Alle vier Punkte des Datensatzes gebaut. `Oeffnungsherkunft` ist von
`anwendung.rs` nach `appkit/editor.rs` umgezogen und dort `pub`;
`Editorbereich::datei_oeffnen` nimmt sie als Pflichtargument, und
`Ausgangsmelder` trägt sie als zweites Argument zurück
(`Box<dyn Fn(Ladeausgang, Oeffnungsherkunft)>`). Damit hält der Übersetzer den
vergessenen Fall an, und zwar von jeder Stelle des Programms aus statt nur
innerhalb des Anwendungsdelegierten.

Entfallen ist das Feld `AnwendungsIvars::editor_aus_sitzung` samt seinem
Doc-Kommentar; `editorausgang_behandeln` bekommt die Angabe als Argument.
`editor_oeffnen_lassen` bleibt, beantwortet aber nur noch die Frage, ob es den
Editorbereich schon gibt.

Punkt 4 — die Herkunft gehört zum laufenden Ladevorgang und nicht zum Bereich —
ist mit einer Begründung beantwortet und nicht umgangen. Die
`Cell<Oeffnungsherkunft>` in `EditorIvars` ist nicht die Bauart aus
`260810-0418`: dort schrieb sie ein Aufrufer und jeder Ausgang las sie, hier
schreibt sie jedes `datei_oeffnen` und jedes `melden` liest sie. Dass die
zurückgehaltene Datei aus C4 keine zweite Angabe daneben braucht, ist am Code
geprüft — siehe Punkt 2.

Zusätzlich trägt `Oeffnungsherkunft::ist_aus_sitzung` die eine
Fallunterscheidung über die beiden Werte, vollständig und ohne Auffangzweig. Der
vorige `==`-Vergleich hätte einen dritten Anlass still als `Befehl` behandelt.

### 2. `260810-1102` — ein Befehl während der Nachfrage aus C4 (widerlegt)

Der Datensatz war mit `inference:` gekennzeichnet, und die Nachprüfung hat ihn
nicht bestätigt. Sein Schritt 2 setzt voraus, dass ein F4 während des stehenden
Blattes einen Öffnungsbefehl ausführt. Das lässt
`Anwendungsdelegierter::kommando_ausfuehren` (`anwendung.rs:2035`) nicht zu: es
weist jedes Kommando ausser dem Abbruch ab, solange
`NSWindow::attachedSheet` ein Blatt meldet
(`kommandos::operationen::waehrend_blatt_erlaubt`). Ein getipptes Zeichen hält
`eingabe_ausfuehren` an derselben Frage an. Ein zweiter Weg an dieser Stelle
vorbei besteht nicht: die Menüleiste führt keinen Befehl, der eine Datei in den
Editor bringt.

Die Beobachtung des Datensatzes über `ersthelfer_gehoert_appkit` war für sich
richtig; falsch war die Folgerung, weil der Vorbehalt im Abgriff die Frage
beantwortet, wem die Taste gehört, und nicht die, welcher Befehl zulässig ist.

**Der eigentliche Defekt lag im Modulkopf von `appkit/ereignisse.rs`**, der den
Vorbehalt als Erben aller fünf Blätter beschrieb und die zweite Sperre nicht
nannte. Er ist berichtigt: der Kopf trennt jetzt die beiden Fragen, nennt beide
Stellen namentlich und nennt diesen Datensatz als den Fehlschluss, der aus der
alten Formulierung entstanden ist. Das ist eine Überschreitung der Dateigrenze
und im Bericht ausgewiesen; ohne sie führt der nächste Leser denselben Defekt
erneut.

Eine engere Spanne bleibt offen und ist als eigener Datensatz abgelegt:
`260810-1207_o_die-spanne-zwischen-dem-schliessen-des-blattes-und-seiner-antwort-ist-ungemessen.md`.

### 3. `260809-2322` — der ganze Stand je Tastendruck (gemessen und angenommen)

Von den beiden offenen Wegen ist der zweite gewählt: der Datensatz ist mit
Zahlen geschlossen statt halb umgebaut. 92 ms je Anschlag bei 19 MB, davon 96
Prozent in `self.ivars().text.string().to_string()`, also im Umschreiben aus
UTF-16; von einigen Megabyte an stockt das Tippen sichtbar, an der
Editorgrenze von 16 MB bei rund 75 ms je Anschlag.

Die Zahlen stehen jetzt im Doc-Kommentar von
`Editorbereich::text_zurueckschreiben` unter einer eigenen Überschrift, samt der
Grössenordnung, ab der es fühlbar wird, und samt dem einen Weg, der sie senken
würde.

Zwei Gründe gegen die Senkung, und beide sind belegt statt behauptet. Der
inkrementelle Rückweg über `editedRange` verlangt Änderungen an
`Editormodell::bearbeiten` und an `krk-core/src/text/datei.rs`, beide ausserhalb
der Dateigrenze, und wäre der zweite Eingang für fremden Text, den der Modulkopf
von `editormodell.rs` ausschliesst. Ein billigeres Umschreiben an derselben
Stelle gibt es nicht: `NSString::to_string` geht in `objc2-foundation` 0.3.2
über `UTF8String` (`src/util.rs:33-46`), und jeder Zugriff auf die Zeichen selbst
braucht `unsafe`, das `krk-ui` allein in `appkit/mod.rs` trägt.

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt --all --check` | exit 0 |

## Geänderte Dateien

- `crates/krk-ui/src/appkit/editor.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/ereignisse.rs` (nur Modulkopf; Grenzüberschreitung,
  begründet unter Punkt 2)

## Nicht angefasst

`krk-core/**`, `crates/krk-ui/src/editormodell.rs`, `resources/**`, das
Plandokument und die vier Instanzproben in `editor.rs`, die über
`MainThreadMarker::new_unchecked` den Hauptfaden behaupten (offene Entscheidung
`decisions/260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`).

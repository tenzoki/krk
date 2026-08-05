# S13c: das Menü "Bearbeiten" und die Kürzel des Hauptmenüs aus der Belegung

**Status:** Complete
**Agent:** coder
**Datum:** 260805-0753
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 13c.`
**Spec:** `planning/260802-1036_o_spec-navigator-geruest.md`, C2 und C3
**Entscheide:** `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`, `decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`

---

## Was umgesetzt ist

`cargo test --workspace` ist wieder grün, zum ersten Mal seit S13b. Der Parser kennt `gehalten_von`, und damit liest sich die Auslieferungsbelegung wieder ein.

### Die Zustellerregel an vier Stellen

Zwei Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen und denselben Zusteller haben. In `crates/krk-core/src/tasten/belegung.rs`:

1. `Belegung::konflikte` vergleicht neben der Kombination den Zusteller. Über diesen Aufruf läuft `Belegung::bauen` und damit jedes Einlesen.
2. `Belegung::zuweisen` tut dasselbe für die Umbelegung durch den Nutzer.
3. `Belegung::nachschlag` überspringt jede Funktion mit `gehalten_von`. Ohne diese Stelle hinge das Verhalten an der Reihenfolge der Einträge in der Datei des Nutzers.
4. `Funktion::kommando` liefert `None`, sobald `gehalten_von` gesetzt ist. **Diese Stelle nennt der Plan nicht.** Ohne sie hinge die Zusage "eine vom Menü gehaltene Funktion liefert kein Kommando" daran, dass `Kommando::KENNUNGEN` die vier Textbefehle zufällig nicht führt — eine Beobachtung über den heutigen Bestand, keine Regel. Messbar wird sie an `fenster_schliessen`, der einzigen Funktion, die seit heute zugleich ein Kommando hat und in einer Nutzerdatei ein `gehalten_von` tragen könnte.

Dazu der Rückweg: `Eintrag` trägt das Feld mit `#[serde(default, skip_serializing_if = "Option::is_none")]`, und `impl From<&Belegung> for Belegungsdatei` reicht es durch. Ohne das schriebe `Belegung::sichern` eine `keymap.toml`, die KRK beim nächsten Start als widersprüchlich abwiese.

`Kommando::FensterSchliessen` ist neu in der Aufzählung und in `KENNUNGEN` (39 auf 40 Einträge).

### Das Menü

`crates/krk-ui/src/appkit/menue.rs` ist weitgehend neu geschrieben. `hauptmenue` bekommt die Belegung gereicht und holt das Kürzel jedes Eintrags unter dessen Kennung aus ihr. Drei Untermenüs: KRK, Bearbeiten (vier Standardeinträge mit Ziel `nil`), Fenster.

Die Übersetzung zwischen einer `Kombination` und dem AppKit-Paar aus Zeichen und Modifikatormaske steht an genau einer Stelle und geht über die Tastentabelle aus `parser.rs`. Sie trägt beide Richtungen: hin für den Aufbau, zurück für das Protokoll. Drei Regeln, keine Liste von Sonderfällen — ein einbuchstabiger Tastenname ist sein eigenes Zeichen, `f1` bis `f12` werden aus `NSF1FunctionKey` gerechnet, die vierzehn übrigen Namen tragen die Zeichen aus `NSEvent.h`. Zwei Prüfungen halten das zusammen: jede Taste der Tabelle hat ein Kürzel, und keine zwei teilen sich eines.

"Fenster schließen" trägt den eigenen Selektor `fensterSchliessen:` am Anwendungsdelegierten; der ruft `performClose:` am Fenster. `performClose:` steht in `menue.rs` nur noch in der Prosa des Modulkopfs.

Die Belegung wird jetzt in `starten` geladen und dem Delegierten gereicht, statt ein zweites Mal in `tastenabgriff_einrichten`. Zwei Abnehmer, eine Quelle.

### `--menue-protokoll`

Neue Befehlszeilenmarke in `main.rs`, an derselben Stelle wie `--tasten-protokoll`. Sie schreibt jeden Eintrag des Hauptmenüs mit Beschriftung, Kombination in der Schreibweise der Belegungsdatei, rohem AppKit-Paar, Zweitform-Kennzeichen, Verdeckt-Kennzeichen und Selektor auf die Standardausgabe und beendet.

**Ein Zusatz gegenüber dem Plan:** vor dem Auslesen ruft die Marke `anwendung.finishLaunching()`. Ohne diesen Aufruf sieht sie nichts als den eigenen Programmtext. Gemessen mit einer Sonde, die `performClose:` vorübergehend wieder eintrug: vor `finishLaunching` stand das Fenstermenü mit zwei Einträgen da, danach mit dreien, der dritte "Close All" auf Opt+Shift+Cmd+W mit dem Selektor `closeAll:`. Die Sonde ist vollständig zurückgenommen. Ein Fenster öffnet die Marke nicht, weil der Anwendungsdelegierte zu diesem Zeitpunkt noch nicht gesetzt ist.

### Nicht im Plan: die Systemzusätze am Menü "Bearbeiten"

Die beiden `Info.plist`-Schlüssel aus S13b greifen nicht. `menue.rs` trägt deshalb `systemzusaetze_unterdruecken`, die dieselben zwei Namen über `registerDefaults:` in die unterste Vorgabenebene stellt. Einzelheiten und Messung im Defekt, siehe unten.

---

## Was gemessen ist

Vier Abnahmekommandos, alle mit 0: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`, `cargo fmt --all --check`. Der Testlauf zählt 13 Testprogramme mit zusammen 348 Prüfungen, davon 0 gescheitert und 1 übersprungen.

Am laufenden, signierten Bündel am 260805-0753, über die Bedienungshilfen gesendet und ausgelesen:

| Zusage | Befund |
|---|---|
| Cmd+V in der Pfadeingabe | Feld von `/Users/k1/Projects/productive/fusion` auf `/tmp/krk-s13c-nachweis`; nach Return steht der Ordner als aktiver Tab in `session.toml` |
| Cmd+X, Cmd+C, Cmd+A ebendort | Cmd+A und Cmd+X: Feld leer, Zwischenablage trägt den Pfad. Cmd+A und Cmd+C: Zwischenablage trägt den Pfad, Feld unverändert |
| Cmd+V im Dateifenster | Menüeintrag "Einfügen" dort `aktiv=false`, ebenso die drei anderen; im Textfeld alle vier `aktiv=true` |
| Cmd+A im Dateifenster | markiert weiter alle Einträge, abgelesen an der Vorschau des Stapel-Umbenennens: "3 Einträge im Stapel umbenennen" |
| "Close All" | im Menü "Fenster" der laufenden Anwendung nicht mehr vorhanden; genau zwei Einträge, "Fenster einblenden" (Cmd+N) und "Fenster schließen" (Shift+Cmd+W) |
| Ctrl+Cmd+Leertaste | kommt nicht vor; "Emoji & Symbols" und "Start Dictation…" sind weg |
| Fenster schließen und einblenden | über die Taste und über den Menüklick je 1 → 0 → 1 Fenster |

Der `inference:` des Plans, die Zweitform hänge allein an `performClose:`, ist damit **nachgemessen und bestätigt**: mit `performClose:` erschien "Close All", mit `fensterSchliessen:` nicht.

---

## Was offen bleibt

**Ein Abnahmekriterium ist nicht erfüllt.** "Der Diff zeigt, dass `menue.rs` keine Kombination mehr als Zeichenkette festlegt" gilt für sechs der sieben Menüeinträge. Der siebte, "KRK beenden", trägt Cmd+Q, und `resources/default-keymap.toml` führt dafür keine Funktion. S13b hat fünf Kürzel nachgetragen und den sechsten übersehen. Die Datei gehört dem `ontocoder`; dieser Schritt durfte sie nur lesen. `menue.rs` trägt deshalb die Konstante `NOTBEHELF_BEENDEN` mit dem Wert `"cmd+q"`, benannt und begründet an Ort und Stelle.

Drei neue Defekte:

- `issues/260805-0753_o_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste.md` — `ontocoder`: ein sechster Eintrag `beenden` mit `gehalten_von = "menue"`.
- `issues/260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` — `coder`: derselbe Mechanismus wie bei "Close All", am Eintrag "KRK beenden" noch offen. Am laufenden Bündel gemessen. Der Defekt nennt zugleich, dass `--menue-protokoll` diese Zweitform nicht sieht, weil sie erst in der laufenden Anwendung entsteht.
- `issues/260805-0753_o_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md` — `ontocoder`: die beiden Schlüssel in `resources/Info.plist` sind seit heute wirkungslos und stehen neben der Stelle, die die Sache trägt.

Geschlossen: `issues/260804-1309_c_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`, mit der Messung als Beleg.

Auf `_i_` gezogen: `decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`. Die `Implemented:`-Zeile lässt den Hash offen.

Zwei Hinweise für die Abstimmung:

- `decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` trägt weiter `<offen — S13b und S13c>` und wartet auf die Abnahme von S13b.
- `issues/260805-0637_o_das-abnahmekriterium-von-s13b-verlangt-einen-gruenen-test-den-erst-s13c-gruen-macht.md` ist sachlich erledigt: der Lauf ist grün. Der Datensatz gehört S13b, deshalb hat dieser Schritt ihn nicht angefasst.
- `resources/default-keymap.toml` verweist im Kopfkommentar auf den Entscheid unter seinem alten Marker `_a_`. Ein Fall für den vorhandenen Datensatz `issues/260805-0000_o_zehn-verweise-in-spec-und-plan-tragen-einen-ueberholten-marker.md`.

---

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/tasten/belegung.rs` | Zustellerregel an vier Stellen, Feld `gehalten_von` in `Funktion` und `Eintrag`, Rückweg über `Belegungsdatei`, `Kommando::FensterSchliessen` |
| `crates/krk-core/tests/belegung.rs` | sechs neue Prüfungen im Abschnitt "Der Zusteller (Schritt 13c)"; `die_ab_werk_freien_kombinationen_kommen_nicht_vor` verliert `cmd+c` und `cmd+v` |
| `crates/krk-ui/src/appkit/menue.rs` | Menü "Bearbeiten", Kürzel aus der Belegung, die eine Übersetzung zum AppKit-Paar, `--menue-protokoll`, `systemzusaetze_unterdruecken`, fünf Prüfungen |
| `crates/krk-ui/src/appkit/anwendung.rs` | Selektor `fensterSchliessen:`, `fenster_schliessen`, Belegung als Ivar, `starten` mit `menue_protokoll` |
| `crates/krk-ui/src/main.rs` | Marke `--menue-protokoll` |

`resources/default-keymap.toml` und `resources/Info.plist` wurden nur gelesen. Nicht angefasst: `crates/krk-bench/`, `xtask/`, Plan und Spec.

Nicht committet, wie beauftragt.

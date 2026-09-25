# Die Sprache des Bündels und die Pfadzitate der Auslieferungsdaten (D8, Turn 25)

**Agent:** ontocoder
**Status:** Complete
**Quellen:**
- `issues/260806-1215_*_der-groessenformatierer-schreibt-nicht-nur-null-sondern-jede-byte-angabe-auf-englisch.md`
- `issues/260806-1320_*_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`

**Zum Stilprofil:** `fusion-rules ontocoder` gab allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`. Für diesen Bericht gilt deshalb kein Langform-Schreibprofil; das Fehlen ist hier vermerkt, wie `rules/agent-setup.md` es verlangt.

---

## Teil 1: `CFBundleLocalizations` in der Bündelbeschreibung

`resources/Info.plist` führt jetzt `CFBundleLocalizations` mit `de` an erster und `en` an zweiter Stelle. Ohne den Schlüssel war KRK für Foundation ein englisches Programm, unabhängig von der Systemeinstellung.

**Gemessen** an einem Foundation-Programm mit demselben `NSByteCountFormatter` und demselben `CountStyle::File`, den `crates/krk-ui/src/appkit/tabelle.rs:426` und `crates/krk-ui/src/appkit/vorschau.rs:212` anlegen. Das Programm lief in einem Bündel, dessen `Info.plist` **wörtlich die von `cargo xtask bundle` erzeugte** ist, einmal mit dem Stand vor der Änderung (`git show HEAD:resources/Info.plist`) und einmal mit dem danach:

| Bytes | vorher | nachher | Tabelle des Defekts |
|-------|--------|---------|---------------------|
| 0 | `Zero KB` | `0 KB` | `0 KB` |
| 1 | `1 byte` | `1 Byte` | `1 Byte` |
| 512 | `512 bytes` | `512 Byte` | `512 Byte` |
| 999 | `999 bytes` | `999 Byte` | `999 Byte` |
| 1.000 | `1 KB` | `1 KB` | `1 KB` |
| 10.240 | `10 KB` | `10 KB` | `10 KB` |

`Bundle.main.preferredLocalizations` wechselt dabei von `["en"]` auf `["de"]`, `Bundle.main.localizations` von `[]` auf `["de", "en"]`. Alle sechs Werte treffen die Spalte, die der Defekt am 260806-1210 vorhergesagt hat.

### Prüfung 1: `xtask` trägt die Datei unverändert, die Versionsersetzung bleibt heil

`xtask/src/bundle.rs:200-210` ersetzt den Platzhalter über `str::replace` und nicht über einen Plist-Parser; `xtask/src/bundle.rs:241-250` liest `CFBundleExecutable` über eine Textsuche, die am Schlüsselnamen ansetzt. Ein `<array>` vor dem Schlüssel `CFBundleExecutable` berührt weder das eine noch das andere.

Am gebauten Bündel nachgeprüft: `diff resources/Info.plist target/KRK.app/Contents/Info.plist` meldet **genau eine** abweichende Zeile, nämlich `__KRK_VERSION__` gegen `0.1.0`. `plutil -p target/KRK.app/Contents/Info.plist` zeigt `CFBundleLocalizations` als Feld mit `de` an Stelle 0 und `en` an Stelle 1, `CFBundleShortVersionString` als `0.1.0` und `CFBundleExecutable` als `krk`. Die Datei ist nach `plutil -lint` und nach `xmllint` gültig.

### Prüfung 2: welche andere Foundation-Ausgabe sich mitändert

KRK hat keine eigenen `.lproj`-Ordner; der Schlüssel sagt Foundation allein, welche Sprachen das Programm anbietet. Durchgegangen sind die vier Stellen, an denen KRK Foundation etwas beschriften lässt:

| Ausgabe | Ändert sich? | Nachweis |
|---|---|---|
| Größenspalte, Metadatenzeilen der Vorschau, fünfter Rang der Statuszeile (`NSByteCountFormatter`) | **ja, und das ist der Zweck der Änderung** | die Tabelle oben |
| Spalte „Änderungsdatum" (`NSDateFormatter`, `ShortStyle`/`ShortStyle`) | **nein, sie war schon vorher deutsch** | gemessen `02.02.26, 03:40` vor wie nach der Änderung; `NSDateFormatter` folgt `NSLocale.current` (hier `de_DE`, aus der Systemregion) und nicht `preferredLocalizations` |
| Standardknöpfe der Blätter und des Hinweisfensters | **nein** | KRK setzt jede Beschriftung selbst: `appkit/blaetter/mod.rs:346` in der Schleife über die Schaltflächen, `appkit/hinweis.rs:69` mit `ns_string!("OK")`. Die drei Aufrufer `loeschbestaetigung.rs:44`, `konflikt.rs:64` und `uebersprungen.rs:29` reichen alle eigene Titel durch; ein Blatt ohne Schaltfläche gibt es nicht, und damit kommt nie eine von AppKit lokalisierte Vorgabe zum Zug |
| Hauptmenü | **nein** | `--menue-protokoll` liefert vor und nach der Änderung dieselben sieben Zeilen. KRK baut das Menü von Hand, und die drei Systemzusätze stellt `menue::systemzusaetze_unterdruecken` ab. `setWindowsMenu:`, `setServicesMenu:` und `setHelpMenu:` ruft KRK nirgends, also füllt AppKit kein Menü selbst |

**Eine fünfte Stelle, die der Defekt nicht nennt, ändert sich mit.** `crates/krk-ui/src/appkit/papierkorb.rs:58` reicht `NSError::localizedDescription` aus `trashItemAtURL:` wörtlich in die Statuszeile durch. Gemessen an einem Pfad, den es nicht gibt:

```
vorher:  The file “krk-messung” doesn’t exist.
nachher: Die Datei „krk-messung“ existiert nicht.
```

In einem deutschen Programm ist das erwünscht. Es steht hier, damit es niemanden überrascht, der die Meldung in einem Bericht wiederfindet.

`inference:` Von AppKit gestellte Oberflächentexte, die keine Messstrecke erreicht, kommen ebenfalls deutsch — das Kontextmenü eines Textfeldes ist der Fall, der dem Nutzer am ehesten begegnet. Gemessen ist er nicht, weil er ein laufendes Fenster und einen Rechtsklick braucht.

---

## Teil 2: die Pfadzitate der beiden Auslieferungsdateien

Alle Zitate von Workbench-Pfaden in `resources/default-keymap.toml` und `resources/default-settings.toml` tragen jetzt die Sternform an der Markerstelle, also `260806-1054_*_belegungsansicht-…`.

**Es sind 13 Zitate, nicht 14.** Der Defekt nennt „14 Zitate auf 13 Zeilen"; gezählt über `grep -o` sind es 13 auf 13 Zeilen, elf in `default-keymap.toml` und zwei in `default-settings.toml`. Eine Zeile mit zwei Zitaten gibt es nicht. Sechs der elf Zitate in der Belegungsdatei stehen über zwei Zeilen, mit dem Verzeichnisteil am Ende der einen und dem Dateinamen am Anfang der nächsten; das ist vermutlich die Quelle der Differenz.

**Zehn der 13 Zitate waren beim Anfassen bereits veraltet** — der Grund, aus dem es die Sternform gibt:

| Zitiert als | Steht heute als | Vorkommen |
|---|---|---|
| `260805-1623_a_taste-und-einstellbarkeit-des-terminal-befehls` | `_i_` | 2 (beide Dateien) |
| `260802-0842_a_f-tasten-unter-macos-systembelegung` | `_i_` | 1 |
| `260802-0842_a_loeschen-papierkorb-oder-endgueltig` | `_i_` | 1 |
| `260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben` | `_i_` | 2 |
| `260805-0713_a_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt` | `_i_` | 2 |
| `260804-0830_a_was-die-zwischenablage-auswertung-liest` | `_i_` | 1 |
| `260805-0753_o_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q` | `_c_` | 1 |

Richtig standen nur noch das Spec-Zitat `260802-1036_o_spec-navigator-geruest` (2 Vorkommen) und `260805-0753_c_cmd-q-loest-etwas-aus-und-steht-in-keiner-tastenliste` (1 Vorkommen). **Jedes der neun zitierten Ziele existiert**, keines ist unauffindbar geworden.

### Der byteweise Vergleich aus S20 trägt weiterhin, weil es ihn nie gab

Das Abnahmekriterium von S20 (Plan, Zeile 1112) lautet: „nach dem Zurücksetzen stimmt sie mit `resources/default-keymap.toml` überein". Verglichen wird die zurückgesetzte Belegung in `~/Library/Application Support/KRK/keymap.toml`, und die entsteht über `Ablage::sichern` (`crates/krk-core/src/ablage/mod.rs:265-271`) aus `toml::to_string`. Die Serialisierung kennt keine Kommentare; der Modulkopf an derselben Stelle (`:261-264`) schreibt das ausdrücklich aus und nennt es als den Grund, aus dem `settings.toml` einen anderen Weg nimmt. Die Auslieferungsdatei besteht zu großen Teilen aus Kommentaren, die geschriebene Datei enthält keinen einzigen.

**Ein byteweiser Vergleich der beiden Dateien war damit nie möglich**, und die Formulierung des Defekts ist an dieser Stelle zu scharf. Verglichen wird die Belegung, nicht der Text: `Belegung::zuruecksetzen` (`crates/krk-core/src/tasten/belegung.rs:659-661`) setzt auf `Belegung::auslieferung()`, und das ist `toml::from_str(AUSLIEFERUNGSTEXT)`. Eine Änderung an einer Kommentarzeile kann diesen Vergleich nicht berühren. **Am Abnahmekriterium ist nichts nachzuziehen.**

### Was sich für den Nutzer ändert

`resources/default-settings.toml` wird beim ersten Start **wörtlich** nach `~/Library/Application Support/KRK/settings.toml` geschrieben, samt Kommentaren (`crates/krk-core/src/ablage/einstellungen.rs:177-183`). Die zwei geänderten Zitate stehen damit künftig auch in der Nutzerdatei. Der Test `die_auslieferungsfassung_traegt_ihre_kommentare` (`einstellungen.rs:196-210`) prüft die `mdls`-Zeile und eine Kommentarzeilenzahl über 20; beides ist unberührt.

---

## Geänderte Dateien

| Datei | Zeilen | Was |
|---|---|---|
| `resources/Info.plist` | 17–49 | Kommentarblock und `CFBundleLocalizations` mit `de`, `en` |
| `resources/default-keymap.toml` | 8, 11, 12, 61, 88, 403, 438, 475, 505, 524, 535 | elf Pfadzitate auf die Sternform |
| `resources/default-settings.toml` | 9, 11 | zwei Pfadzitate auf die Sternform |

## Abnahme

- `make check` grün: `cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`.
- `make bundle` gebaut und signiert mit „Apple Development: Kai Stalmann (FJ8U4B3QAC)", über den Schlüsselbund als einzige gültige Identität gefunden.
- `plutil -lint resources/Info.plist` und `xmllint --noout resources/Info.plist` beide gültig.
- Die sechs Byte-Werte gemessen wie oben, ohne laufendes Fenster: das Messprogramm ist ein Foundation-Programm im Scratchpad, das die vom Bündelbau erzeugte `Info.plist` wörtlich übernimmt. In den Programmtext von KRK ist dafür nichts eingebaut worden.

## Was auffiel und nicht zur Aufgabe gehört

- **Der Defekt 260806-1320 zählt 14 Zitate, gemessen sind 13.** Der Zähler ist nicht nachgeführt worden, weil der Defekt mit dieser Notiz geschlossen wird und die Zahl in der `Resolved:`-Zeile richtiggestellt ist.
- **Die Bezeichnung „byteweise" im selben Defekt trifft nicht.** Siehe oben; der Plan ist unberührt, es ist also nichts nachzuziehen, sondern nur festzuhalten.
- **`CFBundleDevelopmentRegion` fehlt weiterhin**, `Bundle.main.developmentLocalization` liefert `nil`. Für die Byte-Angaben spielt das keine Rolle, gemessen vor wie nach der Änderung. Ob der Schlüssel für die Auslieferung aus S23 dazugehört, ist eine eigene Frage und hier nicht gestellt.
- **Der Vorgängerdefekt `260805-1130_*_der-groessenformatierer-schreibt-zero-kb-auf-englisch.md` trägt bereits `_c_`**, weil er in den hier geschlossenen aufgegangen ist. Sein Abgleichsvermerk vom 260806-1647 hält fest, dass der Marker steht, das Verhalten aber auch: „Zero KB erscheint unverändert; behoben ist hier nichts." Seit dieser Änderung stimmt der zweite Halbsatz nicht mehr. Der Vermerk ist nicht angefasst worden, weil er zum Abgleichsbericht gehört und nicht zu dieser Aufgabe.

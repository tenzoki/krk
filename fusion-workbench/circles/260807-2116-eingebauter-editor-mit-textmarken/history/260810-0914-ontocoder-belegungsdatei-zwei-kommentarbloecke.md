# Die zwei Kommentarblöcke der Belegungsdatei auf den Stand nach S2 gezogen

**Status:** Complete
**Domäne:** data
**Ausführender:** `ontocoder`
**Aktiver Circle:** `260807-2116-eingebauter-editor-mit-textmarken` (`_t_`)
**Grundlage:** `issues/260810-0011_o_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md`
**Abnahme:** `cargo test --workspace` mit Rückgabewert 0. 15 Testziele, alle grün.

**Ein Hinweis zur Ausstattung dieses Laufs:** `fusion-rules ontocoder` gibt `chat-voice-de.yaml` aus, aber kein `default-voice-de.yaml`. Das Chat-Profil ist gelesen und angewandt; für die Langform dieses Berichts und der Defektdatensätze gilt die Artefaktsprache aus `CLAUDE.md`, Zeile `**Language:** de`, ohne abweichende Artefaktsprache. Derselbe Befund steht für den `coder` im Bericht vom 260810-0822 und für den `reconciler` im Abgleich vom 260810-0810.

---

## Was zu tun war

`resources/default-keymap.toml` begründete an zwei Stellen, warum die elf Editor-Kombinationen `y` und `z` meiden, und begründete es mit einem Mechanismus, den S2 abgelöst hat: KRK schlage über den virtuellen Tastencode nach, also über die Stelle auf der Tastatur, und diese beiden Stellen tauschten zwischen der deutschen und der amerikanischen Belegung den Platz. Der dritte und letzte der drei Orte, die dieselbe weggefallene Begründung führten; `260809-1527` und `260809-1746` standen beim Beginn dieser Arbeit schon auf `_c_`.

Die Schreibgrenze lautete auf `resources/default-keymap.toml` und darin auf die beiden Kommentarblöcke. Keine Belegungszeile, keine Datei unter `crates/**`, weil dort parallel andere Agenten arbeiteten.

## Die Behauptung am Code geprüft

Vor dem Umschreiben geprüft, in `crates/krk-core/src/tasten/parser.rs`:

- `Taste::kennung` (Zeilen 192–198) legt jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer Ziffer auf `Tastenkennung::Zeichen`, jeden anderen Namen auf `Tastenkennung::Code`. Eine Regel, keine Liste von Sonderfällen.
- `Kombination::aus_tastendruck` (Zeilen 569–576) schlägt nach Kennung nach und filtert bei der Stellensuche jede Taste aus, die selbst eine Zeichenkennung trägt. Über den Code gehen damit nur noch Funktionstasten, Pfeilblock und Steuertasten.
- Der Modulkopf schreibt die Begründung in seinem Abschnitt "Zwei Nachschlagarten, und warum es zwei sein müssen" aus, samt Nutzerentscheid vom 260808-0155.
- `crates/krk-ui/src/appkit/ereignisse.rs:134-142` hat denselben Sachverhalt schon gezogen und verweist für die Regel auf den Kern.

Der Entscheidungsdatensatz `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md` trägt im Dateibestand `_i_`, ist also beantwortet und umgesetzt. Die Nennung als offene Frage im ersten Block war damit doppelt veraltet.

## Was geändert ist

**Block 1, jetzt Zeilen 484–499.** Statt der Meidung von `y` und `z` sagt er, was für die Einträge dieser Datei folgt: ein einbuchstabiger Tastenname benennt die **Aufschrift** und keine Stelle, kein Eintrag wandert mit der Tastaturbelegung, keiner meidet einen Buchstaben. Die Sachaussage ist nicht ein drittes Mal ausformuliert, sondern verweist auf den Modulkopf von `parser.rs` und den benannten Abschnitt. Die Nennung als offene Frage ist zum Nutzerentscheid vom 260808-0155 geworden. Dass die elf `e`, `s`, `f`, `g`, `j` und `r` benutzen, bleibt als Beobachtung stehen, jetzt zurückgeführt auf die Systematik des Blocks darüber statt auf eine Einschränkung.

**Block 2, jetzt Zeilen 625–640.** Das Ergebnis bleibt, die Herleitung fällt weg. `cmd+z` und `shift+cmd+z` wirken an der beschrifteten Stelle, weil beide Zusteller Buchstaben über das Zeichen nachschlagen: das Menü über `NSMenuItem.keyEquivalent` (`crates/krk-ui/src/appkit/menue.rs`, `zeichen_der_taste`), der Ereignisabgriff seit S2 über das gemeldete Zeichen. Nicht mehr, weil das Menü zustellt. "Auf der Stelle kVK_ANSI_Z" ist zu "auf dem Buchstaben z" geworden, weil ein Buchstabeneintrag keine Stelle mehr benennt. Der Schlusssatz über die Anzeige- und Konfliktseite dieser Kürzel bleibt unverändert, ebenso der Absatz darüber, an dem das Rückgängig des Editors hängt.

**Beide Blöcke halten in einem Satz fest, was bis zum 260810 hier stand und woran sein Grund hing.** Dieselbe Gewohnheit, mit der der Dateikopf das Ausscheiden von Cmd+C und Cmd+V vom 260805 festhält. Der Zweck ist der Schutz davor, die gegenstandslose Regel ein zweites Mal einzuziehen.

Die Formkonventionen der Datei sind gehalten: Kommentare ohne Umlaute in der Transliteration der übrigen Kommentare, Zeilenbreite bei rund 79 Zeichen, lange Datensatzpfade über zwei Zeilen gebrochen wie an den übrigen Stellen.

## Abnahme

```
cargo test --workspace   → exit 0
```

15 Testziele, alle grün: 55, 139, 36, 42, 15, 26, 7, 5, 22, 16, 9, 308, 5 und 35 bestandene Proben, dazu ein Doc-Test-Ziel ohne Proben; ein `ignored`. Die Belegungsdatei geht über `include_str!` in den Bau, ein Formfehler hätte den Lauf angehalten. Der Lauf ist nach der letzten Textänderung wiederholt worden.

## Nebenwirkungen und ein neuer Defekt

Keine Ripple-Wirkung: Kommentare tragen keine abgeleiteten Dateien, die Zahl der Funktionen und Kombinationen im Dateikopf (71 und 79) ist unberührt, keine Belegungszeile ist angefasst.

Ein neuer Defekt ist gefunden und nicht mitbehoben, weil er außerhalb der Schreibgrenze lag: der Dateikopf derselben Datei behauptet in Zeile 42 dieselbe Prämisse ("KRK belegt den Tastencode") als allgemeine Regel. Geführt als `issues/260810-0914_o_der-dateikopf-der-belegung-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`, Schwere Low: die Schlussfolgerung jenes Absatzes über die fn-Taste trägt weiter, weil F3 eine Funktionstaste ist und weiter über den Code nachgeschlagen wird. Falsch ist allein die Reichweite der Prämisse, und die Behebung ist eine Wortänderung.

Der Defektdatensatz `260810-0011` trägt unten `---` und `Resolved:`. Die Umbenennung des Markers macht der Nutzer; kein `git add` und kein `git commit` in diesem Lauf.

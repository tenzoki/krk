# S18c: Das Terminal im angezeigten Ordner (C11)

---
**Status:** Complete
**Agent:** coder
**Datum:** 260805-1845
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 18c
**Entscheidung:** `decisions/260805-1623_a_taste-und-einstellbarkeit-des-terminal-befehls.md`

---

## Was entstanden ist

`ctrl+o` öffnet die eingestellte Terminal-Anwendung im Ordner des sichtbaren
Tabs des aktiven Dateifensters. Die Einstellung steht als Bündelkennung unter
`terminal` in `~/Library/Application Support/KRK/settings.toml`, ab Werk
`com.apple.Terminal`. Die Datei entsteht beim ersten Start aus der eingebetteten
Auslieferungsfassung und wird danach nicht mehr geschrieben.

Zwei neue Dateien, sonst Erweiterungen:

- `crates/krk-core/src/ablage/einstellungen.rs` — `Einstellungen`, die
  eingebettete Auslieferungsfassung, das Laden und die Anlage beim ersten Start.
- `crates/krk-ui/src/appkit/terminal.rs` — die eine Stelle, die eine
  Bündelkennung in einen Anwendungsort auflöst, und die Übergabe des Ordners
  über `NSWorkspace`.

## Die Anlage schreibt den eingebetteten Text wörtlich

Der `ontocoder` hatte darauf hingewiesen, und die Warnung trägt: `serde` kennt
keine Kommentare. Wäre die Anlage über `Ablage::sichern` gegangen, wie die drei
vorhandenen Dateien, stünde nach dem ersten Start eine Datei mit einer einzigen
Zeile im Ablageordner — ohne die 52 Kommentarzeilen, die das `mdls`-Kommando
nennen und damit den einzigen Weg, auf dem der Nutzer die Kennung seiner eigenen
Anwendung findet. `anlegen_falls_fehlt` schreibt deshalb `AUSLIEFERUNGSTEXT`
über denselben `atomar::schreiben`, den die drei anderen Dateien gehen; allein
die Nutzlast ist eine andere.

Nachgemessen am fertigen Bündel: `diff resources/default-settings.toml
~/Library/Application\ Support/KRK/settings.toml` ist leer, die angelegte Datei
trägt 52 Kommentarzeilen.

`Einstellungen` trägt aus demselben Grund **kein** `Serialize`. Ein
Serialisierungsweg wäre der zweite Weg zu dieser Datei und schriebe sie ohne
ihre Kommentare; ohne die Ableitung kann ihn niemand versehentlich nehmen.

## Ein vierter Wert in `Grund`

Die Anlage kann scheitern. Der Fall ist keiner der beiden, die `Grund` bisher
kannte: die Datei ist weder unlesbar noch beschädigt, sie fehlt und ließ sich
nicht schreiben. Statt einer zweiten Meldungsstelle im Kern bekommt die
vorhandene Aufzählung einen vierten Wert `NichtAnlegbar`, und `melden` bleibt
die eine Stelle, die aus einer `Ersetzung` einen Satz macht. Der Satz lautet
dann "… liess sich nicht anlegen und wird durch den Auslieferungszustand
ersetzt: …".

Nur `settings.toml` kann ihn tragen; bei den drei übrigen ist eine fehlende
Datei der erste Start und keine Meldung wert. Die Prüfung dazu entfernt den
Ablageordner zwischen Öffnen und Laden und kommt damit ohne entzogene Rechte
aus — sie läuft unabhängig davon, unter welchem Benutzer sie startet.

## Der Fokusvorbehalt hat sich selbst getragen

`Kommando::TerminalOeffnen` bekommt `Wirkungsbereich::Dateifenster`, und damit
ist C11 beantwortet. Keine Zeile Sonderbehandlung: die Zuleitung fragt den
Wirkungsbereich einmal, wie bei jedem anderen Befehl, und verwirft `ctrl+o`
stumm, solange der Fokus in der Leiste steht. Die Fallunterscheidung in
`Kommando::wirkungsbereich` hat wie zugesagt den Übersetzer als Wächter: ohne
die neue Zeile übersetzt das Kommando nicht.

Die Prüfung, die S18 als Blick voraus auf diesen Schritt in
`kommandos/fokus.rs` hinterlassen hatte, nennt jetzt das Kommando beim Namen
statt einen Stellvertreter desselben Bereichs.

## Was am laufenden Bündel geprüft ist

`osascript` darf in dieser Sitzung keine Tastatureingaben senden — der Aufruf
an "System Events" endet nach 120 s in einer Zeitüberschreitung, weil eine
Freigabe fehlt, die niemand erteilen kann. Geprüft wurde deshalb wieder mit
einer **vorübergehenden Sonde**, nach dem Muster von S13, S16, S16b und S17: ein
Zustandsautomat auf einem 400-ms-Zeitgeber in
`crates/krk-ui/src/appkit/anwendung.rs`, der synthetische Tastenereignisse über
`NSApplication.postEvent:atStart:` in die **eigene** Ereignisschlange stellt und
den Text der Statuszeile zurückliest.

Dieser Weg war hier nicht nur bequem, sondern die richtige Wahl: ein Ereignis in
der eigenen Schlange kann nicht in ein fremdes Fenster laufen. Bei S18 war genau
das einmal geschehen, und dieser Schritt öffnet Terminals.

Sechs Läufe, jeder mit frisch gebautem und signiertem Bündel:

1. **Terminal.app im angezeigten Ordner.** Fokus im Dateifenster, `ctrl+o`.
   Terminal.app startet, die Sitzung hat `cwd=/private/tmp/krk-c11-pruefung`
   (über `lsof -a -p <shell> -d cwd`). Vorher lief keine Terminal.app.
2. **Ghostty in demselben Ordner.** `terminal = "com.mitchellh.ghostty"`,
   Neustart, `ctrl+o`. Eine neue Ghostty-Sitzung mit
   `cwd=/private/tmp/krk-c11-pruefung`, Terminal.app startet nicht.
3. **Fokus in der Leiste.** `shift+cmd+l`, dann `ctrl+o`. Keine neue Sitzung,
   die Statuszeile bleibt über vier Takte leer, der Fokus bleibt in der Leiste.
   Die Auflösung der Kennung wird gar nicht erst erreicht.
4. **Unbekannte Kennung.** `terminal = "com.example.gibtesnicht"`. Nichts
   startet, KRK bleibt vorn, und die Statuszeile trägt "keine Anwendung mit der
   Bündelkennung „com.example.gibtesnicht“ installiert; settings.toml nennt sie
   unter terminal".
5. **Ordner nicht mehr erreichbar.** Angezeigter Ordner `/tmp/krk-c11-gibtsnicht`.
   Nichts startet, die Zeile trägt "… ist nicht mehr erreichbar: No such file or
   directory (os error 2)".
6. **Beschädigte `settings.toml`.** Die Vorbelegung gilt, Terminal.app öffnet
   den Ordner, die Datei bleibt unverändert liegen, und die Meldung über die
   Ersetzung steht beim Aufbau der Oberfläche in der Statuszeile.

**Die Sonde ist zurückgenommen.**
`grep -rniE 'S18C_SONDE|SONDENSCHRITT|sonde_taste|sonde_zeile|sonde_weiter|sonde_einrichten|sondeSchritt'`
über `crates/`, `xtask/`, `resources/` und das `Makefile` liefert null Treffer.
Der Prüfordner, die beiden Sicherungskopien und die beim Prüfen angelegte
`settings.toml` sind gelöscht; der Ablageordner enthält wieder allein
`session.toml`, wie vor der Sitzung. Die beim Prüfen geöffneten Terminals sind
geschlossen.

## Abnahmekommandos

`make check` fährt alle vier grün. `cargo test -p krk-core` beendet mit 0 und
deckt die vier verlangten Fälle für `settings.toml` ab, dazu zwei weitere:

| Prüfung | Fall |
|---|---|
| `alle_vier_dateien_ueberstehen_schreiben_und_wiedereinlesen` | Rundlauf im Prüfordner |
| `eine_fehlende_settings_toml_liefert_die_vorbelegung_und_entsteht_mit_kommentaren` | fehlende Datei: Vorbelegung ohne Meldung, Datei entsteht mit Kommentaren |
| `eine_kaputte_settings_toml_liefert_die_vorbelegung_und_bleibt_liegen` | kaputte Datei: Vorbelegung mit Meldung, Datei unverändert |
| `eine_settings_toml_ohne_terminal_liefert_die_vorbelegung` | Datei ohne den Eintrag `terminal` |
| `ein_unbekanntes_feld_in_settings_toml_gilt_als_beschaedigt` | Tippfehler im Feldnamen |
| `eine_nicht_anlegbare_settings_toml_meldet_sich` | die Anlage scheitert |

Die drei `grep`-Zusagen: keine `use objc2`-Zeile außerhalb von
`crates/krk-ui/src/appkit/`, `Command::new` und `process::Command` kommen
nirgends vor, die `unsafe`-Ausnahme steht weiter an genau zwei Stellen. Die
Zusage des Plans lautet `grep -rn 'Command::new\|std::process'`; sie kann nicht
aufgehen und ist als Defekt festgehalten (siehe unten).

## Angelegte Datensätze

- `issues/260805-1845_o_beim-start-liegt-der-fokus-in-der-leiste-und-nicht-im-dateifenster.md`
  — nach dem Start steht der Ersthelfer auf der Leiste, und damit wirkt bis zum
  ersten `shift+cmd+d` **kein** Befehl mit `Wirkungsbereich::Dateifenster`.
  Betrifft ein Dutzend Befehle aus C2 und C4, nicht nur C11.
- `issues/260805-1845_o_s18c-nennt-die-beschaedigte-einstellungsdatei-eine-befehlsantwort.md`
  — der Plan nennt sie zugleich Startmeldung und Befehlsantwort auf Rang 1;
  beides zusammen geht nicht.
- `issues/260805-1845_o_das-abnahmekriterium-von-s18c-sucht-std-process-und-findet-sechs-treffer.md`
  — dasselbe Muster wie die früheren `grep unsafe`-Kriterien.
- `issues/260805-1845_o_die-dateiliste-von-s18c-nennt-zwei-noetige-dateien-nicht.md`
  — `kommandos/fokus.rs` und `kommandos/mod.rs`.
- `decisions/260805-1845_o_wann-eine-von-hand-geaenderte-settings-toml-wirkt.md`
  — die Datei wird einmal beim Start gelesen; wer sie ändert, muss KRK neu
  starten. Drei Möglichkeiten, Empfehlung: bei jedem `ctrl+o` frisch lesen.

## Berührte Dateien

Neu: `crates/krk-core/src/ablage/einstellungen.rs`,
`crates/krk-ui/src/appkit/terminal.rs`.

Geändert: `crates/krk-core/src/ablage/mod.rs`,
`crates/krk-core/src/ablage/pfade.rs`,
`crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/tests/ablage.rs`,
`crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/kommandos/mod.rs`,
`crates/krk-ui/src/kommandos/operationen.rs`,
`crates/krk-ui/src/kommandos/fokus.rs`.

Nur gelesen und unverändert: `resources/default-settings.toml`,
`resources/default-keymap.toml`, `crates/krk-core/src/ablage/atomar.rs`,
`crates/krk-ui/src/tabs.rs`, `crates/krk-ui/src/appkit/statuszeile.rs`.

`crates/krk-core/tests/belegung.rs` brauchte keine Änderung: die vorhandene
Prüfung `jedes_kommando_traegt_genau_einen_wirkungsbereich` läuft über
`Kommando::KENNUNGEN` und deckt das neue Kommando von selbst mit ab, so wie der
Plan es vorgesehen hat.

## Nicht committet

Wie beauftragt. Der Commit liegt beim Orchestrator.

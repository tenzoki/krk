# S5 und S6: die zwölf Kommandos des Editors und die dreizehn neuen Funktionen

**Status:** Complete
**Agent:** coder
**Datum:** 260809-1527
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritte 5 und 6
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken

## Was umgesetzt ist

Beide Schritte zusammen, in einer Änderung. Die Probe
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` verlangt es:
eine Kennung ohne Eintrag in der Belegungsdatei lässt sie scheitern.

### `crates/krk-core/src/tasten/belegung.rs`

Zwölf neue Varianten in `Kommando`, eingeschoben zwischen `FokusVorschau` und
`BelegungAnsehen`: `Bearbeiten`, `EditorAusVorschau`, `FokusEditor`,
`EditorSchliessen`, `EditorAnsichtUmschalten`, `EditorSichern`,
`EditorZeileSpringen`, `EditorSuchen`, `EditorWeitersuchen`,
`EditorRueckwaertsSuchen`, `EditorErsetzen`, `EditorAlleErsetzen`.

`KENNUNGEN` wächst von 53 auf 65 Einträge, samt der Feldbreite in der Typangabe.

`Kommando::wirkungsbereich` bekommt sechs statt vier Gruppen:

| Kommando | Wirkungsbereich | Grund |
|---|---|---|
| `Bearbeiten` | `Dateifenster` | F4 öffnet den ausgewählten Eintrag des Dateifensters |
| `EditorAusVorschau` | `Vorschau` | der Übergang übernimmt die angezeigte Datei |
| `FokusEditor` | `Ueberall` | ein Befehl, der den Fokus holt, kann nicht voraussetzen, wo er steht |
| die übrigen acht | `Editor` | sie arbeiten in der Datei, die der Editor hält |

Drei bestehende Zweige sind umgezogen: `FensterWechseln`, `AuswahlHoch` und
`AuswahlRunter` gehen von `Ueberall` nach `Navigator`, mit der Begründung aus
Befund 3 als Kommentar über dem Zweig. Der Absatz an `Wirkungsbereich::Ueberall`
nennt den Umzug jetzt, statt weiter den alten Grund zu führen.

### `crates/krk-ui/src/belegungsmodell.rs`

`bereich_des_kommandos` bekommt die zwölf Zweige, alle mit
`Funktionsbereich::Editor`, mit dem Kommentar, warum diese Gliederung und
`Kommando::wirkungsbereich` bei `Bearbeiten` und `EditorAusVorschau`
auseinandergehen: die eine fragt nach der Gegend der Anwendung, die andere nach
dem Fokus, den ein Befehl braucht.

Die Namenszeile `"bearbeiten" => Some(Funktionsbereich::Editor)` in `bereich`
ist entfallen; der Zweig über `Kommando::aus_kennung` greift jetzt.
`text_rueckgaengig` und `text_wiederholen` sind an ihre Stelle getreten, unter
`Funktionsbereich::Textbefehle`.

Die Probe `der_f4_eintrag_ist_als_reserviert_gekennzeichnet_und_steht_im_bereich_editor`
heißt jetzt `der_f4_eintrag_traegt_seine_taste_und_steht_im_bereich_editor` und
hält fest, dass der Eintrag `F4` trägt, kein "reserviert" mehr im Text steht und
die Kennung zu `Kommando::Bearbeiten` führt.

### `resources/default-keymap.toml`

Dreizehn neue `[[funktion]]`-Blöcke, jeder mit dem Grund für seine Kombination
als Kommentar. Elf davon in einem neuen Abschnitt "Der eingebaute Editor", die
beiden Menüfunktionen im bestehenden Abschnitt der Textbefehle.

`bearbeiten` trägt `tasten = ["f4"]` und hat `reserviert_fuer = "editor"`
verloren. `lesezeichen_anlegen` heißt jetzt "Lesezeichen anlegen" statt "Ordner
als Lesezeichen anlegen", mit einem Kommentar über die zwei Sorten;
`cmd+d` bleibt.

Die Kopfzeile nennt **71 Funktionen mit zusammen 79 Kombinationen**.
Ausgezählt, nicht gerechnet:

```
$ python3 -c "…"   # ein Zähler über [[funktion]] und die Einträge in tasten
Funktionen 71 Kombinationen 79
doppelte ids: []
```

Der Abschnittskommentar über den Textbefehlen trägt den Grund, warum
`text_rueckgaengig` und `text_wiederholen` dort stehen: `undo:` und `redo:`
liegen auf dem Mac nicht im Textsystem, sondern als Menükürzel, und ohne die
beiden Einträge hätte der Editor kein Rückgängig.

### `crates/krk-core/tests/belegung.rs`

Vier neue Proben und drei nachgezogene:

- `die_drei_befehle_des_navigators_tragen_den_navigator` — der Umzug.
- `die_zwoelf_kommandos_des_editors_tragen_ihre_bereiche` — alle zwölf.
- `die_auslieferungsbelegung_fuehrt_einundsiebzig_funktionen` — die Zahl aus der
  Kopfzeile, plus die dreizehn Kennungen.
- `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen` — die y/z-Regel.
- `die_auswahl_und_der_fokuswechsel_wirken_in_beiden_bereichen` heißt jetzt
  `der_fokuswechsel_wirkt_aus_jedem_bereich_heraus`; die Auswahlbefehle sind
  daraus weg und in die Navigator-Probe gewandert, `FokusEditor` ist dazugekommen.
- `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste` heißt
  jetzt `…_und_eine_reservierte_keine_taste`, weil die Auslieferungsbelegung
  keine reservierte Funktion mehr führt; die Regel gilt für eine `keymap.toml`
  aus einer älteren Fassung weiter.
- `die_y_kuerzel_liegen_auf_kvk_ansi_y_und_die_stelle_kvk_ansi_z_ist_unbelegt`
  nimmt die vom Menü gehaltenen Funktionen aus. Siehe den Punkt unten.

## Die Stelle, an der der Plan sich widerspricht

Befund 4 und das Abnahmekriterium von S6 sagen, keine neue Kombination liege auf
`y` oder `z`. Die Tabelle in Frage 11, auf die Befund 4 sich beruft, legt
`text_rueckgaengig` auf `cmd+z` und `text_wiederholen` auf `shift+cmd+z`.

Umgesetzt ist die Tabelle. Der Grund für das Verbot trägt bei diesen beiden
nicht: KRK schlägt über den virtuellen Tastencode nach, also über die Stelle auf
der Tastatur, und genau die wandert zwischen der deutschen und der
amerikanischen Belegung. Ein Menükürzel schlägt über das **Zeichen** nach
(`NSMenuItem.keyEquivalent` nimmt eine Zeichenkette), und Befund 4 führt das
selbst aus, als Grund dafür, dass `cmd+c` und `cmd+v` überall an der
beschrifteten Stelle wirken.

Die genauere Regel folgt deshalb dem Zusteller und nicht dem Buchstaben, und die
Belegung führt den Zusteller ohnehin (`gehalten_von`). Beide betroffenen Proben
tragen die Ausnahme mit ihrer Begründung. Der Befund dazu liegt als
`issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`
und braucht eine Bestätigung des Nutzers, bevor der Plantext nachzieht.

## Was außerhalb des Umfangs mitgezogen ist

`crates/krk-ui/src/messmodus.rs`, zwei Prosastellen. Die Meldung
`NICHT_IM_VORDERGRUND` versprach dem Nutzer, im Hintergrund messe die Strecke
"nichts als L1 und L7". Das galt, solange `auswahl_runter`
`Wirkungsbereich::Ueberall` trug; mit `Navigator` ist der Fall "der Fokus liegt
nirgends" ausgeschlossen, und die Strecke misst im Hintergrund gar nichts mehr.
Die Meldung sagt das jetzt, der Doc-Kommentar darüber nennt S5 als Anlass und
S42 als den Schritt, der die Messstrecke selbst abnimmt. Dieselbe Korrektur
steht am Doc-Kommentar der Probe `im_hintergrund_beginnt_keine_messung`.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | grün |
| `cargo test --workspace` | grün, 15 Testbinäre, 0 failed |
| `cargo clippy --workspace --all-targets` | grün, keine Warnung |
| `cargo fmt --all --check` | grün |

Die Konflikterkennung aus C3 schweigt: `Belegung::bauen` macht den ersten Fund
zum Fehler, `Belegung::auslieferung()` würde beim Bauen abbrechen, und
`die_auslieferungsbelegung_ist_konfliktfrei` ist grün. Keine der vierzehn neuen
Kombinationen doppelt eine bestehende bei demselben Zusteller.

`grep -n '"bearbeiten"' crates/krk-ui/src/belegungsmodell.rs` findet die Kennung
nur noch in zwei Zeilen, beide in `mod tests`.

## Geschlossen

`issues/260808-1413_c_der-wert-navigator-ist-dokumentiert-als-truegen-ihn-schon-drei-befehle.md`
— gegenstandslos, weil der Umzug gelandet ist. Abschlussnotiz im Datensatz.

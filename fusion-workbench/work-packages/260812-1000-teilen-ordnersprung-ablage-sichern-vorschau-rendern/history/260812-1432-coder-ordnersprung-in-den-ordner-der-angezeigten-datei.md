# Schritt 3: Der Sprung in den Ordner der angezeigten Datei

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 3
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 101, 28 Fehlschläge, sämtlich `teilen`

---

## Was gebaut wurde

`opt+cmd+o` zeigt im aktiven Dateifenster den Ordner, in dem die angezeigte
Datei liegt, mit der Auswahl auf dieser Datei. Welche Datei „die angezeigte"
ist, beantwortet eine reine Funktion ohne AppKit; wer sie in einen Ordner
übersetzt, ist der Anwendungsdelegierte.

**`crates/krk-ui/src/angezeigtedatei.rs` (neu)**

Eine Funktion, `welche(vorschau_sichtbar, vorschau_pfad, editor_sichtbar,
editor_pfad) -> Option<PathBuf>`. Keine Zeile AppKit, im Zuschnitt von
`fenstertitel.rs` daneben. Der Doc-Kommentar trägt die Begründung, die der Plan
verlangt: **die Sichtbarkeit entscheidet und nicht das Halten.** Ein verdrängter
Editor behält seinen Stand, also können beide einen Pfad halten und „wer hält
eine Datei?" hat zwei Antworten; sichtbar ist nach `Bereich::teilt_flaeche_mit`
höchstens einer, und damit hat die Frage genau eine. Ohne diesen Satz baut der
nächste Leser die Abfrage auf `haelt_datei` um.

**`crates/krk-core/src/tasten/belegung.rs`**

- `Kommando::OrdnerDerDatei`, eingeordnet neben `OrdnerAufwaerts`.
  `Kommando::KENNUNGEN` wächst von 73 auf 74; die Feldbreite in der Typangabe
  hat den Bau angehalten, bis der Eintrag stand.
- `wirkungsbereich` → `Wirkungsbereich::Ueberall`, mit der Begründung am Zweig:
  die Quelle hängt nicht am Fokus, das Ziel gibt es immer. Der Doc-Kommentar der
  Funktion führte drei Befehle, die keiner ihrer sechs Regeln folgen; er nennt
  jetzt einen vierten.

**`crates/krk-ui/src/belegungsmodell.rs`**

`bereich_des_kommandos` → `Funktionsbereich::Dateilisting`, neben
`ordner_aufwaerts` und `zwischenablage_springen`, mit dem Satz, warum die Gegend
der Anwendung und nicht die Herkunft der Quelle entscheidet. Dazu die Probe
`der_ordnersprung_steht_unter_dateilisting`.

**`crates/krk-ui/src/appkit/anwendung.rs`**

Ein Zweig in `kommando_ausfuehren` auf die neue `ordner_der_datei_zeigen`. Sie
liest Sichtbarkeit aus dem Fenstermodell und die beiden Pfade aus
`Vorschaufenster::angezeigter_pfad` und `Editorbereich::pfad`, ruft
`angezeigtedatei::welche` und verzweigt über zwei Fälle: kein Pfad → die
Befehlsantwort „keine angezeigte Datei, zu der gesprungen werden könnte"; ein
Pfad → `ordner_lesen(elternteil, Some(dateiname))` am aktiven Dateifenster.
Damit ist `ordner_lesen` beim dritten Aufrufer neben dem Aufstieg aus C2 und dem
Sprung aus der Zwischenablage aus C10; ein zweiter Navigationsweg entsteht nicht.

**`crates/krk-ui/src/main.rs`**

`mod angezeigtedatei;` und der Modulkopf: aus dreizehn Modulen neben `appkit`
werden vierzehn, das neue mit einem Satz beschrieben.

## Zwei Punkte, die im Bericht stehen sollten

**Ein Pfad ohne Elternteil liest die Wurzel selbst, und der Wunschname fällt
dabei weg.** Der Plan verlangt eine Zeile und keine Meldung; sie ist ein
`match datei.parent()` mit zwei Zweigen. Dass in diesem Fall kein Name
mitgegeben wird, steht als Kommentar dabei: zu `/` gehört kein Eintrag, auf den
die Auswahl springen könnte.

**Die Probe zum Funktionsbereich prüft die Zuordnung und baut nicht die ganze
Ansicht.** Ein `Belegungsmodell::neu` läuft heute in die Panik, die `teilen`
auslöst, und die Probe wäre bis Schritt 5 rot gewesen, ohne etwas über den
Ordnersprung zu sagen. Die Zuordnung selbst prüft `bereich("ordner_der_datei")`
unmittelbar; dass die Funktion in der Ansicht erscheint, prüft die bestehende
`jede_kennung_hat_einen_funktionsbereich` mit, sobald Schritt 5 sie wieder grün
macht.

## Proben

| Probe | Kriterium |
|---|---|
| `angezeigtedatei::tests::alle_acht_kombinationen_tragen_ihre_antwort` | C2.3 |
| `angezeigtedatei::tests::ein_verdraengter_editor_mit_datei_gewinnt_nicht` | C2.3 |
| `angezeigtedatei::tests::ohne_sichtbaren_bereich_gibt_es_keine_angezeigte_datei` | C2.5 |
| `angezeigtedatei::tests::eine_verschwundene_datei_bleibt_die_angezeigte` | C2.6 |
| `belegungsmodell::tests::der_ordnersprung_steht_unter_dateilisting` | C6.1 |

Die Kennung und ihr Wirkungsbereich laufen über die bestehenden Proben in
`crates/krk-core/tests/belegung.rs` mit; alle 56 sind grün.

## Abnahme

Vier Kommandos, keine Messung, kein Bündelbau, kein Vordergrundlauf:

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 101 |

**`cargo test --workspace` ist rot, und zwar nur noch wegen `teilen`.** Vor
diesem Schritt fielen 28 Proben in `krk-ui`, jede mit einer Meldung, die
`ordner_der_datei` nannte. Danach fallen 28, und keine einzige Zeile der
Testausgabe nennt `ordner_der_datei` noch (`grep -c` liefert 0). Die 28
Meldungen lauten 26 mal „die Funktion teilen hat keinen Funktionsbereich; die
Zuordnung steht in belegungsmodell::bereich", einmal „die Funktion teilen hat
keinen Funktionsbereich" und einmal „teilen trägt weder ein Kommando noch einen
Zusteller". Schritt 5 macht sie grün. Alle übrigen Prüfziele des Workspace sind
grün, `krk-core` eingeschlossen.

Vom Kriterienblock C2 sind damit ohne laufendes Bündel nachgewiesen: C2.2 bis
C2.8. C2.1 ist zur Hälfte nachgewiesen (Zielordner und vorgemerkter Name); dass
das Dateifenster den Ordner danach wirklich zeigt, ist am laufenden `KRK.app` im
Vordergrund zu sehen und damit Nutzerarbeit.

## Datensätze

Drei Fragen sind von beantwortet auf umgesetzt gezogen, jede mit der Stelle im
Code. Der Commit steht noch aus; die Zeilen sagen es.

- `decisions/260812-1000_i_oeffnet-der-ordnersprung-einen-neuen-tab-oder-wechselt-er-den-aktiven.md`
  — der Sprung wechselt den Ordner des aktiven Tabs.
- `decisions/260812-1000_i_was-tut-der-ordnersprung-wenn-es-keinen-zielordner-gibt.md`
  — der Satz steht, und Fall 3 springt statt abzubrechen.
- `decisions/260812-1000_i_wird-die-datei-im-zielordner-ausgewaehlt.md`
  — der Dateiname geht als Wunschauswahl an `ordner_lesen`.

`decisions/260812-1000_a_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`
bleibt beantwortet: sie trägt beide Befehle, und `teilen` ist erst mit Schritt 5
in Code.

Keine neuen Defekte.

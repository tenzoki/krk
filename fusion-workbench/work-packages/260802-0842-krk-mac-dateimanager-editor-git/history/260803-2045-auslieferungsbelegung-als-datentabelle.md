# Die Auslieferungsbelegung als Datentabelle (Plan, Schritt 9)

**Datum:** 260803-2045
**Agent:** ontocoder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 9. Auslieferungsbelegung als Datentabelle`
**Neu angelegt:** `resources/default-keymap.toml`
**Nicht angefasst:** `crates/`, `xtask/`, die Plandatei, der Spec, `resources/Info.plist`, `README.md`, `CLAUDE.md`. Kein Commit; das Committen liegt beim Orchestrator.
**Vier Defektdatensätze angelegt:** unten unter `## Was am Spec und am Plan nicht aufgeht`
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen. Ein Langform-Schreibprofil gibt `fusion-rules` für den `ontocoder` nicht aus.

## Was in der Datei steht

46 Funktionen mit zusammen 52 Kombinationen. Eine Funktion trägt einen Eintrag,
auch wenn sie zwei Wege hat; das ist die Ein-Zeilen-Regel der Belegungsansicht
aus C3. Aufbau je Eintrag wie in Schritt 9 vorgeschrieben: `id`, `name`,
`tasten` und bei F4 zusätzlich `reserviert_fuer`.

| Fähigkeit | Funktionen | Was sie abdecken |
|---|---|---|
| C3 (Norton-Reihe) | 7 | die sechs Zeilen der Cmd-Kürzel-Tabelle, dazu F4 mit leerer Tastenliste |
| C1 | 5 | Tab öffnen, schließen, vor, zurück, Wechsel des aktiven Dateifensters |
| C2 | 19 | 9 Navigation, 4 Mehrfachauswahl, 6 Sortierung und versteckte Dateien |
| C4 | 4 | Datei anlegen, Umbenennen, Umbenennen im Stapel, Abbrechen |
| C5 | 7 | Lesezeichen anlegen, umbenennen, löschen, verschieben, Fokus hin und zurück |
| C7 | 4 | Leiste und zweites Dateifenster ein- und ausblenden, Bereich verbreitern und verschmälern |

C6 trägt keinen eigenen Eintrag, und das ist kein Versehen. Das Ein- und
Ausblenden der Vorschau ist nach C3 und C7 dieselbe Funktion wie "Vorschau
anzeigen" und steht als `vorschau_umschalten` bei der Norton-Reihe. Die Tabs des
Vorschaufensters bedient C6 ausdrücklich "mit denselben Befehlen wie in C1", also
mit den vier Tabbefehlen, die dort schon stehen.

C4 steht in der Tabelle, obwohl die Aufzählung in Schritt 9 nur C1, C2, C5, C6
und C7 nennt. Der Grund ist das erste Abnahmekriterium von C2: jede Funktion aus
C1 bis C7 ist über mindestens einen Tastenbefehl erreichbar. Das Umbenennen, das
Anlegen einer leeren Datei, das Umbenennen im Stapel und der Abbruch einer
laufenden Operation sind Funktionen von C4, und ohne Eintrag hier müsste sie
Schritt 15 oder 17 erfinden, womit die Belegung zwei Quellen hätte.

## Woher die Kombinationen kommen

Der Spec schreibt genau sieben Zeilen fest: die sechs der Cmd-Kürzel-Tabelle in
C3 und F4 als unbelegt. Für die übrigen 39 Funktionen sagt er "ein
Tastenbefehl", ohne einen zu nennen. Diese 39 Kombinationen sind deshalb von mir
gewählt, nicht aus dem Spec gelesen. Drei Regeln haben sie geleitet:

1. Wo der Mac für dieselbe Sache ein Kürzel kennt, steht es hier unverändert:
   `cmd+up` für den Aufstieg, `shift+cmd+g` für die Pfadeingabe, `cmd+a` für
   "alle markieren", `cmd+t` und `cmd+w` für die Tabs, `cmd+d` für das
   Lesezeichen.
2. Wo Norton Commander und Total Commander eine Form haben, die auf dem Mac
   frei ist, steht sie: `tab` für den Fensterwechsel, `space` für das Markieren,
   `shift+f6` für das Umbenennen.
3. Sonst der Anfangsbuchstabe des deutschen Verbs, wie es C3 für `shift+cmd+k`
   und `shift+cmd+v` schon vormacht: `ctrl+b` breiter, `ctrl+s` schmaler,
   `shift+cmd+u` umbenennen, `cmd+r` Sortierrichtung.

Zwei Kombinationen tragen einen Kommentar in der Datei, weil sie von der
Mac-Gewohnheit abweichen und der Grund sonst verloren geht: `shift+cmd+h` für
die versteckten Dateien statt Cmd+Umschalt+Punkt und `ctrl+b`/`ctrl+s` für die
Breiten statt der Links- und Rechts-Pfeile. Beide Male fehlt der Schreibweise
der Tastenname, siehe den vierten Defekt unten.

Das Tippen der Anfangsbuchstaben aus C2 steht nicht in der Datei. Es ist keine
Belegung, sondern der Rückfall für jede Taste ohne Zusatztaste, die keiner
Funktion zugeordnet ist; Schritt 11 legt das so fest.

## Was am Spec und am Plan nicht aufgeht

Vier Defekte, je ein Datensatz unter `issues/`:

| Datensatz | Was nicht aufgeht |
|---|---|
| `260803-2045_o_c3-nennt-f6-verschieben-und-umbenennen-die-belegungstabelle-nur-verschieben.md` | C3 nennt F6 im Abnahmekriterium "Verschieben und Umbenennen", in der Kürzel-Tabelle nur "Verschieben". C4 braucht das Umbenennen als eigene Funktion, und zwei Funktionen auf einer Kombination schließt C3 aus. |
| `260803-2045_o_abnahmekriterium-von-schritt-9-schreibt-die-kuerzel-in-einer-anderen-reihenfolge-als-die-schreibweise-erlaubt.md` | Vier der sechs Kürzel im Abnahmekriterium verletzen die Reihenfolge, die derselbe Schritt vorschreibt. |
| `260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md` | Cmd+W hat zwei Parteien, und die eine steht im Menü, wo die Konflikterkennung sie nicht sieht. |
| `260803-2045_o_die-kombinationsschreibweise-kennt-die-links-und-rechts-pfeile-nicht.md` | Die erlaubten Tastennamen decken drei naheliegende Mac-Belegungen nicht ab. |

Der zweite ist der einzige, der die Datei sichtbar von ihrem Abnahmekriterium
abweichen lässt. Die Schreibweise `[ctrl+][opt+][shift+][cmd+]<taste>` und die
sechs Kürzel `cmd+shift+k`, `cmd+shift+v`, `cmd+shift+n` und `cmd+opt+delete`
schließen einander aus. Die Datei folgt der Schreibweise und schreibt
`shift+cmd+k`, `shift+cmd+v`, `shift+cmd+n` und `opt+cmd+delete`, weil die
Schreibweise die Form ist, die der Parser aus Schritt 11 liest, und weil zwei
Reihenfolgen nebeneinander diesem Parser eine Sonderregel abverlangen würden.
Die Zuordnung Funktion zu Taste ist unverändert die der C3-Tabelle.

## Prüfung

Ein `cargo`-Programm in `/tmp` mit der Kiste `toml` hat die Datei gelesen und
ausgewertet; `python3` liegt hier in Version 3.9 vor und hat `tomllib` noch
nicht.

```
gueltiges TOML: ja
Funktionen: 46
Kombinationen gesamt: 52
Kombination bei zwei Funktionen: []
Eintraege mit leerer Tastenliste: [("bearbeiten", "editor")]
Reihenfolge verletzt: []
```

Die letzte Zeile prüft jede der 52 Kombinationen gegen
`[ctrl+][opt+][shift+][cmd+]<taste>`: kein Eintrag setzt zwei Zusatztasten in
der falschen Reihenfolge, und keiner nennt eine Zusatztaste außerhalb der vier.

Die sechs Zeilen der C3-Tabelle, abgegriffen über die Funktionstasten:

```
49-id = "vorschau_umschalten"    51:tasten = ["f3", "cmd+y"]
56-id = "kopieren"               58:tasten = ["f5", "shift+cmd+k"]
61-id = "verschieben"            63:tasten = ["f6", "shift+cmd+v"]
66-id = "ordner_anlegen"         68:tasten = ["f7", "shift+cmd+n"]
71-id = "endgueltig_loeschen"    73:tasten = ["f8", "opt+cmd+delete"]
76-id = "in_papierkorb"          78:tasten = ["delete", "cmd+delete"]
```

Die verbotenen Zeichenketten, roh über die ganze Datei gegrept:

```
shift+delete   kein Treffer
cmd+c          kein Treffer
cmd+v          64:tasten = ["f6", "shift+cmd+v"]
fn+            kein Treffer
```

Der einzige Treffer auf `cmd+v` ist die Teilzeichenkette in `shift+cmd+v`, dem
vom Spec vorgeschriebenen Kürzel für das Verschieben. Als eigenständige
Kombination kommt `cmd+v` nicht vor: `grep -cF '"cmd+v"'` liefert 0, ebenso
`"cmd+c"` und `"shift+delete"`.

## Was der nächste Schritt wissen muss

Schritt 11 bindet die Datei über `include_str!` ein und löst die verdrahtete
Tabelle aus Schritt 7 ab. Die fünf Kommandos, die dort schon stehen, heißen hier
`auswahl_hoch`, `auswahl_runter`, `seite_hoch`, `seite_runter` und `oeffnen`,
mit denselben Tasten wie in `crates/krk-core/src/tasten/mod.rs`. Die
Bezeichner sind bewusst so gewählt, dass sie auf die vorhandenen Namen der
Aufzählung `Kommando` fallen.

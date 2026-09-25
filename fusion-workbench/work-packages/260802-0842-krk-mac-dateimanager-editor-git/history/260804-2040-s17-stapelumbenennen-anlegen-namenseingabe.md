# S17: Umbenennen im Stapel, Anlegen und die Namenseingabe (C4)

---
**Status:** Complete
**Agent:** coder
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 17.`
**Spec:** `planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt `### C4`
**Datensatz:** `decisions/260802-1036_a_umbenennen-im-stapel-umfang.md`

---

## Was gebaut wurde

Zwei Gegenstände, wie der Plan sie zuschneidet.

**Das Stapel-Umbenennen.** Das Regelmodell steht in `crates/krk-core/src/umbenennen/`
mit drei Modulen: `regel.rs` hält Suchen und Ersetzen sowie die fortlaufende
Nummerierung, `vorschau.rs` rechnet je markiertem Eintrag den neuen Namen aus,
`kollision.rs` prüft ihn gegen die drei Fälle aus C4. Groß- und Kleinschreibung
ist nicht enthalten, entsprechend der Festlegung im Datensatz. Die Oberfläche
dazu ist `crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs`: vier
Eingabefelder, eine Zusammenfassungszeile und eine `NSTableView` mit den Spalten
Bisher, Neu und Hinweis. Sie rechnet nichts; sie zeigt, was der Kern
ausgerechnet hat, und rechnet bei jedem getippten Zeichen neu.

**Das Anlegen.** Ein Blatt für beide Befehle,
`crates/krk-ui/src/appkit/blaetter/namenseingabe.rs`. Es fragt einen Namen,
prüft ihn mit `name_pruefen` aus S15 und gibt ihn als `Result<String,
Namensfehler>` zurück. Was damit geschieht, entscheidet der Befehl: `f7` und
`shift+cmd+n` rufen `ordner_anlegen`, `ctrl+cmd+n` ruft `datei_anlegen`, beide
aus `operation/anlegen.rs` aus S15, die damit ihren ersten Aufrufer bekommen.

Drei Kennungen sind in `Kommando` und in `KENNUNGEN` dazugekommen:
`ordner_anlegen`, `datei_anlegen`, `umbenennen_stapel`. Die Tabelle steht jetzt
bei 39 Einträgen.

## Vier Wiederverwendungen, wie der Plan sie vorschreibt

1. `operation/anlegen.rs` und `operation/umbenennen.rs` aus S15 bleiben
   unangetastet und werden gerufen. Ein zweiter Umbenennungsweg neben dem der
   Operationsmaschine entsteht nicht: der Stapel führt `operation::umbenennen`
   je Zeile aus.
2. Die Auffrischung läuft über `auffrischung::ordner_neu_lesen` aus S14, einmal
   je Befehl, für beide Dateifenster.
3. Die Auswahl auf dem neuen Eintrag geht über die eine Stelle, die eine Zeile
   anhand ihres Namens auswählt. Sie heißt jetzt
   `DateifensterQuelle::eintrag_waehlen` und liefert drei Fälle: gewählt,
   vorgemerkt, unbekannt. `eintrag_anspringen`, der Sprung aus der
   Zwischenablage aus S13, ruft sie und meldet allein im dritten Fall.
4. Jede Meldung geht über `befehlsantwort_zeigen`, den obersten der vier Ränge
   aus der Statuszeile. Kein Zweig nach Meldungsart, keine fünfte Quelle.

## Wo der Plan seine Dateiliste ergänzen muss

Zwei Dateien fehlen in der Dateiliste von S17, und ohne beide ließe sich der
Schritt nur bauen, indem man eine andere Zusage desselben Schritts bricht.
Beide sind angefasst und als Defekt gemeldet
(`issues/260804-2040_o_die-dateiliste-von-s17-nennt-anwendung-rs-und-tabs-rs-nicht.md`).

**`crates/krk-ui/src/appkit/anwendung.rs`.** `ordner_neu_lesen` nimmt eine
`Dateifenstersicht` entgegen, und die setzt allein der Anwendungsdelegierte um.
Aus `appkit/tabelle.rs` wäre nur `DateifensterQuelle::neu_lesen` erreichbar, und
das ist der zweite Auffrischungsweg, den der Plan ausschließt: es frischt das
andere Dateifenster nicht mit auf.

**`crates/krk-ui/src/tabs.rs`.** Der Lesevorgang aus `ordner_neu_lesen` ist
gestückelt und läuft noch, wenn der Befehl zurückkehrt; der neue Eintrag steht
zu diesem Zeitpunkt in keinem Modell. Getragen wird der Name deshalb von der
`wunschauswahl` des Tabs, demselben Feld, das die Sitzungswiederherstellung, der
Aufstieg aus C2, der Sprung aus C10 und die Auffrischung aus C9 schon benutzen.
Hinzugekommen ist genau ein Setzer, `Tabliste::wunschauswahl_setzen`.

`crates/krk-ui/src/appkit/blaetter/mod.rs` steht in der Liste als "einbindend"
und ist darüber hinaus gewachsen: der `Eingabewaechter` meldet jetzt auch
Textänderungen, und `Blatt` hat drei kleine Methoden bekommen
(`ersthelfer_setzen`, `waechter_anhaengen`, `textaenderung_melden`), aus denen
das vorhandene `textfeld_setzen` neu zusammengesetzt ist. Der Grund ist die
Vorschau: sie muss bei jedem Zeichen neu rechnen, und ein zweiter Delegierter
neben dem Wächter wären zwei Wahrheiten darüber, was die Eingabetaste in einem
Blatt tut.

## Die fünf Prüffälle des Kerns

`cargo test -p krk-core --test umbenennen` läuft mit 0 und deckt sieben Fälle
ab, die fünf aus dem Abnahmekriterium und zwei dazu:

| Fall | Ergebnis |
|---|---|
| Suchen und Ersetzen über 50 Namen | bestanden |
| Nummerierung mit drei Stellen ab 7 | bestanden |
| Kollision mit einem bestehenden Eintrag | bestanden |
| Kollision zweier neuer Namen untereinander | bestanden |
| leerer neuer Name | bestanden |
| Schrägstrich im neuen Namen (zusätzlich) | bestanden |
| Ordner und Dateien gehen denselben Weg (zusätzlich) | bestanden |

## Am laufenden Bündel geprüft

Gemessen am 260804 zwischen 20:28 und 20:37 am Bündel `target/KRK.app`.
Prüfdaten unter `/tmp/krk-s17` und `/tmp/krk-s17b`, hinterher entfernt.
Gelöscht wurde nichts, was diese Sitzung nicht selbst angelegt hat.

**Anlegen**, Prüfordner mit `bild-a.jpg`, `bild-b.jpg`, `bild-c.jpg`:

| Was | Ergebnis |
|---|---|
| `f7` öffnet ein Blatt | `blatt=true` |
| Name tippen, Return | Ordner `sonde-ordner` angelegt, Auswahl darauf, Zeile "Ordner „sonde-ordner“ angelegt" |
| `shift+cmd+n` | Ordner `zweiter` angelegt, Auswahl darauf |
| `ctrl+cmd+n` | Datei `notiz` angelegt, Auswahl darauf |
| vergebener Name | nichts angelegt, Zeile "es gibt schon einen Eintrag namens „notiz“" |
| leerer Name | nichts angelegt, Zeile "der Name ist leer" |

**Stapel-Umbenennen**, dieselbe Sitzung: `cmd+a` markiert alles, `ctrl+cmd+u`
öffnet das Blatt, Suchtext `bild`, Tabulator, Ersetzungstext `Foto`, Tabulator,
`7`, Tabulator, `3`, Return. Danach:
`Foto-a009.jpg`, `Foto-b010.jpg`, `Foto-c011.jpg`, `notiz012`,
`sonde-ordner007`, `zweiter008`; Zeile "6 Einträge umbenannt", Auswahl auf dem
ersten umbenannten Eintrag. Die Nummer zählt in Sichtreihenfolge, und die
Ordner sind mit umbenannt worden.

**Die Vorschau mit allen drei Kollisionsarten**, Prüfordner mit `andere`,
`foto`, `fotoziel`, `ziel`, Regel: `foto` durch nichts ersetzen. Das Blatt
zeigte:

```
4 Einträge: 0 werden umbenannt, 3 bleiben stehen
 Bisher    │ Neu    │ Hinweis
 andere    │ andere │
 foto      │        │ der Name ist leer
 fotoziel  │ ziel   │ der Name ist schon vergeben
 ziel      │ ziel   │ zweimal derselbe neue Name
```

Alle drei Gründe stehen in Worten da und nicht nur in Rot. Zwei Sachen sind
dabei aufgefallen und behoben: bei 520 Punkten Breite schnitt die
Zusammenfassungszeile ab, und die Spalte "Hinweis" war zu schmal für die
ursprünglichen, längeren Gründe. Die Beigabe ist jetzt 580 Punkte breit, die
Spalte 240, und die beiden Gründe sind auf dreißig Zeichen gekürzt; eine
Prüfung in `kollision.rs` hält die Grenze.

**Bedienung ohne Maus.** Alle Eingaben oben liefen ausschließlich über
synthetische Tastenereignisse, keine einzige über die Maus. Vier Tabulatoren
vom Suchfeld führen in die Vorschau, und zwei Pfeiltasten bewegen dort die
Auswahl; die Bildschirmaufnahme zeigt die zweite Zeile ausgewählt. Escape
schließt das Blatt, ohne umzubenennen: der Prüfordner war danach unverändert.

**Die C4-Kriterien einzeln:**

| Kriterium | Beleg |
|---|---|
| Anlegen: zwei Tastenbefehle, im Ordner des aktiven Fensters, Auswahl danach auf dem neuen Eintrag | am Bündel geprüft, Tabelle oben |
| Stapel: ein Tastenbefehl öffnet Musterregeln mit Suchen/Ersetzen und Nummerierung mit Stellenzahl und Startwert | am Bündel geprüft, `ctrl+cmd+u`, Nummerierung ab 7 mit drei Stellen |
| Stapel: Vorschau vor der Ausführung, erst ein zweiter ausdrücklicher Befehl führt aus | am Bündel geprüft, Aufnahme der stehenden Vorschau; Escape ließ alles stehen |
| Stapel: Vorschau markiert bestehende Kollision, doppelten neuen Namen und leeren Namen mit Grund | am Bündel geprüft, alle drei in einer Vorschau |
| Stapel: wirkt auf Ordner wie auf Dateien, vollständig über die Tastatur | am Bündel geprüft, zwei Ordner und vier Dateien in einem Zug; Tabulator, Pfeiltasten, Return, Escape |
| Diff zeigt die drei Kennungen in `Kommando` | `git diff crates/krk-core/src/tasten/belegung.rs` |
| `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` läuft weiter durch | bestanden |

## Die vorübergehende Sonde ist zurückgenommen

`osascript` darf in dieser Sitzung keine Tastatureingaben senden; geprüft wurde
deshalb wie in S16b über eine Sonde. Sie war über die Umgebungsvariable
`KRK_S17_SONDE` geschaltet, lag ausschließlich in
`crates/krk-ui/src/appkit/anwendung.rs` und bestand aus einem Drehbuch auf
einem 250-ms-Zeitgeber, einem Sender für synthetische Tastenereignisse und
einer Berichtsfunktion, die Blattstand, Auswahl, Statuszeile und Ordnerinhalt
auf die Standardfehlerausgabe schrieb.

Sie ist **vollständig zurückgenommen**:
`grep -rnE 'Sonde|SONDE|sonde_|KRK_S17|activateIgnoringOtherApps' crates/ xtask/ resources/`
liefert dreizehn Treffer, alle davon "Sonderfall" und "Sonderregel" in
vorhandener Prosa sowie zwei Verweise auf die Sonde aus S7 in
`normalisierung.rs` und `ereignisse.rs`. Das Bündel ist nach der Rücknahme neu
gebaut und signiert.

## Was nicht geprüft ist

- **Ein körperlich gedrückter Tastendruck.** Alle Belege oben stammen aus
  synthetischen Ereignissen, die denselben Weg gehen wie ein echter Druck: über
  die Ereignisschlange der Anwendung in den lokalen Abgriff, durch die
  Normalisierung und den Nachschlag im Kern. Dass die Tastatur dieselben
  Ereignisse erzeugt, ist aus der Messung vom 260802-1137 belegt und nicht aus
  dieser Sitzung.
- **Ein Stapel über mehr als ein paar Dutzend Einträge im laufenden Bündel.**
  Geprüft ist er als Rechnung im Kern über 50 Namen und am Bündel über sechs.
  Die Laufzeit steht im Defekt unten.

## Eine Zusage aus C4, die der heutige Aufbau nicht hält

Das Stapel-Umbenennen läuft als gewöhnliche Schleife auf dem Hauptfaden, ohne
Fortschritt und ohne Abbruch. **Gemessen am 260804-2040:** 5.000
`rename(2)`-Aufrufe nacheinander brauchen 525 ms auf demselben
APFS-Datenträger. C4 sagt für eine Operation über mehr als 100 Einträge einen
Fortschritt und einen Abbruch zu und verlangt, dass das Fenster währenddessen
bedienbar bleibt. Über einem größeren Stapel hält der heutige Aufbau beides
nicht. Festgehalten als
`issues/260804-2040_o_das-stapel-umbenennen-laeuft-ohne-fortschritt-und-ohne-abbruch-auf-dem-hauptfaden.md`,
mit den zwei möglichen Antworten; entschieden ist er nicht.

## Nebenwirkung

Die Läufe des Bündels haben `~/Library/Application Support/KRK/session.toml`
fortgeschrieben. Das linke Dateifenster steht dort jetzt auf `/tmp/krk-s17b`,
einen Ordner, den es nicht mehr gibt; KRK weicht beim nächsten Start auf das
Benutzerverzeichnis aus.

## Geänderte Dateien

**Neu**

- `crates/krk-core/src/umbenennen/mod.rs`
- `crates/krk-core/src/umbenennen/regel.rs`
- `crates/krk-core/src/umbenennen/vorschau.rs`
- `crates/krk-core/src/umbenennen/kollision.rs`
- `crates/krk-core/tests/umbenennen.rs`
- `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs`
- `crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs`

**Geändert**

- `crates/krk-core/src/lib.rs` (`pub mod umbenennen;`)
- `crates/krk-core/src/tasten/belegung.rs` (drei Kennungen, `KENNUNGEN` 36 → 39)
- `crates/krk-ui/src/appkit/blaetter/mod.rs` (zwei Module eingebunden, Wächter
  meldet Textänderungen, drei Methoden an `Blatt`)
- `crates/krk-ui/src/appkit/anwendung.rs` (Zuleitung der drei Befehle, Anlegen
  und Stapelausführung)
- `crates/krk-ui/src/appkit/tabelle.rs` (`Auswahlversuch`, `eintrag_waehlen`,
  `alle_namen`)
- `crates/krk-ui/src/kommandos/operationen.rs` (`Anlegeart`, drei Texte, drei
  Prüfungen)
- `crates/krk-ui/src/tabs.rs` (`Tabliste::wunschauswahl_setzen`)

`resources/default-keymap.toml` wurde nur gelesen. `crates/krk-bench/`,
`xtask/`, die Plandatei und der Spec sind unberührt. Kein Defekt aus dem
Bestand wurde behoben. Committet wurde nicht.

## Gefilete Defekte

- `issues/260804-2040_o_das-stapel-umbenennen-laeuft-ohne-fortschritt-und-ohne-abbruch-auf-dem-hauptfaden.md`
- `issues/260804-2040_o_die-dateiliste-von-s17-nennt-anwendung-rs-und-tabs-rs-nicht.md`
- `issues/260804-2040_o_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen.md`
- `issues/260804-2040_o_zwei-module-des-kerns-heissen-umbenennen.md`

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo test -p krk-core --test umbenennen` | 0, sieben Fälle |

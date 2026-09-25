# `CLAUDE.md` auf den Projektstand nachgezogen

**Datum:** 260803-1321
**Agent:** coder
**Status:** Complete
**Auslöser:** Nutzerauftrag: `CLAUDE.md` beschrieb den Stand vom 260802-1130 und war seit Beginn der Implementierung an mehreren Stellen überholt.
**Geänderte Dateien:** `CLAUDE.md`
**Nicht angefasst:** `crates/`, `xtask/`, `resources/`, `README.md`, alle Dateien unter `fusion-workbench/` außer dieser Historiendatei. Nicht committet, kein Bündelbau.

## Vorgehen

Jede Behauptung der Datei wurde vor dem Schreiben am Dateibestand geprüft: der
Verzeichnisbaum per `find`, die Workspace-Mitglieder in `Cargo.toml`, die
Baukommandos in `README.md`, die Lage von `cargo` per `which` und `$PATH`, die
`unsafe`-Regeln per `grep` über `crates/`, der Ausführungsstand am Plan und die
Entscheidungsstände durch Auflisten **beider** Speicher, `shared/decisions/` und
`decisions/` des aktiven Circles.

## Hinzugefügt

**Abschnitt `## Bauen und prüfen`.** Neu, den gab es nicht. Er trägt die vier
Kommandos aus `README.md` (`cargo build|test|clippy|fmt --workspace`), den
Bündelbau `cargo xtask bundle` und drei Hinweise, die ein Agent braucht, bevor
er das erste Kommando absetzt: `cargo` liegt auf diesem Gerät unter
`$HOME/.cargo/bin` und steht **nicht** auf dem Standard-PATH (geprüft: `which
cargo` findet nichts, `$HOME/.cargo/bin/cargo --version` meldet 1.97.1);
`cargo xtask` ist kein eingebautes Kommando, sondern der Alias aus
`.cargo/config.toml`; der Bündelbau verlangt eine Signaturidentität, sucht sie
in drei Stufen und bricht ohne Bündel ab, wenn keine greift.

**Verzeichnisbaum mit dreizehn Einträgen** statt der bisherigen vier, jeder mit
einem Kommentar, wozu er da ist.

**Ausführungsstand.** 8 der 24 Planschritte tragen `[DONE]`, als nächstes steht
S8, die Frühmessung als Gate. Drei Defekte sind offen, alle drei aus der
Umsetzung von Schritt 7.

**Die `unsafe`-Grenze in einem Satz.** Beide Kisten tragen
`#![deny(unsafe_code)]`, die Ausnahme steht in genau zwei Dateien. Das ist die
Regel, gegen die ein Coder als erstes verstößt, wenn er sie nicht kennt.

**Eine Lesehilfe für Pfade.** Pfade der Form `planning/…`, `decisions/…`,
`analyses/…` und `issues/…` sind relativ zum Circle-Verzeichnis zu lesen. Ohne
sie hätte jeder Verweis den vollen 62-Zeichen-Pfad tragen müssen.

## Geändert

**`## Projektstand`** von "keinen Quellcode und keine Architektur" auf den
tatsächlichen Stand: Rust mit AppKit über `objc2`, Workspace steht, Bündel baut
und signiert, Fenster mit bedienbarer Dateiliste.

**`## Technologiewahl`** von "Offen" auf die getroffene Wahl vom 260802-1150,
mit Verweis auf den Datensatz `decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`
und die Analyse `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`.

**Die Entscheidungsstände.** Fünf offene Fragen statt drei, sieben beantwortete
oder umgesetzte statt vier. Neu offen sind die Verfügbarkeitsprüfung für
macOS-26-Schnittstellen und die Sortierfrage; neu beantwortet sind die
Technologiewahl, die L4-Lesart und die `unsafe`-Grenze.

**`## Maximen`** von der Dreierliste plus Absatz auf zwei Sätze gekürzt. Der
Verweis auf C8 und das Referenzgerät bleibt.

## Entfernt

- Der Satz "Es gibt weiterhin keinen Quellcode und keine Architektur" samt dem
  vierzeiligen Verzeichnisbaum.
- Der Absatz "Kein Build-Verzeichnis, keine Projektdatei, keine
  Abhängigkeitsdeklaration, keine Tests… kein Build-Kommando und kein
  Testkommando".
- Die Tabelle der "zwei Dokumente im aktiven Circle". Es sind inzwischen zwei
  Planungsdokumente, zwei Prüfberichte und eine Analyse; die Aufzählung wäre nur
  wieder veraltet. An ihre Stelle tritt der Verweis auf den Plan als führendes
  Dokument des Ausführungsstands.
- Der Satz "Analysen gibt es noch keine".
- Die Tabelle der beantworteten Entscheidungen mit ausgeschriebenen Fragetexten.
  Die Antworten standen ohnehin nicht darin, und der Dateibestand ist verbindlich.
- Der Absatz "Eine beantwortete Entscheidung ist damit noch nicht erledigt…".
  Die Aussage steht jetzt in der Markerliste selbst: `_a_` beantwortet aber noch
  nicht in Code umgesetzt.
- Der Satz "kein Agent wählt ein Toolkit nebenbei im Zuge einer anderen
  Aufgabe". Er hatte seinen Zweck, solange die Wahl offen war.

## Eingehalten

Die Zeile `**Language:** de` steht unverändert in Zeile 3. Der Verweis auf die
Directive im Circle-Datensatz steht mit dem Zusatz, dass die Kurzfassung nicht
die verbindliche Formulierung ist. Prosa deutsch, Bezeichner und Kommandos
englisch. Die Datei ist eine Zeile kürzer als vorher (85 statt 86) bei
gewachsenem Inhalt: 7141 Bytes gegen 7089.

## Drei Befunde für andere Agenten

Beim Abgleich sind drei Abweichungen aufgefallen, die nicht in `CLAUDE.md`
gehören und hier festgehalten sind, weil sie sonst verloren gingen.

**1. Der Plan verweist auf einen Dateinamen, den es nicht mehr gibt.**
`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt S1, nennt
`decisions/260803-1208_a_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`.
Der Datensatz trägt seit dem Commit 569e8e0 den Marker `_i_`, nicht `_a_`. Der
Verweis läuft ins Leere. Wert eines eigenen Defekts für den `reconciler`.

**2. Die Zeile `**Status:**` folgt dem Marker nicht.** Derselbe Datensatz trägt
im Kopf `**Status:** answered`, im Dateinamen aber `_i_`. Der Defekt
`issues/260803-1309_c_entscheidung-zur-unsafe-grenze-steht-noch-auf-beantwortet.md`
ist geschlossen, hat also offenbar nur den Marker gezogen und das Kopffeld
stehen lassen.

**3. Zwei beantwortete Entscheidungen sind in Code umgesetzt, tragen aber
weiter `_a_`.** `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`
und `decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md` haben beide eine
leere Zeile `Implemented:`, obwohl der Workspace in Rust mit `objc2` steht und
die Tastennormalisierung in `crates/krk-core/src/tasten/` liegt. Ob das schon
für `_i_` reicht, entscheidet nicht der `coder`; die Prüfung gehört zum
`reconciler`.

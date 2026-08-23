# Abgleich zum Abschluss der Sitzung 260823-1424

**Bereich:** `b58e9d1..7d86420`, drei Commits
**Baumstand beim Lesen:** `7d86420`, Tag `v1.0.0` auf HEAD
**Domäne:** code
**Aktiver Circle:** keiner — alle Speicher sind die gemeinsamen

---

## Ergebnis in einem Satz

Die Auslieferung der 1.0.0 hält an jeder Stelle, an der dieses Projekt eine Versionszahl führt, die
Begründung für die Hauptzahl trägt gegen den Baum, und die Berichtigung aus `db1a177` stimmt
sachlich. Drei Aussagen sind falsch oder unvollständig geworden, und keine davon steht im Code.

## Was einzeln gegen den Baum gelesen ist

### 1. Die Auslieferung

Fünfzehn Prüfungen, alle grün:

| Was | Wo gelesen | Stand |
|---|---|---|
| Version im Manifest | `Cargo.toml:13` | `version = "1.0.0"` |
| Version in der Ableitung | `Cargo.lock`, `krk-core`, `krk-ui`, `krk-bench` | dreimal `1.0.0` |
| Tag auf HEAD | `git tag --points-at HEAD` | `v1.0.0` |
| Tag auf der Gegenseite | `git ls-remote --tags origin` | `7d86420 refs/tags/v1.0.0` |
| Zweig auf der Gegenseite | `git rev-parse HEAD origin/main` | beide `7d86420` |
| Version im gebauten Bündel | `target/KRK.app/Contents/Info.plist` | `CFBundleShortVersionString = 1.0.0` |
| Beglaubigung | `target/KRK.app/Contents/CodeResources`, erste vier Bytes | `s8ch`, die Kennung aus `xtask/src/veroeffentlichung.rs:68` |
| Paket | `target/KRK-1.0.0.zip` | 6 909 679 Bytes, 23.08. 14:48 |
| Releaseseite | `gh release view v1.0.0` | kein Entwurf, keine Vorabfassung, `KRK-1.0.0.zip` als Anhang |
| Arbeitsbaum | `git status --porcelain` | sauber bis auf `.DS_Store` (siehe unten) |
| Abnahme | `make check` am Stand `7d86420` | Rückgabewert 0, „alle vier gruen" |

**Die Wettrennprobe hat gehalten.** `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist in
diesem Lauf grün geworden, das Prüfziel `text` insgesamt in 8,07 s. Der Befund `260823-1436` bleibt
davon unberührt: er behauptet keine dauerhafte Röte, sondern eine Marge, die unter Last nicht trägt.
Ein grüner Lauf widerlegt ihn nicht.

### 2. Die Versionsstufe

**Die Begründung trägt.** `README.md:384-388` führt als Major-Fall unter anderem, dass „eine Datei
unter `~/Library/Application Support/KRK/` nicht mehr gelesen wird, wie sie geschrieben wurde". Die
Kette dahin ist vollständig am Baum nachgelesen:

- `Belegung::vom_nutzer` (`crates/krk-core/src/tasten/belegung.rs:1226`) ruft `bauen` mit dem
  Wortschatz der Auslieferung.
- `bauen` (`:1420-1424`) bricht beim **ersten** unbekannten Bezeichner mit `return Err` ab. Kein
  Überspringen, kein Sammeln.
- `Belegungsdatei::from(&Belegung)` (`:1651-1677`) trägt jede Funktion mit. Eine `keymap.toml`, die
  KRK selbst geschrieben hat, führt damit notwendig auch die umbenannte Kennung.
- `laden` (`:1493-1513`) beantwortet jeden `Belegungsfehler` mit `Belegung::auslieferung()` und
  einer `Ersetzung`. Die ganze Nutzerbelegung ist damit für die Sitzung fort.

`editor_aus_vorschau` steht heute nur noch in Doc-Kommentaren und Probennamen
(`crates/krk-core/tests/belegung.rs:1820`, `:1990-1991`,
`crates/krk-ui/src/belegungsmodell.rs:1447-1448`, `crates/krk-ui/src/appkit/anwendung.rs:6264`,
`:7058`); die Kennung selbst lautet `editor_rundweg`
(`crates/krk-core/src/tasten/belegung.rs:776`).

**Die README stimmt nach dieser Auslieferung unverändert.** Weder `### Versionsstufen` noch
`### Die acht Stationen` noch der Abschnitt über den Abbruch am Arbeitsbaum trägt eine Aussage, die
die 1.0.0 falsch macht. Die Zahlen `0.2.0` und `0.5.5` in der README und in `xtask/src/` sind
Beispiele und Probenwerte, keine Standsaussagen; `version::pruefen`
(`xtask/src/version.rs:419-440`) verlangt genau drei Zahlenteile und kennt keine Sonderregel für die
Null.

**Keine Prosastelle im Baum ist dadurch falsch geworden, dass KRK die 0.x-Reihe verlassen hat.**
Gesucht mit `grep -rnE '0\.x|Nullerreihe|erste Zahl|Hauptzahl|Major'` über `README.md`,
`CLAUDE.md`, `idea.txt`, `crates/*/src`, `xtask/src`, `Makefile`, `release.sh`, `certify-only.sh`.
Die einzige Fundstelle mit normativem Gehalt ist `README.md:384`, und die trägt.

### 3. Die Berichtigung in `db1a177`

**Sie stimmt sachlich.** `Anwendungsdelegierter::kommando_ausfuehren` hat zwei Ausgänge:
`crates/krk-ui/src/appkit/anwendung.rs:3002-3004` liefert `false`, sobald
`zulaessigkeit::zulaessig` abweist, und die Funktion schließt auf `if gewirkt { … } true`.

**Der Entscheid des Nutzers vom 260823-1350 ist unberührt, und das ist geprüft und nicht
übernommen.** Die `## Frage`, die Möglichkeit 1 („liefert `true`, **sobald der Befehl die beiden
Vorbehalte passiert hat**") und die `Answered:`-Zeile („der Abgriff schluckt, was **zulässig** war")
tragen alle drei die bedingte Fassung. Keine von ihnen hat je die absolute getragen; falsch war
allein die Zusammenfassung in der `Implemented:`-Zeile.

**Es sind genau vier Codestellen**, wie `260823-1433` sagt. Nachgezählt mit
`grep -rn "kommando_ausfuehren" --include='*.rs' crates/ | grep -E "immer|ausnahmslos|jedem Fall"`
und dem Zeilennachschlag daneben: `anwendung.rs:1959`, `:6297-6298`, `:7075` und
`kommandos/rundweg.rs:120-121`. Die drei Gegenstellen (`anwendung.rs:5354-5356`,
`messmodus.rs:93-95`, `appkit/blaetter/mod.rs:304-305`) stehen ebenfalls unverändert. Alle sieben
Zeilenangaben treffen am heutigen Stand, weil seit `b58e9d1` keine Codedatei angefasst worden ist.
Der Nachtrag steht am Datensatz.

### 4. Die offenen Defektdatensätze

**Der Auftrag nennt elf im gemeinsamen Speicher; es sind zweiundfünfzig.** `ls
shared/issues/*_o_*.md | wc -l` liefert 52, dazu 108 in den Circles. Die elf des Auftrags sind die
**zuletzt abgelegten**: einer vom 260820-2056, fünf aus der Sitzung `260823-0442` und fünf aus
dieser. Die Zahl elf beschreibt also den Zulauf zweier Sitzungen und nicht den Bestand.

**Keiner der elf ist durch die Auslieferung erledigt worden**, und das folgt aus dem Bereich: die
drei Commits fassen keine Codedatei an. `7d86420` ändert `Cargo.toml` und `Cargo.lock`, `11d3b29`
das Ereignisprotokoll, `db1a177` allein Werkbankdateien.

**Einer ist zur Hälfte erledigt.** `260823-1433` hatte zwei Hälften: die `Implemented:`-Zeile des
Entscheidungsdatensatzes und vier Codestellen. `db1a177` hat die erste behoben. Der Datensatz bleibt
`_o_` und trägt jetzt den Nachtrag, welche Hälfte steht.

**Einer ist durch die Auslieferung schärfer geworden**, und er gehört nicht zu den elf:
`260813-0026` sagt, `bundle` und `release` schrieben an denselben Ort. Unter `target/KRK.app` liegt
seit dem 260823-1448 das **beglaubigte** Bündel der 1.0.0. Ein `make run` an dieser Stelle nimmt die
Beglaubigung der ausgelieferten Fassung weg. Als `Also seen:` ergänzt.

### 5. `CLAUDE.md`

**Nicht angefasst**, wie der Auftrag verlangt. Einzeln gegen den Baum gelesen:

| Aussage | Stand |
|---|---|
| „Wirkungsbereich trägt sieben Werte" | 7 — trägt |
| „`Bereich` fünf" | 5 — trägt |
| „`Fokus` fünf" | 5 — trägt |
| „für `Kommando` steht hier keine Zahl" | richtig; der Baum trägt 79 |
| „`Kommando` trägt keine einzige Git-Variante" | keine — trägt |
| „`ls circles/*/_a_circle.md` gibt nichts aus" | gibt nichts aus — trägt |
| Rundentabelle mit fünfzehn Zeilen | 10 beschränkt + 5 kohärent = 15 gefahren, 2 zurückgestellt — trägt |
| „die meisten gefahrenen Runden sind beschränkt geschlossen" | 10 von 15 — trägt |
| Untergrenzen-Abschnitt in jeder `appkit/`-Datei außer `koordinaten.rs` und `mod.rs` | 39 von 41, genau diese zwei fehlen — trägt |
| „genau eine Hülle um `NSPasteboard`" | `appkit/zwischenablage.rs`; `vorschau.rs:448` nimmt eine hereingereichte Ablage entgegen, wie die Datei es beschreibt — trägt |
| die vier Absätze zur Auslieferungskette (`./release.sh`, acht Stationen, `certify-only.sh`, `veroeffentlichen` ohne Hülle) | keiner ist durch die drei Commits berührt; `shared/decisions/260821-1115_o_*` steht weiter offen — tragen |
| „seit dem 260815 ist sie an jedem Tag mindestens einmal gestiegen" | **falsch** — siehe unten |

**Eine Aussage ist falsch, und die drei Commits haben sie nicht verursacht.** Am 2026-08-22 steht
kein Tag, und `git log` liefert für diesen Tag null Commits. Der Satz ist am 260822 durch
Unterlassung falsch geworden und wird durch die Auslieferung am 260823 nicht wieder wahr. Abgelegt
als `shared/issues/260823-1649_o_*`.

## Was neu abgelegt ist

| Datensatz | Kern |
|---|---|
| `shared/issues/260823-1649_o_claude-md-sagt-die-version-sei-seit-dem-260815-an-jedem-tag-gestiegen-am-260822-ist-sie-es-nicht.md` | Tagbestand widerlegt die Reihenaussage; Schwere Low |
| `shared/issues/260823-1650_o_die-releaseseite-der-1-0-0-schweigt-zur-verworfenen-keymap-toml-und-der-feste-releasetext-kann-es-nicht-sagen.md` | Möglichkeit 4 aus `260823-1030` ist mit dem festen `RELEASETEXT` nicht ausführbar; Schwere Medium |
| `shared/issues/260823-1651_o_die-auslieferung-ist-der-letzte-commit-einer-sitzung-und-die-sitzung-kann-sich-danach-nicht-mehr-schliessen.md` | Zielkollision zwischen „der Tag steht auf HEAD" und „jede Sitzung schreibt ihr Ende"; Schwere Medium |

Dazu zwei Nachträge an bestehenden Datensätzen: der Abgleichsvermerk an `260823-1433` und die
`Also seen:`-Zeile an `260813-0026`.

## Nichts gefunden zu

- **Falsch gesetzte Marker.** Keine Umbenennung dieser drei Commits; `db1a177` hat den Marker von
  `260813-0053` nicht bewegt, sondern allein den Text der `Implemented:`-Zeile berichtigt. Richtig
  so: der Marker `_i_` stand seit dem 260823-1350 und die Berichtigung ändert das Ergebnis nicht.
- **Pläne mit falschem Stand.** Die Dateien unter `shared/planning/` sind von diesen drei Commits
  nicht berührt.
- **Beantwortete Entscheidungen ohne Marker.** Keine der 14 offenen im gemeinsamen Speicher und
  keine der 20 in den Circles ist durch diese drei Commits beantwortet worden.
- **Als Defekt abgelegte Entscheidungen.** Kein Fall in diesem Bereich.

## Was nicht Sache dieses Abgleichs war

`.DS_Store` ist verfolgt und beim Abschluss dieses Abgleichs geändert. Die Änderung stammt **nicht**
von diesem Lauf: es ist eine Finder-Metadatendatei, und kein Schritt hier hat sie angefasst.
Zurückgesetzt ist sie nicht, weil ein Zurücksetzen selbst eine Änderung wäre.

Der Sitzungsbericht `260823-1424-orchestrator-session.md` steht weiter auf `**Status:** Läuft` und
sein `## Verlauf` ist leer. Dieser Abgleich hat dort allein den Abschnitt `## Coherence` angefügt;
Status und Verlauf gehören dem Orchestrator.

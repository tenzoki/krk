# Coder: der Auslieferungsweg in einem Kommando mit einem Argument

**Datum:** 260813-1556
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** `./release.sh <version>` bauen — Versionszahl setzen, eintragen,
taggen, ausliefern. Bindender Entscheid:
`shared/decisions/260813-1534_a_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`,
Möglichkeit 1. Er hebt
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_s_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md`
auf.
**Abnahme:** `make check` Exit 0 (build, test, fmt-check, clippy unter
`-D warnings`). Proben in `xtask` vorher 60, nachher 89.
**Nicht gefahren:** kein `make bundle`, kein `cargo xtask bundle`, kein
`cargo xtask release`, kein `./release.sh`. Es ist kein Tag entstanden und
nichts eingetragen worden.

## Der Zuschnitt und warum er so liegt

Der Entscheid schließt ein drittes Bauwerkzeug aus: es gibt `xtask` und eine
Hülle darum, das `Makefile`, und ein Skript darf nur weiterreichen. Der Weg
liegt deshalb in drei Schichten, von denen jede genau eine Sache beiträgt:

```text
./release.sh 0.2.0                        genau ein Argument, sonst nichts
  └─ make ausliefern VERSION=0.2.0        Pfad zu cargo, Notarprofil, Reihenfolge
       ├─ cargo xtask version 0.2.0       Zahl, Eintrag, Tag      ← neu
       └─ cargo xtask release             die sieben Stationen    ← unverändert
```

`release.sh` trägt keine Logik und fängt keinen Fehler ab. Das Notarprofil
`krk-notar` steht weiter nur an einer Stelle, im `Makefile`; hätte das Skript
`cargo` unmittelbar gerufen, wäre es eine zweite geworden.

**Warum es unten zwei Kommandos sind und nicht eines.** `xtask` liest die
Versionszahl über `env!("CARGO_PKG_VERSION")`, also beim Übersetzen. Ein Lauf,
der die `Cargo.toml` ändert und im selben Prozess weiter ausliefert, trüge bis
zu seinem Ende die alte Zahl: die `Info.plist` bekäme sie eingesetzt, während
der Tag die neue nennt, und Station 1 sagte dazu nichts, weil sie dieselbe alte
Zahl vergleicht. Der Prozess muss enden.

Das ist zugleich der Gewinn und nicht der Preis: Station 1 läuft im neu
übersetzten Werkzeug und vergleicht die frisch eingebackene Zahl mit dem Tag.
Bliebe ein altes Werkzeug stehen, fände sie den Tag `v0.2.0` nicht und bräche
ab. Der Umweg über zwei Prozesse ist der Wachposten der Einquelligkeit.

## Was gebaut wurde

**`xtask/src/version.rs`, neu — der Unterbefehl `cargo xtask version <zahl>`.**
Er setzt `version` unter `[workspace.package]`, frischt die `Cargo.lock` auf,
trägt beide als **eine** Änderung ein und setzt den Tag `v<zahl>` auf HEAD.

Sechs reine Funktionen tragen die Entscheidungen, nach dem Vorbild von
`stand_pruefen`: `versionszahl_pruefen`, `versionsfeld_finden`, `wertspanne`,
`vorhaben_bestimmen`, `arbeitsbaum_meldung` und die drei Argumentbauer
`tagliste_argumente`, `tag_argumente`, `eintrag_argumente`. Alle 22 Proben des
Moduls laufen an ihnen; keine startet einen Prozess, keine braucht ein
Git-Verzeichnis, keine schreibt.

- **Die Zahl** besteht aus genau drei Zahlenteilen. Ein führendes `v` wird
  benannt und nicht stillschweigend abgestreift, eine führende Null abgewiesen
  (zwei Schreibweisen für dieselbe Zahl), ein Anhang wie `-rc1` ebenso — die
  drei Stufen aus `README.md` kennen drei Zahlen, und Vorabstände sagt dieses
  Projekt nicht zu.
- **Die Zeile in der `Cargo.toml`** wird von Hand getauscht und nicht über einen
  TOML-Zerleger. Die Datei besteht zum größeren Teil aus Begründungen zu den
  fremden Kisten, und ein Zerleger nähme sie beim Ausgeben mit. Getauscht wird
  der Inhalt zwischen zwei Anführungszeichen und sonst kein Byte. Gefunden
  werden muss genau eine Zeile: keine ist ein Abbruch, zwei ebenso.
- **Die `Cargo.lock` gehört zum selben Schritt.** Sie führt die Zahl für jedes
  der vier Mitglieder mit, und `cargo` schreibt sie beim nächsten Bau von
  selbst nach. Bliebe sie liegen, frischte der Bau von `cargo xtask release`
  sie auf, und Station 1 sähe unmittelbar danach einen geänderten Arbeitsbaum
  — an einer Datei, die das Werkzeug selbst erzeugt hat. Aufgefrischt wird sie
  von `cargo update --workspace --offline` und nicht von Hand: `--workspace`
  rührt keine fremde Kiste an, `--offline` geht nicht ins Netz.

**`xtask/src/git.rs`, neu — der eine Zugang zu `git`.** Der Prozessaufruf stand
bis heute in `release.rs`, weil `release` der einzige Abnehmer war. Jetzt sind
es zwei, und der Aufruf ist an die Stelle gewandert, die beide gemeinsam haben;
mit ihm die drei Fragen und die zwei Lesehilfen `geaenderte_dateien` und
`tag_steht`, die `release` und `version` sonst je für sich ausgelegt hätten.
Die Probe `xtask_ruft_git_an_genau_einer_stelle` hält die Zahl weiter auf eins
und zeigt jetzt auf `git.rs`.

Lesen und Schreiben stehen verschieden da. Die drei Fragen sind Konstanten und
tragen keinen Wert aus der Befehlszeile, also sieht `keine_der_drei_fragen_schreibt`
sie Wort für Wort nach. Die beiden schreibenden Kommandos können keine
Konstanten sein, weil beide die Zahl tragen; sie entstehen als reine Funktionen
in `version.rs` und werden dort ebenso Wort für Wort nachgesehen — kein `-f`,
kein `--force`, kein `--amend`, kein `--no-verify`, kein `push`.

Der Eintrag läuft als `git commit --only -m <meldung> -- Cargo.toml Cargo.lock`
und nicht über `git add`. Zwei Wirkungen: ein gescheiterter Eintrag lässt nichts
Vorgemerktes zurück, und der Lauf greift nicht auf die gemeinsame Vormerkung zu,
an der in diesem Projekt auch Agenten arbeiten.

Der Tag ist leicht und nicht annotiert, wie `v0.1.0` vom 260813, den der Nutzer
von Hand gesetzt hat. Station 1 fragt `--points-at` und unterscheidet die Arten
nicht; zwei Arten nebeneinander wären trotzdem zwei Schreibweisen für dieselbe
Sache.

**`Makefile` — das Ziel `ausliefern`.** Zwei Zeilen als eigene Rezeptzeilen und
nicht als Voraussetzungen, aus demselben Grund wie bei `frisch`: make darf
Voraussetzungen in beliebiger Reihenfolge abarbeiten, hier aber muss der
Versionsschritt vor dem Auslieferungsschritt liegen. `make ausliefern` ohne
`VERSION` bricht mit Rückgabewert 2 ab, bevor `cargo` startet.

**`release.sh` — die zweite Hülle.** Prüft, dass genau ein Argument dasteht, und
reicht weiter. Der Modulkopf begründet den Zuschnitt und zeichnet die drei
Schichten.

## Die vier Punkte, an denen es genau werden musste

**1. Übersetzt `cargo xtask release` das Werkzeug wirklich neu?** Ja, gemessen
und nicht angenommen. In einem eigens gebauten Prüf-Workspace mit demselben
Aufbau (`[workspace.package].version`, ein Mitglied mit
`version.workspace = true`, das `env!("CARGO_PKG_VERSION")` ausgibt) gab das
Programm nach jeder Änderung der Zahl die **neue** aus: 0.1.0 → 0.2.0 → 0.3.0 →
0.9.0, jedes Mal über `cargo run`. Am echten Baum, mit versuchsweise auf 0.1.1
gesetzter Zahl, meldete `cargo build -p xtask` „Compiling xtask v0.1.1"; die
Zahl in `Cargo.toml` und `Cargo.lock` ist danach aus einer Sicherung
zurückgeschrieben worden, und `git status` meldet beide Dateien unverändert.

Dass ein `cargo`-Aufruf aus einem laufenden `cargo run` heraus nicht an der
Bausperre hängenbleibt, ist am Prüf-Workspace nachgemessen: das Kind schrieb
die `Cargo.toml`, rief `cargo update --workspace --offline` und bekam „Locking
2 packages … Updating kern v0.3.0 -> v0.9.0". Im echten Baum tut `xtask` das
seit Runde 1 ohnehin — `bundle::uebersetzen` ruft `cargo build` aus demselben
Prozessverhältnis heraus.

**2. Der Zuschnitt.** Die Logik liegt in `xtask`, `release.sh` reicht weiter,
`Makefile` sequenziert. Begründet im Modulkopf von `release.sh`, im
`Makefile`-Kommentar über `ausliefern` und im Modulkopf von `version.rs`.

**3. Was ein Abbruch hinterlässt.** Alles ohne Schreiben Prüfbare wird vor dem
ersten Schreiben geprüft: die Zahl, das Git-Verzeichnis, der Arbeitsbaum, der
Tagname, die Lesbarkeit beider Dateien. Danach bleiben zwei Fenster:

| bricht ab | steht danach |
|---|---|
| `Cargo.lock` auffrischen | nichts — beide Dateien werden zurückgeschrieben |
| Eintrag | nichts — beide Dateien werden zurückgeschrieben |
| Tag setzen | der Eintrag |
| eine der sieben Stationen | Eintrag und Tag |

Die letzten beiden Zeilen sind eine Entscheidung und kein Versäumnis. Eine
Rücknahme des Eintrags hieße `git reset --hard`, also Geschichte umschreiben,
und der Eintrag ist für sich richtig: er trägt die Zahl, die der Nutzer im
Argument gewählt hat. Stattdessen ist der Lauf **wiederholbar**. Steht die Zahl
schon und fehlt nur der Tag, wird nur getaggt; stehen beide, ist nichts zu tun.
Derselbe `./release.sh 0.2.0` noch einmal trägt also nichts doppelt ein und
fährt gleich weiter zu den Stationen. Die Meldung sagt in beiden Fällen, was
steht.

Misslingt eine Rücknahme selbst, sagt die Meldung das und nennt
`git checkout -- Cargo.toml Cargo.lock`. Verschwiegen wird sie nicht.

**4. Bestehender Tag, unveränderte Zahl.** `vorhaben_bestimmen` entscheidet die
sechs Lagen aus Zahl und Tag, überschneidungsfrei und vollständig:

| `Cargo.toml` führt | Tag `v<neu>` | Vorhaben |
|---|---|---|
| die neue Zahl | steht nicht | nur taggen |
| die neue Zahl | steht auf HEAD | nichts zu tun |
| die neue Zahl | steht anderswo | Abbruch |
| eine andere Zahl | steht nicht | setzen, eintragen, taggen |
| eine andere Zahl | steht auf HEAD | Abbruch |
| eine andere Zahl | steht anderswo | Abbruch |

Der dritte Abbruchgrund ist der unscheinbarste: steht der Tag schon auf HEAD,
während die `Cargo.toml` eine andere Zahl führt, schöbe ein Eintrag HEAD um
einen Schritt weiter und ließe den Tag auf dem Commit davor zurück. Der Tag
benennte danach einen Stand mit der alten Zahl. Verschoben wird nie ein Tag;
keine der drei Abbruchmeldungen nennt eine Marke, die es täte.

## Die Lage, die den Weg heute blockiert

`shared/issues/260813-1515_o_…` ist **nicht behoben** — der naheliegende Weg ist
eine Entscheidung über den Umgang mit der Werkbank, und die gehört dem Nutzer.
Behoben ist die Auskunft: der Abbruch nennt jede betroffene Datei beim Namen,
mit der Zustandsspalte aus `git status --porcelain`, und verweist auf den
Datensatz, falls es allein Werkbankdateien sind. Am Baum von heute sieht das so
aus:

```text
xtask: Der Arbeitsbaum weicht vom eingetragenen Stand ab; 8 verfolgte Dateien sind geaendert:

        M Makefile
        M fusion-workbench/.guard-state/churn.json
        …
```

Dieselbe Aufzählung stand schon in Station 1 von `release`; sie ist jetzt in
`git::geaenderte_dateien` zusammengelegt, damit beide Abbrüche dieselbe Auskunft
geben.

## Der überholte Satz

Bis 260813-1534 galt: den Tag setzt der Nutzer, das Werkzeug erzeugt nie einen.
Nachgezogen sind alle Stellen im Baum:

- `xtask/src/main.rs`, Hilfetext — zwei Abschnitte neu, dazu die Probe
  `die_hilfe_traegt_den_ueberholten_satz_nicht_mehr`, die auf den Wortlaut
  anschlägt, falls er zurückkehrt.
- `xtask/src/release.rs`, Modulkopf zu Station 1, Doc-Kommentar zu
  `auslieferungsstand_pruefen`, und die Abhilfe in der Abbruchmeldung: sie nennt
  jetzt `./release.sh <version>` statt `git tag <name>`.
- `README.md`, Abschnitt „Auslieferung" (neu aufgeteilt in „Zahl, Eintrag, Tag"
  und „Das Paket bauen"), Station 1 in der Stationenliste, „Versionspflege" und
  „Versionsstufen". Letzterer sagt die Umkehr ausdrücklich und nennt beide
  Entscheiddatensätze.

**Nicht angefasst, weil Aufzeichnungen eines Standes:** Spec, Plan und
Circle-Datensatz der achten Runde und zwei ihrer History-Logs tragen den Satz
weiter (`planning/260813-1037_c_spec-…` C3.10 und Abschnitt „Beschreibung",
`planning/260813-1110_c_plan-…`, `_c_circle.md` zweimal,
`history/260813-1235-…`, `history/260813-1405-…`, `history/260813-1006-…`). Der
überholende Datensatz verweist auf den überholten, und der überholte trägt seit
heute `_s_` und eine `Superseded by:`-Zeile. `CLAUDE.md` trägt den Satz nicht.

## Offen für den nächsten Lauf

- **Der Entscheid `shared/decisions/260813-1534_a_…` steht noch auf
  „beantwortet".** Er ist mit dieser Änderung in Code umgesetzt und gehört auf
  `_i_`, mit `Implemented: <hash> — …`. Der `coder` trägt nicht ein und hat
  keinen Hash zu zitieren; das gehört zum Commit dieser Arbeit.
- **`./release.sh` ist nie gefahren worden**, weder ganz noch halb: ein Lauf
  setzte einen Tag und schriebe in die Git-Historie. Der grüne Fall ist an den
  reinen Funktionen abgenommen und nicht an einem Lauf. Der erste echte Lauf ist
  Nutzerarbeit — dieselbe Grenze wie beim Abnahmelauf der zehn Zusagen.
- **Die vier Werkbankdateien blockieren ihn weiter.** Solange
  `shared/issues/260813-1515_o_…` offen ist, braucht jeder Lauf vorher ein
  `git stash` oder einen Eintrag der Werkbank.

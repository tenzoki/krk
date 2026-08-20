# Untersuchung: „Nach Installation sind alle Lesezeichen weg"

**Datum:** 2026-08-20 22:35 (Untersuchung) · 2026-08-20 22:42 (Niederschrift)
**Typ:** Fehleruntersuchung
**Status:** Vollständig
**Angefordert von:** Nutzer, über den Orchestrator
**Baumstand:** `01d2365`
**Einschränkung:** Kein Lauf im Vordergrund möglich, also kein Nachstellen. Alles unten ist am Code und an der Platte gelesen.

> **Zur Entstehung dieser Datei.** Die Untersuchung lief am 260820 und gab ihren Bericht als
> Rückgabetext zurück; auf der Platte stand er nicht. Dies ist die unveränderte Niederschrift
> jenes Texts — Verdikt, Beweisstücke, Zeitleiste, Wegskizze, Rangfolge, Messprotokoll,
> Nebenbefunde und die Lücken. Nichts ist gekürzt und nichts neu bewertet. Hinzugekommen ist
> allein der eigens ausgewiesene Abschnitt „Nachmessung des Nutzers am selben Gerät".
> Die Sitzung führt `shared/history/260820-2200-orchestrator-session.md`.

## Verdikt zuerst

Deine Schlussfolgerung trägt nicht, und der Überschreibweg trägt auch nicht. Am Baum gibt es **genau einen Schreiber** für `bookmarks.toml`, und der hängt an einem Lesezeichenbefehl von dir. Weder der Start noch das Beenden noch irgendein Bau- oder Auslieferungsschritt fasst die Datei an. Ein Überkopieren der App kann sie deshalb nicht leeren.

Die Platte trägt trotzdem einen datierten Vorfall, und er hat eine andere Gestalt: **am 17.08. um 19:13:48 hat KRK den ganzen Ablageordner neu angelegt**, drei Minuten und 46 Sekunden nach dem Auslieferungslauf zu `v0.5.1`. Der Ordner war vorher fort. Entfernt hat ihn nichts, was im Baum steht.

Was ich **nicht** entscheiden kann: ob die vier Auslieferungen danach dasselbe getan haben. Dafür brauche ich das Messprotokoll unten.

## Beweisstücke

| Nr. | Beleg | Aussage |
|---|---|---|
| B1 | `~/Library/Application Support/KRK/` — Geburtszeit `2026-08-17 19:13:48` | Der Ordner wurde an diesem Zeitpunkt angelegt, nicht früher |
| B2 | `settings.toml`, `schreiben.lock`, `sitzungsrecht.lock` — Geburtszeit alle `2026-08-17 19:13:48` | `settings.toml` schreibt KRK nur, wenn sie fehlt (`einstellungen.rs:179-184`). Der Ordner war leer |
| B3 | `~/Library/Application Support/` — mtime `2026-08-17 19:13` | Seither ist in diesem Ordner kein Eintrag hinzugekommen oder weggefallen |
| B4 | Tag `v0.5.1` gesetzt `08-17 19:10:02` | Der Auslieferungslauf lag 3:46 vor der Neuanlage |
| B5 | `bookmarks.toml` — 355 Bytes, 5 Einträge, mtime `08-20 16:11:10` | Die Datei steht heute vollständig da |
| B6 | `/Applications/KRK.app` — ctime `08-20 19:47:05`, Geburtszeit vom Quellbündel übernommen | Heute um 19:47 überkopiert. `bookmarks.toml` blieb dabei unangetastet |
| B7 | Laufender Prozess 96272, gestartet 19:47 | Es läuft `target/KRK.app`, **nicht** `/Applications/KRK.app` |
| B8 | Kein `bookmarks.toml.beschaedigt`, kein `.neu` im Ablageordner | Der Beiseitelegeweg der Runde 6 ist nie gelaufen |
| B9 | `codesign -d --entitlements` liefert nichts, kein Container unter `~/Library/Containers` | Keine Sandbox, ein einziger Ablageordner auf der Platte |
| B10 | Kein `com.apple.quarantine` an keinem der drei Bündel | Keine Gatekeeper-Verlagerung, kein App-Translocation-Pfad |
| B11 | `ablage-*.ips` in `DiagnosticReports`, paarweise, 15 Läufe seit dem 20.08. 05:32 | Absichtliche Kindproben aus `tests/ablage.rs:2084`, kein Befund. Taugen als Zeitmarke für `cargo test` |
| B12 | ForkLift ist installiert; sein gesicherter Fensterzustand führt `/Applications` und `~/Projects/productive/krk/target`; `com.binarynights.ForkLift.plist` mtime `08-20 16:10:01` | Das Überkopieren geschieht in ForkLift |
| B13 | Eigenes Leseprogramm gegen `krk_core::ablage::Lesezeichenliste`, Baumstand `01d2365` | Die heutige Datei zerlegt sauber in 5 Einträge |

### Nachmessung des Nutzers am selben Gerät (260820)

Der Nutzer hat B1 bis B3 mit `stat` nachgemessen. Die Werte bestätigen sie wörtlich:

| Eintrag | Geburtszeit | mtime |
|---|---|---|
| `~/Library/Application Support` | – | `2026-08-17 19:13:48` |
| `KRK/` | `2026-08-17 19:13:48` | `2026-08-20 22:36:12` |
| `settings.toml` | `2026-08-17 19:13:48` | `2026-08-17 19:13:48` |
| `bookmarks.toml` | `2026-08-20 16:11:10` | `2026-08-20 16:11:10` |

**Die letzte Zeile ist zugleich der Beleg für die Aussage unter H2**, dass `atomar::schreiben` jede Datei über `rename` ersetzt und ihr dabei eine neue Geburtszeit gibt: an `bookmarks.toml` fallen Geburtszeit und mtime auf dieselbe Sekunde. Eine Datei, die seit ihrer Anlage bestünde, trüge hier zwei verschiedene Zeiten — wie `settings.toml`, die seit dem 17.08. unangetastet dasteht und deren zwei gleiche Zeiten aus demselben einen Schreibvorgang stammen. Die Geburtszeit an `bookmarks.toml` datiert damit den letzten Schreibvorgang und nicht den Bestand. **Was vor dem 20.08. 16:11:10 in der Datei stand, ist aus den Zeitstempeln nicht zu erfahren, und H2 bleibt unentscheidbar.**

## Zeitleiste

```
17.08. 19:10:02   Tag v0.5.1 gesetzt (Auslieferungslauf)
       ~19:1x     ← der Ablageordner verschwindet.  Verursacher unbekannt.
       19:13:48   KRK legt Ordner + settings.toml + beide Sperrdateien neu an
                  ══ Ab hier sind alle Lesezeichen weg ══
18.08. 10:31      v0.5.2   ─┐
19.08. 08:54      v0.5.3    ├─ vier weitere Auslieferungen.  B3 sagt:
19.08. 14:16      v0.5.4    │   der Ordner ist keiner davon zum Opfer gefallen.
20.08. 11:33      v0.5.5   ─┘
20.08. 16:10:01   ForkLift schreibt seine Einstellungen
       16:10:15   target/KRK.app bekommt ein erweitertes Attribut
       16:11:10   bookmarks.toml wird geschrieben (5 Einträge)
       16:13–16:22, 20:53   vier cargo-test-Läufe
       19:44:54   Beglaubigung (certify-only 0.5.5)
       19:47:05   /Applications/KRK.app überkopiert
       19:47      target/KRK.app gestartet, läuft noch
       22:20:13   session.toml geschrieben — bookmarks.toml nicht
```

Die letzte Zeile ist der stärkste Einzelbeleg: dieselbe Sitzung schreibt `session.toml` und lässt `bookmarks.toml` liegen. Genau so ist es gebaut.

## Wie eine leere Leiste überhaupt zustande kommt

Ich habe jeden Weg abgelaufen, auf dem `leiste_einrichten` eine leere Liste bekommt:

```
Start
 └─ sitzung_laden
     ├─ Messmodus (4 Aufgaben) ─────────────► ivars.ablage bleibt None
     │                                        ⇒ leer, KEINE Meldung
     ├─ Ablageordner nicht zu öffnen ───────► leer, Meldung
     ├─ Schreibsperre nicht zu nehmen ──────► leer, Meldung
     └─ gewöhnlich ──► ivars.ablage gesetzt
                        └─ leiste_einrichten ─► Zugang::laden
                            ├─ Datei fehlt ────────► leer, KEINE Meldung
                            ├─ Datei leer (0 Byte) ► leer, KEINE Meldung
                            ├─ oberster Schlüssel
                            │   passt nicht ───────► leer, KEINE Meldung
                            ├─ TOML kaputt ────────► leer, Meldung + .beschaedigt
                            └─ nicht lesbar ───────► leer, Meldung, nichts gesichert
```

Vier der acht Wege schweigen. Und selbst wo geredet wird, kann die Meldung untergehen: die Startmeldungen laufen in einer Schleife durch **eine** Statuszeile und überschreiben einander (Defekt 2 unten).

## Rangfolge der Hypothesen

**H1 — Ein Werkzeug außerhalb von KRK entfernt den Ablageordner beim Installieren.** *Gestützt für den 17.08., ungeklärt für den Rest.*
Dafür: B1, B2 und B4 zusammen sind eindeutig. Ein Ordner, den KRK selbst anlegt, war 3:46 nach dem Auslieferungslauf leer. Kein Codepfad im Baum löscht ihn: ich habe `xtask/`, `Makefile`, `release.sh` und `certify-only.sh` selbst nachgelesen, die einzigen Löschstellen sind `target/KRK.app` (`bundle.rs:220`) und Wegwerfordner im Temporärverzeichnis. Ein Kandidat für den Verursacher steht auf demselben Gerät: ForkLift hat einen App Deleter, der beim Löschen einer Anwendung ihre Stützdateien mitnimmt, `~/Library/Application Support/<Name>` eingeschlossen. B12 belegt, dass du in ForkLift zwischen `/Applications` und `krk/target` arbeitest.
Dagegen: B3 schließt aus, dass der Ordner seit dem 17.08. noch einmal entfernt wurde. Vier Auslieferungen sind seither ohne Verlust vorbeigegangen. Dein „jedesmal" trifft auf den **Ordner** nicht zu.
`speculation:` dass es ForkLifts App Deleter war. Der Beleg dafür (`AppDeleterWindowFrame`) steht im ForkLift-**3**-Plist von 2023, nicht im heutigen.

**H2 — Nur `bookmarks.toml` verschwindet, der Ordner bleibt.** *Nicht entschieden, und aus den vorhandenen Spuren auch nicht entscheidbar.*
Dafür: eine fehlende `bookmarks.toml` ist für KRK der erste Start und **keine Meldung wert** (`mod.rs:490-496`). Der Vorgang wäre vollständig stumm, und genau das passt zu deiner Beschreibung.
Dagegen: nichts. Widerlegen lässt sie sich nicht, weil `atomar::schreiben` jede Datei über `rename` ersetzt und damit bei jedem Schreibvorgang eine neue Geburtszeit setzt. Die 16:11:10 an `bookmarks.toml` sagt nur, wann zuletzt geschrieben wurde, nicht, was davor da war.
Diese Hypothese ist der Grund für das Messprotokoll.

**H3 — Die Datei steht, die Leiste bleibt trotzdem leer.** *Möglich, aber an kein Überkopieren gebunden.*
Dafür: drei Wege führen dahin (Skizze oben). Der Messmodus-Weg schweigt dabei sogar.
Dagegen: keiner der drei hat einen Bezug zur Installation, und zwei davon würden nebenbei die Sitzung und die Belegung mitreißen — das wäre dir aufgefallen. `inference:` dass du eine Meldung übersehen haben könntest, ist wegen Defekt 2 aber nicht auszuschließen.

**H4 — Der Überschreibweg aus dem Modulkopf von `ablage/mod.rs`.** *Für die heutige Datei widerlegt, als Bauform aber offen.*
Dagegen: B13 zerlegt die heutige Datei sauber. Das Dateiformat ist seit dem 03.08. unverändert; `git log --follow` nennt für `lesezeichen.rs` vier Commits, den letzten am 13.08. Keine Runde nach der 7 hat daran gerührt. B8 zeigt, dass der Beiseitelegeweg nie gelaufen ist.
Gemessen habe ich außerdem, wie sich der Leser bei Formänderungen verhält: ein **neues Feld** an einem Eintrag wird getragen, eine **dritte Zielsorte** fällt als Fehler auf und würde beiseitegelegt. Zwei Gestalten kommen still durch — dazu Defekt 1.

**H5 — Sperren und Sitzungsrecht.** *Als Ursache für Lesezeichen widerlegt.*
Das Sitzungsrecht steuert allein `session.toml` (`sperre.rs:150-201`). Der Schreibgriff ist kurzlebig und **wartend**; er kann hängen, aber nicht stillschweigend leeren. Der offene Defekt `260813-0719` betrifft den Sitzungsschreiber und berührt `bookmarks.toml` nicht. Die 0 Bytes an beiden `.lock`-Dateien sind der Normalzustand: `flock` hängt am Deskriptor, nicht am Inhalt.

**H6 bis H8 — Sandbox, Quarantäne, Bau- und Auslieferungskette.** *Alle drei widerlegt*, siehe B9, B10 und den Nachlesebefund zu H1. Deine Prüfung Nr. 3 habe ich nicht übernommen, sondern nachgezogen; sie hält.

## Messprotokoll für den nächsten Vorfall

Ein Zug, vier Halte. Jede Zeile beantwortet eine Frage.

**Vor dem Überkopieren:**

```sh
mkdir -p ~/krk-befund && cd ~/krk-befund
cp -Rp ~/Library/Application\ Support/KRK ./vorher            # vollständige Kopie als Rückfallebene
ls -la@ ~/Library/Application\ Support/KRK                     # Bestand und Größen
stat -f '%N %z %Sm %SB' -t '%F %T' ~/Library/Application\ Support/KRK/* > vorher.stat
shasum ~/Library/Application\ Support/KRK/bookmarks.toml > vorher.sha
```

Der Befehl, der die Frage „Datei oder Leiste" entscheidet, ist der letzte: die Prüfsumme.

**Während des Überkopierens** (in einem zweiten Terminalfenster starten, läuft mit):

```sh
while true; do printf '%s ' "$(date +%T)"; \
  ls -la ~/Library/Application\ Support/KRK/bookmarks.toml 2>&1 | tail -1; \
  sleep 1; done | tee ~/krk-befund/wache.log
```

Beantwortet, **wann** die Datei verschwindet oder schrumpft, und ob das vor oder nach dem ersten Start geschieht.

**Sofort nach dem ersten Start der neuen App, bevor du irgendetwas anklickst:**

```sh
shasum ~/Library/Application\ Support/KRK/bookmarks.toml       # gleich wie vorher.sha ⇒ Datei ist heil, Fehler sitzt in der Leiste
stat -f '%N %SB' -t '%F %T' ~/Library/Application\ Support/KRK/settings.toml   # Geburtszeit neu ⇒ der ganze Ordner war fort
ls ~/Library/Application\ Support/KRK/*.beschaedigt 2>&1       # existiert ⇒ die Datei war beschädigt, KRK hat gesichert
ls -la ~/.Trash | head -20                                     # der App Deleter legt dorthin ab
```

Schau dabei auf die **Statuszeile des aktiven Dateifensters** und notiere den Satz, der dort steht. Ist die Leiste leer und die Statuszeile stumm, war die Datei fort oder leer. Steht ein Satz, hat KRK sie gefunden und nicht verstanden.

**Nach dem Beenden:**

```sh
shasum ~/Library/Application\ Support/KRK/bookmarks.toml
diff <(cat ~/krk-befund/vorher/bookmarks.toml) ~/Library/Application\ Support/KRK/bookmarks.toml
```

Zeigt, ob das Beenden etwas geschrieben hat. Nach meinem Befund darf es das nicht.

**Wiederherstellen, falls verloren:** `cp -p ~/krk-befund/vorher/bookmarks.toml ~/Library/Application\ Support/KRK/` — bei beendetem KRK, sonst überschreibt der nächste Lesezeichenbefehl es wieder.

**Und für den Vorfall vom 17.08.:** es liegen 24 lokale Time-Machine-Schnappschüsse vom 14. und 15.08. auf der Platte. Einer davon trägt den Ablageordner in seinem damaligen Zustand. Mit `sudo mount_apfs -o ro,nobrowse -s com.apple.TimeMachine.2026-08-15-115109.local / /tmp/snap` und einem Blick in `/tmp/snap/Users/k1/Library/Application Support/KRK/` siehst du, welche Lesezeichen du am 15.08. hattest. Ich brauche dafür Rechte, die ich nicht habe.

## Gefilte Defekte

Drei, keiner davon die Wurzel:

- `fusion-workbench/shared/issues/260820-2235_o_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`
  Die Zusage der Runde 6 deckt für diese Datei nur den syntaktischen Fehlschlag. Zwei Gestalten kommen still durch, und der nächste Lesezeichenbefehl schreibt den Verlust fest. Mit einer Messtabelle über fünf Eingabeformen.
- `fusion-workbench/shared/issues/260820-2235_o_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
  Bis zu sechs Startmeldungen laufen durch eine Statuszeile, die eine hält. Genau in der Lage, in der KRK am meisten zu sagen hätte, zeigt er am wenigsten davon.
- `fusion-workbench/shared/issues/260820-2235_o_der-gemessene-start-laedt-die-lesezeichen-nicht-und-die-leiste-schweigt-mit-falscher-begruendung.md`
  In allen vier Messaufgaben kehrt `sitzung_laden` zurück, bevor es `ivars.ablage` setzt. L4 misst damit einen Start ohne den Ablagedurchgang für `bookmarks.toml`, den der echte Start fährt.

## Zwei Nebenbefunde

**Du läufst gerade aus `target/KRK.app`, nicht aus `/Applications/KRK.app`** (B7). Beide tragen dieselbe Bündelkennung `org.stalmann.krk`; welches Bündel `open` erwischt, entscheidet LaunchServices. Wenn du nach dem Überkopieren prüfen willst, ob die neue Fassung läuft, sagt dir das `ps aux | grep KRK.app`. Daneben liegt eine dritte Kopie auf dem Schreibtisch, Fassung 0.1.0 vom 13.08.

**Die `ablage-*.ips`-Berichte sind harmlos** (B11). Sie kommen aus den Kindproben in `tests/ablage.rs`, die absichtlich `std::process::abort()` rufen, um die Lücke zwischen Schreiben und Umbenennen an einem Prozess zu prüfen, der wirklich stirbt. Zwei je `cargo test`-Lauf, und sie datieren deine Prüfläufe recht genau.

## Wo mir die Evidenz fehlt

- **Der Papierkorb ist mir verschlossen** (`Operation not permitted`). Läge dort ein KRK-Ordner mit Lesezeichen, wäre H1 entschieden. Das steht im Protokoll.
- **Die TCC-Datenbank ist mir verschlossen.** Für die Frage, ob ein Überkopieren Zugriffsrechte zurücksetzt, ist das ohne Belang: ein Lesezeichen auf einen unerreichbaren Ordner verschwindet nicht, es bekommt einen Zusatz und eine andere Farbe (`leistenmodell.rs:339-341`). „Weg" heißt es also nicht.
- **Ohne Lauf im Vordergrund** kann ich weder die Statuszeile beim Start sehen noch die Reihenfolge der Systemaufrufe messen. Beides steckt im Protokoll.
- **Die vier Auslieferungen nach dem 17.08.** kann ich nur am Ordner ausschließen, nicht an der einzelnen Datei. Das ist die Lücke, die H2 offen hält.

## Quellen

- Baumstand `01d2365`, gelesen: `crates/krk-core/src/ablage/` (`mod.rs`, `lesezeichen.rs`, `atomar.rs`, `sperre.rs`, `einstellungen.rs`), `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/messmodus.rs`, `crates/krk-ui/src/leistenmodell.rs`, `crates/krk-core/tests/ablage.rs`, `xtask/`, `Makefile`, `release.sh`, `certify-only.sh`
- Platte: `~/Library/Application Support/KRK/`, `/Applications/KRK.app`, `target/KRK.app`, `~/Library/Logs/DiagnosticReports/`, `com.binarynights.ForkLift.plist`
- `git tag -l`, `git log -S 'eintraege' --follow`
- Nachmessung des Nutzers mit `stat`, 260820
- Sitzungsprotokoll: `fusion-workbench/shared/history/260820-2200-orchestrator-session.md`

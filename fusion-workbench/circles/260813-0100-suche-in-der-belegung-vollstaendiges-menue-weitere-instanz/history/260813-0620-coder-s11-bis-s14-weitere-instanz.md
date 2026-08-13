# coder: S11 bis S14, der Instanzstrang der Runde 7

**Datum:** 2026-08-13
**Status:** Complete
**Circle:** `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`
**Plan:** `planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`, Schritte S11 bis S14 samt Nachtrag vom 260813-0233
**Nicht Gegenstand:** S15 (`resources/default-keymap.toml`, `ontocoder`). Die Datei ist unverändert.

---

## Was gebaut ist

**S11 — `flock` als fünfter Fremdaufruf des Kerns.** Ein vierter `unsafe extern "C"`-Block in `crates/krk-core/src/verzeichnis/sys.rs` mit `flock(fd, operation)`, den drei Konstanten `LOCK_EX`, `LOCK_NB`, `LOCK_UN` und `EWOULDBLOCK`, dazu drei Hüllen über einem gemeinsamen `flock_rufen`: `sperre_nehmen` (wartend), `sperre_versuchen` (liefert `Sperrversuch::Genommen` oder `Belegt` statt eines Fehlers) und `sperre_abgeben`. Die Deklaration ist **nicht** variadisch, anders als die von `fcntl` daneben, und der Modulkopf sagt warum: der Header nennt zwei feste Argumente.

Die Zahl „vier Schnittstellen, acht Funktionen" ist an allen vier Stellen auf fünf und neun gezogen: Kopfzeile und Diagramm von `sys.rs`, der Absatz darunter, `crates/krk-core/src/lib.rs` und `crates/krk-core/src/verzeichnis/mod.rs`. Eine dritte Datei mit `#![allow(unsafe_code)]` ist nicht entstanden.

**S12 — zwei Sperren, zwei Lebensdauern, ein neues Modul.** `crates/krk-core/src/ablage/sperre.rs` trägt `Schreibgriff` (kurzlebig, `#[must_use]`, gibt in `Drop` ab) und `Sitzungsrecht` (langlebig, `#[must_use]`, hält seinen Deskriptor bis zum Prozessende). Beide liegen auf eigenen Dateien im Ablageordner, `schreiben.lock` und `sitzungsrecht.lock`, und nicht auf den vier Nutzdateien: `atomar::schreiben` ersetzt eine Zieldatei über ein `rename`, und ein Griff darauf hinge danach an einem Deskriptor, den kein Name mehr nennt.

`Ablage` hält den Deskriptor der Schreibsperre offen und bekommt `durchgang<T>(|zugang| …)`. `laden`, `sichern` und `beiseite_legen` sind von `Ablage` auf den neuen `Zugang` gewandert, den es nur aus einem Durchgang gibt. Damit ist „kein Schreibweg an der Sperre vorbei" eine Eigenschaft der Typen. `atomar::schreiben` selbst bleibt frei, weil die Markdown-Ausgabe und das Sichern der Editordatei außerhalb des Ablageordners schreiben.

Mitgezogen: `einstellungen::laden`, `belegung::laden`, `Belegung::sichern` und die drei Schreibwege des `Sitzungsschreiber`s nehmen jetzt einen `&Zugang`. Der Schreiber trägt seinen Pfad nicht mehr; `Sitzungsschreiber::neu()` nimmt kein Argument, und `Ablage::sitzungsschreiber` ist entfallen.

**S13 — Lesezeichen als Durchgang, Sitzung nur von der Halterin.** `lesezeichen_sichern` ist zu `lesezeichen_aendern` geworden: unter der Schreibsperre wird `bookmarks.toml` frisch gelesen, eine `Aenderung` darauf angewandt und das Ergebnis geschrieben; die Leiste zeigt danach die geschriebene Liste. Das Sitzungsrecht wird in `sitzung_laden` genommen und in den Ivars gehalten; nur wer es hat, bekommt einen `Sitzungsschreiber`, und wer nicht, sagt es einmal beim Start über den vorhandenen Meldungsvektor.

**S14 — der Befehl.** `Kommando::WeitereInstanz` mit der Kennung `weitere_instanz`, `Wirkungsbereich::Ueberall`, `Funktionsbereich::Anwendung` und einem eigenen Zweig in `kommando_ausfuehren`; das neue Modul `crates/krk-ui/src/appkit/weitereinstanz.rs` bestimmt den eigenen Bündelort über `NSBundle::mainBundle().bundleURL()`, prüft die Endung `.app` und startet über `NSWorkspaceOpenConfiguration::setCreatesNewApplicationInstance(true)`. `Kommando` wächst von 75 auf 76; `Wirkungsbereich`, `Bereich`, `Fokus` und `Funktionsbereich` wachsen nicht.

---

## Zwei Entscheidungen, die in der Ausführung gefallen sind

**Eine Änderung nennt ihr Ziel als Eintrag und nicht als Stelle.** Der Schrittext von S13 verweist für die Listenrechnung auf `lesezeichen.rs:279-337`, und jene vier Funktionen nehmen eine Stelle entgegen. Eine Stelle ist aber eine Zahl in der Liste, die der Nutzer gesehen hat; in der frisch gelesenen kann dort ein anderes Lesezeichen stehen, sobald die zweite Instanz eines gelöscht hat. Wer danach umbenennt, benennt das falsche um, und das ist ein schlimmerer Ausgang als die verlorene Änderung, gegen die C3.8 gebaut ist. Der Typ `Aenderung` trägt deshalb das Lesezeichen selbst, `Lesezeichenliste::stelle_von` sucht es in der frisch gelesenen Liste, und danach laufen genau die vier Rechnungen aus `279-337`. Der dritte Ausgang `Verschwunden` sagt dem Nutzer, dass die andere Instanz seinen Eintrag gelöscht hat. Die Abweichung steht als eigener Punkt in S13 des Plans.

**Der Typ heißt `Aenderung` und nicht `Vorgang`.** Der Plan benutzt das Wort „Vorgang"; `krk-ui/src/appkit/anwendung.rs` führt schon einen `Vorgang`, nämlich eine laufende Dateioperation aus C4, und beide Typen treffen sich in derselben Datei.

Als Folge der ersten Entscheidung sind die vier Änderungsmethoden von `Leistenmodell` und `Leistenquelle` weggefallen. An ihre Stelle treten `gewaehltes_lesezeichen_wert` und `uebernehmen(liste, stelle)`: die Leiste rechnet die neue Liste nicht mehr selbst aus, sie zeigt, was unter der Sperre herausgekommen ist. Ihre Proben sind auf denselben Weg umgeschrieben, ohne Datei.

---

## Was rot ist, und warum das planmäßig ist

`cargo test --workspace` endet mit **Exit 101** und drei roten Proben. Alle drei haben dieselbe Ursache: die Auslieferungsbelegung kennt die Funktion `weitere_instanz` noch nicht, und das ist S15, der dem `ontocoder` gehört.

| Probe | Ort |
|---|---|
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | `crates/krk-core/tests/belegung.rs` |
| `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` | `crates/krk-core/src/tasten/belegung.rs` |
| `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` | `crates/krk-ui/src/belegungsausgabe.rs`, meldet 75 gegen 76 |

**Der Plan nannte eine, es sind drei.** Die gemeinsame Ursache ist nachgewiesen und nicht hergeleitet: mit einem vorläufig eingetragenen `[[funktion]]`-Block für `weitere_instanz` und der berichtigten Zählzeile läuft `cargo test --workspace` über alle 19 Ziele vollständig grün, Exit 0, kein einziger Fehlschlag. Der Eintrag ist danach über `git checkout` zurückgenommen worden; `resources/default-keymap.toml` ist Byte für Byte die alte Datei.

---

## Warum `cargo test` keiner laufenden Instanz die Sperre wegnimmt

Die Frage war ausdrücklich gestellt, und die Antwort ist nicht „vorsichtig programmiert", sondern eine Eigenschaft der Ablageorte: **keine Probe öffnet je eine `Ablage` im Benutzerverzeichnis.** Jede legt ihren eigenen Ordner unter dem Temporärverzeichnis an, über die vorhandene Prüfordner-Fassung des Kerns, die Prozesskennung und Laufnummer trägt und sich in `Drop` abräumt. Die einzige Probe, die `Ablageort::im_benutzerverzeichnis` überhaupt nennt, ist `der_ablageordner_liegt_unter_application_support`, und sie **löst nur einen Pfad auf**, ohne etwas anzulegen oder zu öffnen. Die zwei Sperrdateien entstehen deshalb ausschließlich in Prüfordnern.

Der zweite Teil der Frage betrifft den Messplanwächter, der beim Anlegen jede fremde `krk-messplan-*.toml` im Temporärverzeichnis abräumt. Er trifft die Sperrdateien nicht: sie heißen `schreiben.lock` und `sitzungsrecht.lock` und liegen in einem Prüfordner, nicht unmittelbar im Temporärverzeichnis.

Eine vierte Prüfordner-Fassung ist nicht entstanden. Das Prüfmodul von `sperre.rs` trägt einen eigenen kleinen Ordner, weil es kistenintern sichtbare Funktionen braucht und die Fassung unter `tests/gemeinsam/` von dort aus nicht zu erreichen ist; die Zählprobe zu C4.6 zählt Erklärungen selbstabräumender **Prüfordner** und bleibt bei drei.

---

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | Exit 0 |
| `cargo fmt --all --check` | Exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| `cargo test --workspace --no-fail-fast` | Exit 101, drei rote Proben, alle drei aus S15 |

Probenzahlen nach dem Lauf, je Ziel: xtask 56, krk-core lib 150 grün und 1 rot (vorher 146 grün), krk-core `ablage` 51 grün und 5 ignoriert (die fünf Kindproben), krk-core `baum` 3 (neu), krk-core `belegung` 44 grün und 1 rot, `navigation` 15, `operation` 26, `stapelumbenennen` 7, `tasten` 5, `text` 25, `textkopien` 2, `verzeichnis` 16, `zwischenablage` 9, krk-ui bin 535 grün und 1 rot (vorher 533), `syntaxkiste` 5, krk-bench 46.

**Kein Bündelbau, kein Vordergrundlauf, keine Messung.** `target/KRK.app` ist nicht angefasst worden; weder `make bundle` noch `make run` noch `cargo xtask bundle` ist gelaufen, und eine zweite Instanz ist nicht gestartet worden. Am Bündel bleibt zu sehen: dass eine zweite Instanz mit eigenem Fenster nach vorn kommt (C3.1), dass sie in der Statuszeile sagt, dass sie die Sitzung nicht schreibt (C3.10), und dass der Befehl beim Entwicklungslauf ohne Bündel seinen Satz meldet (C3.6).

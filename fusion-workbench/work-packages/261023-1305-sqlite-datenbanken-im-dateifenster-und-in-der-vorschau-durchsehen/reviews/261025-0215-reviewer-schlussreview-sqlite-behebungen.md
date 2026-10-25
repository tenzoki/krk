# Schlussreview: die Behebungen aus dem Nachreview des SQLite-Pakets

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `86ac5164..4777a1cb`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `261025-0030-die-vorfrage-zum-leeren-wal-liest-neben-einem-laufenden-exklusiven-schreiber-ohne-sperre-und-niemand-sagt-es.record.json`, `261025-0031-nach-einem-lesefehler-laedt-die-vorschau-denselben-archiveintrag-bei-gleicher-kennung-nie-wieder.record.json`, `261025-0032-der-doc-kommentar-von-rumpf-von-haengt-seit-dem-einschub-von-freie-funktion-an-der-falschen-funktion.record.json`
**Review domain:** code
**Verdict:** accept

Zu den nicht geöffneten Dateien: Das sind das Ereignisprotokoll und die drei Steuerdateien der behobenen Issues. Ihr Zustand steht nicht im Prüfauftrag. Teilweise gelesen habe ich das Issue 0032 (Kopf) und das Nachreview `261025-0035-…` mit seiner Evidenz (Kopf und Format). Ganz gelesen sind die Diffs aller sieben Code- und Prosadateien von `4777a1cb`, die Issues 0030 und 0031 (Befund, Erwartet, Abnahme, Resolved) und der Nachtrag 261025 im Entscheid `261024-2215-…`. `33c6a468` ändert allein Workbench-Dateien (das Nachreview und seine drei Issues) und keinen Code.

## Summary

Die drei Issues des Nachreviews sind in der Sache behoben. Die Sperrfrage ist gegen das SDK richtig gebaut, und die Einteilung vorübergehend/bleibend ist vollständig und trägt keine neue Schleife herein. Bau, Clippy, `fmt`, `cargo doc -D warnings`, C-Freiheit und die betroffenen Proben sind grün. Neu ist ein kleiner Befund, der schon vor diesem Bereich bestand: Das Öffnen und Schließen eines Deskriptors vor jedem Öffnen kann die Sperren einer zweiten Verbindung von KRK aufheben, und der Modulkopf nennt das nicht (Low).

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 1 |

## Stand der drei Issues

| Issue | In der Sache | Beleg |
|---|---|---|
| 0030 Vorfrage neben laufendem exklusivem Schreiber | behoben nach Folge 2 (Weg B) | `verzeichnis/sys.rs:1213` `fremde_sperre`; `archiv/sqlite.rs:302` `sofort_unveraenderlich`; Probe `neben_einem_laufenden_exklusiven_schreiber_mit_leerem_wal_meldet_der_ort_belegt` grün. Modulkopf, `HowTo.md` und der Nachtrag im Entscheid 2215 nachgezogen |
| 0031 kein erneuter Versuch nach Lesefehler | behoben | `vorschaumodell.rs:2287` `voruebergehend`, `:1673` die bedingte Übernahme von `archivquelle`; Probe und Gegenprobe grün |
| 0032 Doc-Kommentar von `rumpf_von` | behoben | `appkit/tabelle.rs:7112–7122`, der Kommentar steht direkt über `fn rumpf_von` |

## Geprüft, ohne Befund

### Die Sperrfrage (`verzeichnis/sys.rs`)

- Gegen `$(xcrun --show-sdk-path)/usr/include/sys/fcntl.h` gehalten: `F_GETLK 7`, `F_RDLCK 1`, `F_UNLCK 2`, `F_WRLCK 3`. `struct flock` ist `off_t l_start; off_t l_len; pid_t l_pid; short l_type; short l_whence;`. Die Rust-Form `flock_satz` (`sys.rs:1183`) trifft Reihenfolge und Breiten, und `const _: () = assert!(size_of::<flock_satz>() == 24)` hält die Größe beim Übersetzen.
- Der Aufruf geht über die schon gebundene variadische Deklaration `fn fcntl(fd: c_int, befehl: c_int, ...) -> c_int` (`sys.rs:893`) und übergibt einen `*mut c_void`, wie der Header es verlangt. Dazu kommt keine neue Bindung, und die Zählung „dreissig in neun Blöcken“ im Modulkopf bleibt richtig.
- Der `SICHERHEIT`-Kommentar nennt Leihe des Deskriptors, Größe und Lebensdauer des Satzes und die variadische Übergabe. Bei `-1` gibt die Funktion `io::Error::last_os_error()` zurück, ohne den Fehler zu verschlucken.
- `F_GETLK` an einem nur lesend geöffneten Deskriptor gelingt auf macOS. Das zeigt die Probe `an_einer_frischen_datei_haelt_niemand_eine_sperre`, die mit `File::open` öffnet. Eigene Sperren meldet `F_GETLK` nicht, wie der Doc-Kommentar sagt.

### Die Oeffnungsregel (`archiv/sqlite.rs`)

- `sofort_unveraenderlich` (`:302`) öffnet einen Deskriptor und fragt Kopf und Sperre an ihm. Ein zweites Öffnen und Schließen gibt es nicht. Jeder Fehler endet in `false` und damit in `mode=ro`: Öffnen, Kopf, `-wal`/`-shm` und `matches!(…, Ok(false))` für die Sperrfrage.
- Der Bereich `0x4000_0000` über 512 Bytes deckt `PENDING`, `RESERVED` und den geteilten Bereich ab (`os_unix.c`). Die Probe zeigt, dass ein Schreiber im Modus `exclusive` dort gesehen wird.
- Der Modulkopf `# Nur lesen` (`:119 ff.`) nennt den Restwettlauf (ein später startendes Programm) und den Restfall (geteilte Sperre ohne `-shm`) ehrlich. Der Nachtrag im Entscheid 2215 sagt dasselbe und hält die Abwägung gegen Möglichkeit 2 für den Rest offen.

### Vorübergehend oder bleibend (`vorschaumodell.rs`)

- `voruebergehend` ist vollständig über alle acht Varianten von `Lesefehler` (`archiv/lesen.rs:91–115`) und alle sechs von `Verzeichnisfehler` (`archiv/verzeichnis.rs:87–108`), ohne Auffangzweig. Ein neuer Wert hält den Bau an.
- Die Einteilung stimmt mit der Zuordnung in `sqlite::fehler_zuordnen` und `sqlite::lesefehler` (`archiv/sqlite.rs:600–630`) überein. `SQLITE_BUSY`/`SQLITE_LOCKED` → `Verzeichnis(Belegt)` → vorübergehend. Ein Ein-/Ausgabefehler mit Systemfehler → `Lesen` → vorübergehend. `SQLITE_NOTADB` und jeder andere Code → `Ungueltig` → bleibend.
- `Lesen(_)` zählt auch einen Fehler als vorübergehend, der in Wahrheit bleibt, etwa `EACCES`. Das ist kein Defekt. `chmod` ändert die Kennung nicht, und gerade dann soll nach dem Zurücksetzen der Rechte ein erneuter Versuch gelingen. Der erneute Versuch scheitert schnell und löst selbst keine Dateisystemmeldung aus.

**Kann eine Schleife zurückkommen?** Ich habe keinen Weg gefunden (am Quelltext und an der SQLite-Sperrfolge gelesen, im Bündel nicht gemessen). Eine Schleife bräuchte einen vorübergehenden Fehler, dessen erneuter Versuch selbst eine Datei im Ordner berührt, ohne dass sich die Kennung ändert. Die Fälle im Einzelnen:
- `Belegt` neben einem Schreiber im Modus `exclusive`: Der Leser scheitert schon an `SHARED` auf der Hauptdatei, bevor er den WAL-Index öffnet. Er berührt keine `-shm`, gleich ob eine steht. Ohne `-shm` hält das die Probe (e) fest, die prüft, dass nichts entsteht.
- `Belegt` neben einem Schreiber im Rollback-Betrieb: Da gibt es keine `-shm`. Was der Schreiber an `-journal` anlegt, ist seine Meldung und nicht die von KRK, und sie endet, wenn er fertig ist.
- `Belegt` im WAL-Betrieb mit `-shm` (Wiederherstellung, `SQLITE_BUSY_RECOVERY`): Der Leser kann eine Lesemarke setzen. Der Grund endet aber mit der Wiederherstellung des anderen Programms, also ist die Schleife begrenzt.
- `VonAussenVeraendert` entsteht nur mit neuer Kennung und lädt ohnehin neu. `Abgebrochen` kommt von der Marke eines ersetzten Vorgangs, dessen Lieferung verfällt.
- Die Restfall-Lage (geteilte Sperre ohne `-shm`) legt über `mode=ro` vielleicht einmal eine `-shm` an. Gelingt das Lesen dann, ist das Ergebnis merkbar, und der Kurzschluss greift.

Die bleibenden Gründe (PDF, über der Grenze, kein UTF-8, verschlüsselt, `Ungueltig`, `Fehlt`) werden gemerkt. Die Gegenprobe `nach_bleibenden_angaben_laedt_derselbe_archiveintrag_nicht_neu` hält das für PDF und Übergröße fest.

### Projektregeln

- C-Freiheit: `cargo tree --target aarch64-apple-darwin -e normal,build` und dasselbe für `x86_64-apple-darwin` führen kein `cc` und kein Paket mit einem Namen auf `-sys`.
- `#![allow(unsafe_code)]` steht genau in `krk-core/src/verzeichnis/sys.rs:194` und `krk-ui/src/appkit/mod.rs:1`. Die übrigen Treffer sind Prosa. `cargo test -p krk-core --test baum` ist grün (20/20).
- Keine Änderung an `Cargo.toml` oder `Cargo.lock` im Bereich.
- Gefahren: `cargo test -p krk-core --test archiv_sqlite` (22 grün, 1 ignoriert), `cargo test -p krk-ui --bin krk vorschaumodell` (75 grün), `… tabelle::tests` (27 grün), `cargo clippy --workspace --all-targets` ohne Warnung, `cargo fmt --all --check` sauber, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` sauber.

## Befund

### L1: Die Vorfrage vor jedem Öffnen schließt einen Deskriptor der Hauptdatei (Low)

`archiv/sqlite.rs:238` `oeffnen` ruft vor jedem Öffnen `sofort_unveraenderlich` (`:302`). Das öffnet einen Deskriptor auf die Hauptdatei, bevor der Kopf gelesen ist, und schließt ihn am Ende, auch bei einer Datenbank im Rollback-Betrieb. Die Rückfallstufe tut es über `wal_kopf` (`:316`) ein weiteres Mal. Nach POSIX hebt jedes Schließen alle `fcntl`-Sperren des Prozesses auf der Datei auf. Der Doc-Kommentar an `sofort_unveraenderlich` (`:297`) und an `fremde_sperre` (`sys.rs:1206`) nennt diese Regel selbst als Grund für den einen Deskriptor. Halten zwei Fäden von KRK zugleich Verbindungen auf dieselbe Datei, etwa die Vorschau beim Lesen einer Tabelle und das Dateifenster beim Auffrischen des Inhaltsverzeichnisses, dann hebt das Schließen des einen die `SHARED`-Sperre des anderen auf. Ein fremder Schreiber kann dann mitten in das Lesen schreiben (`howtocorrupt.html` §2.2; Inferenz aus dem Quelltext, nicht gemessen).

Das Muster bestand schon vor `4777a1cb` (`wal_kopf` in `oeffnen` seit `a1fec2b7`). Diese Behebung verengt es von zwei Schließen auf eines. Die Datenbank bleibt heil. Eine Tabelle sichert die Kennungsprüfung ab, das Inhaltsverzeichnis kann einmal falsch stehen bis zur nächsten Auffrischung. Der Modulkopf `# Nur lesen` nennt das nicht neben Restwettlauf und Restfall.

Vorschlag: den Fall im Modulkopf als dritte Restklasse nennen, oder Vorfrage und Öffnen prozessweit je Pfad so halten, dass kein Deskriptor außerhalb von SQLite neben einer offenen Verbindung geschlossen wird.

Issue: `261025-0215-die-vorfrage-vor-jedem-oeffnen-schliesst-einen-deskriptor-der-hauptdatei-und-hebt-damit-die-sperren-einer-zweiten-verbindung-von-krk-auf.md`.

## Querschnitt

Die Sperrfrage und die Wiederversuchsregel hängen über `Verzeichnisfehler::Belegt` zusammen. Die Sperrfrage macht aus dem Bild „laufender exklusiver Schreiber“ ein `Belegt`. Die Vorschau versucht nach einem `Belegt` erneut, und das HowTo sagt „nach dem Beenden des Programms liest KRK sie wieder“. Beide Hälften sind gebaut, und ihre Proben sind grün. Ob das Dateifenster nach einem `Belegt` beim nächsten FSEvents-Lauf ebenso neu liest, gehört nicht zu diesem Bereich, und ich habe es nicht geprüft.

## Reihenfolge

Nichts blockiert das Schließen des Pakets. L1 ist Aufräumarbeit und kann in einem späteren Paket oder als Wortlaut im Modulkopf erledigt werden.

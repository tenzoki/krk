Die Fehlerausgabe von ssh wird ohne Längengrenze je Zeile gelesen
---
Der Fehlerfaden hält die letzten acht Zeilen der Fehlerausgabe von `ssh`, liest aber jede Zeile mit `split(b'\n')` ohne Grenze. Ein Server, dessen Fehlerausgabe `ssh` weiterreicht, kann mit einem Strom ohne Zeilenende den Speicher von KRK unbegrenzt wachsen lassen. Für Pakete hält die Grenze von 256 KiB das auf, für diesen Strom gibt es keine.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Speicher-DoS durch einen feindlichen oder fehlerhaften Server)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/fern/gespraech.rs`, `fehlerausgabe_lesen`: `for zeile in BufReader::new(fehler).split(b'\n') {`. Die Grenze `FEHLERZEILEN` zählt Zeilen, nicht Bytes.
- `inference:`: OpenSSH schreibt die Erweiterungsdaten des Kanals (stderr der Gegenseite) auf die eigene Fehlerausgabe, auch beim Teilsystem `sftp`.

**Behebung (Richtung)**

Je Zeile höchstens eine feste Zahl von Bytes behalten, etwa 4 KiB, und den Überhang bis zum nächsten `\n` verwerfen, zum Beispiel über `fill_buf`/`consume`. Die Statuszeile zeigt ohnehin nur eine Zeile.

**Abnahme**

Eine Probe speist `fehlerausgabe_lesen` mit einem Leser, der 10 MiB ohne `\n` und danach `ende\n` liefert. Sie zeigt, dass höchstens die Grenze gehalten wird und `ende` als letzte Zeile ankommt.

---

Resolved: `crates/krk-core/src/fern/gespraech.rs`, `fehlerausgabe_lesen` liest ueber `fill_buf`/`consume`, haelt je Zeile hoechstens `FEHLERZEILENLAENGE` (4096) Bytes, liest den Ueberhang bis zum Zeilenende weiter und verwirft ihn, sodass `ssh` nie an einer vollen Pipe haelt; die letzten `FEHLERZEILEN` bleiben, eine letzte Zeile ohne Zeilenende zaehlt mit (`zeile_halten`). Der Leser nimmt `impl Read`. Proben im neuen Pruefmodul derselben Datei: `eine_endlose_fehlerzeile_wird_gekuerzt_und_die_naechste_kommt_an` (10 MiB ohne Zeilenende, dann `ende`) und `die_letzten_zeilen_bleiben_auch_ohne_zeilenende_am_schluss`.

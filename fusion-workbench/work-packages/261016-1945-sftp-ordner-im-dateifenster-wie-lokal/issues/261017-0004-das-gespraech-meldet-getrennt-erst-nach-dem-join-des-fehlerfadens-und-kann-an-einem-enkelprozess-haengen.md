Das Gespräch meldet „getrennt“ erst nach dem Join des Fehlerfadens und kann an einem Enkelprozess hängen
---
Endet `ssh`, setzt der Gesprächsfaden den Zustand erst nach `kind.wait()` und `fehlerfaden.join()` auf `Getrennt` und bedient erst dann die Wartenden. Hält ein Enkelprozess die Fehlerausgabe offen, kommt der Join nie zurück. Ein solcher Enkel ist etwa ein `ProxyCommand` aus `~/.ssh/config`, der `ssh` überlebt. Keine wartende Anfrage bekommt dann eine Antwort, und Leser und Vorgang hängen. Dazu kommt: ein lebender Server, der nicht mehr liest, hält jeden Schreiber unter dem Mutex der Eingabe fest, und der Abbruch durch den Nutzer kann das nicht lösen.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (`inference:` aus dem Ablauf; nicht ausgelöst)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/fern/gespraech.rs`, `gespraech_fuehren` und `abschliessen`: Join des Fehlerfadens vor dem Setzen von `Getrennt`.
- `gespraech.rs`, `senden`: `write_all` unter dem Mutex der Eingabe, ohne Zeitgrenze. Der Modulkopf nennt das fehlende Zeitlimit als Restrisiko.
- Nebenbefund derselben Datei-Gruppe: `crates/krk-core/src/fern/faden.rs`, das Dokumentationsbeispiel ist nur `compile_fail` und nicht `compile_fail,E0277`. Es bliebe grün, wenn es aus einem anderen Grund nicht übersetzt.

**Behebung (Richtung)**

1. Zuerst `Getrennt` setzen und die Wartenden bedienen, danach den Fehlerfaden ernten. Zeilen, die erst danach ankommen, gehen dabei verloren.
2. Ein Abbruch durch den Nutzer an einem Vorgang, dessen Anfrage länger als eine feste Frist unbeantwortet bleibt, beendet den Kindprozess und gilt als Trennung.
3. `compile_fail,E0277`.

**Abnahme**

Eine Probe startet über einen `Kanalplan` ein Programm, das einen Enkel mit offener Fehlerausgabe zurücklässt und selbst endet, etwa `sh -c 'sleep 30 >&2 & exit 0'`. Sie zeigt, dass eine wartende Anfrage innerhalb von einer Sekunde `Fernfehler::Getrennt` bekommt.

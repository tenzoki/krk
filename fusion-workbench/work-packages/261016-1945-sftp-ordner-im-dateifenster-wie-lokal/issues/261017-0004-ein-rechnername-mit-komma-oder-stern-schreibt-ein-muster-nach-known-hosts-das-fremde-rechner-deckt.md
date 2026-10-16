Ein Rechnername mit Komma oder Stern schreibt ein Muster nach known_hosts, das fremde Rechner deckt
---
„Annehmen“ im Blatt zum Rechnerschlüssel hängt `<suchname> <typ> <schlüssel>` an die `known_hosts` des Nutzers. Der Suchname kommt ungeprüft aus `ssh -G`, und die Adressprüfung lässt Zeichen durch, die `known_hosts` als Musterliste liest (`,`, `*`, `?`, `!`). Eine Adresse wie `files.example,*` schreibt dann eine Zeile, die den gescannten Schlüssel für **jeden** Rechner gelten lässt, für alle späteren `ssh`-Aufrufe des Nutzers, nicht nur in KRK.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Sicherheit; verlangt, dass der Nutzer eine solche Adresse einträgt und den Schlüssel annimmt)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/fern/adresse.rs`, `fernziel`: abgewiesen werden nur ein leerer Rechner, `-` am Anfang, `@`, Leerraum und Steuerzeichen.
- `crates/krk-core/src/fern/rechnerschluessel.rs`, `suchname`: gibt `hostname` aus `ssh -G` wörtlich zurück, sonst `[hostname]:port`.
- `rechnerschluessel.rs`, `gescannt_aus`: `let zeile = format!("{suchname} {typ} {schluessel}");` Geprüft werden Typ und Schlüssel, der Name nicht.
- Geprüft: `ssh -G -p 22 -- 'evil.example,*'` gibt `hostname evil.example,*` aus, ohne Fehler (OpenSSH 9.9p2 des Systems).
- `inference:` aus dem Quelltext von `ssh-keyscan` (`conalloc`): eine Namensliste mit Komma wird der Reihe nach verbunden, bis eine Verbindung steht. Der Scan von `evil.example,*` erreicht also `evil.example`. Nicht am Netz geprüft: auf diesem Gerät läuft kein `sshd`.

**Behebung (Richtung)**

1. In `fernziel` den Rechner auf die Zeichen eines Rechnernamens oder einer IP-Adresse beschränken: ASCII-Buchstaben, Ziffern, `.`, `-`, `_`, dazu `:` und `%` nur in Klammern bei IPv6.
2. In `gescannt_aus` oder `anhaengen` den Suchnamen vor dem Schreiben gegen dieselbe Regel halten (zuzüglich der Form `[name]:port`) und sonst abweisen. Der Suchname stammt auch aus einem `HostKeyAlias` der Konfiguration, deshalb reicht die erste Prüfung allein nicht.

**Abnahme**

- Eine Probe in `tests/fern.rs` zeigt: `fernziel("a.example,*", "")` und `fernziel("a*", "")` werden abgewiesen, und `gescannt_aus` mit einem Suchnamen, der `,`, `*`, `?` oder `!` trägt, liefert einen Fehler statt einer Zeile.
- `ssh.strato.de`, `[::1]:2222` und `10.0.0.1` werden weiter angenommen.

---

Resolved: `crates/krk-core/src/fern/adresse.rs` traegt die neue Regel `rechnername_gueltig` (ASCII-Buchstaben, Ziffern, `.`, `-`, `_`; IPv6 aus Hexziffern, `:`, `.` und Zone hinter `%`; kein fuehrendes `-`), und `fernziel` weist jeden anderen Rechner ab, dazu eckige Klammern ohne IPv6 darin (Modulkopf und `Adressfehler::RechnerUngueltig` nachgezogen). `crates/krk-core/src/fern/rechnerschluessel.rs`: neues `suchname_gueltig` (`rechner` oder `[rechner]:port`), `gescannt_aus` weist einen anderen Suchnamen ab, `anhaengen` prueft vor dem ersten Byte jede Zeile, dass ihr erstes Feld genau der Suchname ist (`InvalidData`, sonst nichts geschrieben); Modulkopf um den Abschnitt zum Musterzeichen erweitert. Probe `ein_rechnername_mit_musterzeichen_kommt_nicht_nach_known_hosts` in `crates/krk-core/tests/fern.rs`; `HowTo.md`, Abschnitt zum Blatt „Verbinden…“, nennt die Regel.

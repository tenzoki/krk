Der Prüfordner von `krk-ui` räumt einstufig ab, die Kernfassung zweistufig
---
`crates/krk-ui/src/pruefordner.rs` und `crates/krk-core/tests/gemeinsam/mod.rs` sind seit dem 260810 auseinandergelaufen: der Kern räumt in `Drop` über `abraeumen` (erst `remove_dir_all`, dann Eintrag für Eintrag mit zurückgedrehten Rechten), `krk-ui` allein über `let _ = remove_dir_all`. Zwei Proben von `krk-ui` setzen `0o000`; eine davon räumt von Hand auf, „bevor die Probe fehlschlagen kann“, und genau diese Handarbeit hat die Kernfassung überflüssig gemacht.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/pruefordner.rs:129-133`: `impl Drop` mit `let _ = std::fs::remove_dir_all(&self.pfad);`.
- `crates/krk-core/tests/gemeinsam/mod.rs:202-242`: `Drop` → `abraeumen` → `entsperren_und_loeschen`.
- `crates/krk-ui/src/kommandos/pfadeingabe.rs:214-221`: `0o000` auf einen Unterordner, dann `set_permissions(0o700)` vor der ersten Zusicherung, mit dem Kommentar, sonst bleibe ein Ordner liegen. Schlägt `pruefen` selbst mit Panik fehl, bleibt er trotzdem liegen.
- `crates/krk-ui/src/leistenmodell.rs:1066-1068`: `0o000` auf eine Datei; die kann `remove_dir_all` löschen, weil Löschen am Ordner hängt und nicht an der Datei. Kein Leck, aber dieselbe Bauform ohne die Sicherung.
- Prozesskennung und Laufnummer sind in beiden Fassungen gleich gebaut (`pruefordner.rs:60-68` gegen `mod.rs:69-78`); auseinander sind allein das Abräumen und der Vorrat an Helfern (`luecke`, `verknuepfung`, `socket`, `verstecken` nur im Kern; `nur_name`, `anlegen`, `loeschen` nur in `krk-ui`).

`CLAUDE.md` sagt, es gebe „genau drei Fassungen, eine je Kiste, und das soll so bleiben“; dass die drei dasselbe tun sollen, steht nirgends, und hier tun sie es nicht.

## Vorschlag

`abraeumen` und `entsperren_und_loeschen` in `pruefordner.rs` übernehmen (etwa vierzig Zeilen) und `nur_name` sowie `Drop` darauf setzen; die Handarbeit in `pfadeingabe.rs:221` kann dann bleiben oder fallen. Ob `krk-bench/src/wegwerfordner.rs` dieselbe Lücke hat, habe ich nicht geöffnet.

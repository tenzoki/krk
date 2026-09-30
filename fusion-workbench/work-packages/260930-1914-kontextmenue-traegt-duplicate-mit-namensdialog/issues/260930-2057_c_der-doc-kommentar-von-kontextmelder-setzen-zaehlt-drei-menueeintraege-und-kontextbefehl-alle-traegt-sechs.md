Der Doc-Kommentar von `kontextmelder_setzen` zählt „die drei Menueeintraege“, und `Kontextbefehl::ALLE` trägt sechs
---
`crates/krk-ui/src/appkit/tabelle.rs:1554`, Doc-Kommentar von `DateifensterQuelle::kontextmelder_setzen`: „Ohne ihn faellt [`Self::kontextbefehl_melden`] still durch, und die drei Menueeintraege stuenden da und taeten nichts“. Die Zahl stammt aus der Runde 17 und ist seit dem 260907 (vier), dem 260918 (fünf) und dem 260930 (sechs, `Kontextbefehl::Duplizieren`) falsch; `awk '/pub enum Kontextbefehl/,/^}/' crates/krk-ui/src/kommandos/kontextmenue.rs` zählt sie.

Schritt 3 des Plans `260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md` verlangt, dass „sein Doc-Kommentar und der Modulkopf von `tabelle.rs` jede Zahl über die eigenen Einträge“ verlieren; zwei Stellen sind nachgezogen (`tabelle.rs:2091` und `tabelle.rs:5655`), diese dritte nicht. Die Aussage bleibt richtig, wenn die Zahl fällt: „die eigenen Menueeintraege stuenden da und taeten nichts“.

Erhebung: `grep -n 'drei Menueeintraege' crates/krk-ui/src/appkit/tabelle.rs`.

Abnahme: die Zeile trägt keine Zahl über die eigenen Einträge mehr; `grep -n -E '(drei|vier|fuenf|sechs) Menueeintraege' crates/krk-ui/src/appkit/tabelle.rs` gibt nichts aus; `make check` grün.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Durchsicht `17750c7..10d2128` (Punkt 7 des Auftrags: keine Zahl über gewachsene Aufzählungen). Ein Einzeiler.

---
Resolved: „die drei Menueeintraege“ im Doc-Kommentar von `DateifensterQuelle::kontextmelder_setzen` (`crates/krk-ui/src/appkit/tabelle.rs`) heißt jetzt „die eigenen Menueeintraege“. Die übrigen Zahlwörter aus `grep -n 'drei\|vier\|fuenf\|sechs'` über `tabelle.rs` und `kontextmenue.rs` zählen anderes (Framework-Berührungen, Verzweigungsstellen, Lagen) oder sind ausdrücklich historisch („stand bis zum 260907 auf drei“) und bleiben. `grep -n -E '(drei|vier|fuenf|sechs) Menueeintraege' crates/krk-ui/src/appkit/tabelle.rs` gibt nichts aus; `make check` grün.

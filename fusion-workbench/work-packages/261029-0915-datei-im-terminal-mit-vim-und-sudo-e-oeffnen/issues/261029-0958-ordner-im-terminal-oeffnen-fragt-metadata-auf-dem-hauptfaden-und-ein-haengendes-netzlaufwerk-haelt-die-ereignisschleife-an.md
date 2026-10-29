„Ordner im Terminal öffnen“ fragt `metadata` auf dem Hauptfaden, und ein hängendes Netzlaufwerk hält die Ereignisschleife an
---
`Anwendungsdelegierter::terminal_oeffnen` (`crates/krk-ui/src/appkit/anwendung.rs`) ruft vor der Übergabe an die Terminal-Anwendung `operationen::ordner_fehlt(&ordner)` (`crates/krk-ui/src/kommandos/operationen.rs`), und das ruft `std::fs::metadata` auf dem Hauptfaden. Ein über NetFS eingehängter Ordner ist ein lokaler `Ort`, also ist `ctrl+o` dort zulässig. Antwortet der Server nicht, wartet der Aufruf auf das Zeitlimit des Dateisystems, und die Ereignisschleife steht so lange. Das ist derselbe Defekt wie `261016-1956-der-loeschweg-fragt-canonicalize-und-den-papierkorb-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md`, an einer weiteren Stelle.
---
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>

Gefunden beim Überarbeiten des Plans `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`. Der neue Befehl dort legt seine Plattenfragen auf einen Arbeitsfaden und nimmt `ctrl+o` ausdrücklich nicht zum Vorbild. Den bestehenden Weg ändert das Paket nicht.

Beleg: am Quelltext gelesen am 261029, nicht an einer hängenden Freigabe gemessen.

Abnahme: Der Rumpf von `terminal_oeffnen` stellt keinen Systemaufruf. Die Frage nach dem Ordner läuft auf einem Arbeitsfaden, und die Antwort kommt über `DispatchQueue::main` zurück, gehalten von einer Quelltextprobe nach dem Muster `der_duplikatweg_stellt_auf_dem_hauptfaden_keinen_systemaufruf`. Ob die Änderung einen Abnahmelauf gegen die zehn Zusagen schuldet, entscheidet der Plan, der sie baut.

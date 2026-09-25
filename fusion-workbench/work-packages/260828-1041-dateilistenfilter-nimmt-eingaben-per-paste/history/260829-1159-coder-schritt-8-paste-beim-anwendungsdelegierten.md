# Coder — Schritt 8: die dritte Antwort beim Anwendungsdelegierten

**Circle:** 260828-1041-dateilistenfilter-nimmt-eingaben-per-paste
**Plan:** planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md, Schritt 8
**Datei:** crates/krk-ui/src/appkit/anwendung.rs (einzige geänderte Datei)

## Was geändert ist

- `define_class!`: `#[unsafe(method(paste:))] fn filter_einfuegen_aktion(&self, _absender: Option<&AnyObject>)` neben `dateien_ausschneiden_aktion`, Einzeiler auf `self.einfuegen_ausfuehren()`. Namenskollision mit `grep` geprüft: keine.
- `impl`: neuer privater Helfer `bearbeiten_am_dateifenster(&self, tun: impl FnOnce(&DateifensterQuelle))` trägt den Vorspann (Lage, `dateiablage_zulaessig`, Löschregel der Befehlsantwort, aktive Seite). `dateiablage_ausfuehren` ruft ihn mit `dateiverweise_ablegen(befehl)`, `einfuegen_ausfuehren` mit `aus_zwischenablage_einfuegen()`. Import `DateifensterQuelle` aus `super::tabelle`.
- `validateMenuItem:`: Zweig um `paste:` erweitert; Doc sagt, dass drei Selektoren der Regel unterstehen und der letzte Zweig für die übrigen fremden Aktionen bleibt.
- Prosa: Modulkopf-Abschnitt heißt „Drei Antworten ohne Kommando: `copy:`, `cut:` und `paste:` (Runden 22 und 21)"; Untergrenzen-Satz nennt `paste:` als dritten erklärten Selektor; „kein `paste:` daneben" an `copy:` gestrichen.
- Probe `dateiablageproben`: `der_delegierte_beantwortet_copy_cut_und_paste`, `responds_to` für alle drei bejaht.

## Verifikation

- `cargo test -p krk-ui -- dateiablageproben zulaessigkeit` — exit 0, 23 Proben grün, darunter `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` (2 Rufer: `validateMenuItem:` und `bearbeiten_am_dateifenster`).
- `make check` — exit 0. Die als rot erwartete Kern-Zählprobe war zum Zeitpunkt des Laufs schon grün (Schritt 9 lag offenbar bereits im Baum); nichts sonst rot.

## Status

Complete. Nicht committet.

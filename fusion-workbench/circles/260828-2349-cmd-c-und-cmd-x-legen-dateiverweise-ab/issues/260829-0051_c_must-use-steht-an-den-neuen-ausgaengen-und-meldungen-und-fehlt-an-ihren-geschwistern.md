`#[must_use]` steht an den neuen Ausgängen und Meldungen der Runde 22 und fehlt an ihren Geschwistern derselben Bauart
---
Die Runde 22 trägt das Attribut an `dateiverweise_auf_ablage_schreiben`, `dateiverweise_schreiben`, `dateiablage_zulaessig`, `ablagemeldung` und `verweise_abgewiesen`. In denselben Dateien stehen Funktionen derselben Bauart ohne das Attribut, und keine Stelle sagt, warum. Dazu zählt ein Doc-Kommentar drei Abnehmer, wo vier sind.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** coder
**Cross-references:** `reviews/260829-0051-coderev-runde-22-dateiverweise-in-der-zwischenablage.md` (Themen 1 und 3); CLAUDE.md, „Ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`".

Befund am `38aa652`:

- `crates/krk-ui/src/appkit/zwischenablage.rs:322` `pub fn text_schreiben(text: &str) -> bool` ohne Attribut; die Schwester `text_auf_ablage_schreiben` (`:310`) und die zwei neuen Ausgänge (`:358`, `:394`) tragen es. Rufer heute: `tabelle.rs:1880`, `:1908` (`if`), `betrachter.rs:371` (`let _ =`).
- `crates/krk-ui/src/kommandos/operationen.rs`: ohne Attribut `kein_terminal :895`, `kopiermeldung :960`, `nichts_zu_kopieren :978`, `nichts_zu_oeffnen :987`, `nichts_zu_teilen :1025`, `ablage_weist_ab :1119`, `oeffnungsmeldung :1250`; mit Attribut `nichts_zu_packen :1006`, `kein_archiv :1046`, `mehrere_archive :1065`, `kein_finder :1086`, `ablagemeldung :1185`, `verweise_abgewiesen :1211`. Die Grenze verläuft nach der Runde, in der die Funktion entstand (4 und 6 ohne, 17 und 22 mit), nicht nach ihrer Bauart.
- `crates/krk-ui/src/appkit/anwendung.rs:3135-3140`, Doc-Kommentar von `lage()`: „Drei Abnehmer lesen sie" und die Liste von dreien; seit `1644ada` ist `dateiablage_ausfuehren` (`:3188`) der vierte, und `validateMenuItem:` fragt in zwei Zweigen (`:961`, `:968`).

**Abnahme:** Die Funktionen der zwei Listen tragen das Attribut oder ihr Doc-Kommentar sagt, warum nicht; `cargo clippy --workspace --all-targets -- -D warnings` bleibt grün; der Doc-Kommentar von `lage()` nennt seine Abnehmer ohne Zahl oder mit der Zahl, die die Liste hat.

---
Abgleich 260829-0734: bleibt offen. Keiner der Commits nach `38aa652` (`35b95b3`) fasst Code an; `operationen.rs` trägt neun `#[must_use]`, `text_schreiben` (`zwischenablage.rs`) weiter keines, `lage()` zählt weiter drei Abnehmer.

---
Resolved: Behoben, alle drei Befunde, und zwei davon waren am `2fa1d0e` schon anders gelagert als am `38aa652`. (1) `text_schreiben` (`crates/krk-ui/src/appkit/zwischenablage.rs`) traegt jetzt `#[must_use]` mit Begruendung im Doc-Kommentar; das ist zugleich die Behebung von `260819-2230-…/issues/260820-0739_*`. (2) `crates/krk-ui/src/kommandos/operationen.rs`: die sieben genannten Funktionen tragen die Marke **schon** — nachgezaehlt am `2fa1d0e` einzeln vor `kein_terminal`, `kopiermeldung`, `nichts_zu_kopieren`, `nichts_zu_oeffnen`, `nichts_zu_teilen`, `ablage_weist_ab` und `oeffnungsmeldung`, und es gibt in dieser Datei keine Funktion dieser Bauart mehr ohne sie. Die Ungleichheit nach Runden ist damit ohne Zutun dieses Laufs gefallen; kein Code geaendert. (3) Der Doc-Kommentar von `Anwendungsdelegierter::lage` (`crates/krk-ui/src/appkit/anwendung.rs`) nennt seine Abnehmer jetzt namentlich statt gezaehlt — Kommandozweig, Zeichenzweig, `bearbeiten_am_dateifenster` und die Ausgrauung in `eintrag_pruefen`, die beide Eingaenge der Regel fragt — und nennt das Kommando, das die Stellen zaehlt. Geprueft: cargo test -p krk-ui 908 gruen, clippy/fmt/doc je Exit 0.

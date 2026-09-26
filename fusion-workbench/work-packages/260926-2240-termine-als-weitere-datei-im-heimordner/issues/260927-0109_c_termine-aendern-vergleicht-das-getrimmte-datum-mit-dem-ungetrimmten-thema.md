termine::aendern vergleicht das getrimmte Datum mit dem ungetrimmten Thema
---
`termine::aendern` (`crates/krk-core/src/heimordner/eintraege.rs`, `pub mod termine`) trimmt die Datumseingabe (`let datum = datum.trim();`) und vergleicht sie danach mit `alt.thema`, das `Notizen::notiz` roh liefert, allein ohne `## ` und ohne `\n`. Trägt die Kopfzeile Leerraum am Ende (`## 261002 `, `## xyz `) oder ein `\r` (Datei mit CRLF), gilt eine **unverändert verlassene** Datumszelle als geändert:

- Ist der getrimmte Text ein gültiges Datum (`261002 ` → `261002`), schreibt die Handlung die Kopfzeile still neu. Die Datei gilt danach als geändert, und auf dem Rückgängigstapel liegt eine Handlung, die der Nutzer nicht ausgeführt hat. Bei CRLF entsteht dabei eine Zeile mit LF zwischen Zeilen mit CRLF.
- Ist er ungültig (`xyz ` → `xyz`), antwortet `termindatum` mit `None`, und die Handlung weist mit `Abweisung::UngueltigesDatum` ab. Die Datumszelle lässt sich dann nicht verlassen, ohne sie mit `esc` zu verwerfen, und jeder Weg über `Editorbereich::zelle_uebernehmen` (Sichern, Ansichtswechsel, Schließen) bleibt `Abgewiesen`, solange sie offen steht.

Das widerspricht dem Doc-Kommentar derselben Funktion („Ein unverändert gelassenes ungültiges Datum wird also nicht geprüft“), `notizen::aendern` („eine Zelle, die ohne Änderung verlassen wird, ergibt keine Handlung“) und `HowTo.md` unter „Termine in `appointments.md`“. Die Notiztabelle ist nicht betroffen: `notizen::aendern` vergleicht ungetrimmt.

Der Weg dorthin: `Eintragsansicht::zelle_darf_enden` und `zelle_geendet` (`crates/krk-ui/src/appkit/eintragsansicht.rs`) reichen den Text des Feldeditors bei jedem Ende, auch einem ohne Änderung, über `Editorbereich::zelle_pruefen` / `zelle_festschreiben` an `zellenrechnung` (`crates/krk-ui/src/appkit/editor.rs`), und die Datumsspalte ruft dort `termine::aendern(stand, zelle.stelle, text, &alt.text)`. Die Zelle zeigt `Terminzeile::datum`, also das rohe Thema samt Leerraum.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260927-0109-reviewer-termine-als-weitere-datei-im-heimordner.md`; `260926-2253_*_spec-termine-als-weitere-datei-im-heimordner.md` (T2, T3, A3)

Gefunden durch Lesen des Codes, nicht am laufenden Bündel nachgestellt.

Abnahme:
- Eine Probe an `termine::aendern` mit dem Stand `## xyz \nalt\n` und der Datumseingabe `xyz ` (so, wie die Zelle sie zeigt) antwortet `Ok(None)`.
- Eine Probe mit `## 261002 \nText\n` und der Eingabe `261002 ` antwortet `Ok(None)`: eine unverändert verlassene Zelle ergibt keine Handlung.
- Eine geänderte Eingabe wird weiter vor der Prüfung getrimmt (die bestehende Probe mit ` 261005 ` bleibt grün).
- Ob eine Datei mit CRLF-Kopfzeilen ihre Termine als gültig lesen soll, ist eine eigene Frage und nicht Teil dieser Abnahme; die Probe hält allein fest, dass das Verlassen einer unveränderten Zelle dort nichts schreibt.

---
Resolved: `termine::aendern` vergleicht die Datumseingabe jetzt getrimmt mit dem ebenso getrimmten bisherigen Thema; gleich sie, geht das rohe Thema an `notizen::aendern` weiter, die Kopfzeile bleibt Byte für Byte, und geprüft wird nichts. Erst eine abweichende Eingabe wird getrimmt und gegen `termindatum` gehalten. Probe `eine_unveraenderte_datumszelle_mit_leerraum_am_ende_ergibt_keine_handlung` (`crates/krk-core/tests/heimordner.rs`): `## xyz ` mit `xyz `, `## 261002 ` mit `261002 ` und eine CRLF-Kopfzeile ergeben `Ok(None)`, und bei geändertem Text bleibt die Kopfzeile samt Leerraum stehen. Die bestehenden Proben mit ` 261005 ` und ` 261002 ` bleiben grün. Commit folgt mit diesem Datensatz (fix(termine)).

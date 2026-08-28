# Coder: Schritt 6 der Runde 22, der Modulkopf des Menüs

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 6
**Agent:** coder

## Was geändert ist

Nur Prosa, kein Code: `crates/krk-ui/src/appkit/menue.rs`, vier Stellen.

- Modulkopf, Absatz zur Antwortkette (vormals `:18-25`): `copy:` und `cut:` erreichen seit der Runde 22 als vierte Fläche den Anwendungsdelegierten, wenn kein Glied davor antwortet, also mit dem Fokus in der Dateiliste (`dateiablage_ausfuehren` in `anwendung.rs`).
- Modulkopf, Absatz „Kein zweiter Zweig in `validateMenuItem:`" (vormals `:83-91`), jetzt „Kein Sonderzweig": der Delegierte antwortet für jede fremde Aktion `true` außer für `copy:` und `cut:`, die er selbst beantwortet und deshalb `dateiablage_zulaessig` unterstellt; der Absatz nennt den Grund, aus dem das die Regel für jeden von KRK beantworteten Eintrag ist und kein Sonderfall, und dass `paste:` weiter `true` bekommt.
- Modulkopf, Absatz zum Einhängepunkt (vormals `:105-116`): der Einhängepunkt ist zur Hälfte besetzt, `copy:` und `cut:` beantwortet der Anwendungsdelegierte, der Menüeintrag heißt weiter „Kopieren" (A9), `paste:` beantwortet weiter niemand und gehört dem vorgesehenen Circle `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` samt dessen offenem Datensatz.
- Doc-Kommentar von `GEMESSEN` im Prüfmodul: die Tafel misst die sechs AppKit-Klassen und den Anwendungsdelegierten nicht; die Zeilen zu `copy:` und `cut:` bleiben, wie am 260811 gemessen; die Antwort des Delegierten hält die Probe aus Schritt 5 (`anwendung.rs`, `responds_to` an der Klasse).

Die Tafel `GEMESSEN` und `die_sechs_zugestellten` bleiben unverändert: sie zählen Ersthelferklassen, und der Delegierte ist keine. `resources/default-keymap.toml` ist nicht angefasst (Constraint 7); die zwei Kommentare dort, die von „einer späteren Runde" sprechen, bleiben unter `## Open Questions` des Plans.

## Verifikation

- `make check`: exit 0, alle vier grün. Beim ersten Lauf und beim zweiten; zwischen beiden waren die Schritte 4, 5 und 7 in `tabelle.rs`, `anwendung.rs` und `betrachter.rs` schon gespeichert (`git show HEAD:<pfad> | diff` zeigt alle drei geändert), und keine Probe aus diesen Schritten war rot.
- Vergleich gegen HEAD: `git show HEAD:crates/krk-ui/src/appkit/menue.rs | diff - crates/krk-ui/src/appkit/menue.rs` — allein Kommentarzeilen.

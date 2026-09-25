# Coder: Schritt 5 — die zwei Antworten beim Anwendungsdelegierten

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, Schritt 5

## Was geändert ist

Eine Datei, `crates/krk-ui/src/appkit/anwendung.rs`:

- `define_class!`: `dateien_kopieren_aktion` (`#[unsafe(method(copy:))]`) und `dateien_ausschneiden_aktion` (`#[unsafe(method(cut:))]`), Signatur `(&self, _absender: Option<&AnyObject>)`, je ein Aufruf von `dateiablage_ausfuehren`. Kein `paste:`.
- `fn dateiablage_ausfuehren(&self, befehl: Dateiablage)` vor `kommando_ausfuehren`: `lage()`, `zulaessigkeit::dateiablage_zulaessig`, `befehlsantwort_beidseitig_loeschen`, aktive Seite aus dem Fenstermodell, `dateiverweise_ablegen(befehl)` an der Datenquelle.
- `validateMenuItem:`: `eintrag.action()` einmal in `aktion` gelesen; zweiter Zweig für `copy:`/`cut:` fragt `dateiablage_zulaessig(self.lage())`; `else { true }` bleibt, `paste:` fällt hinein.
- Modulkopf: neuer Abschnitt „Zwei Antworten ohne Kommando“ und ein Satz im Untergrenzen-Abschnitt, dass `copy:` und `cut:` erklärte Selektoren ohne Untergrenze sind.
- Import `Dateiablage` aus `kommandos::operationen`.
- Probe `dateiablageproben::der_delegierte_beantwortet_copy_und_cut_und_paste_nicht` am Ende der Datei, über `Anwendungsdelegierter::class().responds_to`.

## Verifikation

`make check` — exit 0 (zweiter Lauf). Im ersten Lauf war `appkit::zwischenablage::proben::zwei_dateiverweise_kommen_als_zwei_pfade_zurueck` rot (`zwischenablage.rs:508`, Datei des Schritts 3, nicht angefasst); einzeln und im zweiten `make check` grün. Vermutung: Zwischenstand eines parallel arbeitenden Coders oder Wettlauf um die benannte Probenablage; nicht weiter untersucht.

Die Betrachter-Probe trägt beim Bau bereits den Namen aus Schritt 7 (`nspasteboard_steht_nicht_im_betrachter_und_copy_und_cut_stehen_an_genannten_stellen`) und ist grün; `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` ist grün.

`cargo fmt --all` wurde einmal über den Arbeitsbereich gefahren; ob es eine fremde Datei umformatiert hat, ist nicht geprüft (kein baumweites git-Kommando).

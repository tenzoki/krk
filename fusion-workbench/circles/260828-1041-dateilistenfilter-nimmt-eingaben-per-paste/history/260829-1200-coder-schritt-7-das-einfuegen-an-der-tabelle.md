# Coder: Schritt 7 — Das Einfügen an der Tabelle

**Status:** Complete
**Plan:** `planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 7
**Kriterien:** C1.1–C1.4, C2.8, C4.6, A7, A8

## Was gebaut ist

- `crates/krk-ui/src/appkit/tabelle.rs`: `pub fn aus_zwischenablage_einfuegen(&self)` neben `dateiverweise_ablegen`. Liest `super::zwischenablage::einfuegequelle()`, deutet mit `krk_core::zwischenablage::filtertext_aus`; `Ok` hängt in einer Ausleihe `text_anhaengen` an und ruft danach einmal `nach_filteraenderung()`; `Err` schreibt `operationen::einfuegen_abgewiesen(hindernis)` über `befehlsantwort_zeigen`. Doc-Kommentar: zweiter Eingang neben `filterzeichen_tippen`, Vertrag der Reinigung, Ausleihe endet vor dem Nachzug, kein Satz bei Erfolg. Doc von `nach_filteraenderung` nennt das Einfügen als dritten Rufer; der Modulkopf nennt das abgewiesene Einfügen als weiteren Weg durch `befehlsantwort_zeigen`.
- `crates/krk-ui/src/appkit/zwischenablage.rs`: die Zeile `#[allow(dead_code)]` an `einfuegequelle` entfernt.
- Keine neue AppKit-Berührung; der Untergrenzen-Abschnitt von `tabelle.rs` bleibt unverändert (C4.6).
- Plan Schritt 7 auf `[DONE]`.

## Verifikation

- `cargo test -p krk-ui -- tabelle` — exit 0 (20 Proben).
- `make check` — exit 0. Die im Dispatch erwartete rote Zählprobe war nicht rot: Schritt 9 steht bereits im Baum (`die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei` erwartet `tabelle.rs` als Rufer).
- Ein erster Lauf traf einen Übersetzungsfehler in `anwendung.rs` (`DateifensterQuelle` nicht im Scope) aus der parallelen Bearbeitung von Schritt 8; beim Wiederholungslauf war er weg. Nicht von mir angefasst.

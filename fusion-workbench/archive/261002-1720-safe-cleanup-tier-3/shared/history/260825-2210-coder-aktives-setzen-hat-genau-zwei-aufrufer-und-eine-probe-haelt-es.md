# `aktives_setzen` hat genau zwei Aufrufer, und eine Probe hält es

**Agent:** coder
**Datum:** 2026-08-25, ab 22:05
**Aufgabe:** R-3, Runde 2 der Sitzung zur Runde 18 — Befund M2 der Durchsicht
`fusion-workbench/shared/reviews/260825-2127-coderev-runde-18-vorschau-vertieft-und-zwei-fehler.md`,
Datensatz `shared/issues/260825-2127_*_ein-dritter-weg-nach-aktives-setzen-haelt-den-bau-nicht-an-und-keine-probe-faengt-ihn.md`
**Status:** Complete

## Was entstanden ist

**Eine Zählprobe an der Naht.** `aktives_setzen_hat_genau_zwei_aufrufer` steht im Modul
`aktivschreiberproben` von `crates/krk-ui/src/appkit/anwendung.rs` (Zeile 8957) vor
`keine_vierte_tuer_schreibt_das_aktive_dateifenster`. Sie liest die Datei über
`zettelproben::diese_datei`, zählt mit `quellbaum::aufrufstellen` und hält die Zahl auf zwei.
Die Form ist die der vorhandenen Aufruferzählungen des Moduls: Nadel per `concat!` geteilt,
Doc-Kommentar mit Anlass, Begründung der Aufruferzählung über den Kopf von `quellbaum` und
dem Absatz „Was sie nicht sieht".

**Der berichtigte Satz.** Der Doc-Kommentar von `Rangmitnahme` in
`crates/krk-ui/src/appkit/tabelle.rs` sagte, ein dritter Weg in `angefasst` halte den Bau an.
Er sagt jetzt, was wirklich hält: der Übersetzer gegen einen dritten Wert, die zwei Zählproben
gegen einen dritten Weg, beide beim Namen genannt.

## Gegenprobe

Sicherungskopie von `anwendung.rs` im Scratchpad, dann ein dritter Ruf
`self.aktives_setzen(seite, Rangmitnahme::Appkit)` in `aktives_dem_ersthelfer_nachziehen`
eingefügt: `cargo test -p krk-ui aktives_setzen_hat_genau_zwei_aufrufer` rot mit der Meldung
„hat nicht mehr genau zwei Aufrufer". Aus der Kopie zurückkopiert, `cmp` byteidentisch.

## Was ausdrücklich nicht angefasst wurde

`crates/krk-core/`, `resources/`, `Cargo.toml`; kein Git-Kommando außer `git status` auf
`crates/krk-ui`. Die Probe zählt allein in `anwendung.rs`, weil `aktives_setzen` privat ist;
eine Zählung über den ganzen Baum wäre hier nur teurer, nicht dichter.

## Abnahme

`Verification: make check — exit 0` (mit `PATH="$HOME/.cargo/bin:$PATH"`, „alle vier gruen";
`cargo fmt -p krk-ui` vorher gefahren, `--all` nicht).

Datensatz auf `_c_` mit Auflösungsvermerk.

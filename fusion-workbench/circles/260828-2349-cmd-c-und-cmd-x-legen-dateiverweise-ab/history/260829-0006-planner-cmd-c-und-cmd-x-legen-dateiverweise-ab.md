# Planner-Sitzung: Implementierungsplan der Runde 22, Cmd+C und Cmd+X legen Dateiverweise ab

**Date:** 2026-08-29, 260829-0006
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`
**Spec:** `planning/260829-0005_*_spec-cmd-c-und-cmd-x-legen-dateiverweise-ab.md` (vorab freigegeben)
**HEAD:** `4cfb454`

## Was getan wurde

Den Spec, den Circle-Datensatz, den Plan der Runde 20 als Vorbild und die betroffenen Stellen im Baum gelesen: `zwischenablage.rs` ganz, `zulaessigkeit.rs` bis zur Regel und die Proben am Ende, `menue.rs` Modulkopf und Prüfmodul (`GEMESSEN`, `wer_antwortet`), `anwendung.rs` an `krkKommando:`, `validateMenuItem:`, `lage`, `kommando_ausfuehren`, `bereichskommando`, `fokus`, `ist_eigene_textflaeche`, `tabelle.rs` an `betroffene_eintraege`, den zwei Pfadkopierern und `befehlsantwort_zeigen`, `operationen.rs` an `betroffene`, `pfadzeilen`, `kopiermeldung`, `nichts_zu_kopieren`, `ablage_weist_ab`, `eintragsname`, `betrachter.rs` an `copy:` und der Zählprobe, `quellbaum.rs` an `aufrufstellen`, `vorschau.rs` an `auswahl_ablegen`, `abwurf.rs` an `sorten`, `fokus.rs` an `wirkt`, sowie die Kommentare `resources/default-keymap.toml:75-90`, `:985-1000`, `:1030-1052`. Offene Entscheidungen im gemeinsamen Speicher und im Circle `260828-1041` gelistet.

Den Plan geschrieben: `planning/260829-0006_o_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`, neun Schritte, davon acht für `coder` und der neunte der Abnahmelauf des Nutzers; kein Schritt für `ontocoder` (die Belegung bleibt nach Constraint 7 unangetastet) und keiner für `analyst` (kein strategisches Deliverable). Die Schritte 1 bis 3 haben disjunkte Dateien und keine Vorbedingung.

Die sieben Planerfragen des Specs beantwortet: Antwort am Anwendungsdelegierten; Zulässigkeit über einen privaten Rumpf `gestattet(Anspruch, Lage)` mit zwei Hüllen `zulaessig` und `dateiablage_zulaessig`; `writeObjects:` mit `NSURL` je Eintrag, danach `setString:forType:` auf den ersten Eintrag; Signatur mit Pfaden und fertigen Namenszeilen; die Zählprobe im Betrachter zählt `copy:` und `cut:` mit ausgeschriebenen Stellen; die Meldung über `DateifensterQuelle::dateiverweise_ablegen` und `befehlsantwort_zeigen`; keine Ordnungszahl für die Abnehmer von `betroffene`.

## Befunde gegen den Baum

- Der Spec zählt die Abnehmer von `betroffene()` bis sechs; der Baum zählt das Teilen als siebten (`teilen.rs:182`, `anwendung.rs:3791`).
- C5.5 sagt, `fileURLWithPath:` stehe im Untergrenzen-Abschnitt der Hülle; er steht dort nicht (`zwischenablage.rs:141-166`), nur im Prüfmodul (`:384`).
- C5.1 nennt für `grep -rn NSPasteboard` außerhalb der Hülle drei Dateien; es sind fünf, die zwei weiteren (`mod.rs`, `teilen.rs`) in Kommentaren.
- Gefiled als `issues/260829-0006_o_drei-baumaussagen-des-specs-der-runde-22-stimmen-mit-dem-baum-nicht-ueberein.md`.

## Was nicht getan wurde

Kein Code angefasst, nichts committet, keine Agenten dispatcht. Kein Entscheidungsdatensatz gefiled: die offenen Fragen des Plans binden nur dieses Dokument oder sind Messungen am Bündel.

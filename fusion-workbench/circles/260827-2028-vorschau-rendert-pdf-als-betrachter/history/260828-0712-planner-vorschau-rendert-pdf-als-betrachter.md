# Planner-Sitzung — 260828-0712

**Aufgabe:** der Implementierungsplan der Runde 20, die Vorschau rendert PDF als Betrachter
**Circle:** 260827-2028-vorschau-rendert-pdf-als-betrachter (aktiv)
**Spec:** `planning/260828-0649_o_spec-vorschau-rendert-pdf-als-betrachter.md`, freigegeben am 260828
**Status:** Complete

## Was gelesen wurde

Der Circle-Datensatz mit Grounding, der beantwortete Datensatz zu den Tasten, der Spec vollständig, der Plan der Runde 19 als Muster, die drei geerbten Defekte und der Defekt zum achten `Wirkungsbereich`, die zwei archivierten Datensätze der Runde 14 zu Pfeiltasten und Kontextmenü, der Datensatz der Runde 6 zu den Tastenkombinationen. Im Baum: `vorschaumodell.rs` (Modulkopf, `Inhalt`, `laden`, `ist_bildpfad`, die Bildgrenzenprobe), `appkit/vorschau.rs` (Klassen, Ivars, `anzeigen`, `text_zeigen`, `bild_zeigen`, `fokusansicht`, Abfangstelle, Zählproben), `appkit/statuszeile.rs` (`Rang`, `Quellen`, `zeile`, `zeilentext`, `zeigen`, Proben), `tasten/belegung.rs` (`Wirkungsbereich`, `beschriftung`, `KENNUNGEN`, Zusteller), `tasten/parser.rs` (`Tastenkennung`, `Taste::kennung`, `TASTEN`, `zeichen_als_kennung`, Proben), `tasten/mod.rs` (`Tastendruck`), `appkit/ereignisse.rs` (`gemeldetes_zeichen`, `behandeln`, `ersthelfer_gehoert_appkit`), `appkit/menue.rs` (`zeichen_der_taste`, Selektorregel), `belegungsmodell.rs`, `belegungsausgabe.rs`, `kommandos/zulaessigkeit.rs`, `kommandos/fokus.rs` (`wirkt`, Tafel), `appkit/anwendung.rs` (`ist_eigene_textflaeche`, `kommando_ausfuehren`, `bereichskommando`, `statuszeile_nachziehen`, `bereich_des_ersthelfers`, `fokusansicht`, `teilen`), `appkit/tabelle.rs` (`meldungsquellen`), `appkit/zwischenablage.rs`, `appkit/teilen.rs`, `appkit/fenster.rs`, `appkit/fsevents.rs`, `quellbaum.rs`, `tests/belegung.rs`, `tests/gemeinsam/mod.rs`, `Cargo.toml`, `Cargo.lock`, `resources/default-keymap.toml`, `Makefile`. Von crates.io: `objc2-pdf-kit` 0.3.2 heruntergeladen und `Cargo.toml`, `PDFView.rs`, `PDFDocument.rs`, `PDFSelection.rs` gelesen.

## Was geschrieben wurde

- Plan: `planning/260828-0712_o_plan-vorschau-rendert-pdf-als-betrachter.md` — elf Schritte (acht `coder`, ein `ontocoder`, ein `analyst`, der Abnahmelauf als Nutzerschritt), die zehn Entscheidungen aus `## Open for Planner` ausgeschrieben, zwei Mermaid-Diagramme zur Struktur und ein Abhängigkeitsgraph, Stop-Bedingungen, Teststrategie, Risiken, offene Fragen.
- Entscheidung: `decisions/260828-0712_o_wie-erreicht-eine-us-tastaturbelegung-cmd-plus-wenn-das-pluszeichen-dort-die-umschalttaste-braucht.md` — der Abgriff liest das Zeichen ohne Zusatztasten, und auf einer US-Belegung trägt keine Taste ohne Zusatztaste ein `+`; drei Möglichkeiten mit Folgen, Empfehlung Möglichkeit 1, die der Plan baut.
- Defekt: `issues/260828-0712_o_der-spec-nennt-make-tasten-fuer-die-markdown-ausgabe-der-belegung-die-aus-dem-menue-kommt.md` — C1.3 und C3.6 nennen `make tasten`, das Ziel protokolliert Tastencodes; die Markdown-Datei kommt aus dem Menüeintrag über `belegungsausgabe.rs`.

## Die tragenden Entscheidungen, kurz

`PDFView` über `objc2-pdf-kit` 0.3.2 ohne Vorgabemerkmale (kein C, `build = false`); `Inhalt::Pdf` nach dem Muster von `Bild`, Grenze `BILDGRENZE`, Deutung in der Ansicht, Rückfall bei `initWithData == None` und `isLocked`; eine Aufzählung `Flaeche` mit einem Schalter statt dreier `setHidden`-Paare; `plus`/`minus` über eine Funktion `zeichen_des_namens`, die auch das Menükürzel bedient, mit dem Zehnerblock als Stelle; `Wirkungsbereich::Vorschau` zurück mit drei Trägern und der Variantenprobe, die den Defekt `260826-1302` schließt; `Rang::Seitenzaehler` mit einer `Herkunft`, weil die Vorschau keine Fensterseite ist; `copy:` in der Unterklasse als Abfangstelle, über `zwischenablage::text_schreiben`; keine Anmeldung bei `ist_eigene_textflaeche`, weil `PDFView` keine Textklasse ist (mit Risikozeile).

## Was nicht getan wurde

Kein Code angefasst, nichts committet, kein Agent gestartet. Die Antwort auf den US-Belegungs-Datensatz ist keine Vorbedingung des Plans.

# Planner-Sitzung: Implementierungsplan der Runde 21, Einfügen in den Filter und `*` als Platzhalter

**Date:** 2026-08-29, 260829-1102
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste`
**Spec:** `planning/260829-1052_*_spec-einfuegen-in-den-filter-und-stern-als-platzhalter.md` (vorab freigegeben, autonome Runde)
**HEAD:** `1e44b01`
**Executors im Dispatch:** coder, ontocoder, analyst

## Was getan wurde

Den Spec, den Circle-Datensatz (Grounding teils überholt, dem Spec gefolgt), den offenen Datensatz `decisions/260828-1041_*_…` und die Pläne der Runden 22 und 20 als Vorbild gelesen. Im Baum gelesen: `filter.rs` ganz, `modell.rs` am Prüfschritt, an den Filterzugängen, an `inhalt_wirkt` und `filter_uebernehmen`, `durchlauf.rs` an `starten`, `Auftragslage`, `durchlauffaden`, `datei_entscheiden`, `unterbaum_entscheiden`, `inhalt.rs` an `traegt_der_inhalt`, `tabs.rs` an `durchlauf_nachziehen_an`, `krk-core/src/zwischenablage.rs` ganz, `tests/verzeichnis.rs` an der Zählprobe, an `code_zeilen`, an den `filter_klein`- und `traegt_der_inhalt`-Rufern und an der Probe zu C6.9, `gemeinsam/mod.rs` an `quelldateien`; in `krk-ui`: `zulaessigkeit.rs` bis zur Regel und die vier Zählproben, `anwendung.rs` am Modulkopf, an `copy:`/`cut:`, `validateMenuItem:`, `eingabe_ausfuehren`, `dateiablage_ausfuehren`, `lage`, `dateiablageproben` und dem Untergrenzen-Abschnitt, `appkit/zwischenablage.rs` am Kopf, an `lesen`, `inhalt_lesen`, `dateiverweise` und dem Untergrenzen-Abschnitt, `tabelle.rs` an `filterzeichen_tippen`, `nach_filteraenderung`, `befehlsantwort_zeigen`, `dateiverweise_ablegen`, `quelle`, `operationen.rs` am Block der Runde 22 und an `zahl`, `menue.rs` am Kopf und an `GEMESSEN`, `belegungsmodell.rs` an `zeile_traegt`, `betrachter.rs` an der Zählprobe, `quellbaum.rs` an `aufrufstellen`, `statuszeile.rs` an `Rang` und `filterstand_text`, `resources/default-keymap.toml:78-86`, `:988-998`, `:1045-1051`. Offene Entscheidungen in beiden Speichern gelistet: eine im Circle, zweiundzwanzig `_o_`/`_a_` im gemeinsamen Speicher; keine hält einen Planschritt auf.

Den Plan geschrieben: `planning/260829-1102_o_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, zwölf Schritte, davon zehn für `coder`, einer für `ontocoder` (die zwei Kommentare der Belegungsdatei, die C4.5 verlangt), der zwölfte der Abnahmelauf des Nutzers; keiner für `analyst`. Die Schritte 1 bis 4 haben disjunkte Dateien und keine Vorbedingung; der Platzhalter-Ast (Schritt 1) und der Einfüge-Ast (2 bis 8) treffen sich in Schritt 7.

Die neun Planerfragen des Specs beantwortet: `paste:` am Anwendungsdelegierten; `dateiablage_zulaessig` bleibt der Eingang, kein dritter `Anspruch`, ein privater Helfer `bearbeiten_am_dateifenster` trägt den Vorspann der drei Selektoren einmal (Frager-Zählprobe bleibt bei zwei); ein Leser `einfuegequelle()` aus `dateiverweise` vor `lesen_aus`; die Reinigung als `krk_core::zwischenablage::filtertext_aus(&Einfuegequelle) -> Result<String, Einfuegehindernis>` mit vier Hindernissen für die vier Sätze; der Doppelpunkt als eine Zeile neben der Zeichenregel, die Zählprobe bekommt den dritten Rufer mit Namen; `Ordnermodell::text_anhaengen` mit einem `filter_uebernehmen`; ein Typ `filter::Muster`, der `filter_klein` ersetzt und als Wert in den Durchlauf reist, `traegt_die_folge(name, &Muster)` ohne Rückverfolgung; die Schwelle zählt am Filtertext die Zeichen ohne `*`; die Befehlsantwort über `befehlsantwort_zeigen` an der Tabelle.

## Befunde gegen den Baum

- Die Zählprobe `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` liest über `gemeinsam::quelldateien` **alle** `.rs`-Dateien unter `crates/`, auch die Probendateien; eine neue Probe in `tests/verzeichnis.rs`, die `traegt_die_folge` in einer Codezeile nennt, wäre ein vierter Rufer. Der Plan legt die Vergleichsproben deshalb in die Heimat `filter.rs` und die Modellproben über `Ordnermodell`, `Durchlauf` und `traegt_der_inhalt`.
- `#[must_use]` an einer `Result`-Funktion löst `clippy::double_must_use` aus; C4.4 nennt die Reinigung als Träger des Attributs, und der Plan liest es als „die Antwort ist `must_use`", was `Result` von sich aus ist.
- C1.9 („keine neue Zeile in `default-keymap.toml`") und C4.5 (die zwei Kommentare dort ziehen nach) stehen nebeneinander; der Plan liest C1.9 als „keine Belegungszeile" und hält es mit einem Diff ohne Kommentarzeilen fest. Unter `## Open Questions` festgehalten.
- Alle vom Spec zitierten Zeilenangaben, die der Plan nachgelesen hat, treffen den Baum auf `1e44b01`; kein Defekt am Spec gefunden.

## Was nicht getan wurde

Kein Code angefasst, nichts committet, keine Agenten dispatcht. Kein Entscheidungsdatensatz und kein Defekt gefiled: die offenen Fragen des Plans binden nur dieses Dokument oder sind schon als Datensätze vorhanden und zitiert.

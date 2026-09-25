# Shaper, user-direct: der Spec der Runde 22 (Cmd+C und Cmd+X legen Dateiverweise ab)

**Datum:** 2026-08-28, 23:57 bis 2026-08-29, 00:08
**Filed by:** shaper (user-direct, vom Orchestrator dispatched), Kai Stalmann
**Modus:** user-direct innerhalb des aktiven Circles `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`; keine Klärungsrunde auf Weisung des Nutzers („autonom bauen"), Spec-Tor vorab freigegeben
**Eingabe:** `_t_circle.md` (Directive, Grounding snapshot mit fünf Festlegungen), CLAUDE.md, der Spec der Runde 20 als Vorbild
**Status:** Complete

## Gegen den Baum gelesen

Alle fünf Festlegungen des Datensatzes halten am Stand `83e011c`:
- Einhängepunkt: `default-keymap.toml:988-997` und `:1035-1043`, `menue.rs:105-116`, `GEMESSEN` (`menue.rs:869-871`).
- `betroffene()` (`operationen.rs:170`), Ausleihe `tabelle.rs:1833`, Muster `eintragspfad_kopieren` (`tabelle.rs:1897-1909`), Meldungen `nichts_betroffen` (`:1094`), `ablage_weist_ab` (`:1111`).
- Hülle: `text_auf_ablage_schreiben` mit `#[must_use]`, `dateiverweise` über `readObjectsForClasses:`, `dateien_ablegen` im Prüfmodul mit `writeObjects:`, `probenablage` über `pasteboardWithName:`; Untergrenzen-Abschnitt nennt `writeObjects:` und `fileURLWithPath:` schon.
- Zählprobe `betrachter.rs:713-752` zählt `copy:`-Überschreibungen über den ganzen Quellbaum.
- Statuszeile: `Rang::Befehlsantwort`, `befehlsantwort_zeigen` (`tabelle.rs:3306`).

Eine Abweichung zum Datensatz, im Spec präzisiert (C5.1): „einzige Datei, die `NSPasteboard` anspricht" ist zu stark. `abwurf.rs::sorten` nennt `NSPasteboardTypeFileURL` für die Anmeldung des Abwurfs, `vorschau.rs::auswahl_ablegen` nimmt die Ablage als Parameter und reicht sie an die Hülle weiter. Der Constraint lautet deshalb: keine Datei außer der Hülle liest oder schreibt eine Ablage.

## Lücken, nach dem Muster entschieden (A6 bis A12)

Wortlaut der vier Meldungen (A6), Verknüpfung als Verknüpfung (A7), nur Sichtbares (A8), Namen in Menü und Belegungsansicht unverändert (A9), kein fünfter Kontextmenüeintrag (A10), Zulässigkeit nach der Regel eines Dateifenster-Kommandos ohne zweiten Zweig in `validateMenuItem:` (A11), `#[must_use]` und Probe über eine benannte Probenablage (A12). Keine widerspricht der Directive; keine Runde zurückgegeben. Kein Entscheidungsdatensatz angelegt, weil nichts aufgeschoben ist; kein Defekt gefunden.

## Ergebnis

Spec `planning/260829-0005_o_spec-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`: Directive, Abschnitt zu den zehn Zeitzusagen (keine elfte, keine angefasst; Schreibseite als Auskunft, nicht als Zusage), zwölf Festlegungen, fünf Capabilities mit 40 Abnahmekriterien, Sicherheitsüberlegung (öffentliche Ablage, Versteckte, Verknüpfungen, verschwundene Einträge, was KRK nie tut), sieben Constraints, Out of Scope, Open for Planner, drei offene Nutzerfragen. Der Datensatz `_t_circle.md` ist nicht angefasst; `**Active spec/plan:**` setzt der Orchestrator. Nichts committet.

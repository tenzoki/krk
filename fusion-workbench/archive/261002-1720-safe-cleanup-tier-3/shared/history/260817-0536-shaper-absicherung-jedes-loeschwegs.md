# Shaper: Spec für die Absicherung jedes Löschwegs in KRK

**Datum:** 2026-08-17 05:36
**Agent:** shaper (user-direct)
**Status:** Complete
**Baumstand:** `b8e198e`

## Auftrag

Den Spec für die Absicherung jedes Löschwegs schreiben, nach zwei bereits gefahrenen Klärungsrunden und der Verschärfung vom 260817, die den endgültigen Löschweg ganz streicht. Kein Circle war aktiv; die Ablage ist deshalb der gemeinsame Speicher.

## Gelesene Grundlagen

- `shared/issues/260816-2144_o_das-raeumen-in-den-papierkorb-laeuft-ohne-rueckfrage.md` samt dem Abschnitt `## Verschärfung vom 260817`
- `shared/analyses/260817-0419-verlust-des-speichers-shared.md`
- `shared/decisions/260802-0842_i_loeschen-papierkorb-oder-endgueltig.md`
- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitte C3, C4, C8 und C9, sowie der Abschnitt `## Directive` des Circle-Datensatzes
- die vier bestehenden Specs unter `shared/planning/` und `circles/*/planning/`, um keine bereits zugesagte Fähigkeit ein zweites Mal zu spezifizieren
- am Baum: `operation/{auftrag,loeschen,mod,verschieben}.rs`, `tasten/belegung.rs`, `ablage/pfade.rs`, `verzeichnis/{modell,filter,durchlauf}.rs`, `krk-ui/src/{auffrischung,belegungsmodell,menuemodell}.rs`, `kommandos/{operationen,rueckschritt,fokus}.rs`, `appkit/{anwendung,papierkorb}.rs`, `appkit/blaetter/{mod,loeschbestaetigung}.rs`, `resources/default-keymap.toml`, `crates/krk-bench/src/messen.rs`

## Was entstanden ist

- **Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, sechs Fähigkeiten C1 bis C6.
- **Entscheidungsdatensatz, beantwortet:** `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`. Er hält die Antworten vom 260816 und 260817 und hebt die Festlegung vom 260802 auf.
- **Drei offene Entscheidungsdatensätze**, alle während dieser Klärung entstanden: die gespeicherte `keymap.toml` mit der entfallenen Kennung, die Neuvergabe von `f8`, und die Reichweite der Git-Prüfung.

## Was am Baum nachgezählt wurde

Die Zahlen des Auftrags sind nicht übernommen, sondern selbst erhoben. Gegen `b8e198e`: siebzehn Nennungen der beiden Aufzählungswerte `Kommando::EndgueltigLoeschen` und `Art::EndgueltigLoeschen` in neun Dateien, dazu die Länge des Feldes `KENNUNGEN` (heute 79), die Kernfunktion `loeschen::endgueltig_loeschen` samt ihrer Probe, der Erzeuger `Auftrag::endgueltig_loeschen`, `Anwendungsdelegierter::endgueltig_loeschen`, der Eintrag in `resources/default-keymap.toml` und sechsundvierzig Nennungen in Kommentaren über zwanzig Dateien.

## Vier Befunde, die die Klärung nicht kannte

1. **`baum_entfernen` bleibt und ist kein Löschbefehl.** Zwei Stellen im Kern entfernen weiterhin Bäume ohne Papierkorb, das Ersetzen eines Ziels und das Verschieben über eine Datenträgergrenze. Ohne diese Einschränkung wäre die Zusage „kein Weg zum unwiederbringlichen Löschen" am Baum widerlegbar.
2. **Eine gespeicherte Nutzerbelegung wird ganz verworfen**, sobald sie eine unbekannte Kennung führt. Wer je gesichert hat, verliert damit seine gesamte Belegung. Eigener Datensatz.
3. **Die Norton-Reihe verliert ihre Löschtaste**, wenn `f8` ersatzlos frei wird; C3 der Runde 1 sagt jede Norton-Funktion auf zwei Wegen zu. Eigener Datensatz.
4. **Keine der fünf Zielprüfungen hätte beim Schadensfall angeschlagen.** Der geräumte Ordner liegt unter dem Benutzerordner, lokal, außerhalb der beiden Cloud-Orte und trägt kein `.git`. Verhindert hätten ihn die unbedingte Rückfrage und die Umfangsschwelle. Der Befund steht im Spec als eigener Abschnitt und hat den dritten offenen Datensatz ausgelöst.

## Zeitzusagen

Keine der zehn Zusagen aus C8 der Runde 1 ist berührt; geprüft gegen die Tabelle in C8 und gegen die Kennungen in `krk-bench/src/messen.rs`. Diese Runde setzt keine elfte Zahl. Ungemessen bleibt, was die neue Prüfung vor dem Vorgang auf dem Hauptfaden kostet; beschränkt ist sie durch den Zähldeckel bei 26.

## Nicht getan

Kein Circle angelegt, kein Marker umbenannt, keine Datei am Baum geändert. Der Orchestrator legt den Circle an und lässt ihn den Spec zitieren; die Umbenennung der überholten Festlegung auf `_s_` ist ein Planschritt und steht als C6 im Spec.

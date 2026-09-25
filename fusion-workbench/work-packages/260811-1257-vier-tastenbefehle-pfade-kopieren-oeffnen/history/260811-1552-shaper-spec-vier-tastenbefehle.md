# Shaper: der Spec der Runde 4, vier Tastenbefehle für Pfade, Öffnen und Cmd+W

**Datum:** 2026-08-11
**Agent:** shaper (in-Circle, aktiver Circle `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`)
**Status:** Complete

## Auftrag

Den Spec für den aktiven Circle schreiben. Die vier Fragen aus `decisions/` waren am 260811-1505 beantwortet und galten als Vorgabe. Eine Sache war ausdrücklich der Klärung überwiesen: was die Flüchtigkeit der Markierung für den Pfadkopierer bedeutet.

## Was geschrieben wurde

`planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md`, fünf Fähigkeiten mit zusammen 60 Abnahmekriterien, dazu zwei weitere im Abschnitt über die Zeitzusagen. C1 und C2 sind die beiden Kopierbefehle, C3 das Öffnen per Taste und per Doppelklick, C4 die Reichweite von Cmd+W, C5 die drei neuen Zeilen in den vier vollständigen Fallunterscheidungen. Ein Mermaid-Bild zeigt, welche vier Bauteile die Runde erbt und welche zwei Kanten vom Doppelklick ausgehen.

## Die Festlegung zur Markierung, und woran sie gemessen ist

**Der Kopierer nimmt die Markierung, wie er sie vorfindet.** Der Befund ist am Baum nachgemessen: `Ordnermodell::ersatz_einloesen` (`crates/krk-core/src/verzeichnis/modell.rs:174-183`) leert `markiert` und `auswahl` beim Einlösen des vorgemerkten Ersatzes, die Probe `die_markierung_faellt_mit_dem_ersatz_und_nicht_frueher` (Zeile 609) hält es fest, und `ordner_neu_lesen` (`crates/krk-ui/src/auffrischung.rs`) wird von einem FSEvents-Rückruf und vom Abschluss einer Dateioperation gerufen. Eine fremde Änderung im angezeigten Ordner lässt die Markierung also fallen, ohne Zutun des Nutzers.

Drei Gründe gegen eine Lösung in dieser Runde stehen im Spec: die Frage liegt bereits als offener Datensatz der Runde 1 (`260807-0020_*`), ihre Antwort kostet Zeit innerhalb der von L3 und L10 gemessenen Spanne, und der Schaden ist durch die Rückmeldung sichtbar. Der Spec hält daneben fest, was diese Runde am Gewicht jener Frage ändert: der Pfadkopierer ist der erste Abnehmer der Markierung ohne Rückfrage.

## Was am Bestand darüber hinaus geprüft wurde

- **Die Vorlage für das Öffnen mit dem Standardprogramm ist `im_browser_oeffnen`** (`crates/krk-ui/src/appkit/zwischenablage.rs:133`), die einzige der drei `NSWorkspace`-Stellen, die ohne benannte Anwendung übergibt. `appkit/terminal.rs` ist die Gegenvorlage.
- **Eine zweite schriftliche Zusicherung bricht,** die im Auftrag nicht genannt war: `crates/krk-ui/src/appkit/blaetter/mod.rs:222-227` sagt zu, dass die Auslieferungsbelegung die Eingabetaste nicht belegt. Nach C3 belegt sie sie. Das Verhalten bleibt richtig, die Begründung im Kommentar nicht.
- **Der Nachschlag in der Belegung ist eine lineare Suche** (`crates/krk-core/src/tasten/belegung.rs:866`) und läuft bei jedem Tastendruck. Er wächst von 71 auf 74 Einträge, und das ist der einzige Berührungspunkt mit L1 und L9.
- **`Ueberall` ist der einzige der sieben Wirkungsbereiche, der Cmd+W trägt,** ohne einen achten zu verlangen. Eine Verzweigung nach dem Fokus kommt trotzdem dazu, weil `bereichskommando` (`crates/krk-ui/src/appkit/anwendung.rs:2120`) den Befehl in der Leiste und im Editor heute ins Leere reicht.

## Angelegte Datensätze

- `decisions/260811-1552_o_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md`, offen. Die Wahl zwischen Text allein und Text mit Dateiverweis hat sichtbare Folgen im Finder und für KRKs eigenen Sprung aus der Zwischenablage. Der Spec trägt die empfohlene Möglichkeit als Vorbelegung, damit er nicht daran hängt.

Für die Flüchtigkeit der Markierung ist **kein** neuer Datensatz angelegt worden: die Frage liegt seit dem 260807 als `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md`, und ein zweiter wäre die Doppelablage derselben Frage.

## Offen geblieben

Das Kopffeld `**Active spec/plan:**` des Circle-Datensatzes steht weiter auf `(none yet)`. Es zu setzen gehört nicht in den Schreibbereich des Shapers in dieser Betriebsart.

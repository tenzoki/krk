# S19 — Vorschaufenster mit eigenen Tabs (C6), Vorschau der Zwischenablage (C10)

**Agent:** coder
**Datum:** 260805-2216
**Auftrag:** Planschritt 19 aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, dazu die Auflösung des Rechte-Defekts `issues/260803-2007_o_die-metadatenvorschau-aus-c6-verlangt-rechte-die-der-eintrag-nicht-traegt.md` über Weg 2.

## Was entstanden ist

- **`crates/krk-ui/src/vorschaumodell.rs` (neu):** Tabmodell des Vorschaufensters mit dem Halteverhalten aus dem Zustandsdiagramm des Specs (jede Quelle schreibt allein in den aktiven Tab), der Dreiteilung Text/Bild/Metadaten, dem Arbeitsfaden je Ladeanfrage (Kanal der Tiefe 1, Ladevorgang wohnt im Tab) und der Rechteformatierung `rechte_text`. Keine `objc2`-Kiste. 14 Prüfungen.
- **`crates/krk-ui/src/appkit/vorschau.rs` (neu):** die Ansicht — dieselbe `Tableiste` aus S12 ein zweites Mal, nicht auswählbare `NSTextView` für Text/Metadaten/Hinweise, `NSImageView` über `NSImage` für Bilder, Abholtakt für die Arbeitsfäden, fokussierbare `Inhaltsflaeche` (Klick macht sie zum Ersthelfer).
- **`crates/krk-core/src/tasten/belegung.rs`:** Kommando `ZwischenablageAnsehen` (Kennung `zwischenablage_ansehen`, 51 Einträge); beide C10-Befehle tragen `Wirkungsbereich::Dateifenster` (Nutzerentscheid 260805-0000). Neu der vierte Wert `Wirkungsbereich::Tabbereich` für die vier Tabbefehle: sie bedienen nach C6 und dem Kommentar der Auslieferungsbelegung den Bereich mit Tabs, der den Fokus hat.
- **`crates/krk-ui/src/kommandos/fokus.rs`:** `Fokus::Vorschau` als dritter Bereich; `wirkt(Tabbereich, ·)` für Dateifenster und Vorschau; Prüfungen für Tabbefehle und die beiden C10-Befehle (in Leiste und Vorschau stumm).
- **`crates/krk-ui/src/appkit/zwischenablage.rs`:** `inhalt_lesen()` in der **einen** `NSPasteboard`-Hülle aus S13 — Bild (PNG/TIFF) vor Text, sonst `Leer`. Keine zweite Hülle; KRK schreibt weiterhin nie.
- **`crates/krk-ui/src/appkit/tabelle.rs`:** `Auswahlmelder` an `auswahl_merken`, der einen Stelle, die eine Zeile in einen Eintrag übersetzt; `typ_beschriften` `pub(super)` für die Metadatenanzeige.
- **`crates/krk-ui/src/appkit/anwendung.rs`:** Zuleitung — Auswahl des aktiven Dateifensters füllt den aktiven Vorschau-Tab; `zwischenablage_ansehen` blendet die Vorschau ein und nie aus; `bereichskommando` reicht bei `Fokus::Vorschau` die Tabbefehle an die Vorschau; Fokusrückgabe beim Ausblenden von Leiste **und** Vorschau.
- **`crates/krk-ui/src/appkit/aufteilung.rs`:** der Platzhalter "Vorschau" ist durch das echte Vorschaufenster ersetzt.
- `appkit/mod.rs`, `main.rs`: Einbindung und Modulköpfe; `crates/krk-core/tests/belegung.rs`: Prüfung auf vier Wirkungsbereiche erweitert.

## Auflösung des Rechte-Defekts (Weg 2)

Die Rechte erhebt der Arbeitsfaden erst beim Anzeigen, mit einem `stat(2)` (`std::fs::symlink_metadata`) auf den einen angezeigten Pfad in `vorschaumodell::laden`. `Eintrag` aus S2 ist unberührt; kein Feld, kein zusätzlicher Systemaufruf je Ordnereintrag. Das Schließen des Issue-Files übernimmt der Orchestrator.

## Offene Frage (neu abgelegt)

`decisions/260805-2216_o_tastenweg-des-fokus-in-das-vorschaufenster.md` — es gibt keinen Tastenbefehl, der den Fokus in die Vorschau setzt (Belegungsdatei ist ontocoder-Gebiet), und der C2-Satz "Dateifenster oder Lesezeichenleiste" kennt den dritten Bereich noch nicht.

## Prüfstatus

- `make check`: alle vier grün (Build, 35+ Kern- und 14 neue Modellprüfungen, `clippy -D warnings`, `fmt --check`).
- `make bundle`: gebaut und signiert.
- `make durchstich RUNDEN=1`: Gate bestanden mit angeschlossener Vorschau — die synthetischen Pfeiltasten (postEvent:atStart:) laufen jetzt durch den Vorschau-Anstoß je Auswahländerung; L1 95 % im Bild, L2/L3/L4/L10 gehalten. Bericht `messungen/260805-2018-durchstich.txt`.
- Bündelstart über `open target/KRK.app`: Prozess läuft stabil, sauberes Beenden per Quit-AppleEvent. Eine Bildschirmaufnahme zur Sichtprüfung war mangels Freigabe nicht möglich; die interaktiven Abnahmekriterien (Halteverhalten am Fenster, shift+f3 mit Text/Bild/leer, Fokusproben) stehen zur Sichtprüfung durch den Nutzer aus.

**Status: Complete**

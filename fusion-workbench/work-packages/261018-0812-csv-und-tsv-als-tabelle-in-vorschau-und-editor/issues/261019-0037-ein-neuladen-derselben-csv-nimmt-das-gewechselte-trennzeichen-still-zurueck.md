Ein Neuladen derselben CSV nimmt das gewechselte Trennzeichen still zurück
---
Hat der Nutzer mit „Trennzeichen wechseln“ eine falsch erkannte `.csv` richtig gestellt und wird die Datei danach von außen geändert und neu geladen, wählt KRK das Trennzeichen wieder nach der Regel. Spaltenbreiten und Auswahl bleiben dabei stehen, weil der Pfad derselbe ist. Die Tabelle zeigt danach wieder die falsche Zerlegung; die Statuszeile meldet das Trennzeichen, sagt aber nicht, dass die Wahl des Nutzers gefallen ist.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

**Schwere:** Low.

**Beleg (Stand `c97e264e`):**

- `crates/krk-ui/src/editormodell.rs`, `Editormodell::uebernehmen`: jeder gelungene Ladeausgang, auch das Neuladen, ist `Ladeausgang::Geoeffnet`.
- `crates/krk-ui/src/appkit/editor.rs`, `ladeausgang_einziehen` und `zurueckgehaltenes_uebernehmen`: bei `Geoeffnet` läuft `trenntext_vorbereiten`, das `trenntext::dialekt_erkennen` neu fragt und `Trenntextlage` mit dem erkannten Dialekt ersetzt. Nur `trenntextpfad` unterscheidet „andere Datei“ von „dieselbe“, und der Dialekt hängt nicht daran.
- E2 des Plans: der Wechsel „gilt für die geöffnete Datei und wird nicht gemerkt“. Ein Neuladen ist dieselbe geöffnete Datei.

**Richtung der Behebung:** In `trenntext_vorbereiten` bei gleichem `trenntextpfad` und `Herkunft::Gewechselt` den bisherigen Dialekt behalten.

**Abnahme:** Eine Probe wechselt das Trennzeichen einer `.csv`, lädt dieselbe Datei neu und findet danach denselben Dialekt mit `Herkunft::Gewechselt`; eine andere Datei beginnt wieder mit der Regel.

---
Resolved: `crates/krk-ui/src/appkit/editor.rs`: neue reine Funktion `dialekt_beim_eintreffen` (gleiche Datei und `Herkunft::Gewechselt` → bisheriger Dialekt, sonst Regel), gerufen aus `trenntext_vorbereiten`. Probe `ein_neuladen_derselben_csv_behaelt_das_gewechselte_trennzeichen`. `HowTo.md`, Absatz „Hat die Regel falsch gewählt“.

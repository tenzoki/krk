Eine CSV, die keine Tabelle ergibt, schaltet die Ansicht des Editors für jede folgende Datei auf Roh
---
Der Rückfall in die Rohansicht (offenes Anführungszeichen, mehr Spalten als `trenntext::HOECHSTENS_SPALTEN`) geht über `Editormodell::ansicht_umschalten`. Die Ansicht ist nach C3 eine Wahl, die **nicht der Datei gehört** und über jeden Dateiwechsel stehen bleibt. Öffnet der Nutzer danach eine Markdown-Datei oder eine Eintragsdatei des Notizordners, sieht er Rohtext statt gerendertem Markdown oder Tabelle, ohne den Wechsel je selbst ausgelöst zu haben. Nichts stellt die Ansicht zurück.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

**Schwere:** Medium. Kein Datenverlust, aber eine stille, dauerhafte Änderung einer Nutzerwahl durch das bloße Öffnen einer Datei.

**Beleg (Stand `c97e264e`):**

- `crates/krk-ui/src/appkit/editor.rs`, `Editorbereich::tabellenform_pruefen`: `let _ = self.ivars().modell.borrow_mut().ansicht_umschalten();`.
- Gerufen aus `trenntext_vorbereiten` bei **jedem** eintreffenden Ladeausgang `Geoeffnet`, also auch beim Neuladen nach einer Änderung von außen (`Editormodell::uebernehmen` antwortet immer `Geoeffnet`), dazu aus `trennzeichen_wechseln` und nach dem Ansichtswechsel.
- `crates/krk-ui/src/editormodell.rs`, Probe `die_ansichtswahl_bleibt_ueber_einen_dateiwechsel_stehen`: „C3: die Wahl gehoert nicht der Datei“.
- `trenntext_vorbereiten` antwortet für eine Nicht-CSV mit `None` und fasst die Ansicht nicht an; einen Rückweg gibt es nicht.
- `HowTo.md`, Absatz „Die Tabelle bleibt in zwei Fällen in der Rohansicht“, sagt nichts über die folgenden Dateien. Plan Schritt 3 sagt „stellt der Editorbereich die Ansicht auf Roh“ und hat die Folge für andere Dateien nicht entschieden.

**Richtung der Behebung:** Die Verweigerung an die geöffnete Datei hängen und nicht an die Ansicht. Ein Feld der `Trenntextlage` hält die `Verweigerung`, und `editorform_der_datei` bildet Formatansicht + verweigerte Trenntextdatei auf `Editorform::Text` ab, ohne `modell.ansicht` zu ändern. `ctrl+cmd+e` und `trennzeichen_wechseln` setzen das Feld neu.

**Abnahme:** Eine Probe öffnet nacheinander eine CSV mit offenem Anführungszeichen und eine `.md`. Die zweite steht in der Formatansicht, und `modell.ansicht()` ist danach dieselbe wie vor der CSV. Die vorhandenen Proben zur Verweigerung (`eine_unterminierte_und_eine_zu_breite_csv_bleiben_in_der_rohansicht_mit_meldung`) bleiben grün.

---
Resolved: Die Verweigerung hängt an der Datei. `crates/krk-ui/src/appkit/editor.rs`: Feld `verweigert` an `Trenntextlage`, gesetzt in `tabellenform_pruefen` (ruft `ansicht_umschalten` nicht mehr); `editorform_der_datei` bekommt `verweigert` und bildet Formatansicht + verweigerte CSV auf die Textform ab; `dargestellte_ansicht` für `darstellung_nachziehen` und `formatierung_anwenden`; `ansicht_umschalten` versucht bei stehender Verweigerung die Tabelle erneut, statt die Ansicht umzustellen; `trennzeichen_weiter` und jede neue Lage löschen das Feld. Proben `eine_verweigerte_csv_laesst_die_ansicht_fuer_die_naechste_datei_stehen` (neu) und `eine_unterminierte_und_eine_zu_breite_csv_bleiben_in_der_rohansicht_mit_meldung` (Quelltextnadel angepasst). `HowTo.md`, Absatz „Die Datei steht in zwei Fällen als Text“.

Ein Umbruch in einer Zelle einer CRLF-CSV wird bei jedem Sichern zu `\r\n`, auch in Zeilen, die niemand angefasst hat
---
Excel trennt die Sätze mit `\r\n` und schreibt einen Umbruch in einer Zelle (Alt+Enter) als `\n` in Anführungszeichen. KRK hält den Stand mit `\n`, merkt sich allein „erstes Zeilenende ist `\r\n`“ und macht beim Sichern aus **jedem** `\n` ein `\r\n`. Jede mehrzeilige Zelle der Datei ändert damit beim ersten Sichern ihre Bytes und ihren Wert, gleich welche Zelle der Nutzer bearbeitet hat. Steht der erste Umbruch der Datei in einer gequoteten Kopfzelle, fällt die Erhebung umgekehrt auf `\n`, und jede Satzgrenze der Datei verliert ihr `\r`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md, 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

**Schwere:** High. Die Directive verlangt „ohne dass unveränderte Teile der Datei ihre Form verlieren“; hier ändert sich der **Wert** unberührter Zellen (`a\nb` wird `a\r\nb`), und Excel-Dateien mit mehrzeiligen Zellen sind der Regelfall des Formats.

**Beleg (Stand `c97e264e`):**

- `crates/krk-core/src/text/datei.rs`, `Dateiform::der_bytes`: `crlf` ist allein „das Byte vor dem **ersten** `\n` ist `\r`“, ohne Rücksicht auf Anführungszeichen.
- `crates/krk-core/src/text/datei.rs`, `in_dateiform`: bei `form.crlf` wird jedes `\n` des Standes zu `\r\n`, ohne Unterschied zwischen Satzgrenze und Umbruch im Feld.
- `crates/krk-core/src/text/datei.rs`, `gehaltene_form`: beim Einlesen werden `\r\n` und ein einzelnes `\r` zu `\n`; die Unterscheidung „Satzgrenze `\r\n`, Umbruch im Feld `\n`“ ist danach nicht mehr im Stand.
- Der Doc-Kommentar von `Dateiform` nennt den Fall selbst als Preis. Der Entscheid zur Sicherungsform nennt unter Möglichkeit 2 nur „gemischte Zeilenenden“ und „ein einzelnes `\r` in einem Feld“; den Umbruch in einer Excel-Zelle, also eine Wertänderung unberührter Zellen, hat der Nutzer nicht vorgelegt bekommen.
- Die Doku widerspricht sich: `HowTo.md`, Abschnitt CSV, Absatz „Jede Handlung ändert zuerst nur den Stand im Editor“: „jede andere Zeile kommt Zeichen für Zeichen zurück“; der Absatz direkt danach nennt die Ausnahme. `CLAUDE.md`, Absatz zu `krk_core::trenntext`: „der Rest kommt Byte für Byte zurück“. Byte für Byte gilt im Speicher (`jede_handlung_aendert_nur_die_bytes_der_beruehrten_saetze`), nicht auf der Platte.
- Die Probe `eine_excel_csv_behaelt_marke_und_zeilenenden_und_jede_andere_datei_nicht` (`crates/krk-ui/src/editormodell.rs`) prüft eine Datei ohne mehrzeilige Zelle und sieht den Fall nicht.

**Richtung der Behebung:** Für eine Trenntextdatei mit `crlf` nur die Satzgrenzen in `\r\n` wandeln und die `\n` innerhalb von Anführungszeichen stehen lassen. Die Grenzen kennt der eine Leser (`trenntext::indexieren`, `Satzindex::zeilenende`). Die Erhebung von `crlf` am ersten Zeilenende **außerhalb** von Anführungszeichen messen. Ein `\r\n` innerhalb einer Zelle, selten und nicht von Excel, kommt dann als `\n` zurück. Das ist eine Änderung der Antwort auf den Entscheid zur Sicherungsform und braucht eine Zeile vom Nutzer, bevor sie gebaut wird.

**Abnahme:**

- Eine Probe öffnet `\u{feff}Name;Notiz\r\nA;"x\ny"\r\nB;z\r\n`, ändert `B;z` zu `B;w`, sichert und liest `\u{feff}Name;Notiz\r\nA;"x\ny"\r\nB;w\r\n` Byte für Byte zurück.
- Eine Probe mit einer gequoteten Kopfzelle mit Umbruch (`"a\nb";c\r\n1;2\r\n`) behält nach einer Zelländerung jedes Satz-`\r\n`.
- `HowTo.md` und `CLAUDE.md` beschreiben, was auf der Platte gilt.

---
Resolved: Der Stand einer CSV/TSV trägt seit dem 261019 die Zeilenenden der Datei; nur die Marke wird abgetrennt. `crates/krk-core/src/text/datei.rs`: neu `trenntext_oeffnen`, `Dateiform { marke, zeilenenden: Zeilenenden }` mit `Gewandelt | WieGelesen | NurCr` statt `crlf`, `in_dateiform` schreibt bei `WieGelesen` Marke plus Stand Byte für Byte (ohne angehängtes Zeilenende), `ist_in_stand_form` und `in_stand_form`; Modulkopf, Absatz „Eine solche Frage ist einmal gestellt“. `crates/krk-ui/src/editormodell.rs`: `text_lesen` liest Trenntext über `trenntext_oeffnen`, `bearbeiten` über `in_stand_form`, neu `Editormodell::wandelt`, Schutz in `uebernehmen`. `crates/krk-ui/src/appkit/editor.rs`, `text_zurueckschreiben`: fragt `modell.wandelt`. Proben `ein_umbruch_in_einer_excel_zelle_bleibt_beim_sichern_einer_anderen_zelle_stehen` (beide Abnahmefälle, dazu `opt+return` als `\n` und Datei ohne Schluss-Zeilenende) und `eine_excel_csv_behaelt_marke_und_zeilenenden_und_jede_andere_datei_nicht` (`editormodell.rs`), `crates/krk-core/tests/text.rs` (`eine_csv_mit_marke_und_crlf_kommt_unveraendert_byte_fuer_byte_zurueck`, `der_gehaltene_stand_traegt_weiter_keine_marke_und_kein_cr`). `HowTo.md`, Abschnitt „CSV- und TSV-Dateien › Im Editor“, und `CLAUDE.md`, Absatz zu `krk_core::trenntext`, angeglichen. Abweichung vom Entscheid `261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md` (Möglichkeit 2: Form erheben, Stand mit `\n`): auf Anweisung des Dispatch selbständig ausgeführt; der Entscheid ist nicht ergänzt.

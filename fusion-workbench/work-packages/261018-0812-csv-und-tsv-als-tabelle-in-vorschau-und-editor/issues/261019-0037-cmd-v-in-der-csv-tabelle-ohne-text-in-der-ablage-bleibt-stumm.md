`cmd+v` in der CSV-Tabelle ohne Text in der Ablage bleibt stumm
---
Liegt in der Zwischenablage kein Text, etwa ein Bild oder ein im Finder kopierter Eintrag ohne Textform, tut `cmd+v` in der Tabelle einer CSV nichts und meldet nichts. Für genau diesen Fall gibt es die Antwort `Trenntextantwort::AblageOhneText` („Die Zwischenablage trägt keinen Text, den die Tabelle einsetzen könnte“), aber dieser Weg erreicht sie nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

**Schwere:** Low.

**Beleg (Stand `c97e264e`):**

- `crates/krk-ui/src/appkit/eintragsansicht.rs`, `Eintragstabelle`, Methode `paste:` (`fn einfuegen`): `let Some(text) = zwischenablage::nur_text_lesen() else { return; };`. Die Methode kehrt zurück, bevor `Zellenwege::einfuegen` gerufen ist.
- `crates/krk-ui/src/appkit/editor.rs`, `tabellenhandlung_rechnen`: `AblageOhneText` entsteht allein, wenn ein vorhandener Text keinen Block ergibt.

**Richtung der Behebung:** Ohne Text `Zellenwege::einfuegen` mit leerem Text rufen, damit der Editor `AblageOhneText` meldet, oder einen eigenen Weg zur Meldung geben.

**Abnahme:** Eine Probe am reinen Rechenweg und eine Quelltextprobe am Rumpf von `paste:` zeigen, dass eine Ablage ohne Text zu `Trenntextantwort::AblageOhneText` in der Statuszeile führt.

---
Resolved: `crates/krk-ui/src/appkit/eintragsansicht.rs`, `Eintragstabelle::einfuegen` (`paste:`): ohne Text geht ein leerer Text an `Zellenwege::einfuegen`, und `tabellenhandlung_rechnen` antwortet `Trenntextantwort::AblageOhneText` (Text in allen drei Sprachtabellen schon vorhanden). Quelltextprobe `das_einfuegen_ohne_text_in_der_ablage_erreicht_den_editor`; der Rechenweg steht in `cmd_v_setzt_einen_tsv_block_ab_der_zelle_und_ein_cmd_z_nimmt_ihn_ganz_zurueck` (`editor.rs`). `HowTo.md`, Absatz „Kopieren und Einfügen“.

Vier Fremdprogramm-Hüllen geben `bool` oder `Option` ohne `must_use` zurück, und die 27 appkit-Dateien tragen vierzehn

---

Die Regel aus `CLAUDE.md` („ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt
`#[must_use]`") ist an genau den Antworten nicht umgesetzt, bei denen das Fallenlassen den Nutzer
ohne Meldung ließe: `standardprogramm::oeffnen` (`crates/krk-ui/src/appkit/standardprogramm.rs:90`, `bool`
= „das System hat angenommen"), `terminal::ordner_oeffnen` (`terminal.rs:90`, `bool` = „eine
Anwendung dieser Kennung ist installiert"), `zwischenablage::im_browser_oeffnen`
(`zwischenablage.rs:285`, `bool`) und `weitereinstanz::starten` (`weitereinstanz.rs:105`,
`Option<&str>` = der Satz für die Statuszeile). Alle vier heutigen Rufer (`tabelle.rs:1943`,
`:2408`, `anwendung.rs:2106`, `:2123`, `:6348`) verbrauchen die Antwort; der nächste Rufer muss es
nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** die 27 Dateien direkt unter `crates/krk-ui/src/appkit/` ohne `anwendung.rs`, `tabelle.rs`, `editor.rs`

Nachgezählt am 260826 mit `grep -c '#\[must_use'`: 14 Stellen in 9 der 27 Dateien (`abwurf.rs` 4,
`teilen.rs` 2, `textmerkmale.rs` 2, je 1 in `belegungsansicht.rs`, `ereignisse.rs`, `papierkorb.rs`,
`volumes.rs`, `vorschau.rs`, `zwischenablage.rs`). Weitere reine Antworten ohne Attribut, bei
denen ein nackter Aufruf ebenfalls nichts bewirkte: `zwischenablage::lesen`, `::inhalt_lesen`,
`::dateiverweise`; `volumes::eingehaengte`; `koordinaten::in_utf16`, `::in_bytes`;
`aufteilung::zeilenmass`, `::gemessene_breiten`, `::bereichssicht`; `statuszeile::filterstand_text`,
`::zeile`, `::zeilentext`, `Rang::art`; `menue::tag_des_kommandos`, `::kommando_zum_tag`,
`::hauptmenue`; `bildtakt::bildwiederholrate`; `textmerkmale::grundschrift`,
`::tafel_der_erscheinung`; `titelzusatz::beschriftung`; `leiste::lesezeichenliste`,
`::gewaehltes_lesezeichen`, `Leistenquelle::kommando_ausfuehren`;
`Vorschaufenster::kommando_ausfuehren`, `::angezeigter_pfad`, `::laedt_noch`, `::fokusansicht`.
`text_schreiben` steht schon in `260820-0739`. Die vier Fremdprogramm-Antworten sind der Teil
mit Nutzerfolge; die übrigen sind derselbe Befund, den sechs Prüfer dieser Sitzung je Kiste
gemeldet haben (`260826-1221`, `-1223`, `-1305`, `-1325`, `-1327`, `-1335`).

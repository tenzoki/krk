# Shaper: der Inhaltsfilter der Dateiliste

**Datum:** 2026-08-16
**Status:** Complete
**Modus:** user-direct, kein Circle aktiv
**Baumstand:** `9236dd4`

## Auftrag

Der Nutzer will den Filter der Dateiliste so erweitern, dass Treffer im Dateiinhalt zählen, geschaltet über ein weiteres Ankreuzfeld „Content" in der Bereichsleiste.

## Was am Baum geprüft wurde

Gelesen und belegt: der Circle-Datensatz der Runde 10 samt Abschlussnotiz, `crates/krk-core/src/verzeichnis/filter.rs` ganz, der Prüfschritt `Ordnermodell::sichtbar` (`modell.rs:542-587`), die Schnittstelle des Unterbaumdurchlaufs (`durchlauf.rs:105-200` und `:280-325`), `text/datei.rs` (`lesen`, `oeffnen`, `EDITORGRENZE`), `vorschaumodell.rs` (`TEXTGRENZE`, `bis_zur_grenze_lesen`), die sechs Ränge der Statuszeile, die Zählprobe in `tests/verzeichnis.rs:1786-1836`, der Keymap-Eintrag für „Deep", und die Messstrecke in `krk-bench`.

Vier Befunde tragen den Spec und standen so in keinem Dokument:

1. **Keine der zehn Zeitzusagen deckt das Tippen.** L1 misst zwanzig Pfeil-ab-Ereignisse (`messmodus.rs:820`), nicht ein getipptes Zeichen. Der Namensfilter der Runde 10 ist damit schon ungemessen.
2. **Die Prüfordner der Messstrecke sind dünnbesetzt.** Je Datei 512 echte Bytes, der Rest ein Loch (`fixture.rs:42`); der Modulkopf warnt selbst davor, sie für eine Messung mit echten Bytes zu benutzen. Eine elfte Zusage verlangt deshalb zuerst einen vierten Prüfordner.
3. **Die Größengrenze ist zugleich die Schranke der Abbruchspanne.** Eine gelesene Datei ist die kleinste nicht unterbrochene Einheit; der Abbruch wird heute nur an der Stapelgrenze geprüft (`durchlauf.rs:287-291`).
4. **Der Stand von „Deep" liegt am `Ordnermodell` des Tabs** (`tabs.rs:596`), obwohl der zugehörige Datensatz noch offen steht. „Content" folgt derselben Ablage.

## Was der Nutzer entschieden hat

Sechs Festlegungen, alle am 260816: die gestaffelte Mindestlänge (fünf Zeichen bei tiefer Suche, drei ohne), nur Text, die 1 MB der Vorschau statt der 16 MB des Editors, der Kurzschluss über den Namen, das ODER zwischen Name und Inhalt, und keine elfte Zeitzusage.

## Ergebnis

- Spec: `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md`, sechs Fähigkeiten, 57 Abnahmekriterien, ein Mermaid-Bild des erweiterten Prüfschritts.
- Vier Entscheidungsdatensätze in `shared/decisions/`, alle vom 260816-1310. Zwei sind mit der Antwort des Nutzers beantwortet und tragen `_a_` (Größengrenze, Messgröße), zwei stehen offen und tragen `_o_` (Rückmeldung in der Statuszeile, Kennzeichnung des Treffergrunds). Für beide offenen trägt der Spec eine benannte Vorbelegung.
- Der Rest der Größenfrage, ob die Statuszeile ungelesene Dateien ausweist, ist in den Datensatz zur Statuszeile gewandert statt als stillschweigendes Nein zu gelten.

## Ablage

Der Spec liegt im gemeinsamen Speicher, weil kein Circle aktiv war. Der Circle der elften Runde wird nach diesem Lauf angelegt und nimmt Spec und die vier Datensätze auf.

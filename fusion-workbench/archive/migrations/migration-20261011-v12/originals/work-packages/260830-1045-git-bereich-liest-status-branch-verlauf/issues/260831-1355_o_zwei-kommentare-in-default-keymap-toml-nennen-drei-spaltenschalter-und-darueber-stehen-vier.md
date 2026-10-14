Zwei Kommentare in `default-keymap.toml` nennen drei Spaltenschalter, und darüber stehen vier

---
`resources/default-keymap.toml` trägt seit Schritt 9 der Runde 23 den Eintrag `spalte_marke_umschalten` als vierten Spaltenschalter. Zwei Kommentare am Eintrag `tiefe_suche_umschalten` darunter sind damit unrichtig geworden:

- `:464` — „er bestimmt, was die Dateiliste zeigt, wie die drei Spaltenschalter darueber und das Ein- und Ausblenden der versteckten Eintraege"
- `:478` — „Wie die drei Spaltenschalter darueber faellt der Eintrag damit aus der Markdown-Ausgabe der Runde 3"

Über dem Eintrag stehen `spalte_groesse_umschalten`, `spalte_datum_umschalten`, `spalte_typ_umschalten` und `spalte_marke_umschalten`, also vier. Der Kommentar am Markeneintrag selbst (`:458`, „ohne Kombination wie die drei darueber") ist dagegen **richtig**: über ihm stehen genau drei.

Die Schwesterstellen im Code sind mit Schritt 12 gefallen und tragen jetzt „die Spaltenschalter darueber" ohne Zahl: `crates/krk-ui/src/belegungsmodell.rs:297` und `:299`, `crates/krk-core/src/tasten/belegung.rs:446`. Dieselbe Form passt hier.

**Abnahme:** die Erhebung aus Schritt 12 der Runde 23 liefert für `resources/default-keymap.toml` keine unrichtige Zählaussage mehr, und `make check` bleibt grün (die Datei geht durch `Belegung::auslieferung`, ein Kommentar ändert daran nichts).

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** data — `resources/default-keymap.toml` ist die eine Quelle jeder Tastenbelegung und gehört dem `ontocoder`; Schritt 9 der Runde 23 hat ihre drei neuen Einträge dort eingetragen.
Gefunden beim Nachzug von Schritt 12, mit der Erhebung über Spalten und Schalter. Der Schritt hat die Datei deshalb nicht angefasst.
Verwandt: `260831-1212_c_die-zaehlaussagen-ueber-spalten-und-schalter-stehen-in-sieben-dateien-die-schritt-12-nicht-fuehrt.md` (derselbe Nachzug, Code-Hälfte).

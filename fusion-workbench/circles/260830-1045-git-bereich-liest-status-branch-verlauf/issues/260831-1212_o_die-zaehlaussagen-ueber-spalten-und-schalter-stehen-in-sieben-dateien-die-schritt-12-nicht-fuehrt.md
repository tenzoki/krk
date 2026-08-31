Die Zählaussagen über Spalten und Schalter stehen in sieben Dateien, die Schritt 12 nicht führt

---
Schritt 12 des Plans der Runde 23 zieht die zweite Hälfte der Erhebung aus C9.4 nach und nennt dafür fünf Dateien: `spalten.rs`, `appkit/tabelle.rs`, `appkit/bereichsleiste.rs`, `ablage/sitzung.rs`, `kommandos/loeschwarnung.rs`. Diese Liste stammt aus der Erhebung mit dem **alten** Muster. Die in Schritt 11 erweiterte Erhebung findet dieselbe Sorte Aussage in weiteren Dateien, und die folgenden Stellen sind am Stand nach Schritt 11 unrichtig:

- `crates/krk-ui/src/fenstermodell.rs:75` — „[`Spaltensichtbarkeit`] mit vier Feldern"; sie trägt seit Schritt 2 fünf.
- `crates/krk-ui/src/fenstermodell.rs:302` — „die Leiste 18 Punkte hoch ist und neun Schalter"; die Leiste schickt heute zwölf Umschalter.
- `crates/krk-ui/src/appkit/anwendung.rs:1595` — „Die Tabelle baut ihre vier Spalten immer".
- `crates/krk-ui/src/appkit/anwendung.rs:4580` — „Sie schreibt alle vier Spalten und nicht nur die geaenderte".
- `crates/krk-ui/src/appkit/anwendung.rs:5238` — „Schreibt die zehn Schalterzustaende der Bereichsleiste".
- `crates/krk-ui/src/appkit/anwendung.rs:5256`, `:5267` — „dem neunten Schalter dazu. Die acht ersten" und „Der zehnte Schalter hat deshalb keinen vierten Anlass gebracht".
- `crates/krk-ui/src/appkit/leiste.rs:168` — „vier Spalten, hier gibt es eine Spalte mit einer Beschriftung".
- `crates/krk-ui/src/belegungsausgabe.rs:532`, `:667` — „die drei Spaltenschalter"; es sind vier seit Schritt 8.
- `crates/krk-core/tests/belegung.rs:90` — „Die drei Spaltenschalter der Bereichsleisten-Runde".
- `crates/krk-ui/src/appkit/bereichsleiste.rs:1` — „zehn Ankreuzfelder, sonst nichts" (Datei steht in der Liste von Schritt 12, die Zeile ist hier der Vollständigkeit halber genannt).

Zwei Stellen sind mit Schritt 11 schon gefallen, weil sie „fünf Bereiche" und die Schalterzahl in **einem** Satz führten und ein halber Nachzug den Satz in sich widersprüchlich gelassen hätte: `crates/krk-ui/src/kommandos/loeschwarnung.rs:249-255` und `crates/krk-ui/src/appkit/mod.rs:82-86`. Beide tragen jetzt die Erhebungsvorschrift statt einer Zahl, wie Entscheidung 9 des Plans es für die Ankreuzfelder der Bereichsleiste vorsieht.

**Abnahmetest:** die erweiterte Erhebung aus Schritt 11 liefert für Spalten und Schalter keine unrichtige Aussage mehr. Das Erhebungsprogramm liegt bei Schritt 11 beschrieben (History-Eintrag `260831-1212-coder-schritt-11-zaehlaussagen-bereiche-und-fokuswerte.md`).

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden beim Nachzug von Schritt 11, weil das dort erweiterte Muster mehr Dateien erreicht als die Dateiliste von Schritt 12 führt. Schritt 12 kann den Defekt mit erledigen; er ist gefilt, damit die Stellen nicht zwischen den beiden Dateilisten hindurchfallen.
Verwandt: `260830-1317_*_das-erhebungsmuster-aus-c9-4-ist-zu-eng-und-das-gegenbeispiel-steht-in-belegungsausgabe-rs.md` (derselbe Befundtyp, andere Hälfte der Erhebung).

# Dem Editor fehlt die Auskunft über die Zeile der Schreibmarke

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coder, bei der Umsetzung von S38 und S40
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (die fehlende Funktion), `crates/krk-ui/src/appkit/anwendung.rs:1044-1095` (`lesezeichen_anlegen`, der wartende Aufrufer)
**Cross-references:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` Abschnitt `#### 38.`, `crates/krk-ui/src/appkit/nummernspalte.rs:455-490` (`anfaenge_in_utf16`, die Umrechnung, die es schon gibt), `crates/krk-core/src/text/zeilen.rs` (`Zeilenindex`)

---

## Der Befund

S38 legt das Anlegen einer Textmarke auf den bestehenden Befehl
`lesezeichen_anlegen` (`cmd+d`): mit dem Fokus im Dateifenster merkt er den
Ordner, mit dem Fokus im Editor die Zeile der Schreibmarke, ihre Nummer und
ihren Inhalt.

Die Hälfte davon ist gebaut. Von `anwendung.rs::lesezeichen_anlegen` bis in
`bookmarks.toml` nimmt die Kette seit diesem Schritt das fertige
`krk_core::ablage::Ziel` entgegen und fragt an keiner Stelle nach der Sorte:
`Leistenquelle::lesezeichen_anlegen`, `Leistenmodell::anlegen`,
`Lesezeichenliste::anlegen`. Die Leiste zeigt beide Sorten und unterscheidet sie
am Sinnbild (S40).

Was fehlt, ist der Wert, aus dem der Editor-Zweig sein `Ziel::Textstelle` baut:
**Nummer und Inhalt der Zeile, in der die Schreibmarke steht.** `Editorbereich`
gibt heute `pfad()`, `haelt_datei()`, `hat_ungesicherten_stand()` und
`textflaeche()` heraus, aber nichts über die Stelle der Schreibmarke.

## Warum das nicht am Aufrufer zu lösen ist

`textflaeche()` liefert die `NSTextView`, und aus ihr wären `selectedRange()` und
`string()` zu holen. Wer das im Aufrufer täte, schriebe die Umrechnung von
AppKits UTF-16-Einheiten in Byteversätze ein zweites Mal auf: sie steht schon in
`appkit/nummernspalte.rs` als `anfaenge_in_utf16` und ist dort privat. Das wäre
der zweite Rechenweg für dieselbe Umrechnung, und der Modulkopf von
`krk_core::text` hält ausdrücklich fest, dass jeder Versatz auf einer
Zeichengrenze liegen muss — eine Zusage, die zwei Rechenwege doppelt tragen
müssten.

Die Auskunft gehört deshalb in `appkit/editor.rs`: dort liegen der gehaltene
Stand (`Editormodell`) und die `NSTextView` beieinander, und `Zeilenindex` aus
`krk_core::text::zeilen` rechnet aus beidem, was gebraucht wird.

## Was die Funktion leisten muss

Vorschlag für den Zuschnitt, nicht die Festlegung — wer `editor.rs` baut,
entscheidet die Form:

```rust
/// Die Zeile, in der die Schreibmarke steht: Nummer ab 1 und ihr Inhalt (C6).
pub fn schreibmarkenzeile(&self) -> Option<(u32, String)>
```

Vier Eigenschaften trägt sie, gleich in welcher Form:

1. **`None`, wenn der Editor keine Datei hält.** Ohne Datei gibt es keine Stelle,
   die eine Marke bezeichnen könnte.
2. **Eine Zeile und kein Bereich.** Ist mehrzeilig ausgewählt, gilt die Zeile der
   Schreibmarke; ein Textbereich entsteht nicht
   (`decisions/260807-2147_*_traegt-eine-textmarke-auch-einen-bereich-oder-nur-eine-stelle.md`).
   **Offen und mit zu entscheiden:** welches Ende der Auswahl die Schreibmarke
   ist. AppKit meldet über `selectedRange()` allein den kleineren Versatz, und
   die Richtung, in die der Nutzer gezogen hat, geht daraus nicht hervor. Der
   naheliegende Zuschnitt ist der Anfang der Auswahl, weil er der einzige
   Versatz ist, den AppKit verlässlich nennt; das Abnahmekriterium von S38
   verlangt eine Probe darauf.
3. **Der Inhalt kommt aus dem gehaltenen Stand und nicht von der Platte.**
   Dieselbe Regel, die das neunte Abnahmekriterium von C5 der Suche gibt: der
   Editor merkt, was er zeigt.
4. **Der Inhalt ist die Zeile ohne ihren Zeilenumbruch**, so wie
   `Zeilenindex::inhalt_der_zeile` sie liefert; `krk_core::text::marke::wiederfinden`
   vergleicht später gegen genau diese Form.

Die Umrechnung ist mit dem vorhandenen `anfaenge_in_utf16` in einem Zug zu haben:
die Zeilennummer ist die Zahl der Zeilenanfänge, die nicht hinter der
Schreibmarke liegen. Ob die Funktion dafür aus `nummernspalte.rs` heraus sichtbar
wird oder an eine gemeinsame Stelle wandert, entscheidet, wer beide Dateien hält.

## Was zu tun ist

Zwei Schritte, in dieser Reihenfolge:

1. Die Auskunft in `appkit/editor.rs` bauen. Sie gehört dem Bündel, das
   `editor.rs` hält; die Leiste ist fertig und wartet.
2. In `anwendung.rs::lesezeichen_anlegen` den zweiten Zweig anhängen: mit dem
   Fokus im Editor liefert er ein `Ziel::Textstelle` aus `Editorbereich::pfad()`
   und der neuen Auskunft, sonst wie bisher ein `Ziel::Ordner`. Ein zweiter
   Anlegebefehl daneben entsteht nicht — S38 verlangt eine Funktion, eine
   Kombination, ein Kommando, einen Eintrag in der Belegungsansicht. Der
   Vorschlag für das Eingabeblatt bleibt offen: für einen Ordner ist es sein
   Name, für eine Textstelle liegt der Dateiname mit der Zeilennummer nahe.

Bis dahin bleibt `cmd+d` mit dem Fokus im Editor bei seinem bisherigen Verhalten
und legt eine Ordnermarke auf den Ordner des aktiven Dateifensters an. Das
Kommando trägt `Wirkungsbereich::Ueberall`, der Rumpf ist also erreicht; er kennt
den zweiten Zweig nur noch nicht.

**S38 ist damit nicht erledigt.** Erledigt sind seine Anteile an der Leiste und
an der Kette bis `bookmarks.toml`, und S40 ganz.

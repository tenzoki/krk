# S8: `krk-core::text` — Zeilenindex, Suche und Ersetzen

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 8 (Phase B)

## Was entstanden ist

Ein neues Kernmodul `crates/krk-core/src/text/` mit der Textrechnung des
Editors. Ohne AppKit, ohne Zustand, ohne Dateisystem: jede Funktion bekommt
eine Zeichenkette und liefert Versätze oder eine neue Zeichenkette.

| Datei | Inhalt |
|---|---|
| `crates/krk-core/src/text/mod.rs` | Modulkopf: warum die Rechnung im Kern liegt, und die Zusage über die Byteversätze |
| `crates/krk-core/src/text/zeilen.rs` | `Zeilenindex`, `Zeilensprung`, `Zeilenlage` |
| `crates/krk-core/src/text/suche.rs` | `Treffer`, `Ersetzung`, `Sammelersetzung`, `alle`, `erster_ab`, `naechster`, `voriger`, `einen_ersetzen`, `alle_ersetzen` |
| `crates/krk-core/src/lib.rs` | `pub mod text;` und eine Zeile im Modulkopf |
| `crates/krk-core/tests/text.rs` | die fünf Proben des Abnahmekriteriums |

## Die Festlegungen, die der Plan offen ließ

Vier Fragen fielen bei der Umsetzung an; jede steht als Begründung im Code und
nicht nur hier.

- **Eine Zeile fängt am Textanfang an und hinter jedem `\n`.** Ein Text, der auf
  `\n` endet, hat danach eine leere letzte Zeile. Sie ist die Stelle, an der die
  Schreibmarke steht, wenn der Nutzer am Ende einer solchen Datei
  weiterschreibt; die `NSTextView` zeigt sie. Der leere Text hat eine Zeile.
- **Die Zeilennummer 0 führt an den Textanfang** und trägt das Kennzeichen
  `VorDerErsten`. Eine Nummer über der Zeilenzahl führt an das Textende, nicht
  an den Anfang der letzten Zeile; bei einem Text ohne abschließenden Umbruch
  sind beide verschieden, und C5 sagt "springt an das Dateiende" zu.
- **Die drei Auswahlfunktionen laufen um.** `erster_ab` zählt den Treffer unter
  der Schreibmarke mit (eine frisch begonnene Suche soll die Stelle nicht
  übergehen), `naechster` und `voriger` nicht (sonst bewegte sich der Befehl
  nicht). Alle drei arbeiten auf einem Versatz und nicht auf einer Trefferstelle:
  damit gibt es den Fall "die gemerkte Stelle passt nicht mehr zur Liste" nicht.
- **Das einzelne Ersetzen läuft nicht um** und sucht den nächsten Treffer im
  neuen Stand ab dem Ende des eingesetzten Textes. Ein Umlauf schickte den
  Nutzer zurück in genau das, was er eben eingesetzt hat.

`str::replace` steht bewusst nicht im Modul: es setzt seinen Ersatz bei leerem
Suchtext an jede Zeichengrenze (`"abc".replace("", "-")` liefert `"-a-b-c-"`),
und das Abnahmekriterium verlangt das Gegenteil. Alles Ersetzen steht deshalb
auf der Trefferliste aus `alle`, und der leere Suchtext ist genau einmal
behandelt.

## Abnahme

- `cargo test -p krk-core`: 0, 122 Proben in der Bibliothek (darunter 12 neue in
  `text::`), 5 neue in `tests/text.rs`, alle übrigen Prüfdateien unverändert grün.
- `cargo clippy -p krk-core --all-targets -- -D warnings`: 0, keine Meldung.
- `cargo fmt -p krk-core -- --check`: sauber.

Die fünf Proben des Abnahmekriteriums, alle abgedeckt:

1. `zehntausend_zeilen_liefern_dieselben_versaetze_wie_ein_durchlauf_von_hand`
2. `die_null_und_eine_zu_grosse_nummer_liefern_je_ein_kennzeichen`
3. `treffer_in_umlauten_und_emojis_liegen_auf_zeichengrenzen`
4. `ein_ersatz_der_den_suchtext_enthaelt_endet_und_zaehlt_richtig`
5. `ein_leerer_suchtext_liefert_null_treffer_und_aendert_nichts`

## Übergabe

**S37 (Ersetzen) braucht einen Handgriff, den S8 nicht leisten kann.** Sein
Abnahmekriterium nennt "einen Ersatztext mit `\r\n`, der als `\n` im Stand
ankommt". `text::suche` normalisiert nichts: die eine Stelle, die `\r\n` zu `\n`
macht, entsteht in S9 beim Einlesen. Der Ersatztext aus dem Textfeld muss
deshalb bei der Übernahme durch dieselbe Funktion laufen, sonst entstünde eine
zweite Normalisierungsstelle.

**S12 (Suche in der Nähe einer Textmarke) bekommt den Zeileninhalt aus dem
Index** über `anfang_der_zeile(n)` und `anfang_der_zeile(n + 1)`. Ein eigener
Zugriff auf die Zeilenanfänge ist nicht nötig und wurde deshalb nicht gebaut.

Nicht angefasst, wie beauftragt: `crates/krk-ui/`, `crates/krk-core/tests/belegung.rs`,
`crates/krk-core/src/ablage/`, `crates/krk-ui/src/fenstermodell.rs`.

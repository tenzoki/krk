# S46 und S47: eine Nummernspalte, zweimal eingehängt

**Status:** Complete
**Agent:** coder
**Datum:** 260809-2134
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritte 46 und 47
**Spec:** `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, Fähigkeit C10

## Was entstanden ist

Eine `NSRulerView`-Unterklasse in `crates/krk-ui/src/appkit/nummernspalte.rs`,
eingehängt in die senkrechte Linealstelle der Bildlaufansicht — im Editor über
`textflaeche_bauen`, in der Vorschau über `textanzeige`. **Eine Klasse, zwei
Aufrufstellen**, belegt durch `grep -rn "super = NSRulerView" crates/krk-ui/src`
mit genau einem Treffer und `grep -rn "Nummernspalte::einhaengen"` mit genau
zwei.

Ob sie steht, entscheidet die Bildlaufansicht über `setRulersVisible`. Der
Editor zeigt sie immer; die Vorschau fragt `Vorschaumodell::zeigt_dateitext`,
eine erschöpfende Fallunterscheidung über die fünf Werte von `Inhalt` ohne
Auffangzweig, wahr allein für `Inhalt::Text` **mit** Pfad.

## Der Befund, der den Plan an einer Stelle ergänzt hat

**Der Zeilenindex rechnet in Byteversätzen, AppKit in UTF-16-Einheiten.**
`Zeilenindex::anfang_der_zeile` liefert Byteversätze eines UTF-8-Textes,
`NSLayoutManager::characterIndexForGlyphAtIndex:` dagegen eine Stelle in
UTF-16-Einheiten. Die Antwort aus Frage 15 des Plans ("den Zeichenversatz seines
Anfangs nehmen und damit `zeile_am_versatz` fragen") setzt beide gleich; sie
sind es nur für reines ASCII. Ohne Umrechnung trüge jede Zeile hinter dem ersten
Umlaut eine falsche Nummer.

Umgesetzt als **Koordinatenwechsel und nicht als zweite Zählung**:
`anfaenge_in_utf16` fragt den Index nach dem Anfang jeder Zeile und läuft einmal
über den Text, um dieselben Anfänge in UTF-16 auszudrücken. Welche Stellen
Zeilenanfang sind und wie viele es gibt, sagt weiterhin allein der Index; die
Stelle eines Wertes in der umgerechneten Liste **ist** die um eins verminderte
Zeilennummer. Vier Proben in `crates/krk-ui` decken den leeren Text, reines
ASCII, Umlaute mit Bildzeichen und die leere letzte Zeile ab.

Statt `zeile_am_versatz` steht deshalb eine `binary_search` in der umgerechneten
Liste. Sie beantwortet beide Fragen des vierten Abnahmekriteriums von C10 auf
einmal: ein Treffer heißt "dieser Zeilenkasten fängt auf einem Dateizeilenanfang
an", und seine Stelle heißt "und zwar auf diesem". Ein Kasten ohne Treffer ist
die Fortsetzung einer umgelaufenen Zeile und bekommt keine Nummer.

## Zwei weitere Stellen, an denen der Plan nichts sagte

**Die leere letzte Zeile.** Ein Text, der auf einem Umbruch endet, hat nach dem
Zeilenindex eine leere letzte Zeile, und der Layoutverwalter führt sie als
`extraLineFragmentRect` außerhalb des Glyphenbereichs. Ohne einen eigenen Zweig
stünde die Schreibmarke am Dateiende neben keiner Nummer. Der leere Text geht
denselben Weg und trägt die 1.

**Die Breite wird nach dem Zeichnen gesetzt, nicht darin.** `setRuleThickness:`
legt die Bildlaufansicht neu aus, und eine Auslegung mitten in einem
Zeichendurchgang änderte die Geometrie, in der gerade gezeichnet wird.
`index_erneuern` stößt deshalb `dickeNachziehen` über die Laufschleife an, statt
die Breite sofort zu setzen. Der Standardmodus genügt: die Zeilenzahl ändert
sich beim Tippen und nicht beim Blättern.

## Was der Plan zugesagt hat und eingelöst ist

| Zusage | Wie eingelöst |
|---|---|
| Die Zählung kommt aus dem Kern | `grep -rEn "lines\(\)\|match_indices\|split\('\\n'\)" nummernspalte.rs` findet nichts; `Zeilenindex` steht elfmal darin |
| Zwei Beobachter, zwei Abmeldungen | `NSTextStorageDidProcessEditingNotification` und `NSViewBoundsDidChangeNotification`, abgemeldet in `Drop` |
| Der Index wird je Bild höchstens einmal gebaut | Der Textwechsel setzt nur ein Kennzeichen; gebaut wird beim Zeichnen |
| Die Grenze zu `appkit` hält in beide Richtungen | `grep -c 'objc2' crates/krk-ui/src/editormodell.rs` liefert 0; genau eine Datei trägt `#![allow(unsafe_code)]` |
| Keine Zeile trägt `#[allow(dead_code)]` | in `nummernspalte.rs` keine |
| `Nummernspalte::neu_zeichnen` steht für S33 bereit | öffentlich, und von beiden Meldungswegen benutzt |

## Die vier Abnahmekommandos

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 660 bestanden, 0 gescheitert |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo xtask bundle` | gebaut und signiert |

## Geänderte und neue Dateien

- neu: `crates/krk-ui/src/appkit/nummernspalte.rs`
- `crates/krk-ui/src/appkit/mod.rs` — `mod nummernspalte;` und der Modulkopf,
  der jetzt einundzwanzig Module aufzählt
- `crates/krk-ui/src/appkit/editor.rs` — `textflaeche_bauen` hängt die Spalte
  ein, dazu der Modulkopf
- `crates/krk-ui/src/appkit/vorschau.rs` — `textanzeige` hängt dieselbe Spalte
  ein, `anzeigen` schaltet sie, dazu der Modulkopf
- `crates/krk-ui/src/vorschaumodell.rs` — `zeigt_dateitext` samt Probe

`crates/krk-ui/src/editormodell.rs` trägt aus diesem Stand keine geänderte
Zeile: die Spalte liegt in `appkit`, das Modell nicht.

## Was ein Agent nicht abnehmen kann

Sieben der zwölf Abnahmekriterien von C10 verlangen den Blick auf den Schirm und
damit KRK im Vordergrund. Die Prüfliste dafür ist dem Nutzer übergeben. Drei
weitere Kriterien warten auf S33, S35 und S39 und stehen in der Abnahmeliste von
S42.

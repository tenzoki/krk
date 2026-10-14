Drei Sprach- und Typografiebefunde in fr.rs: U+00A0 vor einem Semikolon, „branchée“ für eingehängt, „Aller à la ligne“ als Zeilensprung
---
Die französische Tabelle trägt an einer Stelle das falsche geschützte Leerzeichen vor einem Semikolon, an einer Stelle ein Wort, das für einen Papierkorb nicht gebraucht wird, und als Namen des Befehls „Zu Zeile springen“ die feste Wendung für den Zeilenumbruch.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1640-reviewer-durchsicht-der-zwei-uebersetzten-sprachtabellen-eintrag-fuer-eintrag.md

Gelesen am Stand `5681f7a`, Eintrag für Eintrag gegen `de.rs`. Schwere: Low, alle drei. Keiner der drei Befunde ändert den Sinn eines Eintrags; der dritte macht einen Menüeintrag missverständlich.

| Datei:Zeile | ist | soll | Grund |
|---|---|---|---|
| `crates/krk-core/src/sprache/tabelle/fr.rs:453` (`LeisteFilterHinweis`) | `actif\u{a0}; il reste` | `actif\u{202f}; il reste` | Die Typografieregel in `tabelle/mod.rs` unter `# Typografie` und der Dateikopf von `fr.rs` (Zeilen 5–7) setzen vor `;` U+202F; jedes andere Semikolon der Datei hält das (`grep -c '\\u{202f};' fr.rs` gegen `grep -c '\\u{a0};' fr.rs`, die zweite Zahl ist 1). Die Probe `franzoesische_eintraege_tragen_die_franzoesische_typografie` (`crates/krk-core/tests/sprache.rs:205`) lässt vor `;` beide Zeichen zu und hat die Stelle deshalb nicht gefangen. |
| `fr.rs:104` (`VorgangKeinPapierkorb`) | `aucune Corbeille n’est branchée` | `aucune Corbeille n’est disponible` | Deutsch sagt „kein Papierkorb eingehängt“: der Datenträger führt keinen. „branché“ heißt angeschlossen, eingesteckt (ein Gerät am Kabel, ein Strom) und wird für einen Papierkorb nicht gesagt. Englisch sagt an derselben Stelle „no Trash attached“ (`en.rs:85`), das trägt den Sinn. |
| `fr.rs:1173` (`Kommando::EditorZeileSpringen`) | `Aller à la ligne` | `Atteindre la ligne…` | „aller à la ligne“ ist die feste Wendung für „eine neue Zeile beginnen“, also den Zeilenumbruch. Als Eintrag im Menü „Éditeur“ neben „Rechercher dans le texte“ liest ein französischer Nutzer den Umbruch und nicht „Zu Zeile springen“. Das Blatt dahinter fragt „À quelle ligne ?“ (`fr.rs:772`) und passt zu beiden Fassungen; „Atteindre“ ist das Verb, das Texteditoren für diesen Befehl führen. Die Schaltfläche des Blattes, „Aller“ (`fr.rs:773`), darf bleiben. |

**Abnahme:** `fr.rs` trägt an Zeile 453 U+202F vor dem Semikolon und kein `\u{a0};` mehr (`grep -c '\\u{a0};' crates/krk-core/src/sprache/tabelle/fr.rs` gibt 0); die Einträge `VorgangKeinPapierkorb` und `Kommando::EditorZeileSpringen` tragen den neuen Wortlaut; `cargo test -p krk-core` ist grün, insbesondere `keine_zwei_funktionen_tragen_in_einer_sprache_denselben_namen` (`crates/krk-core/tests/belegung.rs`), weil ein Befehlsname geändert wird.

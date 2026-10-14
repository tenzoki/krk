Das Glossar in tabelle/mod.rs lässt wiederkehrende Begriffe aus, und „Markierung“ und „Marke“ fallen in fr und en auf ein Wort zusammen
---
Das Glossar im Modulkopf von `crates/krk-core/src/sprache/tabelle/mod.rs` bindet nach seinem eigenen Kopf jeden Eintrag von `fr.rs` und `en.rs`, führt aber neun Begriffe nicht, die in den Tabellen mehrfach vorkommen; bei zweien davon sind die Tabellen uneinheitlich, und bei einem teilen sich zwei verschiedene deutsche Begriffe in beiden Sprachen dasselbe Wort.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1640-reviewer-durchsicht-der-zwei-uebersetzten-sprachtabellen-eintrag-fuer-eintrag.md, 261014-1640-fuenf-begriffe-stehen-in-fr-rs-in-zwei-schreibungen-papierkorb-verb-zuruecksetzen-haelt-eine-datei-zu-gross-editeur-apercu.md

Gelesen am Stand `5681f7a`. Schwere: Low. Das Glossar ist ein Kommentar in einer `.rs`-Datei; die Lücken halten keine Probe und keinen Bau an, sie lassen nur die nächste Übersetzung ohne Vorgabe.

**Lücken.** Je Zeile der deutsche Begriff, die Fassung, die die Tabellen heute (mehrheitlich) führen, und wo sie steht.

| Deutsch | fr.rs führt | en.rs führt | Stellen |
|---|---|---|---|
| Auswahl (die gewählte Zeile des Dateifensters) | sélection | selection | `fr.rs:904`, `:986`, `:1104–1107`; `en.rs:802`, `:879`, `:986–989` |
| Markierung (eines Eintrags für einen Vorgang) | marque | mark | `fr.rs:1039`, `:1117–1118`; `en.rs:925`, `:999–1000` |
| Tabelle (Aufgaben-, Notiz-, Termintabelle des Editors) | tableau | table | `fr.rs:847–849`; `en.rs:748–750` |
| Kombination (einer Belegung) | combinaison | combination | `fr.rs:401`, `:404`, `:485`, `:503`; `en.rs:361`, `:364`, `:439`, `:454` |
| Funktion (der Belegung) | fonction | function | `fr.rs:401`, `:471`, `:485`, `:508`; `en.rs:361`, `:429`, `:439`, `:458` |
| Unterbaum | sous-arborescence | subtree | `fr.rs:449`; `en.rs:407` |
| zu groß (eine Datei) | trop volumineux, einmal trop gros | too large | `fr.rs:329`, `:791`, `:1035`; `en.rs:700`, `:923` |
| hält (der Editor hält eine Datei) | tient, einmal contient | holds | `fr.rs:287`, `:802`, `:938`; `en.rs:259`, `:711`, `:835` |
| Rücktaste | retour arrière | Backspace | `fr.rs:485`; `en.rs:439` |

Die zwei uneinheitlichen (zu groß, hält) stehen mit ihren Zeilen im Datensatz zu `fr.rs`; hier ist allein die fehlende Vorgabe Gegenstand.

**Der Zusammenfall.** Das Glossar führt „Marke (die Spalte des Git-Status) → marque / mark“ (`mod.rs:96`). Dieselben Wörter tragen die Tabellen für „Markierung“, einen anderen Begriff: in der Statuszeile des Filters „, une marque masquée“ / „, one mark hidden“ (`Zahlwort::StatuszeileMarkierungenAusgeblendet`, `fr.rs:1039`, `en.rs:925`) und in den Befehlen „Supprimer toutes les marques“ / „Clear All Marks“, „Inverser les marques“ / „Invert Marks“ (`fr.rs:1117–1118`, `en.rs:999–1000`). Daneben heißt die Git-Spalte „Marque“ / „Mark“ (`fr.rs:443`, `en.rs:403`) und ihr Schalter „Afficher ou masquer la colonne Marque“ / „Show or Hide Mark Column“. Deutsch trennt die beiden (Markierung, Marke). Ein Nutzer, der „3 marques masquées“ neben einer Git-Spalte „Marque“ liest, kann beides meinen. Ein Befund ist das hier für das Glossar, das die Markierung nicht führt. Ob die Tabellen für die Markierung ein eigenes Wort bekommen, entscheidet, wer den Eintrag ins Glossar schreibt, und zieht dann die genannten Stellen nach; „sélection“ scheidet aus, weil die Auswahl daneben steht (speculation: fr „repère“ und en „tag“ sind in beiden Tabellen noch frei).

**Abnahme:** das Glossar in `tabelle/mod.rs` führt je eine Zeile für die neun Begriffe der Tabelle und benennt beim Eintrag „Markierung“ die Abgrenzung zur „Marke“; die Fassung, die es für „zu groß“ und „hält“ festlegt, steht an jeder genannten Stelle von `fr.rs`; `cargo test -p krk-core` ist grün.

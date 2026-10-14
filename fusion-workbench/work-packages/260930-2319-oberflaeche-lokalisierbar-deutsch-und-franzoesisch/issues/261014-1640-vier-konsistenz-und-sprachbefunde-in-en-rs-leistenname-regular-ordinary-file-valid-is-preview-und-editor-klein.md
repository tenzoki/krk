Vier Konsistenz- und Sprachbefunde in en.rs: der Name der Leiste, „regular“ neben „ordinary file“, „valid is a location“ und Preview und Editor klein geschrieben
---
Die englische Tabelle nennt die Lesezeichen- und Geräteleiste in einer Fassung in der Einzahl und sonst in der Mehrzahl, die gewöhnliche Datei mit zwei Wörtern, baut drei Sätze mit einer deutschen Satzstellung („valid is a location …“) und schreibt Preview und Editor an zwei Stellen klein, wo sie sonst Eigennamen sind.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1640-reviewer-durchsicht-der-zwei-uebersetzten-sprachtabellen-eintrag-fuer-eintrag.md

Gelesen am Stand `5681f7a`, Eintrag für Eintrag gegen `de.rs`. Schwere: Low. Kein Eintrag sagt etwas Falsches.

| Datei:Zeile | ist | soll | Grund |
|---|---|---|---|
| `crates/krk-core/src/sprache/tabelle/en.rs:14` (`WirkungsbereichLeiste`) | `bookmark and volume bar` | `bookmarks and volumes bar` | Die Leiste heißt in jedem anderen Eintrag so: `BereichLesezeichenLang` (`en.rs:393`, `the bookmarks and volumes bar`), `Kommando::LeisteUmschalten` (`en.rs:1019`, `Show or Hide Bookmarks and Volumes Bar`), `Kommando::FokusLeiste` (`en.rs:1045`). Der Eintrag steht in der Spalte „Acts in“ der Markdown-Ausgabe neben genau diesen Befehlsnamen. |
| `en.rs:170` (`EinstellungenKeineGewoehnlicheDatei`) | `something other than an ordinary file` | `something other than a regular file` | „gewöhnliche Datei“ heißt sonst „regular file“: `VorgangKeineGewoehnlicheDatei` (`en.rs:86`), `EditorKeineGewoehnlicheDatei` (`en.rs:126`), `DuplikatNichtGewoehnlich` (`en.rs:527`). „regular file“ ist zugleich der Begriff von POSIX und des Finders. |
| `en.rs:181` (`WerksKeineDatei`) | `stands something other than an ordinary file` | `stands something other than a regular file` | wie oben. |
| `en.rs:235` (`OrtLeer`) | `valid is a location starting with “~/” or “/”, by default “~/{ordnername}”` | `a valid location starts with “~/” or “/”, by default “~/{ordnername}”` | „gültig ist ein Ort“ ist wörtlich übertragen; im Englischen steht das Subjekt vor dem Verb, und „valid is …“ liest sich als Satzbau aus einer anderen Sprache. |
| `en.rs:241` (`OrtFremdesBenutzerverzeichnis`) | `valid is “~/” for your own or a path starting with “/”` | `“~/” stands for your own, and a path starting with “/” is valid as well` | wie oben; die zwei Möglichkeiten ausgeschrieben, damit „your own“ sein Bezugswort (home directory) im selben Satz behält. |
| `en.rs:244` (`OrtKeinText`) | `valid is a location in quotation marks, such as {beispiel}` | `a valid location is in quotation marks, such as {beispiel}` | wie oben. |
| `en.rs:396` (`BereichVorschauLang`) | `the preview pane` | `the Preview pane` | „Preview“ ist in der Tabelle der Eigenname des Bereichs und steht groß (`en.rs:15`, `:27`, `:28`, `:220`, `:884`, `:1022`, `:1078–1080`). Der Eintrag erscheint im Hinweistext „Show or hide the preview pane“ (`LeisteBereichUmschalten`). `StapelErlaeuterung` (`en.rs:658`, `The preview shows what the command would do`) meint die Vorschautabelle des Blattes und bleibt klein. |
| `en.rs:397` (`BereichEditorLang`) | `the built-in editor` | `the built-in Editor` | „Editor“ steht in jedem anderen Eintrag groß (`en.rs:16–24`, `:120`, `:122`, `:125`, `:259`, `:425`, `:711`, `:748–750`, `:835`, `:1049–1052`). |
| `en.rs:950–951` (`Zahlwort::StartMeldungen`) | `There was one message at start` / `There were {n} messages at start` | `There was one message at start-up` / `There were {n} messages at start-up` | Der Start heißt sonst „start-up“: `OrtEinstellungenUngelesen` (`en.rs:253`, `at start-up`), `NeuerungenSchlusssatz` (`en.rs:229`, `since start-up`); „at start“ allein ist kein gebräuchlicher Ausdruck. |

**Abnahme:** je Zeile der Tabelle trägt `en.rs` den Wortlaut aus „soll“; `grep -c 'ordinary file\|valid is\|the preview pane\|built-in editor\|at start"' crates/krk-core/src/sprache/tabelle/en.rs` gibt 0; `cargo test -p krk-core` ist grün, insbesondere `die_platzhalter_sind_in_allen_sprachen_gleich` (`crates/krk-core/tests/sprache.rs`), weil drei Einträge mit Platzhaltern umformuliert werden.

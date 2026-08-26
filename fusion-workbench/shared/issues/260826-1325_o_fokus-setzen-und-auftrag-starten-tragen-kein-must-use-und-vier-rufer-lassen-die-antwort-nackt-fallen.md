`fokus_setzen` und `auftrag_starten` tragen kein `#[must_use]`, und vier Rufer in `anwendung.rs` lassen ihre Antwort nackt fallen

---

`CLAUDE.md` legt fest: ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`, und `let _ =` heißt überall „ich brauche den Wert nicht". `Anwendungsdelegierter::fokus_setzen` liefert `false`, wenn der Fokus stumm **nicht** gesetzt wurde (Bereich ausgeblendet, kein Fenster, keine Ansicht), trägt kein Attribut, und drei Rufer werfen die Antwort ohne `let _ =` weg. `auftrag_starten` liefert `bool`, fünf Rufer schreiben `let _ =` mit Begründung, der sechste ruft nackt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum (`crates/krk-ui/src/appkit/anwendung.rs`)

`fn fokus_setzen(&self, ziel: Fokus) -> bool` (`2400-2420`), ohne `#[must_use]`; der Doc-Kommentar sagt „Beides scheitert still" (`2414-2418`). Rufer:

| Zeile | Form | Lage |
|---|---|---|
| `1458` | nackt | Aufbau, `BEIM_START` |
| `2256` | `let gesetzt = …` | `fokus_holen`, gelesen |
| `3210` | nackt | `Kommando::FensterWechseln`, die Rangmitnahme vom 260825 |
| `4468` | nackt | `nach_dem_sichtbarkeitswechsel`, Fokus raus aus dem ausgeblendeten Randbereich |
| `4584` | nackt | `aktives_setzen`, `Rangmitnahme::Krk` |

Die drei nackten Rufer bei `3210`, `4468` und `4584` sind genau die Stellen, an denen die Runde 18 die Zusage „jeder Schreiber von `aktiv` nimmt den Rang mit" aufgehängt hat (`aktivschreiberproben`, `8786-8881`). Scheitert `fokus_setzen` dort still — etwa weil `fokusansicht` für ein noch nicht gebautes Dateifenster `None` liefert —, fällt der Rang und das aktive Dateifenster genau so auseinander, wie es `shared/decisions/260825-1725_*` beschreibt, und nichts wird rot. Das Attribut zwingt zur Aussage `let _ =` und nennt den Preis an der Stelle; zum Vergleich trägt `bereich_einblenden` (`4340`) das Attribut mit demselben Grund („eine Abweisung bleibt stumm").

`fn auftrag_starten(…) -> bool` (`6372-6414`), ohne Attribut. Rufer: `5610`, `6175`, `6261`, `6312` mit `let _ =` und Kommentar; `6044` gibt den Wert weiter; `5868` (`stapel_beauftragen`) ruft nackt. Das ist kein Fehlverhalten — der Wert ist immer `true` —, aber die sechs Rufer schreiben dieselbe Sache in zwei Formen.

## Verwandt und nicht dasselbe

`shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-…` und `260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…` betreffen `krk-core`; dieser Datensatz betrifft die zwei Stellen des Anwendungsdelegierten, an denen die Abweisung wirklich stumm ist.

## Vorschlag

`#[must_use = "eine Abweisung bleibt stumm; …"]` an `fokus_setzen`, dann `let _ =` mit einer Zeile Begründung an `1458`, `3210`, `4468`, `4584` — oder, wo die Antwort tragend ist (`3210`, `4584`), sie lesen und den Fall benennen. An `5868` `let _ =` wie bei den fünf Geschwistern.

Gefunden bei der Vollbaum-Durchsicht R7 an HEAD `7ac511a`.

Das Zählkommando für `ohne_warten_oeffnen` gibt elf Zeilen für sechs Rufer aus

---
Zwei Prosastellen nennen `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` als das Mittel, mit dem die Zahl der Aufrufer zu zählen ist. Am Baum `8779a25` gibt es elf Zeilen aus, von denen fünf keine Aufrufe sind.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** niedrig
**Baumstand:** `8779a25`
**Betroffen:** `crates/krk-core/src/verzeichnis/sys.rs` (Modulkopf, Abschnitt zum Namen des Moduls; und der Doc-Kommentar von `ohne_warten_oeffnen` selbst), `CLAUDE.md` (Abschnitt „Was man nicht sieht, wenn man es nicht weiß", Absatz „Die Prüfung dessen, was da geöffnet wurde, steht am Deskriptor und nicht am Pfad")

## Befund

Beide Stellen ersetzen eine Zahl durch ein Kommando, und das ist die Regel dieses Projekts. Das gewählte Kommando beantwortet aber eine andere Frage als die, die daneben gestellt ist.

`grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` liefert elf Zeilen:

| Zeile | Art |
|---|---|
| `operation/entpacken.rs:137` | Aufruf |
| `operation/zippen.rs:389` | Aufruf |
| `verzeichnis/sys.rs:245` | Aufruf (`Schwungleser::oeffnen`) |
| `text/datei.rs:437`, `:623`, `:695` | Aufrufe |
| `verzeichnis/sys.rs:78`, `:85` | Modulkopf, nennt das Kommando |
| `verzeichnis/sys.rs:914` | Doc-Kommentar, nennt das Kommando |
| `verzeichnis/sys.rs:919` | die Definition `pub fn ohne_warten_oeffnen` |
| `verzeichnis/sys.rs:1411` | eine Probe |

Sechs Aufrufer, fünf Zeilen, die keine sind. Vier der fünf sind Prosa über das Kommando selbst und wachsen mit jeder weiteren Stelle, die es nennt: wer eine dritte Prosastelle schreibt, hebt das Ergebnis um eins, ohne dass ein Aufrufer dazugekommen wäre.

Das ist dieselbe Bauart, die dieses Projekt an anderer Stelle schon geschlossen hat: die Zählprobe `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` (`crates/krk-core/tests/baum.rs`) vergleicht die ganze Zeile und nicht ihr Vorkommen im Text, und ihr Kommentar sagt warum — „ein `contains` zaehlte jede Erwaehnung mit und machte aus einer Zusage ueber den Bau eine ueber die Prosa". Hier steht der Befund noch offen.

## Was die Wahl erschwert

Der Modulkopf begründet ausdrücklich, warum das **breite** Muster nötig ist: `entpacken.rs` holt den Namen über `use` herein und ruft ihn unqualifiziert, entgeht dem engeren `sys::ohne_warten_oeffnen(` also. Ein enger gefasstes Muster ist deshalb keine Abhilfe. Was zu trennen ist, ist Code von Prosa.

## Abhilfe

Ein Kommando, das Doc-Zeilen, Definition und Probe ausnimmt, etwa über ein vorgeschaltetes `grep -v` gegen `//` oder über einen Lauf, der die Definitionszeile und die Prosa nicht mitzählt. Beide Prosastellen nehmen dann dasselbe Kommando. Ob stattdessen eine Zählprobe die richtige Antwort ist — wie bei `deny(unsafe_code)` —, gehört zu derselben offenen Frage wie die Bauform der `ALLE`-Listen (`shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`).

## Abnahme

Das Kommando, das die zwei Prosastellen nennen, gibt so viele Zeilen aus, wie es Aufrufer gibt, und die Zahl ändert sich nicht, wenn eine dritte Prosastelle über die Hülle geschrieben wird.

## Herkunft

Gemeinsamer Speicher. Gefunden beim Behebungsdurchgang an `CLAUDE.md` und `README.md` vom 260905-2254, beim Nachprüfen des Kommandos, das der Absatz zu `ohne_warten_oeffnen` nennt. Kein Circle war aktiv, und der Befund betrifft eine projektweite Gewohnheit.

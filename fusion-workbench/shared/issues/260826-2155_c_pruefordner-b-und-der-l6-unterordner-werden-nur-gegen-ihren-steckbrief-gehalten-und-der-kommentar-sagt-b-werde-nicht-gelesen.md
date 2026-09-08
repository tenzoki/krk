Prüfordner B und der L6-Unterordner werden nur gegen ihren Steckbrief gehalten, und der Kommentar sagt, B werde nicht gelesen

---

`pruefordner_pruefen` (`crates/krk-bench/src/messen.rs:1587-1608`) zählt keinen einzigen Eintrag: es vergleicht die Zahl **im Steckbrief** mit der zugesagten. Der Inhalt wird erst dadurch gedeckt, dass `Messreihe::fahren` (`:182-198`) die tatsächlich gelesene Zahl gegen denselben Steckbrief hält. Diese zweite Hälfte gibt es nur für Prüfordner A und den großen Ordner. **Prüfordner B und der L6-Unterordner haben sie nicht**: ein Steckbrief, der 10.000 sagt, neben einem Ordner mit 3.000 Einträgen kommt bei beiden durch, und L4, L5 und L6 messen dann auf einem Bestand, den keine Zusage meint. Der Kommentar in `bericht.rs:261-262` begründet die fehlende Zahl für B mit „er dient dem Fensterwechsel und wird nicht gelesen"; gelesen wird er, nur nicht von `krk-bench`.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-bench/src/messen.rs:1587-1608`, `:1610-1634` (`unterordner_sicherstellen`), `:1804`, `:1808`; `crates/krk-bench/src/bericht.rs:257-271`
**Cross-references:** `shared/issues/260826-1301_c_kein-pruefordner-ausser-dem-l6-unterordner-wird-gegen-seine-zugesagte-eintragszahl-gehalten.md` — die Behebung setzt den „Denkbaren Weg" jenes Datensatzes wörtlich um; dieser Befund hält fest, was dieser Weg offen läßt, damit die Schließung nicht als „jeder Prüfordner gegen seinen Inhalt" gelesen wird

## Die zwei Hälften und wer sie bekommt

| Ordner | Steckbrief gegen Zusage | gelesene Zahl gegen Steckbrief |
|---|---|---|
| A | `Gesamtlauf::fahren:1077` | `Messreihe::fahren`, `eine_gesamtrunde:1266` |
| 100k | `Gesamtlauf::fahren:1077` | `Messreihe::fahren`, `eine_gesamtrunde:1267` |
| B | `Gesamtlauf::fahren:1077` | — |
| L6-Unterordner | `unterordner_sicherstellen:1631` | — |

Nur wo beide Hälften stehen, folgt „gelesen == zugesagt". Für B und den L6-Unterordner bleibt allein die Aussage einer Textdatei neben dem Ordner. Der Fall, den `260826-1301` selbst nennt — „ein hineingerutschter `.DS_Store`, ein von Hand gelöschter Eintrag" — ist für diese beiden unverändert offen.

Der Commit-Betreff von `960900d` lautet „jeder Pruefordner wird gegen seine zugesagte Eintragszahl gehalten", und die `Resolved:`-Zeile des Datensatzes wiederholt es. Gehalten wird für zwei von vier der Steckbrief, nicht der Bestand.

## Der Kommentar über B trägt seine zweite Hälfte nicht

`crates/krk-bench/src/bericht.rs:261-262`:

```rust
// B traegt keine eigene kopflose Reihe: er dient dem Fensterwechsel und
// wird nicht gelesen. Fuer ihn bleibt der Steckbrief die einzige Auskunft.
```

Der erste Halbsatz stimmt: `krk-bench` fährt keine `Messreihe` über B. Der zweite nicht: `plan_in_verzeichnis_schreiben` (`messen.rs:1800-1811`) schreibt B als Tab in **beide** Dateifenster der Prüfsitzung, und KRK liest ihn bei jedem L5-Tabwechsel und jedem L5-Fensterwechsel. Genau deshalb ist seine Eintragszahl Bestandteil der Zusage. Wer den Satz liest, hält B für einen Ordner, dessen Inhalt gleichgültig ist.

## Zwei Wege

- **Ehrlich beschriften.** Den Kommentar auf „B wird von der Anwendung gelesen, von `krk-bench` nicht; darum gibt es hier keine gelesene Zahl" ändern und im Berichtskopf für B und den L6-Unterordner kenntlich machen, dass ihre Zahl unbestätigt ist.
- **Die zweite Hälfte nachziehen.** Für beide reicht ein Zählen ohne Zeitmessung: `einen_lauf_fahren` (`messen.rs`) liefert die Eintragszahl schon, und `pruefordner_pruefen` könnte sie neben dem Steckbrief anfordern. Kostet je Lauf einen Verzeichnisdurchgang über 10.000 beziehungsweise 1.000 Einträge, einmal vor der ersten Runde.

Welcher Weg, ist eine Nutzerentscheidung; der erste ist Wahrheit im Bericht, der zweite Deckung.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.

---
Resolved: Beide Hälften des Befunds sind auf dem ersten der zwei Wege behoben, „ehrlich beschriften". Der Kommentar über B trägt seine zweite Hälfte seit dem 260905. Der Berichtskopf macht seit dem 260907-2350 kenntlich, dass die Zahl von Prüfordner B und des L6-Unterordners unbestätigt ist: die Zeile „Deckung der Ordner" (`crates/krk-bench/src/bericht.rs`, `DECKUNG_DER_ORDNER`) sagt, welche zwei Ordner eine gelesene Zahl tragen und welche zwei nur eine zugesagte, was ein hineingerutschter Eintrag dort anrichtete und welche Zusagen daran hängen; gehalten wird der Satz von `der_abnahmebericht_traegt_alle_zehn_zusagen_und_den_vollen_kopf` in derselben Datei, mitsamt der gezählten Behauptung, dass genau zwei der vier Ordner eine gelesene Zahl tragen. **Der zweite Weg ist ausdrücklich nicht gegangen** und die Deckungslücke besteht fort: der Nutzer hat am 260907-2334 Möglichkeit 1 gewählt, weil ein Vorablesen von B den Systemcache wärmte, gegen den die zwei getrennten Startwerte von A und B gebaut sind (`260905-2155_*_bekommen-pruefordner-b-und-der-l6-unterordner-die-zweite-haelfte-der-deckung.md`). Der Befund ist damit als Lage angenommen und beschriftet, nicht aufgehoben. Dass die Aufzählung der betroffenen Zusagen in diesem Datensatz L7 nicht kennt, führt `260907-2350_*_zwei-datensaetze-nennen-l4-l5-und-l6-als-betroffene-zusagen-seit-dem-260907-haengt-auch-l7-am-l6-unterordner.md`.

---
Nachtrag 260908: die Aufzählung „L4, L5 und L6" im Rumpf oben und im Abschnitt
„Die zwei Hälften und wer sie bekommt" war am 260826 vollständig und ist es seit
`1936a0f` nicht mehr. L7 misst seitdem zwei Spannen, und die zweite ist der
Sprung auf den L6-Unterordner; an dessen ungeprüfter Eintragszahl hängt damit
neben L6 auch die L7-Zeile für den Ordnersprung. Welche Zusage auf welchem
Ordner misst, sagt `DECKUNG_DER_ORDNER` (`crates/krk-bench/src/bericht.rs`) und
keine Aufzählung in einem Datensatz — dort steht die vollständige Liste, und sie
wird hier nicht wiederholt
(`260907-2350_*_zwei-datensaetze-nennen-l4-l5-und-l6-als-betroffene-zusagen-seit-dem-260907-haengt-auch-l7-am-l6-unterordner.md`).

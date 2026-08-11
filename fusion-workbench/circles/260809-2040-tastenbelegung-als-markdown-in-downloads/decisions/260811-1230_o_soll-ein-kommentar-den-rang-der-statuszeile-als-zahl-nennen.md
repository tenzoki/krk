# Soll ein Kommentar den Rang der Statuszeile als Zahl nennen?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (nach der Einschätzung des coder bei der Behebung von `260811-1210`)
**Cross-references:** `issues/260811-1210_*_eine-dritte-stelle-nennt-den-rang-der-fenstermeldung-falsch.md`,
`issues/260811-0838_*_antwort-zeigen-nennt-vier-raenge-die-statuszeile-fuehrt-fuenf.md`,
`crates/krk-ui/src/appkit/statuszeile.rs:75-83` und `:125-130`

---

## Frage

Vier Kommentare in zwei Dateien haben den Rang einer Meldung in der Statuszeile falsch genannt,
und alle vier sind am 260811 berichtigt worden:

| Stelle | stand da | ist |
|---|---|---|
| `anwendung.rs:3334` | „vier Ränge" | fünf |
| `anwendung.rs`, `Dateifenstersicht::melden` | „einen Rang tiefer" | Rang 3 |
| `anwendung.rs:3620` | „einen Rang tiefer" | Rang 3 |
| `tabelle.rs:322` | „der oberste der vier Ränge" | fünf |

Der letzte widersprach dabei `tabelle.rs:1392` in derselben Datei, wo „der fünfte Rang" schon
richtig stand. Alle vier stammen aus der Zeit vor S16c, als die Statuszeile vier Ränge hatte und
der Markierungsstand noch fehlte.

**Vier Fehler derselben Sorte sind kein Zufall.** Die Frage ist, ob die Prosa die Zahl weiter
nennen soll.

## Was die Frage **nicht** ist

Die naheliegende Antwort — „die Ränge aus einer Stelle lesen statt sie zu beschreiben" — gibt es
nicht. **Kein Prüflauf liest eine Zahl in einem Kommentar.** Was auch immer gebaut wird, der
nächste falsche Kommentar bleibt unbemerkt, bis jemand ihn liest.

## Optionen

1. **So lassen.** Die vier sind berichtigt, achtzehn weitere Stellen sind bei dieser Gelegenheit
   gegen `statuszeile.rs:75-83` geprüft und richtig befunden.
   - Pro: kostet nichts. Der Bestand ist heute nachweislich sauber.
   - Contra: der sechste Rang, falls er je kommt, macht wieder alle Zahlen falsch, und niemand
     merkt es.

2. **Die Zahl weglassen, die Quelle benennen.** Statt „Rang 3" ein Doc-Link auf
   `statuszeile::zeile`, wo die Ordnung genau einmal steht — in der Parameterfolge und der
   `or_else`-Kette (`statuszeile.rs:125-130`).
   - Pro: nichts veraltet mehr. Reine Prosaarbeit, kein Umbau.
   - Contra: rund neun Absätze in drei Dateien umzuschreiben. Und die Zahl sagt **mehr** als der
     Name: dass die Fenstermeldung *unter* der Vorgangsanzeige steht. Wo ein Kommentar über
     Verdrängung argumentiert, bräuchte er dann einen Satz statt einer Ziffer.

3. **Ein `Quelle`-Enum in `statuszeile.rs`** mit fünf Varianten in Rangfolge, über das `zeile`
   läuft.
   - Pro: die Ordnung wird Daten statt Parameterfolge, und die Doc-Links bekommen einen Anker.
   - Contra: Umbau von `zeile`, seinem einen Aufrufer (`tabelle.rs:1383`) und den Proben — **und
     die Zahl im Kommentar prüft er trotzdem nicht.** Für einen Kommentarfehler zahlt sich das
     nicht aus.

## Empfehlung

**Option 2, aber nicht jetzt.** Der Bestand ist heute sauber, und die Arbeit lohnt sich erst,
wenn jemand die Dateien ohnehin anfasst. Option 3 löst das eigentliche Problem nicht — der
`coder` sagt das ausdrücklich, und seine Einschätzung stammt aus dem Lesen und nicht aus einer
Messung.

Wer den sechsten Rang baut, entscheidet diese Frage mit; bis dahin ist sie es nicht wert,
Arbeit auszulösen.

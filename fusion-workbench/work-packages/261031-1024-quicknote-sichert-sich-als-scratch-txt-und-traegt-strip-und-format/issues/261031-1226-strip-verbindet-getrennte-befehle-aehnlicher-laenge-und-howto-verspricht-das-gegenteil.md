Strip verbindet zwei oder mehr getrennte Befehle ähnlicher Länge zu einer Zeile, und `HowTo.md` sagt „Befehle untereinander bleiben untereinander“
---
Seit `b21ca032` gilt die Breite als belegt, sobald zwei Zeilen die längste bis auf ein Zehntel erreichen. Zwei getrennte Befehle, deren Längen um höchstens ein Zehntel auseinanderliegen und die mindestens 40 Zeichen haben, belegen sie selbst. Strip hängt dann den zweiten als Argumente an den ersten. Die Klasse aus A1 ist damit enger geworden, aber nicht geschlossen, und die Folge ist dieselbe: Strip ändert die Bedeutung, auch zerstörend.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1156-die-laengste-zeile-gilt-immer-als-umgebrochen-und-strip-verschluckt-befehle-format-tabellen.md`, `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`, `261031-1226-reviewer-nachdurchsicht-quicknote-behebungen.md`

**Ort:** `crates/krk-core/src/text/glaetten.rs`, `belegte_breite` (Zeile 571) mit `MINDESTZAHL_VOLLER_ZEILEN = 2` (Zeile 136), und `umgebrochen` (Zeile 542). Gerufen aus `befehl` (Zeile 314).

**Belege:** gemessen am Stand `c1c2b7f8`, `glaetten.rs` unverändert per `#[path]` in ein Wegwerfprogramm außerhalb des Baums eingebunden:

```
Eingabe:
rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b
  ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978
Strip:
rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978
```

Das `rm -rf` löscht danach auch das Verzeichnis, das allein gelistet werden sollte. Weitere Fälle mit derselben Ursache:

```
cargo test -p krk-core --test glaetten --test heimordner
  cargo test -p krk-ui -- quicknotesicherung quicknote_stellt
-> cargo test -p krk-core --test glaetten --test heimordner cargo test -p krk-ui -- quicknotesicherung quicknote_stellt

git log --oneline 129d8d71..c1c2b7f8 -- crates/krk-core
  git diff --stat 129d8d71..c1c2b7f8 -- crates/krk-ui/src
  git show --stat c1c2b7f8 -- crates/krk-core/src/text
  git status
-> alle vier auf einer Zeile
```

**Widersprüche in der Dokumentation:**
- `HowTo.md:1223`: „Befehle untereinander bleiben untereinander“. Vor `b21ca032` stand dort „kurze Befehle“. Das Wort ist weggefallen, und der Satz ist jetzt allgemein falsch. Die Liste darunter nennt den Fall („Zwei getrennte Zeilen, die beide fast die volle Breite einnehmen“), der Satz davor widerspricht ihr.
- Modulkopf `glaetten.rs:48-50`: Die Schätzung „darf sich deshalb irren, solange sie es selten und in der Richtung tut, die nichts verbindet“. Die Fälle oben irren in der Richtung, die verbindet.

**Warum keine Probe es sieht:** `strip_haengt_zwei_befehle_nicht_aneinander_wenn_der_laengere_zuerst_steht` (`crates/krk-core/tests/glaetten.rs`) nimmt zwei Befehle, deren Längen weit auseinanderliegen. Keine Probe hat zwei getrennte Befehle ähnlicher Länge.

**Zu klären, bevor gebaut wird:** Aus dem Text allein ist ein zweimal umbrochener Befehl nicht von zwei getrennten Befehlen ähnlicher Länge zu unterscheiden. In welche Richtung Strip dann irrt, ist die offene Frage `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`. Unabhängig von der Antwort gilt die erste Abnahmezeile.

**Abnahme:**
- `HowTo.md` (Abschnitt „Strip und Format“) und der Modulkopf von `glaetten.rs` behaupten nicht mehr, dass Befehle untereinander stehen bleiben oder dass die Schätzung nur in der Richtung irrt, die nichts verbindet. Sie nennen den Fall zweier getrennter Befehle ähnlicher Länge so, wie er auftritt, und sagen, dass das Ergebnis den zweiten als Argumente des ersten ausführt.
- Eine Probe in `crates/krk-core/tests/glaetten.rs` hält das Verhalten für zwei getrennte Befehle ähnlicher Länge fest (etwa die `rm`/`ls`-Eingabe oben), mit der Erwartung, die aus der Antwort auf die Entscheidungsfrage folgt.
- `das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe` und `die_beispiele_der_anleitung_stimmen` bleiben grün und unverändert.

**Resolved:** 261031, durch die Antwort des Nutzers ueberholt: Singlestrip nimmt die Eingabe als einen Befehl und verbindet jeden Umbruch (`f39e14fb`, `46abb49a`); `HowTo.md` sagt, dass es fuer mehrere Befehle untereinander nicht gedacht ist, mit dem `rm`-Fall als Beispiel.

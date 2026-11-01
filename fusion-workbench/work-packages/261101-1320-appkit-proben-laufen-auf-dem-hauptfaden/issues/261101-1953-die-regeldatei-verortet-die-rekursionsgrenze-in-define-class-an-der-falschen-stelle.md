Die Regeldatei verortet die Rekursionsgrenze in `define_class!` an der falschen Stelle
---
Der letzte Punkt unter „Am Prüfziel `hauptfaden` hängen Kopplungen …“ in `.claude/rules/coding-was-man-nicht-sieht.md` nennt den falschen Kommentar, die falsche Fehlermeldung und den falschen Spielraum. Gemessen liegt die Grenze am Doc-Kommentar der Klasse `Namensfeld` und nicht an `umbenennung_beendet`.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-1436-plan-appkit-proben-laufen-auf-dem-hauptfaden.md, 261101-1953-reviewer-appkit-proben-hauptfaden.md

**Was die Regeldatei sagt** (eingeführt in `0067f466`): Der Doc-Kommentar von `umbenennung_beendet` im `define_class!`-Block von `DateifensterDelegierter` stehe an der Rekursionsgrenze. Eine Zeile mehr breche den Bau mit „recursion limit reached while expanding `stringify!`“ ab. Abhilfe sei, den Text an `DateifensterQuelle::umbenennung_beenden` zu verlegen.

**Was gemessen ist** (261101, Kopie von `0067f466` in der Kratzablage, `cargo check -p krk-ui`, eigenes Zielverzeichnis, `crates/krk-ui/src/appkit/tabelle.rs`):

| Einfügestelle | zusätzliche `///`-Zeilen | Ergebnis |
|---|---|---|
| nach Zeile 5334, Doc-Kommentar von `umbenennung_beendet` | 1, 10, 15, 30 | grün; mit 10 auch `cargo clippy --all-targets -- -D warnings` und `RUSTDOCFLAGS="-D warnings" cargo doc` grün |
| nach Zeile 5843, Doc-Kommentar der Klasse `Namensfeld` (`define_class!` ab Zeile 5814, `pub struct Namensfeld;` in Zeile 5890) | 1 | grün |
| dieselbe Stelle | 2, 3, 5, 10, 20 | rot in beiden Zielen `krk` und `hauptfaden`: `error: recursion limit reached while expanding $crate::__macro_helpers::concat!` |

Die Grenze liegt also am Kommentar über `pub struct Namensfeld;`, und dort ist noch genau eine Zeile frei. Gekürzt hat `b8966f9c` genau diesen Kommentar (Hunk `@@ -5828,14 +5828,13 @@`). Die vorgeschlagene Abhilfe passt nicht, weil ein Struct-Kommentar nicht an eine Methode umziehen kann. Am Code selbst steht kein Hinweis.

**Abnahme:**
- Der Punkt in der Regeldatei nennt `Namensfeld` (`appkit/tabelle.rs`), den Wortlaut `$crate::__macro_helpers::concat!` und den gemessenen Spielraum, oder er sagt ohne Zahl, dass der Kommentar an der Grenze steht. Die Abhilfe passt zu einem Struct-Kommentar, etwa Text vor `define_class!(` als `//`-Kommentar verlegen oder `#![recursion_limit]` in beiden Kistenwurzeln.
- Die Nennung von `umbenennung_beendet` und `DateifensterDelegierter` fällt.
- Nachprüfung: In einer Kopie bleibt `cargo check -p krk-ui --all-targets` mit einer zusätzlichen `///`-Zeile über `pub struct Namensfeld;` grün und wird mit zwei rot, oder mit der Zahl, die die neue Fassung nennt.

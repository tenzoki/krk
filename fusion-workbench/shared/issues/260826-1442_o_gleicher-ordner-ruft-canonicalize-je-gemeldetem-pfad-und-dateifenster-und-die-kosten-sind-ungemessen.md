`gleicher_ordner` ruft `canonicalize` je gemeldetem Pfad und Dateifenster, und die Kosten sind ungemessen
---
Jeder FSEvents-Pfad, der nicht zeichengleich mit dem Ordner eines Dateifensters ist, kostet in `ordner_neu_lesen` zwei `realpath(3)`-Aufrufe je Seite, also bis zu vier je Pfad, auf dem Hauptfaden; `auffrischung_aufgeschoben` und `betrifft_editordatei` rechnen je Pfad noch einmal so. Ein Stapel mit tausend Meldungen aus einem Unterordner (der gewöhnliche Fall, `:238-241`) macht daraus einige tausend Systemaufrufe je Rückruf. Gemessen ist das nicht.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/auffrischung.rs:481-489`: `gleicher_ordner` → `std::fs::canonicalize` auf beiden Seiten, sobald der Zeichenvergleich verneint.
- `:281-295`: `ordner_neu_lesen` ruft sie für beide Seiten je Pfad.
- `:214-221`: `betrifft_editordatei` je Pfad des Stapels.
- `:391-395`: `auffrischung_aufgeschoben` je Pfad und je aufgeschobenem Ordner.
- `:209-213` begründet für den Editor, warum „nicht ein Pfad je Ruf“, und nennt den `stat(2)` je Pfad als das, was vermieden wird; `gleicher_ordner` macht ihn trotzdem, zweimal.

Der Fall, in dem es zählt, ist der Aufschub selbst: ein Stapel-Umbenennen über 5 000 Einträge meldet jeden, und jede Meldung fragt `auffrischung_aufgeschoben` (zeichengleich, billig) und `ordner_neu_lesen` für beide Seiten (die andere Seite ist nicht zeichengleich → `canonicalize` beider Pfade).

`speculation:` Ob das auf dem Referenzgerät gegen L9 durchschlägt, ist ungemessen; der Datensatz `260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md` nennt die Nachbarfrage.

## Vorschlag

Die aufgelöste Form der zwei bis drei beobachteten Ordner einmal beim Setzen des Stroms berechnen und halten, statt sie je Meldung neu zu lösen; `canonicalize` dann nur für den gemeldeten Pfad, einmal je Pfad.

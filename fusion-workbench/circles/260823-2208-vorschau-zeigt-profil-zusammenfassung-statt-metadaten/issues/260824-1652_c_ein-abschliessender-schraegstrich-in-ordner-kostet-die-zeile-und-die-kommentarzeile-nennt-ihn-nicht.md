Ein abschließender Schrägstrich in `ordner` kostet die Zeile, und die Kommentarzeile nennt ihn nicht

---

`resources/default-readers.toml:70-73` zählt auf, was eine Ortsangabe abweist: „Ein absoluter Pfad,
ein `..` und ein doppelter Schrägstrich werden abgewiesen". Der Prüfschritt weist zwei weitere
Formen ab, und eine davon ist die wahrscheinlichste Schreibweise überhaupt: `ordner = "planning/"`.

---

**Gemessen am 260824-1652** an der Auslieferungsfassung, mit `toml` 1.1.4 und den Strukturen aus
`crates/krk-core/src/leseprofil/datei.rs`. Die Datei wurde je einmal verändert und neu gelesen:

| Änderung | Ausgang |
|---|---|
| `ordner = "planning"` → `ordner = "planning/"` | von `serde` **angenommen**, danach von `Ortsangabe::aus_angabe` abgewiesen |
| `zaehlung` → `zahlung` | ganze Datei abgewiesen, Meldung nennt den Schlüssel |
| `feldmuster` → `feldmusster` | ganze Datei abgewiesen, Meldung nennt den Schlüssel |
| `pfad` → `pfda` | von `serde` angenommen, danach als Profil ohne Erkennungsmuster abgewiesen |

`Ortsangabe::aus_angabe` (`crates/krk-core/src/leseprofil/mod.rs:341-354`) zerlegt die Angabe an
`/` und weist ab: ein führender `/` (`Ortsmangel::Absolut`), ein **leeres Stück**
(`Ortsmangel::LeeresStueck`, also `a//b` **und** `planning/`) sowie `.` und `..`
(`Ortsmangel::Punktstueck`). Der Kommentar nennt drei der fünf Formen und lässt gerade die zwei
aus, die kein Tippfehler sind, sondern eine gewöhnliche Schreibgewohnheit.

**Was es kostet.** Die dritte Reichweite aus `:44-49`: die Zeile verliert ihren Baustein, behält
ihre Beschriftung und zeigt `--`; die übrigen Zeilen stimmen weiter. Die Meldung dazu steht beim
Start in der Statuszeile und nennt den Grund („trägt ein leeres Stück"). Der Nutzer, der die Datei
liest, findet den abschließenden Schrägstrich in der Aufzählung jedoch nicht wieder.

**Vorschlag.** Den Satz um die zwei fehlenden Formen ergänzen, etwa: „Ein absoluter Pfad, ein `.`,
ein `..` sowie ein doppelter oder ein abschließender Schrägstrich werden abgewiesen."

Gefunden bei der Durchsicht der Auslieferungsfassung, `reviews/260824-1655-ontorev-…`.

---
Resolved: Die Aufzählung in `resources/default-readers.toml:69-75` nennt jetzt alle fünf Formen,
die `Ortsangabe::aus_angabe` abweist: einen absoluten Pfad, ein `.`, ein `..` sowie einen
doppelten und einen abschließenden Schrägstrich. Der abschließende ist ausgeschrieben und mit dem
Beispiel `ordner = "planning/"` belegt, samt dem Satz, dass er keine Verschreibung ist, sondern
eine gewöhnliche Schreibgewohnheit, und die Zeile trotzdem ihren Baustein kostet.

Gegengelesen an `crates/krk-core/src/leseprofil/mod.rs:345-359`: `aus_angabe` weist ab bei
führendem `/` (`Ortsmangel::Absolut`), bei leerem Stück (`Ortsmangel::LeeresStueck`, also `a//b`
**und** `planning/`) und bei `.` oder `..` (`Ortsmangel::Punktstueck`). Fünf Formen, drei
Mangelarten, und die Aufzählung führt jetzt die Formen und nicht die Mangelarten.

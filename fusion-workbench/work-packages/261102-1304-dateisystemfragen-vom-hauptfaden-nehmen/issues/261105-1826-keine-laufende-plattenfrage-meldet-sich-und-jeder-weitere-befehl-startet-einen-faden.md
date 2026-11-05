Keine laufende Plattenfrage meldet sich, und jeder weitere Befehl startet einen Faden
---
Schwere: Niedrig. Auf einer hängenden oder langsamen Platte geschieht nach dem Befehl sichtbar nichts. Der Nutzer hat das nicht entschieden, und die Umsetzung hat es nur benannt.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** Löschen (`crates/krk-ui/src/appkit/anwendung.rs`, `loeschbefund_erheben_lassen`), F2 (`notizordner_einziehen` `:6729`), F4 auf `secrets.txt` (`crates/krk-ui/src/appkit/editor.rs:5033`), der Pfadsprung (`pfad_pruefen_lassen` in `crates/krk-ui/src/appkit/tabelle.rs`) und das Lesezeichen (`anwendung.rs:3150`) stellen ihre Frage und zeigen bis zur Antwort nichts. `Text::LoeschpruefungLaeuftNoch` ist mit der Behebung von 261104-1957-die-sperre-des-loeschwegs-gilt-programmweit-und-faellt-ohne-antwort-nie gefallen. Jede weitere Auslösung startet einen eigenen Faden ohne Obergrenze, und der Resolved-Absatz jenes Datensatzes nennt das auch für die gehaltene Taste. Die Kosten je Faden begrenzt `umfang::zaehlen` (`crates/krk-core/src/verzeichnis/umfang.rs:219`), solange die Platte antwortet. Antwortet sie nicht, steht jeder dieser Fäden.

**Fehlerbild, abgeleitet und nicht gemessen.** Netzlaufwerk getrennt, Löschtaste: nichts. Der Nutzer drückt erneut, jeder Druck ein Faden, und nach dem Aufwachen fällt alles außer der jüngsten Antwort still. Ob die Tastenwiederholung beim Löschbefehl gefiltert wird, habe ich im Quelltext nicht abschließend gelesen.

Der frühere Befund am Abwurf (261104-1958-der-abwurf-startet-je-ordnerwechsel-einen-faden-ohne-obergrenze) war schwerer, weil dort eine Zeigerbewegung genügte. Hier braucht jeder Faden einen Befehl.

`HowTo.md` beschreibt die Wartezeit nur für den Zusatz „(fehlt)“ (`:1517` bis `:1521`), nicht für diese Befehle.

**Richtung.** Der Nutzer entscheidet, ob eine Statuszeile nach kurzer Frist („wartet auf …“) gebaut wird oder ob Stille genügt. Ich schlage die Statuszeile vor. In beiden Fällen gehört ein Absatz in `HowTo.md`.

**Abnahme.** Entweder eine reine Probe am Fristwert samt Lauf an einer getrennten Freigabe durch den Nutzer, oder ein Entscheid, dass Stille gilt, und dazu der Absatz in `HowTo.md`.

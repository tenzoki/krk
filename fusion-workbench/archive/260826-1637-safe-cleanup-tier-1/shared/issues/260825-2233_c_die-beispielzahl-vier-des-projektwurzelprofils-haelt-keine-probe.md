Die Beispielzahl „vier" des Projektwurzelprofils hält keine Probe

---

`resources/default-readers.toml:236-239` gibt der Leselaufregel zwei Beispielzahlen: „Das
Wurzelprofil einer fusion-Werkbank unten kostet deshalb drei Läufe, das Projektwurzelprofil
mit denselben sieben Zeilen vier." Beide stimmen, nachgemessen. Die Drei hält die Probe
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` in
`crates/krk-core/tests/leseprofil.rs` **genau** (`assert_eq!(…, (3, 5))`), die Vier hält
nichts: kein Prüfordner jener Probe hat die Gestalt einer Projektwurzel, und der Name
`Projektwurzel` kommt in der Probendatei nicht vor.

---

**Filed by:** ontorev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `resources/default-readers.toml:227-239`;
`crates/krk-core/tests/leseprofil.rs`, Probe
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` (Doc-Kommentar: „Die
Zahlen stehen hier genau und nicht als ‚unter der Grenze'");
`shared/issues/260825-2126_c_die-leselaufregel-der-datei-zaehlt-den-erkennungslauf-nicht-mit.md`;
`CLAUDE.md`, Absatz „Mehrere Aufzählungen sind seit der Runde 1 gewachsen" (die Gewohnheit,
eine Zahl entweder von einer Probe halten zu lassen oder sie durch den Zählweg zu ersetzen)

## Was gemessen ist

Gemessen am 260825-2233, Baum `1ac5dde`, über `leseprofil::zusammenfassen_gezaehlt`:

| Ort | Profil | Leseläufe | Öffnungen |
|---|---|---|---|
| `krk/fusion-workbench` | die Wurzel | 3 | 4 |
| `krk` | Projektwurzel mit fusion-Werkbank | 4 | 4 |

Die Vier ist die Drei plus der Erkennungslauf, wie der Satz `:238-239` es herleitet: jede
der sieben Zeilen trägt eine Ortsangabe, den erkannten Ordner liest allein die Erkennung.

`cargo test -p krk-core --test leseprofil`: 47 Proben grün. Darunter hält
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` die Wurzel auf genau
(3, 5) und den gemeinsamen Speicher auf genau (10, 0) — mit der Begründung im Doc-Kommentar,
dass eine Probe, die nur „unter der Grenze" prüft, den Schritt von vier auf sieben nicht
meldete. Für das Projektwurzelprofil gibt es keinen Prüfordner.

## Warum das zählt

Die Vier ist die eine Zahl in der Datei, die den neuen Halbsatz der Leselaufregel („plus
einen Lauf für die Erkennung") belegt; die Drei belegt die alte Hälfte. Wer den
Erkennungslauf je aus dem Haushalt herausnähme oder das Projektwurzelprofil um eine Zeile
ohne `ordner` ergänzte, machte den Satz falsch, und keine Probe würde rot. Die Datei wird
beim ersten Start wörtlich in das Heimatverzeichnis kopiert und danach nicht mehr
angefasst — was hier falsch wird, wird bei jedem Nutzer falsch.

`CLAUDE.md` hat für genau diese Lage eine Gewohnheit: eine Zahl steht in der Prosa, wenn eine
Probe sie hält („welche vier durchkommen, zählt die Probe …, und eine fünfte Zulassung lässt
sie rot werden"), und sonst steht statt ihrer der Weg, sie zu zählen. Die Drei erfüllt das,
die Vier nicht.

Schwere **niedrig**: die Zahl stimmt heute, und der Satz daneben sagt, wie sie zustande
kommt, sodass ein Nutzer sie nachrechnen kann.

## Möglichkeiten

1. Die Probe um einen vierten Prüfordner in der Gestalt einer Projektwurzel erweitern und
   `(4, 5)` genau halten — derselbe Zuschnitt wie bei den drei vorhandenen. Arbeit für
   `coder`, nicht für `ontocoder`; die Profildatei bleibt, wie sie ist.
2. Die zwei Zahlen aus `:236-239` streichen und die Herleitung stehen lassen („dort trägt
   jede Zeile eine Ortsangabe, und den erkannten Ordner liest allein die Erkennung"). Dann
   hält nichts eine Zahl, weil keine dasteht.

Die erste Möglichkeit ist die, die die Probe ohnehin für die zwei anderen Profile gewählt
hat, und sie hält zugleich die Aussage `:231-232` „dieser Lauf ist einer der zwölf" an
einem Profil, bei dem er sichtbar dazukommt.
---
Resolved: Möglichkeit 1, und die Arbeit lag bei `coder`: `crates/krk-core/tests/leseprofil.rs` hält
mit dem vierten Prüfordner der Probe zu C6.7 jetzt `(4, 5)` genau und daneben `orte.len() + 1`.
`resources/default-readers.toml` ist an dieser Stelle nicht angefasst. Die Herleitung dort
(`:237-240`) ist gegen die neue Probe gehalten und stimmt: die Drei des Wurzelprofils wandert über
„mit denselben sieben Zeilen" mit, und die zwei Bedingungen des Erkennungslaufs stehen ausgeschrieben
(„dort trägt jede Zeile eine Ortsangabe, und den erkannten Ordner liest allein die Erkennung"). Die
Probe misst dieselben drei Orte (`fusion-workbench`, `fusion-workbench/circles`,
`fusion-workbench/shared/issues`) und prüft eigens, dass keiner der erkannte Ordner selbst ist.
Nachgemessen am 260826-0124 über `zusammenfassen_gezaehlt`: `krk` 4 Leseläufe, `krk/fusion-workbench`
3. Die zwei Zahlen in der Datei bleiben stehen, weil eine Probe sie jetzt hält.

Nachtrag vom 260826-0210, `coder`, nach dem Landen der Probe: **die Vier hält, die Fünf daneben
steht an einer anderen Gestalt als die Kostenmessung.** Der Prüfordner der Probe trägt den vollen
Bestand einer eingerichteten Werkbank und zählt fünf Öffnungen; die Kostenmessung vom 260825-2107
zählt an `krk` vier, weil dort `.active-circle` fehlt und eine Zeile, die ihre Datei nicht findet,
nichts öffnet. Nachgemessen: derselbe Prüfordner ohne diese eine Datei liefert `(4, 4)`. Die
Leseläufe sind in beiden Gestalten vier, und um sie geht dieser Datensatz. Ein leeres
`fusion-workbench` liefert dagegen `(2, 0)`: ein Ort, den es nicht gibt, wird nicht gelesen — die
Vier steht nur an einer eingerichteten Werkbank, und genau die baut der Prüfordner. Beide
Nebenmessungen stehen im Doc-Kommentar der Probe.

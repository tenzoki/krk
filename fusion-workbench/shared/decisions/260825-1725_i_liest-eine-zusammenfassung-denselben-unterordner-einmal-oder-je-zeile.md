# Liest eine Zusammenfassung denselben Unterordner einmal oder je Zeile?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: ein Ort wird je Zusammenfassung hoechstens einmal gelesen. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `crates/krk-core/src/leseprofil/bausteine.rs` (Modulkopf, `Lauf::stand`, `Lauf::am_ort`); `crates/krk-core/tests/leseprofil.rs:1588` (`ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen`); `resources/default-readers.toml` (Abschnitt „Was eine Zusammenfassung höchstens kostet")

---

## Question

Der Nutzer verlangt am 260825 für `fusion-workbench/shared` je Unterordner zwei Auskünfte, die
Zahl der Einträge und das Datum des jüngsten. Der Ordner führt zehn Unterordner; das sind
zwanzig Zeilen auf zehn Orten.

Heute kostet jede Zeile mit `ordner`-Angabe genau einen Leselauf, auch dann, wenn die Zeile
darüber denselben Ordner schon gelesen hat. Der Modulkopf von `leseprofil::bausteine` legt das
ausdrücklich fest: „Ein Unterordner wird dagegen nicht gemerkt", damit „die Zahl der Läufe aus
dem Profil ablesbar bleibt, statt vom Inhalt eines Zwischenspeichers abzuhängen". Zwanzig
Zeilen kosten damit zwanzig Läufe, und `HOECHSTENS_LESELAEUFE` steht auf zwölf: die letzten
acht Zeilen zeigten ihren Platzhalter.

Die Festlegung der Runde 16 ist keine Nachlässigkeit, sondern eine begründete Wahl. Sie jetzt
umzudrehen, ist eine Entscheidung und keine Umsetzungsdetailfrage — sie ändert eine Zusage,
eine Probe und den Kommentartext der Auslieferungsfassung.

## Options

1. **Ein Ort wird je Zusammenfassung höchstens einmal gelesen.** `Lauf` merkt seine Lesungen
   nach aufgelöstem Pfad, so wie er heute schon den erkannten Ordner in `Lauf::stand` merkt.
   Der Haushalt zählt dann **verschiedene Orte** statt Zeilen mit Ortsangabe.
   - Pros: `shared` kostet zehn Läufe statt zwanzig und passt unter die Zwölf, ohne dass die
     Zusage steigt. Die Zahl der Läufe bleibt aus dem Profil ablesbar — als Zahl der
     **verschiedenen** genannten Orte, und das ist die genauere Größe: sie zählt keine Arbeit
     doppelt, die nur einmal geschieht. Die Asymmetrie zwischen dem erkannten Ordner (gemerkt)
     und einem Unterordner (nicht gemerkt) fällt weg; heute muss ein Leser sie kennen, um die
     Zahlen eines Profils vorherzusagen. Der Zwischenspeicher lebt genau so lange wie ein
     `Lauf`, also für eine Zusammenfassung, und hält damit keinen Stand über die Zeit.
   - Cons: Die Zusage C6.1 („ein Baustein kostet höchstens einen Leselauf") bleibt wahr, ihre
     Umkehrung wird falsch: aus der Zahl der Bausteine folgt die Zahl der Läufe nicht mehr.
     Die Probe `ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen`
     erwartet in ihrem letzten Fall ausdrücklich `3` für „zwei Bausteine auf demselben
     Unterordner lesen ihn zweimal"; dieser Fall dreht sich um. Der Speicher wächst um die
     Einträge aller gelesenen Orte statt nur des erkannten; bei zwölf Orten zu je höchstens
     2.000 Einträgen ist das eine benennbare, keine offene Schranke.

2. **Die Zahl `HOECHSTENS_LESELAEUFE` steigt von zwölf auf zwanzig oder mehr.**
   - Pros: Kein Eingriff in die Bauart, keine Probe dreht sich um.
   - Cons: Sie löst nichts, sie verschiebt es. Zwanzig Zeilen auf zehn Orten lesen zwanzigmal,
     wo zehn Lesungen genügen; die doppelte Arbeit bleibt und wird nur erlaubt. Ein elfter
     Speicher unter `shared/` bräuchte die nächste Erhöhung. Und die Zahl schützt vor Arbeit
     auf der Platte, nicht vor einer Zahl: sie zu erhöhen, ohne die doppelte Arbeit zu
     beseitigen, gibt die Zusage auf, statt sie zu halten.

3. **Weniger Zeilen zeigen.** Nur die Zahl je Unterordner, kein Datum, oder nur für einen Teil
   der Unterordner.
   - Pros: Nichts am Kern zu ändern.
   - Cons: Beantwortet den Auftrag nicht. Der Nutzer hat beide Angaben für **jeden**
     Unterordner verlangt.

4. **Beide Zeilen eines Unterordners zu einer zusammenziehen**, etwa „12 Datensätze, zuletzt am
   …" aus einem Baustein.
   - Pros: Zehn Zeilen, zehn Läufe, ohne Merkmerkmal.
   - Cons: Ein Baustein, der zwei Werte liefert, sprengt den Zuschnitt „eine Zeile, eine
     Beschriftung, ein Wert", an dem `Zusammenfassungszeile` und `Wert::als_text` hängen. Es
     wäre ein fünfter Baustein mit zwei Antworten, und das ist der schlechtere Preis für
     dasselbe Ziel.

## Constraints

- Die Zusammenfassung muss innerhalb einer benennbaren Schranke bleiben; die Zahlen dürfen
  nicht vom Bestand des Ordners abhängen, ohne dass gesagt ist, wie.
- Die Zusage C6.4 („wird eine Grenze erreicht, bricht nichts ab, die übrigen Zeilen zeigen
  ihren Platzhalter") bleibt unberührt.
- Der Zwischenspeicher darf nicht über eine Zusammenfassung hinaus leben. Zwei
  Zusammenfassungen desselben Ordners nacheinander müssen zweimal lesen, sonst zeigte die
  Vorschau einen Stand von vorhin.

## Recommendation

**Möglichkeit 1.** Sie ist die einzige, die den Auftrag erfüllt, ohne eine Zusage aufzugeben,
und sie macht die Bauart einfacher statt komplizierter: der Sonderfall „der erkannte Ordner
wird gemerkt, ein Unterordner nicht" verschwindet, und an seine Stelle tritt eine Regel ohne
Ausnahme — **ein Ort wird je Zusammenfassung höchstens einmal gelesen.**

Der Einwand der Runde 16 bleibt beantwortbar und wird nicht übergangen. Die Zahl der Läufe ist
weiter aus dem Profil abzulesen; man zählt jetzt die **verschiedenen** Ortsangaben statt der
Zeilen. Für die Auslieferungsfassung heißt das: das Profil `fusion-Werkbank: ein Speicher`
kostet unverändert einen Lauf, das Circle-Profil unverändert fünf (die zweite Zeile auf
`planning` ist die einzige Wiederholung darin und fiele weg, also **vier**), und das neue
`shared`-Profil zehn statt zwanzig.

Drei Stellen ziehen mit, und keine davon ist bloß Kosmetik:

- Der Modulkopf von `leseprofil::bausteine`, Abschnitt „Der erkannte Ordner wird höchstens
  einmal gelesen": er begründet heute die Asymmetrie, die entfällt, und muss die neue Regel
  samt ihrem Grund tragen.
- Die Probe `ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen`:
  ihr elfter Fall heißt künftig „zwei Bausteine auf demselben Unterordner teilen sich eine
  Lesung" und erwartet `2`. Die Probe bleibt die Stelle, an der die Zusage abgezählt wird, und
  wechselt nur ihre Aussage.
- Der Kommentarblock „Was eine Zusammenfassung höchstens kostet" in
  `resources/default-readers.toml`: dort steht heute „Ein Baustein mit `ordner` kostet genau
  einen Leselauf". Das ist der Satz, den der Nutzer beim Schreiben seines Profils liest, und
  er wird mit dieser Änderung falsch.

---
Implemented: f097e0e — `Lauf` merkt seine Lesungen nach aufgelöstem Pfad statt nur den erkannten Ordner (`crates/krk-core/src/leseprofil/bausteine.rs:361`, `:377`), und `Lauf::am_ort` (`:541`) fragt den Merker, statt bei jeder Ortsangabe unbesehen zu lesen. Der Modulkopf trägt die Regel ohne Ausnahme (`:86`, `:89`). Nachgemessen am 260826-0149 gegen den Baum, `make check` grün.

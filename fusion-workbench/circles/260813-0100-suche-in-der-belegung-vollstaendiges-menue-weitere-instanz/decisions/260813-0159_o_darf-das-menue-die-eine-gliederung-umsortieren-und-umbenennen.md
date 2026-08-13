# Darf das Menü die eine Gliederung umsortieren und einen Bereich umbenennen?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` (C2.2, C2.3, C2.13, Randbedingung zum Menü „Bearbeiten"), `shared/decisions/260813-0053_o_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`, `crates/krk-ui/src/belegungsmodell.rs:104-131`, `crates/krk-ui/src/appkit/menue.rs:254-274`

---

## Frage

C2.3 des Spec verlangt zwei Dinge, die die heutige Gliederung nicht zugleich erfüllt: die
Obermenüs sollen „in der Reihenfolge von `Funktionsbereich::ALLE`" stehen, und zugleich soll
„Anwendung" vorn und „Fenster" hinten liegen. `Funktionsbereich::ALLE`
(`belegungsmodell.rs:104-114`) führt am 260813 diese Reihenfolge:

```
1 Dateilisting   2 Dateioperationen   3 Tabs        4 Vorschau   5 Leiste und Fokus
6 Fenster        7 Anwendung          8 Textbefehle 9 Editor
```

„Anwendung" steht an siebter, „Fenster" an sechster Stelle. Beides ist für eine Menüleiste
falsch, und die erste Stelle ist nicht verhandelbar: macOS ersetzt den Titel des **ersten**
Obermenüs durch den Namen aus der `Info.plist`, also muss dort der Anwendungsbereich stehen.

Eine zweite Stelle hängt daran. Die Randbedingung des Datensatzes zu den neun Obermenüs sagt,
es müsse weiterhin ein Obermenü namens „Bearbeiten" geben, weil macOS Textbefehle und
Systemzusätze an ein Menü dieses Namens hängt und `menue::systemzusaetze_unterdruecken`
(`menue.rs:254-274`) genau dort ansetzt. Der zugehörige Bereich heißt heute
`Funktionsbereich::Textbefehle` und trägt den Anzeigenamen „Textbefehle"
(`belegungsmodell.rs:127`).

Beide Punkte betreffen dieselbe Sache: darf diese Runde die eine Gliederung anfassen, damit
das Menü sie unverändert benutzen kann, oder bekommt das Menü eine eigene Ordnung und eine
eigene Namenstabelle?

## Möglichkeiten

1. **Die eine Gliederung wird angepasst; das Menü benutzt sie unverändert.**
   `Funktionsbereich::ALLE` bekommt „Anwendung" an die erste und „Fenster" an die letzte
   Stelle, und `Funktionsbereich::Textbefehle::name()` liefert „Bearbeiten" statt
   „Textbefehle". Die neue Reihenfolge lautet: Anwendung, Dateilisting, Dateioperationen,
   Tabs, Vorschau, Leiste und Fokus, Editor, Bearbeiten, Fenster.
   - Dafür: Es bleibt bei einer Gliederung mit drei Abnehmern, und der Doc-Kommentar von
     `nach_bereichen` behält seine Zusage. Der Name „Bearbeiten" ist für den Abschnitt in der
     Belegungsansicht sogar genauer als „Textbefehle": die sechs Funktionen tragen alle
     `gehalten_von = "menue"` und sind genau die Einträge jenes Menüs.
   - Dagegen: Belegungsansicht und Markdown-Ausgabe zeigen ihre Abschnitte danach in einer
     anderen Reihenfolge und einen davon unter einem anderen Namen. Der Spec dieser Runde
     nennt diese Änderung nicht, und die Markdown-Ausgabe ist das Ergebnis der Runde 3.
2. **Das Menü bekommt seine eigene Reihenfolge und seine eigene Namenstabelle.** Eine Liste im
   Menümodell sagt, in welcher Folge die neun Bereiche in der Leiste stehen und wie ihr
   Obermenü heißt; `Funktionsbereich::ALLE` bleibt, wie es ist.
   - Dafür: Belegungsansicht und Markdown-Ausgabe bleiben unverändert.
   - Dagegen: Zwei Ordnungen über dieselben neun Bereiche, von Hand gepflegt und ohne Zwang,
     vollständig zu bleiben. Genau das schließt der Doc-Kommentar von `nach_bereichen`
     (`belegungsmodell.rs:540-544`) aus, und C2.2 verlangt das Gegenteil.
3. **Die Reihenfolge wird angepasst, der Name nicht.** `ALLE` wird umsortiert, das Obermenü
   heißt weiter „Textbefehle".
   - Dafür: Die kleinste Änderung an den zwei anderen Oberflächen.
   - Dagegen: Die Randbedingung zum Menü „Bearbeiten" ist gebrochen. Ob macOS seine Zusätze
     an ein anders benanntes Menü hängt, ist am Baum nicht belegt; C2.13 verlangt, dass weder
     „Emoji & Symbols" noch „Start Dictation…" noch „AutoFill" erscheinen, und diese Zusage
     hinge dann an einer ungeprüften Annahme.

## Randbedingungen

- Der Titel des ersten Obermenüs ist nicht wählbar. macOS ersetzt ihn durch den Namen aus der
  `Info.plist`; der Anzeigename von `Funktionsbereich::Anwendung` wirkt dort also gar nicht,
  wohl aber in der Belegungsansicht und in der Markdown-Ausgabe.
- `nach_bereichen` läuft über `Funktionsbereich::ALLE` und übernimmt jede Änderung an dieser
  Liste an alle Abnehmer zugleich. Eine Umsortierung kostet keine zweite Zeile Code.
- `systemzusaetze_unterdruecken` setzt heute über `NSUserDefaults` an und nicht über den
  Menütitel. Ob die Unterdrückung ohne ein Menü namens „Bearbeiten" trägt, ist ungemessen;
  der Kommentar an `starten` (`anwendung.rs:5293-5296`) nennt das Menü ausdrücklich beim
  Namen.

## Empfehlung

Möglichkeit 1. Die Zusage „eine Gliederung, drei Abnehmer" ist der Grund, aus dem der Spec das
Menü überhaupt aus `nach_bereichen` bauen lässt; eine zweite Ordnung daneben nähme der Runde
ihren eigentlichen Gewinn. Die Kosten fallen an zwei Anzeigen an, die dieselben neun
Abschnitte danach in einer Reihenfolge zeigen, die dem Menü folgt, und das ist für einen
Nutzer, der zwischen beiden Oberflächen hin und her sieht, eher hilfreich als störend.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:

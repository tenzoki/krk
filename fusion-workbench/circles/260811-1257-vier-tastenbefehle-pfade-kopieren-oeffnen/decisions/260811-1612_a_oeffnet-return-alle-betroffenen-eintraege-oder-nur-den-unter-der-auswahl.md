# Öffnet `return` alle betroffenen Einträge oder nur den unter der Auswahl?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator (nach einem Befund des shaper beim Schreiben des Specs)
**Cross-references:** `260811-1258_a_was-kopiert-der-pfadkopierer-bei-stehender-markierung.md`,
`planning/260811-1552_*_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` (C3),
`crates/krk-ui/src/kommandos/operationen.rs:150-157` (`betroffene()`)

---

## Frage

Der Spec belegt in C3 vor, dass `return` **alle betroffenen** Einträge an das Standardprogramm
gibt und nicht nur den unter der Auswahl. Das folgt aus der Antwort zum Pfadkopierer, der
`betroffene()` erbt, und aus dem, was Finder und ForkLift tun.

**Entschieden war es nirgends.** Der `shaper` hat das beim Schreiben des Specs bemerkt und die
Vorbelegung als solche ausgewiesen, statt sie als Zusage zu führen. Die Frage ist erheblich, weil
sie den Unterschied zwischen einem harmlosen und einem folgenreichen Tastendruck ausmacht: fünf
markierte Dateien öffnen fünf Programme auf einmal.

## Optionen

1. **Alle betroffenen.** `return` gibt jeden Eintrag aus `betroffene()` an das System.
   - Pro: **eine** Regel im ganzen Haus. `betroffene()` gilt dann für die vier Dateioperationen,
     den Pfadkopierer und den Öffner gleichermaßen; ein Nutzer, der die Regel einmal versteht,
     versteht sie überall. Entspricht Finder und ForkLift.
   - Contra: ein Tastendruck kann viele Programme starten. Rückgängig gibt es dafür nicht.
2. **Nur den unter der Auswahl.**
   - Pro: vorsichtiger. Kein versehentliches Öffnen von zwanzig Dateien.
   - Contra: eine **zweite** Regel neben `betroffene()`, und zwar für einen Befehl, der direkt
     neben `shift+cmd+c` liegt und anders zählte als dieses. Genau die Sorte Ausnahme, die dieses
     Projekt an anderen Stellen ausdrücklich vermeidet.

## Constraints

Die Directive verlangt, dass die vier Befehle über die vorhandene Maschinerie laufen und über
keine zweite daneben. Eine eigene Zählregel für `return` wäre eine zweite.

---
Answered: **Möglichkeit 1, alle betroffenen.** Nutzerantwort am 260811-1610.

`betroffene()` gilt damit ohne Ausnahme: vier Dateioperationen, Pfadkopierer, Öffner. Der
Öffner ist ihr sechster Abnehmer — der `conceptrev`-Lauf vom 260811-1604 hat dazu einen Zählfehler
im Spec gefunden, der ihn als fünften führt, und er ist mit der Diagrammnachbesserung
zu berichtigen.

**Was mit dieser Antwort nicht entschieden ist:** ob KRK bei einer großen Zahl betroffener
Einträge nachfragt, bevor es öffnet. Der Spec sagt dazu heute nichts, und diese Antwort verlangt
es nicht. Wer es will, führt es als eigenen Vorschlag; eine Schwelle wäre eine Zahl, die
irgendwer setzen müsste, und dieses Projekt hat mit gesetzten Zahlen ohne Messung schlechte
Erfahrungen gemacht.

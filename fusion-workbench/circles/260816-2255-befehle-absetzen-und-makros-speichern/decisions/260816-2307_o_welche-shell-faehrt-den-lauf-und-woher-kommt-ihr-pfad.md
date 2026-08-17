# Welche Shell fährt den Befehlslauf, und woher kommt ihr `PATH`?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md` (C1.2, C1.16, C1.17 und `## Offen für den Planner`, dritter Punkt); `shared/consult/260815-1354-befehlslauf-und-makros-in-krk.md` (`## Open Questions`, zweite Frage); `circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_o_plan-befehle-absetzen-und-makros-speichern.md` Schritt A3 und A4

---

## Question

Der Spec übergibt die Wahl der Shell dem Planner und bindet ihn dabei an zwei Kriterien: C1.2 verlangt Namensausdehnung, Röhren und Verkettungen, C1.16 einen `PATH`, der die Anmeldeshell des Nutzers wiedergibt. Beide Kriterien fragen nach verschiedenen Dingen, und die Frage ist, ob eine Shell beide beantwortet oder zwei je eine.

Die Festlegung des Nutzers vom 260816 schließt einen Weg schon aus: der `PATH` wird **einmal beim Start** erfragt und nicht bei jedem Lauf. Damit kann der Lauf selbst keine Anmeldeshell fahren, denn eine Anmeldeshell lädt bei jedem Aufruf das Profil des Nutzers, und genau diese Kosten hat die Festlegung vermieden.

Die Frage bindet über diesen Plan hinaus: sie entscheidet, in welcher Sprache der Nutzer jedes Makro schreibt, das er je anlegt, und eine spätere Umstellung entwertete seine Makrodatei.

## Options

1. **`/bin/sh -c` für den Lauf, die Anmeldeshell allein für die `PATH`-Abfrage** — zwei Shells, zwei Fragen. Die Anmeldeshell beantwortet einmal beim Start „welche Umgebung gibt mir die Anmeldung", `/bin/sh` führt danach jede Befehlszeile aus.
   - Pro: `/bin/sh` liest bei `-c` keine Startdatei, kostet also nichts je Lauf. Das Verhalten ist auf jedem Gerät dasselbe, gleich welche Shell der Nutzer eingestellt hat. Es ist der Weg, den `system(3)`, `make` und jedes andere Werkzeug nimmt, und C1.2 hält damit unverändert.
   - Contra: Ein Makro in zsh-Schreibweise läuft nicht. Auf dem Bauziel ist `/bin/sh` die Bash 3.2 im POSIX-Modus, also weder zsh noch eine junge Bash; `**/*.rs` und `[[ … ]]` fehlen.
2. **Die Anmeldeshell auch für den Lauf, ohne Anmeldung (`$SHELL -c`)** — eine Shell, zwei Aufrufarten.
   - Pro: Der Nutzer schreibt seine Makros in der Sprache, die er ohnehin tippt. Ein Name in der Aufstellung statt zweier.
   - Contra: zsh liest bei jedem `-c` die Datei `~/.zshenv`, Bash die Datei aus `$BASH_ENV`. Damit hängt jeder Lauf an einer Datei, die KRK nicht kennt, und sie kann den beim Start erfragten `PATH` still überschreiben. Das Verhalten wechselt zudem mit der eingestellten Shell, und eine Makrodatei ist dann nicht mehr zwischen zwei Geräten austauschbar.
3. **Die Anmeldeshell mit Anmeldung bei jedem Lauf (`$SHELL -l -c`)** — eine Shell, ein Aufruf, kein `PATH`-Sonderweg.
   - Pro: C1.16 und C1.17 entfielen ersatzlos.
   - Contra: Der Nutzer hat am 260816 ausdrücklich die einmalige Abfrage beim Start gewählt. Das Laden des Profils kostet je Lauf mehrere hundert Millisekunden bei einer gewachsenen zsh-Einrichtung. Möglichkeit 3 ist damit durch die bestehende Festlegung ausgeschlossen und steht hier nur zur Vollständigkeit.

## Constraints

- Die elf Festlegungen des Nutzers vom 260816 stehen nicht zur Disposition; die einmalige `PATH`-Abfrage beim Start ist eine davon.
- C1.2 muss halten: `ls *.rs | wc -l` liefert eine Zahl, `false && echo nein || echo ja` liefert `ja`. Beide Möglichkeiten halten das.
- C1.16 muss halten: ein Werkzeug aus `~/.local/bin` wird gefunden. Beide Möglichkeiten halten das, weil der erfragte `PATH` in die Umgebung des Laufs geht.

## Recommendation

**Möglichkeit 1.** Die beiden Kriterien stellen zwei verschiedene Fragen, und jede bekommt die Antwort, die nur sie geben kann: die Umgebung der Anmeldung kennt allein die Anmeldeshell, das Ausführen einer Befehlszeile kann jede POSIX-Shell. Zwei Shells sind hier keine zweite Wahrheit über eine Frage, sondern je eine Antwort auf zwei.

Ausschlaggebend ist die Vorhersagbarkeit. Eine Makrodatei wird von Hand gepflegt und einmal geschrieben; sie soll auf jedem Gerät dasselbe tun. Unter Möglichkeit 2 hinge jeder Lauf an `~/.zshenv`, einer Datei, die KRK weder liest noch nennt, und der beim Start erfragte `PATH` wäre dort still überschreibbar. Der Preis von Möglichkeit 1 ist benannt und nicht kleingeredet: wer zsh-Schreibweise in ein Makro schreibt, bekommt einen Fehler statt eines Laufs.

Der Plan setzt diese Empfehlung um und schließt den Datensatz nicht. Wer sie umdreht, ändert in Schritt A3 eine Zeile: den Programmnamen und seine Argumente.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

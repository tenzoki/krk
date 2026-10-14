# Der Heimordner bekommt appointments.md als weitere Datei

---
**Status:** done
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260926-2250
**Mode:** autonomous
**Active spec/plan:** 260926-2253_*_spec-termine-als-weitere-datei-im-heimordner.md (Spec), 260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"der home bereich bekommt eine weitere datei: appointments.md, gleicher aufbau und bedienung wie notizen, aber spalte links ist YYMMDD + optional HH:MM." Sortierung nach datumsspalte (CMD+1 toggelt Richtung). Zeile mit aktuellem Tag wird hervorgehoben.

## Abschluss

Geschlossen am 260927-0123 als `done`, Commits `e6fdf7e..aedad1a` (Spec, Plan und Entscheid in `e6fdf7e`; Schritte 1 bis 11 in `8900c30`, `830f401`, `fd967b7`, `7593895`, `572630b`, `5f13420`, `ea62aaa`, `d350c29`, `1dda481`, `52e351f`, `f6981bf`; die Befunde der Durchsicht `260927-0109-reviewer-termine-als-weitere-datei-im-heimordner.md` behoben in `314e149`, `021fa8b`, `26b6ad1`, `aedad1a`). **Gebaut, nicht abgenommen:** der Abnahmelauf mit KRK im Vordergrund ist Nutzerarbeit.

Die Haltebedingungen aus `## Where this work stops` des Plans `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md` sind unter **Mode:** autonomous nicht gestellt worden und stehen hier wörtlich:

1. Jeder der elf Schritte trägt `[DONE]`, und jede behauptete Erledigung ist gegen den Baum gelesen.
2. `make check` endet am HEAD nach Schritt 11 grün.
3. Jedes Kriterium „am Baum nachweisbar“ aus T1 bis T8 nennt in der Closes-Zeile seines Schritts eine Probe oder eine Textstelle, die es hält.
4. Der Entscheid `260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md` ist vor dem Schließen des Arbeitspakets beantwortet und nach Schritt 8 als umgesetzt vermerkt.
5. Haltestelle 1 des Spec, verlorener Konflikt der Auslieferung oder der eigenen Belegung: (condition did not arise: beide luden am 260926-2300 ohne Konflikt, und ihre einzigen doppelten Kombinationen sind am Zusteller getrennt). Schritt 3 prüft es beim Bau ein zweites Mal und stoppt, wenn es sich geändert hat.
6. Haltestelle 2 des Spec, eine andere Antwort an einer Stelle, die nach `secrets.txt` fragt: Schritt 2 stoppt vor dem Commit, wenn sie eintritt.
7. Die Arbeit fügt dem Tastendruckweg keine zweite Erhebung der Lage hinzu, und der Lauf von `Belegung::nachschlag` bleibt innerhalb des vollen Durchgangs, den ein Anschlag ohne Funktion heute schon fährt. Trifft eines davon nicht zu, stoppt der Schritt, und die Frage, ob gegen L1 ein Abnahmelauf geschuldet ist, geht als Entscheid an den Nutzer.
8. Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; kein Agent kann ihn fahren. Das Schließen des Arbeitspakets sagt „gebaut“ und nicht „abgenommen“, und die Kriterien „nur am laufenden Bündel prüfbar“ aus T1 bis T8 bleiben für den Nutzer offen.
9. Vor der Nutzerabnahme von T5 steht der Handgriff aus dem Spec: F1, `cmd+1` auf „Termine: Sortierrichtung umkehren“, Ansicht verlassen.
10. Eine Auslieferung gehört nicht zu diesem Plan. Sie geschieht allein auf ausdrücklichen Auftrag und nach `README.md`, `### Versionsstufen`.

Nicht am laufenden Bündel geprüft (aus der Durchsicht): die Menükürzel-Regel für `⌘1`, die Pfeilbilder im Spaltenkopf, die Lesbarkeit der gelben Hervorhebung in beiden Erscheinungsbildern.

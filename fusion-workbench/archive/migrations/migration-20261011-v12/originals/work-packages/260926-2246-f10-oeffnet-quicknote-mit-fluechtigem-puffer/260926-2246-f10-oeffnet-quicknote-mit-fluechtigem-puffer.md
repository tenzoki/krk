# F10 öffnet eine Quicknote mit flüchtigem Puffer

---
**Status:** done
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260927-0123
**Mode:** autonomous
**Active spec/plan:** 260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md (Spec), 260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"F10 öffnet Quicknote im Text Editor. Die Quicknote speichert in einem Internen Speicher (keine Persistenz). Paste des Zwischenpuffer muss möglich sein. Drei Buttons: Clear (Löscht den Puffer), Close (schließt das Fenster, Inhalt bleibt im Puffer) und Copy (Schließt den Editor, Inhalt wird in die Zwischenablage bewegt, Puffer ist leer)."

## Abschluss

Geschlossen am 260927-0237 als `done`, Commits `385f106..83ac446` (Übernahme und Plan in `385f106`; Schritte 1 bis 8 in `c659db7`, `d3824d9`, `92deee2`, `eb3a60d`, `5a5ba88`, `0924e9a`, `2ef2962`, `171b380`; die Befunde der Durchsicht `260927-0225-reviewer-quicknote-und-termine-nachzug.md` erledigt in `83ac446`, `cmd+z` bei leerem Stapel gemessen in `messungen/260927-0232-quicknote-rueckgaengig.txt`: die Datei darunter wird nicht erreicht). **Gebaut, nicht abgenommen:** der Abnahmelauf mit KRK im Vordergrund ist Nutzerarbeit; davor vergibt der Nutzer F10 und `shift+f10` in F1 mit „Zuweisen“.

Die Haltebedingungen aus `## Where this work stops` des Plans `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md` sind unter **Mode:** autonomous nicht gestellt worden und stehen hier wörtlich:

1. Die Arbeit ist fertig, wenn alle acht Schritte `[DONE]` tragen und `make check` auf dem letzten Commit grün endet.
2. **Ausgeliefert wird nicht vor Schritt 6.** Nach Schritt 1 steht F10 im Menü und tut nichts; nach den Schritten 2 bis 5 fehlen Wege hinaus, und eine offene Quicknote hinterließe beim Beenden die Sichtbarkeit des Editors in `session.toml`. Ein `./release.sh` vor dem Commit von Schritt 6 ist deshalb ausgeschlossen; ob danach ausgeliefert wird, entscheidet ein eigener Auftrag und nicht dieser Plan.
3. Der erste Stopp des Spec („die Quicknote kann nicht erscheinen, ohne den Stand einer offenen Datei anzutasten“) tritt nicht ein (condition did not arise: die Fläche der Datei wird mit `setHidden:` ausgeblendet, `Editormodell`, Schreibmarke und Rückgängigstapel des Fensters bleiben unberührt, und eine laufende Zelle endet über `zelle_uebernehmen` genau so, wie sie bei jedem Klick daneben endet; eine abgewiesene Zelle hält F10 an und lässt alles, wie es war).
4. Der Abnahmelauf am Bündel ist Nutzerarbeit und nicht Teil dieser Arbeit: er verlangt KRK im Vordergrund, und vor ihm vergibt der Nutzer F10 und `shift+f10` in der F1-Ansicht mit „Zuweisen“, nicht mit `cmd+r`. Zeigt er, dass F10 auf seiner Tastatur KRK nicht erreicht, hält die Abnahme von Q1 an, und der Nutzer wählt eine andere Taste; der Bau bleibt stehen (zweiter Stopp des Spec).
5. Ein Abnahmelauf gegen die zehn Zusagen aus C8 ist nicht geschuldet: der Start baut für die Quicknote nichts, und je Tastendruck kommt in `ist_eigene_textflaeche` ein Nämlichkeitsvergleich dazu, der ohne gebaute Quicknote an `OnceCell::get` endet.

Nicht am laufenden Bündel geprüft (aus der Durchsicht): ob F10 auf der Tastatur des Nutzers ankommt, die Grenzprüfung bei Mehrfachauswahl, ob ein Klick auf die Schaltflächen je ohne Fokus bleibt.

# Die Vorschau blättert Fotos eines Jahres- oder Monatsordners nach Aufnahmedatum

---
**Domain:** code
**Status:** done
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260929-1415
**Mode:** autonomous
**Active spec/plan:** 260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md (Spec), 260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"Allgemeines Reader Profile so erweitern, dass ein Baum dieser Struktur (Fotos/<jahr>/<monat>/<bilder>) ein bestimmtes Scrollverhalten in der Vorschau realisiert: Wenn man innerhalb eines Jahresordners ist, soll erst das erste Bild des ersten Monats angezeigt werden, im Monatsordner das erste Bild dieses Monats. Mit CMD-Pfeil-{hoch/runter} soll durch die Bilder navigiert werden. Enter springt in beiden Fällen in den Ordner zu dem Bild." Antworten auf die Klärungsfragen: der ausgewählte Ordner liefert die Bilder; cmd+Pfeil blättert, solange eine Bildfolge angezeigt wird; Enter springt dann zum Bild; Reihenfolge nach Aufnahmedatum, Lesegrenzen dafür erweitert, Obergrenze 7.500 Fotos. Erreicht ist das, wenn die Kriterien des Spec am Baum gehalten sind; ausgeliefert wird zusammen mit dem Zurücksetzen-Befehl als 2.2.0 (Auftrag des Nutzers).

## Abschluss

Geschlossen am 260929-1753 als `done`, Commits `c374be2..ed86461` (Plan `c374be2`, Klärungen und Entscheid zur Konfliktregel `7dacbc2`; Schritte 3 bis 14 in `a0e3e3d`, `2b94c93`, `f011f5b`, `0040151`, `edb7a19`, `5618322`, `75468b7`, `8f8e30d`, `49cba64`, `e9b3b5a`, `f72f2db`, `bea5cbd`; Durchsicht `260929-1646-reviewer-bildfolge-und-nachtraege.md` in `db72744`, der Befund hoher Schwere und zwei niedrige behoben in `13b1ad7`, die zwei Nutzerentscheide dazu in `ed86461`). `make check` endet auf `ed86461` grün; die fensterlose Messung `messungen/260929-1432-bildfolge-kopflos.txt` liegt bei p95 6,7 ms bis zur geordneten ersten Gruppe. **Gebaut, nicht abgenommen:** die Messung am Bündel (`make bildfolge`) und der Abnahmelauf verlangen KRK im Vordergrund und sind Nutzerarbeit. Der Plan sagt in Entscheidung 5, versteckte Einträge zählten mit; der Nutzer hat am 260929 das Gegenteil entschieden, maßgeblich ist Spec C2.1.

Die Haltebedingungen aus `## Where this work stops` des Plans `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md` sind unter **Mode:** autonomous nicht gestellt worden und stehen hier wörtlich:

1. Die Arbeit ist gebaut, wenn alle vierzehn Schritte `[DONE]` tragen und `make check` auf dem Commit des letzten Schritts grün endet.
2. Sagt der Bericht aus Schritt 1 `Urteil: Stop`, beginnt kein Schritt ab 3, und der Nutzer entscheidet, ob C2 auf das Änderungsdatum zurückfällt oder die Zusage aufgeweicht wird (Haltepunkt 1).
3. Nennt der Bericht aus Schritt 1 für ein Format „Stop für dieses Format“, läuft die Arbeit für die übrigen Formate weiter, jenes Format ordnet nach dem Änderungsdatum, und die Auslieferung wartet, bis der Nutzer für dieses Format entschieden hat (Haltepunkt 2).
4. Sagt der Bericht aus Schritt 2 `Urteil: Stop`, beginnt kein Schritt ab 3, und der Nutzer entscheidet, ob eine zweite Regelstelle hinnehmbar ist oder das Blättern auf Umschalt+Cmd+Pfeil hoch/runter wechselt (Haltepunkt 4).
5. Der Messlauf im Bündel (`make bildfolge`) ist Nutzerarbeit, weil er KRK im Vordergrund verlangt; kein Agent fährt ihn. Fährt der Nutzer ihn vor der Auslieferung und vergehen am Referenzgerät mehr als zwei Sekunden bis zum ersten Foto, wartet die Auslieferung auf seine Entscheidung über Grenze oder Vorgehen (Haltepunkt 3). Fährt er ihn nicht, verlässt die Arbeit den Baum als gebaut und nicht als abgenommen; die Auslieferung hängt daran nicht, denn der Spec verlangt es nicht.
6. Ausgeliefert wird als 2.2.0, wie der Nutzer ausdrücklich beauftragt hat, zusammen mit dem gebauten Befehl „Auf Werkseinstellungen zurücksetzen…“ (Commit `9926c78` ist Vorfahre von HEAD). Vorbedingungen, jede für sich prüfbar: Schritt 14 trägt `[DONE]`; eine Durchsicht über den Bereich von `9926c78` bis HEAD ist gefahren, und jeder ihrer Befunde ist behoben oder vom Nutzer zurückgestellt; der ausführende Agent nennt die Zahl 2.2.0 vor dem Lauf (`260918-0842_*_wer-vergibt-die-versionszahl-und-darf-ein-agent-den-auslieferungslauf-fahren.md`); die Haltepunkte oben stehen nicht offen.
7. Ein Abnahmelauf gegen die Zeitzusagen ist nicht geschuldet, solange zwei Proben halten: für einen Eintrag ohne Profil mit Bildfolge verbraucht die Vorschau denselben Haushalt und keinen Leselauf der Bildfolge (Schritt 6), und kein Schritt fügt dem Start einen Systemaufruf hinzu (die Auslieferungsfassung wird wie bisher einmal geparst). Dann ist L7 für jeden Eintrag außerhalb einer Bildfolge unberührt und L4 nicht berührt. Wird eine der zwei Proben rot oder fügt ein Schritt dem Start Arbeit hinzu, hält dieser Schritt an, und der Abnahmelauf gegen L7 oder L4 wird geschuldet. L4 und L7 bleiben unabhängig davon auf der Liste der späteren Messrunde, auf der sie schon stehen; eine elfte Zusage setzt diese Arbeit nicht.
8. Kein Schritt lässt KRK `readers.toml` aus eigenem Antrieb schreiben.
9. Das Arbeitspaket bleibt `claimed`, bis der Nutzer es schließt; dieser Plan setzt `**Status:** done` nicht.

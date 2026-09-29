# Ein Befehl setzt die Ablagedateien auf Werkseinstellungen zurück und liest sie neu ein

---
**Domain:** code
**Status:** done
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260929-1015
**Mode:** autonomous
**Active spec/plan:** 260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md (Spec), 260929-1025_*_plan-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>
---

## Directive

"Wir brauchen einen Befehl unter Krk, der automatisch auf Werkseinstellungen zurücksetzt und die Werte neu einliest." Antworten auf die Klärungsfragen: alle drei von Hand gepflegten Dateien (readers.toml, settings.toml, keymap.toml), im laufenden Betrieb neu eingelesen, alte Dateien mit Zeitstempel beiseitegelegt, neue Dateien sind der Auslieferungsstand. Erreicht ist das, wenn der Befehl im Menü „KRK“ steht und nach Bestätigung die drei Dateien sichert, zurücksetzt und ohne Neustart wirksam macht, wie der Spec es in prüfbaren Kriterien festhält.

## Abschluss

Geschlossen am 260929-1153 als `done`, Commits `fe4b9fb..8b7d7fa` (Plan `fe4b9fb`, Haltepunkte `6317181`, Anforderungsänderung „der Notizordner bleibt“ `79cbb95`; Schritte 3 bis 11 in `ebf94a4`, `a9a8abc`, `c44c71f`, `8b60668`, `e516598`, `e4bf819`, `e71ccd9`, `e8b1e82`, `52be37a`; Durchsicht `260929-1141-reviewer-werkseinstellungen-suchblatt-tabs-eperm-leseprofile.md` in `52afbb2`, ihre drei Befunde behoben in `b54817e` und `8b7d7fa`). `make check` endet auf `8b7d7fa` grün. **Gebaut, nicht abgenommen:** der Abnahmelauf am Bündel mit KRK im Vordergrund ist Nutzerarbeit. Nicht ausgeliefert.

Die Haltebedingungen aus `## Where this work stops` des Plans `260929-1025_*_plan-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md` sind unter **Mode:** autonomous nicht gestellt worden und stehen hier wörtlich:

1. Die Arbeit ist fertig, wenn alle elf Schritte `[DONE]` tragen und `make check` auf dem Commit des letzten Schritts grün endet.
2. Sagt der Bericht aus Schritt 1 oder aus Schritt 2 `Urteil: Stop`, endet die Arbeit dort: kein Schritt ab 3 beginnt, nichts weicht auf einen Neustart oder eine zweite Prüffolge aus, und die Lage geht mit dem Bericht an den Nutzer. (condition did not arise: beide Berichte sagen Go, `6317181`; das Go aus Schritt 2 trägt seit der Anforderungsänderung keinen Schritt mehr.)
3. Kein Schritt schreibt, verschiebt oder löscht eine Datei im Notizordner, und der geltende Notizordner ist nach dem Befehl derselbe wie davor. Gehalten am Baum von `der_werkseinstellungsbefehl_fasst_den_notizordner_nicht_an` (Schritt 8) und von den unveränderten Proben der Ortswahl, am Bündel vom Abnahmelauf.
4. `settings.toml` trägt nach dem Befehl den Wert von `notizordner` aus ihrer alten Fassung, oder die alte Fassung trug keinen und die neue ist die Auslieferungsfassung Byte für Byte; eine beschädigte `settings.toml` hat der Befehl nicht angefasst. Gehalten von den Proben aus Schritt 5.
5. Ausgeliefert wird nicht vor dem Commit von Schritt 8, denn nach Schritt 7 steht der Eintrag im Menü und tut nichts. Auch danach liefert diese Arbeit nicht aus: ein `./release.sh` braucht einen eigenen, ausdrücklichen Auftrag des Nutzers.
6. Der Abnahmelauf am Bündel ist Nutzerarbeit und nicht Teil dieser Arbeit, weil er KRK im Vordergrund verlangt; kein Agent fährt ihn.
7. Ein Abnahmelauf gegen L4 ist nicht geschuldet, solange Schritt 6 den Start nur umordnet und kein Schritt dem Start einen Systemaufruf hinzufügt; der Befehl läuft allein auf Anforderung. Fügt ein Schritt dem Start Arbeit hinzu, hält dieser Schritt an, und der Abnahmelauf wird geschuldet. L4 und L7 bleiben unabhängig davon auf der Liste der späteren Messrunde, auf der sie schon stehen.
8. Das Arbeitspaket bleibt `claimed`, bis der Nutzer es schließt; dieser Plan setzt `**Status:** done` nicht.

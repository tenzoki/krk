# Schuldet das Paket zu Menü „Navigation“, Notizordner und `minutes/` einen Abnahmelauf gegen die zehn Zusagen aus C8?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md, 261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md, 261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261031-1720-schuldet-das-tabellenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261102-1313-schuldet-das-hauptfadenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md

---

## Question

Die Spezifikation stellt die Frage unter `## Out of Scope` der Planung zu: ein Abnahmelauf ist geschuldet, wenn die Arbeit gemessene Wege ändert. Der Plan berührt genau einen gemessenen Weg, und den am Rand: L4, den Start. Die Menüleiste entsteht beim Start aus `Funktionsbereich::ALLE`, und nach Schritt 1 des Plans baut sie ein Obermenü mehr, bei gleicher Zahl der Einträge. Die übrigen neun Zusagen berühren keine geänderte Stelle. Geprüft am Quelltext, nicht gemessen:

- `Heimordner::ist` und `Heimordner::sonderdatei` laufen je Lesevorgang eines Tabs auf dem Hauptfaden. Nach den Schritten 3 und 4 vergleichen sie zwei Dateinamen statt vier, und kein Systemaufruf kommt hinzu.
- Die neue Erkennung `Heimordner::ist_protokollordner` läuft allein beim Bestätigen von „Leere Datei anlegen“ und bei `cmd+m`, nie je Lesevorgang.
- `cmd+m` und F2 fragen die Platte auf dem Faden `PLATTENFRAGE`. F2 nimmt nach Schritt 4 die Schreibsperre der Ablage nicht mehr, weil der Merker der Zettelübernahme entfällt.
- Die Quicknote hängt an keinem Lesevorgang der Dateifenster; ihre Umbenennung läuft auf dem Faden `krk-quicknote`.

Seit dem 261017 stehen alle zehn Zusagen ohnehin auf den Gegenständen der späteren Messrunde. Die Antwort hält keinen Planschritt an, nur das Schließen des Pakets.

## Options

1. **Messen.** Der Nutzer fährt den Abnahmelauf am Bündel im Vordergrund, mindestens L4.
   - Pros: der Start ist nach dieser Arbeit gemessen und nicht erschlossen.
   - Cons: ein Lauf für ein zusätzliches Obermenü; die neun anderen Zusagen sagten über diese Arbeit nichts.
2. **Verzichten.** Dieses Paket fährt keinen Lauf; L4 bleibt mit den neun anderen auf den Gegenständen der späteren Messrunde.
   - Pros: entspricht dem Vorgehen bei den Paketen sftp, Zip, CSV, Tabellen und Hauptfaden (beim SQLite-Paket hat der Nutzer selbst am Bündel geprüft); der berührte Weg wächst um einen `NSMenu` mit zwölf umgehängten Einträgen.
   - Cons: die Aussage „der Start ist nicht langsamer geworden“ bleibt ein Schluss aus dem Quelltext.

## Constraints

- Kein Agent kann den Lauf fahren, er verlangt KRK im Vordergrund.
- Die zehn Zeitzusagen werden nicht verändert und nicht um eine elfte ergänzt (Spezifikation, `## Constraints`).
- Kein Planschritt hängt an der Antwort.

## Recommendation

Möglichkeit 2. Der einzige berührte gemessene Weg ist der Start, und dort kommt ein Obermenü hinzu, dessen Einträge schon bisher gebaut wurden. Eine Messung dieses Unterschieds läge unter der Streuung der bisherigen Läufe. Wer den Start ohnehin messen will, tut es in der späteren Messrunde, in der L4 seit dem 260910 steht.

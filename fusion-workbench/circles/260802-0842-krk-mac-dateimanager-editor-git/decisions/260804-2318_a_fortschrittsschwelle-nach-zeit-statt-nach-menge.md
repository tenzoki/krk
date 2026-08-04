# Hängt die Fortschrittszusage aus C4 an einer Menge oder an einer Zeit?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-2040_o_das-stapel-umbenennen-laeuft-ohne-fortschritt-und-ohne-abbruch-auf-dem-hauptfaden.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1649_o_innerhalb-eines-apfs-datentraegers-gibt-es-kein-mitten-in-einer-datei.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C4), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (`### Frage 6`, S15, S17c)

---

## Frage

C4 sagte zu: "Eine Operation über mehr als 100 Einträge oder mehr als 100 MB zeigt einen Fortschritt und lässt sich mit einem Tastenbefehl abbrechen." Zwei Messungen vom 260804 zeigen, dass diese Schwelle das Gemeinte in beide Richtungen verfehlt. Bleibt sie eine Menge, oder wird sie eine Zeit?

## Was gemessen wurde

| Fall | Messung | Was die Mengenschwelle sagte | Was der Nutzer merkt |
|---|---|---|---|
| 5.000 `rename(2)` nacheinander, Prüfordner unter `/tmp`, derselbe APFS-Datenträger, gemessen 260804-2040 | **525 ms**, der Hauptfaden steht die ganze Zeit | ab dem 100. Eintrag Fortschritt, also nach rund 10 ms | einen Hänger von einer halben Sekunde |
| 100 `rename(2)`, dieselbe Lage, linear aus derselben Messung | rund **10 ms** | Fortschritt zugesagt | nichts |
| Kopie einer 500-MB-Datei innerhalb eines APFS-Datenträgers, `COPYFILE_ALL \| COPYFILE_CLONE`, gemessen 260804-1649 | **0,42 ms**, Statusrückruf wird gar nicht gerufen | Fortschritt und Abbruch zugesagt | nichts, die Operation ist vorbei, bevor sie beginnen konnte |
| Dieselbe Datei, `COPYFILE_ALL` ohne Klon | über 400 ms, Abbruch bei 32 MiB nach 40 ms | Fortschritt und Abbruch zugesagt | eine Wartezeit, hier zu Recht |

`copyfile(3)` sagt zum Klonweg wörtlich: "if cloning is successful, progress callbacks will not be invoked". Das deckt sich mit der Messung.

## Möglichkeiten

1. **Die Menge bleibt, und C4 bekommt eine Ausnahme für den Klonweg.**
   - Pro: die vertraute Formulierung bleibt stehen.
   - Contra: die Ausnahme trägt eine eigene Sonderregel für eine von fünf Operationsarten und ist genau das, was die Maxime "supersimpel" als Ausschlussgrund benennt. Den 525-ms-Hänger löst sie nicht, und die 10-ms-Zusage bleibt sinnlos.
2. **Die Schwelle wird eine Zeit: 150 ms.** *(gewählt)*
   - Pro: eine Schwelle für alle fünf Arten. Sie ist keine Neuerfindung, sondern die Regel, nach der der Fortschritt ohnehin schon erscheint (`### Frage 6` des Plans), jetzt auch als Bedingung der Zusage. Der Klonfall löst sich von selbst, ohne Ausnahme. Der Umbau des Stapel-Umbenennens fällt korrekt unter die Zusage.
   - Contra: eine langsame Operation über wenige große Dateien auf ein Netzlaufwerk fällt jetzt unter die Zusage, eine schnelle über tausend winzige Dateien fällt heraus. Beides ist gewollt, muss aber benannt sein.
3. **C4 nimmt das Stapel-Umbenennen ausdrücklich von der Fortschrittszusage aus.**
   - Pro: der billigste Weg, S17 unangetastet zu lassen.
   - Contra: 525 ms stehende Oberfläche verfehlen die C4-Zusage "während eine Operation läuft, ist das Fenster bedienbar" und L9 aus C8. Eine Ausnahme für eine von fünf Arten, dasselbe Argument wie bei Möglichkeit 1.

## Randbedingungen

- `COPYFILE_CLONE` fallen zu lassen scheidet aus: eine Kopie von 50 GB dauerte dann Minuten statt Millisekunden und verbrauchte 50 GB Plattenplatz.
- Ein Vorablauf über den Ordnerbaum, um die Menge vorher zu bestimmen, scheidet aus: er kostet einen eigenen Durchlauf, der die 200 ms aus L8 selbst aufbrauchen kann. `### Frage 6` schließt ihn seit jeher aus.
- **Keine der zehn Zahlen aus C8 ist berührt.** L8 bleibt bei 200 ms, L9 bleibt beim Anteil der Eingaben im nächsten Bild. Die 150 ms sind eine C4-Schwelle und keine C8-Zusage.

## Antwort

Möglichkeit 2. Grundlage ist der Auftrag des Nutzers vom 260804: "falls die engen Zeitvorgaben Problem machen: aufweichen, pragmatische Lösungen planen." Die Änderung ist keine Lockerung im Ergebnis, sondern eine Verschiebung des Maßstabs auf das, was der Nutzer tatsächlich merkt.

**Alte Fassung:** "Eine Operation über mehr als 100 Einträge oder mehr als 100 MB zeigt einen Fortschritt und lässt sich mit einem Tastenbefehl abbrechen."

**Neue Fassung:** "Eine Operation, die länger als 150 ms läuft, zeigt einen Fortschritt und lässt sich mit einem Tastenbefehl abbrechen. Eine Operation, die vorher fertig ist, zeigt keinen und braucht keinen."

Was daraus folgt, steht im Plan: das Stapel-Umbenennen wandert mit dem neuen Schritt S17c auf die Operationsmaschine aus S15, weil es mit gemessenen 525 ms über der Schwelle liegt. Der Eingriff ist Wiederverwendung und kein Neubau; Arbeitsfaden, Abbruchkennzeichen, Fortschrittskanal und die Sammlung übersprungener Einträge bringt die Maschine mit, und die Arbeit je Eintrag ist ohnehin schon `operation::umbenennen`.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C4, fünftes Abnahmekriterium und der Absatz "Die Schwelle für Fortschritt und Abbruch ist seit dem 260804-2318 eine Zeit und keine Menge") — Schwelle von 100 Einträgen beziehungsweise 100 MB auf 150 ms Laufzeit gezogen; Umsetzung im Code steht als S17c aus.

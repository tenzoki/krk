# Fällt das Default-Profil auch im Messmodus an, und was misst L7 danach?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::sitzung_laden`, der Absatz „Seit der Runde 16 kommen die Leseprofile im selben Durchgang mit"); `crates/krk-ui/src/messmodus.rs` (die L7-Reihe in `sitzungsschritte`, die zwei Vorschau-Bedingungen darüber); `crates/krk-bench/src/fixture.rs` (`Art::Ordner`, der Anteil der Unterordner im Bauplan); `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (Schritt 3 und Schritt 4); `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`

---

## Question

Die Runde 16 hat den Messmodus bewusst profillos gelassen, und diese Runde hebt diese Vorkehrung auf, ohne dass jemand es beschlossen hätte.

Der Doc-Kommentar von `Anwendungsdelegierter::sitzung_laden` schreibt die damalige Lage aus: alle vier Messaufgaben kehren zurück, bevor der Ablagedurchgang läuft, also übergibt `Vorschaufenster::profile_setzen` dort einen leeren Profilsatz, und der Kommentar zieht daraus den Schluss „keine der zehn Zeitzusagen misst an einer Zusammenfassung". Der Schluss war richtig, solange ein leerer Profilsatz gleichbedeutend mit „keine Zusammenfassung" war.

Nach dieser Runde ist er es nicht mehr. Das Default-Profil ist in KRK eingebaut und kommt aus keiner Ablagedatei; ein leerer Profilsatz heißt danach „kein Profil aus `readers.toml`" und nicht „keine Auswertung". Ein ausgewählter Unterordner bekommt im Messmodus damit dieselben drei Zählzeilen wie überall sonst, und der Verzeichnisleselauf dafür fällt innerhalb der Endbedingung von L7 an: die L7-Reihe misst zwanzigmal `auswahl_runter` und wartet je Anschlag, bis der Vorschau-Tab den neuen Eintrag zeigt und kein Arbeitsfaden mehr aussteht.

Getroffen wird die Reihe tatsächlich und nicht nur der Möglichkeit nach. Der Prüfordner-Erzeuger legt nach seinem Bauplan einen Anteil Unterordner an, und die zwanzig Anschläge laufen über aufeinanderfolgende Einträge desselben Ordners. Ein Teil der zwanzig Messwerte enthält danach einen Leselauf, den die Reihe vom 260810 nicht enthielt.

Die Sache ist klein und die Frage trotzdem nicht. Die angelegten Unterordner sind leer, ein Leselauf über ein leeres Verzeichnis ist ein `getattrlistbulk(2)` ohne Ergebnis, und die Zusage L7 steht bei 100 ms. Es geht deshalb nicht darum, ob L7 fällt, sondern darum, ob die Reihen zweier Läufe noch dasselbe messen und wer diesen Unterschied wo aufgeschrieben hat.

## Options

1. **Der Messmodus bleibt, wie er ist, und das Default-Profil fällt dort an wie überall.** Der Doc-Kommentar wird berichtigt und sagt künftig, dass ein leerer Profilsatz die Auswertung nicht mehr abschaltet.
   - Pro: L7 misst, was der Nutzer bekommt. Eine Zeitzusage, die an einer Sonderfassung der Anwendung gemessen wird, sagt über die ausgelieferte Fassung weniger, als ihr Wortlaut behauptet, und dieser Einwand gilt der bisherigen Regelung und nicht dieser Möglichkeit.
   - Contra: Die Läufe vor und nach dieser Runde messen nicht mehr genau dasselbe. Wer die Reihe vom 260810 gegen die nächste hält, vergleicht zwei Endbedingungen und nicht eine, und keine Zeile im Bericht sagt es ihm.
   - Was sie ausschließt: nichts Späteres. Eine Ausnahme ließe sich jederzeit nachziehen, und der Preis dafür wäre derselbe wie heute.
   - Downstream: `messungen/260810-1918-alle-zusagen.txt` bleibt die letzte Reihe der alten Endbedingung. Der nächste Abnahmelauf ist der erste der neuen, und ob L7 dann noch hält, ist bis dahin ungemessen — wie die Arbeit der Vorschau ohnehin (`circles/260823-2208-…/decisions/260824-1900_*_…`, offen). Der Berichtskopf müsste die Änderung nennen, sonst liest ein späterer Vergleich zwei Reihen als eine.

2. **Der Messmodus wird vom Default-Profil ausgenommen.** Die vier Messaufgaben schalten es ab, so wie sie heute den Profilsatz leer lassen.
   - Pro: Die Reihen bleiben vergleichbar, und die Senkung von L9 auf 65, die schon einmal aus je einem Lauf stammte und nachgezogen werden musste, bekommt keine zweite Störgröße. Die Streuung zwischen den Runden beträgt ohnehin zwanzig Punkte.
   - Contra: Ein zweiter Schalter im Werk, und einer, den kein Abnahmekriterium dieser Runde verlangt. Er müsste bis in `leseprofil` hinein reichen oder als Sonderweg im Vorschaumodell stehen, und beides steht gegen Constraint 4 des Specs, nach dem der Rückfallweg einer bleibt. Die gemessene Anwendung entfernte sich außerdem ein zweites Mal von der ausgelieferten.
   - Was sie ausschließt: eine spätere Messung der Vorschauarbeit über die vorhandene Strecke. Wer die Zählzeilen ausschaltet, kann ihre Kosten dort nie messen, und die offene Frage von 260824-1900 bekäme eine Antwort weniger.
   - Downstream: Der Doc-Kommentar bliebe fast richtig und müsste doch nachgezogen werden, denn seine heutige Begründung („der Profilsatz bleibt leer") trägt die Aussage dann nicht mehr, auch wenn die Aussage selbst stimmt.

3. **Der Messmodus bleibt, wie er ist, und der Prüfordner der Messstrecke verliert seine Unterordner.** Der Bauplan legte dann allein Dateien und Verknüpfungen an.
   - Pro: L7 misst weiter ohne Leselauf, ohne dass das Werk einen zweiten Schalter bekommt.
   - Contra: Der Prüfordner soll nach dem Kopf von `krk-bench/src/fixture.rs` einen gewachsenen Arbeitsordner nachbilden, und einer ohne einen einzigen Unterordner tut das nicht. Die Änderung träfe außerdem L1, L3 und L10 mit, die denselben Ordner lesen und sortieren, und deren Reihen wären dann erst recht nicht mehr vergleichbar.
   - Was sie ausschließt: die Nachbildung eines gewachsenen Ordners in der Messstrecke, dauerhaft. Ein späterer Rückbau kostete dieselbe Unvergleichbarkeit noch einmal.

## Constraints

- Diese Runde setzt keine elfte Zeitzusage und fasst keine der zehn an. Die Zahlen selbst stehen in keiner der drei Möglichkeiten zur Wahl; `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u` liefert vor und nach ihr dieselbe Menge.
- Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; kein Agent kann ihn fahren (`circles/260802-0842-…/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`, offen). Keine der drei Möglichkeiten lässt sich vor der Antwort messen.
- Constraint 4 des Specs dieser Runde: der Rückfallweg bleibt einer, und das Default-Profil tritt neben `erkennen` und nicht in es hinein.
- Der Plan dieser Runde baut ohne Ausnahme für den Messmodus und berichtigt den Doc-Kommentar (Schritte 3 und 4). Möglichkeit 1 verlangt danach keine Arbeit mehr, Möglichkeit 2 und 3 je einen eigenen Schritt.
- Die Frage hält den Plan nicht auf und ist keine Vorbedingung für den Abschluss dieser Runde.

## Recommendation

Möglichkeit 1, und die Begründung ist nicht die Bequemlichkeit, sondern die Richtung des Fehlers. Eine Messstrecke, die eine Sonderfassung der Anwendung vermisst, gibt eine Zahl aus, die besser ist als die des Nutzers, und niemand sieht der Zahl das an. Der Unterschied, den diese Runde einbringt, geht in die andere Richtung: die gemessene Anwendung rückt näher an die ausgelieferte heran.

Der Preis ist zu nennen und nicht kleinzureden. Die Reihe vom 260810 und die nächste messen zwei verschiedene Endbedingungen, und das gehört in den Kopf des nächsten Berichts, nicht in einen Datensatz, den beim Vergleichen niemand liest. Wer Möglichkeit 2 vorzieht, wählt die Vergleichbarkeit zweier Reihen über die Aussagekraft jeder einzelnen, und das ist eine vertretbare Wahl, solange die Senkung von L9 noch aussteht.

Die Prüfordner sind dünnbesetzt und taugen nicht für die Kopiermessung L8

---

Der Prüfordner-Erzeuger aus Schritt 3 legt dünnbesetzte Dateien an: nur die ersten 512 Byte je Datei sind echt geschrieben, der Rest ist ein Loch im Dateisystem. Der Ordner mit 100.000 Einträgen nennt deshalb 197 GB Größe und belegt 342 MB Platte.

Für die Zusagen, die Schritt 3 und Schritt 8 messen, ist das folgenlos: L2, L3 und L10 lesen ausschließlich Metadaten, und die sind echt.

**Für die Zusage L8 sind diese Ordner unbrauchbar.** L8 misst einen Kopiervorgang, und ein Kopiervorgang über dünnbesetzte Dateien misst das Kopieren von Löchern, nicht das Kopieren von Daten. Je nach Weg (`copyfile` erhält die Löcher, ein byteweiser Weg füllt sie mit Nullen) kommen dabei Zahlen heraus, die um Größenordnungen auseinanderliegen und beide nichts über den Alltag sagen.

Der `coder` hat den Hinweis in den Modulkopf des Erzeugers geschrieben. Diese Ablage macht ihn auffindbar, bevor jemand in Schritt 22 die vorhandenen Ordner nimmt, weil sie dastehen.

---

**Was zu tun ist.** Der `planner` legt fest, worauf L8 gemessen wird, und ergänzt Schritt 22 entsprechend. Zwei Wege bieten sich an:

- **Ein eigener, dicht geschriebener Prüfordner für L8.** Der Erzeuger bekommt einen Schalter, der die Dateien wirklich füllt. Die Größe muss dann realistisch gewählt sein statt 197 GB; welche Datenmenge L8 meint, sagt der Spec bisher nicht.
- **L8 auf einem vom Nutzer benannten echten Ordner messen.** Näher am Alltag, aber nicht reproduzierbar, und C8 verlangt zwanzig Wiederholungen mit vergleichbarem Ergebnis.

Der erste Weg passt besser zur Messordnung des Specs. Er wirft eine Frage auf, die der Spec offen lässt: **welche Datenmenge L8 zugrunde legt.** Wenn der `planner` sie nicht aus C8 ableiten kann, gehört sie als Entscheidung zum Nutzer.

**Kein Handlungsdruck vor Schritt 16.** Bis dahin gibt es keinen Kopiervorgang zu messen.

**Aufgefallen bei:** der Umsetzung von Schritt 3, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1850-pruefordner-erzeuger-und-kopflose-messstrecke.md`.

---
Resolved: Die Datenmenge für L8 ist aus C8 ableitbar. Es entsteht **kein** Entscheidungsdatensatz und **kein** dichter Prüfordner. Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` ist am 260802-1859 in `### Frage 5`, in S21, in S22 und in der Risikotabelle nachgezogen.

**Die Frage nach der Datenmenge ruht auf einer Prämisse, die C8 nicht trägt.** C8 sagt für L8 wörtlich zu: "Kopier- oder Verschiebevorgang: Fortschritt sichtbar, 200 ms nach Start". L8 ist eine Sichtbarkeitszusage, keine Durchsatzzusage. Ausgelöst wird die Sichtbarkeit nach der Regel aus `### Frage 6` des Plans von einem Zeitgeber nach 150 ms und nicht von einer übertragenen Datenmenge. Der Prüfbestand muss deshalb genau eine Eigenschaft haben: die Operation muss nach 150 ms noch laufen. Wie viele Bytes dabei bewegt werden, geht in die Zahl nicht ein.

**Das leisten die vorhandenen Prüfordner, und die Zahl dazu ist gemessen.** Auf dem Referenzgerät am 260802-1859, mit 10.000 dünnbesetzten Einträgen als Quelle und dem Ziel auf demselben APFS-Datenträger: `cp -Rc`, also derselbe Klonweg, den `copyfile` mit `COPYFILE_CLONE` nimmt, braucht 1,83 bis 1,95 s über drei Läufe; `cp -R` ohne Klonen braucht 4,44 bis 4,51 s. Beide liegen mehr als das Zehnfache über der Auslöseschwelle von 150 ms, weil die Laufzeit an der Zahl der Einträge hängt und nicht an den Bytes. `inference:` KRKs eigener Weg mit `COPYFILE_ALL` kopiert zusätzlich erweiterte Attribute und liegt eher darüber; die gemessenen Zahlen sind eine Untergrenze.

**L8 und L9 messen deshalb auf Prüfordner A**, kopiert in ein Ziel auf demselben Datenträger. Ein zweiter Erzeugerpfad mit einem Schalter für dichte Dateien entsteht nicht; das wäre der Mechanismus, den der Plan mit `## Wie dieser Plan die Maxime "supersimpel" einlöst` gerade vermeidet.

**Die Löcher sind an einer anderen Stelle gefährlich, und dort steht jetzt eine Bedingung.** Ein Ziel auf einem Datenträger, der keine Löcher hält, etwa ein exFAT-formatierter Stick, zwingt `copyfile`, sie als Nullen auszuschreiben: aus 342 MB Plattenbelegung würden 197 GB. Die Messstrecke nimmt für L8 und L9 nur ein Ziel auf demselben APFS-Datenträger wie die Quelle an und bricht sonst mit einer Meldung ab. Der Bericht schreibt aus, dass auf dem Klonweg gemessen wurde, damit die Zahl später nicht als Durchsatzangabe gelesen wird.

**Zur Beobachtung dieses Defekts, die beiden Wege lägen um Größenordnungen auseinander:** am Referenzgerät gemessen tun sie das nicht, und der Grund ist, dass macOS die Löcher auf beiden Wegen erhält. Eine 1-GB-Datei mit 512 echten Bytes belegt nach `cp -c` wie nach `cp` unverändert 8 Blöcke. Die Sorge trifft erst über eine Datenträgergrenze hinweg zu, und genau dort greift die Bedingung oben.

**Kein Handlungsdruck vor Schritt 16 bleibt richtig**, an der Wahl ändert das nichts.

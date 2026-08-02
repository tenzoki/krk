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

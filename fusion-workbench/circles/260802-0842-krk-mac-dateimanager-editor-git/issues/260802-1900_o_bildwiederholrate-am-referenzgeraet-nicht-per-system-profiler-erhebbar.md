Die Bildwiederholrate ist am Referenzgerät nicht per `system_profiler` erhebbar

---

Der Bedingungskopf jedes Messberichts soll nach Zeile 149 des Plans `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` acht Angaben tragen, darunter die Bildwiederholrate des Bildschirms. Sie ist die Herleitung der Zusage L1: 16 ms sind ein Bild bei 60 Hz.

Auf dem Referenzgerät liefert `system_profiler SPDisplaysDataType` zum eingebauten Bildschirm des `MacBookPro15,1` **keine** Zeile `Refresh Rate`. Der `coder` hat das bei der Umsetzung von Schritt 3 festgestellt und die Angabe nicht erfunden: der Bedingungskopf schreibt die Lücke aus und nennt stattdessen die Auflösung.

---

**Warum das mehr als ein fehlendes Feld ist.** Ohne die Rate ist L1 nicht gegen seine eigene Herleitung prüfbar. Ein Messwert von 16 ms heißt "ein Bild" nur, wenn der Bildschirm mit 60 Hz läuft; auf einem Bildschirm mit 120 Hz wäre dieselbe Zahl zwei Bilder.

**Was zu tun ist.** Die Rate muss aus der laufenden Anwendung kommen statt aus `system_profiler`. AppKit stellt sie über `NSScreen.maximumFramesPerSecond` bereit; das ist über `objc2-app-kit` erreichbar, das der Workspace seit Schritt 1 als Abhängigkeit führt.

Das bindet **Schritt 21** (Messmodus in der Anwendung): dort läuft KRK selbst, also ist die Rate dort erhebbar. Der `planner` ergänzt in S21, dass der Bedingungskopf die Rate aus `NSScreen` liest, und in S3, dass der kopflose Weg sie mangels Fenster nicht erheben kann und die Lücke ausschreibt.

**Für die kopflose Messstrecke aus Schritt 3 bleibt es bei der Lücke**, und das ist richtig: ohne Fenster gibt es keinen Bildschirm, dem eine Rate zuzuordnen wäre. Die kopflose Strecke misst ohnehin keine der bildbezogenen Zusagen.

**Nebenbefund derselben Meldung:** Zeile 149 verlangt acht Kopfangaben, die Änderungsliste von Schritt 3 nennt sechs. Der `coder` hat die Obermenge gewählt und alle acht eingebaut. Der `planner` sollte die beiden Stellen angleichen, damit die nächste Umsetzung nicht wieder abwägen muss.

**Aufgefallen bei:** der Umsetzung von Schritt 3, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1850-pruefordner-erzeuger-und-kopflose-messstrecke.md`.

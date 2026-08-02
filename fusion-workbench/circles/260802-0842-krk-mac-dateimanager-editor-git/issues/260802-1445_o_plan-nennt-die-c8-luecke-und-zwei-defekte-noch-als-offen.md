Der Plan nennt die C8-Lücke und zwei Defekte noch als offen

---

Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` beschreibt an drei Stellen einen Zustand, der seit dem 260802-1423 beziehungsweise dem 260802-1445 nicht mehr gilt. Alle drei sind Zustandsaussagen, nicht nur veraltete Pfade.

1. Abschnitt `### Frage 5`, Absatz "Der Prüfordner wird erzeugt, nicht gesammelt": "Beide Größen sind gefordert, die Messbedingungen in C8 nennen nur die kleinere; der Defekt dazu ist gemeldet." Die Messbedingungen in C8 nennen seit dem 260802-1445 beide Größen, den Ordner mit 10.000 Einträgen für L2 und L3 und den mit 100.000 für L10, und verlangen zusätzlich, dass beide nach demselben reproduzierbaren Verfahren entstehen. Der Defekt `260802-1428_c_messbedingungen-c8-nennen-keinen-pruefordner-fuer-l10.md` ist geschlossen.

2. Abschnitt `## Angelegte Defekte und Entscheidungen`, erster Aufzählungspunkt: derselbe Defekt wird unter dem Marker für offen geführt.

3. Abschnitt `## Zwei gemeldete Defekte, die den Plan nicht ändern`: beide genannten Defekte vom 260802-1417 sind seit dem 260802-1423 geschlossen und heißen entsprechend anders. Die Aussage des Abschnitts, dass für die Tastenbelegung C3 des Specs gilt, bleibt richtig; überholt ist allein der Stand der beiden Defekte, und die Ankündigung "Ein anderer Agent behebt beide parallel" ist eingelöst.

---

**Was zu tun ist:** die drei Stellen auf den heutigen Stand ziehen. Für Stelle 1 entfällt der Halbsatz zur Lücke; die Aussage, dass beide Größen aus demselben Verfahren mit festem Startwert entstehen, bleibt unverändert richtig und ist jetzt die Umsetzung einer Zusage des Specs statt eines Vorgriffs darauf. Für die Stellen 2 und 3 genügt es, die Marker der genannten Dateien nachzuziehen und die beiden Defekte vom 260802-1417 als erledigt auszuweisen.

**Warum der Shaper es nicht selbst behoben hat:** der Plan gehört dem Planner. Der Auftrag dieser Runde hat ausdrücklich festgehalten, dass der Shaper ihn liest und nicht ändert.

**Aufgefallen bei:** dem Schließen des Defekts zur C8-Lücke, Sitzung `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1445-shaper-restellen-fn-und-c8.md`.

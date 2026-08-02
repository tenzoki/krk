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

---
Resolved: Am 260802-1501 vom Planner im Plan nachgezogen, zusammen mit dem Nachzug der Konzeptprüfung `reviews/260802-1447-conceptrev-plan-navigator-geruest-runde-1.md`.

Alle drei Stellen sind vor der Änderung am Dateibestand geprüft und nicht dem Defekt geglaubt worden. Geprüft wurde: `issues/260802-1428_c_messbedingungen-c8-nennen-keinen-pruefordner-fuer-l10.md`, `issues/260802-1417_c_directive-zeile-sagt-freie-funktionstasten-zu.md` und `issues/260802-1417_c_circle-datensatz-status-widerspricht-dem-marker.md` tragen alle drei den Marker für geschlossen; der Satz unter **Messbedingungen** in `planning/260802-1036_o_spec-navigator-geruest.md` nennt seit dem 260802-1445 beide Prüfordner und verlangt die Reproduzierbarkeit ausdrücklich. Der Defekt trifft in allen drei Punkten zu.

Was im Plan steht:

1. Abschnitt `### Frage 5`: der Halbsatz zur Lücke ist ersetzt. Der Absatz sagt jetzt, dass C8 seit dem 260802-1445 beide Größen nennt und die Reproduzierbarkeit verlangt, und dass der feste Startwert des Zufallsgenerators in S3 genau diese Zusage umsetzt.
2. Abschnitt `## Angelegte Defekte und Entscheidungen`: der C8-Defekt trägt den Marker für geschlossen und ist als erledigt ausgewiesen, mit dem Datum und dem, was der Shaper geändert hat. Die beiden offenen Entscheidungen daneben sind gleich mit ausgezeichnet, damit die Aufzählung durchgängig ihren Stand nennt.
3. Der Abschnitt `## Zwei gemeldete Defekte, die den Plan nicht ändern` heißt jetzt `## Zwei Defekte am Circle-Datensatz, inzwischen geschlossen`, führt beide Dateien unter ihrem heutigen Namen und nennt das Schließdatum 260802-1423. Die Ankündigung "Ein anderer Agent behebt beide parallel" ist entfallen. Die Sachaussage, dass für die Tastenbelegung C3 des Specs gilt, steht unverändert.

Ein vierter überholter Verweis ist beim Nachziehen mitgeprüft und war keiner: die Verweise auf `spikes/fn-tasten/messung-A.txt` und `messung-A-neuauswertung.txt` in `## Ausgangslage` und in Frage 1 stimmen mit dem Dateibestand überein.

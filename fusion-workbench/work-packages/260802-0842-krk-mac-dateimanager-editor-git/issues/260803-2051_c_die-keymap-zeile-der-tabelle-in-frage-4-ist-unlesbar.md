Die keymap-Zeile der Tabelle in Frage 4 ist unlesbar

---

Die Tabelle in `### Frage 4` beschreibt den Inhalt von `keymap.toml` mit dem
Satz:

> die Belegung des Nutzers, nur die Abweichungen vom Auslieferungszustand
> nicht, sondern die vollständige Tabelle

Der Satz ist grammatisch defekt. Ein "nur" ohne Bezug und ein nachgestelltes
"nicht" lassen ihn beim ersten Lesen genau umgekehrt verstehen: als hielte die
Datei die Abweichungen.

---

**Was gemeint ist.** S11 sagt es sauber: "wobei die Nutzerdatei die
Auslieferungsbelegung vollständig ersetzt und nicht ergänzt". Die Datei hält
also die ganze Tabelle, nicht die Abweichungen. Der Sachverhalt ist damit
belegt und nicht offen; allein die Formulierung in `### Frage 4` trägt ihn
nicht.

**Warum es zählt.** `### Frage 4` ist die Stelle, an der ein Umsetzender das
Format nachschlägt, und die Tabelle ist die kürzeste Fassung davon. Ein Satz,
der beim ersten Lesen das Gegenteil sagt, ist an dieser Stelle teurer als
anderswo: die Vervollständigungslogik einer Abweichungsdatei ist ein anderer
Entwurf als das einfache Ersetzen.

**Vorschlag.** Die Zelle auf "die vollständige Belegung des Nutzers, nicht nur
seine Abweichungen vom Auslieferungszustand" ziehen.

**Dringlichkeit.** Bindet S11, aber nur als Lesefalle; der Sachverhalt steht in
S11 richtig.

**Aufgefallen bei:** der Umsetzung von S10, beim Lesen von `### Frage 4`.

---
Resolved: Die Zelle in der Tabelle von `### Frage 4` lautet jetzt "die vollständige Belegung des Nutzers, nicht nur seine Abweichungen vom Auslieferungszustand", genau der Vorschlag des Datensatzes. Nachgezogen am 260804-2318 vom `planner`.

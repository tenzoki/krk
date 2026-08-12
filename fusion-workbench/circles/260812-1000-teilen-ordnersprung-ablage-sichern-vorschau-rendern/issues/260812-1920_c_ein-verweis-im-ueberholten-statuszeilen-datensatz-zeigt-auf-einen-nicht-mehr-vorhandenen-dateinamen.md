Ein Verweis im überholten Statuszeilen-Datensatz zeigt auf einen Dateinamen, den es nicht mehr gibt

---

`decisions/260812-1105_s_die-statuszeile-zieht-ueber-die-volle-fensterbreite-und-laesst-sich-blaettern.md`
nennt in seiner `Superseded by:`-Zeile den Nachfolger als
`…/decisions/260812-1809_a_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`.
Die Datei heißt seit `df4ec00` `260812-1809_i_…`. Der Pfad läuft ins Leere.

---

**Nachgeprüft:**

```
$ ls fusion-workbench/circles/260812-1000-*/decisions/260812-1809_*
…/260812-1809_i_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md
```

Kein `_a_`. Der Marker ist in `df4ec00` von beantwortet auf umgesetzt gezogen
worden, der Rückverweis nicht.

**Der Schwesterdatensatz macht es anders.** `decisions/260812-1000_s_welchen-umfang-von-markdown-rendert-die-vorschau.md`
nennt seinen Nachfolger im selben Feld mit dem **heutigen** Marker
(`260812-1851_i_…`), und dieser Pfad löst auf. Zwei `Superseded by:`-Zeilen
desselben Speichers, im Abstand von zwei Commits geschrieben, folgen zwei
verschiedenen Konventionen.

**Dieselbe Kurzform steht ein zweites Mal** im Turnlog des Circle-Datensatzes
(`_t_circle.md:171`: „260812-1809_a_ traegt die neue Antwort"). Dort ist sie
als Aufzeichnung des damaligen Standes vertretbar; im `Superseded by:`-Feld
eines fortgeltenden Datensatzes ist sie ein toter Pfad, denn dieses Feld ist
der eine Weg vom überholten zum geltenden Datensatz.

**Der Zuschnitt, den das Projekt sonst benutzt**, ist die Sternform `_*_`: sie
steht in `CLAUDE.md` und in den Modulköpfen durchgängig und überlebt jeden
Markerwechsel. Sie hat einen bekannten Preis — `CLAUDE.md` führt unter
„Was man nicht sieht", dass jedes Suchmuster mit `\.md` an Kurzformen
vorbeiläuft —, aber ein Verweis, der auflöst, wiegt hier schwerer als einer,
der gefunden wird.

**Gewicht:** gering. Ein Leser findet die Nachfolgedatei über den Zeitstempel
in Sekunden. Der Datensatz steht trotzdem hier, weil die
Entscheidungsdatensätze nach `CLAUDE.md` die bindende Grundlage sind und ein
Verweis zwischen ihnen auflösen sollte.

**Herkunft:** Circle der Runde 6, Turn 3, `df4ec00`.

---
Resolved: Beide Zeigerstellen tragen jetzt die Sternform. `decisions/260812-1105_s_…` nennt den
Nachfolger als `260812-1809_*_…`, `decisions/260812-1000_s_…` als `260812-1851_*_…`, und der
Turn-2-Eintrag im Circle-Datensatz ebenso. Die Sternform ist die Regel aus
`rules/circle-records.md`, Abschnitt `### Citation form in the portfolio`, und ihr Grund gilt
hier genauso: ein ausgeschriebener Marker ist ein Zeiger, der beim ersten Zustandswechsel seines
Ziels stirbt. Genau das ist mit `df4ec00` geschehen, als der Nachfolger von beantwortet auf
umgesetzt ging.

Die Nennungen in diesem Datensatz selbst und in
`issues/260812-1816_*_die-durchsicht-von-turn-2-liest-einen-reinen-grundlagen-commit-als-codeaenderung.md`
bleiben ausgeschrieben. Dort ist der Marker die Aussage und kein Zeiger: sie halten fest, welcher
Name damals dastand. Die Sternform loeschte den Inhalt.

Der Fehler stammt vom Orchestrator, der beide `Superseded by:`-Zeilen geschrieben hat.


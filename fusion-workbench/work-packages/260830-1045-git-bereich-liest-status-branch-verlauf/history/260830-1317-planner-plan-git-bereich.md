# Planner: der Plan der Runde 23

**Date:** 2026-08-30 13:17
**Status:** Complete
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Circle:** 260830-1045-git-bereich-liest-status-branch-verlauf
**Baumstand:** `2059138`

## Auftrag

Den Plan der Runde 23 gegen den freigegebenen Spec schreiben, die neun Fragen aus
`## Open for Planner` entscheiden, alle 90 Abnahmekriterien einem Schritt zuordnen und die
25 Nutzerkriterien in einen eigenen Abnahmeschritt legen.

## Was entstanden ist

- `planning/260830-1317_o_plan-git-bereich-liest-status-branch-verlauf.md` — siebzehn Schritte,
  drei Mermaid-Graphen, `**Decidability:**` im Kopf, `## Where this Circle stops` mit dreizehn
  Klauseln.
- Vier Defekte in `issues/`, alle beim Abgleich des Specs gegen den Baum gefunden:
  die Feldbreiten-Behauptung in C1.1, der Widerspruch in der 25er-Liste, die falsche Begründung
  für die Unberührtheit der Messstrecke, das zu enge Erhebungsmuster aus C9.4.
- Zwei Entscheidungsdatensätze in `decisions/`: der zehnte `Funktionsbereich` und die Fadenzahl
  von `gix`.
- Ein `Also seen:`-Vermerk an
  `260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-…`.

## Die eine Messung dieser Sitzung

Die Behauptung aus C1.1 und aus dem verwiesenen Defekt, vier Feldbreiten hielten den Bau an,
sobald `Bereich::ALLE` wächst, ist in einem eigenständigen Wegwerf-Workspace nachgestellt worden
(sechswertige Aufzählung, sechsgliedrige `ALLE`, daneben die vier Bauformen wie im Baum).
`cargo build` hält genau **eine** an, `Bereichsleiste::bereichsschalter`, weil sie über
`Bereich::ALLE.map(…)` gebaut ist; `Aufteilung::rahmen` (Literal), `Aufteilung::gemessene_breiten`
und `bereichsbreiten` (`[0.0; 5]`) übersetzen grün und brechen mit
`index out of bounds: the len is 5 but the index is 5`. `Fenstermodell::breiten_uebernehmen` bricht
gar nicht. Der Plan zieht daraus die Folge, dass Schritt 1 die drei stillen Stellen namentlich
aufzählt statt sich auf die Fehlerliste des Übersetzers zu verlassen.

## Die neun Entscheidungen, je ein Satz

1. `Bereich::teilt_flaeche_mit` fällt; `Bereich::flaeche -> Flaeche` mit vier Werten tritt an seine
   Stelle, und „teilt sich die Fläche" wird Gleichheit einer Klasse, damit die Symmetrie aus der
   Bauform fällt statt von einer Probe bewacht zu werden.
2. `Wirkungsbereich::Navigator` wächst um `Fokus::Git`; ein neunter Wirkungsbereich entsteht nicht,
   und `Wirkungsbereich::beschriftung` für `Navigator` ändert damit einen Text, den der Nutzer liest.
3. Ein Faden, ein `sync_channel(3)`, drei Meldungsarten (Kopf, Verlauf, Marken) und zwei Fragen an
   einer Maschine; die Generation reist mit und wird gegengehalten, weil die Zuordnung über den
   Namen läuft und ein Name den natürlichen Schutz des Eintragsindex nicht hat.
4. `gitmarke: Vec<Option<Marke>>` parallel zum Bestand, geleert in `ersatz_einloesen`, nicht
   angefasst von `befund_zuruecksetzen`; `gitmarken_setzen` baut die Sicht **nicht** neu auf.
5. Ein `Gitfenster` nach dem Muster des Vorschaufensters, drei Flächen ohne zweite `NSSplitView`,
   die Verlaufsliste als einspaltige `NSTableView` wie die Lesezeichenleiste, das Nachladen über
   einen Melder statt über Kenntnis der Tabliste.
6. Mindestbreite 340, Anfangsbreite 420, beide aus den gemessenen Spaltenbreiten der Dateiliste
   abgeleitet; die Lesbarkeit bei 340 ist Nutzerarbeit.
7. Kein Deckel auf die Fadenzahl; die Stelle steht namentlich im Modulkopf als erster Hebel, und
   die Frage ist als Datensatz gefilt, weil sie aus den Eingaben des Mechanismus nicht entscheidbar
   ist.
8. `belegungsausgabe.rs` keine Codezeile (aber zwei Prosastellen und ein unvermeidlicher Diff),
   `belegungsmodell.rs` zwei Codestellen samt zehntem `Funktionsbereich`, `messmodus.rs` keine
   Codezeile — und die Begründung des Specs dafür ist falsch, der Messplatz und nicht der
   Schalterstand schützt die Messung.
9. Die 92 Stellen zerfallen in zwei eigene Schritte nach der Aussage (Bereiche und Fokuswerte,
   Spalten und Schalter), dazu ein dritter für CLAUDE.md und ein vierter für die C-Freiheits-Zusage;
   das Muster wird vor dem Zählen erweitert, und der Anlass ist gefunden.

## Zuordnung der Kriterien

Alle 90 Kriterien haben eine benannte Stelle. Keines ist ohne Schritt geblieben. Die 25
Nutzerkriterien stehen als Prüfliste in Schritt 17 und in keinem Coder-Schritt.

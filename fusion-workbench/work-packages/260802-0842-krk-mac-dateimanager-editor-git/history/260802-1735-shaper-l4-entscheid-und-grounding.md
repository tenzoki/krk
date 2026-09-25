# Shaper 260802-1735: L4-Entscheid eingetragen, Grounding snapshot nachgezogen

**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Modus:** in-Circle clarification, mit Freigabe des Nutzers für den Circle-Datensatz
**Auftrag:** die Nutzerentscheidung zu L4 eintragen, die Messbedingungen um die Sitzungslage ergänzen, den Defekt vom 260802-1445 schließen und den Abschnitt `## Grounding snapshot` sowie C8 vollständig auf den heutigen Stand ziehen, statt weitere Defekte zu melden.

## Die Entscheidung

Der Nutzer hat am 260802-1735 Möglichkeit 1 aus `decisions/260802-1428_a_was-l4-mit-wiederhergestellten-tabs-meint.md` gewählt: der Kaltstart aus L4 ist abgeschlossen, wenn Fenster, Tabs, Leisten und die erste Bildschirmseite jedes sichtbaren Tabs stehen und die Tastatur reagiert. Das vollständige Lesen fällt unter L3 beziehungsweise L10. Punkt zwei des Shaper-Nachtrags vom 260802-1445 ist mitentschieden: dieselbe Lesart gilt für den Tabwechsel aus L5, ausdrücklich als eine Regel für beide Fälle. Keine der zehn Zahlen aus C8 ändert sich.

## Die Messbedingung für L4 und L5

Die Formulierung der Sitzungslage hat der Nutzer dem Shaper überlassen. Gewählt ist eine Prüfsitzung aus zwei Dateifenstern mit je zwei Tabs: im ersten Fenster ist der Tab auf Prüfordner A sichtbar und der auf B im Hintergrund, im zweiten Fenster umgekehrt, die Auswahl jeweils auf dem ersten Eintrag, Lesezeichenleiste und Vorschaufenster eingeblendet, die Fensterbreiten im Auslieferungszustand. A und B sind zwei nach demselben Verfahren erzeugte flache Ordner mit je 10.000 Einträgen an verschiedenen Pfaden.

Der Nachtrag hatte "zwei Dateifenster mit je einem Tab auf dem Prüfordner mit 10.000 Einträgen" vorgeschlagen. Zwei Erweiterungen sind hinzugekommen, beide aus demselben Grund, der Wiederholbarkeit der Messung. Zwei verschiedene Ordner verhindern, dass der zweite Lesevorgang eines Kaltstarts aus dem Cache des Systems bedient wird und L4 zur Hälfte warm gemessen wird. Der zweite Tab je Fenster gibt L5 einen Zieltab; ohne ihn bräuchte der Tabwechsel eine eigene Sitzungslage, und eine zweite Lage wäre die Sonderregel, die die Maxime "supersimpel" ausschließt.

## Eine Folgerung, die keine neue Frage geworden ist

Wechselt der Nutzer auf einen Tab, dessen Ordner KRK noch nicht gelesen hat, kann die erste Bildschirmseite nicht in den 50 ms aus L5 stehen: L2 veranschlagt für genau diese erste Bildschirmseite 100 ms. Der Spec schreibt deshalb aus, dass L5 den Wechsel selbst zusagt, die erste Bildschirmseite des Zielordners unter L2 fällt und das vollständige Lesen unter L3 beziehungsweise L10. Der Fall bleibt der Ausnahmefall, weil KRK nach dem Erreichen der bedienbaren Oberfläche weiterliest. Die Folgerung ergibt sich aus der Entscheidung und den vorhandenen Zahlen; sie ändert keine davon und ist dem Nutzer im Abschlussbericht offengelegt.

## Was geändert wurde

**Spec `planning/260802-1036_o_spec-navigator-geruest.md`** (Marker unverändert `_o_`):
- Kopfzeilen Datum und Status auf 260802-1735, keine offene Nutzerfrage mehr.
- Gatehinweis: der Block "Stand 260802-1445" ist durch "Stand 260802-1735" ersetzt und nennt die vier für den Plan neuen Stellen.
- C8 Messbedingungen: drei Prüfordner statt zwei, mit Begründung für den zweiten 10.000er; neuer Punkt "Sitzungslage für L4 und L5".
- C8 Tabelle: Zeilen L4 und L5 auf die entschiedene Lesart, Zahlen unverändert.
- C8 neuer Absatz "Was L4 und L5 als abgeschlossen zählt" samt der Folgerung zu L2.
- `## Offen für den Planner`: der Punkt zur Automatisierung der Messungen nennt jetzt drei Prüfordner und die Herstellung der Prüfsitzung.
- `## Abgleich mit der Circle-Directive`: der Absatz über die zwei abweichenden Stellen im Grounding snapshot ist durch den Stand 260802-1735 ersetzt.
- `## Offene Nutzerentscheidungen`: der erste Absatz meldet die Frage als beantwortet statt als offen.

**Circle-Datensatz `_t_circle.md`**, ausschließlich `## Grounding snapshot`:
- Ausgangslage: Nachtrag, dass `CLAUDE.md` existiert und die Wahl von Sprache und UI-Werkzeugkasten seit 260802-1150 getroffen ist (Rust mit AppKit über `objc2`).
- Bedienmodell: Nachtrag, dass Shift+Delete ab Werk unbelegt ist und wie das Löschen stattdessen belegt ist (gemeldete Stelle 1).
- Eintrag zum Löschentscheid: "Fn+F8" durch "F8" ersetzt, beide Cmd-Kürzel ergänzt (gemeldete Stelle 2).
- Liste der Entscheidungsdatensätze im Circle: von zwei auf fünf ergänzt, mit Stand je Datensatz.
- "Was der Aktivierungs-Spec zusätzlich festlegen muss": als erledigt markiert, C8 erfüllt die Forderung.
- Stand-Datum des Unterabschnitts "Offene Entscheidungen" auf 260802-1735.

**Datensätze und Defekte:**
- `decisions/260802-1428_o_was-l4-...` → `_a_`, mit `Answered:`-Block, Pfadzitat auf `planning/260802-1036_o_spec-navigator-geruest.md`:249 und Status `answered`.
- `issues/260802-1445_o_grounding-snapshot-...` → `_c_`, mit `Resolved:`-Block, der auch die drei zusätzlich nachgezogenen Stellen nennt.
- `decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`: Statusfeld von `open` auf `answered` gezogen, passend zum Marker; ein überholter Pfad in den Querverweisen auf `260802-1036_a_leistungszusagen-navigator.md` berichtigt.

## Was offen bleibt

Der Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` gehört dem Planner und ist nur gelesen worden. Er führt die L4-Frage an fünf Stellen als offen (Schritte S8, S21, S22, die Risikotabelle und der Abschnitt `## Offene Fragen`) und kennt weder die Prüfsitzung noch den zweiten 10.000er-Prüfordner. Beides gehört in die Messschritte S3, S8 und S21 und in den Erzeugungsschritt für die Prüfordner.

`portfolio.md` gehört dem Playmaker und nennt vier offene Entscheidungsdatensätze samt zweier Pfade mit überholtem Marker. Der Stand des Briefings ist der 260802-0853; es wird beim nächsten Playmaker-Lauf neu erzeugt.

Der Abschnitt `## Activation proposal` im Circle-Datensatz nennt ebenfalls Pfade mit überholten Markern. Er ist ein datierter Bericht des Playmakers vom 260802-0853 und bleibt bewusst unverändert.

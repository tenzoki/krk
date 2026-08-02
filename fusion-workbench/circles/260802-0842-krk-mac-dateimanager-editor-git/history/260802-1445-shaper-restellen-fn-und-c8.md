# Shaper — Reststellen der Fn-Korrektur, Planfeld und die beiden Planner-Punkte (260802-1445)

**Modus:** in-Circle clarification, mit ausdrücklicher Freigabe für den Circle-Datensatz
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Status:** Complete

## Auftrag

Der Nutzer hat die Freigabe für die drei Reststellen erteilt, die die Runde vom 260802-1423 gemeldet hatte, und die vom Shaper selbst vorgeschlagene Reihenfolge bestätigt: erst der Circle-Datensatz, dann der Spec, damit der Abschnitt `## Abgleich mit der Circle-Directive` nur einmal angefasst wird. Zwei Punkte kamen hinzu. Der Plan für Runde 1 liegt seit dem 260802-1428 vor und gehört zusammen mit dem Spec in das Feld `**Active spec/plan:**`. Der Planner hat drei Punkte als eigene Dateien abgelegt, zwei davon berühren den Spec; für sie war begründet zu entscheiden, ob der Spec angepasst wird oder die Klärung offen bleiben muss. Der dritte Punkt, die Verfügbarkeitsprüfung für macOS-26-Schnittstellen, ist eine reine Technikfrage und war nicht anzufassen. Weitere Grenzen: kein Plan, kein Code, keine Technologiefestlegung, kein Zugriff auf `spikes/`, kein Commit, der Spec bleibt offen.

## Der Circle-Datensatz

**Stelle 1, Abschnitt `## Directive`.** Der Satz zum Löschen lautet jetzt: "Die Taste Delete und Cmd+Delete räumen in den Papierkorb, F8 und Cmd+Opt+Delete löschen endgültig und fragen dabei einmal je Vorgang nach." Die Schreibweise "Fn+F8" ist damit auch aus diesem Satz verschwunden, und die Directive behauptet keine Unterscheidung mehr, die die Messung ausschließt.

Der gemeldete Umfang verlangte "Fn+F8" durch "F8" zu ersetzen und Cmd+Opt+Delete zu ergänzen. Der Satz nennt jetzt zusätzlich Cmd+Delete für den Papierkorb. Die Erweiterung um dieses eine Kürzel ist bewusst: C3 führt in seiner Kürzeltabelle beide Wege, und ein Satz, der nur das Kürzel für das endgültige Löschen nennt, legt eine Asymmetrie nahe, die es nicht gibt. Die Freigabe galt diesem Satz, und das Ziel des Defekts war ausdrücklich, ihn auf den Stand von C3 zu ziehen.

**Stelle 2, Abschnitt `## Grounding snapshot`.** Die Zusammenfassung des F-Tasten-Entscheids führt weiterhin die Antwort des Nutzers vom 260802-1105 im Wortlaut und stellt den Nachtrag vom 260802-1409 daneben: die Tastencode-Sicht, das Entfallen der Zusage freier nackter Funktionstasten und den zweiten Weg über die Cmd-Kürzel. Die ursprüngliche Antwort ist nicht getilgt, wie der Defekt es verlangt, weil das Grounding auch die Entstehung trägt.

**Das Feld `**Active spec/plan:**`.** Es nennt jetzt beide Dateien mit ihrer Rolle: den Spec `planning/260802-1036_o_spec-navigator-geruest.md` für die Abnahmekriterien und den Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` für den Ausführungsstand. Die Begründung stand bereits in der Sitzung vom 260802-1423: ein Feld, das nur den Plan nennt, verliert den Zugang zu den Abnahmekriterien, ein Feld, das nur den Spec nennt, verliert den Ausführungsstand. Beide gelten gleichzeitig, solange der Plan offen ist.

Der Abschnitt `## Turn log` blieb unberührt. Es ist kein Turn abgeschlossen worden.

## Der Spec

**Der Abschnitt `## Abgleich mit der Circle-Directive`.** Er trägt jetzt eine Einleitung, die ihn als Geschichte des Abgleichs ausweist und den aktuellen Stand ans Ende verweist. Die drei gemeldeten Stellen stehen im Rückblick statt in der Gegenwart. Der Absatz zum 260802-1127 zitiert seinen Wortlaut als überholte Fassung. Der Absatz zum 260802-1409 hält die Abweichung und ihre Behebung am 260802-1423 fest, nennt den neuen Wortlaut und den Defekt unter seinem geschlossenen Namen. Der neue Schlussabsatz stellt fest, dass zwischen Spec und Directive keine Abweichung bekannt ist, und nennt die zwei Stellen im Grounding snapshot, die weiterhin abweichen und außerhalb der Freigabe lagen.

**Der Gatehinweis am Kopf.** Er meldet die Directive-Abweichung nicht mehr als bestehend. An ihre Stelle tritt ein Hinweis auf die eine Frage, die seit dem 260802-1428 wieder offen ist. Die Kopfzeile `**Status:**` sagt nicht mehr "keine Frage dieser Runde offen"; das stimmte seit der Ablage des L4-Datensatzes nicht mehr.

**Die Messbedingungen in C8.** Der Prüfordner-Satz nennt jetzt beide Größen mit ihrer Zuordnung, einen Ordner mit 10.000 Einträgen für L2 und L3 und einen mit 100.000 für L10, und verlangt, dass beide nach demselben Verfahren entstehen, das bei gleicher Eingabe dieselbe Zusammensetzung liefert. Wie diese Reproduzierbarkeit hergestellt wird, bleibt Sache des Plans.

**Der Abschnitt `## Offene Nutzerentscheidungen`.** Er sagte "Keine" und nennt jetzt die L4-Frage mit ihrem Widerspruch, dem Datensatz und der Feststellung, dass der Plan nicht blockiert ist, die Abnahme aber schon.

## Wie mit der C8-Lücke umgegangen wurde

Der Defekt zum fehlenden Prüfordner für L10 ist geschlossen, ohne den Nutzer zu fragen. Die Prüfung war die Zuordnung zu den Kategorien aus dem Shaper-Auftrag: eine Lücke ist nur dann eine Nutzerentscheidung, wenn mehrere gültige Möglichkeiten mit unterschiedlichen Folgen bestehen. Hier bestehen keine. Die Zusage L10 nennt die Größe 100.000 seit dem 260802-1105 unverändert und ist vom Nutzer bestätigt; die zehn Zahlen standen nicht zur Debatte. Fehlte allein die Beschreibung der Bedingung, unter der gemessen wird, und für diese Beschreibung gibt es keine zweite vertretbare Fassung: entweder die Messbedingungen benennen den Ordner, auf dem L10 misst, oder L10 ist nicht nachprüfbar. Der Fall ist damit ein Mangel des Textes, kein Entscheidungspunkt.

Zwei Grenzen wurden dabei eingehalten. Die Formulierung sagt Reproduzierbarkeit als Abnahmebedingung zu und nennt kein Verfahren; der Erzeuger mit festem Startwert des Zufallsgenerators aus Schritt S3 des Plans bleibt die Antwort des Planners und wird im Spec nicht wiederholt. Und die Zahl 100.000 ist nicht angetastet worden, weil sie eine bestätigte Zusage ist.

## Wie mit dem L4-Widerspruch umgegangen wurde

Der Datensatz `decisions/260802-1428_o_was-l4-mit-wiederhergestellten-tabs-meint.md` bleibt offen und ist um einen Nachtrag des Shapers erweitert worden, statt einen zweiten Datensatz danebenzustellen. Die drei Möglichkeiten des Planners tragen und sind nicht ergänzt worden; die Empfehlung für Möglichkeit 1 bleibt unwidersprochen und trägt jetzt zusätzlich die Begründung des Shapers aus dem Aufbau von C8.

Drei Punkte sind hinzugekommen, alle drei aus der Autorschaft an C8 und C1:

**C8 nennt keine Sitzungslage, auf der L4 gemessen wird.** Die Messbedingungen legen Referenzgerät, Cache-Zustand, Prüfordner und zwanzig Wiederholungen fest. L4 ist die einzige der zehn Zusagen, deren Messung an einem Zustand hängt, den ein vorheriger Lauf hinterlassen hat. Auch unter der milderen Lesart wächst die Startlast mit der Zahl der sichtbaren Tabs. Die Antwort muss deshalb einen Satz für die Messbedingungen mitliefern, sonst besteht der Grund fort, aus dem der Planner die Frage überhaupt gestellt hat.

**Die Frage reicht bis L5.** C1 lässt beliebig viele Tabs je Dateifenster zu, sichtbar ist höchstens einer je Fenster. Möglichkeit 1 spricht von "jedem sichtbaren Tab" und stellt damit die Anschlussfrage, wann ein Tab im Hintergrund gelesen wird. Wird er erst beim Hinwechseln gelesen, gerät L5 mit seinen 50 ms in dieselbe Klemme wie L4. Die Ausweitung ist bewusst in denselben Datensatz gezogen worden: es ist derselbe Begriff an derselben Stufengrenze, und zwei Datensätze würden zwei Antworten erlauben, die einander widersprechen können.

**Was die Antwort am Spec ändert, ist eng begrenzt:** die Zeile L4 in der Tabelle von C8, ein Satz unter den Messbedingungen und, falls der zweite Punkt mitbeantwortet wird, die Zeile L5. C1 bleibt unberührt, und keine der zehn Zahlen ändert sich.

Der dritte Planner-Punkt, `decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`, ist nicht angefasst worden.

## Geschlossene Defekte

- `260802-1425_c_circle-datensatz-wiederholt-die-ueberholte-fn-zusage-an-zwei-stellen.md`
- `260802-1425_c_spec-meldet-die-directive-abweichung-noch-als-offen.md`
- `260802-1428_c_messbedingungen-c8-nennen-keinen-pruefordner-fuer-l10.md`

## Neu gemeldete Defekte

- `260802-1445_o_grounding-snapshot-traegt-den-loeschstand-an-zwei-stellen-ueberholt.md`
- `260802-1445_o_plan-nennt-die-c8-luecke-und-zwei-defekte-noch-als-offen.md`

Beim Nachziehen sind zwei weitere Stellen im Abschnitt `## Grounding snapshot` aufgefallen, die den Löschstand überholt wiedergeben. Der Absatz "Bedienmodell" nennt Shift+Delete als ausgeliefert, obwohl C3 die Kombination seit dem 260802-1105 ab Werk unbelegt lässt. In derselben Aufzählung, aus der Stelle 2 stammt, schreibt der Nachbareintrag zum Löschentscheid weiterhin "Fn+F8".

Beide sind gemeldet statt behoben worden, obwohl die zweite nur eine Ersetzung von drei Zeichen verlangt und im selben Abschnitt liegt, den diese Runde bearbeiten durfte. Die Freigabe nannte drei Stellen, so wie sie gemeldet waren, und eine Freigabe, die sich beim Arbeiten von selbst ausweitet, ist keine. Die Runde vom 260802-1423 hat aus demselben Grund dieselbe Entscheidung getroffen.

## Was nicht angefasst wurde

Der Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` ist gelesen und nicht geändert worden. Er führt an drei Stellen einen überholten Stand: die C8-Lücke als bestehend, denselben Defekt als offen und die beiden Defekte vom 260802-1417 als offen. Der Plan gehört dem Planner, deshalb ist der Befund als eigener Defekt abgelegt statt behoben worden. Die Aussage des Abschnitts "Zwei gemeldete Defekte, die den Plan nicht ändern", dass für die Tastenbelegung C3 gilt, ist unverändert richtig. Das Verzeichnis `spikes/` ist unberührt, der Spec trägt weiterhin den Marker für offen, und es ist nicht committet worden.

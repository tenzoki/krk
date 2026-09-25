# Shaper — Korrektur des Circle-Datensatzes (260802-1423)

**Modus:** in-Circle clarification, mit ausdrücklicher Freigabe für den Circle-Datensatz
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Status:** Complete

## Auftrag

Der Orchestrator hat die Modus-Grenze für zwei gemeldete Defekte am Circle-Datensatz aufgehoben und drei Arbeiten beauftragt: die Directive-Zeile zur Tastenbelegung neu formulieren, die Kopffelder auf den tatsächlichen Stand ziehen und den bis dahin leeren Abschnitt `## Turn log` erstmals füllen. Die Freigabe war eng gefasst: ein einziger Satz im Abschnitt `## Directive`, die Kopffelder, der Turn-Log. Kein Zugriff auf `planning/`, kein Zugriff auf `spikes/`, kein Commit.

## Was geändert wurde

### Die Directive-Zeile zur Tastenbelegung

Der alte Wortlaut lautete: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die die Norton-Reihe auf Fn+F3 bis Fn+F8 legt und die nackten Funktionstasten frei lässt."

Der neue lautet: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die jede Funktion der Norton-Reihe auf zwei Wegen erreichbar macht, über die Funktionstasten F3 bis F8 und über ein Cmd-Kürzel."

Zwei Mängel sind damit behoben. Die Zusage der freien nackten Funktionstasten ist entfallen, weil die Messung vom 260802-1338 belegt, dass KRK Fn+F3 und ein nacktes F3 nicht unterscheiden kann: beide erzeugen den Tastencode 99 mit gesetztem Modifikator `function`. Eine Belegung, die den einen Weg trifft und den anderen frei lässt, ist technisch nicht herstellbar, und eine Directive darf keine Zusage tragen, deren Messung sie als unerfüllbar ausweist. Der zweite Weg über die Cmd-Kürzel, den der Nutzer am 260802-1400 bestellt hat, steht jetzt in der Zeile.

Die Zeile nennt bewusst weder die konkreten Kürzel noch die Beschriftungsregel für die Belegungsansicht. Eine Directive prognostiziert den Zustand des fertigen Artefakts; die Einzelheiten stehen in C3 des Specs und bleiben dort. Der Satz trägt jetzt fünf Zeilen weniger Inhalt als die vorgeschlagene Fassung im Defektdatensatz und dennoch beide Aussagen.

### Die Kopffelder

`**Status:**` stand auf `anticipated`, während der Dateiname seit der Aktivierung am 260802-0913 den Marker `_t_` für aktiv trägt. Das Feld steht jetzt auf `active`. Nach `rules/fusion-workbench-conventions.md` ist der Marker im Dateinamen der maßgebliche Zustand; das Kopffeld ist seine menschenlesbare Wiederholung und lief hier auseinander.

`**Active session history:**` meldete `(none yet)` und zeigt jetzt auf `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1014-orchestrator-session.md`. Das Verzeichnis `history/` enthält acht Protokolle, aber nur eines davon ist eine Sitzung, die den Circle führt. Die übrigen sieben sind Sitzungen einzelner Agenten innerhalb dieser Orchestrator-Sitzung, darunter diese hier. Das Feld benennt die führende Sitzung, nicht die jüngste Datei.

`**Active spec/plan:**` zeigt unverändert auf `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`. Die Datei liegt dort und trägt den Marker für offen; das Feld stimmt und blieb unangetastet. `**Domain:** code` und `**Filed by:** shaper (anticipated-circle mode)` sind ebenfalls richtig. Das zweite Feld hält fest, wer den Circle angelegt hat, nicht wer ihn zuletzt bearbeitet hat, und wird deshalb von dieser Bearbeitung nicht berührt.

Ein Plan für Runde 1 lag am Ende der Bearbeitung noch nicht im Verzeichnis `planning/`. Sobald er dort liegt, gehört er in das Feld `**Active spec/plan:**`. Die begründete Form dafür ist die Nennung beider Dateien: der Spec sagt, was gebaut werden soll, der Plan sagt, in welcher Reihenfolge und mit welchen Schritten, und beide gelten gleichzeitig, solange der Plan offen ist. Ein Feld, das nur den Plan nennt, verliert den Zugang zu den Abnahmekriterien; ein Feld, das nur den Spec nennt, verliert den Ausführungsstand.

### Der Turn-Log

Der Abschnitt war leer und trägt jetzt einen Eintrag, der als Vorlauf gekennzeichnet ist und keine Turn-Nummer beansprucht. Er fasst die sechs Commits c0682ff bis f865fca zusammen, nennt jeden mit seinem Beitrag und verweist auf die Sitzungshistorie. Die Kennzeichnung als Vorlauf ist keine Formsache: diese Arbeit hat den Spec erstellt und die Eingangsfragen geklärt, aber keine Aufgabenliste abgearbeitet und keine Kohärenzprüfung durchlaufen. Sie als Turn 1 zu führen, würde die spätere Zählung um eins verschieben und einen Kohärenzstand behaupten, den es nicht gibt.

## Was nicht geändert wurde, und warum

Drei Stellen tragen die überholte Fn-Fassung weiter. Alle drei liegen außerhalb der Freigabe dieser Runde, und alle drei sind als Defekt gemeldet, statt still hingenommen zu werden.

Im Circle-Datensatz selbst sind es zwei: der auf die korrigierte Zeile folgende Satz "Die Taste Delete räumt in den Papierkorb, Fn+F8 löscht endgültig und fragt dabei einmal je Vorgang nach", dessen Schreibweise "Fn+F8" dieselbe Unterscheidung behauptet, die die Messung ausschließt, und die Zusammenfassung des F-Tasten-Entscheids im Abschnitt `## Grounding snapshot`, die den Stand vor dem Nachtrag vom 260802-1409 wiedergibt. Gemeldet als `260802-1425_o_circle-datensatz-wiederholt-die-ueberholte-fn-zusage-an-zwei-stellen.md`.

Die dritte liegt im Spec. Der Abschnitt `## Abgleich mit der Circle-Directive` und der Gatehinweis am Kopf melden die Abweichung der Directive-Zeile noch als offen, was seit dieser Bearbeitung nicht mehr stimmt. Der Auftrag untersagte jeden schreibenden Zugriff auf `planning/`. Gemeldet als `260802-1425_o_spec-meldet-die-directive-abweichung-noch-als-offen.md`.

Die Reihenfolge zwischen den beiden neuen Defekten ist nicht beliebig. Der Spec-Abschnitt sollte erst nachgezogen werden, wenn der Circle-Datensatz vollständig auf dem Stand von C3 ist, sonst muss er zweimal angefasst werden.

## Geschlossene Defekte

- `260802-1417_c_directive-zeile-sagt-freie-funktionstasten-zu.md` — Abschnitt `Resolved:` angehängt, Marker von offen auf geschlossen gezogen.
- `260802-1417_c_circle-datensatz-status-widerspricht-dem-marker.md` — Abschnitt `Resolved:` angehängt, Marker von offen auf geschlossen gezogen.

## Neu gemeldete Defekte

- `260802-1425_o_circle-datensatz-wiederholt-die-ueberholte-fn-zusage-an-zwei-stellen.md`
- `260802-1425_o_spec-meldet-die-directive-abweichung-noch-als-offen.md`

Beide liegen im Circle, nicht im geteilten Speicher: sie sind aus der Directive dieses Circles entstanden und betreffen ausschließlich seine eigenen Artefakte.

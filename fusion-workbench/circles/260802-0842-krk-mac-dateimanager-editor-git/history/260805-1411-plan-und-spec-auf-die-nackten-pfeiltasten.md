# Plan und Spec auf die nackte Ordnernavigation gezogen, Entscheidungsdatensatz nachgeholt

**Status:** Complete
**Agent:** planner
**Datum:** 260805-1411
**Herkunft:** Nutzerauftrag vom 260805, Nachzug nach der Umbelegung durch den `ontocoder` (`history/260805-1356-ordnernavigation-auf-die-nackten-pfeiltasten.md`)
**Angefasst:** `planning/`, `decisions/`, diese Datei. Kein Eingriff in `crates/`, `resources/`, `issues/`, `xtask/`, `README.md`, `CLAUDE.md`. Nicht committet.

---

## Was der Auftrag verlangte

Der Nutzer hat die Ordnernavigation zum zweiten Mal umbelegt. `oeffnen` liegt auf `right` statt auf `cmd+right`, `ordner_aufwaerts` auf `left` neben dem unveränderten `cmd+up` statt auf `cmd+left`. Der `ontocoder` hat `resources/default-keymap.toml` bereits gezogen; Plan und Spec trugen noch die Fassung vom 260804-1122. Dazu fehlte für beide Umbelegungen ein Entscheidungsdatensatz.

## Wie viele Stellen es waren

Der `ontocoder` hat nach `cmd+left` und `cmd+right` gesucht und zwei Stellen im Spec und zwölf im Plan genannt. Die eigene Suche nach denselben Zeichenketten und zusätzlich nach Umschreibungen hat **24 Stellen** ergeben, die die alte Fassung trugen: 13 im Spec und 11 im Plan. Dazu kommen acht rein hinzufügende Änderungen, die nichts Altes trugen: zwei Kopfzeilen mit Datumsangaben, zwei Stand- beziehungsweise Nachzugsabsätze, ein neuer Absatz im Spec-Anhang, das Erhebungsdatum der Datensatzliste und zwei neue Einträge in dieser Liste.

**Spec, 13 Stellen.** Der Stand-Absatz vom 260804-1122, die Beschreibung von C2, drei Abnahmekriterien von C2, die tragende Festlegung von C2, die Sprungmarken-Festlegung von C2, ein Abnahmekriterium von C3, die Belegungsbilanz von C3, die Vorbild-Festlegung von C3, die Zusteller-Festlegung von C3, ein Abnahmekriterium von C5 und der Absatz zur Folgefrage der Bereichsbreiten in `## Offene Nutzerentscheidungen`.

**Plan, 11 Stellen.** Der Nachzugsabsatz vom 260804-1122, das Etikett der Kante `S11c → S13`, die Erläuterung dieser Kante, S11c als Ganzes (Tabelle, Wortlaut und Abnahmekriterium, gemeinsam durch eine Abweichungsnotiz aufgefangen), vier Absätze und das Abnahmekriterium von S13, zwei Stellen in S18 und der Defektverweis auf den Fokusvorbehalt.

### Fünf Stellen umschreiben die Belegung, ohne sie zu nennen

Diese fünf hätte eine Suche nach `cmd+left` und `cmd+right` nicht gefunden. Zwei davon waren sachlich falsch geworden und nicht nur ungenau, weil sie dem Kriterium der Ordnernavigation im selben Dokument widersprachen.

1. **Spec, C2, Abnahmekriterium 2.** Es lautete "Pfeiltasten bewegen die Auswahl um einen Eintrag". Mit dem Links- und dem Rechts-Pfeil auf der Ordnernavigation widersprach das Kriterium dem unmittelbar folgenden. Es steht jetzt auf dem Auf- und dem Ab-Pfeil und sagt ausdrücklich, dass die Seitwärtspfeile die Auswahl nicht bewegen.
2. **Spec, C2, Beschreibung.** Derselbe Fall in Prosa: "Die Auswahl bewegt sich mit den Pfeiltasten." Der Absatz teilt die vier Pfeile jetzt auf, senkrecht die Auswahl in der Liste, waagerecht der Weg im Verzeichnisbaum.
3. **Spec, C5, Abnahmekriterium.** "Innerhalb der Leiste bewegen die Pfeiltasten die Auswahl." Auch hier gilt es nur noch für den Auf- und den Ab-Pfeil. Das Kriterium sagt jetzt dazu, was die Seitwärtspfeile bei Fokus in der Leiste tun: nichts, weil die Ordnernavigation den Wirkungsbereich "Dateifenster" trägt.
4. **Plan, S13, der Absatz zur Sprungmarke.** Er beschreibt den Rückfall für "jede Taste ohne Zusatztaste, die keiner Funktion gehört". Bis zum 260805 fielen `left` und `right` dort hinein, seither treffen sie eine Funktion und erreichen ihn nicht mehr. Die Regel selbst bleibt richtig; der Absatz sagt jetzt, dass sich der Kreis der betroffenen Tasten verkleinert hat.
5. **Plan, Kante `S11c → S13`.** Ihr Etikett lautete `cmd+left und cmd+right stehen`. Es lautet jetzt `Ordnernavigation steht in der Belegung`, weil die Voraussetzung von S13 die Belegung ist und nicht eine bestimmte Taste darin. Bei zwei Umbelegungen in zwei Tagen ist das der Unterschied zwischen einem Etikett, das hält, und einem, das mitwandern muss. Diese Stelle nannte die Kombinationen zwar, gehört aber hierher, weil die Lehre dieselbe ist.

## Wie der Grund für die nackten Pfeile formuliert ist

Die heutigen Stellen begründeten `cmd+left` mit einem Vorbild: die Seitwärtspfeile sind die Richtung, in der die Ordner nebeneinanderliegen, und ForkLift wie die Norton-Reihe legen den Auf- und Abstieg dorthin. Diese Aussage betrifft die **Richtung** und sagt über die Zusatztaste nichts. Sie hätte `cmd+left` genauso getragen wie das nackte `left`, und stehen zu lassen hieße, die neue Belegung mit einem Argument zu begründen, das die alte nicht von ihr unterscheidet.

Der tragende Grund lautet deshalb jetzt: **eine Ordnernavigation ohne Zusatztaste ist schneller als eine mit.** Ein Ordnerwechsel kostet einen Tastendruck statt zweier, so wenig wie die Bewegung der Auswahl in der Liste, und "superschnell" steht als erste Maxime des Vorhabens in `idea.txt`. Das Richtungsargument bleibt daneben stehen, unverändert gültig und ausdrücklich als zweites gekennzeichnet.

Die Formulierung ist mit dem Kommentar abgeglichen, den der `ontocoder` an `ordner_aufwaerts` in `resources/default-keymap.toml` geschrieben hat, damit Daten, Plan und Spec denselben Grund nennen und nicht drei verwandte.

## Der Marker des neuen Datensatzes

`decisions/260805-1411_a_ordnernavigation-mit-oder-ohne-zusatztaste.md`, angelegt als `_o_` und umgehend auf `_a_` gezogen.

**`_i_` wäre falsch.** `rules/fusion-workbench-conventions.md` verlangt für `_i_` die realisierte Umsetzung mit Beleg. Beim ersten Entwurf dieses Datensatzes fehlten zwei Stücke; während des Nachziehens ist eines davon eingetroffen. Der `coder` hat die Belegungsprüfung um 260805-1420 an der Wurzel behoben, `cargo test -p krk-core --test belegung` meldet seither 32 von 32, und der Defekt `issues/260805-1356_*_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md` ist geschlossen. Übrig bleibt der zweite Grund, und er trägt allein: **der Bedienversuch am laufenden Bündel steht aus.** Ob die nackten Pfeile ein- und aussteigen und ob der Links-Pfeil in der Pfadeingabe die Schreibmarke bewegt, verlangt Tastendrücke in einem sichtbaren Fenster und ist von keinem Agenten geprüft. Genau dort kann die Umbelegung schiefgehen, weil der Fokusvorbehalt für die nackte Taste abgeleitet und nicht gemessen ist. `_i_` ist terminal und nicht zurücknehmbar; ihn zu setzen, bevor die Bedienung einmal gesehen wurde, nähme dem Marker seine Aussage.

Der Datensatz trägt beide Antworten auf dieselbe Frage, die vom 260804-1122 und die vom 260805-1356, und hält den Weg fest, den `oeffnen` in zwei Tagen genommen hat: `return`, `cmd+right`, `right`. Ein Diagramm im Datensatz zeigt beide Ketten und die Prüfung, die bei jedem Übergang gebrochen ist.

## Die zwei älteren Entscheidungsdatensätze bleiben stehen

Beide nennen `cmd+left` in ihrer Begründung, und beide Antworten sind nachgeprüft unberührt.

`decisions/260804-1122_*_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` antwortet, dass `bereich_verbreitern` auf `ctrl+right` und `bereich_verschmaelern` auf `ctrl+left` wandern. Beide Einträge stehen unverändert in der Belegungsdatei, und ein Eintrag mit `ctrl` davor ist ein anderer als der nackte Pfeil, auch wenn beide dieselbe Taste tragen. Die Nachbarschaft, die jener Datensatz beschreibt, besteht weiter; sie hat nur ihre eine Seite gewechselt, von `cmd` auf gar keine Zusatztaste. Der Spec schreibt das in `## Offene Nutzerentscheidungen` aus, damit ein Leser den Datensatz nicht für überholt hält.

`decisions/260805-0000_*_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` antwortet, dass die Menükürzel in die Konflikterkennung einziehen. Die Antwort hängt an der Zustellerregel und an keiner Kombination. Zeile 49 benutzt `cmd+left` als **Beispiel** für die Trennung im Textfeld; das Beispiel ist gealtert, die Aussage nicht. Der Absatz bleibt stehen, weil ein Datensatz beschreibt, was zum Zeitpunkt seiner Antwort galt. Den heutigen Fall trägt jetzt der Spec in C3, wo dieselbe Trennung am nackten Links-Pfeil erklärt ist.

## Was nicht angefasst wurde

Kein `[DONE]`-Vermerk ist geändert. S11c behält seine Vorher-Nachher-Tabelle, den Wortlaut des Nutzerentscheids vom 260804 und sein Abnahmekriterium; eine Abweichungsnotiz darunter benennt den heutigen Stand. Die vorausschauenden Stellen in S13 und S18 sind dagegen auf den heutigen Stand gezogen, weil sie die Belegungsdatei wiedergeben und keine eigene Festlegung tragen. Die Regel steht im Nachzugsabsatz des Plans ausgeschrieben.

Nicht angefasst sind `crates/`, `resources/`, `issues/`, `xtask/`, `README.md` und `CLAUDE.md`. Nicht committet.

## Was dabei auffiel und außerhalb dieses Auftrags liegt

Der Plan hängt an anderer Stelle nach. Sein Kopf war zuletzt am 260805-0838 gezogen, während seither S16c, S17b und S17c abgenommen wurden und ein Aufräumdurchgang am 260805-0952 neun Code-Defekte geschlossen hat. Die Aufstellung `## Angelegte Defekte und Entscheidungen` gibt den Stand vom 260805-0838 wieder und ist von hier aus nur um die zwei Datensätze der Umbelegung ergänzt worden. Ein eigener Nachzug dafür steht aus; er gehört nicht in diesen Auftrag.

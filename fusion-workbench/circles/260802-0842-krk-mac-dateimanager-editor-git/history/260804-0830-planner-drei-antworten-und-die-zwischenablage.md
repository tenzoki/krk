# Drei beantwortete Entscheidungen und zwei neue Funktionen

**Datum:** 2026-08-04, 08:30
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Berührt:** Spec `planning/260802-1036_o_spec-navigator-geruest.md`, Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, drei Entscheidungsdatensätze, drei Defekte

---

## Auftrag

Der Nutzer hat drei offene Punkte entschieden und zwei zusätzliche Funktionen beauftragt. Beides war in Spec und Plan einzuarbeiten. Kein Eingriff in `crates/`, `resources/`, `xtask/`, `README.md` oder `CLAUDE.md`; kein `[DONE]`-Vermerk und keine der zehn Zahlen aus C8 zu ändern; nicht zu committen. Alle drei Grenzen sind eingehalten, nachgeprüft über `git status` und über einen Diff-Filter auf `[DONE]` und auf die Zeilen L1 bis L10.

## Was entschieden wurde

**Das letzte Fenster.** Der Nutzer hat Möglichkeit 2 aus `decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md` gewählt, gegen die Empfehlung jenes Datensatzes: einen Menüeintrag mit Kürzel und `applicationShouldHandleReopen:`. Der Datensatz steht auf beantwortet, nicht auf umgesetzt.

**Die Fehleranzeige.** Möglichkeit 1 aus `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md`: eine Statuszeile am Fuß des Dateifensters, ein Abbruch mit Hinweisfenster allein beim fehlenden Tastenabgriff. Ebenfalls auf beantwortet.

**Cmd+Y.** `issues/260803-2317_c_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md` ist mit der Begründung des Nutzers geschlossen: F3 trägt die Vorschau, das Cmd-Kürzel ist der zweite Weg, und die Belegung ist ab Werk änderbar. Der Datensatz hält dabei fest, was die Entscheidung **nicht** beauftragt: Weg 1 hatte eine Umrechnung in der Belegungsansicht als Preis genannt, und die entfällt. S20 ist unverändert.

## Die zwei Folgefragen zum zweiten Fenster

Möglichkeit 2 wirft zwei Fragen auf, die Runde 1 nie gestellt hat: ob zwei Fenster sich eine Sitzung teilen, und was "das aktive Dateifenster" aus C1 bedeutet, wenn es zwei Fenster mit je zwei Dateifenstern gibt. **Beide binden Runde 1 nicht**, und der Grund liegt im Zuschnitt der gewählten Antwort, nicht in einer Vertagung.

Das Fenster überlebt sein Schließen bereits: `setReleasedWhenClosed(false)` ist gesetzt, und der Anwendungsdelegierte hält es weiter. Der neue Menüeintrag holt dieses eine Fenster zurück, statt eines anzulegen. Damit entsteht kein zweites Fenster, und eine Frage, die ohne zweites Fenster nicht auftritt, ist in dieser Runde keine. Drei Stellen des Specs stützen den Zuschnitt: C1 beschreibt zwei Dateifenster als Bereiche nebeneinander, C7 sichert das Ein- und Ausblenden dieser Bereiche innerhalb des einen Fensters zu, und die Prüfsitzung aus C8 ist für ein Fenster beschrieben. Zwei Fenster machten L4 mehrdeutig, weil unklar wäre, welches Fenster die bedienbare Oberfläche herstellt.

Statt zweier neuer Entscheidungsdatensätze steht die Annahme deshalb ausgeschrieben: **KRK hält in Runde 1 genau ein Anwendungsfenster, und der Menüeintrag löst allein die Sackgasse auf.** Sie steht in S12 des Plans und als Festlegung in C7 des Specs.

Der Preis ist die Beschriftung. Ein Eintrag namens "Neues Fenster" legte auf dem Mac üblicherweise eines an, und dieser legt keines an; er heißt in Runde 1 **"Fenster einblenden"**. Die Runde, die mehrere Fenster einführt, benennt ihn um, beantwortet dabei die beiden Folgefragen und behält das Kürzel.

## Die Statuszeile: eine Zuordnung und eine Richtigstellung

**Zugeordnet ist sie S12.** Das ist der Schritt, der die vier Bereiche und die beiden Dateifenster anlegt; die Statuszeile sitzt am Fuß des Dateifensters und gehört in dieselbe Ansicht. Ein eigener Schritt daneben hätte für das Layout desselben Bereichs eine zweite Partei geschaffen.

`crates/krk-core/src/ablage/mod.rs` bündelt den Ausgabeweg bereits in `ablage::melden`. S12 löst ein, was der Modulkopf dort ankündigt: `melden` gibt den Text zurück, statt ihn zu schreiben, und der Aufrufer in `krk-ui` setzt ihn in die Zeile. Die Aufrufrichtung bleibt von oben nach unten, eine zweite Abhängigkeitsumkehr entsteht nicht, und der Schichtungsgraph aus `## Aufbau` behält seine Zahlen.

**Beim Einarbeiten ist ein Irrtum aufgefallen, der zwei Datensätze durchzogen hat.** Der Entscheidungsdatensatz begründet Möglichkeit 1 damit, C1 verlange die Statuszeile ohnehin, und stützt darauf auch seine Randbedingung, eine zweite Anzeigefläche wäre eine zweite Wahrheit. **Der Spec sagt das nicht.** Eine Textsuche über den ganzen Spec am 260804 findet weder "Statuszeile" noch "Lesefortschritt" noch eine Zusage über die Zahl der Einträge; die sechs Abnahmekriterien von C1 regeln Tabs, aktives Fenster, Standardordner, Sitzungswiederherstellung und getrennte Auswahl. Die Behauptung stammt aus `issues/260803-1536_c_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md` und ist von dort in den Entscheidungsdatensatz gewandert, ohne dass jemand sie gegen den Spec gehalten hat.

Für die Wahl ändert das nichts, für ihre Buchführung schon. Die Statuszeile ist eine Erweiterung des Umfangs und steht seit dem 260804-0830 als eigenes Abnahmekriterium in C1, mit dem Vermerk, woher sie kommt. Beide Datensätze halten den Irrtum jetzt fest.

**Der Abbruch beim fehlenden Tastenabgriff** bekommt mit **S6b** einen eigenen kleinen Schritt, wie der Datensatz es vorzeichnet. Er betrifft allein den `None`-Zweig in `crates/krk-ui/src/appkit/anwendung.rs` und hängt an keinem Bauteil aus S12; S6 selbst bleibt abgenommen und unverändert.

## Die zwei neuen Funktionen

**Sie stehen als eigene Fähigkeit C10 und nicht als Erweiterung von C6 und C2.** Der Grund ist die geteilte Mitte: beide hängen an derselben Auswertung dessen, was in der Zwischenablage steht, und diese Auswertung an zwei Stellen zu beschreiben hieße, zwei Wahrheiten darüber zu führen, was KRK für einen Pfad und was für eine Adresse hält. C2 und C6 tragen je einen Verweis auf C10 statt einer Kopie der Kriterien.

Die Auswertung hat einen Eingang und drei Ausgänge. Ein absoluter lokaler Pfad, gleich ob gewöhnlich oder als `file:`-Verweis geschrieben, führt zum Sprung. Eine Adresse mit `http:` oder `https:` geht an den Systembrowser. Alles andere meldet die Statuszeile. **Die Beschränkung auf zwei Schemata folgt aus C9:** gäbe KRK ein `smb:` oder `ftp:` an das System weiter, baute es über einen Umweg die Serververbindung auf, die C9 ausschließt. Die Regel "absoluter Pfad" ist von C2 geerbt und keine eigene.

**Im Plan wachsen sie in S13 und S19 hinein, wie vorgeschlagen, und der Plantext trägt die Zuordnung.** Der Sprung gehört zu S13, weil `kommandos/pfadeingabe.rs` dort ohnehin entsteht und genau das tut, was der Sprung braucht: einen Pfad prüfen, bei Erfolg navigieren, sonst den Grund melden. Der Unterschied ist allein die Herkunft des Wertes. Dieselbe Form hat der Plan bei `ordner_neu_lesen` schon gewählt: eine Funktion, zwei Auslöser. Die Vorschau gehört zu S19, weil sie eine zweite Quelle für den aktiven Vorschau-Tab ist und keine zweite Anzeigefläche; die Anzeige folgt der Dreiteilung, die C6 für eine Datei bereits kennt.

Ein eigener Schritt war trotzdem nötig, und zwar für die Daten: **S9b** trägt die drei neuen Kombinationen in `resources/default-keymap.toml` ein. Die Datei gehört dem `ontocoder`, S12, S13 und S19 gehören dem `coder`, und ein Schritt trägt genau einen Ausführenden. Drei getrennte Datenänderungen an derselben Datei wären dieselbe Arbeit dreimal geöffnet.

Geprüft, dass die drei Kennungen vor ihren Kommandos landen dürfen: `crates/krk-core/src/tasten/belegung.rs` führt die Aufzählung `Kommando` ausdrücklich als die Teilmenge der Funktionen, zu denen es schon eine Ausführung gibt, und `Kommando::aus_kennung` liefert für jede andere Kennung `None`. S9b bricht damit weder den Bau noch eine Prüfung.

## Die Circle-Grenze und der eigene Browser

Beides ist ausgeschrieben, in C10 und in `## Außerhalb des gesamten Circles`. Das Übergeben einer Adresse an den Systembrowser ist kein integrierter Browser: KRK zeigt keinen Web-Inhalt, hält keinen Verlauf und trägt keine Ansicht dafür. Ein eigener Browser läge außerhalb, weil der Circle-Datensatz ihn ausdrücklich ausschließt; er wäre ein eigener Circle und keine spätere Runde dieses Circles. Ob er als anticipated Circle festgehalten wird, ist dem Nutzer gemeldet und hier nicht entschieden.

## Angelegte Datensätze

- `decisions/260804-0830_o_was-die-zwischenablage-auswertung-liest.md` — liest die Auswertung nur den Text, oder auch den Dateiverweis, den der Finder beim Kopieren ablegt? Bindet S13, nicht die Abnahmekriterien aus C10. Empfohlen ist, den Dateiverweis mitzulesen, weil Cmd+C im Finder der naheliegendste Weg ist, einen Pfad in die Zwischenablage zu bringen, und genau er keinen Text-Pfad liefert.
- `issues/260804-0830_o_s13-nennt-fuer-die-kommando-aufzaehlung-die-falsche-datei.md` — S13 wies die Aufzählung `Kommando` `tasten/mod.rs` zu; sie steht seit S11 in `tasten/belegung.rs`. Aufgefallen, weil die beiden neuen Kommandos der Zwischenablage in dieselbe Aufzählung wachsen.

## Was der Nutzer noch anders sehen kann

Drei Punkte sind Wahlen des Planners und im Plan unter `## Offene Fragen` als solche benannt: die Beschriftung "Fenster einblenden" statt "Neues Fenster", die Flüchtigkeit der Zwischenablage-Vorschau, die das Halteverhalten aus C6 mit sich bringt, und die Frage nach dem anticipated Circle für den eigenen Browser.

## Zahlen nachgerechnet

- Erstes Spec-Diagramm: 9 Knoten, 20 Kanten, Verhältnis 2,22 nach 7 Knoten und 15 Kanten vorher. Ausgangsgrad 7 am Knoten `Tastenbelegung`, 4 am neuen Knoten `Zwischenablage des Systems`. Kein neuer Zyklus.
- Abhängigkeitsgraph der Schritte: 26 Knoten, 38 Kanten, Verhältnis 1,46 nach 24 und 34. Zyklenfrei, jede Kante von der kleineren zur größeren Nummer.
- Schichtungsgraph und Ladepfad unverändert.
- `resources/default-keymap.toml` wächst mit S9b von 46 auf 49 Funktionen und von 52 auf 55 Kombinationen. Die Datei selbst ist nicht angefasst.
- Die drei vorgeschlagenen Kombinationen `cmd+n`, `shift+f3` und `opt+cmd+g` sind gegen alle 52 ausgelieferten geprüft: keine kollidiert.

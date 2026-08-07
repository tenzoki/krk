L9 ist seit dem 260805 messbar schlechter geworden

---

Der Abnahmelauf vom 260807-1538 (`messungen/260807-1538-alle-zusagen.txt`) ist
der erste vollständige Lauf nach den 20 Commits der Sitzung 260806-2257. Neun
der zehn Zusagen halten in allen fünf Runden. **L9 verfehlt, und der Anteil ist
gegenüber dem 260805 um rund 14 Prozentpunkte gefallen.**

| Runde | 1 | 2 | 3 | 4 | 5 | Urteil nach der Fassung vom 260807-0832 |
|---|---|---|---|---|---|---|
| 260805-2207 | 90 % | 85 % | 90 % | 100 % | 85 % | gehalten in allen fünf |
| 260807-1538 | 90 % | 75 % | 80 % | 65 % | 70 % | **gehalten in 1 von 5** |

Gefordert sind 85 Prozent im ersten Bild. Der Mittelwert fällt von 90 auf 76
Prozent, und keine Runde außer der ersten hält.

**Die zweite Hälfte der Zusage hält durchgehend.** Der größte Einzelwert liegt
je Runde bei 1,13 / 1,20 / 1,26 / 1,26 / 1,70 Bildlängen, alle unter der Grenze
von zwei. Keine Eingabe verfehlt das zweite Bild. Schlechter geworden ist allein
der Anteil, der das **erste** erreicht.

---

## Warum das mehr ist als eine verfehlte Zusage

Der Nutzer hat L9 am 260807-0832 von 95 auf 85 Prozent gesenkt
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`).
Die Zahl 85 ist der gemessene Boden der Reihe vom 260805: zwei der fünf Runden
lagen genau darauf. **Die Senkung war damit an einen Zustand kalibriert, den die
Anwendung inzwischen verlassen hat.** Der `planner` hat beim Nachzug
ausdrücklich vermerkt, die neue Zusage habe keinen Spielraum; sie hat seither
auch keinen Boden mehr.

Der beschränkte Abschluss der Runde 1 vom 260807-1035 hat genau diesen Fall
vorhergesagt: sieben Zusagen standen auf Zahlen von vor den Änderungen, und drei
Commits hatten danach gemessene Wege berührt. Dieser Befund ist die Einlösung
der Vorhersage und kein neuer Zufall.

## Was geprüft und ausgeschlossen ist

**Die Sprache des Bündels scheidet aus.** Naheliegend wäre gewesen, dass
`880cb70` den Größenformatierer auf Deutsch umstellt und der Fortschrittstext
der Statuszeile dadurch teurer wird. Nachgesehen: der Fortschritt formatiert
über die handgeschriebene Funktion `menge` in
`crates/krk-ui/src/kommandos/operationen.rs:691` und nicht über
`NSByteCountFormatter`. Die Bündelsprache erreicht diesen Pfad nicht.

**Der Befehlsweg als solcher scheidet aus.** L1 misst denselben Tastendruck
(`auswahl_runter`) auf demselben Weg, nur ohne laufende Kopie, und hält bei
100 Prozent in allen fünf Runden bei einem Höchstwert von 0,99 Bildlängen. Was
L9 von L1 unterscheidet, ist allein die laufende Kopie.

## Die offene Vermutung

`inference:`, nicht gemessen. Verdächtig ist der vierte Anlass aus D5
(`2fbab30`): seit dieser Sitzung zieht jede **abgeschlossene** Dateioperation
die Gültigkeit der Lesezeichen nach. `Lesezeichen::gueltig`
(`crates/krk-core/src/ablage/lesezeichen.rs:51`) ist `self.ordner.is_dir()`,
also ein Systemaufruf je Lesezeichen, und er läuft auf dem Hauptfaden. Die
L9-Messstrecke bricht die Kopie nach jeder Eingabe ab und beginnt sie neu; je
Runde sind das zwanzig Abschlüsse. Der Aufräumdurchgang desselben Tages hat den
Anlass zudem verbreitert: `Leistenmodell::orte_setzen` prüft seither ebenfalls
über dieselbe Stelle.

Ob dieser Weg innerhalb der gemessenen Spanne liegt oder unmittelbar daneben,
ist nicht geprüft. Beides genügte, um den nächsten Zeichendurchgang zu
verschieben.

**Der Weg zur Klärung ist eine Gegenmessung**, keine Reparatur auf Verdacht: den
vierten Anlass vorübergehend stilllegen, L9 erneut fahren, die Anteile
vergleichen. Fällt der Unterschied weg, ist die Ursache benannt; bleibt er, ist
die Vermutung falsch und der nächste Verdächtige ist `5d7e299`.

## Die Gegenmessung ist ausgefallen: die Vermutung ist widerlegt

Nachgetragen am 260807 vom `coder`, vor dem Eingriff. **Die Prüfsitzung führt
null Lesezeichen.** Damit prüft `Leistenmodell::gueltigkeit_pruefen`
(`crates/krk-ui/src/leistenmodell.rs:243`) über eine leere Liste, ruft
`Lesezeichen::gueltig` kein einziges Mal, meldet keine Änderung und löst kein
`reloadData` aus (`crates/krk-ui/src/appkit/leiste.rs:237`). Der vierte Anlass
kostet je abgeschlossener Operation einen Funktionsaufruf über eine leere
Schleife. Er kann die vierzehn Prozentpunkte nicht erklären, und eine
Gegenmessung an ihm hätte nichts gemessen. Der Eingriff ist deshalb nicht
gebaut worden.

**Woher die Zahl kommt.** Die Lesezeichen stehen nicht in `session.toml`,
sondern in `bookmarks.toml` (`crates/krk-core/src/ablage/pfade.rs:54`), einer
eigenen der vier Ablagedateien. Die Messstrecke schreibt allein die Sitzung:
`Messplan::sitzung_herstellen` (`crates/krk-ui/src/messmodus.rs:288`) und die
`Sitzungssicherung` in `crates/krk-bench/src/messen.rs:1359` fassen
`bookmarks.toml` nicht an. Die Datei liegt im selben Ablageordner, den auch das
Bündel öffnet, `~/Library/Application Support/KRK/`, und **sie existiert dort
nicht**: der Ordner trägt am 260807 `session.toml` und `settings.toml` und
sonst nichts. Eine fehlende Datei liefert den Auslieferungszustand
(`crates/krk-core/src/ablage/mod.rs:228`), und der ist für die Lesezeichen die
leere Liste. Angelegt wird `bookmarks.toml` erst mit der ersten Änderung an den
Lesezeichen (`crates/krk-ui/src/appkit/anwendung.rs:897`); sie hat nie
stattgefunden.

~~`inference:` Geprüft ist der Ablagestand am 260807 nach dem Lauf, nicht der
zum Zeitpunkt des Laufs um 15:38.~~ **Vorbehalt erledigt.** Der Nutzer hat am
260807-1830 am frisch gebauten Bündel nachgesehen: unter der Überschrift
„Lesezeichen" in der linken Leiste steht keine Zeile. Die Zahl ist null, und
zwar an der laufenden Anwendung und nicht nur am Dateibestand. Der vierte
Anlass ist als Ursache damit gemessen ausgeschlossen und nicht mehr nur
erschlossen.

## Der Ausschluss oben ist zu eng gefasst

Der Abschnitt „Was geprüft und ausgeschlossen ist“ sagt, L9 unterscheide von L1
**allein** die laufende Kopie. Das trifft nicht zu. Der Messplan markiert vor
jeder L9-Runde alle Einträge des Prüfordners A
(`Handlung::AlleMarkieren`, `crates/krk-ui/src/messmodus.rs:858`), und A trägt
10.000 Einträge (`crates/krk-bench/src/fixture.rs:640`). L1 läuft auf demselben
Ordner ohne eine einzige Markierung. Zwischen den beiden Messungen stehen also
zwei Unterschiede, die laufende Kopie und die vollständige Markierung, und der
Befund trennt sie bislang nicht.

## Was als Nächstes zu prüfen wäre

`inference:`, keine Messung. Das Ordnungskriterium ist nicht „welcher Commit
hat den Tastenweg berührt“, sondern „was ist an L9 anders als an L1“ — denn L1
geht denselben Weg und hält bei 100 Prozent. Ein Commit, der den gemeinsamen
Weg verteuert, hätte L1 mitgenommen.

1. **`16e4558`, die sprachsensitive Kollation.** Der Kollationsschlüssel
   entsteht je Eintrag beim Lesen (`crates/krk-core/src/verzeichnis/kollation.rs`,
   Kopf). Kopieren schiebt die Auffrischung nicht auf
   (`auffrischung::schiebt_auffrischung_auf`), das rechte Fenster zeigt während
   der Messung das Kopierziel, und die Dateisystemwache liest es während der
   laufenden Kopie wiederholt neu. Jeder dieser Lesevorgänge baut seither ICU-
   Schlüssel, auf dem Lesefaden, während zwanzigtausend Dateien kopiert werden.
   L1 hat weder ein laufendes Kopierziel noch diese Lesevorgänge.
2. **Die Markierung selbst.** Ob das Zeichnen von 10.000 markierten Zeilen seit
   dem 260805 teurer geworden ist, ist offen; `3e9613a` und `ac95acf` haben
   `crates/krk-ui/src/appkit/tabelle.rs` angefasst. Gegen diesen Verdacht
   spricht, dass L1 dieselben Zeilen zeichnet, nur unmarkiert.
3. **`5d7e299`, `Tabliste::auswahl_auf_namen`.** Zuletzt, nicht zuerst: die
   L9-Eingabe läuft dort durch, aber die L1-Eingabe ebenso, und L1 hält.

Vor jedem weiteren Verdacht steht ohnehin die Trennung der beiden Unterschiede
aus dem Abschnitt darüber. Sie ist keine Sache des `coder`: sie hinge an einer
zusätzlichen Messgröße, und die Messstrecke ist der Maßstab, an dem der Befund
gemessen wird.

## Was daran hängt

Die Zusage L9 steht im Spec unter C8, und der Nutzer hat ihre Fassung erst am
260807 entschieden. Eine zweite Senkung binnen eines Tages wäre die
stillschweigende Lockerung, die C8 ausschließt. Die Entscheidung gehört dem
Nutzer und braucht die Gegenmessung als Grundlage.

**Zuständig:** ursprünglich `coder` für die Gegenmessung am vierten Anlass.
Die ist entfallen, weil die Vermutung vor dem Eingriff widerlegt war; der
Abschnitt „Die Gegenmessung ist ausgefallen“ trägt den Grund. Der Defekt bleibt
offen und braucht als Nächstes eine Entscheidung des Nutzers darüber, welcher
der drei genannten Verdächtigen gemessen wird und wie die beiden Unterschiede
zwischen L1 und L9 getrennt werden.

**Aufgefallen bei:** dem ersten vollständigen Abnahmelauf nach dem beschränkten
Abschluss der Runde 1, gefahren vom Nutzer am 260807-1538.

Cross-references:
`messungen/260807-1538-alle-zusagen.txt`,
`messungen/260805-2207-MacBookPro15-1-abnahme.txt`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_i_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-1730_c_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md`

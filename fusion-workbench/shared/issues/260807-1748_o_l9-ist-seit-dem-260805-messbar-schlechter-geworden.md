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

## Was daran hängt

Die Zusage L9 steht im Spec unter C8, und der Nutzer hat ihre Fassung erst am
260807 entschieden. Eine zweite Senkung binnen eines Tages wäre die
stillschweigende Lockerung, die C8 ausschließt. Die Entscheidung gehört dem
Nutzer und braucht die Gegenmessung als Grundlage.

**Zuständig:** `coder` für die Gegenmessung, danach der Nutzer für die
Entscheidung.

**Aufgefallen bei:** dem ersten vollständigen Abnahmelauf nach dem beschränkten
Abschluss der Runde 1, gefahren vom Nutzer am 260807-1538.

Cross-references:
`messungen/260807-1538-alle-zusagen.txt`,
`messungen/260805-2207-MacBookPro15-1-abnahme.txt`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-0014_i_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-1730_c_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md`

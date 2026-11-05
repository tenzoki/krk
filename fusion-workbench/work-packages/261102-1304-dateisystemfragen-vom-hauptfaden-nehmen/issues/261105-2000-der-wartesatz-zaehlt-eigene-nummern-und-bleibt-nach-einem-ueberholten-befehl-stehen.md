Der Wartesatz zählt eigene Nummern und bleibt nach einem überholten Befehl stehen
---
Schwere: Mittel. Nur auf einer hängenden Platte, also genau in der Lage, für die der Satz gebaut ist. Dort steht „wartet auf die Platte …“, während nichts mehr wartet, und verdeckt Fehlermeldungen und Zahlen, bis der hängende Faden zurückkommt, im schlimmsten Fall bis zum Programmende.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `Plattenwarte` führt eine eigene Nummer je `Plattenbefehl` (`crates/krk-ui/src/plattenwarte.rs:105` bis `:112`), und allein die Antwort der jüngsten eigenen Frage nimmt den Satz (`:143` bis `:150`). Ob ein Befehl noch gilt, entscheiden aber andere Werte: `leistenauswahl` mit `Leistenauswahlstand`, `Loeschbefehle`, `Notizordnerstand`, `Oeffnungsstand` und, je Dateifenster, `Pfadfrage` (`crates/krk-ui/src/appkit/tabelle.rs:760`). Die zwei Zählungen laufen auseinander, in beide Richtungen.

**Richtung 1, der Satz bleibt.**

- `leistenauswahl_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:3332`) erhöht die Nummer der Auswahl (`:3334`). Ein Gerät fragt nichts (`crates/krk-ui/src/leistenmodell.rs:867`) und läuft über den Zweig `:3343`, ohne `Plattenwarte` zu berühren. Fehlerbild, aus dem Quelltext abgeleitet und nicht gemessen: Lesezeichen auf einer getrennten Freigabe geklickt, nach einer Sekunde steht der Satz, dann Klick auf das Benutzerverzeichnis. Das Dateifenster zeigt das Benutzerverzeichnis, die Antwort der ersten Frage verfällt schon, und der Satz steht weiter.
- Der Löschweg ohne Faden: `loeschen_nach_rueckfrage` zieht eine neue Nummer (`anwendung.rs:8907`) und geht an einem fernen Ort direkt in die Einzugsmethode (`:8955`). Die hängende lokale Frage bleibt in `Plattenwarte` die jüngste.
- Tab, Dateifenster oder geöffnete Datei gewechselt: die Stände verwerfen die Antwort, der Satz bleibt.
- F4 auf `secrets.txt` und F2 haben nur einen Ausweg: dieselbe Taste, die wieder dieselbe Platte fragen kann.

**Was er verdeckt.** `Rang::ALLE` (`crates/krk-ui/src/appkit/statuszeile.rs:325`) stellt den Satz über `Fenstermeldung` und `Tabmeldung`, beide `Art::Fehler`, und über Filterstand, Seitenzähler und Markierungsstand. Der Filtertext selbst bleibt im Etikett der Bereichsleiste lesbar, die Zahlen dazu nicht. Benannt ist das Verdecken für eine Frage, die auf Dauer hängt (`statuszeile.rs:732`), und der Modulkopf begründet es mit „ihre Antwort die einzige, auf die der Nutzer noch wartet“ (`plattenwarte.rs:29` bis `:30`). Für einen überholten Befehl stimmt die Begründung nicht: `HowTo.md:207` nennt ihn selbst überholt. Dem Nutzer vorgelegt war laut `261105-1826-keine-laufende-plattenfrage-meldet-sich-und-jeder-weitere-befehl-startet-einen-faden` die Frist, nicht diese Lebensdauer.

**Richtung 2, der Satz fällt zu früh.** `Plattenbefehl::Pfadsprung` ist ein Stand für beide Dateifenster (`plattenwarte.rs:71`, `tabelle.rs:3865`), die Pfadfrage gilt aber je Dateifenster. Hängt der Sprung links und antwortet danach einer rechts, fällt der Satz, während die Antwort links noch gilt und später noch wirkt. Das ist die harmlosere Richtung.

**Richtung.** Ich schlage vor, dass ein Satz fällt, sobald sein Befehl überholt ist, und nicht erst mit einer Antwort. Eine Möglichkeit, noch nicht zu Ende geprüft: die Befehle melden `Plattenwarte` das Überholen an den Stellen, an denen ihre eigene Nummer steigt oder ihr Stand wechselt; offen ist dabei, wie ein Tabwechsel den Weg dorthin findet. Die kleinere Alternative ist ein Rang unter `Tabmeldung` und `Fenstermeldung`; sie nähme die Fehler aus dem Schaden, ließe den falschen Satz aber stehen. Bleibt es beim jetzigen Verhalten, gehört das als Nutzerentscheid an den Modulkopf, mit der Begründung statt der jetzigen.

**Abnahme.** Eine Tafelprobe an `Plattenwarte` zeigt: ein überholter Befehl mit weiter hängender Frage meldet nichts mehr. Oder ein Entscheid des Nutzers nennt das Stehenbleiben ausdrücklich, und `plattenwarte.rs:29` sagt nichts mehr, was dem widerspricht. Die Lage an einer getrennten Freigabe prüft der Nutzer im Lauf: Lesezeichen klicken, Benutzerverzeichnis klicken, Statuszeile lesen.

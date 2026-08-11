# Shaper: Nachzug am Spec der Runde 4 nach der Bewertung `conceptrev`

**Datum:** 2026-08-11 16:14
**Circle:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`
**Modus:** in-Circle, eng begrenzte Nacharbeit am eigenen Spec
**Datei:** `planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` (Marker bleibt `_o_`)
**Anlass:** Abnahme des Nutzers am 260811-1610 unter der Auflage, das Diagramm nachzuziehen. Grundlage ist `reviews/260811-1604-conceptrev-spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md`, Spruch `acceptable`.

## Was geändert wurde

**Auflage 1, zwei fehlende Kanten an `K3`.** Der Graph führte den Öffner ohne Bezug auf `betroffene()` und ohne Meldeweg, obwohl C3 beides an insgesamt vier Stellen zusagt. Neu:

- `K3 -->|"nur auf dem Tastenweg"| BE` — die Beschriftung trägt den zweiten Unterschied zwischen Taste und Doppelklick. Die Richtung folgt der bestehenden Kante `K2 --> BE`, damit im Graphen durchgehend „Befehl nutzt Geerbtes" steht. Die vom `conceptrev` vorgeschlagene Form `W --> BE --> K3` hätte `BE` von einem Herkunftsknoten zu einer Verarbeitungsstufe gemacht und damit zwei Bedeutungen derselben Kantenrichtung erzeugt.
- `K3 --> ST`, dazu die Beschriftung von `ST` von „der Pfad oder die Zahl" auf „der Pfad, die Zahl oder der Grund". Der dritte Fall ist die Abweisung durch das System aus C3.
- Die Kante `D --> K3` heißt jetzt „die Zeile ist keiner, und sie allein" statt „die Zeile ist keiner". Damit steht der Gegensatz zur Tastenkante im Bild und nicht nur in der Prosa.
- Ein Absatz unter dem Diagramm schreibt aus, dass zwei Wege in `K3` münden und sich in der Wirkungsmenge unterscheiden.

**Auflage 2, Cmd+W als vierte neue Belegung.** `K4` heißt jetzt „Tab schließen, der bestehende Befehl auf cmd+w, dessen Wirkungsbereich wächst", im Wortlaut angelehnt an `EIN`, das dieses Mittel schon trug. Die Kante `T -->|"cmd+w"| W` blieb unverändert; die Auflage nannte den Knoten als Ort der Berichtigung.

**Auflage 3, Zählfehler.** C2 nannte den Pfadkopierer den fünften Abnehmer von `betroffene()` und ließ offen, dass die Runde einen sechsten dazulegt. Berichtigt an zwei Stellen: im zweiten Abnahmekriterium von C2 und im Abschnitt `## Die Flüchtigkeit der Markierung`, wo „alle fünf Abnehmer der Markierung" auf sechs steht und die sechs jetzt benannt sind. Der Ausgangsstand von vier ist am Kommentar über `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:150-157`) nachgeprüft und stimmt.

**Zwei Antworten des Nutzers vom 260811-1610, jetzt als Zusage geführt.**

- Nur Text in der Zwischenablage, kein Dateiverweis (`decisions/260811-1552_a_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md`). Umgeschrieben in den Festlegungen von C1, in den Randbedingungen und im Abschnitt `## Was die Abnahme mitentscheidet`.
- `return` öffnet alle betroffenen Einträge (`decisions/260811-1612_a_oeffnet-return-alle-betroffenen-eintraege-oder-nur-den-unter-der-auswahl.md`). Umgeschrieben in den Festlegungen von C3, dort mit dem Vorbehalt des Datensatzes: ob KRK bei einer großen Zahl nachfragt, ist nicht entschieden.
- Beide als Zeile in der Tabelle `## Beantwortete Nutzerentscheidungen` nachgetragen, und die Quellenzeile im Kopf nennt jetzt sechs Antworten statt vier.

## Messwerte am Diagramm

| | Knoten | Kanten | beschriftete Kanten |
|---|---|---|---|
| vorher | 16 | 21 | 6 |
| nachher | 16 | 23 | 8 |

Gerendert mit `mmdc` 11.16.0 nach SVG und PNG, Bild angesehen. Die fünf Schichten halten ihre Reihenfolge, keine Kante läuft nach oben, keine Waise, kein Kreis.

## Was ausdrücklich nicht geändert wurde

Die vier geringfügigen Befunde der Bewertung (Nummern 3 bis 6) blieben stehen, weil die Auflage sie nicht nennt: `EIN` bleibt im Kasten „Die vier Befehle dieser Runde", die vier Kombinationen bleiben auf den Kanten von `T`, `EIN` bleibt ohne Ergebnisknoten, der Belegungsnachschlag bekommt keinen eigenen Knoten.

Kein Abnahmekriterium wurde gestrichen, hinzugefügt oder inhaltlich geändert; es bleiben 62. Ein einziges hat einen berichtigten Zusatz bekommen, das zweite in C2, und die Berichtigung betrifft die Ordnungszahl und nicht die Zusage.

## Ein Befund am Entwurf, der dem Nutzer vorgelegt und nicht stillschweigend geändert wurde

Im Abschnitt `## Die Flüchtigkeit der Markierung` steht: „Der Pfadkopierer ist der erste Abnehmer ohne Rückfrage." Der Satz ist wörtlich weiter wahr, unterschlägt aber, dass der Öffner der zweite ist und der folgenreichere von beiden. Nicht geändert; dem Nutzer gemeldet.

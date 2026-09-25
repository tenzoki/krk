# Shaper: Diagrammnachzug am Spec der Tastenbelegungs-Ausgabe

**Datum:** 2026-08-11 08:26
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Auftrag:** eng begrenzte Nacharbeit. Der Nutzer hat den Spec abgenommen, mit der Auflage, vorher das Diagramm nachzuziehen. Auslöser ist die Bewertung `reviews/260811-0803-conceptrev-spec-tastenbelegung-als-markdown-in-downloads.md`, Spruch `acceptable`.
**Angefasst:** `planning/260811-0753_o_spec-tastenbelegung-als-markdown-in-downloads.md`, der Mermaid-Block im Abschnitt `### Woher die Ausgabe ihre Daten nimmt, und woher nicht` sowie eine Zeilenangabe in C3. Marker bleibt `_o_`. Kein Code, kein Circle-Datensatz, kein Commit.

## Die drei Mängel und was daraus wurde

1. **Die Belegungsansicht fehlte als Abnehmer.** Sie steht jetzt als eigener Knoten `ANS` in einer Schicht "Zwei Abnehmer derselben Belegung", neben der Ausgabefunktion. **Die Gabelkante kommt aber von der Belegung und nicht vom Belegungsmodell**, anders als der Auftrag es vorzeichnete; die Begründung steht unten.
2. **Der stumme Zyklus `AUS → B → MOD → AUS`** ist weg: die Kante `AUS -->|"liest, ändert nicht"| B` ist ersatzlos gestrichen. Der Graph ist damit zyklenfrei, und das Rendern hält `TD` durch, ohne dass sich zwei Kanten kreuzen. Am Bild geprüft (mmdc, SVG und PNG).
3. **`anzeige()` steht nicht mehr als eigene Quelle**, sondern in der Beschriftung des Modellknotens. Spalte 1 und Spalte 2 verlassen den Graphen jetzt über **eine** Kante aus demselben Knoten, wie im Code: `Belegungsmodell::tastentext` ruft `anzeige` (`crates/krk-ui/src/belegungsmodell.rs:412`).

## Die Berichtigung im Fließtext

C3 nannte `anzeige()` bei `belegungsmodell.rs:527`. Die Signatur steht in **Zeile 530**, am Code nachgezählt. Berichtigt. `pfade.rs:71` stimmt und blieb.

## Der Befund, der über den Auftrag hinausgeht

Der Auftrag verlangte eine Kante `Belegungsmodell → Belegungsansicht`. **Die gibt es im Code nicht.** Außerhalb der Prüfmodule wird `Belegungsmodell::neu` genau einmal gerufen, in `belegung_ansehen` (`crates/krk-ui/src/appkit/anwendung.rs:2159`), und zwar über einer **Kopie** der geltenden Belegung. Die Bildschirmansicht arbeitet also nicht auf demselben Modell, das die Ausgabe benutzen wird, sondern auf einem zweiten Modell derselben Bauform über einem zweiten Wert. Eine Kante von `MOD` nach `ANS` hätte eine geteilte Instanz behauptet, die es nicht gibt, und wäre zugleich mit dem Knoten `BL` kollidiert, der genau diese zweite Instanz ist.

Die Gabelung sitzt deshalb eine Ebene höher, an der Belegung selbst: `B → MOD → AUS` und `B → BL → ANS`. Das ist die Struktur, die die Directive zusagt, denn ihr Versprechen gilt der Aufbereitung und nicht der Instanz: **eine Belegung, dieselbe Aufbereitung, zwei Abnehmer.** Der Teilgraph heißt deshalb "Dieselbe Aufbereitung, zweimal angelegt".

Der Gewinn dieser Fassung: dasselbe Bild trägt jetzt **beide** Aussagen des Specs. Die geteilte Quelle steht als Gabelung an `B`, und die Abweichung bei offener Belegungsansicht — der Preis, den der Nutzer am 260811-0115 gegen die Empfehlung angenommen hat — steht als der zweite Wert, aus dem der zweite Modellknoten entsteht. Vorher war die Abweichung allein an der gestrichelten Kante zu ahnen.

Am Entwurf liegt nichts davon. Der Entwurf ist stimmig; der Auftrag hatte die Gabelung nur eine Ebene zu tief vermutet.

## Maß

Knoten 10 vor, 10 nach dem Nachzug. Kanten 10 vor, 10 nach dem Nachzug. Zyklen 1 vor, 0 nach dem Nachzug. Teilgraphen 3 vor, 5 nach dem Nachzug; kein Knoten steht mehr außerhalb aller Schichten. Der Fließtext über und unter dem Bild bleibt wörtlich richtig: es führen weiterhin zwei Wege zu drei Spalten, und die gestrichelte Kante ist weiterhin die einzige, die keine Wirkung überträgt.

# Ein Entwurf über neue Auslieferungsfassungen wird zur vorgesehenen Runde

**Status:** Complete
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Der Nutzer hat über `/fusion:direct` einen Entwurf eingereicht: KRK soll melden, wenn seine
Auslieferungsfassung von `readers.toml` und `settings.toml` weitergegangen ist, und den
Unterschied zeigen. Der Anlass steht im Entwurf selbst: beide Dateien entstehen beim ersten
Start aus `resources/default-*.toml` und werden danach nie wieder geschrieben, also bekommt
niemand, der KRK schon einmal gestartet hat, die Neuerungen einer neuen Fassung zu sehen.

## Was geklärt wurde

Zwei Runden Rückfragen, acht Entscheidungen. Erste Runde: die Meldung deckt alle drei von Hand
gepflegten Dateien ab, also `keymap.toml` dazu; „weitergegangen" heisst jeder Unterschied zur
heutigen Auslieferungsfassung; die Meldung erscheint beim Start in der Statuszeile **und**
zusätzlich auf Abruf, das Blatt beim Start fällt; gezeigt werden Namen und dazu, auf eigene
Forderung des Nutzers, die Pfade der Dateien. Die Möglichkeit, dass KRK die fehlenden Einträge
selbst in die Nutzerdatei schreibt, hat der Nutzer verworfen.

Zweite Runde, vier weitere Entscheidungen, alle vier auf der Empfehlung: die Startzeile nennt
nur, was die Auslieferungsfassung führt und die Nutzerdatei nicht, während der Abruf jeden
Unterschied zeigt; die Zeile erscheint einmal je Fassung, wofür KRK sich die Versionsnummer
merkt; der Abruf ist ein Blatt über Taste und Hauptmenü; die eine Statuszeile trägt die Zahl
je Datei und den Ordner, die Namen bleiben dem Abruf.

## Was beim Lesen des Baums dazugekommen ist

Drei Befunde stehen im Abschnitt `## Grounding snapshot` des Datensatzes und sind für die
Planung bindend. Der Vergleich braucht kein Bündel, weil alle drei Auslieferungsfassungen über
`include_str!` einkompiliert danebenstehen. Die Startzeile trifft auf einen offenen Defekt: die
Startmeldungen überschreiben einander, und nur die letzte erreicht den Nutzer
(`260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`).
Und die Frage, wo KRK sich die gemeldete Fassung merkt, überschneidet sich mit einer offenen
Frage an `session.toml`
(`260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`);
wer hier schreibt, beantwortet jene mit.

Die Spannung, die der Nutzer in der ersten Runde ausdrücklich zur Behandlung aufgegeben hat,
ist im Datensatz aufgelöst: C1.6 der Runde 16 verbietet, dem Nutzer Profile unterzuschieben,
und diese Runde meldet, statt zu schreiben. Der Rest steht als angenommener Preis da.

## Ergebnis

Angelegt ist die vorgesehene Runde
`circles/260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap` mit dem Datensatz
`_a_circle.md` und den sechs Unterordnern. Kein Spec, kein Plan, keine Aktivierung: das ist der
nächste, getrennte Schritt des Nutzers.

# Shaper: der Spec der Belegungsausgabe

**Datum:** 2026-08-11, 07:53
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Status:** Abgeschlossen
**Agent:** shaper, als Unteragent dispatcht

## Was diese Sitzung geliefert hat

`planning/260811-0753_o_spec-tastenbelegung-als-markdown-in-downloads.md`: vier Fähigkeiten mit 38 Abnahmekriterien, dazu zwei Kriterien unter `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`, zusammen 40.

| Fähigkeit | Gegenstand | Kriterien |
|---|---|---|
| C1 | Der Auslöser ist ein Menüeintrag ohne Tastenkürzel | 6 |
| C2 | Ort, Name und das Überschreiben | 8 |
| C3 | Der Inhalt der Datei | 18 |
| C4 | Die Meldung nach dem Aufruf | 6 |

Die Zahl ist nachgezählt und nicht geschätzt, mit demselben Zählweg, den der Spec der Runde 2 in seinem Abschnitt `## Die vier später hinzugekommenen Fähigkeiten` festhält.

## Woraus der Spec entstanden ist

Zwölf Festlegungen des Nutzers tragen ihn. Fünf stehen als Datensätze unter `decisions/`, alle mit dem Marker beantwortet und je einer `Answered:`-Zeile: der Menüweg ohne Kürzel, der feste Name mit Überschreiben, nur die belegten Funktionen bei Gliederung nach Funktionsbereich, der Wirkungsbereich als dritte Spalte, und der gesicherte Stand bei offener Belegungsansicht.

Sieben stammen aus der Klärungsrunde vom 260811-0115 und stehen allein im Spec: der Dateiname `KRK-Tastenbelegung.md`, der Kopf ohne Zeitstempel, die ausgeschriebene Beschriftung der dritten Spalte, die Beschriftung "Textfelder und Editor" für die sechs Textbefehle, die Erfolgsmeldung mit vollem Pfad, keine gesonderte Meldung beim Überschreiben, und keine Meldung über den gesicherten Stand.

Die Grundlage am Code stammt aus dem vorigen Lauf und steht in `history/260811-0446-shaper-klaerungsrunde-tastenbelegung-ausgabe.md`. Diese Sitzung hat sie an vier Stellen ergänzt: die Bauform von `Funktionsbereich::name()` als Vorbild für die neue Fallunterscheidung, die neun Menüeinträge in `hauptmenue` samt ihren Standardselektoren, den Kommentarblock zu den sechs Textbefehlen in `resources/default-keymap.toml`, und die Zahlen des Abnahmelaufs vom 260810 für die Begründung zu L4.

## Zwei Vorbehalte, die der Spec ausdrücklich trägt

**Die Beschriftung der sechs Textbefehle steht auf einer Ableitung.** Der Shaper hat sie aus dem Aufbau der Antwortkette geschlossen und nicht gemessen. Der Spec kennzeichnet sie als solche und macht die Prüfung zum zwölften Abnahmekriterium von C3. Neu gegenüber dem vorigen Lauf ist der benannte Verdachtsfall: `text_alles_auswaehlen` liegt auf `selectAll:`, und die Lesezeichen- und Geräteleiste ist eine `NSTableView`, die diesen Selektor `inference:` von sich aus beantwortet. Trifft das zu, ist "Textfelder und Editor" für diesen einen Befehl falsch, und die Zelle wird berichtigt oder bleibt leer.

**Der Preis der Antwort auf die fünfte Frage ist ausgeschrieben statt geglättet.** Der Abschnitt `## Die Abweichung bei offener Belegungsansicht, und ihr Preis` hält fest, dass der Nutzer bei offener Ansicht eine Datei bekommt, die dem Schirm widerspricht, ohne es zu erfahren. Drei Eigenschaften begrenzen den Schaden, und alle drei stehen dort. Der Abschnitt hält auch fest, was den Fall ganz auflösen würde: ein bei stehendem Blatt gesperrter Menüeintrag. Ob er gesperrt ist, ist ungemessen und steht als fünftes Abnahmekriterium von C1.

## Was ohne Rückfrage festgehalten ist

Die Schreibweise der Kombinationen kommt aus `anzeige()`. Keine der vier bestehenden vollständigen Fallunterscheidungen wächst, und `resources/default-keymap.toml` wächst nicht, weil der Menüeintrag kein Kommando mitbringt. Eine neue vollständige Fallunterscheidung kommt hinzu, nämlich die Beschriftung der sieben Wirkungsbereiche, gebaut wie `Funktionsbereich::name()` und ohne Auffangzweig.

Keine der zehn Zeitzusagen aus C8 der Runde 1 ist berührt, und der Spec begründet es statt es zu behaupten: acht liegen auf Wegen, die diese Runde nicht anfasst, L1 und L9 hängen am Ereignisabgriff, den der Menüweg umgeht, und L4 ist der einzige Berührungspunkt, mit einem zusätzlichen Menüeintrag beim Start gegen einen gemessenen Abstand von rund 600 ms zur Zusage.

## Sechs Vorbelegungen des Specs

Der Nutzer hat drei der sieben Beschriftungen genannt. Für die vier übrigen, für den Menütitel und für die Einordnung unter den Menütitel "KRK" hat der Spec eine Vorbelegung gesetzt und sie unter `## Was die Abnahme mitentscheidet` als solche ausgewiesen. Jede ist eine Zeichenkette oder eine Zeile.

## Was nicht geschehen ist

Kein Entscheidungsdatensatz ist angelegt worden. Keine der Fragen, die beim Schreiben aufkamen, ist eine offene Nutzerentscheidung: die zwei ungemessenen Punkte sind Abnahmekriterien, die technischen Fragen stehen unter `## Offen für den Planner`, und die sechs Formfragen tragen eine Vorbelegung mit Ausweis.

Das Feld `**Active spec/plan:**` im Circle-Datensatz steht weiter auf `(none yet)`. Der Shaper ändert einen aktiven Circle-Datensatz nicht; das Nachziehen bleibt beim Orchestrator.

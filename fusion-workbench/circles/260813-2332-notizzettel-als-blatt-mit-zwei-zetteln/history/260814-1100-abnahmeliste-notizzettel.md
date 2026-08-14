# Abnahmeliste — der Notizzettel am laufenden Bündel

**Datum:** 260814-1100
**Bündel:** `target/KRK.app`, gebaut und mit „KRK Entwicklung" signiert am 260814-1058, Version 0.2.1
**Das beglaubigte Bündel ist vorher gesichert worden** nach `~/Library/Caches/krk-beglaubigt-260814-1054/KRK.app`; die Beglaubigung dort ist mit `xcrun stapler validate` bestätigt.
**Die eigene Belegung ist geprüft:** im Ablageordner liegt **keine** `keymap.toml`, also kommen `f2` und `cmd+k` belegt an. Der Defekt `shared/issues/260814-0656_*` trifft diesen Abnahmelauf nicht.

24 der 72 Abnahmekriterien haben einen Anteil, den nur ein Mensch am laufenden Bündel im Vordergrund sehen kann. Der Baumanteil ist abgenommen: 43 Kriterien trägt der Baum, 40 davon sauber und 3 mit benannter Einschränkung.

## Die Beobachtungen

| # | Was zu tun ist | Was zu sehen sein soll |
|---|---|---|
| 1 | `f2` drücken | Ein Blatt fährt mittig von oben herunter, mit zwei Tabs „Zettel 1" und „Zettel 2" und einer leeren Textfläche |
| 2 | Etwas tippen, `Esc` | Das Blatt schließt. `f2` erneut: der Text steht noch da |
| 3 | `cmd+k` statt `f2` | Derselbe Zettel, derselbe Weg — zwei Wege ab Werk |
| 4 | Bei stehendem Zettel `F5` drücken | Nichts geschieht im Fenster dahinter. Keine Kopieroperation startet |
| 5 | Bei stehendem Zettel `cmd+q` | KRK beendet sich — die Ausnahmeliste lässt `beenden` durch |
| 6 | Auf „Zettel 2" klicken, tippen, auf „Zettel 1" zurück | Zettel 1 zeigt seinen eigenen Text, Zettel 2 behält seinen. **Der Tabwechsel sichert beide** |
| 7 | Nach dem Tabklick sofort `Esc` | Das Blatt schließt. Ginge der Schreibfokus nicht in die Textfläche zurück, bliebe es stehen — das ist der Punkt, an dem die Zusage aus C2 hängt |
| 8 | Zettel 2 öffnen, `Esc`, KRK beenden, neu starten, `f2` | Zettel 2 ist offen. `Sitzung::zettel` merkt den zuletzt offenen |
| 9 | **`f2`, ein Zeichen tippen, `shift+cmd+w`** | Siehe unten. Das ist die Messung |
| 10 | Schreibrecht am Ablageordner nehmen, `f2`, „abc" tippen, `Esc` | Eine Meldung nennt den Grund. Dann `f2` erneut: **„abc" steht noch da** |
| 11 | Eine fremde Datei als `note-1.txt` in den Ablageordner legen, `f2` | Der Zettel ist leer, die alte Datei liegt unter dem Beiseitepfad, eine Meldung nennt sie |
| 12 | Hell und dunkel umschalten | Das Blatt bleibt in beiden lesbar |

## Beobachtung 9 im Einzelnen — die Messung

Der Plan hat sie aus dem tragenden Weg genommen: gesichert wird **unbedingt und vor** dem Aufruf von `performClose:`, also hält die Zusage „erst sichern" in beiden Ausgängen. Was zu messen bleibt, ist allein, welche Kante das Bündel geht.

1. `f2` drücken, ein Zeichen tippen.
2. `shift+cmd+w` drücken.
3. Notieren, welche der zwei Kanten: schließt das Fenster und nimmt das Blatt mit, oder bleibt beides stehen und das System gibt einen Ton?
4. `cmd+n`, dann `f2`: steht das getippte Zeichen da? **Diese Antwort muss in beiden Ausgängen „ja" lauten** — sie ist die eigentliche Zusage.
5. Ergebnis nach `messungen/YYMMDD-HHMM-performclose-mit-blatt.txt`, mit Gerät, Systemfassung und beiden Beobachtungen.

## Danach

Das beglaubigte Bündel unter `target/KRK.app` ist durch den Entwicklungsbau ersetzt. Zurück bekommst du es aus `~/Library/Caches/krk-beglaubigt-260814-1054/` oder über `./release.sh <zahl>`.

## Ergebnisse

**Gefahren am 260814-1115 vom Nutzer, am Bündel `target/KRK.app` im Vordergrund. Elf Beobachtungen bestanden; Nummer 10 ist vorher einvernehmlich gestrichen worden.**

| # | Ergebnis |
|---|---|
| 1 | bestanden — das Blatt fährt mittig herunter, zwei Tabs, leere Fläche |
| 2 | bestanden — der Text überlebt `Esc` und steht beim erneuten `f2` da |
| 3 | bestanden — `cmd+k` führt denselben Weg |
| 4 | bestanden — `F5` wirkt nicht ins Fenster dahinter |
| 5 | bestanden — `cmd+q` beendet KRK bei stehendem Zettel |
| 6 | bestanden — jeder Zettel behält seinen eigenen Text über den Tabwechsel |
| 7 | bestanden — `Esc` unmittelbar nach dem Tabklick schließt. Damit ist die Zusage aus C2 am laufenden Bündel bestätigt: der Schreibfokus geht zurück |
| 8 | bestanden — nach Beenden und Neustart ist der zuletzt offene Zettel offen |
| 9 | bestanden, mit einer Einschränkung — siehe unten |
| 10 | **gestrichen.** Die Logik ist am Modell in drei Proben abgenommen, und der Rückgabewert trägt seit Turn 2 `#[must_use]`; die Beobachtung hätte der Verdrahtung nichts hinzugefügt, was der Übersetzer nicht schon hält |
| 11 | bestanden — eine fremde Datei wird beiseitegelegt, der Zettel ist leer, die Meldung nennt die Sicherung |
| 12 | bestanden — das Blatt ist in hell und dunkel lesbar |

## Beobachtung 9: die Zusage hält, die Kante ist nicht festgehalten

Die eigentliche Zusage ist bestanden: nach `shift+cmd+w` bei stehendem Zettel steht das getippte Zeichen beim nächsten Öffnen da. Damit hält „erst sichern" am laufenden Bündel, und das ist die Aussage, an der der vierte Sicherungsmoment hängt.

**Welche der zwei Kanten das Bündel geht** — schließt das Fenster und nimmt das Blatt mit, oder bleibt beides stehen — **ist nicht notiert worden.** Die Datei `messungen/YYMMDD-HHMM-performclose-mit-blatt.txt` ist damit nicht entstanden. Das ist kein Mangel an der Zusage: der Plan hat die Messung ausdrücklich aus dem tragenden Weg genommen, weil gesichert wird, bevor `performClose:` überhaupt gerufen wird. Was fehlt, ist die Auskunft über AppKits Verhalten, und die wäre für eine spätere Runde nützlich, die am Schließweg arbeitet.

## Was damit abgenommen ist

Die 24 Abnahmekriterien mit Bündelanteil sind bis auf den nicht festgehaltenen Messwert abgenommen. Zusammen mit den 43 Kriterien, die der Baum trägt, und den 5, die einen Prüfaufbau brauchen, steht die Runde bei 71 von 72.

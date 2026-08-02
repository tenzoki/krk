# Prüfprogramm: Kommen Fn+F3 bis Fn+F8 als Tastenereignisse an?

**Wegwerf-Prüfcode, kein Produktcode.** Dieses Verzeichnis beantwortet eine einzelne
ungeprüfte Annahme und wird danach nicht weitergepflegt. Nichts hier gehört in KRK
übernommen: keine Architektur, keine Tests, keine Fehlerbehandlung über das Nötigste
hinaus. Ist die Frage beantwortet und im Spec festgehalten, kann das Verzeichnis weg.

## Die Frage

Der Spec legt in C3 fest, dass KRK die Norton-Funktionen ab Werk auf Fn+F3 bis Fn+F8
belegt, und begründet das damit, dass die nackten Funktionstasten auf einem
unveränderten Mac vom System verbraucht werden:

> Die Fn-Kombination ist gewählt, weil sie auf jedem Mac ankommt, ohne dass der Nutzer
> eine Systemeinstellung ändert.
>
> `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt C3

Der analyst hat festgehalten, dass diese Annahme nicht belegt ist:

> Dass Fn+F3 bis Fn+F8 auf einem unveränderten Mac als gewöhnliche Tastenereignisse
> ankommen, während die nackten Funktionstasten vom System verbraucht werden, ließ sich
> nicht belegen.
>
> `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`, Abschnitt `## Constraints`

Vier Teilfragen sind zu beantworten:

1. Kommen Fn+F3 bis Fn+F8 in einer gewöhnlichen macOS-Anwendung als normale
   Tastenereignisse an, und mit welchem `keyCode` und welchen Modifikatoren?
2. Kommen die **nackten** F3 bis F8 an, oder verbraucht sie das System vorher?
3. Ändert sich das Bild mit der Systemeinstellung "F1, F2 usw. als Standard-Funktionstasten
   verwenden"?
4. Löst die Fn-Taste selbst ein `flagsChanged`-Ereignis aus, und ist der Fn-Modifikator im
   `modifierFlags` sichtbar?

## Was ohne Tastendruck schon feststeht

### Geprüft (Kommando ausgeführt, Ausgabe gesehen)

| Was | Kommando | Ergebnis |
|---|---|---|
| Systemeinstellung "F1, F2 usw." | `defaults read -g com.apple.keyboard.fnState` | **nicht gesetzt** ("does not exist"), also Systemvorgabe. Die Einstellung ist AUS. |
| macOS | `sw_vers` | 15.7.7, Build 24G720 |
| Gerät | `sysctl -n hw.model` | `MacBookPro15,1` |
| Touch Bar vorhanden | `ioreg -c IOHIDDevice` | ja: `Touch Bar Display`, `TouchBarUserDevice`, `Touch Bar Backlight` |
| Angeschlossene Tastaturen | `ioreg -c IOHIDDevice` | nur `Apple Internal Keyboard / Trackpad`, keine externe |
| Touch-Bar-Anzeigemodus | `defaults read com.apple.touchbar.agent` | Domäne existiert nicht, also durchweg Auslieferungszustand |
| Werkzeugkette | `xcode-select -p`, `swift --version` | `/Library/Developer/CommandLineTools`, Swift 6.1.2. Das Programm hier baut damit ohne Xcode-Projekt, verifiziert. |

Zwei Punkte daraus sind für die Auswertung tragend.

**Der Messrechner ist das Abnahmegerät.** `MacBookPro15,1` ist genau das Referenzgerät aus
`decisions/260802-1036_a_leistungszusagen-navigator.md`. Was hier gemessen wird, gilt
unmittelbar für die Abnahme, nicht nur analog.

**Der Messrechner hat keine physische F-Tastenreihe.** Das 15-Zoll-Modell von 2018 trägt
einen Touch Bar. Im Auslieferungszustand zeigt der Touch Bar die App-Bedienelemente und den
Control Strip; erst das Halten der fn-Taste blendet F1 bis F12 als Schaltflächen ein. Damit
ändert sich die Bedeutung beider Fälle auf diesem Gerät:

```
Mac mit F-Tastenreihe          Referenzgerät (Touch Bar)
─────────────────────          ─────────────────────────
Fn+F3  = zwei Tasten           Fn+F3  = fn halten, F3 auf Glas antippen
F3     = eine Taste            F3     = existiert nicht, solange fn nicht gehalten wird
```

Für Frage 2 folgt daraus: auf diesem Gerät gibt es im Auslieferungszustand gar keine nackte
F3. Frage 2 ist hier nur messbar, wenn der Touch Bar dauerhaft auf F1 bis F12 gestellt wird,
oder an einer externen Tastatur. Deshalb kennt die Anleitung unten drei Durchgänge.

### Gefolgert (dokumentiert, in dieser Messung noch nicht bestätigt)

Diese Werte stammen aus den Systemkopfdateien und der AppKit-Dokumentation, **nicht** aus
einer Messung. Der Bericht des Programms stellt sie am Ende neben die gemessenen Werte;
weicht die Messung ab, gilt die Messung.

- Virtuelle Tastencodes (Carbon HIToolbox, `Events.h`): F1=122, F2=120, **F3=99**, F4=118,
  **F5=96**, F6=97, F7=98, **F8=100**, F9=101, F10=109, F11=103, F12=111, fn=63.
- Zeichen der F-Tasten (AppKit, `NSF1FunctionKey` fortlaufend): F3=U+F706, F4=U+F707,
  F5=U+F708, F6=U+F709, F7=U+F70A, F8=U+F70B.
- Der Modifikator `NSEvent.ModifierFlags.function` ist Bit 23, also `0x800000`.

**Die inhaltlich wichtigste Folgerung, und der eigentliche Grund für die Messung:** der
`function`-Modifikator wird nach der AppKit-Dokumentation bei *jeder* Taste aus dem
Funktionstasten-Unicodebereich gesetzt, also auch bei den Pfeiltasten, Pos1, Ende und den
F-Tasten selbst, unabhängig davon, ob die fn-Taste körperlich gedrückt war. Trifft das zu,
dann kann KRK Fn+F3 und ein nacktes F3 **nicht unterscheiden**: beide erzeugen `keyCode 99`
mit gesetztem `function`-Bit. Die ausgelieferte Belegung heißt in Wahrheit "keyCode 99" und
nicht "Fn+F3".

Für den Spec hat das zwei Seiten. Es **stützt** Abnahmekriterium 8 in C3, das genau darauf
beruht ("KRK unterscheidet die beiden Wege nicht und braucht dafür keine zweite Belegung").
Es legt zugleich ein Risiko offen, das C3 nicht nennt: schaltet der Nutzer die
Systemeinstellung ein, verbraucht das System vermutlich die Kombination fn+F3 für Mission
Control, und das Ereignis kommt dann von der nackten F3. Die Funktion löst weiter aus, aber
die Beschriftung "Fn+F3" in der Belegungsansicht wäre für diese Nutzer falsch. Beides prüft
Durchgang B. Solange die Messung aussteht, sind diese beiden Absätze Folgerungen, keine
Befunde.

## Bauen und starten

Voraussetzung sind nur die Command Line Tools. Kein Xcode, kein Projekt, keine
Abhängigkeiten. Das Programm braucht **keine** Freigabe für Bedienungshilfen, weil es einen
lokalen Ereignisabgriff verwendet und keinen globalen. Genau deshalb misst es das, worum es
geht: was eine gewöhnliche Anwendung im Vordergrund erhält.

```sh
cd spikes/fn-tasten
./starten.sh A          # baut und startet Durchgang A
```

Oder ohne das Skript:

```sh
swiftc -o beobachter beobachter.swift
./beobachter A
```

Das Fenster kommt von selbst in den Vordergrund und zeigt die Tastenfolge sowie jedes
ankommende Ereignis sofort als Zeile. Beenden mit **Cmd+Q** im Fenster oder mit Ctrl+C im
Terminal; beide Wege schreiben die Ergebnisdatei.

## Die Tastenfolge

Elf Tastendrücke je Durchgang. Die Buchstaben a, b und c sind Trennmarken: das Programm
zerlegt das Protokoll an ihnen und wertet die Abschnitte getrennt aus. Ohne sie ist das
Protokoll nicht zuzuordnen.

| # | Taste | Wozu |
|---|---|---|
| 1 | `a` | Trennmarke, zugleich Nachweis, dass keyDown überhaupt ankommt |
| 2 | Fn+F3 | Frage 1 |
| 3 | Fn+F5 | Frage 1 |
| 4 | Fn+F8 | Frage 1 |
| 5 | `b` | Trennmarke |
| 6 | F3 ohne Fn | Frage 2 |
| 7 | F5 ohne Fn | Frage 2 |
| 8 | F8 ohne Fn | Frage 2 |
| 9 | `c` | Trennmarke |
| 10 | Fn allein, drücken und loslassen | Frage 4 |
| 11 | Shift allein, drücken und loslassen | Kontrollprobe: beweist, dass flagsChanged ankommt |

Danach Cmd+Q.

**Gibt die Tastatur die Schritte 6 bis 8 nicht her**, also im Auslieferungszustand des
Touch Bar, dann statt der drei Tasten **einmal `x`** drücken. Der Bericht meldet den
Abschnitt dann als "übersprungen" statt als "nein". Ein übersprungener Abschnitt darf nicht
als Antwort auf Frage 2 gelesen werden.

Wird dort stattdessen fn gehalten, um die F-Tasten überhaupt zu erreichen, meldet der
Bericht "nicht messbar auf diesem Gerät". Auch das ist kein "nein": Abschnitt 2 hat dann
nur Abschnitt 1 wiederholt. Ob fn körperlich gedrückt war, entnimmt die Auswertung allein
den `flagsChanged` der Taste 63 — das `mod=`-Feld eines `keyDown` taugt dafür nicht, weil
AppKit `function` bei jeder Taste aus dem Funktionstasten-Unicodebereich setzt, auch bei
einer nackten F3 (siehe oben, "Gefolgert").

### Drei Durchgänge

| Durchgang | Zustand des Rechners | Beantwortet | Schritte 6 bis 8 |
|---|---|---|---|
| **A** | unverändert, so wie jetzt | Fragen 1 und 4 | mit `x` überspringen |
| **B** | Touch Bar dauerhaft auf F1 bis F12 | Frage 2 und 3 für dieses Gerät | normal drücken |
| **C** | externe Tastatur mit echter F-Tastenreihe, sonst unverändert | Fragen 1 bis 3 für den Normalfall | normal drücken |

Durchgang A ist der Pflichtteil und beantwortet die tragende Frage 1. B und C sind
Zusatzmessungen; C ist der Fall, den C3 eigentlich meint, und braucht eine externe Tastatur.

**Umschalten für Durchgang B:** Systemeinstellungen, Tastatur, Touch-Bar-Einstellungen,
"Touch Bar zeigt" auf "F1, F2 usw. als Tasten". Danach `./starten.sh B`. **Nach der Messung
zurückstellen**, sonst ist der Rechner für spätere Messungen nicht mehr im
Auslieferungszustand.

Bei einem Rechner mit echter F-Tastenreihe entspricht Durchgang B dem Einschalten von
"F1, F2 usw. als Standard-Funktionstasten verwenden" in denselben Einstellungen. Der Zustand
lässt sich mit `defaults read -g com.apple.keyboard.fnState` nachlesen; das Programm liest
ihn bei jedem Start selbst mit und schreibt ihn in den Kopf seines Berichts, damit keine
Messung ohne ihren Zustand dasteht.

### Was dabei passieren wird

- Ein nacktes F3 öffnet im Auslieferungszustand Mission Control. Das ist der erwartete
  Befund für Frage 2, kein Fehler. Mission Control schließen, das Fenster wieder anklicken,
  weitermachen.
- Ein nacktes F8 kann die Medienwiedergabe starten oder anhalten.
- Die fn-Taste allein öffnet auf manchen Rechnern die Zeichenauswahl. Kommt sie hoch,
  schließen und das Fenster wieder anklicken. Der Befund zu Frage 4 steht trotzdem im
  Protokoll, weil das `flagsChanged` vor der Zeichenauswahl eintrifft.

## Wo das Ergebnis landet

`spikes/fn-tasten/messung-<Durchgang>.txt`, also `messung-A.txt` und so fort, jeweils neben
dem Programm. Derselbe Bericht geht zusätzlich auf die Standardausgabe im Terminal.

Die Auswertung lässt sich ohne neue Messung wiederholen:

```sh
./beobachter --auswerten messung-A.txt
```

Das liest das rohe Ereignisprotokoll aus der Datei zurück, rechnet die Antworten neu und
schreibt sie auf die Standardausgabe. Die Quelldatei bleibt unangetastet. Wird die
Auswertung korrigiert, gehört das Ergebnis in eine Datei daneben — die Messdatei selbst ist
der Beleg und wird nicht überschrieben.

Der Bericht enthält vier Teile: den Kopf mit Zeitpunkt, macOS-Version und dem Wert von
`com.apple.keyboard.fnState`; das rohe Ereignisprotokoll mit Nummer, Zeit, Art, `keyCode`,
Zeichen, Modifikatoren und rohem Flag-Wert; die Auswertung nach den drei Abschnitten; und
die abgeleiteten Antworten auf die vier Fragen. Fehlen Trennmarken, meldet der Bericht
"nicht gemessen" statt eines Ergebnisses.

## Platz für das Ergebnis

Hier trägt der Nutzer oder der nächste Agent ein, was gemessen wurde. Bis dahin ist die
Annahme aus C3 unbelegt und der Plan darf nicht auf ihr aufsetzen.

**Durchgang A** (Auslieferungszustand, `fnState` nicht gesetzt), gemessen am: _offen_

| Frage | Befund | Beleg |
|---|---|---|
| 1. Fn+F3/F5/F8 kommen an? | _offen_ | `messung-A.txt` |
| 2. nackte F3/F5/F8 kommen an? | _offen_ | |
| 3. Wirkung der Systemeinstellung | _offen_ | |
| 4. fn löst flagsChanged aus? | _offen_ | |

**Durchgang B** (Touch Bar auf F1 bis F12), gemessen am: _offen_

| Frage | Befund | Beleg |
|---|---|---|
| 2. nackte F3/F5/F8 kommen an? | _offen_ | `messung-B.txt` |
| 3. Wirkung der Systemeinstellung | _offen_ | |

**Durchgang C** (externe Tastatur), gemessen am: _offen_

| Frage | Befund | Beleg |
|---|---|---|
| 1. Fn+F3/F5/F8 kommen an? | _offen_ | `messung-C.txt` |
| 2. nackte F3/F5/F8 kommen an? | _offen_ | |

**Folgen für C3** (erst ausfüllen, wenn die Durchgänge vorliegen):

- Trägt die Begründung "die Fn-Kombination kommt auf jedem Mac an" weiterhin? _offen_
- Bleibt Abnahmekriterium 8 in C3 richtig? _offen_
- Braucht C3 einen Zusatz zum Touch-Bar-Gerät? _offen_

## Dateien

| Datei | Was es ist |
|---|---|
| `beobachter.swift` | das Programm, eine Datei, ohne Abhängigkeiten |
| `starten.sh` | baut und startet, Durchgangsetikett als Argument |
| `messung-*.txt` | Ergebnisberichte, entstehen beim Beenden |
| `beobachter` | das gebaute Programm, nicht im Repository (siehe `.gitignore`) |

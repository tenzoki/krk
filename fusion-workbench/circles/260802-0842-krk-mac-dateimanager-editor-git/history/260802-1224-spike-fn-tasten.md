# Prüfprogramm zur Fn-Tasten-Annahme aus C3

**Agent:** coder
**Zeitpunkt:** 260802-1224 bis 260802-1233
**Status:** Complete
**Auftrag:** Prüfprogramm bauen, das die ungeprüfte Annahme aus C3 des Navigator-Specs
beantwortet: kommen Fn+F3 bis Fn+F8 als gewöhnliche Tastenereignisse an?

## Auslöser

`decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`, Abschnitt `## Constraints`,
letzter Punkt: die Annahme aus C3 ist nicht belegt und muss vor der Implementierung geprüft
werden. Der Nutzer hat Swift als Werkzeug vorgegeben, weil Rust auf dem Rechner nicht
installiert ist und die Frage die Ereigniszustellung durch macOS betrifft, nicht die
Sprache.

## Was entstanden ist

Alles unter `spikes/fn-tasten/` im Projektwurzelverzeichnis, als Wegwerf-Prüfcode
gekennzeichnet:

| Datei | Inhalt |
|---|---|
| `beobachter.swift` | AppKit-Programm, eine Datei, ohne Abhängigkeiten. Zeichnet jedes keyDown und flagsChanged mit Tastencode, Zeichen, Modifikatoren und Zeitpunkt auf und wertet die vorgegebene Tastenfolge beim Beenden selbst aus. |
| `starten.sh` | baut mit `swiftc` und startet, Durchgangsetikett als Argument |
| `README.md` | die Frage, die Anleitung, die ohne Tastendruck geklärten Punkte, Platz für das Ergebnis |
| `.gitignore` | schließt das gebaute Binärprogramm aus |

Kein Commit; der Orchestrator committet.

## Ohne Tastendruck geklärt

Geprüft, mit Kommando und gesehener Ausgabe:

- `defaults read -g com.apple.keyboard.fnState` meldet "does not exist". Die Einstellung
  "F1, F2 usw. als Standard-Funktionstasten verwenden" ist also AUS, im Auslieferungszustand.
- `sw_vers`: macOS 15.7.7, Build 24G720. `sysctl -n hw.model`: `MacBookPro15,1`.
- `ioreg -c IOHIDDevice`: das Gerät trägt einen Touch Bar (`Touch Bar Display`,
  `TouchBarUserDevice`, `Touch Bar Backlight`). Angeschlossen ist nur die interne Tastatur.
- `defaults read com.apple.touchbar.agent`: Domäne existiert nicht, also auch der Touch Bar
  im Auslieferungszustand.
- Werkzeugkette: `/Library/Developer/CommandLineTools`, Swift 6.1.2. Der Bau ohne
  Xcode-Projekt läuft durch, verifiziert.

Zwei Befunde daraus sind für den Spec erheblich.

**Der Messrechner ist das Abnahmegerät.** `MacBookPro15,1` ist das Referenzgerät aus
`decisions/260802-1036_a_leistungszusagen-navigator.md`. Die Messung gilt unmittelbar für
die Abnahme.

**Das Abnahmegerät hat keine physische F-Tastenreihe.** Auf einem Touch-Bar-Gerät heißt
"Fn+F3" nicht "zwei Tasten drücken", sondern "fn halten und F3 auf dem Glas antippen"; ohne
gehaltenes fn existiert im Auslieferungszustand gar keine F3. Das berührt C3 in seiner
Begründung ("weil sie auf jedem Mac ankommt") und in seiner Bedienbarkeit: eine
tastaturzentrierte Anwendung, deren Kopier-Befehl einen Blick auf den Touch Bar verlangt,
verfehlt die Maxime der Tastatursteuerung auf genau diesem Gerät. Das ist ein Befund für
den Spec, kein Implementierungsdetail. Der Auftrag untersagte Schreibzugriffe unter
`fusion-workbench/` außer dieser Historie, deshalb ist hier kein Issue abgelegt; der
Orchestrator entscheidet.

Gefolgert, nicht gemessen, im README als solches gekennzeichnet: die Tastencodes (F3=99,
F5=96, F8=100, fn=63), die Zeichen (U+F706 bis U+F70B) und `function` als Bit 23. Die
tragende Folgerung: der `function`-Modifikator wird laut Dokumentation bei jeder Taste aus
dem Funktionstasten-Unicodebereich gesetzt, unabhängig von der körperlichen fn-Taste. Trifft
das zu, kann KRK Fn+F3 und nacktes F3 nicht unterscheiden. Das stützt Abnahmekriterium 8 in
C3 und legt zugleich offen, dass die Beschriftung "Fn+F3" in der Belegungsansicht für
Nutzer mit eingeschalteter Systemeinstellung irreführend wäre.

## Entwurfsentscheidungen des Prüfprogramms

- **Lokaler Ereignisabgriff, kein globaler und kein Event Tap.** Damit braucht das Programm
  keine Freigabe für Bedienungshilfen und misst genau das, worum die Frage geht: was eine
  gewöhnliche Anwendung im Vordergrund erhält. Ein Event Tap würde mehr sehen und die Frage
  verfehlen.
- **Trennmarken a, b, c in der Tastenfolge.** Das Programm kann nicht wissen, ob der Nutzer
  fn gehalten hat. Die Marken zerlegen das Protokoll, sodass die Zuordnung ohne Vertrauen in
  die Reihenfolge funktioniert und auch bei ausgefallenen Ereignissen trägt.
- **Kontrollprobe mit Shift.** Kommt kein shift-Wechsel an, ist der Abgriff selbst
  unbewiesen und jedes "nein" im Bericht wertlos. Der Bericht sagt das dann ausdrücklich.
- **Leere und übersprungene Abschnitte sind kein "nein".** Ein abgebrochener Durchgang meldet
  "nicht gemessen", ein mit `x` bewusst übersprungener Abschnitt meldet "übersprungen". Ohne
  diese Unterscheidung läse sich ein früh beendeter Durchgang wie ein Messergebnis. Der
  Fehler war in der ersten Fassung vorhanden und wurde vor der Übergabe behoben.
- **Signalbehandlung für SIGINT und SIGTERM.** Ctrl+C im Terminal schreibt die Ergebnisdatei,
  statt die Messung zu verlieren.

## Verifiziert

- Bau mit `swiftc` ohne Xcode-Projekt: läuft durch, keine Warnungen.
- Start über `./starten.sh <Etikett>`: Fenster erscheint, kommt in den Vordergrund.
- Beenden über SIGTERM: schreibt `messung-<Etikett>.txt`, Exit-Code 0.
- Leerlauf-Durchgang ohne Tastendruck: meldet für die Fragen 1, 2 und 4 "NICHT GEMESSEN"
  statt eines falschen "nein".

Nicht verifiziert, weil ohne Tastendruck nicht möglich: die Auswertung eines echten
Durchgangs mit gefüllten Abschnitten.

## Offen

Die vier Fragen selbst. Sie beantwortet erst der Nutzer, indem er `./starten.sh A` startet
und die elf Tasten aus dem README drückt. Bis dahin bleibt die Annahme aus C3 unbelegt und
der Plan darf nicht auf ihr aufsetzen.

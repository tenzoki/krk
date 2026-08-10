# Prüfprogramm: Setzt AppKit `attachedSheet` vor dem Abschlussblock auf nil?

**Wegwerf-Prüfcode, kein Produktcode.** Dieses Verzeichnis beantwortet eine einzelne
ungeprüfte Annahme und wird danach nicht weitergepflegt. Nichts hier gehört in KRK
übernommen. Die Frage ist beantwortet; das Verzeichnis bleibt nur als Beleg stehen.

## Die Frage

`kommando_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs:1971`) weist jedes Kommando
außer dem Abbruch ab, solange `blatt_steht` ein Blatt meldet, und `blatt_steht`
(`anwendung.rs:1948`) fragt dafür `NSWindow::attachedSheet`. Die Antwort des Nutzers
führt der Abschlussblock von `beginSheetModalForWindow:completionHandler:` aus.

Ungeprüft war, ob AppKit diese beiden Zeitpunkte zusammenfallen lässt. Fällt
`attachedSheet` schon mit dem Beginn der Blattanimation auf nil, während der
Abschlussblock erst danach läuft, dann steht dazwischen eine Spanne, in der die Sperre
nicht mehr greift und die Antwort noch nicht ausgeführt ist. Ein Kommando in dieser
Spanne käme durch, und ein Sprung auf eine Textmarke verlöre seine vorgemerkte Stelle,
weil `anlass_unterbleibt` `vorgemerkte_marke` blind löscht.

Der Datensatz dazu:
`fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1207_*_die-spanne-zwischen-dem-schliessen-des-blattes-und-seiner-antwort-ist-ungemessen.md`

## Wie das Programm es misst

Es baut ein Fenster und ein Blatt mit zwei Schaltflächen, beantwortet das Blatt selbst
und tastet die Umgebung der Antwort mit zwei voneinander unabhängigen Armen ab.

```
   Blatt steht                Schließbefehl                   Antwort
        │                          │                             │
        ▼                          ▼                             ▼
  attachedSheet ────────────────────────────── ? ──────────────── ? ─────>
                                   │
                     Takt (1 ms) ──┤── Tastendrücke in die eigene Schlange
```

- **Der Taktarm** ist ein Zeitgeber mit einer Millisekunde Abstand auf der Hauptschleife.
  Er liest `attachedSheet` und hält fest, wann es erstmals nil ist. Er ist zugleich der
  Beweis, dass die Hauptschleife in der gemessenen Spanne arbeitet: liefe sie dort nicht,
  gäbe es dort keinen Takt.
- **Der Tastenarm** wirft Tastendrücke über `NSApp.postEvent(_:atStart:)` in die eigene
  Ereignisschlange, so wie es der Hinweis in `CLAUDE.md` verlangt, und sieht sie in einem
  lokalen Abgriff auf `keyDown` wieder — derselbe Mechanismus, den KRKs Ereignisabgriff
  benutzt. Jeder Tastendruck wird nach genau der Frage einsortiert, die `kommando_ausfuehren`
  stellt: stand `attachedSheet`, und war der Abschlussblock schon gelaufen? Vierzig
  Tastendrücke liegen vorab in der Schlange, damit auch die Zeit abgetastet ist, in der der
  Schließbefehl den Hauptfaden hält.

Beide Wege, auf denen KRK ein Blatt schließt, werden getrennt gemessen:

| Durchgang | Aufruf | Entspricht in KRK |
|---|---|---|
| `griff` | `endSheet(_:returnCode:)` | `Blattgriff::abbrechen` und der `Eingabewaechter` |
| `klick` | `performClick(nil)` | der Klick des Nutzers auf eine Schaltfläche |

**Das Programm braucht den Vordergrund nicht.** Es beantwortet sein Blatt selbst, läuft als
`.accessory` und nimmt dem Nutzer den Fokus nicht weg. Damit ist diese Messung, anders als
der Abnahmelauf, keine Nutzerarbeit.

## Bauen und starten

```sh
cd spikes/blatt-spanne
./starten.sh griff     # Ergebnis in messung-griff.txt
./starten.sh klick     # Ergebnis in messung-klick.txt
```

Voraussetzung sind nur die Command Line Tools. Jeder Durchgang läuft etwa zwei Sekunden
und beendet sich selbst.

## Das Ergebnis

Gemessen am **260810** auf **macOS 15.7.7**, Gerät **MacBookPro15,1**, also auf dem
Referenzgerät der Zeitzusagen. Belege: `messung-griff.txt` und `messung-klick.txt`.

| Was | `griff` | `klick` |
|---|---|---|
| `attachedSheet` vor dem Schließbefehl | gesetzt | gesetzt |
| **`attachedSheet` im Abschlussblock** | **gesetzt** | **gesetzt** |
| Abschlussblock nach dem Schließbefehl | +1,0 ms | +1,6 ms |
| `attachedSheet` erstmals nil | +270 ms | +272 ms |
| Takte der Hauptschleife mit nil vor dem Abschlussblock | 0 | 0 |
| Tastendrücke mit nil vor dem Abschlussblock | 0 | 0 |

**Die vermutete Spanne gibt es nicht.** AppKit setzt `attachedSheet` nicht vor dem
Abschlussblock auf nil, sondern rund 270 ms danach: der Abschlussblock läuft eine
Millisekunde nach dem Schließbefehl und noch mitten in der Blattanimation, `attachedSheet`
fällt erst mit deren Ende. Die Reihenfolge ist damit die umgekehrte der befürchteten, und
sie ist auf beiden Wegen gleich. Die Sperre aus `blatt_steht` greift bis zur ausgeführten
Antwort einschließlich; ein Kommando kann in KRK nicht zwischen die beiden Zeitpunkte
fallen.

**Die Gegenrichtung kostet auch nichts.** In den 270 ms, in denen `attachedSheet` nach der
ausgeführten Antwort weiter ein Blatt meldet, hält der Schließbefehl den Hauptfaden: kein
Takt schlägt, und die vierzig vorab eingelegten Tastendrücke kommen alle erst danach an. Die
Sperre weist dort nichts ab, weil dort nichts eintrifft.

**Was die Messung nicht deckt.** Sie ist an einem `NSAlert` mit zwei Schaltflächen und ohne
Beigabe gemessen, nicht an jedem der neun Blätter von KRK, und auf einer Systemfassung. Die
gemessene Größe ist eine Reihenfolge von AppKit und keine von KRK; ändert Apple sie, ändert
sich der Befund, und die Sperre wäre neu zu prüfen.

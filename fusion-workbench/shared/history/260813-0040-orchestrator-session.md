# Orchestrator Session — 260813-0040

**Directive:** Drei Faehigkeiten fuer KRK: eine inkrementelle Suche in der Belegungsansicht (F1),
eine zweite Instanz auch per Tastenbefehl, und alle Tastenbefehle auch ueber das Menue erreichbar.
**Mode:** custom, mit Shaper und Planner, autonom zu Ende gefuehrt
**Status:** In Arbeit

## Snapshot bei Sitzungsbeginn

- git HEAD: 188b81a
- Aktiver Circle: keiner, die Runde 6 ist am 260812 beschraenkt abgeschlossen
- Erkannte Domaene: code
- Offene Defekte: 26 im Circle der Runde 6, 5 in dem der Runde 5, 8 gemeinsam
- Offene Fragen: 12 ueber alle Speicher
- Waechter: kein Halt
- Nutzeranweisung: "mache das als neuen cycle, nutze shaper und planner, fuehre die aufgabe autonom zu ende"

## Wie "autonom" ausgelegt wird

Ohne Bestaetigungshalte an den ueblichen Stellen. Wo eine Entscheidung sich nicht ableiten
laesst, entsteht ein offener Datensatz mit Empfehlung statt einer Wahl des Agenten. Der sonst
verbindliche Halt vor ontocoder-Arbeit an der Belegungsdatei gilt als von der Anweisung gedeckt
und wird berichtet statt vorgelegt.

## Vorbefund am Baum, vor dem Shaper erhoben

- **Zweite Instanz:** kein `flock`, kein `O_EXCL`, keine Sperre unter `crates/krk-core/src/ablage/`.
  Zwei Instanzen schrieben dieselben vier Dateien ohne Absprache. Die Runde 6 hat das Zur-Seite-Legen
  einer **beschaedigten** Datei gebaut; gegen zwei gleichzeitige Schreiber traegt das nicht.
- **Menue:** rund zwanzig Eintraege heute gegen 81 Funktionen in der Belegung. Befehle tragen einen
  Wirkungsbereich, ein gerade unwirksamer Eintrag gehoert ausgegraut.
- **Suche:** `belegungsmodell.rs` kennt Suchbegriffe, `appkit/belegungsansicht.rs` nicht.

## Per-Turn Log

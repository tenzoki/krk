# Shaper: Suche in der Belegung, vollständiges Menü, weitere Instanz

**Datum:** 2026-08-13
**Status:** Complete
**Modus:** user-direct (kein Circle aktiv, Spec und Datensätze im gemeinsamen Speicher)
**Auftrag:** die drei Wünsche des Nutzers vom 260813 zu einer Runde zuschneiden, autonom, ohne Rückfrage wo ableitbar

## Was entstanden ist

- Spec: `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md`
- Vier Entscheidungsdatensätze, alle offen, alle mit Empfehlung, alle in `shared/decisions/` unter dem Zeitstempel `260813-0053`: Schaltflächentasten der Belegungsansicht, Zahl der Obermenüs, geteilte Ablage bei zwei Instanzen, Schluckverhalten des Ereignisabgriffs.

**Keine `AskUserQuestion` gestellt.** Jede Frage war entweder aus dem Baum ableitbar oder als Datensatz mit Empfehlung ablegbar; die Runde ist auf den vier Empfehlungen zuschneidbar.

## Was am Baum erhoben wurde

Gelesen: `appkit/menue.rs`, `appkit/belegungsansicht.rs`, `appkit/ereignisse.rs`, Teile von `appkit/anwendung.rs`, `belegungsmodell.rs`, `kommandos/fokus.rs`, `ablage/mod.rs`, `ablage/pfade.rs`, `ablage/atomar.rs`, `ablage/sitzung.rs`, `verzeichnis/sprungmarke.rs`, `text/suche.rs`, `main.rs`, `resources/default-keymap.toml`, `resources/Info.plist`, dazu C7 und C8 des Spec der Runde 1 und der Plan der Runde 6 als Muster für die Kriterienform.

Sieben Befunde tragen den Zuschnitt, und drei davon waren vorher nicht bekannt:

1. **Das Menü führt zehn Befehle, nicht rund zwanzig.** Zwei im Anwendungsmenü, sechs unter „Bearbeiten", zwei unter „Fenster".
2. **Die Ausgrauung im Menü ist eine Korrektheitsbedingung.** Der Ereignisabgriff reicht einen abgewiesenen Befehl unverändert an AppKit weiter; ein Menüeintrag mit Kürzel führte ihn dann aus. Mit dem Fokus im Editor bewegte ein Auf-Pfeil die Dateiliste statt der Schreibmarke.
3. **`atomar::nachbarpfad` trägt bewusst keine Laufnummer.** Zwei Instanzen schreiben deshalb dieselbe Nachbardatei, und das `rename` kann ein Gemisch veröffentlichen. Der Schaden ist damit nicht bloß eine verlorene Änderung, sondern eine beschädigte Datei; die Runde 6 fängt die Folge auf und verhindert die Ursache nicht.
4. Es gibt in `crates/` kein `flock`, kein `O_EXCL`, keine Sperre.
5. `Kommando` trägt 75 Varianten, die Belegung 81 Funktionen mit 87 Kombinationen, `Funktionsbereich` neun Werte. Die Differenz 81 minus 75 sind die sechs Textbefehle des Menüs.
6. **Leertaste und Eingabetaste sind in der Belegungsansicht vergeben** und stehen dem Suchwunsch im Weg.
7. **`opt+cmd+n` ist frei**, `cmd+n`, `shift+cmd+n` und `ctrl+cmd+n` sind es nicht.

## Was ohne Rückfrage abgeleitet wurde

„Zweite Instanz" als zweiter Prozess (aus C7 der Runde 1, die den Mehrfenster-Umbau an L4 gebunden und hinausgeschoben hat); Teilzeichenfolge statt Wortanfang und Unempfindlichkeit gegen die Schreibweise (aus Zweck und Sprungmarke); Umlauf der Suche (aus `text::suche`); Suche über die zwei angezeigten Spalten (ein unsichtbarer Treffer ist keiner); `esc` ohne dritte Bedeutung; Ausgrauung als Pflicht; Menügliederung aus `nach_bereichen`; `cmd+n` bleibt bei „Fenster einblenden"; `opt+cmd+n` für den neuen Befehl.

## Aussage zu C8

Keine elfte Zusage, keine der zehn angefasst. Drei liegen auf dem Weg und gehören in den nächsten Abnahmelauf: **L4**, weil das Menü auf dem Startpfad von zehn auf zweiundachtzig Einträge wächst und die Sperre einen Systemaufruf dazustellt, und **L1** und **L9**, die herleitbar nicht betroffen sind, weil ein geschluckter Tastendruck `NSApplication::sendEvent:` nie erreicht — eine Herleitung und keine Messung.

## Empfehlung zum Zuschnitt

Eine Runde mit drei Fähigkeiten. Suche und Menü teilen sich Belegung, Gliederung und eine Kollisionsregel; die weitere Instanz teilt mit beiden zwei Zeilen. Sie ist damit die benannte Naht: wird die Runde lang, lässt sie sich zum Preis dieser zwei Zeilen als eigene Runde herauslösen.

## Was der Orchestrator als nächstes tut

Den Circle anlegen, die vier offenen Fragen dem Nutzer vorlegen und danach den Planner ansetzen. Kein `make bundle` und kein `cargo xtask bundle`: unter `target/KRK.app` liegt ein beglaubigtes Bündel.

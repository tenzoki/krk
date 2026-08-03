# Was tut KRK, wenn der Nutzer das letzte Fenster schließt?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-1536_c_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 12), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C1 und C7)

---

## Frage

Cmd+W schließt heute das einzige Fenster, das KRK kennt. Die Anwendung läuft
danach weiter, mit Menüleiste im Bildschirmkopf und ohne jeden Weg zurück zu
einem Fenster: es gibt weder `applicationShouldTerminateAfterLastWindowClosed:`
noch `applicationShouldHandleReopen:` noch einen Menüeintrag "Neues Fenster".
Der Nutzer kann nur noch beenden. Für eine Anwendung, deren erste Maxime die
Tastatursteuerung ist, liegt damit ein Kürzel in Reichweite, das sie
unbedienbar macht, und keines, das sie zurückholt.

Die Frage muss jetzt beantwortet werden, weil S12 das Fenstermodell anlegt und
damit der Schritt ist, in dem beide möglichen Antworten am billigsten sind.
Danach wäre die zweite Antwort eine Nachrüstung an einer bereits gebauten
Fensterverwaltung.

**Der Spec beantwortet sie nicht.** C1 regelt das Schließen des letzten **Tabs**
und sagt dafür ausdrücklich, dass das Dateifenster stehen bleibt und einen
Standardordner zeigt; C7 regelt das Ein- und Ausblenden der vier Bereiche
innerhalb des einen Fensters und sichert zu, dass mindestens ein Dateifenster
sichtbar bleibt. Über das Schließen des Fensters selbst steht in beiden nichts.
Die Zusage aus C7 ist der nächste Verwandte der Frage, aber sie greift eine
Ebene tiefer: sie schützt die Bereiche im Fenster, nicht das Fenster.

## Optionen

1. **Cmd+W beendet KRK.** `applicationShouldTerminateAfterLastWindowClosed:`
   liefert `true`.
   - Dafür: der übliche Weg für eine Anwendung ohne Dokumente. Kostet vier
     Zeilen in `crates/krk-ui/src/appkit/anwendung.rs`. Die Sackgasse ist
     ausgeschlossen, weil es keinen fensterlosen Zustand mehr gibt. Keine neue
     Tastenbelegung, keine Zeile in `resources/default-keymap.toml`.
   - Dagegen: Cmd+W und Cmd+Q tun dasselbe, solange es ein Fenster gibt, und
     der Menüeintrag "Fenster schließen" ist damit sinnentleert. Wer später
     mehrere Fenster will, nimmt die Zeile wieder heraus.

2. **Ein Weg zurück zum Fenster.** Ein Menüeintrag "Neues Fenster" mit einem
   Kürzel und `applicationShouldHandleReopen:` für den Klick auf das
   Dock-Symbol.
   - Dafür: passt zu einem Dateimanager, der mehrere Fenster tragen soll, und
     fällt mit der Arbeit von S12 zusammen, das die Fensterverwaltung ohnehin
     anlegt. Cmd+W behält seine Bedeutung.
   - Dagegen: berührt `crates/krk-ui/src/appkit/menue.rs` und braucht eine
     Belegung in `resources/default-keymap.toml`. Ein zweites Fenster wirft
     Folgefragen auf, die Runde 1 nicht gestellt hat: teilen sich zwei Fenster
     eine Sitzung, und was heißt "das aktive Dateifenster" aus C1, wenn es
     zwei Fenster mit je zwei Dateifenstern gibt.

## Constraints

- Die Antwort darf keine der zehn Zahlen aus C8 berühren. Beide Optionen tun
  das nicht.
- Das Kürzel aus Option 2 darf nicht mit der Auslieferungsbelegung aus C3
  kollidieren. Cmd+N ist dort frei; belegt sind Cmd+Y, Cmd+Shift+K,
  Cmd+Shift+V, Cmd+Shift+N, Cmd+Opt+Delete und Cmd+Delete. Zu prüfen bleibt
  die Abgrenzung gegen den Befehl "neuen Tab öffnen" aus C1, für den Cmd+N
  ebenfalls naheläge.
- Ein laufendes KRK ohne Fenster und ohne Rückweg ist in beiden Fällen
  ausgeschlossen; das ist die eigentliche Zusage, und die Optionen
  unterscheiden sich nur darin, wie sie sie einlösen.

## Empfehlung

Option 1 für Runde 1. Der Zuschnitt der Runde kennt genau ein Fenster: C1
spricht von zwei Dateifenstern nebeneinander, und das sind Bereiche innerhalb
eines Fensters, nicht zwei Fenster des Systems. Ein zweites Fenster einzuführen,
nur um einen Rückweg aus einem Zustand zu schaffen, den es ohne den Rückweg gar
nicht geben müsste, ist die teurere Antwort auf die kleinere Hälfte der Frage;
die Folgefragen unter Option 2 wären dann in Runde 1 zu beantworten, obwohl
keine Fähigkeit sie stellt.

Option 2 wird richtig, sobald mehrere Fenster zum Umfang gehören. Das ist eine
Frage für eine spätere Runde, und sie ändert dann eine Zeile im
Anwendungsdelegierten zurück.

---
Answered:
Implemented:
Deferred:
Superseded by:

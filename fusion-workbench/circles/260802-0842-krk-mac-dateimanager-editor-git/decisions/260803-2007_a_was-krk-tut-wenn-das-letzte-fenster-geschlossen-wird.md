# Was tut KRK, wenn der Nutzer das letzte Fenster schließt?

---
**Domain:** code
**Status:** answered
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

## Antwort des Nutzers, 260804-0830

**Der Nutzer hat Möglichkeit 2 gewählt, gegen die Empfehlung dieses
Datensatzes.** KRK bekommt einen Weg zurück zum Fenster: einen Menüeintrag mit
einem Kürzel und `applicationShouldHandleReopen:` für den Klick auf das
Dock-Symbol. Cmd+W behält damit seine Bedeutung "Tab schließen" aus
`resources/default-keymap.toml`, und der Menüeintrag, der das Fenster schließt,
weicht auf Shift+Cmd+W aus, wie es Webbrowser halten.

### Die beiden Folgefragen binden Runde 1 nicht

Möglichkeit 2 nennt unter "Dagegen" zwei Fragen, die ein zweites Fenster
aufwirft: ob zwei Fenster sich eine Sitzung teilen, und was "das aktive
Dateifenster" aus C1 bedeutet, wenn es zwei Fenster mit je zwei Dateifenstern
gibt. Beide bleiben in Runde 1 ungestellt, und der Grund ist nicht Nachlässigkeit,
sondern der Zuschnitt der gewählten Antwort.

**Runde 1 kennt genau ein Fenster, und der Menüeintrag legt kein zweites an.**
Das Fenster überlebt Cmd+W bereits heute: `setReleasedWhenClosed(false)` ist
gesetzt, und der Anwendungsdelegierte hält es weiter
(`crates/krk-ui/src/appkit/anwendung.rs`, geprüft am 260804 gegen den Befund in
`issues/260803-1536_c_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md`).
Der Menüeintrag holt dieses eine Fenster nach vorn, und
`applicationShouldHandleReopen:` tut dasselbe beim Klick auf das Dock-Symbol.
Die Sackgasse ist damit aufgelöst, ohne dass ein zweites Fenster entsteht, und
eine Frage, die ohne zweites Fenster nicht auftritt, ist in dieser Runde keine.

Drei Stellen des Specs bestätigen den Zuschnitt: C1 beschreibt zwei
Dateifenster als Bereiche nebeneinander und nicht als zwei Fenster des Systems,
C7 sichert das Ein- und Ausblenden dieser Bereiche innerhalb des einen Fensters
zu, und die Prüfsitzung aus C8, auf der L4 und L5 abgenommen werden, ist für ein
Fenster mit zwei Dateifenstern beschrieben. Ein zweites Fenster machte L4
mehrdeutig, weil unklar wäre, welches Fenster die Zusage beendet.

**Der Preis dieser Auflösung ist die Beschriftung.** Ein Eintrag namens "Neues
Fenster" legte auf dem Mac üblicherweise eines an, und dieser legt keines an. Er
heißt deshalb in Runde 1 **"Fenster einblenden"**. Die Runde, die mehrere Fenster
einführt, benennt ihn in "Neues Fenster" um, beantwortet dabei die beiden
Folgefragen und behält das Kürzel.

### Was die Antwort im Plan und im Spec berührt

- **S12** setzt sie um: Menüeintrag, `applicationShouldHandleReopen:`, die
  Verschiebung des Eintrags "Fenster schließen" auf Shift+Cmd+W und die
  ausgeschriebene Annahme, dass Runde 1 höchstens ein Fenster kennt.
- **S9b** trägt die Kombination in `resources/default-keymap.toml` ein:
  `id = "fenster_einblenden"` auf `cmd+n`. Cmd+N ist in der ausgelieferten Datei
  frei, geprüft am 260804 gegen alle 52 Kombinationen; belegt sind daneben
  `shift+cmd+n` für das Anlegen eines Ordners und `ctrl+cmd+n` für das Anlegen
  einer Datei.
- **C7 des Specs** trägt die Zusage, dass KRK nach dem Schließen des Fensters
  bedienbar bleibt, samt der Festlegung auf ein Fenster in dieser Runde.
- `issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`
  ist damit in der Sache entschieden: Cmd+W gehört dem Tab. Der Defekt bleibt
  offen, bis S12 den Menüeintrag verschoben hat.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 12) — Möglichkeit 2: Menüeintrag "Fenster einblenden" auf Cmd+N plus `applicationShouldHandleReopen:`; Runde 1 kennt weiterhin genau ein Fenster, die beiden Folgefragen zu zwei Fenstern binden sie nicht.
Implemented:
Deferred:
Superseded by:

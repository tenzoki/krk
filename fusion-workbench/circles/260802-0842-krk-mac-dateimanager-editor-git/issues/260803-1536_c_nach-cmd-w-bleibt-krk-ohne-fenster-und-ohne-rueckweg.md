Nach Cmd+W bleibt KRK ohne Fenster und ohne Weg zu einem neuen

---

Der Menüeintrag "Fenster schließen" schließt das einzige Fenster, das KRK kennt.
Die Anwendung läuft danach weiter, mit Menüleiste im Bildschirmkopf und ohne
jeden Weg zurück zu einem Fenster. Der Nutzer kann nur noch beenden.

---

## Der Nachweis

`crates/krk-ui/src/appkit/menue.rs:33-42` legt "Fenster schließen" auf
`performClose:` mit Cmd+W. Der Fensterdelegierte reagiert darauf in
`crates/krk-ui/src/appkit/fenster.rs:52-58`:

```rust
unsafe impl NSWindowDelegate for FensterDelegierter {
    #[unsafe(method(windowWillClose:))]
    fn fenster_schliesst(&self, _meldung: &NSNotification) {
        self.ivars().quelle.lesen_abbrechen();
    }
}
```

Das ist alles. Der Anwendungsdelegierte implementiert genau eine Methode,
`applicationDidFinishLaunching:` (`crates/krk-ui/src/appkit/anwendung.rs:53-59`).
Weder `applicationShouldTerminateAfterLastWindowClosed:` noch ein Menüeintrag
"Neues Fenster" noch ein `applicationShouldHandleReopen:` existiert. Das
Anwendungsmenü trägt einen einzigen Befehl, "KRK beenden"
(`menue.rs:23-32`).

Das Fenster selbst überlebt: `setReleasedWhenClosed(false)`
(`fenster.rs:90`), und der Anwendungsdelegierte hält es weiter in
`AnwendungsIvars.fenster` (`anwendung.rs:33`). Es ist nur unsichtbar und
unerreichbar.

## Warum das ein Defekt und keine Zuschnittfrage ist

Der Zustand ist nicht bloß leer, er ist eine Sackgasse. Eine Anwendung, deren
erste Maxime die Tastatursteuerung ist, hat mit Cmd+W ein Kürzel in Reichweite,
das sie unbedienbar macht, und keines, das sie zurückholt. Ein Nutzer, der Cmd+W
statt Cmd+Q trifft, sieht KRK aus dem Bild verschwinden, findet es aber weiter im
Dock und im Programmwechsler.

Kein Schritt des Plans nimmt das auf. Eine Durchsicht des Plans nach
`performClose`, `applicationShouldTerminate` und "neues Fenster" findet den Begriff
nur in der Änderungsliste von S6 selbst
(`planning/260802-1428_o_plan-navigator-geruest-runde-1.md:478`). S12 baut die vier
Bereiche in das bestehende Fenster, S13 die Tastaturnavigation innerhalb der
Liste. Die Lücke wächst also nicht heraus.

Das Abnahmekriterium von S6 ist wörtlich erfüllt: "Cmd+Q beendet, Cmd+W schließt
das Fenster." Beides tut es. Dass danach nichts mehr kommt, prüft es nicht ab.

## Was zu tun ist

Zwei Antworten sind vertretbar, und die Wahl gehört dem Nutzer:

1. **Cmd+W beendet KRK.** `applicationShouldTerminateAfterLastWindowClosed:`
   liefert `true`. Das ist der übliche Weg für eine Anwendung ohne Dokumente,
   und er kostet vier Zeilen im Anwendungsdelegierten. Der Preis: Cmd+W und Cmd+Q
   tun dasselbe, solange es nur ein Fenster gibt, und der Menüeintrag "Fenster
   schließen" wird damit sinnentleert, bis es mehrere Fenster gibt.
2. **Ein Weg zurück zum Fenster.** Ein Eintrag "Neues Fenster" (Cmd+N) im Menü
   und `applicationShouldHandleReopen:` für den Klick auf das Dock-Symbol. Das
   passt zu einem Dateimanager, der später mehrere Fenster tragen soll, und ist
   die Vorarbeit, die S12 ohnehin braucht.

Wenn die Frage über den Zuschnitt von Runde 1 hinausgeht, ist der richtige
Ausgang dieses Datensatzes ein Entscheidungsdatensatz zur Frage "Was tut KRK,
wenn der Nutzer das letzte Fenster schließt?" mit den beiden Möglichkeiten oben.
Bis dahin steht der Zustand.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.

---
Resolved: Der Datensatz nannte selbst den richtigen Ausgang, und er ist genommen. Die Wahl zwischen den beiden Antworten ist eine Festlegung über das Verhalten der Anwendung und keine Planungsentscheidung; weder C1 noch C7 des Specs beantwortet sie, weil C1 das Schließen des letzten Tabs regelt und C7 das Ein- und Ausblenden der Bereiche innerhalb des einen Fensters. Die Frage liegt deshalb als `decisions/260803-2007_o_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md` mit beiden Möglichkeiten, ihren Dateifolgen und einer Empfehlung für Möglichkeit 1.

Die Lücke im Plan ist zugleich geschlossen: S12 nimmt sie auf. S12 ist der Schritt, der das Fenstermodell anlegt, er hat noch nicht begonnen, und beide Antworten sind dort am billigsten; die zweite fällt sogar mit seiner ohnehin anstehenden Arbeit zusammen. S12 trägt jetzt die Beschreibung der Sackgasse, den Verweis auf den Entscheidungsdatensatz als Abhängigkeit und ein Abnahmekriterium, das ein laufendes KRK ohne Fenster und ohne Rückweg ausschließt. S6 bleibt unberührt: sein Vermerk `[DONE]` und sein Kriterium ändern sich nicht, weil beide wörtlich erfüllt sind. Nachgezogen am 260803-2007.

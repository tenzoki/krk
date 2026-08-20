`f4` setzt den Fokus nur dann in den Editor, wenn der Editor schon eine Datei zeigt

---

Der Nutzer meldet am 260820-1030 aus dem Abnahmelauf der Runde 14: „ich navigiere auf eine Datei,
drück F4: der Fokus ist dann NICHT UNBEDINGT im Editor (er ist dann im Lesezeichen-Fenster oder
nirgends). Der Fokus ist nur dann richtig, wenn vorher der Editor schon eine Datei angezeigt hat."

---

**Gefilt von:** orchestrator, Sitzung `260819-2026`, aus dem Bündeldurchgang des Nutzers
**Baumstand:** `dad0a36`, Bündel 0.5.4 aus `05cb614`
**Schwere:** hoch. Die Datei öffnet sich, aber die Tasten des Nutzers landen woanders. Das ist
ein Zustand, in dem der nächste Tastendruck etwas anderes tut als erwartet, und der Nutzer sieht
den Grund nicht.

## Was geprüft ist

**Die Runde 14 hat den Befehl nicht verursacht.** `Anwendungsdelegierter::im_editor_oeffnen`
(`anwendung.rs:6194`) ist im Bereich `fce0b6f..dad0a36` unverändert, ebenso
`editor_oeffnen_lassen`. In `fokusansicht` (`:2173`) ist allein der Zweig `Fokus::Vorschau`
geändert; `Fokus::Editor => Some(self.ivars().editor.get()?.textflaeche())` steht unverändert.

## Was erschlossen und nicht gemessen ist

`Anwendungsdelegierter::fokus_setzen` (`anwendung.rs:2219`) verweigert den Fokus in einen
**ausgeblendeten** Bereich:

```
let ausgeblendet = fokus::bereich_mit_fokus(ziel, modell.aktiv())
    .is_some_and(|bereich| !modell.sichtbar(bereich));
if ausgeblendet { return false; }
```

Sein Doc-Kommentar nennt die Sperre ausdrücklich und begründet sie: „In einen ausgeblendeten
Randbereich geht der Fokus nicht: dort sähe der Nutzer weder seine Auswahl noch, dass seine Tasten
irgendwo ankommen." Das passt auf die Beobachtung des Nutzers — der Editor ist ausgeblendet,
solange er nie eine Datei gezeigt hat, und genau dann greift die Sperre.

**`inference:`** Die Ursache wäre dann die Reihenfolge von Einblenden und Fokussetzen in
`editor_oeffnen_lassen`: wird der Fokus gesetzt, bevor das Fenstermodell den Bereich als sichtbar
führt, greift die Sperre gegen einen Bereich, der gleich sichtbar sein wird. **Gemessen ist das
nicht.** Wer den Datensatz behebt, erhebt es zuerst.

Die Gestalt ist in diesem Projekt bekannt: `a6b3818` („einblenden und lesen sind beim Angleichen
zwei Handlungen") behob dieselbe Art von Reihenfolgefehler in der Runde 13.

## Reproduktion

KRK frisch starten, sodass der Editor keine Datei zeigt. In der Dateiliste auf eine Textdatei
navigieren, `f4` drücken. Erwartet: die Datei steht im Editor und der Fokus ist darin. Beobachtet:
die Datei steht im Editor, der Fokus liegt in der Lesezeichenleiste oder nirgends. Ein zweites
`f4` auf dieselbe oder eine andere Datei setzt den Fokus dann richtig.

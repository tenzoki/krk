Ein modales Blatt widerspricht der Zusage, dass die Oberfläche bedienbar bleibt

---

C4 sagt: "Während eine länger laufende Operation arbeitet, bleibt die Oberfläche bedienbar, zeigt den Fortschritt und lässt sich abbrechen." S16 zeigt den Fortschritt als Blatt. Ein Blatt ist fenstermodal und sperrt genau die Oberfläche, die bedienbar bleiben soll.

---

## Der Widerspruch im Einzelnen

Ein Sheet blockiert das Fenster, an dem es hängt. Während einer Kopie von 5.000 Einträgen kann der Nutzer deshalb nicht navigieren, nicht markieren und keinen Tab wechseln. Er kann abbrechen, und das ist alles.

Die Umsetzung verschärft das noch, notwendigerweise: `crates/krk-ui/src/kommandos/operationen.rs` führt die Regel `waehrend_blatt_erlaubt`, die jeden Tastenbefehl außer dem Abbruch an AppKit weiterreicht, solange ein Blatt steht. Ohne diese Sperre schluckte der Ereignisabgriff den Tabulator (er liegt auf `fenster_wechseln`) und das Blatt wäre ohne Maus nicht zu beantworten; mit ihr ist die Sperre ausdrücklich.

## Was die Zusage vermutlich meint

Zwei Lesarten:

1. **Die Anwendung friert nicht ein.** Der Hauptfaden führt keine Dateisystem-Arbeit aus, das Fenster zeichnet, das Blatt reagiert, der Abbruch greift binnen Millisekunden. Diese Lesart hält, und sie ist die, die L9 misst ("keine Eingabe wartet länger als 16 ms während einer Stapeloperation").
2. **Der Nutzer arbeitet weiter.** Er navigiert im anderen Dateifenster, während die Kopie läuft. Diese Lesart hält nicht und kann mit einem Blatt nicht halten.

`### Frage 6` des Plans begründet die Nebenläufigkeit ausdrücklich mit L9 und damit mit Lesart 1. Der Wortlaut von C4 legt Lesart 2 mindestens nahe.

## Was zu entscheiden ist

Ob der Fortschritt ein Blatt bleibt oder in eine nicht sperrende Anzeige wandert, etwa in die Statuszeile aus C1. Die zweite Wahl löst zugleich den Defekt `260804-1814_o_ein-blatt-braucht-360-ms-bis-es-steht-und-l8-sagt-200-ms-zu.md`, weil eine Statuszeile ohne Einblendung erscheint. Sie kostet dafür die Sichtbarkeit: eine graue Zeile am Fuß ist leichter zu übersehen als ein Blatt in der Mitte, und der Abbruch bräuchte einen sichtbaren Griff.

Die Konfliktfrage und die Rückfrage vor dem endgültigen Löschen bleiben in jedem Fall Blätter: sie **sollen** sperren, weil sie auf eine Antwort warten.

**Aufgefallen bei:** der Umsetzung von Schritt 16 am 260804-1814.

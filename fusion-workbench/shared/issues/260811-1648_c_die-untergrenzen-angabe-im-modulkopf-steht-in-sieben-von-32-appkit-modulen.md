Die Angabe der macOS-Untergrenze im Modulkopf steht in sieben von 32 AppKit-Modulen

---

`CLAUDE.md` führt unter "Technologiewahl" als Gegenmaßnahme gegen die fehlenden Verfügbarkeitsangaben von `objc2` eine Gewohnheit: "jedes AppKit-Modul dieses Projekts nennt in seinem Modulkopf die Untergrenze jeder Klasse, die es anspricht". Gezählt am 260811 über `crates/krk-ui/src/appkit/*.rs` und `crates/krk-ui/src/appkit/blaetter/*.rs` erwähnen sieben von 32 Dateien überhaupt eine macOS-Version:

```
37 editor.rs      9 menue.rs      3 nummernspalte.rs      2 fsevents.rs
 2 anwendung.rs   1 leiste.rs     1 aufteilung.rs
```

Ohne jede Nennung sind unter anderen `tabelle.rs`, `zwischenablage.rs`, `terminal.rs`, `volumes.rs`, `vorschau.rs`, `fenster.rs`, `ereignisse.rs`, `papierkorb.rs`, `statuszeile.rs` und alle neun Module unter `blaetter/`.

---

Gefunden vom `planner` am 260811 beim Erheben der Grundlage für die Runde 4 (`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`). Der Befund ist ein Widerspruch zwischen der Dokumentation und dem Baum und kein Fehler im Programm: die angesprochenen Klassen stehen sämtlich seit macOS 10.0 zur Verfügung, soweit nachgesehen, und ein Absturz ist von dieser Lage nicht zu erwarten.

**Er ist nebenbei gefunden und nicht durch die Directive dieser Runde verursacht**, deshalb liegt er im gemeinsamen Speicher und nicht im Circle. Die Runde 4 zieht die Angabe für die beiden Module nach, die sie ohnehin anfasst (`zwischenablage.rs` und das neue `standardprogramm.rs`), und lässt die übrigen stehen; das steht in ihrem Umsetzungsplan.

Zwei Wege stehen offen, und sie schließen einander nicht aus. Der eine trägt die Angabe in den 25 Modulen nach, die sie nicht haben — Handarbeit, einmalig, und danach stimmt der Satz in `CLAUDE.md`. Der andere schwächt den Satz in `CLAUDE.md` auf das ab, was gilt, etwa: die Angabe steht dort, wo eine Klasse oder eine Methode nach macOS 10.0 hinzugekommen ist. Welcher der beiden richtig ist, hängt daran, ob die Gewohnheit die Untergrenze **jeder** angesprochenen Klasse meint oder allein die der zweifelhaften; der Satz in `CLAUDE.md` sagt heute das erste, `menue.rs:135-146` macht das erste vor, und die Mehrheit der Module tut das zweite.

---
Resolved: **31 von 33 Dateien unter `appkit/` tragen den Abschnitt**, nachgezaehlt am Bestand.
Die zwei uebrigen sprechen keinen Typ an und brauchen ihn nicht: `koordinaten.rs` rechnet auf
Byte- und UTF-16-Versaetzen, `mod.rs` ist die Modulwurzel.

**Die Zahl im Titel dieses Datensatzes war zu hoch.** Sie zaehlte jede Datei, in der irgendwo
eine macOS-Fassung vorkommt; vier der sieben erwaehnten sie bloss beilaeufig im Fliesstext. Als
Zusage im Kopf stand die Ueberschrift **fuenfmal**, nicht siebenmal. 26 sind nachgetragen.

**Keine Klasse im Baum liegt ueber macOS 15.** Die hoechsten Untergrenzen sind 10.15
(`NSWorkspaceOpenConfiguration`) und 14.0 (`CADisplayLink`, `NSApplication.activate`). Das
einzige auf oder ueber 15.0 ist die Schreibwerkzeug-Gruppe in `editor.rs`, dort seit der Runde 2
dokumentiert und mit `respondsToSelector:` gehuetet. Nichts stillschweigend behoben.

**Vier bestehende Angaben waren falsch und sind berichtigt:** `stapelumbenennen.rs` behauptete,
`NSTableViewStyle` trage keine eigene Angabe (sie traegt 11.0), `vorschau.rs` zaehlte
`NSImageScaling` zu den 10.0-Aufzaehlungen (sie traegt 10.5), `volumes.rs` unterschlug
`NSVolumeEnumerationOptions` (10.6), und `anwendung.rs` nannte `activateIgnoringOtherApps:`
abgekuendigt, wo der Kopf `API_TO_BE_DEPRECATED` sagt.

**Fuenf Stellen stehen als das da, was sie sind, statt mit einer erfundenen Zahl** — Konstanten
und `typedef`s ohne Symbol, und zwei Klassen, bei denen "keine Angabe" nur "spaetestens seit
10.0" hergibt, weil Apple Angaben unterhalb der unterstuetzten Fassung entfernt.

**Zwei Annahmen meines Auftrags haben nicht getragen**, und der `coder` hat sie berichtigt statt
uebernommen: die erzeugte objc2-Bindung fuehrt **keine** `API_AVAILABLE`-Angaben, einzige Quelle
ist der SDK-Kopf ueber `xcrun --show-sdk-path`; und `/System/Library/Frameworks/AppKit.framework/Headers`
gibt es auf diesem Geraet gar nicht. Beides ist unabhaengig nachgeprueft.

**Zur Pruefbarkeit** hat er drei Stufen mit ihren Kosten vorgelegt; sie sind als eigene Frage
abgelegt: `shared/decisions/260811-2050_o_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`.

**Ein Beifund, nicht behoben:** `fenster.rs` sagt zweimal "vier Bereiche", seit der Editor-Runde
sind es fuenf.

Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/history/260811-1454-orchestrator-session.md`.

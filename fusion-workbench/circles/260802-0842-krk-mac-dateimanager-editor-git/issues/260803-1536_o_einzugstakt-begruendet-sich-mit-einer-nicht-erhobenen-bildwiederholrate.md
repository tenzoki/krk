Der Einzugstakt begründet sich mit einer Bildwiederholrate, die das Projekt als unerhoben führt

---

`crates/krk-ui/src/appkit/tabelle.rs:54-58` legt den Takt des Zeitgebers auf ein
Sechzigstel einer Sekunde und begründet das mit einer Eigenschaft des
Referenzgeräts, die im Projekt ausdrücklich noch nicht gemessen ist.

---

## Die Stelle

```rust
/// Der Takt, in dem der Hauptfaden den Kanal des Lesers leerraeumt.
///
/// Ein Sechzigstel einer Sekunde ist ein Bild auf dem Referenzgeraet. Haeufiger
/// zu raeumen brauchte es nicht, weil die Tabelle ohnehin nicht oefter zeichnet.
const EINZUGSTAKT: NSTimeInterval = 1.0 / 60.0;
```

Beide Sätze setzen 60 Hz voraus. Der geschlossene Defekt
`issues/260802-1900_c_bildwiederholrate-am-referenzgeraet-nicht-per-system-profiler-erhebbar.md`
hält den Gegenstand fest: `system_profiler SPDisplaysDataType` meldet zum
eingebauten Bildschirm des `MacBookPro15,1` keine Zeile `Refresh Rate`, und der
Bedingungskopf der Messberichte schreibt die Lücke seither als Lücke aus, statt
eine Zahl zu erfinden. Erhoben wird sie erst in S8, aus
`NSScreen.maximumFramesPerSecond`.

Die Zahl im Programmtext ist damit dieselbe Annahme, die die Messstrecke
ausdrücklich nicht macht. Sie steht nur an einer Stelle, an der niemand nach ihr
sucht.

## Warum das mehr als ein Kommentarfehler ist

Der Defekt von damals nennt die Folge schon: "Ein Messwert von 16 ms heißt 'ein
Bild' nur, wenn der Bildschirm mit 60 Hz läuft; auf einem Bildschirm mit 120 Hz
wäre dieselbe Zahl zwei Bilder."

Dasselbe gilt für den Takt. Läuft der Bildschirm mit 120 Hz, dann räumt der
Zeitgeber den Kanal nur bei jedem zweiten Bild, und der zweite Satz des
Kommentars ("die Tabelle zeichnet ohnehin nicht öfter") ist falsch. Die Zusage
aus dem Modulkopf, "die Tabelle zeichnet hoechstens einmal je Bild"
(`tabelle.rs:15-17`), hielte dann in der einen Richtung, aber der Aufbau der
Liste liefe halb so schnell, wie der Bildschirm es zuließe.

Der Zeitpunkt ist günstig: S8 liest die Rate ohnehin und ist der nächste
Schritt. Wird die Zahl dort gemessen und passt sie nicht zu 60, ist die Frühmessung
gegen einen Takt gefahren, der zu ihrem eigenen Bedingungskopf im Widerspruch
steht.

## Was zu tun ist

Kein Umbau. Zwei Möglichkeiten, je nachdem, was S8 misst:

- **Ist die Rate 60 Hz:** den Kommentar auf den Beleg umschreiben, mit Verweis
  auf den Messbericht von S8 statt auf eine Behauptung.
- **Ist sie es nicht:** den Takt aus `NSScreen.maximumFramesPerSecond` ableiten
  statt ihn festzuschreiben. Der Nachschlag liegt nach S8 ohnehin unter
  `crates/krk-ui/src/appkit/`, siehe
  `issues/260803-1345_o_dateiliste-von-s8-legt-objc2-code-ausserhalb-von-appkit-ab.md`.
  Ein fester Rückfallwert wäre die Sonderregel mit eigenem Rückfallweg, die die
  Maxime "supersimpel" ausschließt; die Haltung des Projekts bei `--kalt` und bei
  `NSWindow.screen()` ist der Abbruch mit Meldung.

Bis dahin bleibt der Kommentar als das zu kennzeichnen, was er ist: eine Annahme.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.

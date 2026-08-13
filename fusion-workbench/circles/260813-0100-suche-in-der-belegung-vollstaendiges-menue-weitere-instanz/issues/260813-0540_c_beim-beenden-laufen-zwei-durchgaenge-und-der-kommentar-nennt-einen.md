Beim Beenden laufen zwei Durchgänge, und der Kommentar nennt einen

---

`applicationWillTerminate:` trägt seit dieser Runde den Kommentar:

> `crates/krk-ui/src/appkit/anwendung.rs:814-817` — „**Ein Durchgang und nicht zwei.** Das
> Vormerken und das Beenden laufen unter derselben Schreibsperre; zwei Durchgänge liessen
> dazwischen eine andere Instanz schreiben, ohne dass es einen Grund dafuer gaebe."

Der Rückruf beginnt aber drei Zeilen darüber mit `self.sitzung_vormerken()`
(`crates/krk-ui/src/appkit/anwendung.rs:806`), und `sitzung_vormerken` nimmt die Sperre selbst
(`:5336-5344`). Beim Beenden laufen damit **zwei** Durchgänge hintereinander, und genau
dazwischen kann eine andere Instanz schreiben — der Fall, den der Kommentar ausschließt.

**Der erste Aufruf ist dazu wirkungslos.** `sitzung_vormerken` baut den Stand über
`sitzung_bauen()` und merkt ihn vor; die Zeile darunter baut denselben Stand ein zweites Mal
und überschreibt den vorgemerkten. Was der erste Durchgang schreibt, hängt allein am Takt aus
`SITZUNGSTAKT` und ist derselbe Inhalt. Der Aufruf steht seit der Runde 1 dort und war vor
dieser Runde billig; seit S12 kostet er ein `flock` und einen Schreibvorgang.

---

**Schwere:** gering. Kein Datenverlust: beide Durchgänge schreiben denselben Stand, und beide
stehen unter der Sperre. Ein Kommentar, der eine Zusage macht, die die Funktion nicht hält,
und ein doppelter Schreibvorgang auf dem Beendenpfad.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:805-822`

**Domain:** code

## Vorschlag

`self.sitzung_vormerken()` in `applicationWillTerminate:` streichen. Die zwei Zeilen darunter
tun dasselbe und mehr: sie bauen den Stand, merken ihn vor und schreiben ihn ohne Rücksicht auf
den Takt, alles in einem Durchgang. Danach stimmt der Kommentar wörtlich.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813, auf dem im Datensatz vorgeschlagenen Weg. `self.sitzung_vormerken()` ist aus `applicationWillTerminate:` gestrichen; die zwei Zeilen darunter bauen denselben Stand, merken ihn vor und schreiben ihn ohne Ruecksicht auf den Takt, alles in **einem** Durchgang. Der Kommentar „Ein Durchgang und nicht zwei" stimmt damit woertlich. An seiner Stelle steht jetzt eine Zeile, die sagt, was dort bis zur Runde 7 stand und warum es weg ist.

Die Blattregel sieht den Freigabedialog nicht, und niemand hat entschieden, was das heißt

---

KRK hält Tastenbefehle an, solange ein Blatt steht. Die Abfrage dafür ist
`Anwendungsdelegierter::blatt_steht`
(`crates/krk-ui/src/appkit/anwendung.rs:2063`):

```rust
fn blatt_steht(&self) -> bool {
    self.ivars()
        .fenster
        .get()
        .and_then(|fenster| fenster.attachedSheet())
        .is_some()
}
```

Sie fragt `NSWindow::attachedSheet`, und `kommando_ausfuehren` (`:2086`) weist
daraufhin jeden Befehl außer dem Abbruch ab
(`kommandos::operationen::waehrend_blatt_erlaubt`).

Der Freigabedialog aus C1 der Runde 6 ist **kein** Blatt. Er entsteht über
`showRelativeToRect:ofView:preferredEdge:` und hängt sich an eine Fläche, nicht
an das Fenster; `attachedSheet` liefert währenddessen `None`. Die Runde hat
damit zum ersten Mal einen stehenden Systemdialog, den die eine Sperre dieses
Programms nicht sieht, und keiner der elf Planschritte und keiner der vierzehn
Datensätze der Runde stellt die Frage, was in dieser Zeit mit einem Tastendruck
geschehen soll.

---

**Was offen ist**

`inference:` Erreichbar ist der Fall wahrscheinlich nicht. `showRelativeToRect:`
fährt eine Verfolgungsschleife wie jedes Kontextmenü, und Tastendrücke gehen
darin an das Menü. KRKs Abgriff ist ein lokaler Ereignisbeobachter
(`NSEvent::addLocalMonitorForEventsMatchingMask` auf `NSEventMask::KeyDown`,
`crates/krk-ui/src/appkit/ereignisse.rs:296`), und ob ein solcher Beobachter
während einer Menüverfolgung gerufen wird, ist eine Zusage von AppKit, die wir
nicht gelesen haben — dieselbe Art von Annahme, die Schritt 6 an der Vorschau
ausdrücklich nicht eingehen wollte („eine Flaeche ohne Menue waere der stille
Fehlschlag", `vorschau.rs`).

Kommt ein Tastendruck durch, ist die Wirkung nicht dramatisch, aber auch nicht
gewollt: `cmd+w` schlösse den Tab unter dem Dialog weg, `opt+cmd+o` wechselte
den Ordner, ein zweites `shift+cmd+s` setzte den festgehaltenen Wähler ab. Ein
Absturz droht nicht, weil der Wähler festgehalten wird
(`teilen.rs:115-136`).

**Was zu tun ist**

Am Bündel eine einzige Beobachtung machen: Freigabedialog über `shift+cmd+s`
öffnen und, während er steht, `cmd+w` drücken. Schließt sich der Tab, kommen
Tastenbefehle durch und die Frage ist fällig; geschieht nichts, ist sie
beantwortet und dieser Datensatz zu schließen.

Kommen sie durch, gibt es zwei Zuschnitte, und keiner ist hier gewählt:

1. `blatt_steht` bleibt, wie es ist, und daneben tritt eine zweite Abfrage
   „steht ein Freigabedialog". Das wären zwei Sperren mit einer Frage, und der
   Modulkopf von `anwendung.rs` hält heute ausdrücklich fest, dass es genau
   zwei Stellen mit **verschiedenen** Fragen gibt.
2. Die Abfrage wird zu „steht irgendetwas Modales vor dem Fenster" und
   beantwortet sich aus `attachedSheet` **oder** dem Halt in `teilen.rs`. Das
   bliebe bei einer Stelle, verlangt aber, dass der Halt weiß, wann der Dialog
   zugeht — heute weiß er es nicht, siehe
   `issues/260812-1529_o_die-besitzregel-des-freigabewaehlers-gilt-nur-in-einer-der-zwei-huellen.md`.

**Kontext**

- Der Rechtsklickweg ist von der Frage nicht betroffen. Er geht über
  `menuNeedsUpdate:` und `standardShareMenuItem`, also über AppKits eigenes
  Menü, und ein Blatt lässt einen Klick in das Fenster dahinter ohnehin nicht
  zu.
- Die beiden Datensätze hängen zusammen und lösen sich in einer Richtung
  gemeinsam: wer den Wähler nach dem Schließen freigibt (Möglichkeit 2 dort),
  hat zugleich die Auskunft, die Möglichkeit 2 hier braucht.
- Gefunden bei der Durchsicht von Turn 1 der Runde 6; nicht behoben.

---

**Nachtrag 260813: die neue Schlüsselfensterfrage der Runde 8, und was sie für diesen Datensatz leistet**

Die Zulässigkeitsregel hat seit Schritt A1 der Runde 8 einen vierten
Bestandteil: `Lage::schluesselfenster_gehoert_krk`. Er ist wahr, wenn
`NSApplication::keyWindow` KRKs Hauptfenster ist oder ein Blatt, das daran
hängt, und sonst falsch; erhoben wird er einmal je Eingabe in
`Anwendungsdelegierter::schluesselfenster`
(`crates/krk-ui/src/appkit/anwendung.rs`). Ist er falsch, kommt kein Befehl
mehr durch außer den beiden der Ausnahmeliste, `beenden` und
`fenster_schliessen`. Angelegt wurde er für den Über-Dialog aus C5 der Runde 8,
also für denselben blinden Fleck, den dieser Datensatz beschreibt.

**Was er leistet: jedes fremde Fenster ist damit erledigt**, und zwar an einer
Stelle und ohne zweite Abfrage. Der Zuschnitt 1 oben, „eine zweite Sperre
daneben", ist damit gegenstandslos; Zuschnitt 2 bleibt der einzige Weg, falls
noch einer nötig ist.

**Was er nicht leistet: den Freigabewähler erreicht er nicht.** Der Wähler ist
kein Fenster. `teilen.rs:222` zeigt ihn über
`showRelativeToRect:ofView:preferredEdge:`, also als Verfolgungsschleife an
einer Fläche. Bleibt das Hauptfenster dabei das Schlüsselfenster — und nichts
spricht dagegen —, antwortet `schluesselfenster_gehoert_krk` mit wahr, und die
neue Bedingung sperrt nichts. Der Befund liegt eigens als
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/issues/260813-1110_o_die-schluesselfensterfrage-erreicht-den-freigabewaehler-nicht-weil-er-kein-fenster-ist.md`.

**Dieser Datensatz bleibt deshalb offen.** Die Beobachtung, die er selbst
verlangt, ist unverändert fällig und steht als Schritt E2 im Plan der Runde 8:
Wähler über `shift+cmd+s` öffnen und, während er steht, `cmd+w` drücken.
Geschieht nichts, ist die Frage beantwortet und der Datensatz wird mit dem
Ergebnis geschlossen; schließt sich der Tab, bleibt er offen und trägt danach
einen gemessenen Befund statt einer Vermutung. Sie ist Nutzerarbeit, weil sie
KRK im Vordergrund verlangt.

---
Resolved: Am 260813-1410 am laufenden Bündel gemessen, im Rahmen der Abnahme E2 der achten Runde (`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1405-abnahmeliste-e2.md`, Beobachtung 11). Freigabewähler über `Shift+Cmd+S` geöffnet, dann `Cmd+W` gedrückt: es geschieht nichts, kein Tab schließt sich. Die in Turn 1 der achten Runde gebaute Schlüsselfensterbedingung (`zulaessigkeit::zulaessig`, viertes Feld `schluesselfenster_gehoert_krk`, Commit `c3ada4d`) fängt den Fall also, obwohl der Wähler über `showRelativeToRect:` als Verfolgungsschleife und nicht als Fenster entsteht. Die Vermutung des Plans in Schritt A3, sie erreiche ihn nicht, ist damit widerlegt statt bestätigt.

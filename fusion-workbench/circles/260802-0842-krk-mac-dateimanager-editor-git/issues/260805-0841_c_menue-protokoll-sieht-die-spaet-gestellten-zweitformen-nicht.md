`--menue-protokoll` sieht die spät gestellten Zweitformen nicht

---

Die Befehlszeilenmarke `--menue-protokoll` in `crates/krk-ui/src/appkit/menue.rs` verspricht, auszulesen statt aufzuzählen: "Ausgegeben wird deshalb, was am `NSMenu` wirklich hängt, einschließlich der verdeckten Zweitformen, die AppKit von sich aus dazustellt." Sie hält das nur zur Hälfte.

Die Marke liest das Menü unmittelbar nach `finishLaunching` aus und beendet. Zu diesem Zeitpunkt steht die Zweitform "Close All" von `performClose:` schon da, die Zweitform "Quit and Keep Windows" von `terminate:` noch nicht: AppKit stellt sie erst, wenn die Anwendung wirklich läuft. Der Befund vom 260805-0753 stammt deshalb aus der laufenden Anwendung über die Bedienungshilfen und nicht aus der Marke.

---

## Warum es zählt

Das Abnahmekriterium von C3 verlangt, dass das Hauptmenü der laufenden Anwendung genau die Kombinationen trägt, die die Belegung für seine Einträge führt, und keine weitere. `--menue-protokoll` ist das Werkzeug, mit dem dieses Kriterium geprüft werden soll. Solange es einen Teil der dazugestellten Zweitformen nicht sieht, kann eine grüne Ausgabe der Marke das Kriterium nicht belegen, und der Prüfer muss es nebenher wissen.

Der Fall ist heute keiner mehr, weil beide betroffenen Einträge einen eigenen Selektor tragen und AppKit ihnen nichts mehr dazustellt (gemessen am 260805-0841). Er wird wieder einer, sobald ein Menüeintrag einen AppKit-Selektor bekommt, zu dem es eine spät gestellte Zweitform gibt.

## Was zu prüfen ist

Ob ein späterer Auslesezeitpunkt die Marke vollständig macht. Denkbar ist, das Auslesen nicht unmittelbar nach `finishLaunching` zu tun, sondern über einen Zeitgeber oder aus `applicationDidFinishLaunching:` heraus, nachdem die Ereignisschleife einmal gelaufen ist. Ob das reicht, ist nicht gemessen; die Zweitform von "Quit and Keep Windows" könnte auch an der Aktivierung der Anwendung hängen und nicht an der Ereignisschleife.

Die Gegenprobe braucht eine Sonde, die `terminate:` vorübergehend wieder einträgt, so wie die Sonde vom 260805-0753 es mit `performClose:` getan hat: erst dann gibt es überhaupt wieder eine Zweitform, an der sich der Auslesezeitpunkt messen lässt.

Schlägt kein Auslesezeitpunkt an, ist die zweite Antwort, im Kopfkommentar der Marke festzuhalten, was sie nicht sieht, damit niemand ihre Ausgabe für vollständig hält.

---

Herkunft: benannt im Datensatz `issues/260805-0753_c_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md` als zweiter, kleinerer Defekt an derselben Stelle; beim Beheben von dessen Hauptteil am 260805-0841 nicht mitgeprüft und deshalb als eigener Datensatz herausgezogen.

---
Resolved: Nachgemessen mit einer Sonde — die Zweitform erscheint an keinem Auslesezeitpunkt, weil die Marke kein Fenster öffnet und die Anwendung deshalb nie aktiv wird. Es gilt die zweite Antwort des Defekts: der Kopf von protokollieren hält jetzt fest, was die Marke nicht sieht.

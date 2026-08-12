Zwei Modulköpfe nennen für NSLayoutManager macOS 10.0, das SDK sagt 10.7

---

Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` in
`crates/krk-ui/src/appkit/editor.rs` und in `crates/krk-ui/src/appkit/nummernspalte.rs`
führt `NSLayoutManager` ohne eigene Angabe und damit als „seit 10.0". Das SDK sagt etwas
anderes: `NSLayoutManager.h:65` trägt `API_AVAILABLE(macos(10.7), ios(7.0), tvos(9.0))`.

Nachgeprüft am 260812 gegen
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk`.

---

Folgenlos für den Bau und für das Bündel: 10.7 liegt weit unter dem Zielsystem 15.0, und
keine Klasse im Baum liegt darüber. Der Schaden ist die Angabe selbst. Sie ist die einzige
Gegenmaßnahme dieses Projekts gegen den Absturz, den `objc2` nicht abfängt, und sie ist
eine Gewohnheit ohne Werkzeug dahinter. Eine falsche Zahl in ihr ist schlimmer als eine
fehlende: eine fehlende sieht man, eine falsche wird geglaubt.

Gefunden vom `coder` beim Anlegen von `appkit/textmerkmale.rs` (Planschritt 7 der Runde 6).
Der neue Kopf jener Datei nennt die gelesene 10.7 und weist in einem Satz auf die Abweichung
hin. Die zwei vorhandenen Köpfe sind bewusst nicht angefasst worden: eine Korrektur allein
in `editor.rs` risse die beiden Stellen auseinander, und `nummernspalte.rs` lag außerhalb
der drei Dateien jenes Schrittes.

Zu beheben sind beide Köpfe in einem Zug. Ob dabei gleich die übrigen Angaben des Baumes
gegen das SDK gelesen werden, hängt an der offenen Frage
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`, die drei
Stufen mit ihren Kosten aufführt. Dieser Datensatz nimmt ihr nichts vorweg: er nennt eine
Zahl, die nachweislich falsch ist.

Herkunft: gemeinsamer Speicher. Die falsche Angabe ist älter als diese Runde und betrifft
eine projektweite Gewohnheit, nicht die Directive der Runde 6.

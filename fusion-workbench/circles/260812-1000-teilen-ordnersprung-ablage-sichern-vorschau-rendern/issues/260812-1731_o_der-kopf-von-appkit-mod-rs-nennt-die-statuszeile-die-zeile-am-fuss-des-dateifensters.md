Der Kopf von appkit/mod.rs nennt die Statuszeile „die Zeile an seinem Fuß"

---

`crates/krk-ui/src/appkit/mod.rs:76` beschreibt `statuszeile` als „die Zeile an seinem Fuss",
wobei „sein" das Dateifenster meint. Seit Planschritt 10 der Runde 6 stimmt das nicht mehr:
es gibt eine Statuszeile über die volle Fensterbreite, und die beiden Zeilen an den Füßen
der Dateifenster sind weg.

---

Der Modulkopf von `mod.rs` beschreibt jedes Modul des Verzeichnisses und ist damit die erste
Auskunft, die ein Leser über den Aufbau bekommt. Ein falscher Satz darin schickt ihn an die
falsche Stelle.

Gefunden vom `coder` beim Bau von Schritt 10. Die Datei gehört nicht zu den fünf jenes
Schrittes und ist deshalb nicht angefasst worden.

**Zusammen zu beheben mit
`260812-1702_o_der-kopf-von-appkit-mod-rs-sagt-die-vorschau-rufe-textmerkmale-noch-nicht.md`.**
Jener Datensatz nennt zwei weitere Stellen desselben Kopfes, die seit Schritt 9 falsch sind.
Drei falsche Sätze in einem Modulkopf, eine Änderung. Getrennt abgelegt sind sie, weil jede
Aussage für sich falsch ist; behoben gehören sie in einem Zug, sonst bleibt der Kopf nach der
Reparatur weiterhin teilweise falsch.

Wer sie behebt, liest den ganzen Kopf gegen den Baum und nicht nur die drei genannten Sätze:
die Runde hat drei Module hinzugefügt (`teilen`, `textmerkmale`, dazu die neue Rolle der
Vorschau), und die Vermutung liegt nahe, dass mehr als drei Sätze veraltet sind.

Also seen: 260826-1416 by coderev — `mod.rs:95-96` sagt weiter „[statuszeile] die Zeile an seinem Fuss"; `fenster.rs:3-7` und `statuszeile.rs:34-42` führen sie als Schwester der Aufteilung über die volle Breite.

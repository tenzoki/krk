Ein toter Netzpfad lässt den Lesefaden hängen, und das Abbruchkennzeichen erreicht ihn nicht

---

Ein vom Finder eingehängtes Netzlaufwerk, dessen Server verschwindet, lässt
Systemaufrufe darauf blockieren, statt sie scheitern zu lassen. Der
Verzeichnisleser aus S2 läuft auf einem Arbeitsfaden und prüft sein
Abbruchkennzeichen **zwischen** zwei Systemaufrufen; ein Aufruf, der nie
zurückkehrt, wird davon nicht erreicht. Je Navigationsversuch auf einen solchen
Pfad bliebe damit ein Faden liegen.

---

`inference:` Nicht gemessen. Zum Nachprüfen fehlt ein Server, und die Prüfung
selbst dürfte nur gegen einen eigens dafür aufgesetzten laufen. Die Aussage
stützt sich auf das dokumentierte Verhalten blockierender Dateisystemaufrufe
auf einem nicht mehr erreichbaren Mount und auf die Umsetzung von S2, in der
`Lesevorgang::drop` das Abbruchkennzeichen setzt und der Lesefaden es vor jedem
Systemaufruf und zwischen zwei Stapeln liest.

**Was davon nicht betroffen ist.** Die Zusage aus C9, ein Dateifenster blockiere
nicht, hält: der Lesevorgang läuft auf einem Arbeitsfaden, und die Oberfläche
bleibt bedienbar. Betroffen ist allein der Faden selbst.

**Warum es einen eigenen Datensatz bekommt.** Die Beobachtung stand als zweiter
Teil in `issues/260804-1451_c_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf.md`.
Jener Defekt ist am 260805-0000 durch die Nutzerentscheidung geschlossen, die
selbsttätige Auffrischung auf lokale Dateisysteme einzuengen. Diese
Beobachtung löst die Einengung **nicht** mit auf: sie betrifft den Zugriff und
nicht die Auffrischung, und C9 sagt den Zugriff auf einen eingehängten Netzpfad
weiterhin zu. Ohne eigenen Datensatz wäre sie mit dem beantworteten Defekt
verschwunden.

**Wann es fällig ist.** Kein Schritt hängt daran, und keiner der ausstehenden
Schritte berührt es. Zu klären wäre es, bevor jemand die Zusage aus C9 auf einem
Netzpfad abnimmt, und zu messen erst, wenn ein Server dafür bereitsteht.
Denkbare Wege sind eine Zeitschranke am Lesevorgang oder das Aufgeben des Fadens
ohne ihn abzuwarten; beide sind Entwurfsentscheidungen und keine Kleinigkeit,
weil die erste einen Zeitgeber in einen Pfad bringt, der heute ohne auskommt.

**Aufgefallen bei:** der Beantwortung der Nutzerfragen am 260805-0000,
`history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`. Ursprünglich
gefunden bei der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1451_c_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C9),
`crates/krk-core/src/verzeichnis/leser.rs`

---

## Nachtrag vom 260805-0947: geprüft, aber nicht behoben

Der Aufräumdurchgang vom 260805 hat diesen Datensatz angesehen und **liegen gelassen**. Der Datensatz bleibt offen. Was dabei geprüft wurde, damit die nächste Runde nicht bei null anfängt:

**Der Umfang des Schadens ist bestätigt, gelesen und nicht gemessen.** Kein Aufrufer in `krk-ui` wartet je auf einen Lesefaden. `Tabliste::lesen_starten` setzt `tab.lesevorgang = None`, und `Lesevorgang::drop` setzt allein das Abbruchkennzeichen, ohne `join`. `Tabliste::abbrechen` nimmt den Vorgang heraus, ruft `abbrechen` und lässt ihn fallen. Der einzige Aufrufer von `Lesevorgang::warten` im ganzen Arbeitsbereich ist `krk-bench/src/messen.rs:251`. Die Oberfläche kann also nicht hängen bleiben; die Zusage aus C9 hält, wie der Datensatz oben schreibt. Was bleibt, ist je Navigationsversuch ein Faden, der in `File::open` oder in `getattrlistbulk` steht und dort bleibt.

**Der zweite der beiden im Datensatz genannten Wege ist bereits gegangen.** "Das Aufgeben des Fadens ohne ihn abzuwarten" ist der heutige Zustand. Er löst den Fadenverlust nicht, er ist seine Ursache: aufgeben kann KRK den Faden, beenden nicht. POSIX und Rust geben keinen sicheren Weg, einen Faden von außen abzuräumen, der in einem Systemaufruf steht.

**Damit bleibt allein der erste Weg, und er ist eine Entwurfsentscheidung.** Eine Zeitschranke bringt einen Zeitgeber in einen Pfad, der heute ohne auskommt, und sie beendet den hängenden Faden auch dann nicht; sie könnte nur verhindern, dass ein zweiter dazukommt. Denkbar wäre daneben eine Sonderbehandlung für Netzeinhängungen, also eine Klasse von Einhängepunkten mit eigener Regel — genau die Sorte Sonderfall, die die Maxime "supersimpel" ausschließt und die zuerst begründet sein will.

**Und ohne Server bleibt jede Wahl unbelegt.** Der Datensatz sagt es selbst: zu messen erst, wenn ein Server dafür bereitsteht. Ein eigens aufgesetzter Server, den man mitten im Lesen abschaltet, ist die Voraussetzung dafür, überhaupt zu sehen, an welchem Systemaufruf es hängt und ob eine Zeitschranke ihn erreicht.

Der Aufwand liegt damit über dem der übrigen sieben Aufräumdefekte desselben Durchgangs zusammen, und das Ergebnis wäre ohne Messung eine Vermutung mehr. Fällig bleibt es, wie oben, vor der Abnahme von C9 auf einem Netzpfad.

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

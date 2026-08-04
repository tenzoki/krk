Auf einem Netzlaufwerk frischt KRK fremde Änderungen nicht auf

---

C9 schließt die vom Finder eingehängten Netzlaufwerke ausdrücklich ein: "Ein vom
Finder verbundenes Netzlaufwerk erscheint damit als gewöhnlicher Pfad und ist
eingeschlossen." Die Dateisystem-Beobachtung aus S14 läuft über FSEvents, und
FSEvents deckt Netzdateisysteme nicht ab. Ein Dateifenster auf einem SMB- oder
NFS-Pfad zeigt fremde Änderungen deshalb erst, wenn der Nutzer den Ordner
wechselt und wieder zurückkommt.

`inference:` Nicht gemessen. FSEvents wird vom Kern für lokal eingehängte
Dateisysteme geführt; ein SMB- oder NFS-Pfad erzeugt keine Ereignisse. Zum
Nachprüfen fehlte am 260804 ein Server, und die Prüfung selbst dürfte nur gegen
einen eigens dafür aufgesetzten laufen.

---

**Wen es trifft und wen nicht.** Der Zugriff selbst ist nicht betroffen: Lesen,
Navigieren und die Dateioperationen aus C4 laufen über gewöhnliche
Systemaufrufe und funktionieren auf einem eingehängten Netzpfad. Betroffen ist
allein die Zusage aus `### Frage 3` des Plans, dass KRK auf Änderungen reagiert,
die eine andere Anwendung verursacht hat. Eigene Änderungen erreichen die
Ansicht weiterhin, weil S16 denselben Auffrischungspfad über den gemeldeten
Abschluss einer Dateioperation anstößt und nicht über FSEvents.

**Eine zweite Beobachtung zum selben Gegenstand, ebenfalls ungeprüft.** Ein
Netzlaufwerk, dessen Server verschwindet, blockiert Systemaufrufe darauf, statt
sie scheitern zu lassen. Der Verzeichnisleser aus S2 läuft auf einem
Arbeitsfaden, die Oberfläche bleibt also bedienbar, und C9s Zusage "statt zu
blockieren" hält. Der Arbeitsfaden selbst hinge aber fest:
`Lesevorgang::drop` setzt ein Abbruchkennzeichen, das der Faden zwischen zwei
Systemaufrufen prüft, und ein hängender Aufruf kehrt nie zurück. Je Versuch
bliebe ein Faden liegen.

**Was zu entscheiden wäre.** Ob C9 die Einschränkung ausschreibt (etwa: die
Auffrischung gilt für lokale Datenträger, auf Netzlaufwerken frischt der Nutzer
mit einem Befehl auf), oder ob eine spätere Runde einen zweiten Mechanismus
bekommt. Ein Abfragetakt neben FSEvents wäre der zweite Auffrischungsweg, den
S14 ausdrücklich ausschließt; er müsste am selben `ordner_neu_lesen` enden.

**Aufgefallen bei:** der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.

---
Resolved: Nutzerentscheidung vom 260805-0000. C9 wird auf lokale Dateisysteme eingeengt; einen zweiten Auffrischungsweg bekommt KRK nicht. Begründung des Nutzers: ein Abfragetakt neben FSEvents widerspräche der Ein-Weg-Regel, die dieser Plan an vier Stellen durchhält, und nachprüfen ließe er sich ohne Server ohnehin nicht. C9 schreibt jetzt als Abnahmekriterium aus, was der Nutzer auf einem Netzpfad stattdessen erlebt: Lesen, Navigieren und die Dateioperationen aus C4 wirken wie auf einem lokalen Pfad, eigene Änderungen erscheinen weiterhin ohne Zutun, weil die abgeschlossene Operation die Auffrischung selbst anstößt, und eine fremde Änderung holt er durch Verlassen und Wiederbetreten des Ordners herein. Ein eigener Auffrischungsbefehl entsteht in dieser Runde nicht; die ausgelieferte Belegung kennt keinen. Eingearbeitet in C9 des Specs und in `### Frage 3` des Plans; kein neuer Schritt.

**Die zweite Beobachtung dieses Datensatzes ist damit nicht beantwortet und steht weiter.** Ein Netzpfad, dessen Server verschwindet, lässt Systemaufrufe hängen statt scheitern, und der Lesefaden prüft sein Abbruchkennzeichen zwischen zwei Aufrufen; je Versuch bliebe ein Faden liegen. Sie betrifft den Zugriff und nicht die Auffrischung, wird von der Einengung also nicht mit erledigt und liegt jetzt als eigener Datensatz: `issues/260805-0000_o_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`.

Entscheidungsdatensatz `decisions/260805-0000_a_auffrischung-auf-netzlaufwerken.md`. Sitzungsbericht `history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`.

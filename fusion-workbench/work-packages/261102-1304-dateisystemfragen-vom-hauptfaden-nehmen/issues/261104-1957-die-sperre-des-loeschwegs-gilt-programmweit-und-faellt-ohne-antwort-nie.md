Die Sperre `loeschpruefung` gilt für das ganze Programm und fällt nie, wenn die Antwort des Fadens ausbleibt
---
Schwere: Mittel. Seit `5cdf39e3`. Eine Löschfrage auf einer hängenden Freigabe sperrt jedes weitere Löschen, auch auf der lokalen Platte und im anderen Dateifenster, bis der Faden zurückkommt, und ohne Antwort bis zum Neustart.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/anwendung.rs:1444` bis `:1447`: ein `Cell<bool>` im Delegierten. `:8247` bis `:8250`: steht es, meldet jeder Löschbefehl `Text::LoeschpruefungLaeuftNoch` und kehrt zurück, und diese Abfrage steht vor der Unterscheidung nach lokal, fern und Archiv. `:8365`: allein `loeschbefund_einziehen` löst sie. Es gibt keine Frist und keinen Abbruch.

**Fehlerbild 1, die Frage kehrt nicht zurück.** `lokaler_loeschbefund` (`:910`) beginnt mit `canonicalize` auf dem angezeigten Ordner. Auf einer Freigabe ohne Zeitlimit wartet der Faden unbegrenzt. Von da an löscht KRK nirgends mehr, auch nicht im lokalen Ordner des anderen Dateifensters und nicht auf einem Server, bis der Nutzer das Programm neu startet. `inference:` ein hart eingehängtes NFS verhält sich so; an einer hängenden Freigabe gemessen ist das nicht. Der erste Tastendruck gibt dabei keine Rückmeldung, erst der zweite sagt, dass eine Prüfung läuft, und er sagt nicht, wo.

**Fehlerbild 2, die Arbeit stirbt.** `crates/krk-ui/src/appkit/nebenfaden.rs:29`: „`fertig` wird in jedem Fall genau einmal gerufen“, ausgenommen eine stehende Hauptschlange. Das stimmt nicht für eine Panik in `arbeit` (`:81` bis `:84`): `nehmen` wird nie erreicht, und `fertig` fällt mit dem Halter. `Cargo.toml` setzt kein `panic`, es gilt also `unwind`. Für den Löschweg heißt das dieselbe bleibende Sperre. Eine konkrete Panikstelle in `lokaler_loeschbefund` habe ich nicht gesucht. Der Satz im Modulkopf ist unabhängig davon falsch.

**Richtung.** Ich schlage eine Nummer statt des Wahrheitswerts vor: jeder lokale Löschbefehl bekommt eine neue Nummer, die Einzugsmethode verwirft jede Antwort außer der mit der jüngsten, und ein neuer Befehl ersetzt die laufende Frage, statt an ihr zu scheitern. Fern und Archiv brauchen sie nicht zu fragen. Dann bleibt der hängende Faden stehen und sperrt nichts mehr. Für die Panik gehört der Rückweg in `nebenfaden.rs`, etwa `catch_unwind` um `arbeit`, damit die Zusage des Modulkopfs für alle Rufer gilt; das ist ungeprüft, weil `T` dafür einen Ersatzwert aus `startfehler` braucht, dessen Typ heute einen `io::Error` nimmt.

**Abnahme.** Eine reine Probe oder Rumpfprobe zeigt: Während eine lokale Frage läuft, erreicht ein Löschbefehl an einem anderen Ort die Stufentafel, und eine überholte Antwort öffnet kein Blatt. Der Satz in `nebenfaden.rs` stimmt mit dem Verhalten bei einer Panik überein. Der Lauf an einer hängenden Freigabe bleibt Nutzerarbeit.

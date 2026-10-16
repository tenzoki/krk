Der Löschweg fragt `canonicalize` und den Papierkorb auf dem Hauptfaden, und ein hängendes Netzlaufwerk hält die Ereignisschleife an
---
Vor der Rückfrage zum Löschen ruft der Anwendungsdelegierte `std::fs::canonicalize(&quellordner)` und `papierkorb::fuehrt_einen_papierkorb` auf dem Hauptfaden (`crates/krk-ui/src/appkit/anwendung.rs`, Rumpf von `loeschen_nach_rueckfrage`, Abschluss an `loeschwarnung::vor_der_rueckfrage`). Liegt der angezeigte Ordner auf einer eingehängten Freigabe, deren Server nicht antwortet, wartet jeder der zwei Aufrufe auf das Zeitlimit des Dateisystems, und die Ereignisschleife steht so lange; L1 und L9 aus C8 fielen dann im Betrieb. Dieselbe Rückfrage zählt über `umfang::zaehlen` die Auswahl, ebenfalls vor dem Blatt (`loeschtexte`).
---
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>

Gefunden beim Planen des Pakets `261016-1945-sftp-ordner-im-dateifenster-wie-lokal`; die Machbarkeitsanalyse `261016-1746-sftp-verbindung-als-dateisystem-machbarkeit.md` nennt die Stelle in Abschnitt 2. Das Paket berührt den Löschweg für ferne Orte und für Orte ohne Papierkorb, stellt für einen fernen Ort keine dieser Fragen und behebt den Defekt für lokale Ordner nicht; der lokale Weg bleibt nach seiner Directive unverändert.

Beleg: am Quelltext gelesen (Stand `c461b3a1`), nicht an einer hängenden Freigabe gemessen.

Abnahme: Keiner der drei Aufrufe (`canonicalize`, `fuehrt_einen_papierkorb`, `umfang::zaehlen`) steht im Rumpf eines Weges, der vom Tastendruck bis zum Blatt auf dem Hauptfaden läuft, gehalten von einer Quelltextprobe nach dem Muster `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat`. Danach schuldet die Änderung einen Abnahmelauf gegen L1 und L9.

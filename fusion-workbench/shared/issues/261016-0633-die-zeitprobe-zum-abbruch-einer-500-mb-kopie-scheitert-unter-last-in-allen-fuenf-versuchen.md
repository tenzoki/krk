Die Zeitprobe zum Abbruch einer 500-MB-Kopie scheitert unter Last in allen fünf Versuchen
---
`der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` (`crates/krk-core/tests/operation.rs`) ist am 261016 in mehreren `make check`-Läufen rot geworden, gemessen 120 bis 330 ms statt höchstens 100, und allein gefahren in zwei von drei Läufen ebenfalls rot. Der Kopiercode (`krk-core/src/operation/`) ist in diesem Zeitraum nicht geändert worden; die Commits `ea6dac2..9d1b111` fassen ihn nicht an.
---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Lage beim Scheitern: Lastmittel 4,74 / 3,94 / 3,26 (`uptime`, 261016-0633), freier Plattenplatz 13 GiB. Der Doc-Kommentar der Probe rechnet unter künstlicher Last mit knapp 30 Prozent Fehlschlag je Versuch und daraus mit etwa zwei von tausend roten Läufen; beobachtet sind hier mehrere rote Läufe in Folge. Entweder ist die Last dieser Maschine stärker als die gemessene, oder die Abbruchspanne des Kopierens ist gewachsen.

Beleg: `make check`-Läufe dieser Sitzung; `cargo test -q -p krk-core --test operation der_abbruch_mitten` dreimal hintereinander, Ergebnis ok, FAILED, FAILED.

Abnahme: Auf einer ruhigen Maschine (Lastmittel unter 1) besteht die Probe in zehn von zehn Einzelläufen, oder eine Messung zeigt, dass die Abbruchspanne selbst über 100 ms liegt, und der Defekt wird im Kopiercode behoben. Die Zusage L-Abbruch aus C8 wird dabei nicht gedehnt.

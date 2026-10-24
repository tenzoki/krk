# Die Zeitprobe zum Abbruch in einer 500-MB-Datei scheitert auch ohne Änderung in drei von fünf Läufen

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Cross-references:** work-packages/261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen (dort zuerst beobachtet)

---

## Befund

`der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` (`crates/krk-core/tests/operation.rs`, um Zeile 445) hält die Zusage aus C8: ein Abbruch mitten in einer Datei kehrt binnen 100 ms zurück. Am 261023/261024 ist sie in jedem vollen `make check` mehrerer Läufe rot geworden (gemessen 158 bis 441 ms), einzeln mal grün, mal rot.

Gegenprobe am 261025: auf `dc4ed474`, also vor jeder Änderung des SQLite-Pakets, scheitert dieselbe Probe allein gefahren in 3 von 5 Läufen; auf `a1fec2b7` in 2 von 3. Der Defekt liegt damit nicht im SQLite-Paket.

Die Platte des Geräts hatte dabei rund 13 GiB frei (`df -h /`); die Probe schreibt eine 500-MB-Datei. Ob Plattenfüllstand, Last oder eine Regression außerhalb des Pakets die Ursache ist, ist nicht untersucht.

## Erwartet

Ursache klären, bevor die Probe oder die Zusage angefasst wird: dieselbe Probe bei mehr freiem Platz und ohne parallele Läufe wiederholen. Hält sie dann, ist es eine Eigenschaft des Geräts; hält sie nicht, gehört die Zusage L-Abbruch auf die Gegenstände der späteren Messrunde (CLAUDE.md, Abschnitt „Maximen“). Die Erwartung der Probe wird nicht aufgeweicht, ohne dass der Nutzer den Befund gesehen hat.

## Folge bis dahin

`make check` endet auf diesem Gerät häufig mit exit 2, obwohl alles andere grün ist. Executors fahren den Lauf dann erneut und berichten es.

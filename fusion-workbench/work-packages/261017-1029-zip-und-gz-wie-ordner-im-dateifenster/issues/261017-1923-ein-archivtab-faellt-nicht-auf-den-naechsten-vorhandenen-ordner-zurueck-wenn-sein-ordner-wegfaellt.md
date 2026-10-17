Ein Archivtab fällt nicht auf den nächsten vorhandenen Ordner zurück, wenn sein Ordner wegfällt
---
Löscht oder benennt ein Umbau einen Ordner im Archiv, den ein anderer Tab gerade zeigt, liest jener Tab nach der Auffrischung neu und meldet allein `Text::ArchivOrdnerFehlt`, „Den Ordner … gibt es im Archiv nicht mehr.“. Er bleibt auf einem Ort stehen, den es nicht gibt. Der Nutzer muss mit Pfeil links hinaus.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Bedienkomfort; kein Schaden)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-ui/src/tabs.rs`, `archivfehlertext`: `Archivfehler::KeinOrdner => satz(Text::ArchivOrdnerFehlt, &[("ort", &ort.anzeige())])`. Ein Rückfall auf den nächsten vorhandenen Vorfahren steht dort und in `archivordner` nicht.
- `inference:`, nicht nachgelesen: ob ein lokaler Tab in derselben Lage auf einen Vorfahren zurückfällt, habe ich nicht geprüft. Die Durchsicht der Oberfläche hat es so berichtet.

**Behebung (Richtung)**

Fehlt der Innenpfad im frisch gelesenen Verzeichnis, geht der Tab auf den nächsten vorhandenen Vorfahren im Archiv, notfalls auf die Wurzel, und die Statuszeile nennt den Grund. Das wird an dem Verhalten eines lokalen Tabs ausgerichtet, sobald es nachgelesen ist.

**Abnahme**

Eine Probe am Tabmodell: ein Archivtab auf `a/b`, das Verzeichnis ohne `a/b`. Danach steht der Tab auf `a` oder auf der Wurzel, und die Meldung nennt `a/b`.

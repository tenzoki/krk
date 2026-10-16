„Zur Zwischenablage springen“ liest in einem fernen Tab einen lokalen Pfad als Serverpfad
---
`Kommando::ZwischenablageSpringen` trägt `Ortsbedarf::Ueberall`. In einem fernen Tab wird der Pfad aus der Zwischenablage deshalb als Pfad auf dem Server angesprungen. Was dort steht, ist aber meist ein lokaler Pfad, etwa aus dem Finder oder aus „Pfad kopieren“. Dann führt der ferne Tab auf einen gleichnamigen Ort am Server, oder ein Dateipfad endet in einem Lesefehler statt in einer Auswahl.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-ui/src/appkit/tabelle.rs`: Zwischenablagesprung → `pfad_anspringen` → `fern_pfad_anspringen`, wenn der Tab fern ist.
- `crates/krk-core/src/tasten/belegung.rs`, `Kommando::ortsbedarf`: `ZwischenablageSpringen` steht unter `Ueberall`. Plan E13 nennt das Kommando nicht.

**Behebung (Richtung)**

Das ist bewusst zu entscheiden. Möglich ist, in einem fernen Tab einen absoluten lokalen Pfad lokal anzuspringen, also den Tab auf lokal zu setzen. Möglich ist auch, das Kommando auf `Lokal` zu stellen und es am fernen Ort auszugrauen. Welche Lösung gilt, steht danach in `HowTo.md`.

**Abnahme**

Eine Probe der Zulässigkeit oder des Sprungs zeigt das gewählte Verhalten für einen fernen Tab und einen absoluten lokalen Pfad in der Zwischenablage. `an_einem_fernen_ort_sind_genau_diese_befehle_ausgegraut` ist entsprechend nachgezogen.

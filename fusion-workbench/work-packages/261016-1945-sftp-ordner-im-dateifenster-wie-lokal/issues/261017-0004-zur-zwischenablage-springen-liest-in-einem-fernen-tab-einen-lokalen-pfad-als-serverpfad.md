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

---

Resolved: gewaehlt ist der lokale Sprung, das Kommando bleibt `Ortsbedarf::Ueberall` und `an_einem_fernen_ort_sind_genau_diese_befehle_ausgegraut` unveraendert. `crates/krk-ui/src/appkit/tabelle.rs`: `zwischenablage_springen` geht ueber `lokalen_pfad_anspringen`, das in einem fernen Tab mit `pfadeingabe::pruefen_fuer_fernen_tab` prueft und ueber `ergebnis_anwenden` und `ordner_lesen` den Tab auf den lokalen Ordner stellt; die Pfadeingabe von Hand bleibt beim Serverpfad. `crates/krk-ui/src/kommandos/pfadeingabe.rs`: `pruefen_fuer_fernen_tab` (eine Datei fuehrt immer in ihren Ordner), Modulkopf, Abschnitt „In einem fernen Tab“. `HowTo.md`, „Ordner auf einem Server“, Punkt „Bewegen“: der Pfad in der Zwischenablage gilt immer als Pfad auf dem Mac. Proben im Pruefmodul von `pfadeingabe.rs`: `fuer_einen_fernen_tab_fuehrt_eine_lokale_datei_in_ihren_ordner` und die Rumpfprobe `die_zwischenablage_liest_in_einem_fernen_tab_einen_lokalen_pfad`.

Das Entfernen nach dem Verschieben aus einem Archiv prüft nicht, ob das Archiv seit dem Kopieren dasselbe ist
---
Beim Verschieben aus einem Archiv kopiert die Maschine zuerst und entfernt danach die angekommenen Einträge mit einem zweiten Umbau. Dieser Umbau liest das Inhaltsverzeichnis zur **dann** gültigen Kennung der Datei und entfernt die Pfade aus `Entfernposten`. Ob das die Datei ist, aus der kopiert wurde, fragt er nicht. Ersetzt ein anderes Programm das Archiv zwischen Kopieren und Entfernen, entfernt KRK Einträge, deren jetziger Inhalt nie am Ziel angekommen ist. Das betrifft den Weg „Archiv nach lokal oder fern“ und den Weg „Archiv in ein anderes Archiv“.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Datenverlust, setzt eine Änderung von außen während des Vorgangs voraus; der Rest des Pakets schützt genau gegen diese Lage)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/operation/archiv.rs`: `Entfernposten { zugang, innen, bezeichnung }` trägt keine Kennung. `aus_archiven_entfernen` baut daraus `Umbauschritt::Entfernen` und ruft `umbau::umbauen`.
- `crates/krk-core/src/archiv/umbau.rs`, `umbauen_mit`: die Kennung stammt aus dem `fstat` des eben geöffneten Deskriptors, und `verzeichnis_zur_kennung` holt das Verzeichnis zu genau dieser Kennung. Eine erwartete Kennung des Rufers gibt es nicht.
- `crates/krk-core/src/archiv/lesen.rs`: der `Archivleser`, aus dem kopiert wurde, hält `verzeichnis.kennung()`. Diese Kennung fragt beim Entfernen niemand.
- Das Fenster reicht vom Lesen des ersten Eintrags bis zum Ende des Vorgangs. Darin liegen alle Konfliktfragen und beim Weg in ein anderes Archiv der ganze Umbau des Zielarchivs.

**Behebung (Richtung)**

`Entfernposten` trägt die Kennung des Verzeichnisses, aus dem kopiert wurde. `umbauen` bekommt eine erwartete Kennung und endet mit `Umbaufehler::VonAussenVeraendert`, wenn die Datei sie nicht mehr trägt. Die Einträge stehen dann doppelt da und im Bericht, wie bei jedem anderen gescheiterten zweiten Umbau.

**Abnahme**

Eine Probe verschiebt einen Eintrag aus einem Zip in einen lokalen Ordner. Nach dem Kopieren und vor dem Entfernen ersetzt sie die Archivdatei durch ein anderes Zip mit demselben Eintragsnamen, etwa an der Konfliktfrage eines zweiten Eintrags. Danach steht das ersetzende Zip Byte für Byte unverändert da, und der Bericht nennt den Eintrag als „kopiert, aber in der Quelle geblieben“.

---
Resolved: `crates/krk-core/src/operation/archiv.rs`, `Entfernposten` traegt die Kennung des Inhaltsverzeichnisses, aus dem kopiert wurde (gesetzt in `hinein` ueber `aus_einem_archiv` und in `operation/fern.rs`, `aus_dem_archiv_verschieben`); `aus_archiven_entfernen` baut ueber das neue `umbau::umbauen_an_kennung` (`crates/krk-core/src/archiv/umbau.rs`) um, das bei abweichender Kennung mit `Umbaufehler::VonAussenVeraendert` endet, bevor es ein Byte schreibt, und weist eine Gruppe mit zwei verschiedenen Kennungen ebenso ab. Probe `das_entfernen_nach_dem_verschieben_trifft_allein_das_archiv_aus_dem_kopiert_wurde` in `crates/krk-core/tests/archiv_operation.rs` (alle drei Formate, Ersetzen an der Konfliktfrage des zweiten Eintrags; ohne Behebung rot).

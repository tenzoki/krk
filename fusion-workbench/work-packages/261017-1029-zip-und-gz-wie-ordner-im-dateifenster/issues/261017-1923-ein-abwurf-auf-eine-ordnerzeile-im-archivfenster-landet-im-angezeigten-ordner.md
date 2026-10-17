Ein Abwurf auf eine Ordnerzeile im Archivfenster landet im angezeigten Ordner
---
Lokal und fern ist beim Abwurf auf eine Ordnerzeile dieser Ordner das Ziel. In einem Archivtab kennt `ort_in_zeile` die Zeile nicht und antwortet `None`. Die Abwurfregel setzt dann die Marke „Liste“, und das Ziel wird der angezeigte Ordner. Die Umrandung der ganzen Liste zeigt das an, es ist also sichtbar. Es weicht aber von „wie in einem Ordner“ ab, und der Plan nennt die Abweichung nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (sichtbare Abweichung, kein Schaden)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-ui/src/appkit/tabelle.rs`, `ort_in_zeile`: `match self.eintrag_in_zeile(zeile) { Some((pfad, typ)) => Some((Ort::Lokal(pfad), typ)), None => self.fern_eintrag_in_zeile(zeile).map(…Ort::Fern…) }`. Ein Arm für `archiv_eintrag_in_zeile` fehlt, obwohl es die Funktion in derselben Datei gibt.

**Behebung (Richtung)**

`ort_in_zeile` bekommt einen dritten Arm über `archiv_eintrag_in_zeile`, oder die Abweichung steht im Doc-Kommentar und in `HowTo.md`.

**Abnahme**

Eine Probe der Abwurfregel: in einem Archivtab ist bei einem Abwurf auf eine Ordnerzeile das Ziel der Archivort dieses Ordners.

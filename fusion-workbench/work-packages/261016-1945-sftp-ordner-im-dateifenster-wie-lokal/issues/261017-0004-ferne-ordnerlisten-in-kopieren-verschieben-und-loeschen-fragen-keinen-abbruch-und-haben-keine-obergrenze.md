Ferne Ordnerlisten in Kopieren, Verschieben und Löschen fragen keinen Abbruch und haben keine Obergrenze
---
`fern_namen` liest `READDIR` bis `EOF` in einen `Vec`. Dabei fragt es nicht nach dem Abbruch und zählt nicht mit. Ein Abbruch wirkt bei einem großen fernen Ordner erst nach dem vollständigen Lesen. Ein Server, der nie `EOF` sendet, hält den Vorgang unabbrechbar fest, und der Speicher wächst dabei. Auch `fern_baum_entfernen` fragt innerhalb eines Baums nie nach dem Abbruch. Der ferne Lesevorgang des Dateifensters macht es richtig: er fragt vor jedem `READDIR`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/operation/fern.rs`, `fern_namen`: `loop { match gespraech.readdir(pfand, &handle) { Ok(Some(stapel)) => namen.extend(stapel), … } }`.
- Rufer: `Stelle::liste` (Kopieren, Verschieben) und `fern_baum_entfernen`.
- Gegenbeispiel: `crates/krk-core/src/verzeichnis/leser.rs`, der ferne Zweig fragt den Abbruch vor jedem `READDIR`.

**Behebung (Richtung)**

`fern_namen` bekommt die Abbruchfrage der `Steuerung` hereingereicht und fragt sie vor jedem `READDIR`. `fern_baum_entfernen` fragt sie je Eintrag.

**Abnahme**

Eine Probe bricht einen Vorgang über einem fernen Ordner mit 10 000 Einträgen ab, während er liest. Der Bericht des Vorgangs kommt, bevor alle `READDIR` beantwortet sind. Messbar ist das etwa an einer Zählung der Anfragen oder an einem Server, der nach dem ersten Stapel nicht weiterantwortet.

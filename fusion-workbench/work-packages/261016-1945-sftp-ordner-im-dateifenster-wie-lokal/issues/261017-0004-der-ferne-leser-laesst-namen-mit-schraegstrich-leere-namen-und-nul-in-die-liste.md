Der ferne Leser lässt Namen mit Schrägstrich, leere Namen und NUL in die Liste
---
`fern_eintrag` lässt `.`, `..` und Namen ohne UTF-8 fallen, nicht aber Namen mit `/`, leere Namen und Namen mit NUL. Ein Server kann so eine Zeile wie `../../x` in die Dateiliste bringen. Ausbrechen kann sie nicht: jede ferne Stelle entsteht über `Fernpfad::verbinden`, das solche Namen abweist. Der Eintrag steht aber sichtbar in der Liste, und jede Handlung an ihm scheitert mit einer unpassenden Meldung.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/verzeichnis/leser.rs`, `fern_eintrag`: es filtert nur `.`, `..` und Namen ohne UTF-8.
- `crates/krk-core/src/fern/pfad.rs`, `Fernpfad::verbinden`: `if name.is_empty() || name == "." || name == ".." || name.contains('/') { return None; }`. NUL wird angenommen.
- Keine Probe fährt einen Server, der solche Namen liefert.

**Behebung (Richtung)**

Eine einzige Namensregel für ferne Namen, die `verbinden` und `fern_eintrag` beide fragen. Sie weist leere Namen, `.`, `..`, `/` und NUL ab. Abgewiesene Namen zählt der Leser mit den übersprungenen, so wie bei fehlendem UTF-8.

**Abnahme**

Eine Probe ruft die Namensregel mit `""`, `"a/b"`, `"a\0b"`, `"."` und `".."` auf: jeder Aufruf wird abgewiesen, `"a.b"` wird angenommen. Die Zählung des Lesers schließt die abgewiesenen Namen ein.

---

Resolved: `crates/krk-core/src/fern/pfad.rs`, neue Namensregel `ist_einzelner_name` (weist leere Namen, `.`, `..`, `/` und NUL ab), gerufen von `Fernpfad::verbinden` und von `fern_eintrag` in `crates/krk-core/src/verzeichnis/leser.rs`, das abgewiesene Namen als `Fernname::KeinEinzelnerName` liefert; `namen_aufnehmen` zaehlt sie mit den Namen ohne UTF-8, `.` und `..` zaehlen weiter nicht (Modulkopf, Abschnitt „Ein Ordner auf einem Server“). Dieselbe Regel in `crates/krk-core/src/operation/fern.rs`, `Stelle::liste` (Feld `Liste::unansteuerbar`). Die Meldung `Zahlwort::FernNamenUebersprungen` nennt in allen drei Sprachtabellen beide Gruende. Probe `ein_name_mit_schraegstrich_leer_oder_mit_nul_wird_uebersprungen_und_gezaehlt` in `crates/krk-core/tests/fern.rs`.

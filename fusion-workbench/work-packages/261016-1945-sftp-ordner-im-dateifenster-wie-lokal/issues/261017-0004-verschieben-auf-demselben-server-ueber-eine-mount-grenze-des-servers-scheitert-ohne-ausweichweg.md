Verschieben auf demselben Server über eine Mount-Grenze des Servers scheitert ohne Ausweichweg
---
Auf demselben Server verschiebt KRK immer mit `RENAME`. Liegen Quelle und Ziel auf dem Server in verschiedenen Dateisystemen, antwortet der Server mit `FAILURE` (dahinter steht `EXDEV`), und der Eintrag wird übersprungen. Lokal weicht KRK in derselben Lage auf Kopieren und Entfernen aus (`ueber_datentraeger`), am fernen Ort nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Funktionslücke, kein Datenverlust)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/operation/fern.rs`, `Stelle::umbenennen`: liefert für dieselbe Serverkennung immer den `RENAME`-Weg.
- `fern.rs`, `eintrag_verschieben`: scheitert das `RENAME` und fehlt das Ziel, endet der Weg in `ueberspringen`, ohne `ueber_die_grenze` zu versuchen.

**Behebung (Richtung)**

Scheitert `RENAME` mit `FAILURE` und steht am Ziel nichts, auf `ueber_die_grenze` ausweichen. Das gilt mit derselben Regel wie lokal: die Quelle wird nur entfernt, wenn nichts übersprungen wurde.

**Abnahme**

Eine Probe mit einem Server, der `RENAME` mit `FAILURE` beantwortet (etwa Quelle und Ziel unter zwei verschiedenen Volumes, oder ein gestellter Server), zeigt: der Eintrag kommt am Ziel an, und die Quelle ist danach fort.

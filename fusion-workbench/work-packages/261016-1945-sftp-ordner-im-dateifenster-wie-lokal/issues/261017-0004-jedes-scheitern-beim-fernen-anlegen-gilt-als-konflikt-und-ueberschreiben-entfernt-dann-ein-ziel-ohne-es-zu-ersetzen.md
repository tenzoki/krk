Jedes Scheitern beim fernen Anlegen gilt als Konflikt, und „Überschreiben“ entfernt dann ein Ziel, ohne es zu ersetzen
---
Kopieren und Verschieben werten jedes Scheitern des exklusiven Anlegens oder des `RENAME` als Konflikt, sobald `LSTAT` am Ziel etwas findet. Scheitert das Anlegen aber aus einem anderen Grund (keine Rechte, Kontingent erschöpft, `FAILURE` des Servers), dann zeigt das Konfliktblatt einen falschen Grund. Nach „Überschreiben“ ist das Ziel endgültig entfernt, der zweite Versuch scheitert erneut, und am Ende ist das Ziel weg und nichts an seiner Stelle.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Datenverlust am Ziel nach einer Bestätigung, die auf einer falschen Frage beruht)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege**

- `crates/krk-core/src/operation/fern.rs`, `Fehlschlag::ist_kein_konflikt`: `Fehlschlag::Lokal(_) | Fehlschlag::Fern(_) => false`. Jeder Fehlercode ist damit ein möglicher Konflikt.
- Im Kopierweg nach `anlegen`: `if fehler.ist_kein_konflikt() { … }`, dann `ziel.angaben(pfand)` und danach `konflikt_klaeren`. Der Verschiebeweg (`eintrag_verschieben`) hat dieselbe Form.
- In derselben Datei schließt `namensgrund` die Codes `NO_SUCH_FILE` und `PERMISSION_DENIED` ausdrücklich aus. Die zwei Stellen beantworten dieselbe Frage also verschieden.
- Lokal steht `io::ErrorKind::AlreadyExists` zur Verfügung und bleibt hier ungenutzt.

**Behebung (Richtung)**

Eine einzige Regel für „Anlegen scheiterte an einem vergebenen Namen“, die `ist_kein_konflikt` und `namensgrund` beide fragen:
- lokal allein `AlreadyExists`;
- fern jeder Code außer `NO_SUCH_FILE` und `PERMISSION_DENIED`, und nur bei vorhandenem Ziel.

SFTP v3 kennt keinen eigenen Code für „vorhanden“, also bleibt `FAILURE` mehrdeutig. Die Regel schließt aus, was sicher etwas anderes heißt.

**Abnahme**

Eine Probe gegen `/usr/libexec/sftp-server` kopiert in einen Zielordner ohne Schreibrecht, in dem der Name schon steht. Sie zeigt: Die Konfliktfrage wird nicht gestellt, der Eintrag wird mit dem Rechtegrund übersprungen, und das vorhandene Ziel steht unverändert da.

---

Resolved: `crates/krk-core/src/operation/fern.rs`. Eine Regel `vorhandenes_ziel` ueber `Fehlschlag::kann_vergeben_heissen` (lokal allein `AlreadyExists`, fern jeder Statuscode ausser `NO_SUCH_FILE` und `PERMISSION_DENIED`, Trennung und Protokollfehler nie) plus `LSTAT` am Ziel; sie fragen `eintrag_kopieren`, `eintrag_verschieben`, `namensgrund` und das Anlegen der Zwischennamen, `ist_kein_konflikt` ist gefallen. „Ueberschreiben“ (`Klaerung::Ersetzen`) entfernt nichts mehr in `konflikt_klaeren`: `kopie_ersetzt_ziel` legt den Ersatz vollstaendig unter `.krk-ersatz-<pid>-<n>` an, `umbenennen_ersetzt_ziel` benennt die Quelle dorthin um, und erst danach entfernt `tauschen` das Ziel und benennt den Ersatz um; fehlt am Ersatz etwas, faellt er weg und das Ziel bleibt. Drei neue Schluessel in `crates/krk-core/src/sprache/schluessel.rs` und `tabelle/{de,fr,en}.rs` (`VorgangErsatzUnvollstaendig`, `VorgangErsatzUnterZwischennamen`, `VorgangQuelleUnterZwischennamen`). Proben: `ueberschreiben_entfernt_das_ziel_erst_wenn_der_ersatz_vollstaendig_dasteht` in `crates/krk-core/tests/fern_operation.rs` gegen `/usr/libexec/sftp-server` (unlesbare Quelle, Datei und Ordner ueber eine Datei; gegen den alten Stand rot gefahren) und die Tafel `allein_ein_moeglicher_vergebener_name_ist_ein_konflikt` im Pruefmodul von `operation/fern.rs`. Die Abnahme des Befunds (Zielordner ohne Schreibrecht) laesst sich am Server des Systems nicht stellen: `open(2)`, `mkdir(2)` und `link(2)` setzen dort `EEXIST` vor `EACCES`, der Server meldet also `FAILURE`; deshalb steht die Regel als Tafel.

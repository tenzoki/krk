Eine Sonderdatei in einem lokalen Quellbaum lässt das ganze Hineinkopieren scheitern
---
Liegt in einem lokalen Ordner, der in ein Archiv kopiert oder verschoben wird, eine benannte Röhre, ein Sockel oder ein Gerät, scheitert der ganze Umbau. Nichts kommt ins Archiv, auch keine andere Quelle desselben Vorgangs. Eine ferne Quelle lässt denselben Eintrag dagegen mit Grund aus, und der Rest kommt an. Lokal und fern antworten also verschieden auf dieselbe Lage.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (kein Datenverlust, Archiv unverändert; ein ganzer Vorgang scheitert an einem Eintrag)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-core/src/archiv/umbau.rs`, `Neueintrag::lokal_lesen`: was weder Verknüpfung noch Ordner ist, wird `Neuart::Datei(Box::new(LokaleDatei::neu(…)))`.
- `LokaleDatei::oeffnen`: `if !angaben.is_file() { return Err(…VorgangKeineGewoehnlicheDatei…) }`. Dieser Fehler wird über `quelle_oeffnen` → `quellfehler` zu `Umbaufehler::Quelle` und beendet den Umbau.
- `crates/krk-core/src/operation/archiv.rs`, `stelleneintrag_bauen` (fern): `Typ::Datei if !angaben.gewoehnlich => { steuerung.ueberspringen(…); return Ok(None); }`. Der ferne Weg lässt den Eintrag aus und meldet ihn.

**Behebung (Richtung)**

`Neueintrag::lokal_lesen` fragt den Typ schon beim Zusammenstellen, wie der ferne Weg. Eine Sonderdatei fällt mit Grund in den Bericht, und `ganz` des Postens wird falsch. Ein Verschieben entfernt die Quelle dann nicht. Dafür braucht `Neueintrag::lokal` eine `Steuerung` oder gibt die Auslassungen zurück.

**Abnahme**

Eine Probe kopiert einen lokalen Ordner mit einer Datei und einer benannten Röhre in ein Zip. Danach steht die Datei im Archiv, die Röhre mit `VorgangKeineGewoehnlicheDatei` im Bericht. Beim Verschieben bleibt der Quellordner stehen.

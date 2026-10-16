Überschreiben zerstört die Quelle, wenn Ziel und Quelle auf dem Server über eine Verknüpfung derselbe Eintrag sind
---
Ob zwei ferne Stellen denselben Eintrag benennen, entscheidet der Fernzweig allein am Pfadtext; lokal entscheiden Gerät und Inode. Benennen zwei verschiedene Pfade auf demselben Server über eine Verknüpfung dieselbe Datei, kommt die Konfliktfrage. „Überschreiben“ entfernt dann die eine gemeinsame Datei, also die Quelle. Beim Kopieren entsteht danach eine leere Datei, und der Vorgang meldet Erfolg. Beim Verschieben scheitert das `RENAME`, und die Quelle ist fort.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Datenverlust nach einer Bestätigung durch den Nutzer, an einer gewöhnlichen Serverlage, etwa `~/htdocs` → `/var/www`)
**Domain:** code
**Cross-references:** `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`

**Belege** (Code gelesen, nicht gegen einen Server mit Verknüpfung ausgeführt)

- `crates/krk-core/src/operation/fern.rs`, `Stelle::derselbe_eintrag`, Fernzweig: `kennung == andere_kennung && pfad == anderer_pfad`.
- Lokaler Zweig derselben Funktion: `benennen_denselben_eintrag(eine, andere)`, gerechnet über Gerät und Inode.
- `fern.rs`, `konflikt_klaeren`: der Zweig `Konfliktantwort::Ueberschreiben` ruft `ziel.baum_entfernen(pfand)`. Danach legt der Kopierweg neu an und füllt die Datei aus der nun leeren Quelle.
- Dieselbe Ursache hat `liegt_in` (rein lexikalisch): einen Ordner über eine Verknüpfung in sich selbst zu kopieren, rekursiert, bis der Server abbricht.

**Behebung (Richtung)**

Bevor „Überschreiben“ bei zwei Stellen auf demselben Server ausgeführt wird, auf beide Elternordner `REALPATH` anwenden und dann mit dem Namen vergleichen. Sind beide gleich, gilt der Fall als „derselbe Eintrag“, wie es lokal geschieht. Für `liegt_in` gilt dieselbe Prüfung, angewandt auf Quelle und Zielordner. Die Grenze, die bleibt, nennt der Modulkopf ausdrücklich als möglichen Datenverlust, wenn sie nicht ganz geschlossen wird.

**Abnahme**

Eine Probe in `tests/fern_operation.rs` gegen `/usr/libexec/sftp-server` legt im `Wegwerfordner` den Ordner `echt/` mit `x` an, dazu `alias -> echt`. Kopiert und verschiebt sie dann `echt/x` nach `alias/` mit der Antwort „Überschreiben“, steht `echt/x` danach mit seinem ursprünglichen Inhalt da.

---

Resolved: `crates/krk-core/src/operation/fern.rs`, `Stelle::derselbe_eintrag` antwortet jetzt mit `Gleichheit` (Ja, Nein, Unklar) und vergleicht fern bei verschiedenen Pfaden den Namen unter den zwei per `REALPATH` aufgeloesten Elternordnern, auf dem Arbeitsfaden. `einen_uebertragen` ueberspringt bei Ja; vor jedem Entfernen durch „Ueberschreiben“ fragt `ersetzen_erlaubt` erneut und ueberschreibt bei Ja und bei Unklar nicht. `Stelle::liegt_in` fragt nach dem Pfad ebenso `REALPATH` beider Ordner. Modulkopf „Was dieses Modul nicht haelt“ nachgezogen (harte Verweise und zwei Kennungen fuer einen Server bleiben als Grenze benannt). Probe `ueberschreiben_ueber_eine_verknuepfung_laesst_die_quelle_stehen` in `crates/krk-core/tests/fern_operation.rs` gegen `/usr/libexec/sftp-server` (`echt/x`, `alias -> echt`, Kopieren und Verschieben mit „Ueberschreiben“, dazu ein Zielordner ueber eine Verknuepfung in der Quelle); gegen den alten Stand rot gefahren.

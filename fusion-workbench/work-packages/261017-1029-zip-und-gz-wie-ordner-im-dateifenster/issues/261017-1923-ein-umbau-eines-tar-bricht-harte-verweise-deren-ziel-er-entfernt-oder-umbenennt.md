Ein Umbau eines tar bricht harte Verweise, deren Ziel er entfernt oder umbenennt
---
Ein harter Verweis im tar (Typ `1`, `EntryType::Link`) hat keine eigenen Daten. Er nennt den Namen eines früheren Eintrags, dessen Daten er beim Auspacken teilt. Wird dieser frühere Eintrag im Archiv gelöscht, umbenannt oder samt Ordner verschoben, schreibt der Umbau den Verweis unverändert weiter. Danach zeigt er ins Leere, und beim Auspacken fehlt seine Datei. Der Nutzer hat nur den einen Eintrag angefasst und verliert den zweiten mit. Der Bericht meldet `Fertig` ohne Hinweis.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** High (stiller Verlust eines Eintrags, den der Nutzer nicht angefasst hat, entgegen der Zusage der Directive)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`

**Belege**

- Nachgestellt an einer Kopie des Stands `8c8579af`. Das tar trägt `a.txt` (18 Bytes) und `b.txt` als harten Verweis auf `a.txt`:
  - `Fernauftrag::loeschen([a.tar:/a.txt])` meldet `Fertig`, `uebersprungen: []`. Danach endet `/usr/bin/tar -xvf` mit Status 1 („Error exit delayed from previous errors“), und `b.txt` entsteht nicht.
  - `Fernauftrag::umbenennen(a.tar:/a.txt, "c.txt")` gibt denselben Ausgang.
- `crates/krk-core/src/archiv/verzeichnis.rs`, `tar_eintraege_sammeln`: `EntryType::Link => (Typ::Datei, Lesbarkeit::HarterVerweis)`. Das Verzeichnis kennt den Verweis, aber nicht sein Ziel.
- `crates/krk-core/src/archiv/umbau.rs`, `Tarstrom::schreiben`: ein behaltener Eintrag geht über `roh_schreiben` mit unverändertem Kopf hinüber, also auch mit unverändertem `linkname`. `umbenannt_schreiben` setzt allein den eigenen Namen neu. Keine Stelle in `Aufstellung` fragt, ob ein entfernter oder umgehängter Pfad Ziel eines harten Verweises ist.
- Der Modulkopf von `umbau.rs` nennt unter „Was der Umbau nicht schreibt“ GNU-Lücken und abweichende PAX-Größen, harte Verweise nicht.

**Behebung (Richtung)**

Das Inhaltsverzeichnis merkt sich je hartem Verweis sein Ziel als `Innenpfad`. Die Aufstellung behandelt dann jeden Schritt, der ein solches Ziel entfernt oder umhängt, auf eine von zwei Arten:

- Sie zieht den Verweis mit, durch Umschreiben des `linkname` bei einem Umhängen, oder durch Hochstufen des ersten Verweises zu einer Datei mit den Daten bei einem Entfernen.
- Sie weist den Schritt vor dem ersten Byte mit eigenem Grund ab, wie `Unumschreibbar`.

Welche der beiden, entscheidet der Bearbeiter. Die zweite ist die kleinere.

**Abnahme**

Eine Probe je tar und tar.gz mit `a.txt` und einem harten Verweis `b.txt` darauf. Nach Löschen und nach Umbenennen von `a.txt` gilt eines von beiden:

- `/usr/bin/tar -xf` endet mit Status 0, und `b.txt` trägt den alten Inhalt.
- Das Archiv ist Byte für Byte unverändert, und der Bericht nennt den Grund.

---
Resolved: gewaehlt ist die kleinere der zwei Richtungen, ein tar mit hartem Verweis gilt als allein lesbar; das ist ohne Zuordnung von Verweis zu Ziel sicher, auch wenn das Ziel ein Eintrag ist, den das Fenster nicht zeigt. `crates/krk-core/src/archiv/umbau.rs`: `Unumschreibbar::TarHarteVerweise` (Text `ArchivTarHarteVerweise` in allen drei Tabellen), geprueft in `umbauen_mit` vor der Aufstellung an `Lesbarkeit::HarterVerweis` und in `Tarstrom::schreiben` an jedem Kopf vom Typ `EntryType::Link`; Modulkopf „Was der Umbau nicht schreibt“. Probe `ein_tar_mit_hartem_verweis_wird_nicht_umgeschrieben` in `crates/krk-core/tests/archiv_umbau.rs` fuer tar und tgz, Loeschen und Umbenennen: Archiv Byte fuer Byte unveraendert, Bericht nennt den Grund, `/usr/bin/tar -xf` packt `b.txt` mit altem Inhalt aus.

Verschieben einer lokalen Quelle, die das Zielarchiv ist oder enthält, löscht das Archiv samt Inhalt
---
Wer die Archivdatei selbst oder einen lokalen Ordner, in dem sie liegt, in dieses Archiv verschiebt, verliert das Archiv. Der Umbau schreibt das neue Archiv vollständig und benennt es an den Platz des alten. Danach entfernt `hinein` die „vollständig angekommene“ lokale Quelle mit `baum_entfernen`, und diese Quelle ist jetzt das neue Archiv oder der Ordner, der es trägt. Danach ist an keinem Ort mehr etwas vorhanden. Der Bericht meldet `Fertig` ohne einen übersprungenen Eintrag.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Critical (stiller Totalverlust über einen gewöhnlichen Bedienweg: F6 oder Abwurf mit Verschieben)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`, `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`

**Belege**

- Nachgestellt an einer Kopie des Stands `8c8579af` im Scratchpad. Zwei Proben fahren `Fernauftrag::verschieben` über `operation::starten`, also denselben Weg wie im Betrieb:
  - `Ort::Lokal(a.zip)` nach `Ort::Archiv(a.zip:/)`: Bericht `Bericht { abschluss: Fertig, eintraege: 1, bytes: 142, uebersprungen: [] }`, danach `a.zip` fort.
  - Ordner `projekt/` mit `projekt/a.zip` nach `a.zip:/`: Bericht `Fertig`, `uebersprungen: []`, danach `projekt` und `a.zip` fort.
- `crates/krk-core/src/operation/archiv.rs`, `hinein`: der Arm `Ort::Lokal(pfad) => match Neueintrag::lokal(pfand, pfad)` nimmt jede lokale Quelle an und prüft nicht, ob sie die Zieldatei ist oder ein Vorfahr von ihr. Nach dem Umbau entfernt `Herkunft::Stelle(stelle) => { … stelle.baum_entfernen(pfand, &nie_abgebrochen)` sie.
- `crates/krk-core/src/archiv/ersetzen.rs`, `ersetzen`: `zwischen.umbenennen(archiv)` setzt das neue Archiv an genau den Pfad, den danach `baum_entfernen` trifft.
- Die Oberfläche hält den Fall nicht auf:
  - `crates/krk-ui/src/appkit/anwendung.rs`, `uebertragen`: `(quelle, ziel @ Dateiort::Archiv(_)) => self.fernauftrag_stellen(…)`.
  - `fernauftrag_stellen` prüft allein `if quelle == ziel`, also gleiche angezeigte Orte.
  - `crates/krk-ui/src/appkit/tabelle.rs`, Abwurfregel: `Ort::Archiv(archiv) if archiv.format().beschreibbar() => (Schreibrecht::Unbekannt, false)`. Damit kommt auch ein Abwurf aus dem Finder durch.
- Der Modulkopf von `operation/archiv.rs` („Was dieses Modul nicht hält“) nennt allein das Kopieren („steht das alte Archiv als Datei im neuen“) und nicht das Verschieben.
- Die Probe `archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei` liest allein `crates/krk-core/src/archiv/`. Das Löschen geschieht in `operation/archiv.rs` über `Stelle::baum_entfernen` und liegt außerhalb ihrer Sicht. Ihr Doc-Kommentar nennt diese Grenze allgemein („einen Schreiber über einen Helfer außerhalb von `archiv/`“).
- `inference:`, nicht nachgestellt: dieselbe Lage entsteht beim Verschieben zwischen zwei Archivorten, die über zwei Schreibweisen dieselbe Datei nennen. `hinein` vergleicht mit `quelle.datei() == ziel.datei()` den Pfadtext, und das Register schlüsselt nach dem Pfad, wie er im Tab steht (E2). Ein Ordner `a`, der nach `a/b` verschoben wird, käme so erst als Kopie nach `a/b/a`, und der zweite Umbau entfernte danach `a` samt dieser Kopie.

**Behebung (Richtung)**

Bevor der Plan entsteht, weist `hinein` beim Verschieben jede lokale Quelle ab, die die Zielarchivdatei ist oder sie enthält. Gemeint ist die aufgelöste Datei (`canonicalize` auf dem Faden), nicht der Pfadtext. Der Grund ist `VorgangZielLiegtInDerQuelle` oder ein eigener Text. Beim Kopieren bleibt der Fall wie dokumentiert. Archivquellen werden an Gerät und Inode der Datei verglichen und nicht am Pfadtext. Die Probe zum einzigen Schreiber bekommt in ihrem Doc-Kommentar einen Satz zu diesem Weg.

**Abnahme**

- Eine Probe je beschreibbarem Format verschiebt `Ort::Lokal(archiv)` in `archiv:/` und einen Ordner, der das Archiv enthält, in `archiv:/`. Danach stehen die Archivdatei und der Ordner unverändert, Byte für Byte, und der Bericht nennt die Quelle mit Grund.
- Eine dritte Probe verschiebt zwischen zwei Schreibweisen derselben Archivdatei, eine davon über eine Verknüpfung, einen Ordner in seinen eigenen Unterordner. Danach fehlt kein Eintrag.

---
Resolved: `crates/krk-core/src/operation/archiv.rs`, `hinein` mit den neuen Helfern `zielarchiv_in_quelle` und `dieselbe_datei` (Modulkopf, Absatz „Eine lokale Quelle, die das Zielarchiv ist oder es enthaelt“): eine lokale Quelle, die die aufgeloeste Zielarchivdatei ist oder einer ihrer Vorfahren (Geraet und Inode, eine Verknuepfung als Quelle wird als Verknuepfung behandelt, jeder Fehler einer Frage weist ab), wird vor dem Umbau beim Verschieben wie beim Kopieren mit `Text::VorgangZielLiegtInDerQuelle` abgewiesen; das gilt fuer F6 wie fuer den Abwurf, weil beide ueber `hinein` laufen. Ob eine Archivquelle im Zielarchiv liegt, entscheiden Geraet und Inode statt des Pfadtexts, also wird zwischen zwei Schreibweisen derselben Datei umgehaengt. Proben `eine_lokale_quelle_die_das_zielarchiv_ist_oder_enthaelt_wird_abgewiesen` und `zwei_schreibweisen_desselben_archivs_sind_ein_archiv` in `crates/krk-core/tests/archiv_operation.rs` (ohne Behebung rot); der Doc-Kommentar von `archiv_ersetzen_ist_der_einzige_schreiber_an_einer_archivdatei` (`tests/archiv_umbau.rs`) nennt den Entfernweg ausserhalb ihrer Sicht.

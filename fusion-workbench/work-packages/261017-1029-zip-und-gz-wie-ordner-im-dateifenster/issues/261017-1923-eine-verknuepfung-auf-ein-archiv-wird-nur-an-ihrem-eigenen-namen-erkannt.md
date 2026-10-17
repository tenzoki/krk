Eine Verknüpfung auf ein Archiv wird nur an ihrem eigenen Namen erkannt
---
E8 des Plans verlangt, dass eine Verknüpfung, die auf eine Datei zeigt, betreten wird, wenn **ihr Name oder der Name ihres Ziels** eine Archivendung trägt. Gebaut ist allein der eigene Name. `aktuell -> paket-1.2.zip` wird deshalb nicht betreten, und der Doppelklick reicht es an das System. Die Rumpfprobe zum Einstieg nennt außerdem nicht, dass der Verknüpfungszweig vorher `verweisziel::bestimmen` ruft, also ein `stat` auf dem Hauptfaden. Das war schon vor dem Paket so.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Abweichung vom Plan, kein Schaden)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`, `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`

**Belege** (Code gelesen, nicht ausgeführt)

- `crates/krk-ui/src/appkit/tabelle.rs`, `in_zeile_einsteigen`: `Typ::Verknuepfung => match verweisziel::bestimmen(&ziel) { … Verweisziel::KeinOrdner => return self.archiv_betreten(&ziel),`.
- `archiv_betreten`: `let Some(format) = datei.file_name().and_then(Archivformat::aus_namen) else { return Einstieg::KeinOrdner; };`. Gefragt wird der Name des Verknüpfungspfads, nicht der seines Ziels.
- Probe `der_archiveinstieg_stellt_auf_dem_hauptfaden_keinen_systemaufruf` (`tabelle.rs`): ihr Doc-Kommentar nennt die tieferen Rufer, aber nicht den `stat` des Verknüpfungszweigs vor `archiv_betreten`.

**Behebung (Richtung)**

Eine von zwei Richtungen:

- `verweisziel::bestimmen` liefert für `KeinOrdner` den Zielnamen mit, und `archiv_betreten` fragt beide Namen. Das kostet keinen weiteren Systemaufruf, weil `bestimmen` das Ziel ohnehin auflöst.
- Die Abweichung steht im Plan-Nachtrag und in `HowTo.md`.

Der Doc-Kommentar der Probe nennt in beiden Fällen den `stat` des Verknüpfungszweigs.

**Abnahme**

Eine Probe am Wert: eine Verknüpfung `aktuell`, deren Ziel `paket.zip` heißt, ergibt `Einstieg::Eingestiegen`.

---
Resolved: Eine Verknuepfung, deren eigener Name keine Endung traegt, geht durch `DateifensterQuelle::verknuepfung_betreten` (`crates/krk-ui/src/appkit/tabelle.rs`) an `Tabliste::verknuepfung_pruefen` (`crates/krk-ui/src/tabs.rs`): ein Faden loest das Ziel auf (`verweisziel_format`), `Tabliste::einziehen` betritt das Archiv oder reicht die Verknuepfung ueber `Einzug::an_das_system` an das System, wenn der Doppelklick fragte (`Einstieg::Ausstehend`). Auf dem Hauptfaden kommt kein Systemaufruf hinzu; der Doc-Kommentar der Rumpfprobe `der_archiveinstieg_stellt_auf_dem_hauptfaden_keinen_systemaufruf` nennt jetzt den `stat(2)` von `verweisziel::bestimmen`, und die Probe liest die zwei neuen Rumpfe mit. Abweichung von der Abnahme: der Einstieg antwortet `Ausstehend` und nicht `Eingestiegen`, denn eingestiegen wird erst im Einzug. Proben: `eine_verknuepfung_wird_am_namen_ihres_ziels_als_archiv_erkannt` (Pruefmodul von `tabs.rs`), `eine_verknuepfung_auf_ein_archiv_wird_ueber_den_namen_ihres_ziels_betreten` (Pruefmodul von `tabelle.rs`); `HowTo.md` `## Archive im Dateifenster`.

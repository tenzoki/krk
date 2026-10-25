# Nach einem Lesefehler lädt die Vorschau denselben Archiveintrag bei gleicher Kennung nie wieder

---
Seit `a1fec2b7` lädt `Vorschaumodell::archiv_anzeigen` einen Archiveintrag nicht neu, solange die Kennung des Archivs gleich ist. Die Quelle wird auch dann gemerkt, wenn das Laden mit einem Fehler endete und nur die Ersatzangaben stehen. Ein vorübergehender Fehler bleibt danach stehen, bis der Nutzer einen anderen Eintrag wählt und zurückkehrt.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Low
**Cross-references:** 261024-2113-die-vorschau-einer-wal-tabelle-loest-ueber-die-shm-ihre-eigene-auffrischung-aus.md

## Befund

- `crates/krk-ui/src/vorschaumodell.rs`, `archiveintrag_laden`: jeder Fehler, `Archivleser::oeffnen` eingeschlossen, endet in `(Inhalt::Hinweis(ersatz), None)`. `Err(_) => return (Inhalt::Hinweis(ersatz), None)` unterscheidet `Belegt`, `Lesen(io::Error)` und `VonAussenVeraendert` nicht von einem Fehler, der bleibt.
- Dieselbe Datei, Lieferung im Ladetakt: `tab.archivquelle = vorgang.archivquelle.take();` steht ohne Bedingung an den Inhalt.
- `archiv_anzeigen`: `if Archivquelle::von(&eintrag.ort).is_some_and(|quelle| tab.zeigt_archiveintrag(&quelle)) { return; }`.
- Erreichbar ohne Änderung der Kennung zum Beispiel so: Eine Datenbank im Rollback-Betrieb, deren Schreiber die Sperre über `WARTEZEIT` (200 ms) hält und dann zurückrollt. `-journal` entsteht und vergeht, FSEvents meldet den Ordner, die Auffrischung meldet die Auswahl erneut, und die Hauptdatei ist unverändert. Ebenso ein vorübergehender Ein-/Ausgabefehler bei einem Zip auf einer Freigabe. Vor `a1fec2b7` versuchte jede Auffrischung es erneut. Das Bündel ist nicht geprüft; die Kette ist am Quelltext gelesen.
- Die Proben `eine_auffrischung_laedt_einen_unveraenderten_archiveintrag_nicht_neu` und `nach_einer_echten_aenderung_wird_derselbe_archiveintrag_neu_geladen` fahren allein erfolgreiche Ladevorgänge.

Ein erneuter Versuch bei **jedem** Fehler brächte die Schleife aus 2113 zurück, solange ein bleibender Fehler über `mode=ro` die `-shm` berührt. Die Regel muss deshalb nach der Art des Fehlers unterscheiden.

## Erwartet

Der Lieferweg unterscheidet einen vorübergehenden Lesefehler (`Belegt`, `Lesen`, `VonAussenVeraendert` ohne neue Kennung) von einem gelungenen oder bleibend gescheiterten Laden. Nach einem vorübergehenden Fehler bleibt `Vorschautab::archivquelle` leer, und die nächste Meldung der Auswahl lädt neu. Alternativ hält der Doc-Kommentar an `Archivquelle` ausdrücklich fest, dass ein Fehlerzustand bis zum Auswahlwechsel stehen bleibt, und der Nutzer nimmt das hin.

## Abnahme

Eine Probe in `vorschaumodell.rs` lädt einen Archiveintrag, dessen Lesen vorübergehend scheitert, etwa eine Datenbank, deren Sperre ein Kindprozess über `WARTEZEIT` hält. Danach gibt der Kindprozess die Sperre ohne Änderung frei, und dieselbe Auswahl wird erneut gemeldet. Die Probe prüft, dass ein zweiter Ladevorgang startet und die Tabelle erscheint.

**Resolved:** 261025, code-implementer. `archiveintrag_laden` (`crates/krk-ui/src/vorschaumodell.rs`) liefert ein `Eintragsergebnis` mit `merkbar`; falsch allein, wenn `Archivleser::oeffnen` oder das Lesen mit einem vorübergehenden Fehler endet (`voruebergehend`: `Abgebrochen`, `Lesen`, `VonAussenVeraendert`, `Verzeichnis(Belegt | Lesen | Abgebrochen)`, vollständig ohne Auffangzweig). `einziehen` setzt `tab.archivquelle` nur bei `merkbar`. Bleibende Angaben (PDF, über der Grenze, kein UTF-8, verschlüsselt) werden weiter gemerkt. Proben: `nach_einem_voruebergehenden_lesefehler_laedt_derselbe_archiveintrag_erneut` (rot ohne die Regel geprüft) und die Gegenprobe `nach_bleibenden_angaben_laedt_derselbe_archiveintrag_nicht_neu`.

# Die Vorschau einer Tabelle in einer WAL-Datenbank löst über `-shm` vermutlich ihre eigene Auffrischung aus

---
Jedes Lesen über `mode=ro` ändert neben einer WAL-Datenbank die `-shm` (mindestens ihre Änderungszeit). Der Archivtab beobachtet den Ordner der Datei, eine Auffrischung meldet die Auswahl erneut, und die Vorschau liest die Tabelle neu. Das ergibt eine Schleife, solange eine Tabelle oder Sicht einer solchen Datenbank in der Vorschau steht. Am laufenden Bündel nicht geprüft.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Medium (inference, Prüfung im Bündel steht aus)
**Cross-references:** 261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md, 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md

## Befund (Kette, jedes Glied am Quelltext gelesen)

1. `messungen/261023-1556-sqlite-nur-lesend-shm.txt`, Versuch c: zwei Leseläufe hintereinander ändern die Änderungszeit von `-shm` auch bei gleichem Inhalt (`stat`-Spalte 3: `…792` zu `…793`).
2. `crates/krk-ui/src/auffrischung.rs`, `archivordner` und `ort_neu_lesen`: ein Archivtab beobachtet den Ordner seiner Archivdatei und wird bei jeder Meldung dieses Ordners neu gelesen. `appkit/fsevents.rs` setzt `kFSEventStreamCreateFlagFileEvents` nicht und filtert eigene Änderungen nicht.
3. `crates/krk-ui/src/appkit/tabelle.rs`, `nach_lesebeginn`: „Eine Auffrischung kommt hier ebenso durch wie eine Navigation und meldet dann den weiterhin ausgewaehlten Eintrag“, über `auswahl_merken`.
4. `crates/krk-ui/src/vorschaumodell.rs`, `archiv_anzeigen`: startet immer einen neuen `Ladevorgang`. Anders als `datei_anzeigen` (`zeigt_folge_von`) gibt es dort keine Ausnahme für den Eintrag, der schon steht.
5. `archiveintrag_laden` öffnet die Datenbank über `archiv::sqlite::oeffnen` neu, und damit schließt sich der Kreis bei 1.

`Archivzugang::verzeichnis` liest dabei nicht neu, weil `-shm` nicht in der `Dateikennung` steht. Der Ordner wird gelesen, und die Vorschau liest erneut, im Takt der FSEvents-Latenz.

Betroffen sind gerade die häufigen Datenbanken laufender Programme (WAL mit stehender `-shm`).

## Erwartet

Zuerst die Schleife im Bündel nachweisen oder ausschließen: eine Tabelle einer WAL-Datenbank mit `-shm` wählen, `fs_usage -f filesys krk` oder Protokoll der Ladevorgänge beobachten. Bestätigt sie sich, braucht es eine Regel, nach der eine Änderung allein an `-shm` keine Auffrischung des Archivtabs auslöst, oder nach der die Vorschau einen unveränderten Eintrag (gleiche `Dateikennung`) nicht neu lädt.

## Abnahme

Eine Probe in `krk-ui`, die nach dem Laden einer Tabelle aus einer WAL-Datenbank mit `-shm` eine Auffrischung des Tabs fährt und prüft, dass kein zweiter Ladevorgang für denselben Eintrag startet. Alternativ eine Probe, dass eine Meldung, die allein `-shm` betrifft, den Archivtab nicht neu liest.

**Resolved:** 261024, code-implementer. Die Schleife ist im laufenden Bündel **nicht** nachgewiesen und nicht ausgeschlossen (KRK im Vordergrund ist Nutzerarbeit). Gebaut ist die Absicherung, die unabhängig davon gilt: `Vorschaumodell::archiv_anzeigen` (`crates/krk-ui/src/vorschaumodell.rs`) lädt denselben Archiveintrag nicht neu, solange die Kennung des Archivs (`Archivzugang::gespeichertes`, ohne Systemaufruf) dieselbe ist wie beim Laden (`Archivquelle`, analog zu `zeigt_folge_von`). `-shm` steht nicht in der Kennung; eine Änderung an Hauptdatei oder `-wal` gibt nach der Auffrischung eine neue Kennung und lädt neu. Proben: `eine_auffrischung_laedt_einen_unveraenderten_archiveintrag_nicht_neu` (Abnahme) und `nach_einer_echten_aenderung_wird_derselbe_archiveintrag_neu_geladen` (Gegenprobe). Die Auffrischung des Archivtabs selbst (Ordner neu lesen bei einer Meldung allein zu `-shm`) bleibt; sie liest dank der Kennung nichts neu.

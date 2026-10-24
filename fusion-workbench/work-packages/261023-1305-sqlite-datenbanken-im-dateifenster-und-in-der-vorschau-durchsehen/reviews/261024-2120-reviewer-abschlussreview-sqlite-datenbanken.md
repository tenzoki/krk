# Abschlussreview: SQLite-Datenbanken im Dateifenster und in der Vorschau

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `c987c04f..c824a972`
**Not-opened:** `crates/krk-core/tests/archiv.rs`, `crates/krk-core/tests/archiv_operation.rs`, `crates/krk-core/tests/archiv_sqlite.rs`, `crates/krk-core/tests/sprache.rs`, `CLAUDE.md`, `HowTo.md`, `README.md`, `fusion-workbench/orchestrator-events.jsonl`, `261023-1507-wie-stehen-null-und-blob-in-der-tsv-einer-sqlite-tabelle.md`, `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.record.json`
**Review domain:** code
**Verdict:** revise

Zu den nicht geöffneten Dateien: die drei Probendateien des Kerns habe ich ausgeführt (`cargo test -p krk-core --test sqlite_bindung --test archiv_sqlite --test archiv_operation`, 31 + 16 + 6 grün, eine ignoriert) und nach `shm`, `Duration`, `Instant` und `sleep` durchsucht, aber nicht gelesen. `crates/krk-core/tests/sqlite_bindung.rs` ist teilweise gelesen (die Abbruchprobe und die Liste der Proben). Die Dokumentationsdateien liegen außerhalb des Codeauftrags. Gelesen sind die `.claude/rules/coding-was-man-nicht-sieht.md`, die Messung `messungen/261023-1556-sqlite-nur-lesend-shm.txt`, der Entscheid `261023-1605-…` und der geschlossene Defekt `261023-2035-…`.

## Summary

Die Bindung an `libsqlite3` ist an jeder `unsafe`-Stelle sauber begründet und nach meiner Lesung korrekt. Die Zusage „nur lesen“ hält für Hauptdatei und `-wal`, aber nicht für die Begleitdatei `-shm`: neben einer WAL-Datenbank mit `-wal` und ohne `-shm` legt KRK eine neue `-shm` an, nachgemessen. Der Entscheid zur Lesemarke ruht auf der gegenteiligen Prämisse. Dazu kommen eine vermutete Rückkopplung zwischen `-shm`, FSEvents und Vorschau, eine Stelle, an der Zeilen als Bytes erscheinen, und ein unnötig voller BLOB-Weg.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 3 |
| Low | 2 |

## Findings by theme

### Nur lesen

**H1: `mode=ro` legt neben einer WAL-Datenbank mit `-wal` und ohne `-shm` eine `-shm` an** (High).
`crates/krk-core/src/archiv/sqlite.rs`, Modulkopf `# Nur lesen`, sagt zu: „es legt neben der Datenbank keine Datei an“. `oeffnen` nimmt `immutable=1` allein, wenn `-wal` fehlt. Nachgemessen mit `/usr/bin/sqlite3 -readonly 'file://…?mode=ro'` an einer Wegwerfdatenbank: vorher `x.db x.db-wal`, nachher `x.db x.db-shm x.db-wal`, Hauptdatei und `-wal` byte-gleich. Die Messung des Pakets deckt den Fall nicht (b, c: beide stehen; d: beide fehlen). Der Entscheid `261023-1605-…` sagt unter `## Question` „Eine neue Datei legt sie nie an“. Das braucht eine Nutzerentscheidung, bevor der Code folgt.
Issue: `261024-2112-mode-ro-legt-neben-einer-wal-datenbank-mit-wal-und-ohne-shm-eine-neue-shm-an.md`.

**M1: Rückkopplung `-shm` → FSEvents → Auffrischung → Vorschau** (Medium, inference).
Jedes Lesen ändert die Änderungszeit von `-shm` (Messung, Versuch c). `auffrischung.rs` liest einen Archivtab bei jeder Meldung seines Ordners neu, `tabelle.rs::nach_lesebeginn` meldet nach einer Auffrischung die Auswahl erneut, und `vorschaumodell.rs::archiv_anzeigen` startet immer einen neuen Ladevorgang, der die Datenbank wieder öffnet. Jedes Glied ist am Quelltext gelesen, die Schleife selbst ist im Bündel nicht beobachtet.
Issue: `261024-2113-die-vorschau-einer-wal-tabelle-loest-ueber-die-shm-ihre-eigene-auffrischung-aus.md`.

SQL-Einschleusung: kein Befund. `bezeichner` (`archiv/sqlite.rs`) setzt den Namen in doppelte Anführungszeichen und verdoppelt `"`; `sicht_lesen` bindet den Namen als Parameter. Ein Name mit Nullbyte kürzt den SQL-Text in `sqlite3_prepare_v2` und endet als Fehler (P6, unbekannte Zeilenzahl), nicht als anderer Befehl. Ein heißes `-journal` beantwortet eine nur lesende Verbindung mit `SQLITE_READONLY_ROLLBACK` (inference aus dem Quelltext von SQLite) und schreibt nicht.

### Byte gegen Zeilen

**M2: Die Ersatzangaben der Vorschau zeigen Zeilen als Bytes** (Medium).
`verzeichnis/leser.rs::archiveintrag_zeigen` legt `Archiveintrag::groesse` (Zeilen) in `Eintrag::groesse`. `appkit/tabelle.rs::archivangaben_des_eintrags` reicht es als `Fernangaben::groesse`, und `appkit/vorschau.rs::angabenzeilen` formatiert es als Bytes. Sichtbar bei jedem Lesefehler einer Tabelle, etwa `Belegt` oder `VonAussenVeraendert`. Die Erhebung aus Schritt 4 lief allein über drei Dateien des Kerns.
Issue: `261024-2114-die-ersatzangaben-der-vorschau-zeigen-die-zeilenzahl-einer-tabelle-als-bytes.md`.

Im Kern ist die Erhebung vollständig: `operation/fern.rs` (`Angaben::aus_archiveintrag`), `operation/archiv.rs` (`archiveintrag_bauen`, `im_einen_archiv`) und `archiv/umbau.rs` lesen Bytes über `datenlaenge` oder über `Geoeffnet::groesse`, das die Zulieferung nach dem Zähllauf setzt. Die Größenspalte und der Markierungsstand (`groessenzelle`, `masseinheit`, `datenbankzeilen`) stellen keinen Systemaufruf und fragen `gespeichertes`, nicht `verzeichnis`; die Probe `die_groessenzelle_stellt_auf_dem_hauptfaden_keinen_systemaufruf` hält das an den Rümpfen.

### Speicher

**M3: Ein BLOB wird ganz kopiert, eine Zeile ist unbegrenzt** (Medium).
`sys.rs::Abfrage::blobbytes` kopiert den BLOB, `archiv/sqlite.rs::feldtext` braucht allein `len()`. Die Grenze der Vorschau greift erst nach einer ganzen Zeile, und an der Verbindung steht kein `sqlite3_limit`.
Issue: `261024-2115-ein-blob-wird-ganz-kopiert-um-seine-laenge-zu-nennen-und-eine-zeile-ist-unbegrenzt.md`.

### Bindung (`verzeichnis/sys.rs`)

Geprüft und ohne Befund:
- Lebensdauer: `Abfrage<'d, 'a>` leiht `&'d Datenbank<'a>`, also ist jede Anweisung vor `sqlite3_close_v2` finalisiert. `Drop` beider Typen ruft genau einmal.
- Spaltenzeiger: `textbytes` und `blobbytes` fragen erst den Zeiger, dann `sqlite3_column_bytes`, und kopieren sofort. `bytes_lesen` leiht `&self`, und `naechste_zeile` verlangt `&mut self`, also kann kein Ausschnitt einen Schritt überleben.
- Fehlerpfade: ein gescheitertes `sqlite3_open_v2` mit Griff wird über `Drop` geschlossen, wie `sqlite3.h` verlangt. `griff == null` heißt `SQLITE_NOMEM`.
- Fäden: der rohe Zeiger nimmt `Send` und `Sync`; der Fortschrittsrückruf liest allein ein `AtomicBool`, dessen Lebensdauer `'a` die Verbindung bindet. `mem::forget` ließe einen hängenden Kontext zurück, aber ohne weiteren Aufruf in die Verbindung.
- Zählung: siebzehn Funktionen im neuen Block, einunddreißig in neun `unsafe extern "C"`-Blöcken, wie Modulkopf, `lib.rs` und `verzeichnis/mod.rs` sagen (Lauf über die Blöcke).
- Abbruch: der Rückruf greift alle `FORTSCHRITT_JE_BEFEHLE`, `tabelle_lesen` fragt die Marke zusätzlich zwischen zwei Zeilen. Während der Wartezeit auf eine Sperre ruft SQLite den Rückruf nicht, der Abbruch wartet dort höchstens `WARTEZEIT` (200 ms).

Eine Beobachtung ohne eigenen Defekt: `bytes_lesen` unterscheidet einen leeren Wert von Speichermangel über `sqlite3_extended_errcode` nach einem erfolgreichen Aufruf. Die Dokumentation nennt diesen Wert dann unbestimmt; `laenge == 0` zuerst zu fragen wäre die entscheidbare Frage. Praktisch ohne Folge (inference), daher kein Issue.

**L1: `sqlite_uri` und `%00`** (Low). Der Kommentar sagt, SQLite weise `%00` ab. Nachgemessen: SQLite kürzt den Pfad dort und öffnet die verkürzte Datei. `c_pfad` in derselben Datei weist ein Nullbyte ab.
Issue: `261024-2116-sqlite-uri-schreibt-ein-nullbyte-als-prozent-00-und-sqlite-kuerzt-dort-den-pfad.md`.

### Meldungen

**L2: Lesefehler sagen „Archiv“** (Low). `Lesefehler::grund` (`archiv/lesen.rs`) nennt für `VonAussenVeraendert` und `Fehlt` das Archiv, auch im Datenbankweg. Derselbe Schnitt wie im geschlossenen Defekt `261023-2035-…`, dort nicht erfasst.
Issue: `261024-2117-lesefehler-in-einer-sqlite-datenbank-sprechen-weiter-von-einem-archiv.md`.

### Projektregeln

- C-Freiheit: `cargo tree --target aarch64-apple-darwin -e normal,build` und dasselbe für `x86_64-apple-darwin` zeigen weder `cc` noch ein Paket mit einem Namen auf `-sys`. `Cargo.toml` und `Cargo.lock` sind im Bereich unverändert.
- `#![allow(unsafe_code)]` steht als Attribut genau in `crates/krk-core/src/verzeichnis/sys.rs` und `crates/krk-ui/src/appkit/mod.rs`.
- Sprachtabelle: neun Schlüssel in `Text` und drei in `Zahlwort`, je in `de.rs`, `fr.rs`, `en.rs` und in `ALLE` (761 auf 770, 40 auf 43). Feste Archivtexte (`ArchivNurLesbar`, `ImArchivNichtVerfuegbar`, `ArchivAlleinLesbar`, `ArchivUngueltig`) stehen im Betriebscode nur noch in den Antworten von `Archivformat` und im lokalen Zweig von `tabs.rs`.

### Proben

Die drei Probendateien des Kerns laufen grün und ohne Zeitabhängigkeit in der Abbruchprobe der Bindung (die Marke steht vor dem ersten Schritt). Zwei Lücken decken die Issues oben: der Fall „`-wal` ohne `-shm`“ (H1) und eine Zeile mit großer Zelle (M3). Dass `kopieren_einer_tabelle_in_ein_zip_legt_einen_eintrag_name_tsv_an` trotz ihres Namens drei Formate fährt, steht schon in der Regeldatei.

## Cross-cutting observations

- **Die Zusage zu `-shm` hat eine Stelle, an der sie gemessen wurde, und eine, an der sie gilt.** H1 und M1 haben dieselbe Ursache: die Begleitdatei verändert sich beim Lesen auf mehr Wegen, als die Messung abdeckt, und der Rest des Archivwegs (Kennung, Auffrischung, Vorschau) behandelt den Ordner, als ändere ein Lesen nichts darin.
- **`groesse` ist ein `u64` mit zwei Bedeutungen, und der Übersetzer trennt sie nicht.** Die Regeldatei sagt das selbst. M2 ist der erste Treffer außerhalb der Erhebung; ein Typ für die Größe eines Eintrags (Bytes oder Zeilen) würde die nächste Stelle beim Bau anhalten, statt sie einer Erhebung zu überlassen. Ein Vorschlag, keine Forderung.
- **„Folgt dem Format“ ist bisher eine Liste von Stellen, keine Eigenschaft.** `261023-2035-…` und L2 sind dieselbe Lücke an zwei Stellen.

## Recommended sequencing

1. H1 zuerst dem Nutzer vorlegen: welche Zusage gilt, und ob die Prämisse des Entscheids `261023-1605-…` korrigiert wird. Davon hängt ab, wie M1 gelöst wird.
2. M1 im Bündel nachweisen oder ausschließen (Nutzerarbeit oder eine Messung am laufenden Bündel).
3. M2 und M3 vor der nächsten Auslieferung.
4. L1 und L2 als Aufräumarbeit.

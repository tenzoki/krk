# Nachreview: Behebungen nach dem Abschlussreview des SQLite-Pakets

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `c824a972..86ac5164`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `261025-0010-die-zeitprobe-zum-abbruch-in-einer-500-mb-datei-scheitert-auch-ohne-aenderung-in-drei-von-fuenf-laeufen.md`, `261025-0010-die-zeitprobe-zum-abbruch-in-einer-500-mb-datei-scheitert-auch-ohne-aenderung-in-drei-von-fuenf-laeufen.record.json`, `261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md`, `261023-1340-betritt-der-doppelklick-eine-sqlite-datenbank-und-oeffnet-return-sie-weiter-extern.md`, `261023-1340-betritt-der-doppelklick-eine-sqlite-datenbank-und-oeffnet-return-sie-weiter-extern.record.json`, `261023-1340-was-tut-kopieren-aus-einer-sqlite-datenbank.md`, `261023-1340-was-tut-kopieren-aus-einer-sqlite-datenbank.record.json`, `261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.md`, `261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.record.json`, `261023-1340-welche-dateien-gelten-als-sqlite-datenbank.md`, `261023-1340-welche-dateien-gelten-als-sqlite-datenbank.record.json`, `261023-1340-wie-bindet-krk-sqlite-an-ohne-die-c-freiheits-zusage-zu-brechen.md`, `261023-1340-wie-bindet-krk-sqlite-an-ohne-die-c-freiheits-zusage-zu-brechen.record.json`, `261023-1340-zeigt-krk-sichten-einer-sqlite-datenbank-oder-nur-ihren-sql-text.md`, `261023-1340-zeigt-krk-sichten-einer-sqlite-datenbank-oder-nur-ihren-sql-text.record.json`, `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md`, `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.record.json`, `261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.record.json`, `261024-2112-mode-ro-legt-neben-einer-wal-datenbank-mit-wal-und-ohne-shm-eine-neue-shm-an.record.json`, `261024-2113-die-vorschau-einer-wal-tabelle-loest-ueber-die-shm-ihre-eigene-auffrischung-aus.record.json`, `261024-2114-die-ersatzangaben-der-vorschau-zeigen-die-zeilenzahl-einer-tabelle-als-bytes.record.json`, `261024-2115-ein-blob-wird-ganz-kopiert-um-seine-laenge-zu-nennen-und-eine-zeile-ist-unbegrenzt.record.json`, `261024-2116-sqlite-uri-schreibt-ein-nullbyte-als-prozent-00-und-sqlite-kuerzt-dort-den-pfad.record.json`, `261024-2115-ein-blob-wird-ganz-kopiert-um-seine-laenge-zu-nennen-und-eine-zeile-ist-unbegrenzt.md`
**Review domain:** code
**Verdict:** revise

Zu den nicht geöffneten Dateien: Es sind Workbench-Datensätze ohne Code, die Steuerdateien der sechs Issues und Entscheide, das Ereignisprotokoll und das Issue zur bekannten Zeitprobe. Ihre Zustände stehen nicht im Prüfauftrag. Teilweise gelesen habe ich: das Issue 2115 (Befund, Erwartet, Abnahme und Resolved, die ersten 30 Zeilen), die Analyse `261024-2133-…` (Messtabellen, Abschnitt 2 und Anhang, sonst über `grep`), das Abschlussreview `261024-2120-…` (Kopf und erste Abschnitte) und die neuen Proben in `crates/krk-core/tests/archiv_sqlite.rs` (die drei WAL-Proben und der Kopf der BLOB-Probe, nicht ihr Kindteil). Ganz gelesen sind die Code-Diffs aller 15 Code-Dateien, `HowTo.md` und `README.md` (Diff), die Issues 2112 bis 2117 außer 2115 und der Entscheid 2215.

## Summary

Alle sechs Issues sind in der Sache behoben, und Bau, Clippy unter `-D warnings`, C-Freiheit und die betroffenen Proben sind grün. Bei einem Befund hat die Behebung eine neue Lücke geöffnet. Die Vorfrage zum leeren `-wal` greift auch neben einem **laufenden** Programm im Sperrmodus `exclusive`. KRK liest dann ohne Sperre, wo vorher „wird gerade geschrieben“ kam, und weder Modulkopf noch `HowTo.md` noch der Entscheid 2215 sagen das (nachgemessen). Dazu kommen zwei kleine Punkte: Ein Fehlerzustand der Vorschau wird nicht mehr neu versucht, und ein Doc-Kommentar hängt an der falschen Funktion.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 2 |

## Stand der sechs Issues

| Issue | In der Sache | Beleg |
|---|---|---|
| 2112 `-shm` bei `-wal` ohne `-shm` | behoben nach Entscheid 2215 (Möglichkeit 4, Rest 1) | `archiv/sqlite.rs` `oeffnen`, `leeres_wal_ohne_shm`; Proben (a), (b), (d) in `tests/archiv_sqlite.rs`. Neue Lücke: M1 unten |
| 2113 Rückkopplung `-shm` → Vorschau | Absicherung gebaut: `Archivquelle`, `zeigt_archiveintrag`. Die Schleife im Bündel ist weder belegt noch ausgeschlossen, wie das Resolved ehrlich sagt | `vorschaumodell.rs`; Proben `eine_auffrischung_laedt_…_nicht_neu`, `nach_einer_echten_aenderung_…`. Nebenwirkung: L1 unten |
| 2114 Zeilen als Bytes | behoben, die Einheit hält der Typ (`Groessenangabe`) | `appkit/tabelle.rs` `archivangaben_des_eintrags`, `datenbankzeilen_am`; `appkit/vorschau.rs` `angabenzeilen`. Rest der Abnahme: die Einordnung in `.claude/rules/coding-was-man-nicht-sieht.md` ist nicht geschrieben (siehe unten) |
| 2115 BLOB ganz kopiert | BLOB-Teil behoben. `sqlite3_column_blob` ist aus der Bindung gefallen, 16 Funktionen nachgezählt | `verzeichnis/sys.rs` `bloblaenge`. Die Grenze für Textzellen ist nach Nutzerentscheid nicht gebaut, kein Befund |
| 2116 `%00` | behoben. `sqlite_uri` gibt `io::Result` und weist ab, `nur_lesend_oeffnen` macht daraus `SQLITE_MISUSE`. Damit greift die `immutable`-Stufe nicht, und der Weg geht über `fehler_zuordnen` zu `Ungueltig` | Probe `ein_pfad_mit_nullbyte_wird_abgewiesen_statt_gekuerzt_geoeffnet`. Sie prüft `sqlite_uri` und nicht den Weg über `archiv::sqlite::oeffnen` |
| 2117 „Archiv“ in Datenbankgründen | behoben. `Lesefehler::Fehlt(Archivformat)` und `VonAussenVeraendert(Archivformat)`, dazu die zwei festen Stellen in `operation/archiv.rs` und `operation/fern.rs` | Probe `eine_waehrend_des_kopierens_geaenderte_datenbank_nennt_im_bericht_kein_archiv`, Formattafel |

## Findings by theme

### Nur lesen und Sperre

**M1: Die Vorfrage zum leeren `-wal` liest neben einem laufenden exklusiven Schreiber ohne Sperre** (Medium).
`crates/krk-core/src/archiv/sqlite.rs`, `oeffnen`:

```rust
if wal_kopf(datei) && leeres_wal_ohne_shm(datei) {
    return nur_lesend_oeffnen(datei, true, abbruch);
}
```

Nachgemessen am 261025 mit `/usr/bin/sqlite3`. Ein Schreiber mit `locking_mode=exclusive` bleibt nach `wal_checkpoint(TRUNCATE)` offen, und daneben steht `x.db-wal` mit 0 Bytes, keine `-shm`, Kopf `02 02`. Dann gilt: `?mode=ro` → `database is locked (5)`, `?mode=ro&immutable=1` → liest. Nach dem Ende des Schreibers steht dasselbe leere `-wal`. „Läuft“ und „geschlossen“ sind an den Dateien nicht zu unterscheiden.

Die Analyse 2133 hat Weg B neben einem laufenden exklusiven Schreiber nicht gemessen („die heutige Regel wählt B hier nie“). Der Entscheid 2215 verwirft Möglichkeit 2 wegen genau dieses Wettlaufs, ohne ihn für Möglichkeit 4 zu wägen. Der Modulkopf (`# Nur lesen`: „in denen es nichts Neueres zu lesen gibt“) und `HowTo.md` („hält das andere Programm die Datenbank länger gesperrt, sagt die Statuszeile …“) stimmen für diesen Fall nicht mehr.

Abgemildert ist das für den Inhalt einer Tabelle durch die Kennungsfrage vor und nach dem Lesen (`Archivleser::datenbank_lesen`). Für das Inhaltsverzeichnis gilt das nicht. Die Datenbank selbst bleibt heil.

Zur Frage im Auftrag: Der Wettlauf zwischen Vorfrage und Öffnen ist dieselbe Risikoklasse wie die bestehende Rückfallstufe. Gesagt ist er im Modulkopf für keine der beiden Stufen. Neu und schwerer wiegt, dass ein **schon laufender** Schreiber gar nicht erkannt wird.
Fix: Nutzerentscheid, Wortlaut ehrlich machen oder eine `F_GETLK`-Vorfrage bauen (gemessen in Analyse 2133).
Issue: `261025-0030-die-vorfrage-zum-leeren-wal-liest-neben-einem-laufenden-exklusiven-schreiber-ohne-sperre-und-niemand-sagt-es.md`.

Sonst zur Vorfrage ohne Befund:
- Der WAL-Kopf wird an den Bytes 18 und 19 gelesen (`wal_kopf`, über `ohne_warten_oeffnen`, folgt einem Verweis). Ein nicht lesbarer Kopf ergibt nein.
- Das leere `-wal` kommt über `wal_angaben` → `fs::metadata(wal_pfad(datei))` und folgt einem Verweis wie die Bibliothek.
- Die fehlende `-shm` kommt über `fs::symlink_metadata(begleitpfad(datei, "-shm"))` und allein `NotFound`. Ein hängender Verweis zählt als stehend und führt damit zum vorsichtigeren `mode=ro`.
- Beide Begleitpfade hängen über `begleitpfad` an der aufgelösten Hauptdatei (`fs::canonicalize`), wie der Modulkopf es für `-wal` begründet.
- Die Rückfallstufe `ruhende_wal_datenbank_ohne_wal` trägt keine `-shm`-Bedingung, und Probe (d) hält das.
- Die zwei Ausnahmen überlappen nicht: stehendes gegen fehlendes `-wal`.

### Vorschau

**L1: Nach einem Lesefehler lädt die Vorschau denselben Archiveintrag bei gleicher Kennung nie wieder** (Low).
`vorschaumodell.rs`: `archiveintrag_laden` macht aus jedem Fehler `Inhalt::Hinweis(ersatz)`, und die Lieferung setzt `tab.archivquelle = vorgang.archivquelle.take()` ohne Bedingung. Das trifft alle fünf Formate, nicht nur SQLite. Ein vorübergehender Fehler ohne neue Kennung bleibt stehen, bis die Auswahl wechselt; Beispiele sind ein `Belegt` durch einen Rollback-Schreiber, der zurückrollt, oder ein E/A-Fehler auf einer Freigabe. Vorher half jede Auffrischung.
Issue: `261025-0031-nach-einem-lesefehler-laedt-die-vorschau-denselben-archiveintrag-bei-gleicher-kennung-nie-wieder.md`.

Ohne Befund geprüft:
- anderer Tab: `archivquelle` hängt je Tab.
- anderes Archiv: `Archivort` vergleicht Pfad und `Innenpfad`, nicht den Zugang; ein ersetztes Archiv ändert die Kennung.
- schnelles Wechseln: der laufende Auftrag geht in `zeigt_archiveintrag` vor.
- ein Verzeichnis, das sich zwischen Auftrag und Lesen ändert: falsch markiert führt das zu einem Ladevorgang zu viel, nie zu einem verschluckten.
- Sprache: sie ist ein Wert des Prozesses und wechselt nicht.
- Profile und Heimordner: `neu_laden_wo` überspringt Archivtabs schon vorher.

Eine Randbeobachtung ohne Issue: Nach einem Wechsel des Erscheinungsbildes zeigt ein gefärbter Archiveintrag bis zum Auswahlwechsel die alte `Tafel`. Vorher färbte die nächste Auffrischung ihn nebenbei neu; zugesagt war das nirgends, und die offene Frage zur Verweisfarbe (`260812-1701_*_…`) liegt daneben.

### Aufräumreste

**L2: Doc-Kommentar von `rumpf_von` hängt an `freie_funktion`** (Low).
`crates/krk-ui/src/appkit/tabelle.rs`, Prüfmodul, eingeführt in `8a3b212a`. Die Begründung „Eine Naeherung …“, die vom Schnitt an vier Leerzeichen handelt, steht jetzt an der Funktion, die auf Dateiebene schneidet.
Issue: `261025-0032-der-doc-kommentar-von-rumpf-von-haengt-seit-dem-einschub-von-freie-funktion-an-der-falschen-funktion.md`.

**Rest der Abnahme von 2114, ohne eigenes Issue:** Die Abnahme verlangte die Einordnung jeder Lesestelle von `Eintrag::groesse` in `crates/krk-ui/src` in der Regeldatei, und das Resolved sagt, sie sei nicht geschrieben. Meine eigene Erhebung (`grep -rn '\.groesse\b' crates/krk-ui/src`) findet drei Lesestellen im Archivtab: `appkit/tabelle.rs` in `fernangaben_des_eintrags`, in `archivangaben_des_eintrags` und in `groessenzelle`. Alle drei sind richtig eingeordnet. Der Absatz zu `Archiveintrag::groesse` in `.claude/rules/coding-was-man-nicht-sieht.md` nennt für die Oberfläche weiter allein `groessenzelle` und `masseinheit`, nicht `archivangaben_des_eintrags` und `datenbankzeilen_am`. Das ist ein Nachtrag für einen Curate-Lauf und kein Codedefekt.

### Projektregeln

- Die C-Freiheit hält: `cargo tree --target {aarch64,x86_64}-apple-darwin -e normal,build` zeigt kein `cc` und kein `-sys`.
- `#![allow(unsafe_code)]` steht genau in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`, und die Probe `baum` ist grün.
- Die neue `unsafe`-Stelle `bloblaenge` trägt einen SICHERHEIT-Kommentar. `sqlite3_column_bytes` wandelt bei einem BLOB nichts um, also ist kein Zeiger davor nötig.
- Die Zählangaben „dreissig“ und „sechzehn“ in `lib.rs`, `verzeichnis/mod.rs` und `verzeichnis/sys.rs` stimmen mit dem Block überein (16 `fn sqlite3_*` nachgezählt).
- Die Sprachtabelle hat `DatenbankBeimLesenVeraendert` und `DatenbankEintragFehlt` in `de.rs`, `fr.rs` und `en.rs`, und `ALLE` steht auf 772. Die Umlaute stehen allein in der Tabelle.

Gefahren: `cargo test -p krk-core --test sqlite_bindung --test archiv_sqlite --test archiv_operation --test baum --test sprache` (alle grün, je eine Probe ignoriert in zwei Läufen), `cargo test -p krk-ui -- archiv datenbank ersatzangaben` (49 grün) und `cargo clippy --workspace --all-targets -- -D warnings` (sauber). `make check` habe ich nicht ganz gefahren. Die bekannte Zeitprobe `der_abbruch_mitten_in_einer_500_mb_datei_…` ist nicht Teil dieses Laufs.

## Cross-cutting observations

- Beide Abmilderungen dieser Runde ruhen auf derselben `Dateikennung` aus Hauptdatei und `-wal`: die Kennungsfrage um das Lesen einer Tabelle (M1) und der Kurzschluss der Vorschau (2113, L1). Was sich allein an `-shm` oder allein an der Sperre ändert, sieht keine der beiden, und das ist bei beiden einmal gewollt (`-shm`) und einmal nicht (Sperre, vorübergehender Fehler).
- Eine Messung deckte wieder einen Fall nicht ab, auf den eine Regel sich stützt. Die Analyse hat das leere `-wal` nur ruhend gemessen und den laufenden exklusiven Schreiber nur mit vollem `-wal`. Im Abschlussreview war es dasselbe Muster mit `-wal` ohne `-shm`.

## Recommended sequencing

1. M1 vor der Abnahme durch den Nutzer: Er entscheidet zwischen Wortlaut und `F_GETLK`, weil die Häufigkeit (Firefox, Safari laut Erhebung 2133) die Lage zum Normalfall macht.
2. L1 und L2 als Aufräumarbeit, L1 mit der Frage im Issue, welche Fehler vorübergehend heißen.
3. Der Nachtrag in der Regeldatei zu 2114 geht in den nächsten Curate-Lauf.

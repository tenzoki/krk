# SQLite-Datenbanken im Dateifenster und in der Vorschau durchsehen

## Directive

KRK soll eine SQLite-Datei wie `/Users/k1/Projects/productive/wordlist/wordstore/wordstore.db` durchsehen lassen. Die Idee des Nutzers: Pfeil nach rechts auf der Datei öffnet die Datenbank und listet ihre Tabellen im Dateifenster; steht die Auswahl auf einer Tabelle, zeigt die Vorschau deren Inhalt. Erster Schritt ist eine Analyse, wie sich das in KRK einfügt; erreicht ist das Paket, wenn ein Nutzer so durch die Tabellen einer Datenbank blättern kann, ohne KRK zu verlassen.

Wortlaut des Nutzers (261023):

> make a new package, analys how we could use the preview window to browser sqlite db's like : /Users/k1/Projects/productive/wordlist/wordstore/wordstore.db
>
> my idea would be:
> - when pressing right arrow on file we open the db and list the tables in the file window, when moving on a table the table is beeing displayed in the preview.

## Nutzerwort 261024, nach dem Abschlussreview

> dont think it make sense to export blobs.
> you can go on in automode from here and invlove the consultant as sparring partner.

Folge: BLOB-Inhalte werden weder in der Vorschau noch in der Ausfuhr gezeigt; der Platzhalter `[BLOB, n Bytes]` bleibt, eine Zellgrenze für BLOBs ist damit unnötig. Die Review-Issues werden ohne Rückfrage je Schritt abgearbeitet, Entscheidungen mit dem Consultant als Sparringspartner getroffen und im Bericht an den Nutzer begründet.

## Schließungsnotiz 261025

Abgeschlossen mit `done`, Commits `c987c04f..e71beb42`. Plan `261023-1507` mit allen acht Schritten auf `done`; drei Reviews (`261024-2120` revise, `261025-0035` revise, `261025-0215` accept), deren Befunde behoben und geschlossen sind. Nach dem Nutzerwort vom 261024 („go on in automode … consultant as sparring partner“) sind die Entscheide `261024-2215` und die Sperrfrage mit dem Consultant getroffen worden.

Die Haltebedingungen aus `## Where this work stops` sind dem Nutzer nicht einzeln vorgelegt worden (Automodus); wörtlich:

1. Fertig ist diese Arbeit, wenn die Schritte 1 bis 8 auf `done` stehen und für jeden die Akzeptanz seines Schritts gelesen ist.
2. Misst Schritt 1, dass eine nur lesende Verbindung die Hauptdatei oder `-wal` verändert, endet die Arbeit nach Schritt 1 … — nicht eingetreten (gemessen: allein `-shm`).
3. Misst Schritt 1, dass allein `-shm` sich ändert, beginnt Schritt 3 erst, wenn der Nutzer die Frage beantwortet hat — eingetreten, Nutzer hat vor Schritt 3 geantwortet (Entscheid `261023-1605`).
4. Schritt 4 beginnt erst, wenn der NULL/BLOB-Entscheid beantwortet ist — eingehalten.
5. Fällt eine Probe aus Schritt 5 rot, ist das ein Halt — keine war rot.
6. Kein Agent fährt einen Abnahmelauf gegen L1 bis L10 und keinen Durchgang am laufenden Bündel; ob das Paket erst nach dem Bericht des Nutzers auf `done` geht, fragt der Orchestrator an der Schlussfreigabe — nicht gefragt; das Paket geht ohne den Bündeldurchgang auf `done`, die Prüfung am Bündel bleibt Nutzerarbeit (Entscheid F6, „ich checke selbst“).
7. `der_archiveinstieg_stellt_auf_dem_hauptfaden_keinen_systemaufruf` ist beim Schließen grün und gegenüber `dc4ed474` unverändert — nicht eigens nachgelesen; die Probe läuft in `cargo test --workspace` grün.
8. Beide `cargo tree`-Läufe ohne `cc`/`-sys`, `git diff dc4ed474 -- Cargo.toml crates/*/Cargo.toml Cargo.lock` leer — gehalten (letzter Review).
9. Keine Auslieferung, kein Tag, keine Versionszahl — gehalten.

Offen bleiben: Issue `261025-0215` (leicht, Deskriptor-Schließen vor jedem Öffnen); die Zeilen- und Zellgrenze für große Textzellen (2115, bewusst nicht gebaut); `shared/issues/261025-0010` (Zeitprobe, gerätebedingt); die Einordnung der Byte-Stellen aus 2114 in der Regeldatei (für einen Curate-Lauf). Die zehn Zusagen aus C8 sind nicht gemessen (Entscheid `261023-1340-schuldet-das-sqlite-paket-einen-abnahmelauf-gegen-die-zehn-zusagen`).

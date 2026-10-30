# Abschlussdurchsicht: die Vorschau lädt Cloud-Dateien verzögert, einzeln und abbrechbar

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `38869c7f..0650dd47`
**Not-opened:** none
**Review domain:** code
**Verdict:** revise
**Cross-references:** `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1444-welche-leser-duerfen-eine-platzhalterdatei-herunterladen.md`, `261025-1444-was-heisst-abbrechen-fuer-einen-download-den-die-vorschau-schon-angestossen-hat.md`

Gelesen wurden die fünf Commits `8679feb7`, `09d3ff91`, `6abfeac1`, `2c5abac4` und `0650dd47` über `git diff` je Datei der Auftragsliste, dazu der umgebende Code in `vorschaumodell.rs` (`Ladevorgang`, `einziehen`, `folge_nachziehen`, `datei_anzeigen`, `neu_laden_wo`, `Leseart`), `cloudladen.rs` ganz, `verzeichnis/leser.rs` (jeder Bau eines `Eintrag`) und `git/leser.rs` (`Gitleser::marken`). Nicht gelaufen: `cargo test`, `cargo check`, `clippy` (Auftrag). Keine Datei unter `/Users/k1/Google Drive` geöffnet.

## Summary

Der Bau hält, was der Plan zusagt: kein überholter Faden öffnet eine Platzhalterdatei, höchstens einer belegt den Ladeplatz, ein Ergebnis erreicht nie eine neuere Auswahl, und eine lokale Datei nimmt den Weg ohne Wartezeit und ohne Platz. Drei Befunde, keiner hält eine Auslieferung an: der Platz bleibt über das Lesen hinaus belegt (Widerspruch zu Plan und Kommentar), ein vierter Hintergrundleser, die Markenspalte des Git-Bereichs, ist ungeprüft, und ein Satz der Anleitung stimmt für das Foto der Bildfolge nicht.

## Totals

| Critical | High | Medium | Low |
|---|---|---|---|
| 0 | 0 | 1 | 2 |

## Was geprüft ist und hält

**Abbruch und Ladeplatz.** Die Marke setzt allein `Drop for Abbruchmarke` (`vorschaumodell.rs`), und der Vorgang fällt mit seinem Empfänger. Ein Faden, dessen `laden` `None` liefert, tut das nur nach gesetzter Marke (`abwarten` → `false`, `belegen` → `None`, Schleuse → `Lesehindernis::Abgebrochen`), also nur, wenn niemand mehr zuhört. Die vorläufige Meldung geht in einen leeren `sync_channel(1)` und blockiert nicht; die endgültige wird nach dem Ende von `laden` geschickt, die `Belegung` ist dann schon gefallen, ein blockiertes `send` hält den Platz also nicht. `Belegung::drop` gibt auch beim Abwickeln nach einem Absturz frei, und eine vergiftete Sperre wird weiter benutzt (`Ladeplatz::sperren`). Ein Platz, der nie frei wird, bleibt allein im angenommenen Fall eines Systemaufrufs, der nie zurückkehrt (Entscheid zum Abbruch, Möglichkeit 1).

**Kein veraltetes Ergebnis.** Jeder Auftrag an einen Tab ersetzt `tab.ladevorgang` und damit den Empfänger; `einziehen` liest nur aus dem aktuellen. In der Bildfolge prüft `folge_nachziehen` zusätzlich `foto.pfad() == vorgang.pfad`, und `blaettern` ersetzt den `bildvorgang`. Die vorläufige Meldung setzt Inhalt und Pfad zusammen, `Vorschautab::pfad` sagt also weiter, was auf dem Schirm steht.

**Prüfen vor dem Öffnen.** `vor_dem_oeffnen` (`text/datei.rs`) fragt `std::fs::metadata` (folgt einer Verknüpfung wie das Öffnen danach, öffnet nichts), prüft Kennzeichen und Größe allein für eine gewöhnliche Datei und lässt alles andere zur Typfrage am Deskriptor. Eine Röhre blockiert damit weiter nicht. Der Wettlauf zwischen `stat` und `open` ist im `**Decidability:**` des Plans benannt und hingenommen. `stueckweise_lesen` verliert an der Stücknaht kein Byte (Probe `eine_datei_ueber_einem_lesestueck_kommt_ganz`); an einer Datei genau auf der Stückgrenze fragt es `weiter` ein letztes Mal vor einem leeren Lesen, was harmlos ist und vom Doc-Kommentar „nach dem letzten noetigen Stueck wird `weiter` nicht mehr gefragt“ leicht abweicht.

**Hintergrundleser.** Die drei Leser des Entscheids öffnen keine Platzhalterdatei mehr: `traegt_der_inhalt` und `angelesener_text` rufen mit `Platzhalterregel::Meiden`, `gruppe_ordnen` ruft den Datumsleser für `eintrag.platzhalter` nicht. Jeder lokale `Eintrag` entsteht über `Eintrag::aus_roh` (`verzeichnis/leser.rs`), trägt das Kennzeichen also; die übrigen Bauten (Archiv, fern, Proben) setzen `false`. Die Archivart wird am Namen erkannt (`Archivformat::aus_namen`), nicht am Kopf. Eine Suche nach jedem Öffnen im Betriebscode von `krk-core` und `krk-ui` findet sonst nur Wege, die der Nutzer auf eine Datei richtet, und die Markenspalte (Befund 2).

**Der lokale Weg.** Für eine Datei ohne `SF_DATALESS` steht vor der `Leseart` weder Wartezeit noch Platz (Probe `eine_lokale_datei_wartet_nicht_und_belegt_den_platz_nie`). Neu sind ein `stat` vor jedem Öffnen, das Lesen in Stücken zu 1 MiB und der Abbruch an der Marke; die Ergebnisse je `Leseart` sind dieselben wie vorher.

## Findings by theme

### Ladeplatz

**1. Der Platz bleibt bis zum Ende von `laden` belegt, nicht bis zur Rückkehr des Lesens.** Low. Scope: `crates/krk-ui/src/vorschaumodell.rs`, allein der Weg einer Platzhalterdatei.

`let _belegung = if platzhalter { … }` lebt bis zum Ende von `laden`; dazwischen stehen `text_als_inhalt` (mit `markdown::rendern`), `jsonzeilen_aus_bytes`, `endzeichen_der_datei` und `trenntext_aus_bytes`, keines fragt `weiter`. Der Kommentar daneben sagt „also bis das Lesen darunter zurueckgekehrt ist“, der Modulkopf „allein das Lesen haelt den Ladeplatz“, der Plan „Den Platz gibt er frei, wenn sein Lesen zurückgekehrt ist.“ „Höchstens einer zugleich“ hält trotzdem; die nächste Cloud-Vorschau wartet nur länger. Richtung: erst lesen, Belegung fallen lassen, dann aufbereiten, oder die drei Texte auf das Verhalten ziehen.
Issue: `261026-0608-der-ladeplatz-der-vorschau-bleibt-ueber-das-lesen-hinaus-belegt-bis-markdown-und-trenntext-aufbereitet-sind.md`.

### Hintergrundleser

**2. Die Markenspalte des Git-Bereichs ist ungeprüft.** Medium, Sicherheit niedrig. Scope: `crates/krk-core/src/git/leser.rs`, außerhalb des Diffs.

`Gitleser::marken` fährt `repo.status(…).into_iter(muster)` für jeden gelesenen Ordner in einem Repository. Der Entscheid zu den Lesern bindet „jeden künftigen Leser“ und nennt diesen nicht. `inference:` `gix` hasht eine verfolgte Datei, deren `stat`-Angaben vom Index abweichen, und das Auslagern hebt `st_ctime`; träfe beides zu, lüde das Öffnen eines Ordners jede ausgelagerte verfolgte Datei herunter. Nicht gemessen.
Issue: `261026-0608-die-markenspalte-des-git-bereichs-ist-ein-hintergrundleser-den-der-entscheid-zu-platzhalterdateien-nicht-nennt.md`.

### Anleitung

**3. „Die Bildfolge … lädt nichts herunter“ stimmt für das Foto an der Stelle nicht.** Low. Scope: `HowTo.md`.

Das Foto an der Stelle einer Bildfolge geht über `Ladevorgang::starten` durch das Tor in `laden` und wird nach 0,2 s geladen; so sagt es auch die Tabelle im Modulkopf von `vorschaumodell.rs`. Der Satz folgt dem Wortlaut von Schritt 8 des Plans.
Issue: `261026-0608-die-anleitung-sagt-die-bildfolge-lade-nichts-herunter-und-das-foto-an-der-stelle-laedt-sie-doch.md`.

## Cross-cutting observations

- **Eine Auffrischung während eines Downloads** startet den Vorgang derselben Datei neu und bricht den laufenden an der Marke ab. Der Plan nimmt das unter `## Risks & Mitigations` hin; die Kosten jeder Auffrischung der Vorschau trägt schon der offene Defekt `260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md`. Kein neuer Datensatz.
- **`endzeichen_der_datei`** liest für eine lokale CSV über 1 MB ohne `\n` im ersten MB ein zweites Mal bis `EDITORGRENZE` (16 MiB) mit `&|| true`, also nicht abbrechbar. Für eine Platzhalterdatei ist der Weg unerreichbar, weil das Tor sie über 1 MB abweist; für eine lokale ist es der Stand vor dem Paket. Kein Datensatz; wer Befund 1 baut, kann `umstand.weiter` dort mitreichen.
- **Eine Binärdatei unter 1 MB** mit `Leseart::Text` (etwa ein kleines `.zip` oder `.mp4`) lädt die Vorschau nach 0,2 s herunter, um danach ihre Metadaten zu zeigen. Das folgt der Regel des Nutzers („die Vorschau lädt“) und ist kein Defekt; erwähnt, weil der Nutzer es an seinem Laufwerk sehen wird.
- **Der Tiefenfilter** stellt jetzt je Datei ein `stat` mehr (vor `open` und `fstat`), auch lokal. Ungemessen; keine Zusage hängt am Inhaltsfilter.

## Recommended sequencing

Nichts davon hält eine Auslieferung an. Befund 3 ist eine Textänderung in `HowTo.md` und gehört vor die nächste Auslieferung, weil die Anleitung im Releasepaket mitreist. Befund 1 ist klein und kann mit Befund 3 gehen. Befund 2 braucht zuerst die Messung am Laufwerk des Nutzers (Möglichkeit a der Abnahme); erst ihr Ergebnis sagt, ob Code nötig ist. Die Abnahme des Pakets auf Google Drive bleibt Nutzerarbeit (`## Where this work stops` des Plans).

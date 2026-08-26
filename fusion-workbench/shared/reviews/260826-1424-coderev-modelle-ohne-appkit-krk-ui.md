# Durchsicht R12: die sechs Modelle ohne AppKit unter `crates/krk-ui/src/`

**Reviewed-range:** `004ff72..ca8072d`
**Not-opened:** none
**Reviewer:** coderev
**Gelesen:** `fenstermodell.rs` (2.792), `tabs.rs` (2.456), `editormodell.rs` (2.256),
`vorschaumodell.rs` (1.457), `leistenmodell.rs` (1.253), `zettelmodell.rs` (474) — je ganz,
dazu die Rufer in `appkit/anwendung.rs`, `appkit/editor.rs`, `appkit/leiste.rs`,
`appkit/aufteilung.rs`, `appkit/tabelle.rs` und `krk-core/src/verzeichnis/leser.rs` an den
zitierten Stellen.

## Summary

Die sechs Modelle sind die sauberste Schicht des Baums: vollständige Fallunterscheidungen,
eine Schreibstelle je Zustand, Proben, die Verhalten messen. Was auseinanderläuft, ist nicht der
Zustand, sondern die Zusagen darüber: der Editor hält seine 16 MB nur am Eingang, die
Übertragung beim Tabersatz steht in drei Fassungen, und die `#[must_use]`-Regel, die die Dateien
selbst ausschreiben, fehlt an rund 25 ihrer eigenen Antworten. Sieben Defekte gefiltert: einer
Mittel, sechs Niedrig. Sieben Altbefunde geprüft, alle sieben gelten weiter.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 6 |

## Findings by theme

### 1. Grenzen, die nur eine Seite halten

**Mittel — der Editor hält die 16 MB nur am Eingang.** `EDITORGRENZE` prüft allein
`datei::oeffnen` auf dem Ladefaden (`editormodell.rs:467`). `bearbeiten` (`:941-947`),
`treffer_ersetzen` (`:1180-1203`), `alle_treffer_ersetzen` (`:1213-1230`) lassen den Stand
wachsen, `sichern` (`:986-1007`) schreibt ihn ungeprüft; das nächste F4 nach `schliessen` oder
Neustart antwortet `Abgewiesen(ZuGross)`. Die Ansicht bindet ihr Budget an die Zahl
(`appkit/editor.rs:879-885`), der Stand hat keine Bindung.
→ `shared/issues/260826-1417_*_der-editor-haelt-die-16-mb-nur-am-eingang-…md`

**Niedrig — ohne Stempel meldet `fremd_geaendert` nie mehr.** `fremd_geaendert`
(`editormodell.rs:1036-1041`) antwortet `false` bei `stempel = None`, und `sichern` (`:999`)
sowie `uebernehmen` (`:778`) können genau diesen Zustand bei gehaltener Datei herstellen. Die
Invariante „Pfad und Stempel zusammen gesetzt" hält nichts.
→ `shared/issues/260826-1418_*_ohne-stempel-meldet-fremd-geaendert-nie-mehr-…md`

### 2. Dieselbe Regel in mehreren Fassungen

**Niedrig — `schliessen` am letzten Tab ist die dritte Fassung der Übertragung.**
`ordner_setzen` trägt fünf Werte (`tabs.rs:653-681`), `verdeckten_tab_setzen` zwei
(`:485-495`), `schliessen` null (`:560-571`); Sortierung und Verstecke fallen dort ohne Satz.
Der Altbefund `260815-0020` (zwei von vier) steht damit auf zwei von fünf und drei Fassungen.
→ `shared/issues/260826-1419_*_schliessen-am-letzten-tab-ist-die-dritte-fassung-…md`;
Nachtrag am Altbefund.

### 3. Tote Zweige

**Niedrig — `lesereihenfolge` liefert eine zweite Stufe, die ihr Rufer verwirft.**
`fenstermodell.rs:957-972` baut zwei Stufen; `lesevorgaenge_starten`
(`anwendung.rs:2879-2890`) filtert auf `stelle == sichtbar`. Die zweite Stufe lebt in
`Tabliste::nachzuegler_starten` (`tabs.rs:792`). `Tabuebersicht::zahl` und zwei Proben messen
den toten Teil.
→ `shared/issues/260826-1420_*_lesereihenfolge-liefert-eine-zweite-stufe-…md`

### 4. `#[must_use]`

**Niedrig — rund 25 reine Antworten ohne Marke, in allen sechs Dateien.** Die vier, deren
Fallenlassen genau den Schaden hätte, den die Doc-Kommentare daneben beschreiben:
`Zettelmodell::wechseln` (`zettelmodell.rs:218`, `GewechseltZuSichern`),
`Editormodell::sichern` (`editormodell.rs:986`), `Vorschaumodell::einziehen`
(`vorschaumodell.rs:578`), `Leistenmodell::gueltigkeit_pruefen` (`leistenmodell.rs:303`).
Heute bindet jeder Rufer; die Marke kostet keine Aufrufstelle.
→ `shared/issues/260826-1421_*_must-use-fehlt-an-rund-25-reinen-antworten-…md`

### 5. Proben

**Niedrig — der Helfer `liste` verspricht „liest nie", drei Probengruppen lesen.** `waehlen`,
`schliessen`, `ordner_setzen` starten `Lesevorgang` gegen `/a`…`/c`, `/` und `temp_dir()`
(`tabs.rs:1153-1160`, `:1447-1450`); dieselbe Datei nutzt ab `:1995` den `Pruefordner` der
Kiste.
→ `shared/issues/260826-1422_*_der-probenhelfer-liste-verspricht-…md`

Sonst messen die Proben aller sechs Dateien Verhalten und nicht den eigenen Aufbau. Die eine
Ausnahme, die das Muster streift, ist
`die_zuordnung_von_bereich_auf_sichtbarkeit_trifft_jedes_feld` (`fenstermodell.rs:2138`): sie
baut die Zuordnung im Prüfmodul ein zweites Mal auf, prüft damit aber Injektivität, und das ist
eine Aussage über `sichtbar_in`. Nicht gefiltert.

### 6. Zählangaben in Prosa

**Niedrig — zwei Zahlen zu `Inhalt` sind seit der Runde 16 um eins falsch.**
`vorschaumodell.rs:552-555` („ein siebter"), `:1162-1169` („alle sechs").
→ `shared/issues/260826-1423_*_zwei-zaehlangaben-zu-inhalt-…md`

## Die Zusagen aus `CLAUDE.md`, je Datei geprüft

- **`fenstermodell.rs`:** `Bereich` trägt fünf Werte (`:103-118`), stimmt. Der zeitliche
  Ausschluss geht über die eine Schreibstelle `sichtbar_setzen` (`:524`) und
  `gegenueber_raeumen` (`:538`); `umschalten` (`:677-679`) räumt nach jedem Einschalten,
  `aus_sitzung` (`:428-430`) beim Start. **Beide sichtbar ist über keinen Weg erreichbar**
  (Probe `:2533` fährt alle Paare); **beide unsichtbar** ist ein zulässiger Zustand und kein
  Fehler (Auslieferung ohne Editor und Vorschau weggeschaltet). Kein Befund.
- **`tabs.rs`:** `auswahl_auf_namen` fragt `liest()` zuerst (`:758`), stimmt; die anderen Wege
  zur Auswahl gehen über `wunschauswahl` (`ordner_setzen` `:671`, `aktiven_neu_lesen` `:723`,
  `aus_zustand` `:110`) und lösen sie in `wunschauswahl_anwenden` beim Abschluss ein (`:1126`).
  Der Filtertext lebt im `Ordnermodell` des Tabs und fällt mit ihm. **„Deep" gilt am Baum je
  Tab:** `tief` wohnt im `Ordnermodell` jedes `Tabinhalt`, ein neuer Tab bekommt die
  Vorbelegung (Probe `:1605`), `ordner_setzen` trägt ihn innerhalb des Tabs weiter,
  `verdeckten_tab_setzen` nicht. Die Frage `260814-1830` bleibt offen; der Code hat sie mit
  „je Tab" beantwortet, ohne dass ein Datensatz es sagt.
- **`editormodell.rs`:** die Grenze hält das Modell nicht selbst (Befund 1). „Ungesichert" ist
  die Marke `abweichung`, gesetzt in `bearbeiten`/Ersetzen, gelöscht in `uebernehmen`, `sichern`,
  `schliessen`; der Preis (zurückgenommene Änderung meldet weiter) steht im Modulkopf. **Das
  Modell kann nicht „gesichert" sagen, während die Platte etwas anderes trägt** — ausser über den
  Stempel-`None`-Weg (Befund 2).
- **`vorschaumodell.rs`:** die fünf Entscheidungen `260825-1725_i_*` sind in `krk-core`
  umgesetzt; das Modell reicht die `Zusammenfassung` unverändert durch (`:708-710`) und ist der
  eine Rufer von `zusammenfassen` (Zählprobe `:1430`). **`juengste` mit Anzahl null fängt das
  Modell nicht ab** und kann es nicht: es sieht Zeilen, keine Profile. `260826-1225` gilt weiter
  und liegt bei `krk-core`. „Ordner ohne Auswahl" wohnt in `appkit/tabelle.rs`, nicht hier.
- **`leistenmodell.rs`:** der Fehler beim Sichern erreicht das Modell nie. `uebernehmen`
  (`:518-523`) nimmt die Liste als gegeben und liefert `()`; das Modell rechnet seit der Runde 7
  nicht mehr selbst. `260826-1325` gilt weiter, und die Behebung liegt ganz in
  `anwendung.rs::lesezeichen_aendern`.
- **`zettelmodell.rs`:** das Modell weiss je Zettel, was ungesichert ist (`weicht_ab`, `:88`),
  und `zu_sichern` nennt **alle** abweichenden (`:248`). Die vier Schreibmomente liegen beim
  Rufer. Kein Befund ausser dem fehlenden `#[must_use]` an `wechseln`.

## Altbefunde

| Datensatz | Stand am Baum `ca8072d` |
|---|---|
| `260812-0801` (`aufteilung::sichtbar_im` in zwei Modulköpfen) | **gilt weiter**: `spalten.rs:12` und `appkit/tabelle.rs:310` (die Zeile im Datensatz, 185, ist gewandert) |
| `260812-0700` (Breitenschritt neben gedeckeltem Bereich gekürzt) | **gilt weiter**: `massstab` (`fenstermodell.rs:862-873`) ist ein Faktor; Probe `:2018` schreibt 20,36 aus |
| `260812-0512` (F4 am schmalen Fenster) | **gilt weiter**: `mindestbreiten_passen` (`:697`) ist privat und hat einen Rufer (`:653`) |
| `260815-0020` (`verdeckten_tab_setzen`, zwei von vier) | **gilt weiter, jetzt zwei von fünf und drei Fassungen**; Nachtrag angehängt |
| `260814-1830` (Deep je Tab oder je Fenster) | **offen**; der Code tut „je Tab" |
| `260826-1225` (`juengste` mit Anzahl null) | **gilt weiter**; nicht im Modell abfangbar |
| `260826-1325` (Lesezeichen anlegen meldet „angelegt") | **gilt weiter**; der Fehler geht vor dem Modell verloren |

## Cross-cutting observations

- **Die Übertragung beim Tabersatz ist die eine Regel dieser Schicht, die in drei Fassungen
  steht**, und jede Runde, die dem Tab einen Wert gibt, verlängert die längste und nicht die
  kürzeren. Eine Funktion mit drei Rufern schlösse das.
- **Die `#[must_use]`-Lücke ist über alle fünf bisherigen Durchsichten dieselbe Lücke**: die
  Regel steht in `CLAUDE.md`, wird in den Dateien ausgeschrieben und dort nicht auf die Nachbarn
  angewandt. Eine Clippy-Zeile (`clippy::must_use_candidate`) fände alle auf einmal; ob sie den
  Baum zu laut macht, wäre zu messen.
- **Zustand an zwei Stellen** gibt es hier genau einmal, und beide Halter wissen es:
  `AufteilungsDelegierter::wuensche` neben `Fenstermodell::breiten`, abgeglichen über
  `Aufteilung::anwenden`. `wuensche_nachfuehren` (`fenstermodell.rs:1221-1233`) gibt bei einer
  Ziehbewegung `gemessen` ganz zurück, und das trägt für jeden ausgeblendeten Bereich `None`
  (`aufteilung.rs:527-541`); die gespeicherte Breite eines ausgeblendeten Bereichs lebt dann bis
  zum nächsten `anwenden` allein im Modell. Sichtbare Folge habe ich keine gefunden, weil jeder
  Weg zum Wiedereinblenden über `anwenden` geht. Nicht gefiltert; hier festgehalten, damit die
  nächste Durchsicht von `aufteilung.rs` weiss, dass die Frage gestellt war.

## Recommended sequencing

1. Befund 1 (Editorgrenze) vor die nächste Auslieferung: eine Zusagefrage mit drei Wegen, und
   der billigste (`sichern` weist ab) ist eine Handvoll Zeilen.
2. `#[must_use]` an die vier fetten Stellen aus Befund 5, im selben Zug wie die Geschwister aus
   den Durchsichten R7 bis R11.
3. Die Übertragungsfunktion für den Tabersatz (Befund 3 samt `260815-0020`), sobald der Nutzer
   die Entwurfsfrage beantwortet.
4. Der Rest ist Aufräumen ohne Reihenfolge.

**Verification:** alle Zeilenangaben am Baum `ca8072d` abgelesen und ein zweites Mal
gegengelesen; kein Kommando hat den Quellbaum angefasst, nichts übersetzt; die Rufer der
gemeldeten Stellen in `appkit/` per `grep` und Lesen der Fundstellen geprüft.

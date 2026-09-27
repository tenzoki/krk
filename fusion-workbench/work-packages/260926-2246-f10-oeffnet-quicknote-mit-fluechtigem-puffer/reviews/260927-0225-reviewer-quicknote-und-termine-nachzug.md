# Durchsicht: die Quicknote auf F10 und der Nachzug der Termine

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `f6981bf..171b380`
**Not-opened:** none
**Domain:** code
**Work-item:** `260926-2246-f10-oeffnet-quicknote-mit-fluechtigem-puffer`

Geöffnet ist jede Datei des Bereichs: die Quelldateien und `appkit/quicknote.rs` ganz oder über ihren vollen Diff, `CLAUDE.md` über den Wortdiff, die Werkbankdatensätze aus `385f106` und die Zeilen von `fusion-workbench/orchestrator-events.jsonl` im Bereich überflogen. Spec und Plan der Quicknote sind ganz gelesen, dazu die Messung `messungen/260926-0828-zellen-rueckgaengig.txt`. `make check` läuft am Stand `171b380` grün, alle fünf Kommandos.

## Zusammenfassung

Die Quicknote ist sauber als dritte Fläche und sechste Form gebaut: kein Weg, den der Code nimmt, fasst das `Editormodell` an, jeder Wechsel läuft durch `flaeche_waehlen`, und `form()` stellt die Quicknote vor die Datei, sodass auch ein spät eintreffender Ladeausgang die Fläche nicht zurücktauscht. Die vier Termin-Behebungen sind richtig. Gefunden sind zwei Befunde: eine ungemessene AppKit-Annahme, an der die Unversehrtheit der Datei darunter hängt, und ein veralteter Doc-Kommentar.

## Summen

| Schwere | Anzahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 1 |

## Befunde

### M1: Ob `cmd+z` in einer leeren Quicknote die Datei darunter erreicht, ist ungemessen

`appkit/quicknote.rs`, `verwalter_fuer`, gibt der Fläche einen eigenen `NSUndoManager` über `undoManagerForTextView:`. Die Editorfläche darunter meldet am Verwalter des Fensters an. Für den leeren eigenen Stapel (direkt nach F10, nach „Kopieren“ mit `removeAllActions`, nach dem letzten Zurücknehmen einer Tippfolge) sagt kein Beleg, welchen Verwalter `NSWindow.undo:` nimmt. Die Messung vom 260926 (Durchgang `eigener`) zeigt für einen Feldeditor mit überschriebenem `undoManager`, dass AppKit trotzdem den Verwalter des Fensters nahm. Übertragbar ist das auf den Delegiertenweg nicht belegt; ausgeschlossen auch nicht (inference). Träfe es zu, nähme `cmd+z` in der Quicknote still eine Änderung der unsichtbaren Datei zurück, gegen A2 und gegen den Satz in `HowTo.md` „Ihr Stand, ihre Schreibmarke und ihr Rückgängig bleiben, wie sie waren“.

Der Plan führt das Risiko und schiebt es in den Abnahmelauf. Der Spike unter `spikes/zellen-rueckgaengig/` lief aber ohne Nutzer; dieselbe Frage lässt sich dort ohne KRK im Vordergrund messen. Datensatz: `260927-0225_*_ob-cmd-z-in-einer-leeren-quicknote-die-datei-darunter-erreicht-ist-ungemessen-und-ein-spike-koennte-es-messen.md`.

### L1: Der Kopf von `rueckgaengigstapel_leeren` sagt, den Delegiertenweg nehme niemand

`appkit/editor.rs`, Doc-Kommentar von `rueckgaengigstapel_leeren`: „Der Weg steht damit offen, wird aber nicht genommen: es gibt keinen zweiten Anmelder, und ein Verwalter mehr waere ein Mechanismus ohne Fall.“ Die Quicknote nimmt ihn seit `d3824d9`. Datensatz: `260927-0225_*_der-kopf-von-rueckgaengigstapel-leeren-sagt-den-weg-ueber-undomanagerfortextview-nehme-niemand-die-quicknote-nimmt-ihn.md`.

## Geprüft und ohne Befund

**Die Datei darunter.** Kein Quicknote-Weg ruft `Editormodell` schreibend. `quicknote_zeigen` übernimmt zuerst die Zelle (eine abgewiesene hält F10 an) und tauscht dann; `quicknote_verlassen` tauscht zurück. `form()` fragt die Rückkehr vor Ansicht und Typ, also lässt jeder spätere `flaeche_waehlen`-Rufer (`zurueckgehaltenes_uebernehmen`, `schliessen`, `ansicht_umschalten`) die Quicknote stehen. `cmd+s`, `opt+cmd+e`, `ctrl+cmd+e` und „PIN ändern“ sagen in `form_passt` für `Editorform::Quicknote` nein, gehalten von `IN_DER_QUICKNOTE` und `in_der_quicknote_wirkt_kein_befehl_der_datei`. `cmd+e` in der Quicknote geht über `quicknote_schliessen` und nicht über `editor_schliessen(true)`. F4 und jeder andere Ladeweg gehen über `datei_oeffnen`, das die Quicknote nach der Zellenübernahme verlässt; die vier Rufer von `editor_oeffnen_lassen` tragen `Befehl` oder `Sitzung`, ein automatisches Öffnen gibt es nicht. Beenden mit ungesichertem Stand geht über `nachfrage_zeigen`, das die Quicknote vor dem Blatt schließt. Das Schließen des Hauptfensters schließt sie über den Schließmelder nach dem Abbruch der Lesevorgänge. Ein ausgeblendeter Editorbereich verlässt sie in `nach_dem_sichtbarkeitswechsel` vor dem Fokusumzug; ein Ausblenden an `sichtbarkeit_aendern` vorbei gibt es nicht (`modell.borrow_mut()` in `anwendung.rs`: Start, Fensterwechsel, Spalte, Breite, Aktiv, keiner ändert die Sichtbarkeit des Randes).

**Geheimnisse und Persistenz.** Das Kopieren liest allein `quicknote_text()` und schreibt über `zwischenablage::text_schreiben`. Titel (`Editoranzeige::Quicknote`), Kopf (`QUICKNOTEKOPF`), Textmarke (`textmarke_verweigert` vor dem Modell), Teilen und Ordnersprung (`angezeigter_pfad` → `None`) sehen nicht hindurch; die übrigen Leser von `editor.pfad()` (Sitzung, Dateisystemwache, Ort des Notizordners) lesen die gehaltene Datei für interne Zwecke und geben sie nicht aus. `tasten_verdeckt` verdeckt weiter, solange die Datei darunter `secrets.txt` ist. `sitzung_bauen` rechnet an einer Kopie des Modells und nimmt den Puffer nicht; `sitzung.rs` nennt die Quicknote nicht.

**Fokus und Tasten.** `ist_eigene_textflaeche` fragt die Fläche über `isEqual` und baut sie nicht. Der `esc`-Rang steht nach Blatt und Zelle, vor Vorgang und Filtertext. `form_passt` verzweigt vollständig. Alle drei Kommandos stehen in `KENNUNGEN`, `wirkungsbereich`, `bereich_des_kommandos` und `zweigproben::BEFEHLE`. Keines ist in `immer_erreichbar` oder `waehrend_blatt_erlaubt`. Die Kopfzeile der Auslieferung (106 Funktionen, 107 Kombinationen) stimmt, von der Probe gehalten. `cmd+z`, `cmd+v`, `cmd+a` tragen `gehalten_von = "menue"` und erreichen die Fläche über das Menü.

**Geänderte Proben.** Jede der sieben hält ihre alte Zusage und fügt die Quicknote hinzu: `der_bereich_editor_fuehrt_genau_die_befehle_des_editors` erlaubt genau `quicknote_leeren` ohne Kombination und prüft das in beide Richtungen; `getauscht_wird_allein_…` zählt die Rufer weiter exakt; `sichern_schliessen_und_ansicht_wirken_in_jeder_form` überspringt allein die Quicknote, und die Gegenrichtung hält `in_der_quicknote_wirkt_kein_befehl_der_datei`; `pin_aendern_…` schließt allein die Quicknote aus; `beim_tausch_…` behält beide alten Paare; die Probe der sechs Tabellenbefehle setzt die Quicknote auf nein; `fenstertitel::tests` hält jeden Fokuswert außer `Editor` unverändert.

**Grenze, Automatiken, Untergrenze, `#[must_use]`.** Die Schranke `(UTF-16-Länge von Text und Einfügung) × 3` ist eine obere Schranke der UTF-8-Länge, die genaue Rechnung zieht den ersetzten Bereich ab. `automatiken_abschalten` steht an der Fläche und wird in `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` mitgemessen. Der Untergrenzen-Abschnitt nennt jeden hereingeholten Namen, jünger als 10.0 ist allein `buttonWithTitle:target:action:` (10.12). `#[must_use]` steht an `f10_wirkung`, `randrueckkehr`, `rand_zurueckstellen`, `quicknote_zeigen`, `quicknote_verlassen`, `quicknote_schliessen`, `Kopierausgang`, `aenderung_passt`.

**Termine.** `314e149`: gleicher getrimmter Vergleich auf beiden Seiten, bei Gleichheit geht das rohe Thema weiter; die Probe deckt `## xyz `, `## 261002 `, CRLF und den geänderten Text mit stehender Kopfzeile. `021fa8b`, `26b6ad1`, `aedad1a` erfüllen die Abnahmen ihrer Datensätze; „Fertig“, „Zuweisen“ und „Auslieferungszustand“ stehen in `belegungsansicht.rs`.

**Dokumente.** `HowTo.md`, `README.md` und `CLAUDE.md` nennen Tasten, Menünamen und Statuszeilensätze wortgleich mit Code und Belegung. Neue Zahlwörter in `CLAUDE.md` („Zwei Flächen“, „die ersten zwei“) stehen neben ihrer Aufzählung.

**Werkbank.** Arbeitspaket und Plan der Termine geschlossen und stimmig, der Entscheid auf `_i_` mit Commits. Der Spec der Quicknote steht auf `_o_`, passend zum Arbeitspaket auf `claimed`.

## Querbeobachtungen

- M1 und L1 hängen an derselben Stelle: das Projekt hat zwei Messungen zum Rückgängigverwalter, und die Quicknote stützt sich auf die ältere, deren Fall (leerer Stapel) sie nicht abdeckt. Der Doc-Kommentar, der das hätte sagen können, wurde nicht nachgezogen.
- „Quicknote“ steht als Anzeigetext zweimal (`QUICKNOTEKOPF` in `editor.rs`, `String::from("Quicknote")` in `fenstertitel.rs`). Kein Defekt, als Notiz.

## Nicht nachgeprüft

- Ob F10 und `shift+f10` KRK auf der Tastatur des Nutzers erreichen (zweite Haltestelle des Spec).
- Ob AppKit bei einer Mehrfachauswahl `textView:shouldChangeTextInRange:replacementString:` für jeden Bereich ruft oder nur für einen; die Grenze könnte dort mehrfach zu wenig zählen (speculation).
- Ob `makeFirstResponder:` im Klickweg der Schaltflächen je abgelehnt wird; dann wirkte „Schließen“ als „Fokus hinein“.

## Reihenfolge

Nichts blockiert eine Auslieferung zwingend. M1 sollte vor einer Auslieferung gemessen sein, weil es die eine Zusage betrifft, die der Spec als Haltestelle führt, und weil die Messung ohne den Nutzer möglich ist. L1 ist Aufräumarbeit und fällt mit M1 in einem Gang an.

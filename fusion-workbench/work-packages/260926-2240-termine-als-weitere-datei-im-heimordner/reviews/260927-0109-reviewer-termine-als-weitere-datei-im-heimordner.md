# Durchsicht: appointments.md als vierte Eintragsdatei

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `cf458eb..f6981bf`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `260926-2240-termine-als-weitere-datei-im-heimordner.md`, `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`, `260926-2246-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`, `260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`
**Domain:** code
**Work-item:** `260926-2240-termine-als-weitere-datei-im-heimordner`

Die nicht geöffneten Dateien sind Werkbankdatensätze aus `e6fdf7e` und liegen außerhalb des Prüfgegenstands; Spec und Entscheid der Termine sind gelesen. Jede Code- und Dokumentdatei des Bereichs ist geöffnet, die zwei Probendateien `crates/krk-core/tests/belegung.rs` und `crates/krk-core/tests/heimordner.rs` über ihren Diff.

## Zusammenfassung

Die Naht zwischen Stelle und Zeile ist sauber gezogen: jede Zeilennummer, die AppKit in `eintragsansicht.rs` hereingibt oder bekommt, geht über `Zeilen::stelle_der_zeile` / `zeile_der_stelle`, und Editorbereich und Kern sehen nur Stellen. Die Konfliktregel über `Wirkungsbereich::seite` ist disjunkt und vollständig und wird von einer Probe über jedes Kommandopaar und jede `Lage` gegen die echte Zulässigkeit gehalten. `make check` läuft am Stand `f6981bf` grün, alle fünf Kommandos. Gefunden sind drei Defekte, keiner blockiert eine Auslieferung.

## Summen

| Schwere | Anzahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 2 |

## Befunde

### M1: Eine unverändert verlassene Datumszelle gilt bei Leerraum am Ende der Kopfzeile als geändert

`crates/krk-core/src/heimordner/eintraege.rs`, `termine::aendern`:

```rust
let datum = datum.trim();
let Some(alt) = Notizen::lesen(stand).notiz(stelle) else { … };
if datum != alt.thema && termindatum(datum).is_none() {
    return Err(Abweisung::UngueltigesDatum);
}
```

`alt.thema` ist roh (allein `## ` und `\n` abgenommen), die Eingabe ist getrimmt. Die Zelle zeigt `Terminzeile::datum`, also dasselbe rohe Thema, und `zelle_darf_enden` / `zelle_geendet` reichen bei **jedem** Ende den Zellentext weiter (`eintragsansicht.rs`, dann `zelle_pruefen` / `zelle_festschreiben` → `zellenrechnung` in `editor.rs`). Folgen bei `## 261002 ` oder einer CRLF-Datei: stilles Umschreiben der Kopfzeile, Datei geändert, Handlung auf dem Rückgängigstapel. Bei `## xyz `: die unveränderte Zelle wird abgewiesen, lässt sich nur mit `esc` verlassen, und `zelle_uebernehmen` hält Sichern und Schließen an, solange sie offen steht. Das widerspricht dem Doc-Kommentar der Funktion und `HowTo.md`. Die Notiztabelle trimmt nicht und ist nicht betroffen. Betrifft allein von Hand bearbeitete Dateien; kein Datenverlust.

Vorschlag: den Vergleich auf dieselbe Form bringen, etwa `datum == alt.thema.trim()` als „unverändert“ werten und dann `Ok(None)` liefern, bevor geprüft wird. Datensatz: `260927-0109_*_termine-aendern-vergleicht-das-getrimmte-datum-mit-dem-ungetrimmten-thema.md`.

### L1: „Drei Funktionen auf einer Kombination sind immer ein Konflikt“ stimmt über Zusteller hinweg nicht

`HowTo.md` (Abschnitt „Die Tastaturbelegung“) und `CLAUDE.md` (Absatz zu `Nachschlag::Geteilt`) führen den Satz ohne Einschränkung. `begegnen` (`belegung.rs`) gibt für verschiedene Zusteller `false`; `cmd+f` auf `filter_einfuegen` (Menü), `editor_suchen` (Editor) und zusätzlich `sortierung_name` (Dateifenster) lädt ohne Konflikt. Richtig ist der Satz für Funktionen desselben Zustellers, so wie der Doc-Kommentar von `Belegung::nachschlag` und die Probe `drei_funktionen_auf_einer_kombination_sind_ein_konflikt` ihn fassen. Datensatz: `260927-0109_*_howto-und-claude-md-sagen-drei-funktionen-auf-einer-kombination-seien-immer-ein-konflikt.md`.

### L2: Der Doc-Kommentar von `editorbefehl` steht über `klickmelder`

`crates/krk-ui/src/appkit/anwendung.rs`: `klickmelder` und `termine_richtung_umkehren` (beide aus `d350c29`) stehen zwischen dem Doc-Kommentar von `editorbefehl` und dessen `fn`. `klickmelder` trägt dadurch zwei verschmolzene Kommentare, `editorbefehl` keinen. Der verschobene Absatz nennt außerdem „die vier Befehle“, und `editorbefehl` hat mehr Rufer. Datensatz: `260927-0109_*_der-doc-kommentar-von-editorbefehl-steht-seit-d350c29-ueber-klickmelder.md`.

## Geprüft und ohne Befund

**Stelle und Zeile** (`eintragsansicht.rs`, `editor.rs`). Hereinkommend über `stelle_an`: `selectedRow` (`gewaehlte_stelle`), `clickedRow` (Doppelklick), `rowForView:` (`kasten_geklickt`, `zelle_von`). Hinausgehend über `zeile_von`: `selectRowIndexes:` (`auswahl_setzen`), `editColumn:row:` (`zelle_beginnen` über `ziel_der_zelle`), `tab` (`naechste_zelle` über beide Umrechnungen), `copy:` (`gewaehlter_text`), `abgeleiteter_text`. Die Delegiertenmethoden `tableView:viewForTableColumn:row:` und `tableView:didAddRowView:forRow:` lesen mit AppKits Zeile an Zeilen in Schirmfolge, und das ist richtig. `zeilen_zeigen` erhebt `vorher` als Stelle, **bevor** es die Zeilen ersetzt. Hinzufügen, Bearbeiten, Löschen, Abhaken und `terminrichtung_umkehren` gehen alle über `gewaehlte_stelle`; Rückgängig und Wiederholen laufen über den Stand und `tabelle_nachziehen`, ohne Zeilennummer. Die Proben `jede_zeilennummer_von_appkit_geht_ueber_die_umrechnung` und `die_termintabelle_loescht_aendert_und_legt_an_der_stelle_der_zeile_an` decken die heutigen Übergänge; einen künftigen Übergang über `editedRow` oder `rowAtPoint:` sähe die erste nicht, und ihr Doc-Kommentar sagt das.

**Konfliktregel.** `Wirkungsbereich::seite` verzweigt vollständig und ohne Auffangzweig; `schliesst_aus` ist symmetrisch; `begegnen` ist die eine Fassung für `konflikte` und `zuweisen`, und eine Funktion ohne Kommando begegnet jeder desselben Zustellers. `einander_ausschliessende_bereiche_sind_nie_zugleich_zulaessig` läuft über jedes Paar aus `Kommando::KENNUNGEN` und jede `Lage` aus `jede_lage()`, die alle sechs Felder von `Lage` aufzählt, Blatt, Ersthelfer, Schlüsselfenster und `pin_aenderbar` eingeschlossen; `immer_erreichbar` und `waehrend_blatt_erlaubt` sind damit mitgeprüft. `nachschlag` überspringt zugestellte Funktionen, also treffen dort allein Funktionen des Abgriffs zusammen, und mehr als zwei kann `bauen` dort nicht durchlassen. `waehlen` nimmt die zweite allein dann, wenn allein sie zulässig ist; die Lage wird in `eingabe_ausfuehren` einmal erhoben und an `kommando_ausfuehren_bei` weitergereicht. Überlappungen innerhalb einer Seite (`sortierung_groesse` auf `cmd+1`, `fenster_schliessen` auf `cmd+1`, `eintrag_loeschen` neben `editor_sichern`) bleiben Konflikte, beim Einlesen wie bei `zuweisen` (`crates/krk-core/tests/belegung.rs`).

**Datei.** `termindatum` ist streng: genau 6 oder 12 Bytes, `bytes[6] == b' '`, `bytes[9] == b':'`, Schaltjahr nach der vollen gregorianischen Regel, Jahre 2000 bis 2099 über `Tag::neu`. Die Probentafel deckt jeden Fall des Spec T2. Byte-Treue bei unverändertem Stand, Vorspann, fehlendem Schlussumbruch und ungültiger Kopfzeile: `Zerlegung` mit `split_inclusive`, `in_reihenfolge` und `dateiende_angleichen` halten sie, und `termine::hinzufuegen` geht über `notizen::angehaengt`. Das Umkehren der Richtung fasst den Stand nicht an. Die Vorschau ordnet im Speicher um (`vorschautext`), schreibt nichts, und eine `appointments.md` anderswo bleibt Markdown in Dateireihenfolge.

**Schutz von `secrets.txt` und Erkennung.** `Heimordner::ist`, `sonderdatei` und `sonderdatei_genau` sind unverändert; der vierte Wert geht allein über `Sonderdatei::ALLE` in `sonderdatei`, ohne Systemaufruf. `ohne_inhaltsauftrag` nennt weiter allein `secrets.txt`. `oeffnungsweg` lädt `appointments.md` wie `notes.txt`. `form_passt` verzweigt über `Reihenfolge` und `Termine` vollständig; `KENNUNGEN`, `bereich_des_kommandos` und der Ausführungszweig (`zweigproben`) tragen `TermineRichtungUmkehren`; `die_zellenuebernahme_hat_genau_diese_rufer` führt `terminrichtung_umkehren`; die Zellen der Termine sind `Notizfeld`er und rufen damit die Textautomatiken ab. Der Untergrenzen-Abschnitt von `eintragsansicht.rs` nennt `NSTableRowView`, `NSColor`, `systemYellowColor`, `NSImage` und die zwei neuen Delegiertenmethoden, alle unter macOS 15.

**Heute und Melder.** `termine::heute` ist die eine Funktion, die die Uhr liest (`localtime_r`, Ortszeit), gerufen aus `tabelle_nachziehen` und beim Hinzufügen. `heute_nachziehen` hängt als dritter Empfänger hinter `fokusanzeige_nachziehen`, tut ohne Termintabelle und bei laufender Zelle nichts und ruft weder `anwenden` noch `setHidden` noch `makeFirstResponder:`. Geprüft, dass keiner der Wege mit `modell.borrow_mut()` im Editor während seiner Ausleihe den Ersthelfer wechselt; `titel_nachziehen` las den Editor schon vorher an derselben Stelle.

**Dokumente.** `HowTo.md` und `README.md` nennen Kopfzeilenform, Tasten, `cmd+1`, `cmd+t` in F1 und die Hervorhebung passend zum Code; die neuen Zahlwörter in `CLAUDE.md` („zwei weitere Werte“, „drei Empfänger“) stehen neben ihrer Aufzählung. Die Zeile „Ausgeliefert sind 103 Funktionen mit zusammen 105 Kombinationen“ in `default-keymap.toml` stimmt und wird von einer Probe gehalten.

## Querbeobachtungen

- Die zwei Dokumentfehler (L1, L2) sind von derselben Art wie die älteren Zahl-Befunde: ein Satz, der eine Regel verkürzt wiedergibt, und keine Probe hält Prosa. L1 steht an zwei Stellen gleichlautend, also ist der Satz einmal geschrieben und kopiert worden.
- Über Zusteller hinweg verdeckt eine Funktion des Abgriffs eine vom Menü zugestellte still, sobald ihre Zulässigkeit sich überschneidet (Beispiel in L1). Das ist älter als diese Arbeit; die neue Regel macht es nur mit einer zusätzlichen Zuweisung statt einer Umbelegung erreichbar. Nicht als Defekt geführt, im Datensatz zu L1 vermerkt.

## Nicht nachgeprüft

- Dass AppKit bei zwei gleichen Tastenentsprechungen dem späteren Eintrag das Zeichen nimmt, stützt sich auf die Messung vom 260813; am laufenden Bündel nicht wiederholt.
- Ob `imageNamed:` für `NSAscendingSortIndicator` und `NSDescendingSortIndicator` ein Bild liefert: fehlt es, zeigt der Kopf keinen Pfeil, sonst geschieht nichts.
- Lesbarkeit von `systemYellowColor` mit Deckung 0,25 in beiden Erscheinungsbildern und das Zeichnen der Auswahl über der Färbung: Nutzerabnahme nach T6.

## Reihenfolge

Nichts davon blockiert eine Auslieferung. M1 zuerst, weil es an der Byte-Treue der Datei hängt; L1 und L2 sind Aufräumarbeit und lassen sich in einem Gang erledigen.

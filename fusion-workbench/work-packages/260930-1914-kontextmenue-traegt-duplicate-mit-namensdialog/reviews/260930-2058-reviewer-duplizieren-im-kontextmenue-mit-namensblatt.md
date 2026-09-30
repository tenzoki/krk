# Durchsicht: „Duplizieren…“ im Kontextmenü mit Namensblatt

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `17750c7..10d2128`
**Not-opened:** none
**Domain:** code
**Work-item:** 260930-1914-kontextmenue-traegt-duplicate-mit-namensdialog
**Cross-references:** 260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md, 260930-2056_*_zwei-prosastellen-nennen-proben-in-namenseingabe-rs-die-den-rueckruf-des-namensblatts-hielten-und-dort-steht-keine.md, 260930-2057_*_der-doc-kommentar-von-kontextmelder-setzen-zaehlt-drei-menueeintraege-und-kontextbefehl-alle-traegt-sechs.md

## Zusammenfassung

Der Weg vom Rechtsklick über das Namensblatt, den Arbeitsfaden, die Namensnachfrage und den Abschluss ist am Quelltext nachvollzogen und tut, was der Plan sagt: kein Weg des Duplizierens ersetzt oder entfernt einen vorhandenen Eintrag, der Arbeitsfaden bekommt auf jedem Ausgang des Blattes eine Antwort, der Hauptfaden stellt keinen Dateisystemaufruf, und Kopieren und Verschieben nehmen nach dem Umbau dieselben Systemaufrufe in derselben Folge. `make check` endet auf `10d2128` grün (fünf Kommandos, alle Proben bestanden). Zwei Befunde, beide an Prosa: eine Probenbehauptung, die auf Proben zeigt, die es nicht gibt (Medium), und ein liegengebliebenes Zahlwort über die eigenen Einträge des Kontextmenüs (Low).

## Zahlen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 1 |

## Was geprüft wurde, und was dabei herauskam

Die acht Punkte des Auftrags, je mit dem, was gelesen wurde.

### 1. Kann eine vorhandene Datei überschrieben oder entfernt werden? Nein.

- `sys::datei_kopieren` setzt `COPYFILE_EXCL` außerhalb der Wahl der Übertragungsart (`crates/krk-core/src/verzeichnis/sys.rs`, `mit_zustand_kopieren`: `let kennzeichen = COPYFILE_EXCL | match art { … }`). Ein vorhandenes Ziel scheitert vor dem ersten Byte mit `EEXIST`, das als `io::ErrorKind::AlreadyExists` ankommt.
- `crates/krk-core/src/operation/duplizieren.rs:135-173` (`eintrag_duplizieren`) ruft weder `ziel_klaeren` noch `baum_entfernen` noch `in_den_papierkorb`; die Schleife hat je Versuch genau drei Ausgänge (angelegt, `AlreadyExists` → `namen_erfragen`, sonst überspringen). Die Probe `das_duplizieren_kennt_keinen_weg_der_einen_eintrag_entfernt` (`crates/krk-core/tests/baum.rs`) hält das an den Codezeilen und hält ihre Nadeln über die Gegenprobe am Leben.
- Das Wegräumen der halben Zieldatei in `kopieren::datei_uebertragen` (`crates/krk-core/src/operation/kopieren.rs:120-164`) steht allein hinter `kopie.abgebrochen`, und dorthin kommt der Lauf nur, wenn `copyfile(3)` selbst mit `ECANCELED` zurückkam; ein `EEXIST` geht vorher über `?` an den Rufer, `remove_file(ziel)` wird nicht erreicht. Das Ziel ist also immer die Datei, die derselbe Aufruf ausschließend angelegt hat, nie die Quelle und nie eine fremde. Für den verwaisten Verweis am Zielnamen, der das Gegenteil bräuchte (`copyfile` folgte dem Namen, `remove_file` nähme den Verweis), hält `ein_ordner_und_ein_verwaister_verweis_am_namen_sind_vergebene_namen` (`crates/krk-core/tests/operation.rs`) in beiden Übertragungsarten, dass `EEXIST` kommt und hinter dem Verweis nichts entsteht.
- `Steuerung::namen_erfragen` (`crates/krk-core/src/operation/fortschritt.rs:429-436`) liest die Konfliktregel nicht und übersetzt allein `UmbenennenIn` in `Some`; `keine_konfliktregel_laesst_das_duplizieren_ersetzen_oder_von_selbst_umbenennen` fährt alle fünf Regeln gegen vier Antworten (darunter den fallengelassenen Kanal) und hält die belegte Datei bytegleich.

### 2. Hängt der Arbeitsfaden? Nein, und kein Blatt geht aus einem Abschlussblock auf.

- `geprueft_zeigen` (`crates/krk-ui/src/appkit/blaetter/namenseingabe.rs:274-333`) ruft `fertig(name)` unbedingt am Ende des Rückrufs von `blatt.zeigen`; `Blatt::zeigen_mit_wahl` (`blaetter/mod.rs:1052-1121`) ruft den Rückruf aus dem einen Abschlussblock von AppKit, und eine Antwort zu keiner Schaltfläche fällt auf die Abbruchstelle. `frei_zeigen` übersetzt `None` in „Rückruf läuft nicht“ und lässt Anlegen und Lesezeichenblätter unverändert.
- `duplikatname_nachfragen` (`crates/krk-ui/src/appkit/anwendung.rs:8494-8531`) schickt auf `Some` `UmbenennenIn`, auf `None` `Abbrechen`, und ruft danach `blatt_geschlossen`. Kehrt die Funktion ohne Fenster zurück, fällt `frage` und mit ihr der Antwortkanal, und `nachfragen` (`fortschritt.rs:438-466`) liest das als `Abbrechen`.
- Der Rückruf des ersten Blattes (`duplikat_erfragen`, `anwendung.rs:8366-8428`) stellt den Auftrag und ruft `blatt_geschlossen`; er öffnet kein Blatt. Die Namensnachfrage kommt über `vorgang_zeichnen` (`anwendung.rs:8730+`), das Konflikt und Bericht liegen lässt, solange `attachedSheet` antwortet, und einen Durchgang nach `blatt_geschlossen` abholt. `vorgang_beenden` wird allein aus `vorgang_zeichnen` gerufen. Die Ausleihe von `ivars().vorgang` in `duplikatnamen_merken` (`anwendung.rs:8542-8550`) trifft keine gehaltene: `vorgang_zeichnen` kopiert vor jedem AppKit-Aufruf heraus, `abbrechen` beendet die Ausleihe des Schlitzes vor `blatt.abbrechen()`.

### 3. Der Umbau von `kopieren::datei` zu `datei_uebertragen`: unverändert.

Vorher: `Ok(kopie) if abgebrochen` → `teilstueck`, `remove_file`, `Abgebrochen`; `Ok(kopie)` → `eintrag_fertig` mit Klon-Sonderfall, `Weiter`; `Err` → `ueberspringen(pfad, grund)`, `Weiter`. Nachher: dieselben drei Zweige, der dritte in `datei` (`kopieren.rs:85-98`) mit `quelle.pfad`, was `pfad` war. `kopieren_nach` ruft weiter `datei`; `verschieben.rs:128` ruft weiter `kopieren_nach`. Jede vorhandene Probe des Kopierens, Verschiebens, Packens und Entpackens besteht mit unveränderter Erwartung (`make check`).

### 4. Kein Dateisystemaufruf auf dem Hauptfaden. Bestätigt.

`duplikatbezug` (`crates/krk-ui/src/kommandos/kontextmenue.rs:643-666`) liest `Ordnermodell::zeilen` und `Eintrag::typ`; `DateifensterQuelle::duplikatbefund` (`tabelle.rs:2307-2312`) fragt `betroffene` und `duplikatbezug` in einer Ausleihe; `namensgrund` liest den Text. Die Suche über `zeilen()` nach dem Namen ist eindeutig, weil die Liste immer der flache Ordner ist (die tiefe Suche entscheidet nur, welche Ordnerzeile stehen bleibt, und fügt keine Zeilen aus Unterordnern hinzu; `Eintrag::name` ist „der Name ohne Pfad“).

### 5. Fallunterscheidungen und Zulässigkeit N1. Vollständig.

`Kontextbefehl` (`titel`, `menuemarke`, `kontextbefehl_ausfuehren`), `Duplikatbefund` (in `duplikat_erfragen`), `Konfliktform` (in `konflikt_fragen`, `anwendung.rs:8840-8843`), `Art` an allen Stellen (`neuer_name`, `entpackziel`, `zielordner`, `ausfuehren`, `einen_abarbeiten`, `ueberschrift`, `erzeugt_genau_ein_ziel`, `ersetzungsweg`, `konfliktform`, `schiebt_auffrischung_auf`, `Vorgang::ordner`, `vorgang_beenden`) verzweigen ohne Auffangzweig. `Kontextbefehl::ALLE` und die Marken folgen der Aufzählung (Tafel in `kontextmenue.rs`, `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`). N1: Ordner und Verknüpfung bekommen ihren Satz statt des Blattes; Mehrfachmarkierung ist `Mehrere`, auch wenn jeder Eintrag eine Datei ist; der Rechtsklick auf eine unmarkierte Zeile rückt die Auswahl über `rechtsklick_zielzeile` vor `betroffene`, wie bei jedem Eintrag des Menüs. Röhre, Socket und Gerätedatei, die die Liste als `Typ::Datei` führt, weist `symlink_metadata(…).file_type().is_file()` im Kern ab; `was_keine_gewoehnliche_datei_ist_wird_nicht_dupliziert` fährt Ordner, Verknüpfung, verwaiste Verknüpfung und Röhre mit Frist.

### 6. Umlautregel und Untergrenzen-Abschnitt. Eingehalten.

Nutzersichtbar mit Umlauten: „Duplizieren…“, „Wie soll das Duplikat heißen?“, „keine gewöhnliche Datei“, die vier Statuszeilensätze (`operationen.rs`), gehalten von `jeder_satz_des_duplizierens_traegt_seinen_wortlaut_mit_umlauten`. Bezeichner und Kommentare in Umschrift. Der Abschnitt in `namenseingabe.rs` nennt jeden hereingeholten Namen (`NSTextField`, `NSView`, `NSWindow`, `NSString`, `NSPoint`, `NSRect`, `NSSize`, `MainThreadMarker`) und `labelWithString:` mit 10.12 als einzige Berührung über 10.0.

### 7. `HowTo.md` und `CLAUDE.md` gegen den Code. Ein Befund.

Die Zitate in `HowTo.md` (Menüeintrag, Frage, Schaltflächen, die vier Statuszeilensätze, die drei Gründe aus `Namensfehler::grund`, „es gibt schon einen Eintrag namens „<name>““, „keine gewöhnliche Datei“) sind zeichengleich mit `Kontextbefehl::titel`, den Textfunktionen in `operationen.rs` und `Namensfehler::grund` (`umbenennen.rs:53-60`). Die Stellung „zwischen „Unzip“ und „Im Finder öffnen““ stimmt mit `ALLE`. Kein neuer Satz in `CLAUDE.md` trägt eine Zahl über eine gewachsene Aufzählung; jeder genannte Name steht im Baum. **Befund:** `CLAUDE.md:149` behauptet, „die Proben in `namenseingabe.rs`“ hielten, dass die Antwort auf jedem Weg des Rückrufs gesendet wird; solche Proben gibt es nicht (Medium, unten). Daneben ein Zahlwort in `tabelle.rs`, das der Schritt 3 laut Plan hätte streichen sollen (Low, unten).

### 8. Halten die Proben, was sie behaupten? Bis auf eine Behauptung.

Die Kernproben in `tests/operation.rs` prüfen jeden Ausgang am Dateisystem (frei, vergeben, eigener Name, zweimal vergeben, jede Regel gegen jede Antwort, unbeantwortet, keine gewöhnliche Datei, kein Name, beide Übertragungsarten, Ordner und verwaister Verweis am Namen, zweite Quelle von Hand). Die Quelltextproben in `anwendung.rs` sagen in ihren Doc-Kommentaren, was sie nicht sehen (Zweige, die der Rumpf nie erreicht; tiefer gerufene Funktionen). `der_duplikatweg_reisst_nirgends_ab` hält die Paarung `Konfliktform::Namensnachfrage` / `duplikatname_nachfragen` auf einer Zeile. Die eine Ausnahme ist der Befund unten.

## Befunde

### Medium: zwei Prosastellen nennen Proben, die es nicht gibt

`crates/krk-ui/src/appkit/anwendung.rs:13306-13307` und `CLAUDE.md:149` sagen, die Proben in `namenseingabe.rs` hielten, dass der Bauer `fertig` auf jedem Weg genau einmal ruft. Das Prüfmodul dort (`namenseingabe.rs:402-467`) trägt drei Proben, und alle drei prüfen allein `blattstand`. Die Zusage ruht auf dem Abschlussblock von `Blatt::zeigen_mit_wahl` und dem unbedingten `fertig(name)` in `namenseingabe.rs:328`, und keine Probe sieht das. Der Code ist richtig; wer ihn umbaut, wird von keiner Probe angehalten, obwohl zwei Stellen das Gegenteil sagen. Fix: beide Sätze nennen, was die Zusage trägt, oder eine Quelltextprobe kommt hinzu und wird genannt. Datensatz: `260930-2056_*_zwei-prosastellen-nennen-proben-in-namenseingabe-rs-die-den-rueckruf-des-namensblatts-hielten-und-dort-steht-keine.md`.

### Low: „die drei Menueeintraege“ in `tabelle.rs:1554`

Doc-Kommentar von `kontextmelder_setzen`, Zahl aus der Runde 17, heute sechs. Schritt 3 des Plans verlangt, dass `tabelle.rs` jede Zahl über die eigenen Einträge verliert; zwei Stellen (`tabelle.rs:2091`, `tabelle.rs:5655`) sind nachgezogen, diese nicht. Fix: „die eigenen Menueeintraege“. Datensatz: `260930-2057_*_der-doc-kommentar-von-kontextmelder-setzen-zaehlt-drei-menueeintraege-und-kontextbefehl-alle-traegt-sechs.md`.

## Querschnitt

- Beide Befunde sind dieselbe Art Drift, die `CLAUDE.md` unter „Projektstand“ für Zahlwörter beschreibt, hier einmal als Zahl und einmal als Verweis auf eine Probe. Die Probe `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` hält Listen, keine Prosa; eine Behauptung „Probe X hält Y“ hält nichts.
- Was am laufenden Bündel bleibt (Anordnung der Zeile unter dem Feld, Zustand der Schaltfläche, erneutes Aufgehen, Auswahl der Vorgabe nach `addSubview:`), kann diese Durchsicht nicht sehen; die neun Punkte unter `## Testing Strategy` des Plans decken sie, und sie sind Nutzerarbeit.

## Reihenfolge

Kein Auslieferungsblocker. Beide Befunde sind Prosa, der Medium-Befund zuerst, weil er eine Zusage über Proben trägt, auf die sich der nächste Umbau des Blattes verlassen würde.

## Prüfung

`make check` auf `10d2128`, Ausgabe „alle fuenf gruen“, Exit 0 (Bau, Proben, `clippy -D warnings`, `fmt --check`, `cargo doc -D warnings`). Arbeitsbaum nach der Durchsicht unverändert bis auf `fusion-workbench/orchestrator-events.jsonl`, das schon vorher geändert war.

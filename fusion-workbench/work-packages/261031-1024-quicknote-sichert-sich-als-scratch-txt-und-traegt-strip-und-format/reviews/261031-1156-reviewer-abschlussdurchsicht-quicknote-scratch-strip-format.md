# Abschlussdurchsicht: Quicknote sichert sich als `scratch.txt` und trägt Strip und Format

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `135960bf..be1a725d`
**Not-opened:** `spikes/quicknote-textdidchange/mit_fenster.swift`, `spikes/quicknote-textdidchange/ohne_schluesselfenster.swift`, `fusion-workbench/orchestrator-events.jsonl`
**Domain:** code und Daten (`resources/default-keymap.toml`). Ontologie trägt das Paket keine.
**Verdict:** revise
**Cross-references:** `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`, `261031-1024-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md`

## Zusammenfassung

Die Sicherung ist sorgfältig gebaut. Ein Faden, Lesen als erster Auftrag, kein Schreiben vor dem Abholen der Antwort, Stempelsperre, exklusives Anlegen mit `0600`, kein Ordner, kein Systemaufruf auf dem Hauptfaden: das steht so im Code, und die Proben an echten Dateien halten es. Strip und Format reproduzieren das Beispiel des Nutzers Zeichen für Zeichen. Ihre eine Umbruchregel verbindet aber jede längste Zeile mit ihrer Folgezeile, und daraus folgen eine Bedeutungsänderung in Strip und zerstörte Tabellen in Format. Dazu verliert das Beenden nach einem fehlenden Notizordner oder einem gescheiterten Schreiben den letzten Stand.

## Zählung

| Schwere | Anzahl | Befunde |
|---|---|---|
| Critical | 0 | |
| High | 1 | A1 |
| Medium | 2 | B1, C1 |
| Low | 6 | A2, A3, A4, B2, B3, C2 |

Erhoben aus den Befunden darunter, gezählt nach ihren Kennungen.

## A. Strip und Format (`crates/krk-core/src/text/glaetten.rs`)

### A1 (High): Die längste Zeile gilt immer als umgebrochen

`umgebrochen` verlangt `laenge(zeile) + 1 + laenge(wort) > breite - breite / 10`, wobei `breite` die längste Zeile ist. Für die längste Zeile gilt das bei jedem Folgewort. Das zweite Signal, die Einrückung der Folgezeile, ist bei Chat-Text immer erfüllt: der Chat rückt jede Zeile nach der ersten ein, wie der Brief selbst festhält. Bei Chat-Text bleibt also allein die Länge übrig, und die längste Zeile erfüllt sie immer.

Gemessen am Stand `be1a725d`, `glaetten.rs` unverändert per `#[path]` in ein Wegwerfprogramm eingebunden:

- Strip: Ein Kommentar über einem Befehl verschluckt den Befehl (`# … bauen cargo build`). Die Shell führt `cargo build` danach nicht mehr aus. Damit ist die Zusage aus E11 gebrochen, dass kein Inhalt verloren geht, der für die Shell etwas bedeutet.
- Strip: Zwei oder drei Befehle untereinander werden verbunden, sobald einer davon die längste Zeile ist und nicht der letzte (`cargo test … appkit:: cargo clippy --workspace`).
- Format: Eine Tabelle in Kastenzeichen und eine Baumskizze (`├──`) werden zu je einer Zeile, weil jede ihrer Zeilen gleich lang und damit die längste ist. Ohne Auswahl wirkt Format auf den ganzen Text.

Plan (Zeile „Was nicht entscheidbar ist“ in E11), Modulkopf und `HowTo.md` führen das als seltenen Grenzfall: „deren erster fast die volle Breite einnimmt“. Es ist der Regelfall jeder Eingabe, deren längste Zeile nicht am Ende steht. Die Proben sehen es nicht: `strip_laesst_kurze_befehle_untereinander_stehen` setzt die längste Zeile ans Ende.

Die **Decidability**-Zeile des Plans nennt zwei Signale. Für die Eingabe, für die die Funktion gebaut ist, ist es eins. Eine Nachbesserung an der Schwelle hilft deshalb nicht. Nötig ist ein Signal, das bei Chat-Text tatsächlich unterscheidet, oder die Frage geht an den Planer zurück. Issue: `261031-1156-die-laengste-zeile-gilt-immer-als-umgebrochen-und-strip-verschluckt-befehle-format-tabellen.md`.

### A2 (Low): Nachlaufender Leerraum fällt auch in Heredoc-Rumpf und Anführung

`zerlegen` kürzt jede Zeile mit `trim_end()`, bevor der Automat die Lage kennt. Die zwei Leerzeichen eines harten Umbruchs in einem `cat <<EOF > a.md` fallen dadurch weg (gemessen). Issue: `261031-1156-strip-kuerzt-nachlaufenden-leerraum-im-heredoc-rumpf-und-liest-arithmetisches-als-heredoc.md`.

### A3 (Low): `<<` in `$(( … ))` öffnet einen Heredoc

`zeile_lesen` und `heredoc_lesen` lesen in `echo $((1 << 4))` die `4` als Endmarke. Danach gilt jede weitere Zeile als Rumpf, und nichts wird mehr verbunden (gemessen). Der Fehler liegt auf der sicheren Seite. Steht im selben Issue wie A2.

### A4 (Low, kein Issue): Fälle, die bleiben und benannt sind oder benannt werden sollten

- Ein Umbruch des Chats mitten in einer Anführung (`git commit -m "… Strip und\n  Format"`) bleibt nach Strip ein Zeilenumbruch in der Nachricht, und die Einrückung davor fällt. `HowTo.md` nennt die Regel („Was in Anführungszeichen … steht, bleibt mit seinen Umbrüchen“). Für das Ziel 2 des Briefs ist das ein häufiger Fall, in dem Strip den Befehl nicht so wiederherstellt, wie er gemeint war. Zur Kenntnis, kein Defekt gegen den Plan.
- `foo\` + eine einzelne eingerückte Folgezeile: Die Kopiereinrückung ist die kleinste Einrückung ab der zweiten Zeile, bei einer einzigen Folgezeile also ihre ganze. Aus `echo abc\` + `     def` wird `echo abcdef`, die Shell hätte am Original `abc def` gelesen, wenn die Einrückung gemeint war. Aus dem Text nicht entscheidbar, zur Kenntnis.
- inference: Claude Code zeigt Codeblöcke im Terminal ohne die drei Backticks. Wenn das stimmt, greift die Zaunregel aus E10 bei Text, der aus dem Chat kopiert ist, nie, und Code in Format läuft unter A1. Nicht geprüft.

Die übrigen Regeln aus E9 bis E11 stehen so im Code, wie der Plan sie beschreibt: CRLF, Kopiereinrückung, `'`/`"`/`$'`, Kommentar am Wortanfang, `<<-`, mehrere Heredocs je Zeile, `<<<`, die Fortsetzung mit und ohne Leerraum, das Ende ohne `\n`, die Ausgabeeinrückung von Punkten. `cargo test -p krk-core --test glaetten --test heimordner`: 132 bestanden, 0 fehlgeschlagen.

## B. Die Sicherung (`crates/krk-ui/src/quicknotesicherung.rs`, `appkit/editor.rs`, `appkit/quicknote.rs`, `appkit/anwendung.rs`)

Zu den Fragen der Durchsicht:

- **Kann ein Schreiben vor dem Einlesen `scratch.txt` leeren?** Nein. `Sicherung::schreiben` weist ab, solange `stand()` `Ungelesen` ist, und der Stand wechselt erst in `einziehen`. `quicknote_bauen` schaltet die Fläche nicht bearbeitbar, und frei wird sie in `quicknote_sicherung_einziehen` erst nach `einsetzen_ohne_rueckgaengig`. Dass eine nicht bearbeitbare `NSTextView` auch `ersetzen` abweist und `setString:` kein `textDidChange:` auslöst, ist in `messungen/261031-1300-quicknote-textdidchange.txt` gemessen. Die Probe `vor_der_antwort_auf_das_lesen_wird_nichts_geschrieben` hält die Kernseite.
- **Geht beim Beenden Text verloren?** Im gewöhnlichen Fall nicht: `wird_beendet` ruft `quicknote_abschliessen` vor dem Sitzungsstand, das gibt eine fällige Sicherung und wartet höchstens 2 s. Nach `OrdnerFehlt` oder `Gescheitert` aber ja, siehe B1.
- **Plattenarbeit auf dem Hauptfaden?** Keine gefunden. `metadata`, `Stempel::von_pfad`, Lesen, exklusives Anlegen und `datei::sichern` stehen allein im Faden. Der Pfad ist ein `join` (`Heimordner::quicknotedatei`).
- **Rechte 0600:** `exklusiv_anlegen` mit `OpenOptionsExt::mode(0o600)` und `create_new`. `atomar::vorbereiten` überträgt die Rechte mit `rechte_uebernehmen` vor `io::copy`, also liegt nie Inhalt in einer Nachbardatei mit `0644`. Die Proben `…_mit_0600` und `die_rechte_einer_vorhandenen_datei_bleiben` halten beides.
- **Änderung von außen:** Der Stempel wird vor dem Lesen und nach jedem eigenen Schreiben erhoben und vor jedem Schreiben verglichen. Das Fenster zwischen Vergleich und `rename` bleibt, wie im Plan benannt. Eine Datei, die während der Fehlanzeige entsteht, und `EEXIST` beim Anlegen sperren, die Proben halten beides.
- **Fehlender Ordner:** Es wird nichts angelegt und nichts geschrieben, und die Meldung kommt einmal. Ohne Heimordner gilt dasselbe. Proben vorhanden.

Die Proben der Sicherung und die neuen Quelltextproben laufen: `cargo test -p krk-ui -- quicknotesicherung quicknote_stellt die_aktionen_melden strip_und_format eine_umformung das_eingelesene vor_dem_einlesen kein_weg_schreibt_klartext`: 22 bestanden, 0 fehlgeschlagen.

### B1 (Medium): Nach `OrdnerFehlt` oder `Gescheitert` sichern Verlassen und Beenden den letzten Stand nicht

`Editorbereich::quicknote_sichern` nimmt `quicknote_faellig` schon beim Absenden und nicht erst bei `Gesichert`. Antwortet der Faden mit `OrdnerFehlt` oder `Gescheitert`, ist die Vormerkung weg. `quicknote_verlassen` und `quicknote_abschliessen` schreiben dann nichts. Ablauf beim ersten Kontakt: neue Installation, `f10`, tippen, Meldung „F2 legt ihn an“, F2, beenden, Text fort (abgeleitet aus dem Code, nicht am Bündel gefahren). Ziel 1 des Briefs verlangt, dass „jede Änderung“ landet. Issue: `261031-1156-nach-fehlendem-ordner-oder-gescheitertem-schreiben-sichern-verlassen-und-beenden-den-letzten-stand-nicht.md`.

### B2 (Low, kein Issue): Eine hängende Antwort auf das Lesen lässt die Quicknote stumm gesperrt

Antwortet der Faden auf `Lesen` nicht (Notizordner auf einem hängenden Netzlaufwerk), bleibt die Fläche ohne Meldung und ohne Zeitgrenze nicht bearbeitbar, und der Takt läuft mit 60 Hz weiter (`quicknote_sicherung_laeuft` fragt `Ungelesen`). Vor dem Paket ging die Quicknote immer. `HowTo.md` sagt „für einen Augenblick“. Wenn das hingenommen ist, gehört der Fall in `HowTo.md`. Sonst wäre eine Zeitgrenze, nach der die Quicknote ungesichert freigegeben wird und das meldet, der Weg.

### B3 (Low, kein Issue): `Deskriptormangel` sperrt bis zum Neustart

`Lesehindernis::Deskriptormangel` ist ein vorübergehender Zustand des Prozesses und sperrt nach E7 trotzdem endgültig. So im Plan gewählt, zur Kenntnis.

## C. Prüfbarkeit, Plantreue und Dokumentation

### C1 (Medium): Ersetzte AppKit-Proben lassen Akzeptanzen von Schritt 7 am Verhalten ungeprüft

Schritt 7 verlangt fünf Proben an einer Fläche (`an_einer_flaeche`). Gebaut sind sie als Quelltextproben (Commit `ad67ceea`, Begründung: `libtest` hält eine Fläche, deren Text sich ändert, nicht aus). Was das bedeutet:

- **Am Verhalten gemessen, aber nicht wiederholbar:** Rückgängig als eine Handlung, eine nicht bearbeitbare Fläche weist ab, `setString:` ohne `textDidChange:`. Belegt durch den Swift-Spike in `messungen/261031-1300-quicknote-textdidchange.txt`. Der Spike ist nicht KRKs Code, und `make check` fährt ihn nicht.
- **Allein am Wortlaut gehalten:** Auswahl sonst alles, Ergebnis danach ausgewählt (`eine_umformung_wirkt_auf_die_auswahl_sonst_auf_alles` sucht Teilzeichenketten), Reihenfolge der Rufe in `ersetzen`.
- **Von keiner Probe erreicht, weder am Verhalten noch am Wortlaut:** die Vormerk- und Abgabelogik des Editorbereichs (Fälligkeit nach `QUICKNOTE_RUHE`, Abgabe am Takt, Abgabe beim Verlassen und Beenden, Ende des Takts), und dass `wird_beendet` `quicknote_abschliessen` ruft. Darauf ruht Ziel 1 des Briefs. Diese Logik braucht keine Fläche, sie wäre ohne AppKit prüfbar gewesen. Genau dort sitzt B1.

Der Plan setzt die Abnahme am Bündel als Bedingung vor einer Auslieferung (Abschnitt „Where this work stops“). Sie ist nicht gefahren, und bis dahin ist Ziel 1 durch die Proben des Fadens und durch Lesen des Codes gedeckt, nicht durch einen Lauf. Die Probe ohne AppKit, die B1 verlangt, schließt die Lücke zum größten Teil. Ein eigenes Issue steht deshalb nicht da: die Abnahme von B1 trägt sie.

Der bekannte Abbruch der AppKit-Proben unter `libtest` und das Hängen von `Quicknote::bauen` in einer Probe auf dem HEAD-Stand sind nicht Gegenstand dieses Pakets. Einen offenen Datensatz dazu habe ich nicht gefunden (Liste der offenen Defekte, Suche nach `libtest|appkit|flaeche|probe|haengt` in den Namen). Wenn er fehlt, verdient er einen eigenen.

### C2 (Low, kein Issue): Eine Probe hat ihre Erwartung geändert, ohne dass der Plan die Ausnahme nennt

`kein_weg_schreibt_klartext_nach_secrets_txt` (`crates/krk-ui/src/editormodell.rs`) erlaubt seit `79f79c7c` einen zweiten Rufer von `datei::sichern`, `quicknotesicherung.rs`. Die Grundabnahme verlangt, dass vorhandene Proben ihre Erwartung behalten, außer der Schritt nennt die Ausnahme. Schritt 3 nennt sie nicht. E5 hatte allein `nur_benannte_dateien_erreichen_das_atomare_schreiben` bedacht. Die Änderung ist nötig, im Doc-Kommentar und in der Commit-Nachricht begründet und benennt ihre Lücke (ein künftiger Rufer mit einem Pfad auf `secrets.txt`). Zur Kenntnis: die Probe, die Klartext in `secrets.txt` verhindern soll, lässt jetzt einen zweiten Schreibweg durch.

### Geprüft und ohne Befund

- **Sicherheitszwänge:** Neues `unsafe` steht allein in `crates/krk-ui/src/appkit/quicknote.rs` (`unsafe impl NSTextDelegate`, drei `#[unsafe(method(…))]`). Kein `Command::new`, kein `preisgeben`, kein Bezug auf `ablage::pfade` oder Application Support im Diff (`git diff 135960bf..be1a725d -- crates xtask | grep '^+'` mit diesen Nadeln). `Cargo.toml`, `crates/*/Cargo.toml` und `Cargo.lock` sind unverändert. `scratch.txt` liegt im Notizordner, `Datei::ALLE` und `Sonderdatei::ALLE` sind unverändert.
- **Belegung und Sprachtabellen:** Zwölf neue `Text`-Schlüssel stehen in `schluessel.rs` und in `ALLE` (781 auf 793) und haben je einen Eintrag in `de.rs`, `fr.rs` und `en.rs`. `kommandoname` hat für beide Kommandos einen Eintrag in allen drei Tabellen. Die Wiederverwendung von `EditorKeineGewoehnlicheDatei` und `EditorGeheimnisseKeinDateizugriff` passt grammatisch in „({grund})“ in allen drei Sprachen. `KENNUNGEN` geht von 110 auf 112, `default-keymap.toml` trägt beide mit `tasten = []`, die Kopfzeile „119 Funktionen“ stimmt mit `grep -c '^\[\[funktion\]\]' resources/default-keymap.toml` überein (119). Menü, `UNBELEGT_AB_WERK`, Zulässigkeit und Befehlszweige folgen dem Muster von `quicknote_leeren`.
- **Dokumentation:** `HowTo.md` beschreibt Ort, Zeitpunkte, Rechte, Klartext, Sperren und „Ort wählen…“ richtig. Die Abnahme-Greps aus Schritt 8 bestehen (`allein im Arbeitsspeicher` an keiner der vier Stellen, `scratch.txt` siebenmal in `HowTo.md`). Die Regeldatei nennt den Klartextweg neben der Regel um `haelt_geheimnisse`. Ausnahmen davon: A1 (der Satz zum nicht Entscheidbaren) und B1 („bei der nächsten Änderung“ verschweigt den Verlust beim Beenden).

## Querschnitt

- **Eine Bedingung, die aus einem Beispiel stammt.** Die Umbruchregel ist am Beispiel des Nutzers gemessen (E9: „jede zu verbindende Zeile hat 156 bis 168 Zeichen“). Am Beispiel ist die längste Zeile tatsächlich umgebrochen. Das Signal, das die Regel davon abhalten sollte, die längste Zeile in jedem anderen Text ebenso zu verbinden, gibt es bei Chat-Text nicht. Strip und Format teilen die Regel, deshalb trifft der Fehler beide.
- **„Fällig“ gleich „ungesichert“ gesetzt.** B1 kommt daher, dass „eine Sekunde nach der letzten Änderung“ und „seit dem letzten Gelingen nicht auf der Platte“ ein Feld teilen. Der Faden kennt den zweiten Zustand (`ordner_fehlte`, `gescheitert`), der Editorbereich nicht.
- **Quelltextproben statt Verhaltensproben.** C1 und B1 treffen sich: Die Logik, die keine Fläche braucht, hat nicht einmal die Fläche gebraucht, um ungeprüft zu bleiben. Sie steht in `appkit/` und wird deshalb wie AppKit-Code behandelt.

## Reihenfolge

1. **Vor einer Auslieferung:** A1 entscheiden und beheben. Strip, das einen Befehl in einen Kommentar schiebt, und Format, das eine Tabelle zerstört, sind die zwei Wirkungen, die der Nutzer beim ersten Gebrauch sieht.
2. **Vor einer Auslieferung:** B1 samt der Probe ohne AppKit, die zugleich C1 schließt.
3. Danach die Abnahme am Bündel, die der Plan ohnehin vor einer Auslieferung verlangt.
4. Aufräumen: A2/A3. B2 entweder in `HowTo.md` nennen oder mit einer Zeitgrenze versehen.

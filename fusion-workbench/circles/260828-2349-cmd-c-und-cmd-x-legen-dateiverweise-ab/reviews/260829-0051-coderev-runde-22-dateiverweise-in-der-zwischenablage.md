# Durchsicht der Runde 22: Cmd+C und Cmd+X legen Dateiverweise ab

**Date:** 2026-08-29, 260829-0051
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `83e011c..38aa652`
**Not-opened:** none
**Circle:** `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`
**Spec:** `planning/260829-0005_*_spec-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`
**Plan:** `planning/260829-0006_*_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md`
**Frühere Durchsicht dieser Runde:** keine (`carried=(not recorded)`)

Geöffnet: alle acht Dateien des Diffs (`git diff --stat 83e011c..38aa652 -- crates resources`: `kommandos/{operationen,zulaessigkeit,mod}.rs`, `appkit/{zwischenablage,tabelle,anwendung,menue,betrachter}.rs`), dazu zum Nachschlagen `kommandos/operationen.rs` (`betroffene`, `Auswahl`, `eintragsname`), `appkit/anwendung.rs` (`lage`, `bereichskommando`, `kommando_ausfuehren`), `appkit/ereignisse.rs` (Nachschlag-Zweig), `pruefordner.rs`, `kommandos/kontextmenue.rs`. Der Arbeitsbaum ist gleich HEAD `38aa652` (`git diff --stat HEAD -- crates resources` leer).

## Summary

Die Runde ist so gebaut, wie Spec und Plan sie beschreiben: ein Rumpf der Zulässigkeitsregel mit zwei benannten Eingängen, ein zweiter Ausgang der einen Hülle, zwei Antworten am Anwendungsdelegierten, kein neues `Kommando`, kein `paste:`, keine Verschiebung nach `cmd+x`. Die tragende Erschließung des Plans (ein Datei-`NSURL` legt keine Textsorte ab, `setString:forType:` landet am ersten Eintrag, ein Textleser bekommt allein die Namen) habe ich mit einer eigenen Messung am Ablageserver bestätigt. Kein Release-Blocker. Zwei Defekte (beide Low) und ein Entscheidungsdatensatz sind gefiled; ein Doc-Kommentar zählt falsch.

## Totals

Critical 0 / High 0 / Medium 0 / Low 3 (davon zwei als Defekt gefiled, einer in den Defekt zur Zählung aufgenommen), dazu eine Nutzerfrage als Entscheidungsdatensatz.

## Was ich verifiziert habe

Im Sinn von `critical-stance.md` §3: jede Zeile hier ist geprüft, mit Befehl oder Stelle.

- **Bau, Proben, Clippy.** `cargo clippy --workspace --all-targets -- -D warnings`: exit 0. `cargo test -p krk-ui`: 851 passed, 0 failed (ein Lauf, allein; die Nebenläufigkeitsfrage der Probenablagen ist `issues/260829-0041_*_die-probenablagen-der-huelle-teilen-sich-zwei-gleichzeitige-testlaeufe.md` und bleibt dort).
- **`objc2`-Sicherheit in `dateiverweise_auf_ablage_schreiben`** (`crates/krk-ui/src/appkit/zwischenablage.rs:359-384`). `clearContents` vor `writeObjects:`, `Retained<NSURL>` werden in `verweise` gehalten, solange die `ProtocolObject`-Referenzen in `schreiber` leben; `NSArray::from_slice` kopiert die Zeiger, `writeObjects:` behält sie selbst. Kein `unsafe` außer den zwei Sortenkonstanten, wie im Bestand. Bei `false` aus `writeObjects:` sofortige Rückkehr; sonst `setString_forType` als Antwort. `#[must_use]` an beiden Ausgängen (`:358`, `:394`), der Rufer `DateifensterQuelle::dateiverweise_ablegen` (`tabelle.rs:1940-1952`) wertet den Wert in `if` aus, kein `let _ =`.
- **Was ein Datei-`NSURL` schreibt, und wo `setString:forType:` landet.** Gemessen mit einem Swift-Programm gegen eine benannte Ablage (`pasteboardWithName:`), zwei Dateien, dieselbe Reihenfolge der Aufrufe wie in der Hülle:
  - `writableTypesForPasteboard:` eines Datei-`NSURL`: `public.file-url` und sonst nichts. Der Plan (Entscheidung 3) und die Risikotabelle nennen daneben `public.url`; das trifft nicht zu, ist aber folgenlos.
  - nach `writeObjects:` zwei Einträge, je allein `public.file-url`; `stringForType:` liefert `nil`.
  - nach `setString:forType:` trägt Eintrag 0 `public.file-url` und `public.utf8-plain-text` mit den Namenszeilen, Eintrag 1 weiter allein `public.file-url`.
  - `readObjectsForClasses:[NSString]` über alle Einträge liefert genau eine Zeichenkette, die Namenszeilen; ein Textziel, das alle Einträge liest, bekommt also nicht die `file:`-Adressen der Einträge 2 bis n. Das war das Risiko der ersten Zeile der Risikotabelle; es tritt nicht ein.
  - `readObjectsForClasses:[NSURL]` mit `NSPasteboardURLReadingFileURLsOnlyKey` liefert beide Pfade in Reihenfolge.
  - die Sortenliste der Ablage (`types`) trägt daneben serverseitig `NSFilenamesPboardType`, `CorePasteboardFlavorType 0x6675726C` („furl") und `Apple URL pasteboard type`: der Ablageserver leitet die alte Dateinamen-Sorte aus `public.file-url` ab, KRK schreibt sie nicht. `public.file-url` je Eintrag genügt damit für den Finder; ein eigenes `NSFilenamesPboardType` wäre eine zweite Schreibweise derselben Sache.
  Damit ist die Sortenfrage, die der Modulkopf (`zwischenablage.rs:181-186`) und Schritt 9 dem Abnahmelauf überlassen, am Ablageserver beantwortet; was der Abnahmelauf noch zeigt, ist das Verhalten der fremden Anwendungen beim Einfügen.
- **Verknüpfung als Verknüpfung.** `fileURLWithPath:` löst nicht auf; die Probe `eine_verknuepfung_wird_als_verknuepfung_abgelegt` (`zwischenablage.rs:563-581`) legt eine Verknüpfung per `symlink` an und liest ihren Pfad zurück, grün. `betroffene` liefert `ordner.join(&eintrag.name)` (`operationen.rs:172-200`), also nie das Ziel.
- **Die zwei Frager der Regel.** `dateiablage_zulaessig` (`zulaessigkeit.rs:230`) wird genau zweimal außerhalb seiner Datei gerufen: `validateMenuItem:` (`anwendung.rs:967-968`) und `dateiablage_ausfuehren` (`anwendung.rs:3188-3190`), beide auf `self.lage()`. Die Zählprobe `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` (`zulaessigkeit.rs:419`) hält es. `paste:` fällt in `else { true }` (`anwendung.rs:969-970`); die Klassenprobe `der_delegierte_beantwortet_copy_und_cut_und_paste_nicht` (`anwendung.rs:9852`) hält, dass der Delegierte `paste:` nicht beantwortet.
- **Blattsperre.** `Anspruch::Dateiablage` antwortet `waehrend_blatt_erlaubt = false` und `immer_erreichbar = false` (`zulaessigkeit.rs:261-285`); `waehrend_eines_blattes_kommen_genau_diese_vier_durch` prüft die Dateiablage bei stehendem Blatt eigens, in `Fokus::Anderswo` und in `Fokus::Dateifenster` (`zulaessigkeit.rs:903-911`), und die Liste der vier bleibt. `immer_erreichbar` (`:305-318`) ist unverändert.
- **Fokus in Editor, Vorschau, Betrachter, Textfeldern.** Die Antwortkette findet `copy:` und `cut:` an einer `NSTextView` (Editor, Vorschautext, Feldeditor eines `NSTextField`), bevor sie den Delegierten erreicht; `GEMESSEN` (`menue.rs:897-899`) hält, dass `NSTextView` beide beantwortet. Der Delegierte wird also dort nicht gefragt, und sein `validateMenuItem:` auch nicht. Erreicht ihn `cut:` doch, etwa vom Betrachter, dessen `PDFView` `copy:` selbst beantwortet (`betrachter.rs`) und `cut:` nicht, antwortet `dateiablage_zulaessig` über `fokus::wirkt(Dateifenster, Betrachter)` mit `false`, und der Eintrag ist grau wie vor der Runde. Die Probe `die_dateiablage_wirkt_genau_mit_dem_fokus_im_dateifenster` (`zulaessigkeit.rs:444-462`) hält das über `Fokus::ALLE` und alle sieben Hindernisse. Was ich nicht am Bündel gesehen habe, sage ich nicht: dass AppKit die Kette im Editor wirklich vor dem Delegierten beendet, ist Erschließung aus `GEMESSEN` und der Antwortkettenregel, kein Lauf.
- **Leere Menge.** `dateiverweise_ablegen` (`tabelle.rs:1941-1944`) fragt `ist_leer()` vor jedem Schreiben; die Ablage bleibt unberührt, der Satz ist `nichts_zu_kopieren` und kein zweiter. Die Hülle selbst schließt die leere Menge nicht aus und sagt im Doc-Kommentar, warum der Rufer es tut (`zwischenablage.rs:351-355`). Zwei Rufer hat sie nicht; ein zweiter müsste dieselbe Vorprüfung wiederholen, das ist die Stelle, an der die Lage später kippen kann.
- **Vollständigkeit ohne Auffangzweig.** `Anspruch` (zwei Varianten, drei vollständige `match`, `zulaessigkeit.rs:247-285`), `Dateiablage` (zwei Varianten, ein vollständiges `match` in `ablagemeldung`, `operationen.rs:1185-1195`). `Kontextbefehl` bleibt bei drei Werten (`awk '/pub enum Kontextbefehl/,/^}/' … | grep -c`: 3), `Kommando` und `Art` sind im Diff nicht enthalten.
- **Die Zählprobe im Betrachter** (`betrachter.rs:731-773`): zwei Nadeln, je Nadel die Liste `(Datei, Zahl)` in der Sortierung von `quelldateien`, erwartet `copy:` je einmal in `anwendung.rs` und `betrachter.rs`, `cut:` einmal in `anwendung.rs`. Grün.
- **`cmd+x` ohne Verschiebung und ohne Kennzeichnung (A4).** `dateiverweise_ablegen` stellt keinen Auftrag, fasst weder Markierung noch Auswahl an, und der einzige Unterschied der zwei Werte von `Dateiablage` ist der Zusatz in `ablagemeldung` (`operationen.rs:1189-1194`); die Probe `die_meldung_nach_ausschneiden_beginnt_mit_der_nach_kopieren` hält, dass die Meldung nach `cmd+x` mit der ganzen nach `cmd+c` beginnt.
- **Aktive Fensterseite.** `dateiablage_ausfuehren` nimmt `modell.borrow().aktiv()` (`anwendung.rs:3196`); `bereichskommando` tut für `Fokus::Dateifenster` dasselbe (`anwendung.rs`, Zweig `Fokus::Dateifenster | Fokus::Anderswo`). Die Ausleihe des `RefCell` endet mit der Anweisung, bevor `dateiverweise_ablegen` läuft.
- **Belegung, Menü, Kisten.** `resources/default-keymap.toml` und `Cargo.lock` sind nicht im Diff (`git diff --stat 83e011c..38aa652 -- crates resources`: acht Dateien, alle unter `crates/krk-ui/src/`). `NSPasteboard` wird außerhalb der Hülle in keiner Codezeile angesprochen; `die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei` hat `writeObjects` als dritte Nadel (`zwischenablage.rs:685`).
- **Untergrenzen.** `zwischenablage.rs:192-203` nennt `fileURLWithPath:` (seit 10.0) und `NSPasteboardWriting` (seit 10.6); `anwendung.rs:209-211` sagt, dass `copy:` und `cut:` erklärte Selektoren ohne Untergrenze sind. Keine genannte Untergrenze liegt über macOS 15.

## Findings by theme

### 1. `#[must_use]` steht an den neuen Stellen und fehlt an ihren Geschwistern (Low, cross-cutting)

Die Runde trägt das Attribut an `dateiverweise_auf_ablage_schreiben`, `dateiverweise_schreiben`, `dateiablage_zulaessig`, `ablagemeldung` und `verweise_abgewiesen`, wie Spec A12 und Constraint 2 es verlangen. Daneben stehen in denselben Dateien Geschwister derselben Bauart ohne das Attribut:

- `zwischenablage.rs:322` `pub fn text_schreiben(text: &str) -> bool` — die Hülle um `generalPasteboard` für die zwei Pfadkopierer; ihre Schwester `text_auf_ablage_schreiben` (`:310`) trägt es, die zwei neuen Ausgänge tragen es. Die drei Rufer werten den Wert heute aus (`tabelle.rs:1880`, `:1908` in `if`; `betrachter.rs:371` mit `let _ =`), ein vierter dürfte ihn still fallen lassen.
- `operationen.rs`: die Meldungen der Runden 4 und 6 (`kopiermeldung :960`, `nichts_zu_kopieren :978`, `nichts_zu_oeffnen :987`, `nichts_zu_teilen :1025`, `ablage_weist_ab :1119`, `oeffnungsmeldung :1250`, `kein_terminal :895`) tragen es nicht; die der Runde 17 (`nichts_zu_packen :1006`, `kein_archiv :1046`, `mehrere_archive :1065`, `kein_finder :1086`) und der Runde 22 tragen es.

Das ist kein Defekt dieser Runde, sondern ein Bestand, den sie um eine Runde verlängert: die Regel aus CLAUDE.md („Ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`") gilt je Funktion und nicht je Runde, und die Datei sagt an keiner Stelle, warum die alten Geschwister ausgenommen sind. Gefiled: `issues/260829-0051_*_must-use-steht-an-den-neuen-ausgaengen-und-meldungen-und-fehlt-an-ihren-geschwistern.md`. Vorschlag: das Attribut an die Geschwister nachziehen, in einem Zug, und `cargo clippy -- -D warnings` sagt, ob ein Rufer den Wert fallen lässt.

### 2. Die Abweisungsmeldung nennt die Einträge, auch wenn allein die Namenszeilen abgewiesen wurden (Low)

`dateiverweise_auf_ablage_schreiben` (`zwischenablage.rs:359-384`) liefert `false` in zwei Lagen: `writeObjects:` hat abgewiesen, dann trägt die Ablage nach `clearContents` nichts; oder `writeObjects:` hat angenommen und `setString:forType:` danach abgewiesen, dann trägt die Ablage die Verweise und keine Namen. Der Rufer (`tabelle.rs:1950`) sagt in beiden Lagen `die Zwischenablage hat die Einträge nicht angenommen` (`verweise_abgewiesen`, `operationen.rs:1211`). In der zweiten Lage ist der Satz falsch: ein `cmd+v` im Finder legte die Einträge ab. Die Lage ist am Ablageserver nicht beobachtet, und ich halte sie für selten; der Rückgabewert der Hülle trennt sie aber nicht, und der Doc-Kommentar von `verweise_abgewiesen` sagt „meldet, dass die Ablage nicht stattgefunden hat", was in der zweiten Lage nicht stimmt. Gefiled: `issues/260829-0052_*_die-abweisungsmeldung-nennt-die-eintraege-auch-wenn-allein-die-namenszeilen-abgewiesen-wurden.md`. Zwei Wege: die Hülle räumt bei abgewiesenem Text die Ablage wieder (`clearContents`), damit der Satz stimmt; oder sie liefert, was angenommen wurde, und der Rufer sagt es. Der erste ist kleiner und hält die Zusage „ein Ablegen ist ganz oder gar nicht".

### 3. Der Doc-Kommentar von `lage()` zählt drei Abnehmer, es sind vier (Low, in den Defekt 1 aufgenommen)

`anwendung.rs:3135-3140`: „Drei Abnehmer lesen sie: der Kommandozweig in `kommando_ausfuehren` …, der Zeichenzweig von `eingabe_ausfuehren` …, und die Ausgrauung des Hauptmenues". Seit der Runde 22 ist `dateiablage_ausfuehren` (`:3188`) der vierte, und die Ausgrauung fragt die Erhebung jetzt in zwei Zweigen. Die Zahl steht als Wort neben ihrer Liste, genau der Fall aus `critical-stance.md` §5. Ein Satz im selben Defekt wie Thema 1, weil beides Nachzug an Kommentaren derselben Runde ist.

### 4. C2.1 erwartet im Terminal den Namen; ein Terminal liest die Dateiverweise vor dem Text (Nutzerfrage)

`inference:` Terminal.app fügt bei Dateiverweisen auf der Ablage den Pfad ein, mit Shell-Maskierung, und nicht die Textsorte daneben; das ist das Verhalten nach einem Kopieren im Finder, und nach dieser Runde liegt dieselbe Sortenlage vor (gemessen oben: `public.file-url` je Eintrag, `NSFilenamesPboardType` serverseitig). Wenn das zutrifft, ist C2.1 („`cmd+v` in einem Terminal: es erscheint der Name, ohne Ordner") am Bündel nicht zu erfüllen, ohne die Verweise wegzulassen, und die Aussage der Directive („Terminal, Textfeld: die Namen" im Diagramm) ist an dieser Stelle falsch. Der Code ist dabei richtig: der Spec will beides, Finder-Ablage und Namen, und das Ziel wählt. Ich habe es nicht am laufenden Terminal geprüft. Als Entscheidungsdatensatz gefiled, weil die Antwort das Kriterium und nicht den Code ändert: `decisions/260829-0053_*_was-erwartet-c2-1-beim-einfuegen-in-ein-terminal-den-namen-oder-den-pfad.md`.

### Nicht gefiled, weil schon gefiled oder kein Defekt

- Die Probenablagen und parallele Testläufe: `issues/260829-0041_*_…` steht offen; mein Lauf war allein und grün, das sagt zur Frage nichts.
- Die drei Baumaussagen des Specs: `issues/260829-0006_*_…` des Planners; nicht doppelt gefiled.
- `public.url` in Plan und Risikotabelle: die Messung sagt, ein Datei-`NSURL` schreibt allein `public.file-url`. Der Plan ist eine Aufzeichnung seines Tages und bleibt; der Modulkopf der Hülle behauptet die Sorte nicht.
- `CLAUDE.md` nennt den zweiten Ausgang der Hülle nicht: der Plan bucht es dem Kurator, nicht dieser Runde.

## Cross-cutting observations

- **Die Runde legt an drei Stellen dieselbe Bauform an, und sie hält:** ein benannter Eingang vor einem privaten Rumpf (`zulaessig`/`dateiablage_zulaessig` → `gestattet`), ein benannter Ausgang vor einem Ausgang mit gereichter Ablage (`dateiverweise_schreiben` → `dateiverweise_auf_ablage_schreiben`), eine Aktion je Selektor vor einer Funktion je Befehl (`dateien_kopieren_aktion`/`dateien_ausschneiden_aktion` → `dateiablage_ausfuehren`). Je Stelle hält eine Zählprobe die Zahl. Das ist das Muster des Bestands (`text_schreiben`/`text_auf_ablage_schreiben`, `krk_kommando`/`kommando_ausfuehren`) und keine dritte Art.
- **`#[must_use]` folgt in `operationen.rs` und `zwischenablage.rs` der Runde, in der eine Funktion entstand, und nicht ihrer Bauart** (Thema 1). Wer die Regel aus CLAUDE.md als Zusage über die Datei liest, findet sie an elf Stellen gehalten und an acht nicht.

## Recommended sequencing

Kein Release-Blocker. Die Runde kann schließen, wie der Plan es vorsieht (beschränkt, bis der Nutzer Schritt 9 gefahren hat). Vor dem Abnahmelauf lohnt die Nutzerfrage zu C2.1, damit der Lauf im Terminal gegen das richtige Kriterium prüft. Die zwei Defekte sind Aufräumarbeit für eine spätere Sitzung des `coder`; keiner ändert eine Zusage.

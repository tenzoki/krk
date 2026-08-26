# Vollbaum-Durchsicht der 27 übrigen Dateien unter `crates/krk-ui/src/appkit/`

**Reviewed-range:** `004ff72..ca8072d`
**Not-opened:** none

> Vollbaum-Durchsicht ohne Codeänderung im Bereich: der Quelltext ist seit `004ff72` unverändert, alle Commits der Spanne tragen Werkbankdateien. Die zwei Pflichtfelder stehen in der Form, die `bin/fusion-review-coverage` liest.

**Sender:** coderev (Aufgabe R9)
**Gelesen:** 27 Dateien, 15.272 Zeilen, mit `wc -l` am Baum abgelesen: `abwurf.rs` 465, `aufteilung.rs` 641, `belegungsansicht.rs` 857, `bereichsleiste.rs` 864, `bildtakt.rs` 162, `ereignisse.rs` 1.182, `fenster.rs` 466, `fsevents.rs` 359, `hinweis.rs` 93, `koordinaten.rs` 171, `leiste.rs` 611, `menue.rs` 1.317, `mod.rs` 219, `nummernspalte.rs` 572, `papierkorb.rs` 312, `standardprogramm.rs` 93, `statuszeile.rs` 1.626, `tableiste.rs` 151, `teilen.rs` 431, `terminal.rs` 107, `textautomatik.rs` 318, `textmerkmale.rs` 470, `titelzusatz.rs` 375, `volumes.rs` 666, `vorschau.rs` 2.063, `weitereinstanz.rs` 162, `zwischenablage.rs` 519. Dazu als Rufer gelesen, nicht im Bereich: `anwendung.rs:1585-1605, 4820-4836, 7698-7712`, `krk-core/src/ablage/sperre.rs:40-80, 170-215`, `vorschaumodell.rs` (Grenzen und Leseweg per `grep`), `menuemodell.rs:542-558`, `belegungsmodell.rs` (Zählung `Funktionsbereich`).
**Gefiltert:** 8 neue Datensätze unter `shared/issues/` (1 Mittel, 7 Niedrig), dazu 3 Nachträge „Also seen" (`260812-1702`, `260812-1731`, `260826-1223`). Vor dem Schreiben die 273 offenen Namen gelistet; keiner der acht Befunde stand schon.

## Zusammenfassung

Die 27 Dateien halten die Zusagen aus `CLAUDE.md`, die diese Aufgabe zu prüfen aufgab, am Code: der Abgriff kennt Editor und Vorschau nicht, `makeFirstResponder:` ist der eine Auslösepunkt, die Aufteilung trägt fünf Kästen und `rahmen_setzen` schreibt nur Farben, die Nummernspalte ist eine Klasse, `NSPasteboard` wird nirgends an der Hülle vorbei angefasst, kein `removeItem`/`unlink` steht im Verzeichnis, und 25 von 25 Dateien, die den Untergrenzen-Abschnitt tragen sollen, tragen ihn. Der eine Befund mit Gewicht ist keine Verletzung einer Zusage, sondern ihr blinder Fleck: die Nummernspalte baut je Anschlag den Zeilenindex über den ganzen Flächentext neu, während `hervorhebung.rs` daneben fortschreibt. Die übrigen sieben sind Prosa, die dem Code hinterherläuft, ein `must_use`-Nachzug, und zwei kleine Uneinheitlichkeiten (UTF-8-Pfade, Abwahl in der Leiste).

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 1 |
| Niedrig | 7 |

## Befunde nach Thema

### 1. Kosten im Zeichenpfad

**Mittel — `shared/issues/260826-1416_o_die-nummernspalte-kopiert-bei-jeder-textaenderung-den-ganzen-text-und-baut-den-zeilenindex-von-vorn.md`.** `nummernspalte.rs:314-321` kopiert bei jedem `textGeaendert:` (`:196-200`) den ganzen Flächentext (`string().to_string()`), baut `Zeilenindex::neu` und rechnet `anfaenge_in_utf16` (`:509-514`) über alles — im Zeichenpfad, synchron, bis 16 MB. Der Modulkopf (`:63-79`) begrenzt die Zahl der Neuaufbauten auf einen je Bild, nicht ihre Kosten. Ungemessen; verwandt mit der offenen C3-Messung.

### 2. Prosa gegen Code

**Niedrig — `260826-1418` (`menue.rs:105-109`, `:173-176`).** Der Modulkopf beschreibt den Fokusvorbehalt noch als frühen Ausstieg des Abgriffs; `ereignisse.rs:112-124` und `behandeln` (`:602-669`) widersprechen. Exakt die Lesart, vor der `CLAUDE.md` warnt.

**Niedrig — `260826-1419` (`mod.rs:79-82`, `bildtakt.rs:3`, `mod.rs:16-37`).** Neun statt zehn Ankreuzfelder; „fünf übrige Module" bei dreißig; Überblick ohne `abwurf` und `weitereinstanz`. Zwei weitere veraltete Sätze in `mod.rs` waren schon gefiltert (`260812-1702`, `260812-1731`, dort nachgetragen).

**Niedrig — `260826-1420` (`statuszeile.rs:1508-1509`, `:1590-1591`).** Zwei Probenköpfe zählen fünf Ränge, `Rang::ALLE` (`:235-242`) trägt sechs. Dasselbe Muster wie der R3-Befund in `tabelle.rs`, hier an der Quelle.

**Niedrig — `260826-1423` (`belegungsansicht.rs:172-207`, `:712`, `:728`).** Die Tafel verspricht „eine Quelle", das Kürzelzeichen steht aber an der Aufrufstelle; die Probe hält Satz und Taste nicht aneinander.

### 3. Uneinheitliches Verhalten über mehrere Dateien

**Niedrig — `260826-1421`.** Pfade ohne gültiges UTF-8: `papierkorb.rs:130-135, 186-188`, `abwurf.rs:225-227`, `volumes.rs:269-271` weisen mit Befund ab; `terminal.rs:98`, `standardprogramm.rs:91`, `teilen.rs:293`, `fsevents.rs:290` glätten still mit `to_string_lossy`. Auf APFS kaum erreichbar, über fremde Datenträger schon.

**Niedrig — `260826-1422` (`leiste.rs:233-244`, `:570`).** `allowsEmptySelection(true)` erlaubt die Abwahl per Cmd-Klick; `selectedRow() == -1` wird verworfen, das Modell behält die alte Zeile, die nächste Pfeiltaste navigiert von ihr aus. Am Code abgelesen, nicht am Bündel.

### 4. `must_use`

**Niedrig — `260826-1417`.** 14 Attribute in 9 der 27 Dateien. Ohne stehen die vier Fremdprogramm-Antworten `standardprogramm::oeffnen`, `terminal::ordner_oeffnen`, `zwischenablage::im_browser_oeffnen`, `weitereinstanz::starten` — alle fünf heutigen Rufer verbrauchen sie — und rund 25 weitere reine Antworten, im Datensatz aufgezählt.

## Die Zusagen aus `CLAUDE.md`, je geprüft

- **`ereignisse.rs`:** `ersthelfer_gehoert_appkit` (`:703-719`) fragt erst die Nämlichkeit über den hereingereichten Abschluss, dann `isKindOfClass` auf `NSTextView`, `NSTextField`, `NSText`. Der Editor kommt in der Datei nicht vor; die Probe `die_menge_der_eigenen_textflaechen_steht_an_genau_einer_stelle` (`:1008-1028`) erwartet `ist_eigene_textflaeche` allein in `anwendung.rs`. Die Warnung „nicht die einzige Sperre" steht im Kopf (`:131-140`). `Anschlag` (`:296-302`) trägt Druck und `isARepeat` (`:620-623`); die dritte Größe der Rückschritt-Regel (Beginn bei stehendem Filtertext) ist ein Merker beim Delegierten und gehört nicht in das Ereignis — vollständig für seinen Zweck. Grenze benannt: der Messmodus baut Ereignisse mit `isARepeat = false` (`:587`) und kann den Wiederholungszweig nicht fahren.
- **`fenster.rs`:** `Hauptfenster` überschreibt `makeFirstResponder:` (`:226-237`), `becomeKeyWindow` (`:241-247`), `resignKeyWindow` (`:251-257`), meldet nur bei Erfolg. Zweiter Beobachter am Fokus in den 27 Dateien: keiner (`grep firstResponder|makeFirstResponder|addObserver`): `vorschau.rs:342` **ruft** `makeFirstResponder`, beobachtet nicht; die `addObserver`-Stellen in `nummernspalte.rs:287,294` und `volumes.rs:441` beobachten Textspeicher, Klemme und Datenträger.
- **`aufteilung.rs`:** `rahmen: [Retained<NSBox>; 5]` (`:245`), fünf `gerahmt`-Aufrufe (`:278-283`). `rahmen_setzen` (`:396-401`) ruft allein `setBorderColor`. `anwenden` (`:323-331`) hat im ganzen Baum einen Rufer, `aufteilung_nachziehen` (`anwendung.rs:4833`), und der ist nicht der Ersthelfer-Nachzug — wie `CLAUDE.md` es zusagt.
- **`nummernspalte.rs`:** eine `define_class!` (`:161-229`), ein Bauweg `einhaengen` (`:240-301`), gerufen von `vorschau.rs:1538` und dem Editor; Zählung aus `krk_core::text::Zeilenindex` (`:126`, `:317`). Eine Klasse. Befund 1 gilt ihr.
- **`vorschau.rs`:** `Vorschautext` ist `NSTextView`-Unterklasse (`:409-412`), `setSelectable(true)` (`:1524`), `writeSelectionToPasteboard:types:` (`:462-478`) legt bei `Quellbezug` den Quelltext über `zwischenablage::text_auf_ablage_schreiben` ab, sonst an die Oberklasse. `textflaeche()` (`:888-890`) geht zum Vergleich an den Delegierten. Die Zusammenfassung läuft über `Inhalt::Zusammenfassung` → `als_text` (`:1127-1129`). **Platzhalter, `zeigt`/`juengste`, Ordner ohne Auswahl der Runde 18 stehen nicht in dieser Datei** — sie liegen im Kern (`leseprofil`) und im `vorschaumodell`; `vorschau.rs` nennt sie nicht und braucht sie nicht. Leseweg: `vorschaumodell.rs:158` importiert `bis_zur_grenze_lesen`, ruft `ohne_warten_oeffnen` nicht (`:102` nennt es nur in Prosa); Grenzen `TEXTGRENZE = 1 MiB` (`:166`) und `BILDGRENZE` darüber (`:183`), `EDITORGRENZE > TEXTGRENZE` beim Übersetzen gehalten (`:196`).
- **`zwischenablage.rs`:** die einzige Datei mit `generalPasteboard`/`setString_forType`; die Probe `die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei` (`:483-518`) hält beide Nadeln. `NSPasteboard` außerhalb: `vorschau.rs:465` als hereingereichter Parameter der Überschreibung, `tabelle.rs:3568,3781` reichen `draggingPasteboard()` an `dateiverweise`. Keine Umgehung.
- **`menue.rs`:** `validateMenuItem:` liegt beim Delegierten, die Probe `die_freigabe_eines_eintrags_wird_nirgends_gesetzt` (`:1190-1225`) hält genau eine Erklärung und verbietet `setEnabled:`/`setAutoenablesItems:`. `KENNUNGEN`: **elf Zeilen in `menue.rs`** (`:44,351,437,442,445,446,459,1069,1080,1085,1109`) — vier Prosa, vier Code (`tag_des_kommandos` `:442-447` mit zwei `expect`, `kommando_zum_tag` `:458-462`), drei Proben. `:437-440` zitiert die Wirkungsbereichs-Probe für die Eindeutigkeit; für die Vollständigkeit hält weiter nichts (`260826-1223`, nachgetragen).
- **`statuszeile.rs`:** Rangfolge vollständig: `Rang` sechs Werte, `ALLE` (`:235-242`), `art` (`:251-260`), `Quellen::text` (`:295-304`) je ohne Auffangzweig, `jeder_der_sechs_raenge_hat_genau_ein_feld` (`:1341-1352`). Code-Stand zu `260814-1552`: `Filterstand` an fünfter Stelle, unter `Tabmeldung`, über `Markierungsstand`; nicht entschieden, gemeldet. Zahl an der Quelle: sechs; zwei Probenköpfe sagen fünf (Befund `260826-1420`).
- **`papierkorb.rs`:** `trashItemAtURL_resultingItemURL_error` (`:139-141`) ist der eine Löschweg. `removeItem`, `unlink`, `remove_file`, `remove_dir` kommen im Verzeichnis nicht vor.
- **`weitereinstanz.rs`, `volumes.rs`, `fsevents.rs` — was der Code tut:** die zweite Instanz startet über `openApplicationAtURL:` mit `createsNewApplicationInstance` (`weitereinstanz.rs:109-117`) und teilt sich denselben Ablageordner. Zwei `flock`-Sperren daneben (`sperre.rs:80-83`): `schreiben.lock` je Schreibdurchgang, `sitzungsrecht.lock` einmal beim Start; die zweite Instanz bekommt `Sitzungsrecht::gehalten() == false` (`sperre.rs:179-190`), `Sitzungsschreiber::neu(&recht)` liefert dann `None` (`anwendung.rs:1600`), und `sitzung_vormerken` (`:7703-7705`) kehrt sofort zurück — **sie schreibt die Sitzung nicht und meldet es beim Start** (`:1601-1605`, `OHNE_SITZUNGSRECHT`). Lesezeichen, Belegung, Zettel, Einstellungen laufen über `schreiben.lock` und werden von beiden geschrieben. `volumes.rs` und `fsevents.rs` beobachten je Prozess und teilen nichts. Die Frage `260813-0053` bleibt offen; der Code beantwortet sie heute mit „alles außer der Sitzung".
- **`terminal.rs`, `standardprogramm.rs`, `teilen.rs`:** Pfade gehen über `NSURL::fileURLWithPath` (`terminal.rs:98`, `standardprogramm.rs:91`, `teilen.rs:293`); Sonderzeichen im gültigen UTF-8 kommen unverändert an, ungültiges UTF-8 wird geglättet (Befund `260826-1421`). Fehler des fremden Programms: `terminal.rs:100-105` und `weitereinstanz.rs:113-117` übergeben `None` als `completionHandler`, `standardprogramm.rs:92` liefert allein das synchrone `bool` von `openURL:` — ein Fehlschlag nach der Annahme wird an keiner der drei Stellen gemeldet, und alle drei schreiben es als Entscheidung in den Modulkopf aus (`terminal.rs:43-55`, `standardprogramm.rs:23-38`, `weitereinstanz.rs:42-48`). Kein Verschlucken ohne Begründung; die Begründung ist die Nutzerantwort vom 260811-1610.
- **Untergrenzen-Deckung:** `grep -L 'Ab welchem macOS die angesprochenen Klassen stehen'` über die 27 nennt genau `koordinaten.rs` und `mod.rs` — **25 von 25** verlangten Dateien tragen den Abschnitt (über alle 30 Dateien des Verzeichnisses: 28). Stichprobe über 15: keine. Die höchsten Angaben sind `CADisplayLink`/`displayLinkWithTarget:selector:` 14.0 (`bildtakt.rs:32-43`), `NSApplication.activate` 14.0 (`hinweis.rs:50-52`), `setMathExpressionCompletionType:`/`setWritingToolsBehavior:` 15.0 (`textautomatik.rs:65-68`), und `setAllowsWritingToolsAffordance:` 15.4 geht über `respondsToSelector:` (`textautomatik.rs:217-229`).
- **`#![allow(unsafe_code)]`:** in `krk-ui` genau einmal, `appkit/mod.rs:1`; `main.rs:1` trägt `deny`. `mod.rs` tut sonst nichts mit der Erlaubnis: 30 `mod`-Zeilen und `pub use anwendung::starten` (`:220`).
- **`unwrap`/`expect` mit echtem Fehlerfall:** keiner außerhalb von Proben. `textautomatik.rs:218,242` sind unerreichbar (Setzername ohne Nullbyte, nicht leer); `menue.rs:445-446` hängen an der Vollständigkeit von `KENNUNGEN` (bekannt, `260826-1223`); `bereichsleiste.rs:496` wird von `genau_drei_spalten_sind_schaltbar` gehalten.

## Querschnitt

- **Zahlen in Prosa altern in vier Dateien auf dieselbe Weise** (`mod.rs`, `bildtakt.rs`, `statuszeile.rs`, `belegungsansicht.rs`): der Code hat eine Probe oder ein Feld mit fester Breite, der Kopf zählt den Stand einer früheren Runde. `menue.rs:141,831,899` („85 Funktionen", „79 der 85") stimmen heute (85 `[[funktion]]` in `resources/default-keymap.toml`) und werden mit dem nächsten Eintrag falsch.
- **Die Melder-Bauart ist einheitlich:** `fenster.rs:268-284`, `bereichsleiste.rs:550-552, 575-580`, `leiste.rs:262-264, 405-414`, `tableiste.rs`, `bildtakt.rs`, `volumes.rs` halten den Delegierten schwach, die Ausleihe lesend während des Rufs. Kein Ring gefunden.
- **`Drop` meldet ab:** `ereignisse.rs:442-448`, `bildtakt.rs:140-144`, `fsevents.rs:346-359`, `volumes.rs:447-455`, `nummernspalte.rs:454-477`. Einheitlich.

## Nicht geprüft, benannt

- `fsevents.rs:346-359`: ob ein bereits auf die Hauptwarteschlange gestellter Rückruf nach `FSEventStreamInvalidate` noch läuft und dann `info` auf die eben gefallene `Box` zeigt, ist an Apples Beschreibung nicht entschieden und hier nicht gemessen. `inference`; nicht gefiltert.
- `tableiste.rs:143-149`: `setSelectedSegment(aktiv)` bei leerer `titel`-Liste; ob das Tabmodell je eine leere Liste liefert, entscheidet `crate::tabs`, nicht diese Datei. Nicht gefiltert.

## Empfohlene Reihenfolge

Kein Auslieferungshemmnis. Zuerst `260826-1416` messen (ein Lauf mit einer 16-MB-Datei, Tippen, Zeitstempel um `index_erneuern`), dann entscheiden; die sieben Niedrig-Datensätze sind Aufräumarbeit einer Sitzung.

**Verification:** 27 Dateien vollständig gelesen (`find crates/krk-ui/src/appkit -maxdepth 1 -name '*.rs'` ohne `anwendung|tabelle|editor`, 15.272 Zeilen per `wc -l`), jede Zeilenangabe am Baum `ca8072d` abgelesen und gegen die `cat -n`-Ausgabe ein zweites Mal gelesen; Rufer in `anwendung.rs`, `sperre.rs`, `vorschaumodell.rs`, `menuemodell.rs`, `belegungsmodell.rs` per `sed`/`grep` gelesen; nicht übersetzt; keine Datei im Quellbaum geändert; `~/Library/Application Support/KRK/` nicht angefasst; die Aussagen zu FSEvents nach `Invalidate`, zur Reichweite ungültiger UTF-8-Pfade auf APFS und zum Verhalten der Leiste nach einer Abwahl sind am Code abgelesen und als `inference` gekennzeichnet, nicht gemessen.

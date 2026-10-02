# Vollbaum-Durchsicht der Blätter — `crates/krk-ui/src/appkit/blaetter/`

**Reviewed-range:** `004ff72..7ac511a`
**Not-opened:** none

> Vollbaum-Durchsicht ohne Codeänderung im Bereich: der Quelltext ist seit `004ff72` unverändert, alle Commits der Spanne tragen Werkbankdateien. Die zwei Pflichtfelder stehen in der Form, die `bin/fusion-review-coverage` liest.

**Sender:** coderev
**Gelesen:** 11 Dateien, 3.622 Zeilen, mit `wc -l` am Baum abgelesen: `konflikt.rs` 483, `loeschbestaetigung.rs` 188, `mod.rs` 1.113, `namenseingabe.rs` 124, `pfadeingabe.rs` 78, `stapelumbenennen.rs` 627, `suche.rs` 185, `uebersprungen.rs` 49, `ungesichert.rs` 112, `zeilennummer.rs` 77, `zettel.rs` 586. Dazu als Aufrufer gelesen, nicht im Bereich: die Blattstellen von `appkit/anwendung.rs`, `appkit/belegungsansicht.rs:630-760`, `appkit/teilen.rs:105-275`, `kommandos/zulaessigkeit.rs`, `kommandos/fokus.rs:343-368`, `kommandos/operationen.rs:283-285, 482-491`, und im Kern die `Konfliktantwort`-Zweige von `operation/{mod,zippen,entpacken,fortschritt}.rs` und `stapelumbenennen/regel.rs:126-156`.
**Gefiltert:** 6 neue Datensätze unter `shared/issues/` (2 Mittel, 4 Niedrig), dazu 5 Nachträge „Also seen" an bestehenden Datensätzen. Der eine Befund der Schwere Hoch war um 13:25 vom parallelen Prüfer R7 schon gefiltert (`shared/issues/260826-1325_*_esc-im-stapel-umbenennen-blatt-…md`); mein eigener Datensatz dazu ist zurückgenommen, die zwei Ergänzungen stehen als Nachtrag dort.

## Zusammenfassung

Die Hülle `Blatt` ist sauber gebaut: die zwei Fragen „welche Schaltfläche lässt liegen" und „welcher gehört die Eingabetaste" stehen je einmal als reine Funktion, jedes Blatt nennt seinen ungefährlichen Ausgang, eine unbekannte Antwort fällt auf ihn, und die Rückfrage vor dem Räumen kann mit keinem Weg als „ja" schließen, den der Nutzer nicht gegangen ist. Der eine ernste Befund liegt nicht **in** einem Blatt, sondern zwischen Hülle und Anwendungsdelegiertem: fünf der elf Blätter legen ihren Griff nie in `offenes_blatt`, und der Abbruchbefehl, der auf diesen Griff baut, fällt dann auf den laufenden Vorgang hinter dem Blatt durch. Der zweite Befund derselben Stelle ist der Einzelschlitz, den zwei asynchrone Blätter ungeprüft überschreiben.

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 (bei R7 gefiltert, hier bestätigt) |
| Mittel | 2 |
| Niedrig | 4 |

## Befunde nach Thema

### 1. Der Griff und der Abbruchbefehl

**Hoch — `shared/issues/260826-1325_o_esc-im-stapel-umbenennen-blatt-mit-fokus-in-der-vorschautabelle-schliesst-das-blatt-nicht-sondern-leert-den-filter-dahinter.md` (R7, unabhängig gefunden; mein Nachtrag steht dort).** `Blatt::zeigen` (`mod.rs:764-766`) verwirft den `Blattgriff`; Pfadeingabe, Namenseingabe, Zeilennummer, Suche und Stapel-Umbenennen stehen deshalb nie in `offenes_blatt`. Solange ein Textfeld Ersthelfer ist, verdeckt der Feldeditor das. Im Stapelblatt führt der Tabring ausdrücklich in die Vorschautabelle (`stapelumbenennen.rs:615-627`); dort ist `Kommando::Abbrechen` zulässig (`zulaessigkeit.rs:174-183`, `fokus.rs:343-345`, `anwendung.rs:5942-5947`), `abbrechen` (`anwendung.rs:5648-5673`) findet keinen Griff und bricht den laufenden Vorgang ab oder leert den Filter des Dateifensters dahinter. Das Blatt bleibt stehen. Ein Vorgang kann laufen, weil `stapel_umbenennen` (`:5803-5835`) beim Öffnen nicht danach fragt. Betrifft alle fünf `Blatt::neu`-Rufer, mit Reichweite ohne Systemeinstellung nur beim Stapelblatt.

**Mittel — `shared/issues/260826-1332_o_offenes-blatt-ist-ein-einzelschlitz-…md`.** Neun Öffner schreiben `offenes_blatt = Some(griff)` ohne den alten Wert anzusehen. Sieben sind Tastenbefehle, die bei stehendem Blatt nicht zulässig werden; zwei kommen vom Arbeitsfaden, `konflikt_fragen` (`anwendung.rs:6505-6539`) und die Abschlussliste in `vorgang_beenden` (`:6686-6692`), und prüfen `blatt_steht` nicht. Steht der Notizzettel oder die Belegungsansicht, bekommt das Fenster ein zweites Blatt, der erste Griff ist überschrieben, und der Abschluss des ersten leert den zweiten (`:4188`). `beenden_erlauben` (`:7606-7608`) kennt dieselbe Frage und antwortet `TerminateCancel`. Was AppKit mit dem zweiten `beginSheetModalForWindow:` tut, ist am Bündel nicht gemessen; der Datensatz sagt das.

### 2. Das Stapelblatt

**Mittel — `shared/issues/260826-1333_o_return-bei-unlesbarer-regel-schliesst-das-stapelblatt-…md`.** Bei `Regelfehler` steht `Vorschau::default()` (`stapelumbenennen.rs:306-319`), Return bestätigt es (`:436-440`), `stapel_beauftragen` meldet „jede Zeile trägt einen Hinweis" (`anwendung.rs:5863-5866`). Keine tut es; die vier Felder sind weg.

**Kollisionsprüfung (bestehender Datensatz `260826-1221`, Nachtrag).** Das Blatt stellt die Vorschau ohne Vorbehalt dar (`:398-401`, `:452-460`) und rechnet nichts selbst; die zu niedrige Zahl kommt aus dem Kern. Kein zweiter Datensatz.

### 3. Prosa gegen Code

**Niedrig — `260826-1334`:** `frei_zeigen` sagt „ausgewaehlt" (`namenseingabe.rs:95-98`), ruft `selectText:` nicht; `pfadeingabe.rs:67` und `suche.rs:119` tun es. Betrifft das Umbenennen eines Lesezeichens.
**Niedrig — `260826-1336`:** `mod.rs:4` zählt zehn Blätter; `belegungsansicht.rs:747` baut das elfte mit demselben Bauer, und `mod.rs:367` nennt es selbst.
**Niedrig — `260826-1337`:** `text_geaendert` (`mod.rs:245-250`) hält die `RefCell`-Ausleihe während des Rufs; `antworten` daneben (`:274-281`) nimmt sie vorher heraus und begründet es. Heute hält es, weil `neu_rechnen` nicht zurückruft.
**Niedrig — `260826-1335`:** zwei `#[must_use]` im ganzen Verzeichnis (`mod.rs:435, 481`); rund zwanzig reine Antworten ohne, und `Blattgriff` selbst — der Typ, dessen Fallenlassen den Befund 1 kostet.

## Die Zusagen aus dem Auftrag, einzeln geprüft

- **Rückfrage vor dem Räumen.** Kann nicht als „ja" schließen, ohne dass der Nutzer es gegeben hat. `loeschbestaetigung::schaltflaechen` (`:109-114`): „Abbrechen" vorn mit Return und `Liegenlassen`, der Vorgang hinten mit Cmd+Return; `abbruchstelle` liefert 0, `fertig(stelle == 1)` (`:141-143`); eine unbekannte `NSModalResponse` fällt in `zeigen_mit_wahl` auf `abbruchstelle` (`mod.rs:795-798`), also auf `false`. Die Probe `eine_unbekannte_antwort_stellt_keinen_auftrag` (`:161-169`) hält es. `Esc`: der Ersthelfer ist eine Schaltfläche, `Abbrechen` ist zulässig, `abbrechen` findet den Griff (`anwendung.rs:5469`) und schließt mit `abbruchcode`. `cmd+w` (Tab schließen) ist bei stehendem Blatt nicht zulässig — nicht unter den vier. `shift+cmd+w` (`FensterSchliessen`, immer erreichbar) sichert den Zettel und ruft `performClose:`; was AppKit dann mit dem anhängenden Blatt tut, ist laut Doc-Kommentar (`anwendung.rs:4703-4711`) **nicht gemessen**, und der Löschauftrag wird erst im Abschlussblock gestellt — ein Fenster, das ohne Abschlussblock zugeht, stellt keinen. Die zwei offenen Entscheidungen: `260818-0250` — der Code steht heute bei Möglichkeit 1 plus der Hälfte von 3: `assert!` (`mod.rs:638-643`), `unwrap_or(0)` (`:440`), reine Bauplanfunktionen für `Blatt::neu` (`standardschaltflaechen`, `:576`), die Löschrückfrage und seit der Runde 17 auch das Konfliktblatt (`konflikt.rs:139`); die drei übrigen (`uebersprungen`, `ungesichert`, `zettel`, dazu `belegungsansicht`) nennen ihre Schaltflächen im Rumpf. Die Aussage des Datensatzes „Stelle 0 ist in jedem Blatt die abbrechende" trifft am heutigen Baum nicht mehr zu (bei `Blatt::neu` ist es 1, bei `ungesichert` 2, beim Konfliktblatt 3 oder 2); die Zusicherung hängt seit dem 260818 nicht mehr daran. `260818-0512` — der Wortlaut steht unverändert bei Möglichkeit 1: „mit 25 Einträgen" und „mit mehr als 25 Einträgen" (`kommandos/loeschwarnung.rs:604-605`). Ich entscheide nichts.
- **Jedes Blatt hängt als Sheet.** Alle elf gehen durch `Blatt::zeigen_mit_wahl` und `beginSheetModalForWindow_completionHandler` (`mod.rs:804-805`); `Blatt::zeigen` ruft dieselbe Funktion (`:765`). Kein freies Fenster. Die Belegungsansicht außerhalb des Verzeichnisses ebenso (`belegungsansicht.rs:757`).
- **Keine Blattfläche ist als eigene Textfläche angemeldet.** `ist_eigene_textflaeche` vergleicht gegen Editor und Vorschau; keine Datei des Verzeichnisses ruft in diese Richtung, und `zettel.rs:15-44` begründet die Gegenrichtung. Hält.
- **Notizzettel, wann geschrieben wird.** Nicht bei jedem Zeichen: der `Zettelwaechter` beantwortet kein `NSTextDelegate` (`zettel.rs:196-200`, ausdrücklich). Geschrieben wird an vier Momenten (`anwendung.rs:4050-4064`): Tabklick, Schließen des Blattes, `shift+cmd+w`, `applicationWillTerminate:`. Beenden mit offenem Blatt geht über den vierten Moment, der zuerst `zettel_stand_uebernehmen` ruft (`:977`) und dann unter derselben Sperre schreibt (`:1009`); ein Fehlschlag dort wird nicht gemeldet, und der Kommentar nennt den Preis. **Verloren geht Text bei einem Absturz** zwischen dem letzten Anschlag und einem der vier Momente; das ist die Wahl des Spec, kein Defekt, und die Erläuterung des Blattes („wird gesichert, ohne dass du etwas tun musst", `zettel.rs:420-422`) sagt es dem Nutzer ohne diese Einschränkung.
- **Freigabedialog.** Beide Datensätze gelten am heutigen Baum; Nachträge geschrieben. Der Runde-17-Weg über das Kontextmenü läuft durch `eintrag_anfuegen` (`teilen.rs:270`), also durch die Hülle des **zweiten** Datensatzes, und nicht durch die des ersten.
- **Konfliktblatt bei Zip und Unzip.** `erzeugt_genau_ein_ziel` (`operationen.rs:482-491`) kürzt für jedes Packen und ein Entpacken mit genau einem Archiv auf drei Antworten; „Überspringen" fällt weg, weil es dort dasselbe ist wie Abbrechen (`konflikt.rs:3-15`). Alle vier Antworten verstehen `zippen.rs:295-306` und `entpacken.rs:212-223` gleich wie `mod.rs:441-452`; `UmbenennenIn` prüft den Namen, ein ungültiger wird übersprungen. Ein **leerer** Name wird schon beim Delegierten zu `Ueberspringen` (`anwendung.rs:6522-6532`) — in der gekürzten Gestalt eine Antwort, die der Nutzer nicht sieht; für ein Archiv wirkt sie wie Abbrechen. Der getippte Name gegen den Bestand: bestehender Datensatz `260825-1130`, nicht wiederholt.
- **Fortschritt und Abbruch.** Ein Fortschritts**blatt** gibt es seit S16b nicht (`mod.rs:20-29`); der Fortschritt steht in der Statuszeile. Der Abbruch erreicht den Vorgang: `abbrechen` ruft `vorgang.zustand.abbrechen()`, das ein `AtomicBool` setzt (`operation/fortschritt.rs:242-244`), das der Arbeitsfaden liest. Kein Blatt ist beteiligt.
- **macOS-Untergrenzen.** Jede der elf Dateien trägt den Abschnitt. Die jüngste genannte Berührung ist `NSTableView.style` mit 11.0 (`stapelumbenennen.rs:68-70`), alles Übrige darunter; keine über 15. Ich habe die Angaben gegen die Modulköpfe gelesen und **nicht** am SDK nachgeschlagen.
- **`#![allow(unsafe_code)]`** steht in `krk-ui` allein in `appkit/mod.rs:1`; keine Blattdatei trägt es.
- **`unwrap`/`expect` mit echtem Fehlerfall.** Einer: `zettel.rs:372-373`, `expect` an `isize::try_from(offener.index())` für einen von zwei Zetteln — kein erreichbarer Fehlerfall. `abbruchstelle` (`:440`) und `antwort` (`konflikt.rs:199, 206`) fangen auf, beide begründet. Tote Zweige: keine gefunden; die Auffangzweige `antwort(9, …)` sind absichtlich total.
- **Doppelungen derselben Regel.** Keine: die Reihenfolge-Regel steht je Blatt einmal als Bauplanfunktion oder im Rumpf, und die drei Frager (Abschlussblock, Griff, Wächter) lesen `abbruchstelle`/`bestaetigungsstelle`. Die zwei `eingabezeile`-Funktionen (`suche.rs:160-185`, `stapelumbenennen.rs:558-582`) sind Zwillinge mit einem Parameter Unterschied — Anordnung, keine Regel, nicht gefiltert.

## Querschnitt

- **Die Sperre ist vollständig, der Griff nicht.** `blatt_steht` deckt jedes Blatt ab (AppKit hält es), `offenes_blatt` nur sechs von elf (KRK hält es). Beide Befunde der Schwere Hoch und Mittel sind Ausdruck derselben Asymmetrie: die Sperre fragt AppKit, der Abbruch fragt einen Schlitz, den nicht jeder füllt und den jeder überschreiben kann.
- **Das Muster „reine Bauplanfunktion" hat drei Blätter erreicht und vier nicht** — genau der halbe Schritt, den `260818-0250` als Möglichkeit 3 beschreibt.

## Empfohlene Reihenfolge

1. `260826-1325` (Hoch, R7): `Blatt::zeigen` gibt den Griff zurück, `Blattgriff` bekommt `#[must_use]`, fünf Rufer legen ihn ab. Danach ist der zweite Rang von `abbrechen` ohne Blatt unerreichbar.
2. `260826-1332` (Mittel): die zwei asynchronen Öffner warten, solange `blatt_steht`.
3. `260826-1333` (Mittel), dann die vier Niedrigen als Aufräumen; `260826-1335` fällt mit 1 zur Hälfte mit.

**Verification:** 11 Dateien vollständig gelesen (`git ls-files 'crates/krk-ui/src/appkit/blaetter/*'`, 3.622 Zeilen per `wc -l`), jede Zeilenangabe am Baum `7ac511a` abgelesen und gegen die Ausgabe ein zweites Mal gelesen; Aufrufer in `anwendung.rs`, `belegungsansicht.rs`, `teilen.rs`, `zulaessigkeit.rs`, `fokus.rs`, `operationen.rs` und im Kern gelesen, nicht übersetzt, keine Datei im Quellbaum geändert; die Aussagen zu AppKits Verhalten bei zweitem Blatt und bei `performClose:` mit Blatt sind als `inference` gekennzeichnet und nicht gemessen.

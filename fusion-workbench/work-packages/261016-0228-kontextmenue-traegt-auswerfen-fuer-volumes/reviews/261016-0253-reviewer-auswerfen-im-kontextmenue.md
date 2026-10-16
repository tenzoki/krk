# Durchsicht: „Auswerfen“ im Kontextmenü der Dateiliste

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `9a0ef1e..bb2f861`
**Not-opened:** none
**Review domain:** code
**Work-item:** 261016-0228-kontextmenue-traegt-auswerfen-fuer-volumes

## Zusammenfassung

Der Commit erfüllt die Directive: der Eintrag steht im Kontextmenü, wirft eingehängte Datenträger über `NSFileManager` aus, wirkt bei gewöhnlichen Einträgen nicht und meldet Erfolg und Fehlschlag in der Statuszeile. Die asynchrone Hülle ist speichersicher, und der Hauptfaden fragt im Auswurfweg die Platte nicht. Zwei niedrige Befunde, keiner hält die Auslieferung auf.

Urteil: **accept**.

## Gesamtzahl

Critical 0, High 0, Medium 0, Low 2 (beide als Issue angelegt). Dazu ein ungeprüftes Risiko, das allein der Abnahmelauf am Bündel klärt.

## Geprüft und in Ordnung

**Bau.** `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (alle Läufe grün), `cargo fmt --all --check` und `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` am Stand `bb2f861` grün.

**Block-Lebensdauer und Fadengrenze** (`crates/krk-ui/src/appkit/auswurf.rs`, `auswerfen`).
- Der Block ist ein `RcBlock` über eine `'static`-Hülle; er hält allein `Mutex<Option<F>>` mit `F: Send`, ist also `Send + Sync`, wie es der Sicherheitsvermerk der Bindung verlangt (`objc2-foundation-0.3.2/src/generated/NSFileManager.rs`: „`completion_handler` block must be sendable“).
- `&block` reicht einen Heap-Block; dass Foundation einen asynchronen Abschlussblock kopiert und hält, ist Apples Vertrag für `completionHandler:`-Methoden. Das lokale `RcBlock` darf danach fallen.
- `NSError` ist in `objc2-foundation` 0.3.2 `Send` und `Sync` (`src/generated/NSError.rs`, `unsafe impl Send`/`Sync`); `localizedDescription` auf dem fremden Faden ist damit gedeckt.
- `fertig` läuft über `DispatchQueue::main().exec_async` auf dem Hauptfaden und höchstens einmal (`Option::take` unter dem `Mutex`). Im Block steht kein Weg, der einen Panik über die ObjC-Grenze trüge.
- `auswurf_einziehen` sucht den Delegierten über `NSApplication::sharedApplication(mtm).delegate()`, wie `vorgang_einziehen`; kein `Retained` reist über die Fadengrenze.

**Hauptfaden.** `auswurf_beginnen` liest Ordnermodell und Ivars; `auswurf::auswerfen` baut die URL über `fileURLWithPath:isDirectory:` (fragt nicht nach) und stellt den asynchronen Aufruf. Die Aufzählung der Datenträger ist aus dem Klick in `geraete_uebernehmen` gewandert, das dieselbe Aufzählung schon vorher bei jeder Wachemeldung auf dem Hauptfaden fuhr (`orte()` vor dem Commit). Neue Systemaufrufe auf dem Hauptfaden entstehen damit nicht.

**Fehlschläge.** Jeder Ausgang, den das System liefert, erreicht `antwort_zeigen`: `Ok` über `auswurf_erfolgt`, `Err` über `auswurf_gescheitert` mit dem Wortlaut des Systems. `befehlsantwort_zeigen` hat den obersten Rang (`appkit/tabelle.rs`), die Meldung aus `datentraeger_verloren` geht über `meldung_zeigen` darunter und verdrängt sie nicht. Still bleiben drei Ausgänge, alle unerreichbar oder ausgewiesen: kein Delegierter, ein fehlender `MainThreadMarker` auf der Hauptschlange, und ein System, das den Block nie ruft (im Doc-Kommentar der Probe `der_auswurfweg_reisst_nirgends_ab` genannt).

**Auswahl der auswerfbaren Einträge** (`kommandos/kontextmenue.rs`, `auswurfbezug`). Verglichen wird der Pfad `ordner.join(name)` gegen `NSURL.path` der Aufzählung (`appkit/volumes.rs`, `eingehaengte`, `SkipHiddenVolumes`); `path` liefert ohne Schrägstrich am Ende, ein Treffer ist also genau ein Einhängepunkt. `/` fällt über `parent().is_some()` heraus; `/Volumes/Macintosh HD` ist eine Verknüpfung auf `/` und trifft deshalb nicht. Ein Ordner auf einem Datenträger trifft nicht. Ein falscher Treffer ist mit dieser Gleichheit nicht möglich; ein fehlender (Weg über eine Verknüpfung) führt auf „nichts auszuwerfen“, wie der Doc-Kommentar sagt.

**Fallunterscheidungen und Tabellen.** `Kontextbefehl::Auswerfen` steht in `ALLE`, `titel`, `menuemarke` und im Zweig des Delegierten, alle ohne Auffangzweig; die Marken rücken lückenlos auf 5 bis 7, die Probe `die_null_und_alles_daneben_benennen_keinen_befehl` prüft jetzt 8. Die fünf neuen `Text`-Schlüssel stehen in `Text::ALLE` und in allen drei Tabellen, `{name}` und `{fehler}` in jeder Sprache.

**Untergrenzen-Abschnitt.** `auswurf.rs` nennt jeden hereingeholten Namen aus `objc2_foundation` (`MainThreadMarker`, `NSError`, `NSFileManager`, `NSFileManagerUnmountOptions`, `NSString`, `NSURL`); die Methode steht seit macOS 10.11, unter der Zielmarke 15.0. Die zwei Proben in `crates/krk-core/tests/baum.rs` laufen grün.

**Proben.** Die fünf Proben an `auswurfbezug` decken Treffer, Nicht-Treffer (Datei, Ordner, Ordner auf dem Datenträger, Namensähnlichkeit `Installer2`), Mehrfachauswahl in Zeilenreihenfolge, Wurzel und leere Liste. `der_auswurfweg_reisst_nirgends_ab` und `der_auswurfweg_stellt_auf_dem_hauptfaden_keinen_systemaufruf` sind Quelltextproben in der Bauform der Duplikatproben und nennen ihre Grenzen selbst.

## Befunde

### Low: Ein zweiter Klick schickt einen zweiten Auftrag

`auswurf_beginnen` (`crates/krk-ui/src/appkit/anwendung.rs`) kennt keinen laufenden Auswurf. Bis die Wache das Aushängen meldet, steht der Einhängepunkt weiter in `einhaengepunkte`, und ein zweiter Klick stellt einen zweiten `unmountVolumeAtURL:`. Inference, am Bündel nicht geprüft: die zweite Antwort ist ein Fehler, und je nach Reihenfolge steht zuletzt „ließ sich nicht auswerfen“ da, obwohl der Datenträger fort ist.
Issue: `261016-0253-ein-zweiter-klick-auf-auswerfen-waehrend-des-laufenden-auswurfs-schickt-einen-zweiten-auftrag.md`.

### Low: Einzigkeitsbehauptung im Modulkopf von `appkit` gekippt

`crates/krk-ui/src/appkit/mod.rs` nennt `finder` „das eine Modul dieser Reihe, dessen Aufruf **nichts** zurueckgibt“. `auswurf::auswerfen` gibt seit diesem Commit ebenfalls nichts zurück.
Issue: `261016-0253-der-modulkopf-von-appkit-nennt-finder-das-eine-modul-ohne-rueckgabe-auswurf-ist-ein-zweites.md`.

## Ungeprüftes Risiko (kein Issue)

**Ob KRK den eigenen Auswurf behindert, sagt nur das Bündel.** Die Datenträgerwache holt Tabs bei `NSWorkspaceWillUnmountNotification` vom Datenträger, damit der Auswurf nicht an KRK scheitert (`datentraeger_gewechselt`). speculation: ob `NSWorkspace` diese Meldung auch für einen über `NSFileManager` angestoßenen Auswurf schickt, ist nicht belegt; Apples Doku beschreibt sie für den Finder. Hält ein Teil von KRK dann noch etwas auf dem Datenträger offen (denkbar: ein PDF im Betrachter), weist das System den Auswurf als „in Gebrauch“ ab, und die Statuszeile sagt es. Die Directive ist damit nicht verletzt, der Nutzer bekäme aber eine Abweisung, die der Finder nicht hätte. Prüfen im Abnahmelauf: Tab rechts auf einem Ordner des DMG, PDF davon in der Vorschau, links in `/Volumes` auswerfen.

## Querschnitt

Der Auswurfweg folgt den eingeführten Mustern: Antwort über die Hauptschlange wie der Weckruf des Vorgangs, Delegierter über `NSApplication` statt `Retained`, Auswahl als reine Funktion ohne Dateizugriff wie `duplikatbezug` und `entpackbezug`, Quelltextproben mit benannten Grenzen. Eine zweite Datenträgeraufzählung entsteht nicht.

## Reihenfolge

Kein Befund hält die Auslieferung auf. Beide Issues sind Nacharbeit; die Statuszeile nach Doppelklick zuerst, weil sie den Nutzer falsch unterrichten kann.

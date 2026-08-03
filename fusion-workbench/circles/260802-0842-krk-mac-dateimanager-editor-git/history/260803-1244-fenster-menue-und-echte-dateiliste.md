# Fenster, Menü und echte Dateiliste (Schritt 6)

**Datum:** 260803-1244
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 6 in der Fassung des Nachzugs vom 260803-1208
**Geänderte Dateien:** `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/appkit/{mod.rs,anwendung.rs,fenster.rs,tabelle.rs,menue.rs}` (alle fünf neu), `crates/krk-ui/Cargo.toml`, `Cargo.lock` (nicht in der Dateiliste des Plans, siehe unten)
**Nicht angefasst:** `crates/krk-core/`, `crates/krk-bench/`, `xtask/`, `resources/`, `README.md`, `CLAUDE.md`, `Cargo.toml` des Workspace

## Was entstanden ist

Das AppKit-Gerüst steht, und es zeigt beim Start das Benutzerverzeichnis in vier
Spalten. Fünf Dateien tragen es.

`crates/krk-ui/src/appkit/mod.rs` ist die Grenze. Es trägt `#![allow(unsafe_code)]`
als einzige Ausnahme der Kiste und erklärt im Modulkommentar, warum sie hier und
nirgends sonst steht: Lint-Regeln schlagen in die eingebetteten Module durch, der
Kopf dieser Datei deckt damit den ganzen Teilbaum `src/appkit/` ab.

`crates/krk-ui/src/appkit/anwendung.rs` hält `NSApplication` und den
Anwendungsdelegierten. `starten()` setzt die Aktivierungsrichtlinie auf
`Regular`, hängt das Hauptmenü ein, setzt den Delegierten und ruft `run()`. Der
Delegierte baut in `applicationDidFinishLaunching:` die Oberfläche auf und liest
`$HOME`.

`crates/krk-ui/src/appkit/menue.rs` baut das Hauptmenü von Hand: das
Anwendungsmenü mit "KRK beenden" (Cmd+Q, `terminate:`) und ein Menü "Fenster" mit
"Fenster schließen" (Cmd+W, `performClose:`). Beide Befehle bekommen als Ziel
`nil` und laufen über die Antwortkette.

`crates/krk-ui/src/appkit/fenster.rs` baut das Fenster und seinen Delegierten.

`crates/krk-ui/src/appkit/tabelle.rs` ist das Dateifenster: `NSTableView` in
einer `NSScrollView`, vier Spalten, Datenquelle, Delegierter und die Anbindung an
das Ordnermodell aus Schritt 2.

Dazu kommt die Umstellung der `unsafe`-Regel in `crates/krk-ui/src/main.rs` von
`#![warn(unsafe_code)]` auf `#![deny(unsafe_code)]`, mit nachgezogenem
Modulkommentar.

## Sechs Festlegungen, die der Plan offenließ

**Der Weg der Stapel auf den Hauptfaden ist ein `NSTimer`.** Der Plan schreibt
die gestückelte Übergabe vor, nennt aber kein Verfahren. Der Leser aus `krk-core`
sendet über einen `std::sync::mpsc`-Kanal, und der Hauptfaden muss diesen Kanal
irgendwie abfragen, ohne zu blockieren. Ein Zeitgeber im Takt von 1/60 s räumt
ihn leer, hängt alle wartenden Stapel an und meldet der Tabelle **einmal** je
Takt eine neue Zeilenzahl. Damit hält der Plansatz "höchstens einmal je Bild"
strukturell und nicht durch Sorgfalt. Die Alternativen wären ein Block über
`dispatch_async` auf die Hauptwarteschlange gewesen, was die Abhängigkeit
`block2` oder `dispatch2` in den Workspace gezogen hätte, oder ein Warten auf dem
Hauptfaden, was L1 und L9 verletzt. Der Zeitgeber hängt in
`NSRunLoopCommonModes` und nicht im gewöhnlichen Modus: sonst stünde das Lesen
still, solange der Nutzer blättert oder ein Menü offen hält.

**Der Zeitgeber läuft nur, solange gelesen wird.** Er entsteht mit dem
Lesevorgang und wird beim Abschluss über `invalidate` wieder abgebaut. Das ist
zugleich die Auflösung des Rings, den er sonst bildet: `NSTimer` hält sein Ziel
fest, und die Datenquelle hält den Zeitgeber.

**Zwei Klassen statt einer, und der Delegierte hält die Quelle.** Der Plan nennt
`DateifensterQuelle` und `DateifensterDelegierter` als zwei `define_class!`.
Damit stellte sich die Frage, wer wen hält, denn `NSTableView` hält Datenquelle
und Delegierten beide nur schwach. Die Richtung ist: der Delegierte hält die
Quelle, weil er aus ihr liest; die Gegenrichtung gibt es nicht und damit auch
keinen Zyklus. Die starken Referenzen auf beide sitzen in der Struktur
`Dateifenster`, die der Anwendungsdelegierte hält.

**Die Beschriftung von Größe und Datum kommt von Foundation.** `NSByteCountFormatter`
mit `countStyle = File` und `NSDateFormatter` mit kurzem Datums- und Zeitstil.
Eine eigene Rechnung wäre eine zweite Wahrheit neben der des Systems gewesen, und
sie hätte das Trennzeichen der Spracheinstellung nicht getroffen. Beide
Formatierer entstehen einmal und liegen in den Instanzvariablen des Delegierten:
ein `NSDateFormatter` baut beim Anlegen Kalender- und Sprachtabellen auf und wäre
je Zelle der teuerste Posten im Zeichenweg.

**Ordner tragen in der Größenspalte `--`.** Ein Ordner hat keine eigene Größe,
und die seines Inhalts zu summieren hieße, ihn zu durchlaufen.

**Der Zugriff auf eine Zeile läuft über einen Rückruf.** `mit_zeile(zeile, |eintrag| …)`
statt einer herausgegebenen Referenz. Der Grund ist die Ausleihe der `RefCell`
um das Ordnermodell: gäbe die Quelle eine Referenz heraus, könnte ein Aufrufer
sie über einen AppKit-Aufruf hinweg halten, und `reloadData` würde beim
Rückschlag in `numberOfRowsInTableView:` gegen die noch offene Ausleihe laufen.
Mit dem Rückruf endet die Ausleihe an der Klammer.

## Zwei Stellen, an denen die Werkzeuge etwas anderes verlangten als erwartet

**`define_class!` schreibt den Rückgabetyp um.** Die Protokollmethode
`tableView:viewForTableColumn:row:` liefert `Option<Retained<NSView>>`, aber
innerhalb des Makros ist der Fragezeichenoperator dort nicht verwendbar: das
Makro ersetzt den Typ durch einen eigenen Rückgabetyp, der `FromResidual` nicht
erfüllt. Der Rumpf steht deshalb in einer gewöhnlichen Methode `zellenansicht`,
und die Makromethode ruft nur sie. Ein Kommentar an der Stelle sagt, warum.

**Die Klassen und ihre Instanzvariablen-Strukturen müssen `pub` sein.** Ein
`define_class!` mit privater Klasse lässt sich aus einem Nachbarmodul nicht
benennen, und eine private Instanzvariablen-Struktur an einer öffentlichen Klasse
ist ein Sichtbarkeitsfehler. Beides ist innerhalb der Kiste `krk-ui` folgenlos,
weil die Module unter `appkit` selbst privat sind.

## Abnahme

**Belegt, mit Ausgabe:**

| Prüfung | Ergebnis |
|---|---|
| `cargo build -p krk-ui` | Rückgabewert 0 |
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | Rückgabewert 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | Rückgabewert 0 |
| `cargo test --workspace` | Rückgabewert 0, 82 Prüfungen in sechs Gruppen, davon 2 neue in `krk-ui` |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `cargo clippy --workspace --all-targets` | Rückgabewert 0, keine Warnung |
| `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` | genau eine Datei, `crates/krk-ui/src/appkit/mod.rs` |
| `cargo xtask bundle` (erster Lauf des Tages) | Rückgabewert 0, `Signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)"` |
| `codesign --verify --strict target/KRK.app` | Rückgabewert 0 |
| `codesign -dvv target/KRK.app` | `Authority=Apple Development: Kai Stalmann (FJ8U4B3QAC)`, `flags=0x0(none)` |
| `vtool -show-build-version …/MacOS/krk` | `minos 15.0` |
| Prüfordner mit 100.000 Einträgen | über `krk-bench fixture --eintraege 100000 --seed 3` erzeugt, 100.000 Einträge |

**Dass die Sperre wirklich greift, ist nachgemessen und nicht behauptet.** Ein
probeweise in `main.rs` eingefügter `unsafe`-Block ließ den Bau mit
`error: usage of an unsafe block` scheitern; die Zeile ist danach wieder
entfernt. `deny` bricht den Bau ab, `warn` hätte ihn durchgelassen.

**Dass das Fenster steht, ist über den Fensterserver belegt.** Ein
Einwegwerkzeug unter `/tmp` fragt `CGWindowListCopyWindowInfo` nach den Fenstern
des laufenden Prozesses. Es meldet nach `open target/KRK.app` genau ein Fenster
auf der Bildschirmebene 0, Eigentümer `KRK`, Rahmen 900 × 628 Punkte, mittig
gesetzt. Das ist die Geometrie, die der Code vorgibt: 900 × 600 Inhaltsfläche
plus Titelleiste. Der Prozess lief über drei Minuten ohne Absturz und endete auf
`SIGTERM`.

**Dass die echten Einträge ankommen, ist über eine vorübergehende Sonde belegt.**
Die Sonde druckte beim Abschluss des Lesevorgangs die Zeilenzahl und die ersten
Zeilen, dazu jede Zellenabfrage des Delegierten samt erzeugtem Text. Auf `$HOME`:
12 sichtbare Zeilen bei 13 Einträgen, weil das Dateisystem `Library` als
versteckt kennzeichnet und der Filter des Ordnermodells sie ausblendet, genau wie
der Finder. Die Zellen kamen zuerst in Lesereihenfolge
(`Music`, `Dropbox-qantr`) und nach dem Abschluss in sortierter
(`Applications`, `Desktop`, `Documents`, …), was die Zweistufigkeit aus L2 und L3
im Betrieb zeigt. Beispielzeile aus der Sonde:
`zeile=1 Name="Dropbox-qantr" Größe="44 bytes" Geändert="01.08.26, 13:25" Typ="Verknüpfung"`.
Die Sonde ist vollständig zurückgenommen, `grep -c PROBE` über
`crates/krk-ui/src/appkit/` liefert 0.

**Dass das Menü richtig gebaut ist, ist ebenfalls über eine vorübergehende Sonde
belegt.** Sie las nach dem Start `NSApplication.mainMenu` aus:
Eintrag 0 trägt das Untermenü "KRK" mit "KRK beenden", Kürzel `q`, Maske
Befehlstaste, Aktion `terminate:`, aktiv. Eintrag 1 trägt das Untermenü "Fenster"
mit "Fenster schließen", Kürzel `w`, Maske Befehlstaste, Aktion `performClose:`,
aktiv. AppKit hat von sich aus einen Alternativeintrag "Close All"
(Cmd+Option+W) ergänzt, wie es das bei `performClose:` tut. Auch diese Sonde ist
zurückgenommen.

**Zur Durchblätterprüfung, als Hinweis und nicht als Messung.** Eine
vorübergehende Zeitmessung im Einzugstakt las den Ordner mit 100.000 Einträgen
zweimal warm: erster Stapel nach 35 beziehungsweise 37 ms, vollständig gelesen
und sortiert nach 692 beziehungsweise 687 ms. Die Zusage L10 nennt 4 s warm für
das vollständige Lesen. Das ist keine Abnahme von L10: gemessen wurde ab dem
Aufruf von `ordner_lesen` und nicht ab einem Tastendruck, ohne die zwanzig
Wiederholungen und ohne den Berichtskopf, den `### Frage 5` vorschreibt. Die
Zahlen sagen nur, dass der Einzug im Takt von 1/60 s kein Nadelöhr ist, und diese
Sorge war der Grund, sie zu erheben.

**Nicht belegt:**

- **Das Bild selbst.** Ein Bildschirmfoto ist nicht möglich: `screencapture -l`
  meldet `could not create image from window`, die Freigabe zur Bildschirmaufnahme
  fehlt. Über System Events nach den Fenstern zu fragen scheitert ebenso, mit
  `AppleEvent lieferte eine Zeitüberschreitung (-1712)`, weil die Freigabe für
  Bedienungshilfen fehlt. Was auf dem Bildschirm zu sehen ist, bleibt damit
  ungeprüft; belegt sind Fenster, Geometrie und Zelleninhalte, nicht ihre
  Darstellung.
- **Cmd+Q und Cmd+W im Betrieb.** Dass die beiden Einträge mit den richtigen
  Kürzeln, Aktionen und Zuständen im Menü stehen, ist belegt. Dass macOS sie beim
  Tastendruck auslöst, ist es nicht; dafür bräuchte es dieselbe Freigabe für
  Bedienungshilfen.
- **Das flüssige Durchblättern von 100.000 Einträgen.** Der Ordner ist erzeugt und
  das Lesen gemessen, das Blättern selbst ist eine Bewegung auf dem Bildschirm und
  bleibt ungeprüft.

## Ein abgebrochener zweiter Bündelbau, offen für den Nutzer

Der erste `cargo xtask bundle` des Tages lief ohne Rückfrage durch und ist oben
belegt. Ein zweiter Lauf am Ende der Aufgabe, der dasselbe Bündel aus derselben
Quelle noch einmal bauen sollte, blieb nach 120 s im Zeitlimit hängen:
`codesign` wartete auf die Freigabe des privaten Schlüssels, `SecurityAgent`
hielt den Dialog offen. Der hängende `codesign` ist beendet.

**Zwei Folgen, beide für den Nutzer:**

1. `target/KRK.app` ist jetzt **unsigniert**. `codesign --force` hatte die alte
   Signatur bereits entfernt, bevor es auf die Freigabe wartete;
   `codesign --verify` meldet `code object is not signed at all`. `target/` steht
   in der `.gitignore`, im Repository ist damit nichts beschädigt. Ein erneuter
   `cargo xtask bundle` stellt das Bündel her, sobald der Dialog beantwortet ist.
2. Der Dialog des Schlüsselbunds steht möglicherweise noch auf dem Bildschirm
   (ein Fenster von `SecurityAgent`, 443 × 193 Punkte). Ein Klick auf "Immer
   erlauben" nimmt die Rückfrage dauerhaft weg. Umgangen wurde sie nicht, und auf
   eine Ad-hoc-Signatur ist der Bau nicht ausgewichen.

## Was aufgefallen ist

**Eine Datei außerhalb der Plandateiliste: `Cargo.lock`.** Sie ändert sich um
eine Zeile, weil `krk-ui` jetzt `krk-core` führt. Die Datei ist versioniert
(so begründet in der `.gitignore`), also gehört die Änderung in den Commit. Die
`Cargo.toml` des Workspace blieb unberührt: `krk-core` wird über den Pfad
eingebunden, nicht über `[workspace.dependencies]`, genau wie `krk-bench` es in
Schritt 3 tat.

**Kein Defekt in `crates/krk-core/` gefunden.** Ein Verdachtsfall hat sich
aufgelöst: die Sonde nannte für `~/Desktop` das Änderungsdatum `01.01.70, 03:31`,
was nach einem falsch gelesenen Zeitstempelfeld aussah. `stat -f '%m' ~/Desktop`
liefert 9081, also denselben Wert. Der Ordner trägt auf diesem Gerät wirklich
dieses Datum, der Leser liest richtig.

**Eine Beobachtung, die eher ein Fall für eine spätere Runde ist als ein
Defekt.** `NSByteCountFormatter` schreibt "44 bytes", englisch und klein, während
`NSDateFormatter` in derselben Zelle deutsch formatiert ("01.08.26, 13:25"). Der
Grund ist, dass das Bündel keine Lokalisierung mitbringt: die Zahlenformate
folgen der Regioneinstellung, die Einheitenwörter der Entwicklungssprache aus der
`Info.plist`. Für Schritt 6 ist das folgenlos, das Abnahmekriterium verlangt
richtige Größen und keine bestimmte Sprache. Sobald KRK Oberflächentexte
lokalisiert, löst sich das mit; ein eigener Datensatz dafür scheint mir jetzt
verfrüht.

## Was der nächste Schritt vorfindet

Schritt 7 setzt auf `crates/krk-ui/src/appkit/mod.rs` auf und ergänzt dort
`mod ereignisse;`. Die vier Klassen aus Schritt 6 stehen, `MainThreadMarker`
trägt durch den ganzen Aufbau, und `define_class!` ist an einem Delegierten, an
einer Datenquelle und an einem Zeitgeberrückruf erprobt. Damit sind die vier
Machbarkeitsfragen beantwortet, die der Plan dem frühen Durchstich zuschreibt,
bis auf die fünfte, die Schritt 8 misst.

Der Prüfordner mit 100.000 Einträgen liegt unter `/tmp/krk-pruefordner-gross`
samt Steckbrief. `/tmp` überlebt keinen Neustart; wer die Durchblätterprüfung
nachholt, erzeugt ihn gegebenenfalls neu, deterministisch mit Startwert 3.

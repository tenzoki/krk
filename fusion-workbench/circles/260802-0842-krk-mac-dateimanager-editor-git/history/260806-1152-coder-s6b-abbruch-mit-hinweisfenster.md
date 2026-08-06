# S6b — Abbruch mit Hinweisfenster beim fehlenden Tastenabgriff — coder

**Status:** Complete
**Agent:** coder
**Auftrag:** Nutzerauftrag vom 260806, Planschritt 6b aus `planning/260802-1428_o_plan-navigator-geruest-runde-1.md:639` — bindend ist Möglichkeit 1 aus `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md`.

## Was gebaut wurde

Eine sichere Hülle um `NSAlert` in `crates/krk-ui/src/appkit/hinweis.rs`:
`zeigen(mtm, titel, satz)` zeigt das Fenster über `runModal()` — modal für die
**ganze** Anwendung — und kehrt zurück, sobald der Nutzer bestätigt hat. Sie
gibt nichts zurück: sie stellt keine Frage, also gibt es keine Antwort, an der
sich ein Aufrufer verzweigen könnte.

Drei Festlegungen, alle in der Datei begründet: `NSAlertStyle::Critical` statt
der Vorgabe `Warning`, weil dieses Fenster nur erscheint, wenn KRK danach
aufhört; die eine Schaltfläche mit ausdrücklichem Titel "OK" statt der Vorgabe
von `NSAlert`, deren Beschriftung sonst aus der Lokalisierung von AppKit käme;
und ein `NSApplication.activate()` vor `runModal`, weil der Hinweis gezeigt
wird, **bevor** `makeKeyAndOrderFront` das Hauptfenster nach vorn holt — ohne
den Aufruf stünde er hinter der Anwendung, aus der heraus KRK gestartet wurde.

## Der `None`-Zweig, und warum es zwei davon sind

Der Plan nennt „den `None`-Zweig des Tastenabgriffs". Seit S20 (Umbelegung, C3)
gibt es zwei: `tastenabgriff_einrichten` beim Aufbau der Oberfläche und
`tastenabgriff_nachziehen` nach einer Umbelegung. Beide standen auf
`eprintln!`, beide bedeuten dasselbe — KRK steht ohne Tastatursteuerung da.
Beide gehen jetzt durch **eine** neue Stelle,
`Anwendungsdelegierter::ohne_tastenabgriff_beenden`: Hinweis zeigen, dann
`beenden()`.

Nur den ersten umzustellen hätte zwei Wahrheiten darüber geschaffen, was KRK
ohne Abgriff tut, und der zweite wäre im Bündel weiterhin still gewesen — genau
der Defekt, den `issues/260803-1536_c_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md`
gemeldet hat.

Beendet wird über das vorhandene `beenden()`, also `terminate:` und nicht
`exit`. Beim Start ist das folgenlos; beim Nachziehen nicht, denn dort hat der
Nutzer gearbeitet, und `applicationWillTerminate:` schreibt seine Tabs noch
weg.

## Geänderte und neue Dateien

- `crates/krk-ui/src/appkit/hinweis.rs` (neu, 77 Zeilen) — die Hülle. Der
  Modulkopf grenzt sie gegen `blaetter` ab (vier Unterschiede: kein Fenster
  nötig, sperrt die Anwendung, kehrt erst nach der Bestätigung zurück, danach
  geht nichts weiter) und hält die Vorgabe des Plans fest, dass ein späterer
  modaler Hinweis diese Hülle nimmt und keine daneben stellt.
- `crates/krk-ui/src/appkit/mod.rs` — `mod hinweis;`, „Achtzehn Module" →
  „Neunzehn", Eintrag im Modulbild und im erläuternden Text.
- `crates/krk-ui/src/appkit/anwendung.rs` — `use super::hinweis;`, neue Methode
  `ohne_tastenabgriff_beenden`, beide `None`-Zweige darauf umgestellt, die
  beiden `eprintln!` entfernt; ein Absatz im Modulkopf.

Keine neue Abhängigkeit, keine Testdatei: die Hülle besteht aus AppKit-Aufrufen
ohne eigene Rechnung, und der Fall ist nach dem Planschritt nicht per Kommando
auslösbar.

## Die drei Abnahmekriterien

**1. `grep -n 'eprintln!' crates/krk-ui/src/appkit/anwendung.rs` findet
nichts** — **nicht erfüllbar, Kriterium überholt.** Der gemeinte Zweig ist
weg; übrig bleiben sechs `eprintln!` (Zeilen 634, 643, 652, 2311, 2377, 2387),
alle aus dem Messmodus von S8 und S21, alle mit `std::process::exit` dahinter.
Dort ist die Standardfehlerausgabe der richtige Kanal, weil der Messmodus
ausschließlich unmittelbar aus dem Terminal gestartet wird; ein modales Fenster
an ihrer Stelle ließe den Messlauf auf einen Klick warten. Gemeldet als
`issues/260806-1150_o_abnahmekriterium-von-s6b-ist-an-zwei-stellen-ueberholt.md`.

**2. Der Diff zeigt den Aufruf und das Beenden — erfüllt. `hinweis.rs` ist die
einzige Datei mit einem `NSAlert` — nicht erfüllbar, Kriterium überholt.**
`blaetter/mod.rs` legt seit S13 einen an. Der Planschritt selbst sagt, die
Blätterhülle entstehe erst in S13 und S6b dürfe nicht darauf warten — das
Kriterium beschreibt den Bestand von damals. Zusammenlegen wäre falsch, und
derselbe Schritt sagt zwei Zeilen vorher, warum. Geprüft: genau zwei Dateien
legen einen `NSAlert` an, `blaetter/mod.rs` und `hinweis.rs`; die vier weiteren
Treffer unter `blaetter/` sind Kommentarerwähnungen. Ebenfalls in dem Issue.

**3. Sichtprüfung am laufenden Bündel — erfüllt.** Siehe unten.

## Prüfung am Bündel, mit temporärer Sonde

Der Fall setzt einen fehlgeschlagenen `NSEvent`-Abgriff voraus, den KRK nicht
herbeiführen kann. Eine Befehlszeilenmarke dafür entsteht nach dem Planschritt
ausdrücklich **nicht**. Geprüft wurde deshalb mit einer temporären, nicht
committeten Sonde in zwei Läufen; beide Male `cargo xtask bundle`, Start über
`open -a`, Schirmabzug im Sitzungs-Scratchpad.

**Lauf 1 — der Hinweis selbst.** Sonde: `abgriff_aufsetzen` liefert sofort
`None`. Ergebnis: kein Hauptfenster, stattdessen der modale Hinweis vorn, KRK
als aktive Anwendung in der Menüleiste. Titel „KRK kann keine Tastendrücke
lesen", der Satz darunter, Warnzeichen des Systems, die Schaltfläche "OK" blau
hervorgehoben — also mit der Eingabetaste bedienbar, was hier der Punkt ist:
der Hinweis meldet, dass KRK die Tastatur nicht liest, und darf nicht selbst
nur mit der Maus wegzuklicken sein. Die Umlaute stehen richtig.

**Lauf 2 — das Beenden.** Das Drücken von "OK" ließ sich nicht automatisieren:
`osascript` löste eine Systemabfrage nach Automatisierungsrechten aus. Die
wurde **nicht** erteilt, die Abfrage wurde durch Beenden des anfragenden
Prozesses zurückgenommen, der Schirm ist geprüft sauber. Stattdessen zweite
Sonde: `abgriff_aufsetzen` weiter auf `None`, dazu der Hinweisaufruf
übersprungen, sodass `beenden()` unmittelbar läuft. Ergebnis: der Prozess
beendet sich von selbst, ohne je ein Fenster zu zeigen. Damit ist die offene
Frage beantwortet — `terminate:` greift auch aus
`applicationDidFinishLaunching:` heraus, also vor dem ersten sichtbaren
Fenster. Dass `runModal` nach der Bestätigung zurückkehrt, ist die Zusage der
Schleife selbst und in Lauf 1 nicht gegenprüfbar gewesen.

**Rücknahme.** Beide Sonden aus einer vor dem Eingriff gezogenen Kopie
zurückgesetzt, `grep -rn "SONDE" crates/` findet nichts, `abgriff_aufsetzen`
steht wieder unverändert. Danach `make check` erneut grün, Bündel ohne Sonde
neu gebaut und signiert, und ein **normaler** Start geprüft: Fenster mit
beiden Dateifenstern, Leiste, Vorschau und wiederhergestellter Sitzung, kein
Hinweis — der neue Zweig löst nicht fälschlich aus.

## Bauzustand

`make check` grün (build, test, clippy mit `-D warnings`, fmt). `make bundle`
baut und signiert. Nicht committet, wie beauftragt.

## Nachlauf

- `issues/260806-1150_o_abnahmekriterium-von-s6b-ist-an-zwei-stellen-ueberholt.md`
  (neu) — die beiden überholten Sätze des Abnahmekriteriums, mit Vorschlag für
  die Neufassung. Adressat planner oder reconciler.
- Der Plan trägt S6b noch als `[IN PROGRESS]`, und
  `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md` steht auf `_a_`.
  Beides ist Sache des Orchestrators nach dem Commit; die Entscheidung ist mit
  diesem Schritt vollständig umgesetzt, denn ihre andere Hälfte, die
  Statuszeile, ist mit S12 abgenommen.

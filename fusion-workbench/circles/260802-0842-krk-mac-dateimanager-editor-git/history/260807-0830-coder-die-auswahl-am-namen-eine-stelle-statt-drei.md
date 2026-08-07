# Die Auswahl am Namen: eine Stelle statt drei (Turn 26)

**Agent:** coder
**Status:** Complete
**Quelle:** `issues/260807-0800_c_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-und-die-auswahl-faellt-danach-ersatzlos.md` (hoch), `issues/260807-0800_c_auswahlname-haelt-die-veraltete-modellauswahl-fuer-gueltig.md` (mittel), Durchsicht `reviews/260807-0800-coderev-turn-25-lesestelle-messstrecke-grenzpruefung.md`

## Der Bruch

`5f2e45d` hat die Lesestelle umgestellt: das Ordnermodell behält zwischen `lesevorgang_beginnen` und dem ersten gelieferten Stapel den Bestand des vorigen Laufs. Damit ist ein Zustand entstanden, den es vorher nicht gab — Bestand und Auswahl gehören dem vorigen Lauf, die Generation schon dem neuen —, und die Leser des Modells in `krk-ui` sind ihm nicht gefolgt.

Beide Befunde sind derselbe Bruch an zwei Stellen: die Auswahl wurde in dieser Spanne an einen **Eintragsindex** gehängt, obwohl allein ein **Name** die Spanne übersteht.

## Der Entwurf: eine Stelle, kein neuer Mechanismus

Die tragende Beobachtung ist, dass beide Fragen — „setze die Auswahl auf diesen Namen" und „auf welchem Namen steht die Auswahl" — Lesen und Schreiben **derselben** Sache sind, und dass diese Sache zwei Wohnorte hat: `Ordnermodell::auswahl` (ein Index, fällt mit dem Ersatz) und `Tabinhalt::wunschauswahl` (ein Name, übersteht ihn). Wer arbitriert, muss beide sehen. Das tut genau ein Typ: `Tabinhalt` in `crates/krk-ui/src/tabs.rs`, der den Lesevorgang und die `wunschauswahl` nebeneinander hält. Weder das `Ordnermodell` (kennt die `wunschauswahl` nicht) noch die Ansicht (hat beides nur über zwei Ausleihen) kann das.

Daraus folgen zwei Änderungen, und in **einer einzigen** von beiden steht eine Bedingung:

**Schreiben — `Tabliste::auswahl_auf_namen` (`tabs.rs`), neu.** Fragt **zuerst** `Tabinhalt::liest()`. Steht ein Lesevorgang aus, wandert der Name in die `wunschauswahl`, Antwort `Vorgemerkt`. Erst danach wird der Bestand befragt. `DateifensterQuelle::eintrag_waehlen` (`appkit/tabelle.rs`) ist nur noch die AppKit-Seite: sie setzt bei `Gewaehlt(zeile)` die Zeile in der `NSTableView` und entscheidet nichts mehr. `Tabliste::wunschauswahl_setzen` entfällt, das Enum `Auswahlversuch` zieht von `appkit/tabelle.rs` nach `tabs.rs` — der Wert gehört dorthin, wo er entsteht.

**Lesen — `Tabinhalt::auswahlname` (`tabs.rs`).** Die `wunschauswahl` steht jetzt **vor** der Auswahl des Modells statt dahinter, und zwar **ohne Fallunterscheidung**. Das trägt, weil die `wunschauswahl` genau dann gesetzt ist, wenn die Auswahl des Modells nicht tragfähig ist: gefüllt wird sie nur von einem Aufrufer, der einen Namen vormerkt, herausgenommen nur von `wunschauswahl_anwenden` mit dem Abschluss des Lesevorgangs. Steht sie, steht also ein Lesevorgang aus.

Damit gibt es im ganzen Fix **eine** Bedingung an **einer** Stelle. Der zweite Befund braucht keinen eigenen Zweig, sondern folgt aus einer Invariante, die ohnehin schon galt und jetzt ausgeschrieben ist.

## Die Wahl zwischen `ersetzt_beim_naechsten_stapel` und `liest()`

Genommen wurde `liest()`. Drei Gründe, jeder für sich hinreichend:

1. **`ersetzt_beim_naechsten_stapel` ist die Frage der Ansicht, nicht die der Auswahl.** Ihr Term `!sichtreihenfolge.is_empty()` beantwortet „muss der nächste Stapel als `reloadData` kommen statt als bloße Zeilenzahl" — die Probe `der_ersatz_wird_nur_angekuendigt_wenn_zeilen_fallen` hält genau das fest. Für die Auswahl ist dieser Term Beiwerk und im Fall „alle Einträge ausgeblendet" schlicht falsch: der Bestand gehört dann dem vorigen Lauf, und die Antwort wäre trotzdem „nein". Eine Bedingung mit zwei Bedeutungen läuft auseinander.
2. **Sie deckt die Spanne nicht ganz ab.** Ist der erste Stapel schon da, der Lauf aber noch nicht abgeschlossen, ist `ersetzt_beim_naechsten_stapel` falsch — und eine jetzt gesetzte Auswahl fällt trotzdem, weil `wunschauswahl_anwenden` beim Abschluss die Vormerkung einlöst und die frische Auswahl überschreibt. `liest()` deckt beide Wege, auf denen eine Auswahl in dieser Spanne verlorengeht.
3. **`liest()` wohnt am richtigen Typ.** Es sitzt auf `Tabinhalt`, also neben der `wunschauswahl`, die die Alternative ist. `ersetzt_beim_naechsten_stapel` sitzt am `Ordnermodell`, das von der `wunschauswahl` nichts weiß.

## Die dritte Stelle: `alle_namen`

Nicht geändert, und das mit Absicht. `DateifensterQuelle::alle_namen` liefert den Bestand für die Kollisionsvorschau des Stapel-Umbenennens. Während eines laufenden Lesevorgangs ist das der Bestand des vorigen Laufs — **desselben Ordners**, einen Augenblick alt, denn eine Auffrischung wechselt den Ordner nicht.

Das ist kein Defekt und braucht keinen: bis zum 260807 war die Antwort in derselben Spanne eine **leere** Liste, weil `aktiven_neu_lesen` den `Tabinhalt` vorab ersetzte. Ein alter Bestand ist die bessere Näherung, nicht die schlechtere; `5f2e45d` hat diese Stelle verbessert, nicht gebrochen. Die Wahrheit über vergebene Namen hält ohnehin das Dateisystem, wie `umbenennen_ausfuehren` (`appkit/anwendung.rs:1897-1900`) ausschreibt. Was fehlte, war die ausgeschriebene Begründung — die steht jetzt am Kopf von `alle_namen`. Ein dritter Sonderfallzweig wäre die Sammlung gewesen, die „supersimpel" ausschließt.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-ui/src/tabs.rs` | `Auswahlversuch` (neu hier, `Gewaehlt` trägt die Zeile); `Tabliste::auswahl_auf_namen` ersetzt `wunschauswahl_setzen`; `Tabinhalt::auswahlname` stellt die `wunschauswahl` voran; drei Proben |
| `crates/krk-ui/src/appkit/tabelle.rs` | `Auswahlversuch` entfernt (Import aus `crate::tabs`); `eintrag_waehlen` auf die AppKit-Seite zusammengezogen; `alle_namen` um die Begründung ergänzt |
| `crates/krk-ui/src/appkit/anwendung.rs` | Import umgestellt, `Gewaehlt(_)` im Messstrecken-Zweig, der Kommentar am Stapel-Umbenennen sagt jetzt, was wirklich geschieht |
| `crates/krk-ui/src/messmodus.rs` | ein Doc-Verweis auf den neuen Ort des Enums |

## Proben

Drei neue in `crates/krk-ui/src/tabs.rs`. Die Entscheidung sitzt seit dieser Änderung unter der Fensterebene und ist damit **ohne Fenster** prüfbar — das war einer der Gründe, sie dorthin zu ziehen.

- `der_erste_neue_name_eines_stapel_umbenennens_wird_vorgemerkt` — der deterministische Fall: `IMG_1.jpg, IMG_2.jpg` wird zu `IMG_2.jpg, IMG_3.jpg`, Auffrischung und Auswahl im selben synchronen Aufruf. Erwartet `Vorgemerkt` und den Namen in der `wunschauswahl`.
- `eine_zweite_auffrischung_laesst_den_vorgemerkten_namen_stehen` — der mittlere Befund: zwei `aktiven_neu_lesen` vor dem ersten Stapel, mit einem Vormerken dazwischen.
- `ohne_lesevorgang_waehlt_der_name_seine_zeile` — die Gegenprobe: ohne laufenden Lesevorgang bleibt es beim alten Weg, `Gewaehlt(1)` beziehungsweise `Unbekannt`.

**Beide Defektproben schlagen gegen die alte Reihenfolge fehl.** Nachgewiesen, nicht behauptet: die alte Fassung beider Methoden wurde vorübergehend wieder eingesetzt, und der Lauf meldete genau diese zwei Fehlschläge bei zwölf grünen Proben.

Was damit **nicht** gedeckt ist: der Weg von `DateifensterQuelle::eintrag_waehlen` in die `NSTableView`. Er ist ohne Fenster nicht erreichbar. Er entscheidet aber auch nichts mehr — er setzt bei `Gewaehlt(zeile)` die Zeile und reicht den Wert durch.

## Zeitzusagen

Nicht berührt, nachgeprüft statt geglaubt. L2, L3 und L10 hängen am Weg `lesen_starten` → `anhaengen` → `abschliessen`; die kopflose Messstrecke aus `krk-bench` fährt ihn über `krk-core` und rührt `krk-ui/src/tabs.rs` nicht an. Der Diff berührt `krk-core` mit keiner Zeile und den Einzugstakt (`einzug_je_tab`) ebenfalls nicht. `auswahlname` läuft je Auffrischung und je Sitzungsabgleich einmal und tut jetzt **weniger** als vorher (eine Option-Klonung statt einer Modellabfrage). Im Messmodus steht `Handlung::Auswaehlen` nur hinter `Bedingung::AktivZeigt`, die `!liest_aktiv` verlangt — dort nimmt `auswahl_auf_namen` denselben Zweig wie zuvor. Eine Nachmessung im Format von `messungen/260807-0002-…` ist damit gegenstandslos.

## Abnahme

`make check` grün (Bau, 12 Probenreihen, Clippy mit `-D warnings`, `fmt --check`), Ausstieg 0.

## Was daneben auffiel

**`issues/260807-0219_o_drei-aufrufer-von-eintrag-waehlen-werfen-den-auswahlversuch-weg.md` bleibt offen.** Die drei Aufrufer in `appkit/anwendung.rs` (`:1885` Anlegen, `:1908` Umbenennen, `:2316` Stapel-Umbenennen) verwerfen den Rückgabewert weiterhin. Diese Änderung berührt den Befund nicht — sie macht den Wert nur zuverlässiger, nicht lauter.

**Ein Verhaltensunterschied, den `auswahlname` mitbringt.** Klickt der Nutzer mitten in einem laufenden Lesevorgang auf eine Zeile des schon gelieferten Teils, meldet `auswahlname` ab sofort die `wunschauswahl` statt des Klicks. Das ist keine Verschlechterung, sondern eine frühere Auskunft über dasselbe Ergebnis: `wunschauswahl_anwenden` überschreibt diesen Klick beim Abschluss ohnehin. Ob dieses Überschreiben richtig ist, ist eine eigene Frage und steht in `decisions/260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md` in ihrer Nachbarschaft.

**Der Kommentar am Stapel-Umbenennen war schon vorher ungenau.** Er sagte zu, bei einer gescheiterten Umbenennung bleibe „die Auswahl stehen". Tatsächlich bleibt sie leer, weil `eintrag_waehlen` die `wunschauswahl` des Aufrufers überschreibt — auch schon vor `5f2e45d`. Der Kommentar sagt das jetzt.

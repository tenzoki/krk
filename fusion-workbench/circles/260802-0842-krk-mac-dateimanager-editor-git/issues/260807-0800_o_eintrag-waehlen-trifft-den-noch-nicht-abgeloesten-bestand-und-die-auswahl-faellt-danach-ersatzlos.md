eintrag_waehlen trifft den noch nicht abgelösten Bestand, und die Auswahl fällt danach ersatzlos

---

Seit `5f2e45d` behält das Ordnermodell zwischen `lesevorgang_beginnen` und dem
ersten gelieferten Stapel den Bestand des vorigen Laufs. In dieser Spanne
findet `DateifensterQuelle::eintrag_waehlen` einen Namen im **alten** Bestand,
meldet `Auswahlversuch::Gewaehlt` und setzt die Auswahl auf eine Zeile, die der
Ersatz gleich darauf wegräumt. Weil `Gewaehlt` keine `wunschauswahl` hinterlegt,
kommt die Auswahl nicht wieder.

Vorher war das nicht möglich: `leeren` hatte das Modell beim Start des
Lesevorgangs geleert, der Name war nicht zu finden, und `eintrag_waehlen` fiel
in den Zweig `Vorgemerkt`, der den Namen über `wunschauswahl` in den Abschluss
trägt.

---

## Die Stellen

- `crates/krk-ui/src/appkit/tabelle.rs:1082-1103` — `eintrag_waehlen` fragt
  zuerst den Modellbestand (`index_von_namen`) und erst danach, ob noch ein
  Lesevorgang läuft. Bei vorgemerktem Ersatz sind beide Bedingungen zugleich
  wahr, und die erste gewinnt.
- `crates/krk-ui/src/appkit/tabelle.rs:874-883` — `zeile_setzen` setzt die
  Auswahl über `auswahl_merken` in das Modell und rührt `wunschauswahl` nicht
  an.
- `crates/krk-core/src/verzeichnis/modell.rs` — `ersatz_einloesen` setzt
  `auswahl = None`, sobald der erste Stapel kommt.

## Der deterministische Fall: Stapel-Umbenennen

`crates/krk-ui/src/appkit/anwendung.rs:2303-2317`:

```rust
for ordner in vorgang.ordner() {
    auffrischung::ordner_neu_lesen(self, &ordner);
}
match &vorgang.art {
    Art::UmbenennenImStapel { neue_namen } => {
        if let Some(erster) = neue_namen.first() {
            self.dateifenster(vorgang.seite).quelle().eintrag_waehlen(erster);
        }
    }
```

Beide Zeilen laufen im selben synchronen Aufruf. Der Einzugstakt kommt
dazwischen nicht zum Zug, der Ersatz steht also **garantiert** noch aus, und
`eintrag_waehlen` sieht den vollständigen Bestand von **vor** der Operation. Ein
Stapel-Umbenennen schiebt seine Auffrischung außerdem auf
(`crates/krk-ui/src/auffrischung.rs:183`), es ist also der erste Lesevorgang
seit dem Beginn des Vorgangs.

Steht `neue_namen[0]` schon im alten Bestand, greift der Fehler. Das ist bei
einer Umnummerierung nach oben der Normalfall: `IMG_1.jpg, IMG_2.jpg` wird zu
`IMG_2.jpg, IMG_3.jpg`, und `IMG_2.jpg` stand vorher da. Die Auswahl landet dann
auf dem alten Eintrag, fällt mit dem ersten Stapel, und am Abschluss gewinnt die
`wunschauswahl`, die `aktiven_neu_lesen` kurz zuvor auf die **vorherige**
Auswahl gesetzt hat. Der Kommentar bei `:2307-2311` sagt zu, dass die Auswahl
danach auf dem ersten neuen Namen steht; sie tut es in diesem Fall nicht.

## Der zweite Weg: der Sprung aus der Zwischenablage (C10)

`crates/krk-ui/src/appkit/tabelle.rs:1063-1067` — `eintrag_anspringen` meldet nur
bei `Unbekannt` etwas in die Statuszeile. Trifft der Sprung die Spanne mit
vorgemerktem Ersatz, meldet `eintrag_waehlen` `Gewaehlt`, der Nutzer sieht die
Zeile kurz markiert, und der erste Stapel nimmt sie wieder weg. Die Spanne ist
hier ein Rennen und kein sicherer Fall; sie reicht von `lesen_starten` bis zum
ersten Takt des Einzugs und wird auf einem langsamen oder entfernten
Datenträger länger.

Dieselbe Spanne trifft die Messstrecke
(`crates/krk-ui/src/appkit/anwendung.rs:2559-2580`): sie wertet `Gewaehlt` als
Erfolg, obwohl die Auswahl gleich darauf fällt.

## Denkbarer Weg

`eintrag_waehlen` fragt vor dem Bestand, ob ein Ersatz aussteht. Das Modell
beantwortet das bereits über `Ordnermodell::ersetzt_beim_naechsten_stapel`
(`crates/krk-core/src/verzeichnis/modell.rs`), das für die Ansicht ohnehin da
ist. Steht ein Ersatz aus, gehört der Name in die `wunschauswahl` und die
Antwort ist `Vorgemerkt` — also genau der Weg, den die Stelle vor dem 260807
von selbst nahm. Ein neuer Mechanismus entsteht nicht.

Zu prüfen ist dabei, ob `ersetzt_beim_naechsten_stapel` die richtige Frage ist
oder ob es `liest()` sein muss: `ersetzt_beim_naechsten_stapel` ist falsch,
wenn die Sichtreihenfolge leer ist, obwohl der Bestand noch dem alten Lauf
gehört (alle Einträge des vorigen Ordners ausgeblendet). In diesem Fall findet
`index_von_namen` zwar noch, `zeile_von` aber nicht, und der Zweig `Gewaehlt`
wird nicht erreicht.

## Dringlichkeit

Hoch für den Stapel-Umbenennen-Fall: er ist deterministisch, nutzersichtbar und
widerspricht einer im Code ausgeschriebenen Zusage. Mittel für die beiden
Rennen. Keine der zehn Zeitzusagen aus C8 ist berührt.

**Betrifft:** `crates/krk-ui/src/appkit/tabelle.rs`,
`crates/krk-ui/src/appkit/anwendung.rs`.

**Aufgefallen bei:** der inkrementellen Durchsicht nach Turn 25 der Sitzung
260806-2257, Diff `f9a0462..HEAD`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0800_o_auswahlname-haelt-die-veraltete-modellauswahl-fuer-gueltig.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0219_o_drei-aufrufer-von-eintrag-waehlen-werfen-den-auswahlversuch-weg.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-1337_c_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`

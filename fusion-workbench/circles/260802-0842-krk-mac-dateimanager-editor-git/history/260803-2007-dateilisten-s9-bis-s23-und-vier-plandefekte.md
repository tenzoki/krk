# Dateilisten von S9 bis S23, drei Defekte an Schritt 7 und der Rückweg nach Cmd+W

**Datum:** 2026-08-03, 20:07 bis 20:22
**Agent:** planner
**Status:** Complete
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

---

## Auftrag

Fünf Meldungen am Plantext abarbeiten: die Durchsicht der Dateilisten von S9
bis S23 unter der erweiterten Regel, drei Defekte an Schritt 7 aus dessen
Umsetzung, und der fehlende Rückweg nach Cmd+W aus der Codeprüfung. Kein Code,
kein Commit, keine Änderung an `[DONE]`-Vermerken, Schrittnummern oder den zehn
Zahlen aus C8.

## Was die Durchsicht der fünfzehn Dateilisten ergeben hat

Zwölf der fünfzehn Schritte haben Einträge dazubekommen, drei standen sauber.
Sauber sind S9, das eine Datentabelle anlegt und nirgends abliest, S10, das die
Ablage auf einer Kiste aufbaut, deren Abhängigkeiten seit S1 stehen, und S22,
das eine Messreihe fährt und zwei Dokumente schreibt.

| Schritt | Was dazugekommen ist |
|---|---|
| S11 | Ereignisabgriff aus S7 (Ablösung der verdrahteten Tabelle), `default-keymap.toml`, `normalisierung.rs`, `ablage/{pfade,atomar}.rs` |
| S12 | `fenster.rs`, `anwendung.rs`, `default-keymap.toml` |
| S13 | `ereignisse.rs`, `tasten/mod.rs`, `verzeichnis/modell.rs`, `sortierung.rs`, `default-keymap.toml` |
| S14 | `Cargo.lock`, `fenstermodell.rs`, `tabelle.rs`, `anwendung.rs` |
| S15 | `verzeichnis/leser.rs`, `verzeichnis/eintrag.rs` |
| S16 | `auffrischung.rs`, `papierkorb.rs`, `tabelle.rs`, `default-keymap.toml` |
| S17 | `operation/umbenennen.rs`, `verzeichnis/modell.rs`, `kommandos/operationen.rs` |
| S18 | `aufteilung.rs`, `fenstermodell.rs`, `tests/ablage.rs` |
| S19 | `aufteilung.rs`, `tableiste.rs`, `tabelle.rs`, `verzeichnis/eintrag.rs` |
| S20 | `tasten/belegung.rs`, `tasten/konflikt.rs`, `default-keymap.toml` |
| S21 | `krk-ui/src/main.rs`, `krk-bench/src/main.rs`, `ablage/sitzung.rs`, `anwendung.rs` |
| S23 | `xtask/src/bundle.rs`, `xtask/src/sign.rs` |

Der Datensatz hatte S13, S16, S17, S19 und S21 als wahrscheinlichste Treffer
benannt. Alle fünf hatten einen Fund, was die Vermutung bestätigt, ohne sie zu
erschöpfen: sieben weitere Schritte hatten ebenfalls einen.

Zwei wiederkehrende Formen sind dabei sichtbar geworden, und beide stehen jetzt
im Kopf von `## Implementierungsschritte`. Die erste ist die Datei, in der ein
Schritt einen vorhandenen Mechanismus **ablöst** statt einen zweiten
danebenzustellen; das deutlichste Beispiel ist S11, das die fünf fest
verdrahteten Tasten aus S7 durch die Belegung ersetzt und dafür den Abgriff in
`ereignisse.rs` umhängen muss. Die zweite ist der **Einhängepunkt**, den ein
früher Schritt anlegt und ein späterer benutzt; S14 legt `auffrischung.rs` mit
der ausdrücklichen Bemerkung an, der zweite Auslöser komme "später aus S16",
und S16 nannte die Datei bisher nicht.

Der Kopf hält jetzt auch die Grenze der Durchsicht fest: sie sieht die
Abhängigkeiten, die der Plantext benennt, nicht die, die erst der Übersetzer
zeigt. S7 und S8 haben je eine Kiste gebraucht, die keine Zeile des Plans
vorhergesagt hatte, `block2` und `objc2-quartz-core`.

## Drei Befunde, die über eine Dateiliste hinausgehen

Sie sind als eigene Meldungen abgelegt statt in dieser Bearbeitung
mitzuerledigen, weil jeder eine Entwurfsentscheidung verlangt.

`260803-2007_o_s16-nennt-keinen-mechanismus-fuer-die-buendelung-der-fortschrittsmeldungen.md`.
S16 sagt zu, den Fortschritt höchstens einmal je Bild anzuzeigen, und nennt
keinen Takt dafür. Der naheliegende Griff wäre der `CADisplayLink` aus S8, und
genau ihn hätte die Dateiliste stillschweigend gewählt, hätte ich `bildtakt.rs`
als `(lesend)` eingetragen. Ein `CADisplayLink` weckt den Prozess aber an jeder
Bildgrenze, unabhängig davon, ob es etwas zu zeichnen gibt; für einen Messlauf
von zwanzig Wiederholungen ist das folgenlos, für einen stundenlang laufenden
Dateimanager nicht.

`260803-2007_o_die-metadatenvorschau-aus-c6-verlangt-rechte-die-der-eintrag-nicht-traegt.md`.
C6 verlangt in der Metadatenanzeige die Rechte, und `Eintrag` führt kein
Rechtefeld. Entweder wächst die abgenommene Datenstruktur aus S2, was bei
100.000 Einträgen auf L3 und L10 durchschlägt, oder S19 erhebt die Rechte für
den einen angezeigten Eintrag mit einem eigenen Systemaufruf.

`260803-2007_o_s14-bindet-fsevents-ohne-das-framework-coreservices-zu-verlinken.md`.
S14 vergleicht die FSEvents-Bindung mit der von `getattrlistbulk`. Der
Vergleich trägt nicht bis zum Binder: `getattrlistbulk` liegt in `libSystem`,
`FSEventStreamCreate` in `CoreServices`, das niemand verlinkt. Geprüft gegen
die Symboltabellen des SDK der Command Line Tools; `CoreFoundation`, das
`objc2-core-foundation` mitbringt, führt das Symbol nicht.

## Die drei Defekte an Schritt 7

Die Dateiliste trägt jetzt die fünf gemeldeten Einträge, dazu die `Cargo.lock`,
die die neue Abhängigkeit `block2` mechanisch mitzieht.

Der Fehler im Abnahmekommando reichte weiter, als die Meldung annahm. Sie
vermutete, S2 und S15 trügen dasselbe Muster, seien aber ungefiltert; für S15
stimmte das nicht. `cargo test -p krk-core <wort>` filtert über die
Prüfungsnamen und wählt nicht das Testprogramm. Nachgezogen sind sieben
Kriterien: S7, S10, S11, S12, S13, S15 und S17 verlangen jetzt `--test`.
Ungefiltert und damit unberührt bleiben S1 mit `cargo test --workspace` und S2.

S12 war der schwerste Fall. Es verlangte `cargo test -p krk-core sitzung`, und
ein Testprogramm dieses Namens wird es nie geben: die Prüfungen wachsen nach
der eigenen Dateiliste des Schrittes in `crates/krk-core/tests/ablage.rs`
hinein. Ein Namensfilter hätte hier je nach Benennung der Prüfungen zufällig
funktioniert oder ohne Fehler nichts ausgeführt. Das Kriterium steht jetzt auf
`--test ablage`.

Das Tastenprotokoll startet das Binärprogramm im Bündel unmittelbar statt über
`open`. Der Absatz zum Kriterium hält den Grund fest, damit der Startweg nicht
bei der nächsten Durchsicht zurückgezogen wird. Die Prüfung von S6 verwendet
`open` weiter und zu Recht: sie sieht ein Fenster an und liest keine Ausgabe.

## Der Rückweg nach Cmd+W

Zwei Dinge waren zu trennen: dass kein Planschritt die Sackgasse aufnimmt, und
welche der beiden Antworten gilt.

Das Erste ist eine Planungsfrage und hier entschieden. **S12 nimmt den Punkt
auf.** S6 hätte ihn thematisch getragen, ist aber abgenommen und mit `[DONE]`
vermerkt, und sein Kriterium ist wörtlich erfüllt: Cmd+W schließt das Fenster,
und dass danach nichts kommt, prüft es nicht ab. S12 ist der Schritt, der das
Fenstermodell anlegt, hat nicht begonnen, und beide Antworten sind dort am
billigsten; die zweite fällt sogar mit seiner ohnehin anstehenden Arbeit
zusammen. S12 trägt jetzt die Beschreibung der Sackgasse, den
Entscheidungsdatensatz als Abhängigkeit und ein Abnahmekriterium, das ein
laufendes KRK ohne Fenster und ohne Rückweg ausschließt.

Das Zweite ist eine Festlegung über das Verhalten der Anwendung und gehört dem
Nutzer. Der Auftrag verwies auf C1 und C7 des Specs, ob die Antwort dort schon
feststehe. Sie steht nicht: C1 regelt das Schließen des letzten **Tabs** und
sagt dafür ausdrücklich, dass das Dateifenster stehen bleibt, C7 regelt das
Ein- und Ausblenden der vier Bereiche innerhalb des einen Fensters und sichert
zu, dass mindestens ein Dateifenster sichtbar bleibt. C7 ist der nächste
Verwandte der Frage, greift aber eine Ebene tiefer: es schützt die Bereiche im
Fenster, nicht das Fenster.

Angelegt ist deshalb
`decisions/260803-2007_o_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
mit beiden Möglichkeiten, ihren Dateifolgen und einer Empfehlung für die erste:
Runde 1 kennt genau ein Fenster, denn die zwei Dateifenster aus C1 sind
Bereiche innerhalb eines Fensters. Ein zweites Fenster einzuführen, nur um
einen Rückweg aus einem Zustand zu schaffen, den es ohne den Rückweg gar nicht
geben müsste, wirft Folgefragen auf, die keine Fähigkeit dieser Runde stellt:
teilen sich zwei Fenster eine Sitzung, und was heißt "das aktive Dateifenster"
aus C1 bei zwei Fenstern mit je zwei Dateifenstern.

## Geänderte Dateien

- `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` — Kopfzeile,
  Nachzugsvermerk, Regelabsatz im Kopf von `## Implementierungsschritte`, die
  Dateilisten von S7 und S11 bis S23, sieben Abnahmekriterien, zwei Absätze in
  S12, der Abschnitt `## Angelegte Defekte und Entscheidungen`.
- Geschlossen mit `Resolved:` und Marker `_c_`:
  `issues/260803-1309_c_dateiliste-von-schritt-7-nennt-fuenf-noetige-dateien-nicht.md`,
  `issues/260803-1309_c_abnahmekommando-von-schritt-7-filtert-nach-testnamen-statt-nach-datei.md`,
  `issues/260803-1309_c_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`,
  `issues/260803-1536_c_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md`,
  `issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`.
- Neu angelegt: die drei Meldungen oben und der Entscheidungsdatensatz
  `decisions/260803-2007_o_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`.

Kein Code angefasst, kein Commit. Die Diagramme sind unverändert, der
Abhängigkeitsgraph steht weiter auf 24 Knoten und 34 Kanten; der
Entscheidungsdatensatz ist kein Schritt und trägt keine Kante.

## Was liegen blieb

Der Auftrag kündigte einen Defekt des `coder` zu `### Frage 2` an: die
Beschreibung des abgebrochenen Lesevorgangs nennt den unwirksamen Mechanismus,
die Generationsprüfung, statt des wirksamen, des Fallenlassens des Empfängers.
Der Datensatz lag am Ende dieser Sitzung nicht vor; der Nutzer reicht ihn nach.

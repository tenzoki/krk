# Sieben Defekte aus der Codeprüfung des AppKit-Durchstichs

**Datum:** 260803-2025
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md` und die sieben dort angelegten Defektdatensätze
**Geändert:** `crates/krk-core/src/verzeichnis/modell.rs`, `crates/krk-core/src/tasten/normalisierung.rs`, `crates/krk-core/tests/tasten.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/appkit/ereignisse.rs`
**Neu angelegt:** kein Quelltext; ein Entscheidungsdatensatz und drei Defektdatensätze, unten genannt
**Nicht angefasst:** die Plandatei, der Spec, `xtask/`, `crates/krk-bench/`, `resources/`, `README.md`, `CLAUDE.md`, `crates/krk-core/src/verzeichnis/leser.rs`, `crates/krk-core/tests/verzeichnis.rs`
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen, wie für den `coder` vorgesehen. Ein Langform-Schreibprofil gibt `fusion-rules` für diesen Agenten nicht aus.

## Zwei Blöcke, getrennt zu committen

Der Auftrag verlangte den Auswahl-Fehler für sich, weil er als einziger das
Verhalten betrifft. Die Dateien überschneiden sich in genau einer Datei, und
diese eine Stelle ist unten benannt.

| Block | Dateien |
|---|---|
| Auswahl (Verhalten) | `crates/krk-core/src/verzeichnis/modell.rs`, `crates/krk-ui/src/appkit/tabelle.rs` |
| Belegkette | `crates/krk-core/src/tasten/normalisierung.rs`, `crates/krk-core/tests/tasten.rs`, `crates/krk-ui/src/appkit/ereignisse.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-core/src/verzeichnis/modell.rs` (eine Dokumentationsstelle) |

`tabelle.rs` trägt Änderungen aus beiden Blöcken; das ließ sich nicht vermeiden,
weil vier der sieben Defekte in dieser Datei sitzen. In `modell.rs` gehört genau
eine Stelle zum zweiten Block: die Dokumentation von `Ordnermodell::generation`
sagte den Satz, den der sechste Defekt betrifft, und wäre nach dem Entfernen der
Generationsprüfung falsch stehen geblieben.

## Der Auswahl-Fehler

**Die Auswahl wohnt jetzt im `Ordnermodell` und hängt dort am Eintragsindex.**
Der Datensatz schlug ein Feld in `QuelleIvars` vor, also in der Oberfläche. Sie
liegt stattdessen im Kern, aus zwei Gründen.

Der erste ist der Ort des Problems. Umsortiert wird in `sicht_neu_aufbauen`, und
dorthin führen drei öffentliche Wege: `abschliessen` am Ende eines
Lesevorgangs, `sortierung_setzen` über den Spaltenkopf aus C2 und
`verstecke_ausblenden_setzen`. Läge die Auswahl in der Oberfläche, bräuchte jeder
dieser Wege seine eigene Sicherung. Der Modulkopf des Modells sagt seit Schritt 2
zu, dass die Auswahl einen Sortierwechsel übersteht; jetzt hält das Modell diese
Zusage selbst, statt sie einem Aufrufer zu überlassen.

Der zweite Grund ist die Prüfbarkeit. Eine Auswahl in `QuelleIvars` ist ohne
Fenster nicht prüfbar, und der Auftrag verlangte eine Prüfung, die den Fehler
fängt.

Dazugekommen sind `Ordnermodell::auswahl()`, `auswahl_setzen()` und
`auswahl_zeile()`; `leeren` hebt die Auswahl auf, weil sie auf einen Eintrag des
verlassenen Ordners zeigt. In `tabelle.rs` übersetzt `auswahl_merken` eine Zeile
in ihren Eintrag und `auswahl_zeigen` einen Eintrag zurück in eine Zeile. Beide
Richtungen laufen über je eine Stelle:

```text
Tastatur  ──> auswahl_verschieben ──┐
                                    ├──> auswahl_merken ──> Modell (Eintragsindex)
Maus ──> tableViewSelectionDidChange:┘

einziehen / lesen_abbrechen / ordner_lesen ──> auswahl_zeigen ──> NSTableView
```

Der Delegiertenrückruf `tableViewSelectionDidChange:` ist neu. Ohne ihn wäre der
Fehler für die Maus unbehoben geblieben, und der Entwurf nennt die Maus
ausdrücklich als zweiten Bedienweg.

**Eine Falle steckte in der Reihenfolge.** `einziehen` hält den ausgewählten
Eintrag in einer lokalen Bindung, bevor es `reloadData` ruft. Fasst AppKit dabei
die Auswahl der Tabelle an, löst das den Rückruf aus, und der fände die schon
sortierte Sicht vor und schriebe den falschen Eintrag ins Modell. Mit der
lokalen Bindung davor ist die Reihenfolge unabhängig davon, was `reloadData` mit
der Auswahl macht.

### Wie nachgewiesen ist, dass die Prüfung den Fehler fängt

`die_auswahl_ueberlebt_das_sortieren_am_ende_des_lesevorgangs` in `modell.rs`
baut den Fall des Datensatzes nach: `zzz.txt` steht in Lesereihenfolge vor
`Applications`, die Auswahl liegt auf `zzz.txt`, dann kommt `abschliessen`. Ein
Sortierschlüssel ist dafür nicht nötig, weil Ordner in jeder Sortierung vor
Dateien stehen; die Probe hängt damit an keiner nachgebauten Rechnung.

Nachgewiesen ist die Wirksamkeit nicht durch grünes Licht, sondern durch einen
Lauf gegen den defekten Mechanismus. Dafür lieferte `auswahl_zeile` versuchsweise
die gemerkte Zeilennummer unverändert zurück, also genau das, was die
`NSTableView` vorher hielt. Ergebnis:

```text
thread 'verzeichnis::modell::tests::die_auswahl_ueberlebt_das_sortieren_am_ende_des_lesevorgangs'
panicked at crates/krk-core/src/verzeichnis/modell.rs:300:9:
assertion `left == right` failed
  left: Some("Applications")
 right: Some("zzz.txt")
```

Das ist wörtlich der Ausgang, den der Defektdatensatz beschreibt. Die dritte
Probe, `eine_ausgeblendete_auswahl_kommt_beim_einblenden_zurueck`, scheitert
gegen denselben Stand ebenfalls. Danach zurückgenommen.

Die Probe enthält außerdem eine eigene Gegenkontrolle: sie prüft mit `assert_ne!`,
dass unter der alten Zeilennummer nach dem Sortieren wirklich ein anderer Eintrag
steht. Ohne diese Zeile könnte sie grün bleiben, weil die Sortierung nichts
bewegt hat.

### Am laufenden Bündel nachgefahren

`cargo xtask bundle`, dann
`target/KRK.app/Contents/MacOS/krk --messmodus spannen` mit den Prüfordnern zu
10.000 und 100.000 Einträgen, zweimal. Das sind je Lauf 43 Lesevorgänge und 20
Pfeil-ab-Drücke durch den neuen Weg. Kein doppelter `RefCell`-Zugriff, kein
Absturz. Alle zwanzig L1-Werte kamen zustande, und die zählen nur, wenn die
Auswahl wirklich umspringt. Die Zahlen liegen im Rahmen des Messberichts vom
260803-1641: L2 im Mittel 41,3 ms, L10 vollständig im Mittel 888 ms.

Der Grund für diesen Lauf war nicht die Vollständigkeit, sondern ein konkretes
Risiko: der neue Delegiertenrückruf feuert mitten in AppKit-Aufrufen, und eine
gehaltene Ausleihe des Modells wäre dort der doppelte Zugriff, den die Prüfung
des `coderev` als sorgfältigste Stelle des Durchstichs bezeichnet hat. Jeder
Aufrufweg ist daraufhin gelesen; der Lauf ist die Gegenprobe dazu.

## Die Belegkette

**Der Modulkopf der Normalisierung** trennt jetzt, was gemessen ist, von dem, was
daraus abgeleitet ist. Für `function` bei den F-Tasten bleibt die Messung der
Beleg, aber nur für den gemessenen Fall mit gehaltener fn
(`spikes/fn-tasten/messung-A.txt:17-19`). Dass ein nacktes F3 dasselbe Ereignis
liefert, steht als Annahme da, mit Verweis auf das "NICHT MESSBAR AUF DIESEM
GERÄT" der Neuauswertung. Für den Zehnerblock ist die Messdatei als Beleg
gestrichen; an ihrer Stelle steht der SDK-Kopf selbst, `NSEvent.h`, Zeilen 173 und
175, am 260803-2025 nachgesehen. Er trägt "Set if any key in the numeric keypad is
pressed" und "Set if any function key is pressed", also beide Male eine
Eigenschaft der gedrückten Taste und keine gehaltene Zusatztaste. Was er nicht
trägt, ist die Aussage über die Pfeiltasten; sie steht jetzt als ungemessen da.

Der Kopf schreibt zusätzlich aus, warum die Löschung an keiner der beiden offenen
Fragen hängt. Das war der eigentliche Mangel: der alte Text brauchte die
Messaussagen als Stütze, obwohl C3 allein trägt.

**Die zwei SAFETY-Kommentare** nennen jetzt zuerst die Bedingung aus dem
`# Safety`-Abschnitt der Bindung und danach, wodurch sie erfüllt ist. Beim
Ereignisabgriff ist das die eine dokumentierte Bedingung über den Rückgabewert des
Blocks, bei Datenquelle und Delegiertem die nullende schwache Eigenschaft. Der
Halbsatz "leben laenger als die Tabelle" ist weg; die Tabelle überlebt beide.

**Die Gegenprobe der acht Bitwerte** liegt in `appkit/ereignisse.rs`, nicht wie
vorgeschlagen außerhalb von `appkit/`. Der Grund steht im geschlossenen Datensatz
`260803-1345_c_…`: die Grenze hängt an jeder Berührung mit `objc2` und nicht an
der Übersetzerregel, und `260803-1530_o_…` schlägt genau dafür eine Prüfvorschrift
auf `use objc2` vor. Eine Datei mit `use objc2_app_kit` neben `appkit/` wäre ihr
erster Verstoß. Nachgeprüft: `grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src`
gibt weiterhin keine Zeile außerhalb von `appkit/`.

Auch hier ist nachgewiesen, dass die Prüfung greift: `roh::BEFEHL` versuchsweise
auf `1 << 21`, und sie meldet `der Wert fuer Command weicht von
NSEventModifierFlags ab, left: 2097152, right: 1048576`. Die fünf Tastencodes
stehen jetzt einmal als Zahl in `tests/tasten.rs`, gegen die Carbon-Tabelle
`kVK_*` aus `HIToolbox.framework/Headers/Events.h` gehalten, alle fünf im SDK
nachgesehen.

**Die Generationsprüfung** ist entfernt, nicht belegt. Eine Prüfung zu belegen,
die die Umsetzung nie erreicht, hieße einen zweiten Mechanismus neben dem
tragenden stehen zu lassen. Der Modulkopf von `tabelle.rs` beschreibt jetzt den
wirksamen Weg und hält fest, dass die Prüfung bis zum 260803 danebenstand und
warum sie nicht greifen konnte, damit sie nicht unbesehen zurückkehrt.

**Der Einzugstakt** war der siebte Defekt, und er hat sich seit seiner Meldung
verändert.

## Was der siebte Defekt nach der Nachprüfung wert war

Die Voraussetzung des Datensatzes ist entfallen: die Bildwiederholrate ist
erhoben. `appkit/bildtakt.rs` liest sie aus `NSScreen.maximumFramesPerSecond` am
Bildschirm des gemessenen Fensters, und der Bedingungskopf von
`messungen/260803-1641-durchstich.txt` trägt sie mit 60 Hz. In beiden Läufen von
heute meldet das Bündel `krk-messung bildwiederholrate 60`. Damit gilt die erste
der beiden Möglichkeiten, die der Datensatz nennt, wörtlich.

Ob der Takt die Rate lesen sollte, ist nachgeprüft, und die Antwort ist nein.
Drei Dinge sprechen dagegen. Der Zeitgeber hängt an der Laufschleife und nicht
am Bildschirm; ihn abzuleiten hieße, ihn bei jedem Bildschirmwechsel des Fensters
neu aufzuhängen. Ein abgeleiteter Takt braucht eine Antwort auf "das Fenster steht
auf keinem Bildschirm", und die Antwort des Projekts darauf ist der Abbruch mit
Meldung, die für einen gewöhnlichen Lesevorgang nicht in Frage kommt; ein fester
Rückfallwert wäre die Sonderregel mit eigenem Rückfallweg, die die Maxime
"supersimpel" ausschließt, und der Datensatz nennt sie selbst. Und am
Referenzgerät kauft die Ableitung nichts.

Geändert ist deshalb nur der Kommentar. Er nennt die Fundstelle der Rate statt
sie vorauszusetzen und schreibt aus, was die feste Zahl auf einem schnelleren
Bildschirm kostet: dort räumt der Takt nur bei jedem zweiten Bild, die Liste baut
sich langsamer auf, als der Schirm es zuließe, und die Zusage "höchstens einmal
je Bild zeichnen" hält weiter.

## Hat sich einer der sieben als nicht haltbar erwiesen?

Nein. Alle sieben tragen. Einer ist ohne Änderung am Programmtext geschlossen,
und das ist der vierte, die zwei Fehlermeldungen, die im Bündel niemanden
erreichen. Der Befund selbst stimmt und ist nachgeprüft. Der Datensatz sagt
seinen Ausgang aber selbst: "Beides ist eine Festlegung, keine Reparatur." Der
Ordnerfehler braucht die Statuszeile aus C1, die kein Schritt heute baut; der
fehlende Tastenabgriff braucht eine Antwort auf die Frage, ob KRK ohne
Tastatursteuerung weiterlaufen soll, und die gehört dem Nutzer. Ein `NSAlert` an
dieser Stelle wäre eine erfundene Verhaltensänderung. Der Datensatz ist deshalb
mit einem Verweis auf den neuen Entscheidungsdatensatz geschlossen, und die
beiden `eprintln!` stehen unverändert im Programmtext.

Der siebte hat sich nicht als unhaltbar erwiesen, sondern als überholt: er
verlangt einen Beleg, den es inzwischen gibt.

## Angelegt

| Datei | Was drinsteht |
|---|---|
| `decisions/260803-2025_o_wie-zeigt-krk-dem-nutzer-fehler.md` | Wie KRK dem Nutzer einen Fehler zeigt, den er sehen muss. Drei Möglichkeiten, eine Empfehlung, die Zuordnung zu Schritten. |
| `issues/260803-2025_o_frage-2-des-plans-nennt-den-unwirksamen-mechanismus.md` | Der Plananteil des sechsten Defekts. Getrennt gemeldet, weil der `planner` zur selben Zeit in derselben Datei arbeitet. |
| `issues/260803-2025_o_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md` | `Meldung::generation` und `Lesevorgang::generation` in `leser.rs`, das außerhalb des Auftragsumfangs lag. |
| `issues/260803-2025_o_der-tastencode-von-pfeil-ab-steht-an-zwei-stellen.md` | `CODE_PFEIL_AB` in `ereignisse.rs` neben `code::PFEIL_AB` im Kern. Heute durch eine Prüfung abgedeckt, aber weiterhin zwei Wahrheiten. |

## Abnahme

Alle vier Kommandos mit Rückgabewert 0, gefahren nach dem letzten Eingriff:
`cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets`. Der Prüfbestand ist von 95 auf 121
gewachsen.

Die `unsafe`-Grenze ist unverändert: `#![deny(unsafe_code)]` in
`crates/krk-ui/src/main.rs:1`, die eine Ausnahme in `appkit/mod.rs:1`, keine
`use objc2`-Zeile in `crates/krk-ui/src/messmodus.rs`.

**Eine Falle beim Nachweisen, für den nächsten, der dasselbe tut.** Der Lauf
gegen den defekten Stand lief über `sed -i.bak` und ein anschließendes
Zurückschieben der Sicherung. Die Sicherung trägt die alte Änderungszeit, und
`cargo` hielt die Kiste danach für unverändert und prüfte gegen ein veraltetes
Übersetzungsergebnis weiter. Aufgefallen ist es, weil zwei fremde Prüfungen
plötzlich fehlschlugen. Ein `touch` auf die Datei räumt es aus; wer so vorgeht,
prüft das Ergebnis danach noch einmal.

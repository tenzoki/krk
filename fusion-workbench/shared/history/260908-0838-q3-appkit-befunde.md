# Bahn Q3: 23 Befunde in `crates/krk-ui/src/appkit/` behoben, 50 offen gelassen

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Der Auftrag

Die offenen Defektdatensätze abarbeiten, deren Behebung in `crates/krk-ui/src/appkit/`
landet. Drei weitere Bahnen liefen gleichzeitig (`krk-core/`, `krk-ui` außerhalb `appkit/`,
`xtask/`+`krk-bench/`); alles außerhalb von `appkit/` blieb unangetastet.

## Die Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

189 offene Datensätze. Davon zitieren 77 eine Datei unter `appkit/`; die Bahn sind die,
deren **Behebung** dort landet. Der Rest zitiert `appkit/` als Nachbarstelle und wird
anderswo behoben (`krk-core`, `krk-ui` außerhalb `appkit/`, `CLAUDE.md`, Werkbankdatensätze).

## Geschlossen (23)

Je Datensatz die Kurzform; die `Resolved:`-Zeile am Datensatz trägt das Einzelne.

| Datensatz | Was gebaut ist |
|---|---|
| `260812-1529` Besitzregel des Freigabewählers | `eintrag_anfuegen` hält seinen `NSSharingServicePicker` fest, in einem **zweiten** Schlitz; beide Nebenbefunde mitbehoben |
| `260812-1805` `textmerkmale.rs` ohne Probe | vier Proben ohne Fenster: die Tafel von `grundmerkmale`, die Ordnung der Überschriftsfaktoren, zwei Zusagen daneben |
| `260812-1920` Auszeichnung in einer Überschrift | der Größenverlust steht im Kommentar; zwei Nachbardatensätze annotiert |
| `260815-1858` dritte Aufzählung der `Unerreichbar`-Gründe | Beispiele statt Aufzählung, mit den zwei Fällen, die durchfallen |
| `260818-2221` Abwurf reicht das Ziel als Quellordner | `Vorgang::ordner` entdoppelt; das Feld behält seine Bedeutung |
| `260820-0735` Anker des Freigabedialogs | `visibleRect()` statt `bounds()` |
| `260823-1442` Modulkopf der `rundwegproben` | Zählprobe auf `editor_schliessen`, und der Kopf nennt die richtige Abwehr |
| `260823-1445` die neue Regel verweist ins Leere | `sichtbarkeit_aendern` sagt, dass es nicht misst und warum es nicht muss |
| `260826-1325` „Lesezeichen angelegt" über einem Fehlschlag | `lesezeichen_aendern` liefert `bool` mit `#[must_use]` |
| `260826-1325` `Esc` im Stapel-Umbenennen-Blatt | `Blatt::zeigen` liefert den Griff, sechs Öffner legen ihn ab, und `abbrechen` fragt zuerst `blatt_steht()` |
| `260826-1327` `setContainerSize:` | `setSize:`, samt Eintrag im Untergrenzen-Abschnitt |
| `260826-1327` `new_unchecked` „sonst nirgends" | die drei Stellen namentlich, samt Erhebungskommando |
| `260826-1327` Freigabe des Rückgängig-Blocks | gemessen, alle vier Wege; der Freigabeverbund ist die Bedingung |
| `260826-1327` zweite Abschrift der Einfärbung | als angenommener Preis benannt, mit dem Abstand zur ersten |
| `260826-1327` `abwurf_pruefen`, Preisargument | gestrichen; der zweite Grund trägt allein |
| `260826-1332` `offenes_blatt` als Einzelschlitz | Konflikt und Bericht bleiben liegen, solange ein Blatt steht; ein Schreib- und ein Löschpunkt |
| `260826-1337` Ausleihe während des Rückrufs | `Rc` statt `Box`, aus der Ausleihe geklont |
| `260826-1421` Pfade ohne gültiges UTF-8 | alle vier Hüllen weisen ab; zwei neue Proben |
| `260828-1046` `dokument_setzen` | Merkposten trägt die `Deutung`, eine Schreibstelle für alle drei Zweige |
| `260829-0041` Probenablagen | eine Ablage je Prozess, unter einer Sperre |
| `260829-0052` Abweisungsmeldung | Ablegen ist ganz oder gar nicht |
| `260907-0750` Nachbildung der Konfliktschaltflächen | verschwunden; jeder Bauplan wird bei seinem Bauer geholt |
| `260907-2127` zwei Selektorlisten | drei Proben halten sie aneinander, über das Laufzeitsystem |

Dazu `260812-1920` mit Anmerkungen an `260812-1851_d_*` und `260812-1805_c_*`.

## Offen gelassen (50)

Von den 77 der Erhebung sind heute 27 geschlossen — 23 davon in dieser Bahn, vier von den
Bahnen daneben (`260907-0726`, `260907-0859`, `260907-1226`, `260907-2300`, alle
`CLAUDE.md` oder `krk-core`). Nichts ist ohne Änderung geschlossen worden: jeder der 50
offenen Datensätze besteht am heutigen Baum unverändert fort, und jeder ist einzeln gegen
ihn gelesen worden.

Drei der vier Gruppen darunter zählen zusammen 17 und nennen jeden Datensatz. Die vierte
sind die 33, deren Behebung außerhalb von `appkit/` landet; sie stehen nach Zielbahn
gebündelt, weil ihre Adresse die Bahn ist und nicht der einzelne Datensatz.

**Braucht eine Nutzerentscheidung, die der Datensatz offen lässt (10).**
`260818-0410` (welche Schaltfläche ausführt: benennen oder ableiten) ·
`260820-2235` Startmeldungen (eine Meldung aus n, oder ein Blatt) ·
`260823-0731` Ziehbewegung (Messung an die Rufer oder in `aufteilung_nachziehen`) ·
`260812-1854` Kurzhinweis (drei Zuschnitte) ·
`260826-1422` Abwahl in der Leiste (`setAllowsEmptySelection(false)` oder `-1` an das Modell) ·
`260826-1416` Nummernspalte (drei Wege, einer davon „erst messen") ·
`260816-0040` Takt des Lesevorgangs (drei Wege, ausdrücklich Nutzerfrage) ·
`260825-1922` Programmstart und Tabwechsel (eine Tür oder drei) ·
`260820-0733` Abfangstelle (hängt an der Bündelabnahme von C2.12) ·
`260826-1333` Stapelblatt (die Meldung ist berichtigt; die verlorenen Eingaben brauchen einen
Weg, eine Schaltfläche nach dem Zeigen zu schalten).

**Braucht den Abnahmelauf am Bündel, also Nutzerarbeit (4).**
`260813-0311` Klick in die Bereichsleiste (Weg 1 gewählt, der Verlust steht auf der
Abnahmeliste) · `260818-0413` `Cmd+Return` und `Opt+Return` im Namensfeld ·
`260812-1529` Freigabedialog außerhalb eines `mouseDown` · `260823-0732` L1 mit
Umschaltbefehlen in der Reihe.

**Führt aus der Bahn hinaus (33).**
`260812-0512` (`fenstermodell.rs`, die öffentliche Frage fehlt) ·
`260812-1204`, `260813-0719`, `260814-1612`, `260816-1932`, `260816-1934`, `260821-0142`,
`260826-1221`, `260826-1302`, `260826-2156`, `260828-1046` (http/https) —
alle `krk-core` · `260812-1701` (`markdown.rs`), `260815-0020` (`tabs.rs`),
`260826-1417` (`editormodell.rs`), `260826-1418` und `260828-1046` (Variantenleser) und
`260907-0858` (Rechtesperre) — `krk-ui` außerhalb `appkit/` ·
`260907-0858` (ALLE-Listen, `tests/gemeinsam/mod.rs`) · `260826-1442` (`messmodus.rs`) ·
`260820-0602` (`Makefile`) · `260812-1526`, `260827-1911` (Ressourcendateien) ·
`260812-0415` (`spalten.rs` und `krk-core`) · `260812-1816`, `260813-1110`, `260813-1345`,
`260814-1247`, `260818-0752`, `260823-1336`, `260823-1439`, `260827-1710`
(Werkbankdatensätze und fremde Prosa) · `260813-0540`, `260826-1420`, `260825-1922`
(Auffrischungskosten) — je zur Hälfte oder ganz außerhalb.

**Halb gebaut, Datensatz offen mit Nachtrag (3).**
`260823-1433` — drei der vier Codestellen tragen die bedingte Fassung; die vierte steht in
`kommandos/rundweg.rs` und liegt außerhalb dieser Bahn.
`260820-2235` gemessener Start — der Kommentar nennt jetzt alle drei Lagen; die Frage, ob der
gemessene Start den Ablagedurchgang mitfährt, ist offen.
`260826-1333` — siehe oben.

## Was die Messung nebenbei ergeben hat

Die vier Wege, auf denen ein `NSUndoManager` eine Handlung fallen lässt, halten alle vier —
**aber nur innerhalb eines offenen `autoreleasepool`**. Ohne ihn fällt der Zähler auf keinem
der vier. Das stand im Kopf von `Stapellast` als Möglichkeit und steht jetzt als Messung.
Ein Umlauf über `runMode:beforeDate:` ersetzt den Verbund nicht: er kehrt ohne Eingabequelle
sofort zurück, und die Probe stand eine Fassung lang auf diesem Irrtum.

Daraus folgt ein neuer Datensatz:
`260908-0838_*_claude-md-nennt-die-freigabe-des-rueckgaengig-blocks-ungemessen-seit-heute-haelt-sie-eine-probe.md`.

## Abnahme

Alle fünf Kommandos grün, am Ende der Sitzung gefahren:

```
cargo build --workspace                              — exit 0
cargo test --workspace                               — exit 0 (krk-ui: 941 Proben)
cargo clippy --workspace --all-targets -- -D warnings — exit 0
cargo fmt --all --check                              — exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps — exit 0
```

Nicht committet, wie beauftragt.

## Eine Anmerkung zum parallelen Betrieb

`cargo fmt -p krk-ui` formatiert die ganze Kiste und damit auch die Dateien der Bahn, die
`krk-ui` außerhalb `appkit/` fährt — genau der Fall, vor dem
`260820-0602_*_make-check-prueft-den-ganzen-arbeitsbereich-*` warnt. Er ist einmal
eingetreten. Danach lief die Formatierung nur noch über
`git status --porcelain -- crates/krk-ui/src/appkit | awk '{print $2}' | xargs rustfmt --edition 2024`.

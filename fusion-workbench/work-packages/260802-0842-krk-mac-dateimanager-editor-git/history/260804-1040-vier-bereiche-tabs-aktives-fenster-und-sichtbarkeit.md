# Schritt 12: Vier Bereiche, Tabs, aktives Fenster und Sichtbarkeit (C1, C7)

**Datum:** 2026-08-04, 10:40
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 12.`
**Spec:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, C1, C7, C8
**Nicht committet.** Der Auftrag schließt das Committen aus; der `[DONE]`-Vermerk am Plan setzt der Nutzer.

---

## Was entstanden ist

**Fünf neue Dateien**, drei davon unter `crates/krk-ui/src/appkit/` und zwei
daneben:

| Datei | Was sie hält |
|---|---|
| `crates/krk-ui/src/appkit/aufteilung.rs` | die `NSSplitView` mit vier Bereichen, ihren Delegierten mit den Mindestbreiten, das Ein- und Ausblenden und die Markierung des aktiven Dateifensters |
| `crates/krk-ui/src/appkit/tableiste.rs` | die Leiste am Kopf eines Dateifensters, ein Abschnitt eines `NSSegmentedControl` je Tab |
| `crates/krk-ui/src/appkit/statuszeile.rs` | die Textzeile am Fuß eines Dateifensters |
| `crates/krk-ui/src/fenstermodell.rs` | das aktive Dateifenster, die Sichtbarkeit der vier Bereiche, ihre Breiten, die Rechenvorschrift für die Breitenverteilung und die Lesereihenfolge beim Start |
| `crates/krk-ui/src/tabs.rs` | die Tabs eines Dateifensters mit Ordner, Inhalt, Lesevorgang, Auswahl, Bildlaufposition und Meldung |

**Elf erweiterte Dateien:**

| Datei | Was sich geändert hat |
|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs` | hält zwei Dateifenster statt eines, dazu Fenstermodell, Aufteilung und Sitzungsschreiber; `applicationShouldHandleReopen:`, `applicationWillTerminate:`, `fensterEinblenden:`; die Verteilung der Kommandos |
| `crates/krk-ui/src/appkit/tabelle.rs` | von einem Ordnermodell auf eine Tabliste umgebaut; Auswahl und Bildlaufposition je Tab setzen und lesen; die Statuszeile am Fuß; kein `eprintln!` mehr |
| `crates/krk-ui/src/appkit/fenster.rs` | Inhaltsansicht ist die Aufteilung; der Delegierte bricht die Lesevorgänge beider Dateifenster ab |
| `crates/krk-ui/src/appkit/menue.rs` | "Fenster einblenden" auf Cmd+N dazu, "Fenster schließen" von Cmd+W auf Shift+Cmd+W |
| `crates/krk-ui/src/appkit/ereignisse.rs` | der Abgriff nimmt eine Belegung und eine Rust-Senke entgegen statt einer Datenquelle |
| `crates/krk-ui/src/appkit/mod.rs` | drei neue Module eingebunden, Modulkopf nachgezogen |
| `crates/krk-ui/src/main.rs` | `mod fenstermodell; mod tabs;` |
| `crates/krk-core/src/ablage/mod.rs` | `melden` gibt den Text zurück; `Geladen::gemeldet` ist zu `mit_meldung` geworden |
| `crates/krk-core/src/ablage/sitzung.rs` | `Tab::bildlauf` dazu; `Tab` und `Dateifenster` verlieren `Eq`, weil eine Gleitkommazahl keine vollständige Gleichheit kennt |
| `crates/krk-core/src/tasten/belegung.rs` | `Kommando` wächst von fünf auf sechzehn; `fuer_den_betrieb` liefert die Meldung mit |
| `crates/krk-core/tests/ablage.rs` | drei neue Prüfungen für das Fenster- und Tabmodell; die Kindprobe für die Standardfehlerausgabe ist entfallen |

`resources/default-keymap.toml` ist unverändert. `xtask/`, `crates/krk-bench/`,
die Plandatei und der Spec sind nicht angefasst.

## Zwei Entwurfsfragen, die der Plan offengelassen hat

**Wo die Tabs wohnen.** Der Plan schreibt vor, dass `fenstermodell.rs` und
`tabs.rs` allein das Modell halten und keine `objc2`-Kiste nennen. Umgesetzt ist
das so: `tabs.rs` hält je Tab ein eigenes `Ordnermodell` und einen eigenen
`Lesevorgang`, beides aus `krk-core` und damit ohne AppKit. Das war die
Bedingung für die Zusage aus C8, dass ein verdeckter Tab bereitsteht, bevor der
Nutzer ihn ansteuert: mit einem Ordnermodell je Dateifenster ließe sich der
Inhalt eines zweiten Tabs nirgends halten, und jeder Wechsel stieße einen neuen
Lesevorgang an, den L5 mit seinen 50 ms nicht deckt. `appkit/tabelle.rs` hält
weiter genau eine `NSTableView` je Dateifenster und tauscht beim Tabwechsel
aus, was sie zeigt.

**Wie die Breiten zustande kommen.** Eine Rechenvorschrift, an einer Stelle:
`fenstermodell::bereichsbreiten`. Die beiden Randbereiche bekommen ihre
gespeicherte Breite, die beiden Dateifenster teilen sich den Rest im Verhältnis
ihrer Breiten. Der Delegierte der `NSSplitView` ruft dieselbe Funktion, wenn
AppKit die Bereiche neu auslegen lässt, und speist ihr die Breiten ein, die
gerade auf dem Schirm stehen. Damit überlebt eine mit der Maus verschobene
Trennlinie jede Fenstergrößenänderung, ohne dass eine zweite Rechenvorschrift
daneben entsteht.

## Der Kern gibt nichts mehr aus

`ablage::melden` liefert den Meldungstext zurück, statt ihn zu schreiben, und
`Geladen::gemeldet` ist durch `Geladen::mit_meldung` ersetzt, das Wert und Text
als Paar liefert. `belegung::fuer_den_betrieb` reicht die Meldung nach oben
durch. Der Aufrufer ist `Anwendungsdelegierter::oberflaeche_aufbauen`; er sammelt
die Startmeldungen und stellt sie in die Statuszeile des linken Dateifensters.
Denselben Weg nimmt der unvollständig gelesene Ordner, der bis hierher in
`tabelle.rs` auf die Standardfehlerausgabe ging.

Die Aufrufrichtung bleibt von oben nach unten: der Kern liefert einen Wert und
ruft niemanden an. Eine zweite Abhängigkeitsumkehr neben der
Papierkorb-Schnittstelle aus `## Aufbau` entsteht nicht.

`grep -rn 'eprintln!' crates/krk-core/src crates/krk-ui/src/appkit/tabelle.rs`
findet nichts, Rückgabewert 1. Der Modulkopf von `ablage/mod.rs` schreibt den
Namen des Ausgabemakros deshalb nicht aus; er würde die eigene Prüfung brechen.

Nicht berührt sind der Protokollmodus `--tasten-protokoll` und die Messberichte.
Ebenfalls stehen geblieben ist der `eprintln!` im `None`-Zweig des
Tastenabgriffs in `anwendung.rs`: er gehört zu Schritt 6b, der noch aussteht.

## Zwei Befunde aus der Sonde, beide behoben

Die Sonde (siehe unten) hat zwei Abweichungen von C7 gezeigt, die im Modell
nicht auffielen. Beide sind behoben und tragen seither je eine Prüfung in
`crates/krk-ui/src/fenstermodell.rs`.

**Das wiedereingeblendete Dateifenster kam auf der falschen Breite zurück.**
Gemessen: 406 Punkte vor dem Ausblenden, 269 danach. Ursache war, dass das
sichtbare Dateifenster den Platz des ausgeblendeten mitträgt und diese Zahl über
`breiten_uebernehmen` als sein Wunsch in das Modell zurückfloss; das Verhältnis
der beiden zueinander stand danach auf einem Wert, in dem das ausgeblendete gar
nicht vorkam. `breiten_uebernehmen` lässt die beiden Dateifenster jetzt in Ruhe,
solange nur eines von ihnen sichtbar ist. Nachgemessen: 406 vor und nach dem
Ausblenden. Prüfung
`das_wiedereingeblendete_dateifenster_hat_wieder_seine_alte_breite`.

**Der Tastenbefehl verschob die Trennlinie um 13 statt um 40 Punkte.** Ursache
war, dass `breite_aendern` nur den Zähler des Verhältnisses erhöhte, während der
Nenner mitwuchs. Bei einem Dateifenster verschiebt der Befehl jetzt die
Trennlinie: das andere gibt genau so viel ab, wie dieses bekommt. Nachgemessen:
581 statt 541, also genau 40. Prüfung
`der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt`.

## Die vorübergehende Sonde

Das Bild auf dem Bildschirm und der körperliche Tastendruck sind ohne Freigaben
nicht prüfbar, die diese Sitzung nicht hat. Belegt ist das Verhalten deshalb
über eine **vorübergehende Sonde im laufenden Bündel**, denselben Weg, den die
Schritte 7 und 8 gegangen sind. Sie baute echte `NSEvent`-Tastendrücke und
stellte sie über `postEvent:atStart:` in die Ereignisschlange der Anwendung; die
Ereignisse liefen damit durch den lokalen Abgriff, die Normalisierung und den
Nachschlag in der Belegung, also über genau die Kette, die auch ein
körperlicher Tastendruck nimmt. Daneben las sie den Zustand des Fenstermodells,
der beiden Tablisten, der Statuszeilen und des Hauptmenüs aus.

**Die Sonde ist vollständig zurückgenommen.** Sie bestand aus einer eigenen
Datei `crates/krk-ui/src/appkit/sonde.rs` und aus Zusätzen in `anwendung.rs`,
`tabelle.rs`, `statuszeile.rs`, `tabs.rs` und `appkit/mod.rs`. Die Datei ist
gelöscht, die fünf anderen sind aus einer Sicherung wiederhergestellt, die vor
dem Einbau entstand. `grep -rn 'sonde\|Sonde\|KRK_SONDE' crates/ xtask/` findet
keine Zeile der Sonde mehr, und `cargo build --workspace`, `cargo test
--workspace`, `cargo fmt --all --check` und `cargo clippy --workspace
--all-targets` sind danach noch einmal gelaufen.

Die eine Änderung, die nach der Sicherung entstand und deshalb von Hand
nachgezogen wurde, ist der Aufruf von `breiten_uebernehmen` in
`Anwendungsdelegierter::breite_aendern`.

## Was die Sonde gezeigt hat

Zwei Läufe. Der erste ging die Befehle der Reihe nach durch, der zweite startete
auf einer von Hand vorbereiteten `session.toml` mit zwei Tabs je Dateifenster.

| Marke | Beobachtung |
|---|---|
| A0 | zwei Dateifenster, je ein Tab, Breiten `[180, 406, 406, 260]`, Fenster sichtbar |
| A1 bis A3 | Cmd+T macht zwei Tabs, Ctrl+Tab und Ctrl+Shift+Tab wechseln zwischen ihnen und laufen um |
| A4 | Cmd+W schließt den Tab, das Fenster bleibt sichtbar |
| C3 | Cmd+W auf den letzten Tab lässt das Dateifenster stehen und zeigt `/Users/k1`, den Standardordner |
| A6 | Tab wechselt das aktive Dateifenster von links nach rechts |
| B1, B2 | Opt+Cmd+L blendet die Lesezeichenleiste aus (`[0, 501, 501, 260]`) und wieder ein (`[180, 406, 406, 260]`) |
| B3, B4 | Opt+Cmd+D blendet das zweite Dateifenster aus (`[180, 822, 0, 260]`) und wieder ein (`[180, 406, 406, 260]`); das aktive Dateifenster wandert dabei nach links |
| B5 | F3 blendet die Vorschau aus |
| B6, B7 | Ctrl+B und Ctrl+S verschieben die Trennlinie um genau 40 Punkte und wieder zurück |
| C1 | beide Dateifenster auf `/usr/share/dict`, zweimal Pfeil ab links: links Auswahl in Zeile 1, rechts keine |
| C2 | ein Ordner mit `chmod 000` füllt die Statuszeile seines Dateifensters mit "liess sich nicht vollstaendig lesen: Permission denied (os error 13)", die andere bleibt leer |
| D1, D2 | das Fenster schließt, KRK läuft weiter, Cmd+N holt es zurück |
| N0 | nach dem Neustart: Ordner, Tabs, Auswahl, Breiten und Sichtbarkeit stehen wie in der `session.toml`; alle vier Tabs sind gelesen, auch die beiden verdeckten |
| N2 | der Wechsel auf den verdeckten Tab stößt keinen Lesevorgang an: er trug schon 67 Zeilen und `liest=false` |
| Menü | "KRK beenden" Cmd+Q, "Fenster einblenden" Cmd+N, "Fenster schließen" Shift+Cmd+W |

Die Standardfehlerausgabe blieb über den ganzen ersten Lauf leer.

## Was ungeprüft bleibt

**Das Bild auf dem Bildschirm.** Ob die Markierung des aktiven Dateifensters
sichtbar ist, ob die Statuszeile lesbar am Fuß steht und ob die Tableiste ihre
Beschriftungen zeigt, ist nicht geprüft. Belegt ist, dass die zugehörigen
Aufrufe laufen und das Modell den richtigen Zustand trägt.

**Der körperlich gedrückte Shift+Cmd+W.** Der Menüeintrag trägt das Kürzel, im
laufenden Bündel ausgelesen. Dass ein Druck darauf das Fenster schließt, ist
nicht geprüft: ein über `postEvent:atStart:` eingestelltes Ereignis wird von
`NSApplication` nicht gegen die Menükürzel gehalten. Der Ereignisabgriff
schluckt es nachweislich nicht, geprüft mit `--tasten-protokoll`:
`tastencode=13 maske=shift+cmd kombination=shift+cmd+w funktion=(unbelegt)`.

**Der Klick auf das Dock-Symbol.** `applicationShouldHandleReopen:` ist
implementiert und ruft denselben Weg wie Cmd+N. Ein Mausklick auf das
Dock-Symbol ist in dieser Sitzung nicht auslösbar.

**Das Verschieben der Trennlinien mit der Maus.** Der Delegierte liefert
Mindest- und Höchstlagen; ein Mauszug ist nicht auslösbar.

**Die Reihenfolge der Lesevorgänge in der Zeit.** Dass die verdeckten Tabs erst
nach den sichtbaren beginnen, ist im Modell geprüft
(`die_nachzuegler_sind_erst_faellig_wenn_der_sichtbare_tab_steht`) und im
laufenden Bündel nur mittelbar: 300 ms nach dem Start waren alle vier Tabs
gelesen. Eine Messung der Reihenfolge gehört zu S21.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | 0 |
| `cargo test --workspace` | 0, 194 Prüfungen |
| `cargo test -p krk-core --test ablage` | 0, 19 Prüfungen |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Meldung |
| `cargo xtask bundle` | 0, signiert |
| `codesign --verify --strict target/KRK.app` | 0 |
| `grep -rn 'eprintln!' crates/krk-core/src crates/krk-ui/src/appkit/tabelle.rs` | 1, kein Treffer |

## Neue Defekte

- `issues/260804-1040_o_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md`
- `issues/260804-1040_o_der-verworfene-ausblendbefehl-aus-c7-hat-keinen-ausloeser.md`
- `issues/260804-1040_o_dateiliste-von-schritt-12-nennt-zwei-noetige-dateien-nicht.md`
- `issues/260804-1040_o_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`

## Zwei Datensätze, deren Antwort dieser Schritt einlöst

`decisions/260803-2007_a_was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird.md`
und `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md` sind beide in
Code umgesetzt. Ihr Marker steht weiter auf `_a_`, weil die Buchführung einen
Commit verlangt und dieser Schritt keinen produziert; der Nachzug auf `_i_`
gehört zu dem Commit, der diese Arbeit festschreibt.

Ebenfalls in der Sache erledigt, aber ohne Commit nicht zu schließen:
`issues/260803-1536_c_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md`
war bereits geschlossen und ist jetzt auch im Code beantwortet, und
`issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`
ist mit der Verschiebung auf Shift+Cmd+W aufgelöst.

`issues/260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`
löst sich **nicht** auf. Shift+Cmd+W steht weiter allein im Menü und nicht in
der Belegungsdatei, und der neue Defekt oben zeigt, dass es nicht bei einer
Kombination außerhalb der Konflikterkennung bleibt.

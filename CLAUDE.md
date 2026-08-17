# KRK

**Language:** de
**Artifact language:** en

## Worum es geht

KRK ist eine native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, in der Tradition von ForkLift und Norton Commander: Lesezeichen- und Geräteleiste links, zwei Dateifenster mit je mehreren Tabs in der Mitte, Vorschaufenster rechts, dazu ein Editor mit Rohansicht und Formatansicht und eine auf vier Operationen beschränkte Git-Anbindung.

Die vollständige Directive steht im Circle-Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_*_circle.md`, Abschnitt `## Directive`. Dieser Abschnitt hier ist die Kurzfassung, nicht die verbindliche Formulierung.

**Zehn Runden sind gefahren.** Wie viele es sind und wie jede geschlossen hat, sagt der Dateibestand und nicht diese Zeile: `ls fusion-workbench/circles/*/_*_circle.md`. Die Tabelle darunter ist ein Verweisregister für die Pfadregel im Absatz danach.

| | Circle | Gegenstand |
|---|---|---|
| 1 | `260802-0842-krk-mac-dateimanager-editor-git` | Navigator, Dateioperationen, die zehn Zeitzusagen |
| 2 | `260807-2116-eingebauter-editor-mit-textmarken` | der eingebaute Editor |
| 3 | `260809-2040-tastenbelegung-als-markdown-in-downloads` | Ausgabe der Tastenbelegung als Markdown |
| 4 | `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` | Pfade kopieren, Standardprogramm, Cmd+W |
| 5 | `260811-1304-statusleiste-mit-bereichsschaltern` | Bereichsleiste, proportionale Breitenregel |
| 6 | `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | Teilen, Ordnersprung, Ablage beiseitelegen, gerenderte Vorschau, volle Statuszeile |
| 7 | `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` | Suche in der Belegung, vollständiges Hauptmenü, weitere Instanz |
| 8 | `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | Version in der Titelleiste, Versionstags, `xtask release` |
| 9 | `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | Notizzettel als Blatt mit zwei Zetteln |
| 10 | `260814-1551-tippen-filtert-dateiliste-flach-und-tief` | Tippen filtert die Dateiliste, Ankreuzfeld „Deep" für den Unterbaum |

Pfade der Form `planning/…`, `decisions/…`, `analyses/…` und `issues/…` sind relativ zum Verzeichnis des **jeweils genannten** Circles zu lesen. **Ohne Nennung gilt die Runde 2** — sie hat die meisten der hier zitierten Datensätze hervorgebracht. Alle Speicher binden weiter, auch die der vorgesehenen Circles, die diese Tabelle nicht führt.

## Maximen

Aus `idea.txt`: superschnell, supersimpel, Steuerung über die Tastatur bei zusätzlicher Maus- und Trackpad-Unterstützung.

"Superschnell" trägt in dieser Form keine Abnahmekriterien. Der Spec der Runde 1 übersetzt die Maxime in Abschnitt `### C8: Messbare Geschwindigkeit` in zehn Zeitzusagen; das Referenzgerät, auf dem sie gemessen werden, steht im Datensatz `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_*_leistungszusagen-navigator.md`.

**Keine Runde nach der ersten hat eine elfte Zahl gesetzt oder eine der zehn angefasst**; nachzuzählen mit `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u`. Eine Zusage, die eine Runde nicht messen kann, wäre ein Wunsch. Die Runde 2 setzt an ihre Stelle zwei ohne Messstrecke prüfbare Kriterien; sie stehen in ihrem Spec unter `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`, dort auch die vier Gegenstände für die spätere Messrunde. **Drei davon sind mit dem Lauf vom 260810 erledigt** (L1, L4 und L7 halten). Offen bleibt der vierte, die Geschwindigkeit der Syntaxhervorhebung aus C3 — sie gehört zu keiner der zehn Zusagen, kommt aus einer fremden Kiste und ist auf dem Referenzgerät weiterhin ungemessen.

## Projektstand

Geprüft am 260815-0600. KRK entsteht in Rust mit AppKit über `objc2`. Der Cargo-Workspace steht, das Bündel `target/KRK.app` baut, trägt sein Symbol, ist signiert und liegt als `v0.4.1` aus. Aus der Runde 1 trägt die Anwendung ihr Gerüst: Lesezeichen- und Geräteleiste, zwei Dateifenster mit Tabs, Vorschaufenster, Dateioperationen mit Fortschritt und Abbruch samt Stapelumbenennen, Terminalaufruf im angezeigten Ordner und einen Messmodus, der die Zeitzusagen aus C8 am laufenden Bündel abnimmt. **Was die Runden 2 bis 10 hinzugefügt haben, steht in der Tabelle oben und wird hier nicht wiederholt**: eine Aufzählung an dieser Stelle wüchse mit jeder Runde um ein Glied, und die Tabelle führt dieselbe Auskunft schon nach Runden geordnet.

```
krk/
├── Cargo.toml            # Workspace mit vier Mitgliedern, Version an einer Stelle
├── rust-toolchain.toml   # Rust 1.97.1, beide Mac-Architekturen
├── .cargo/config.toml    # MACOSX_DEPLOYMENT_TARGET=15.0, Alias `cargo xtask`
├── crates/krk-core/      # Kern ohne AppKit: Verzeichnisleser, Ordnermodell,
│                         #   Tastennormalisierung, Ablage, `text/`, `stapelumbenennen/`
├── crates/krk-ui/        # Binärziel `krk`, AppKit-Anteil unter src/appkit/,
│                         #   Blätter unter src/appkit/blaetter/,
│                         #   die Modelle ohne AppKit daneben in src/
├── crates/krk-bench/     # Prüfordner-Erzeuger und kopflose Messstrecke
├── xtask/                # Bauwerkzeug: Bündel, Versionsersetzung, Signierung, Auslieferung
├── resources/Info.plist  # Bündelbeschreibung mit Versionsplatzhalter
├── resources/default-keymap.toml  # die eine Quelle jeder Tastenbelegung
├── Makefile              # Hülle um dieselben Kommandos, setzt den PATH zu cargo selbst
├── release.sh            # zweite Hülle: ein Kommando, ein Argument, keine Logik
├── iconset/              # die sieben PNGs, aus denen xtask die .icns baut
├── messungen/            # Messberichte: kopflose Strecke, Durchstich, Abnahmereihen
├── spikes/               # verworfene Vorstudien, als Aufzeichnung behalten
├── README.md             # Bauen, Signieren, Auslieferung, Versionspflege im Einzelnen
└── idea.txt              # der ursprüngliche Entwurf, Quelle der Directive
```

**Fast jede gefahrene Runde ist als beschränkter Abschluss (`_b_`) geschlossen, und immer aus demselben Grund: der Abnahmelauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit.** Welche das sind, sagt `ls fusion-workbench/circles/*/_b_circle.md`. Die eine Ausnahme ist die Runde `260813-0939-titelleiste-fuehrt-version-und-semantische-tags`, in der der Nutzer den Lauf selbst gefahren hat; sie schließt als bisher einzige kohärent (`_c_`). Kein Agent kann ihn fahren; warum, steht unten unter „Was man nicht sieht". Solange er nicht gefahren ist, ist „gebaut" die richtige Aussage über eine Runde und „abgenommen" nicht. Jede Runde hat ihre Planschritte vollständig auf `[DONE]` und jede behauptete Erledigung einzeln gegen den Baum gelesen — die Belege sind die Abgleiche unter `history/` des jeweiligen Circles.

**Das ist eine Eigenschaft dieses Projekts und keine Häufung von Fehlschlägen.** Wer eine Rangheuristik über die Circles legt, die allein `_c_` als erfüllte Vorbedingung zählt, bekommt hier eine irreführende Auskunft: sie trennt die eine Runde, deren Abnahmelauf der Nutzer gefahren hat, von neun Runden, die genauso weit gebaut und nur nicht abgenommen sind. Der Marker misst hier die Verfügbarkeit des Nutzers und nicht die Reife der Runde.

**Der Editor der Runde 2 ist der fünfte Bereich der Fensterzeile** und teilt sich die Fläche zeitlich mit der Vorschau; er nimmt Textdateien bis rund 16 MB an. Sein Kern liegt in `krk-core/src/text/` (Zeilenindex, Suche, Ersetzen, Einlesen, Sicherungsform), die ohne AppKit prüfbaren Teile in `krk-ui/src/editormodell.rs` und `krk-ui/src/hervorhebung.rs`. Letzteres **rechnet den vorigen Durchgang fort statt ihn zu wiederholen** (`3596e16`): je Zeile ein `Zerlegerstand` als `Haltepunkt`, Wiedereinstieg am letzten Haltepunkt vor der ersten Abweichung. Drei Anzeigen der Runde gelten allen fünf Bereichen: Fokusrahmen (`appkit/aufteilung.rs`, ein `NSBox` je Bereich), Zeilennummern (`appkit/nummernspalte.rs` — **eine** Klasse für Editor und Vorschau, Zählung aus `krk_core::text::zeilen`) und der volle Pfad im Fenstertitel (`krk-ui/src/fenstertitel.rs`).

**Welche Taste welchen Befehl auslöst, sagt `resources/default-keymap.toml` und nicht diese Datei**; `make tasten` und `make menue` geben den gebauten Stand aus. Zwei Verhalten stehen daneben und nicht in der Belegung: der Doppelklick auf eine Zeile (Ordner: hineingehen; unerreichbares Verknüpfungsziel: Meldung in der Statuszeile; sonst: an das System) und das App-Symbol, das `xtask/src/bundle.rs` zur Bauzeit über `iconutil` aus den sieben PNGs unter `iconset/` erzeugt.

**Es gibt genau eine Hülle um `NSPasteboard`**, `crates/krk-ui/src/appkit/zwischenablage.rs`; sie ist seit der Runde 4 auch Ziel und nicht mehr nur Quelle. Eine zweite daneben wäre der Fehler, den die Datei ausdrücklich vermeidet.

**Vier Aufzählungen sind seit der Runde 1 gewachsen, und jede hält den Bau an, wenn eine Stelle fehlt.** Am 260815 nachgezählt: `Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) trägt sieben Werte, `Bereich` (`krk-ui/src/fenstermodell.rs`) fünf und `Fokus` (`krk-ui/src/kommandos/fokus.rs`) fünf. Für `Kommando` in derselben Datei steht hier keine Zahl: sie wächst mit fast jeder Runde und ist in dieser Datei viermal in vier Tagen falsch geworden (`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`). Wer sie braucht, zählt sie mit `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs`. Wer eine davon erweitert, bekommt vom Übersetzer die Liste der Stellen, die nachzuziehen sind; der Mechanismus steht unten unter „Was man nicht sieht".

**`syntect` und `two-face` sind ohne ihre Vorgabemerkmale eingebunden**, weil der Vorgabesatz beider eine Bibliothek in C hereinzöge und die Bauvoraussetzungen änderte. Es sind zwei und nicht eine, weil `syntect` kein TOML kennt und der Spec TOML ausdrücklich verlangt. Die Begründungen stehen in der Wurzel-`Cargo.toml`, wie bei jeder fremden Kiste dieses Projekts. Auf dem Bauziel baut keine davon C-Code: `Cargo.lock` führt kein `cc` und außer `windows-sys` kein `-sys`-Paket.

**Der Abnahmelauf der zehn Zusagen aus C8 ist zuletzt am 260810 gefahren, und alle zehn halten in allen fünf Messdurchgängen** (`messungen/260810-1918-alle-zusagen.txt`; davor `260810-1912` mit einem Durchgang). Das ist der erste vollständig saubere Lauf, und **er liegt vor den Runden 5 bis 10** — keine der sechs ist gegen die zehn Zusagen gemessen. Der Lauf davor, `messungen/260807-1538-alle-zusagen.txt`, hielt neun von zehn und verfehlte L9.

**Die Zahl bleibt vorerst bei 65, und der Auslöser, der die Frage wieder aufmacht, sind weitere Abnahmeläufe an verschiedenen Tagen.** Der Nutzer hat am 260810-2140 „erst messen" gewählt (`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-…`, zurückgestellt): die Streuung zwischen den Runden beträgt 20 Punkte, und beide bisherigen Senkungen stammen aus je einem Lauf und mussten nachgezogen werden. **Das steht hier, weil ein zurückgestellter Datensatz aus der Suche nach aktiver Grundlage herausfällt** — wird nie wieder gemessen, ist „bei 65 bleiben" der Sache nach entschieden, ohne dass es jemand aufgeschrieben hätte.

Offene Defekte führt `issues/` im gemeinsamen Speicher und in jedem Circle (Marker `_o_`); verbindlich ist der Dateibestand, nicht diese Zeile:

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

`krk-core`, `krk-ui` und `krk-bench` tragen `#![deny(unsafe_code)]` an ihrer Kistenwurzel; die Ausnahme `#![allow(unsafe_code)]` steht nur in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`. Der Bau erzwingt diese Grenze.

## Bauen und prüfen

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
cargo xtask bundle          # baut und signiert target/KRK.app im Profil release
```

**`cargo` steht auf diesem Gerät nicht auf dem Standard-PATH.** Es liegt unter `$HOME/.cargo/bin`. Jeder Aufruf braucht deshalb den vollen Pfad oder ein vorangestelltes `export PATH="$HOME/.cargo/bin:$PATH"`.

Das `Makefile` im Projektwurzelverzeichnis nimmt einem genau das ab und ist eine Hülle um dieselben Kommandos, kein zweites Bauwerkzeug. `make check` fährt die vier Abnahmekommandos in einem Zug; die übrigen Ziele listet `make help`. Wer lieber `cargo` tippt, verliert nichts.

`cargo xtask` ist kein eingebautes Kommando, sondern der Alias aus `.cargo/config.toml`. Der Bündelbau **verlangt eine Signaturidentität**, sucht sie in drei Stufen und bricht ohne Bündel ab, wenn keine greift; auf eine Ad-hoc-Signatur weicht er nicht aus. Die drei Stufen, das Anlegen einer Entwicklungsidentität, der Fehler `errSecInternalComponent` und die Versionspflege stehen in `README.md`.

**Ausgeliefert wird über `./release.sh <version>`, und die Kette darunter hat vier Schichten, von denen jede genau eine Sache hinzufügt:** `release.sh` prüft das eine Argument, `make ausliefern VERSION=…` setzt den Pfad zu cargo und das Notarprofil, `cargo xtask version` schreibt die Zahl an die eine Stelle, `cargo xtask release` trägt die ganze Logik. **Seit der Runde 8 bricht `cargo xtask release` ab, wenn HEAD keinen Tag `v<version>` trägt, der zur `Cargo.toml` passt** — den Tag setzt der Nutzer, nicht das Werkzeug. `cargo xtask bundle` und `make check` hängen nicht an dieser Prüfung. Wann Major, Minor oder Patch steigt, steht in `README.md` unter `### Versionsstufen`.

## Was man nicht sieht, wenn man es nicht weiß

Eigenschaften, von denen jede schon einmal eine Sitzung gekostet hat. Gezählt sind sie nicht: die Liste wächst mit jeder Runde, und eine Zahl davor veraltet, wie es die Aufstellungen getan haben, die diese Datei aus demselben Grund abgelegt hat.

**Der Abnahmelauf verlangt KRK im Vordergrund.** Aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab, und die Messstrecke meldet `NICHT_IM_VORDERGRUND` statt Zahlen. Aus einem Terminalfenster im Vordergrund läuft sie durch. Kein Agent kann sie deshalb fahren; das ist Nutzerarbeit. Die Frage dazu ist offen (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`). Synthetische Tastendrücke gehören in KRKs eigene Ereignisschlange über `postEvent:atStart:` und nicht über `osascript`.

**Der Messplatz liegt unter `~/Library/Caches/krk-messplatz`**, nicht unter `/tmp`. Prüfordner einzelner Testläufe gehören dagegen nicht dorthin: sie tragen Prozesskennung und Laufnummer und räumen sich in `Drop` selbst auf. **Es gibt genau drei Fassungen, eine je Kiste, und das soll so bleiben:** `crates/krk-core/tests/gemeinsam/mod.rs` für alle Abnahmeproben des Kerns, `crates/krk-ui/src/pruefordner.rs` für die Proben jener Kiste, `crates/krk-bench/src/wegwerfordner.rs` unter dem Namen `Wegwerfordner`. Wer einen Prüfordner braucht, nimmt die Fassung seiner Kiste, statt eine vierte zu schreiben.

**Ein `make check` löscht den Messplan eines gleichzeitig laufenden Messlaufs.** `Messplanwaechter::neu` (`krk-bench/src/messen.rs`) räumt beim Anlegen jede fremde `krk-messplan-*.toml` im Temporärverzeichnis ab. So kommt der Plan auch nach einem Strg+C weg, an das kein `Drop` heranreicht; gewählt hat das der Nutzer als Option 4 von `shared/decisions/260810-1850_*_wie-kommt-der-messplan-bei-strg-c-weg-ohne-die-zusage-der-sitzungssicherung-zu-brechen.md`. **Vorausgesetzt ist dabei, dass nie zwei Läufe zugleich auf dieses Verzeichnis greifen**, und der zweite Greifer ist nicht nur ein zweiter Messlauf: die Probe `der_messplan_traegt_die_pruefsitzung_…` ruft `plan_schreiben`, also räumt auch `cargo test` ab (`shared/issues/260810-1925_*_eine-probe-schreibt-ins-echte-temporaerverzeichnis-und-raeumt-dort-jetzt-fremde-messplaene-ab.md`).

**Ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt in diesem Projekt `#[must_use]`** — nicht eine Konvention in Kommentaren. So entschieden vom Nutzer am 260811-2140, in der Sache schon am Defekt `260810-0423`. Es tragen: `EditorModell::bearbeiten` (`krk-ui/src/editormodell.rs`), `Auswahlversuch` und `Einzug` (`krk-ui/src/tabs.rs`), dazu vier Stellen in `krk-core`. **`let _ =` davor heißt überall dasselbe: „ich brauche den Wert nicht"**, und ein nackter Aufruf baut nicht mehr. **Der Bau ist dabei die eigentliche Prüfung:** `unused_must_use` ist erst unter `-D warnings` ein Fehler, `cargo build` und `cargo test` allein laufen grün.

**Etliche Fallunterscheidungen sind vollständig und haben keinen Auffangzweig.** Das ist Absicht: eine neue Variante hält den Bau an und erzwingt eine bewusste Einordnung. Jedes neue Kommando braucht eine Zeile in `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und in `bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`); jede neue Operationsart eine in `schiebt_auffrischung_auf` (`krk-ui/src/auffrischung.rs`). Die vier gewachsenen Aufzählungen stehen oben unter „Projektstand"; wer eine erweitert, baut und liest die Fehlerliste, die der Übersetzer genauer nennt als jede Aufstellung hier.

**Der Ereignisabgriff fragt nach der Nämlichkeit des Ersthelfers und nicht nach seiner Klasse.** `ersthelfer_gehoert_appkit` (`krk-ui/src/appkit/ereignisse.rs`) reicht jeden Tastendruck unverändert an AppKit weiter, sobald der Ersthelfer eine `NSTextView`, ein `NSTextField` oder ein `NSText` **ist**; die Textfläche des Editors ist die eine Ausnahme davon, und sie wird über die Objektgleichheit erkannt, nicht über die Klasse. Eine Frage nach der Klasse könnte sie vom Feldeditor eines Textfeldes nicht trennen, denn der ist dieselbe Klasse. **Wer eine zweite bedienbare Textfläche baut, entscheidet bewusst, ob er sie dort anmeldet — und die Antwort hängt davon ab, was die Fläche ist.** Ein Bereich der Fensterzeile wird angemeldet, sonst gehören seine Tasten AppKit und kein Befehl von KRK wirkt darin; so der Editor. Die Fläche eines Blattes wird **nicht** angemeldet, denn dort ist genau das erwünscht: `Esc` schließt den Notizzettel nur, weil sein Ersthelfer AppKit gehört und `Abbrechen` damit unzulässig bleibt (`appkit/blaetter/zettel.rs`, Modulkopf; `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/_b_circle.md`). Wer die Warnung ohne diese Fallunterscheidung liest, meldet die falsche Fläche an. Der Vergleich selbst steht beim Anwendungsdelegierten, der die Fläche hält; `appkit/ereignisse.rs` kennt den Editor nicht und soll ihn nicht kennenlernen.

**Ein stehendes Blatt hält Tastenbefehle nicht über den Fokusvorbehalt an, sondern beim Anwendungsdelegierten.** `Anwendungsdelegierter::kommando_ausfuehren` (`krk-ui/src/appkit/anwendung.rs`) weist jedes Kommando außer dem Abbruch ab, solange `NSWindow::attachedSheet` ein Blatt meldet; die Abfrage dafür ist `blatt_steht` in derselben Datei, die Regel selbst eine einzige Zeile in `kommandos::operationen::waehrend_blatt_erlaubt`. `eingabe_ausfuehren` hält daneben das getippte Zeichen an, mit derselben Abfrage. **Seit der Runde 7 steht die Frage an einer Stelle und hat zwei Frager.** `krk-ui/src/kommandos/zulaessigkeit.rs` beantwortet sie aus vier Bestandteilen: es steht kein Blatt (oder der Befehl ist während eines Blattes erlaubt), der Ersthelfer des Schlüsselfensters gehört nicht AppKit, `fokus::wirkt` sagt ja, und das Schlüsselfenster gehört KRK. Der Ereignisabgriff über `kommando_ausfuehren` und das Hauptmenü über `validateMenuItem:` rufen dieselbe Funktion auf derselben `Lage`; der Fokusvorbehalt ist keine Station in `appkit/ereignisse.rs` mehr. Wer nur `ereignisse.rs` liest, hält den Vorbehalt für die einzige Sperre und schließt daraus auf einen Defekt, den es nicht gibt. Genau so entstand der Fehlbefund `issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`. Der Modulkopf von `appkit/ereignisse.rs` trägt die Warnung seit `8807844`.

**Jeder Wechsel des Ersthelfers geht durch die Überschreibung von `makeFirstResponder:` in `appkit/fenster.rs`.** Die Klasse `Hauptfenster` ist der eine Auslösepunkt, und sie trägt daneben `becomeKeyWindow` und `resignKeyWindow`, also auch den Vorder- und Hintergrundwechsel. **Wer eine Anzeige an den Fokus hängt, hängt sie dort an und baut keinen zweiten Beobachter**: `NSWindow` verschickt keine Benachrichtigung über den Ersthelfer, und die Beobachtung der Eigenschaft ist von Apple nicht zugesagt. Empfänger ist `Anwendungsdelegierter::fokusanzeige_nachziehen`; es schreibt ausschließlich die fünf Rahmenfarben der Aufteilung (`appkit/aufteilung.rs`, ein `NSBox` je Bereich) und den Fenstertitel (`krk-ui/src/fenstertitel.rs`, eine reine Funktion über die fünf Fokuswerte). **Es ruft weder `anwenden` noch `setHidden`**, und der Grund gehört dazu: eine ausgeblendete Ansicht, die den Ersthelfer hält, lässt AppKit den Rang neu vergeben und diese Meldung ein zweites Mal auslösen.

**Das Tippen im Dateifenster filtert seit der Runde 10 und springt nicht mehr an.** Die Sprungmarke ist gefallen, und mit ihr das sechste Abnahmekriterium von C2 der Runde 1, das **ersetzt und nicht ergänzt** ist. Das Modul heißt jetzt `krk-core/src/verzeichnis/filter.rs` und trägt zwei Regeln, die je genau einmal dastehen: `traegt_ein_dateiname` entscheidet, welches Zeichen aufgenommen wird, und `traegt_die_folge` ist der eine Vergleich. Jede hat genau zwei Rufer, und eine Zählprobe hält das fest — sie hat während der Runde einen dritten gefangen, und behoben wurde die Wurzel und nicht die Zahl in der Probe. Der Vergleich ist eine Teilzeichenfolge ohne Rücksicht auf Groß- und Kleinschreibung und faltet keine Umlaute. **Der Filtertext übersteht seit dem 260815 jeden Ordnerwechsel**, gleich wie das Ankreuzfeld „Deep“ steht — eine Regel statt zweier, weil ein Filter, der beim Einstieg in einen tief gefundenen Ordner fiele, dem Modell der tiefen Ansicht seinen Gegenstand nähme —, und fällt nur durch `Esc`, durch das Zurücknehmen seines letzten Zeichens, mit dem Tab, der ihn hält, oder mit der Sitzung (`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`).

**Der Sortierschlüssel entsteht einmal beim Lesen** und trägt die Kollation als Bytefolge. Das ist die Voraussetzung dafür, dass L3 und L10 halten, und darf nicht in einen paarweisen Vergleich zurückfallen.

**Der Durchlauf über den Unterbaum hält genau einen Verzeichnisdeskriptor, gleich wie tief der Baum ist** (`krk-core/src/verzeichnis/durchlauf.rs`). Er liest einen Ordner ganz, merkt dessen Unterordner als **Pfad** auf einem Stapel vor und öffnet den nächsten erst danach. Wer daraus einen Abstieg macht, der den Leser der übergeordneten Ebene offen hält, baut den Defekt wieder ein, den die Durchsicht der Runde 10 gefunden hat: der Durchlauf erzeugte den Deskriptormangel selbst und legte ihn dann als „kein Treffer darunter" aus. Ein Mangel von außen lässt den Auftrag seitdem **unentschieden** statt ihn negativ zu entscheiden (`verzeichnis::sys::ist_deskriptormangel`, trennt `EMFILE` und `ENFILE`). Gemessen wird beides von Kindproben unter `ulimit -n 64`, weil `cargo test` sonst die angehobene Grenze der Sitzung erbt und die Zusage nur behauptet.

**Ein Lesevorgang leert sein Ordnermodell nicht vorab**, sondern ersetzt es mit dem ersten gelieferten Stapel (`Ordnermodell::lesevorgang_beginnen`). Wer in dieser Spanne den Bestand befragt, sieht den **alten** Ordner. Wer eine Auswahl setzen will, geht deshalb über `Tabliste::auswahl_auf_namen`: es fragt `liest()` zuerst und merkt den Namen vor, statt ihn im alten Bestand zu finden.

**Die Typprüfung vor dem Öffnen einer Textdatei steht am Deskriptor und nicht am Pfad.** `krk_core::verzeichnis::sys::ohne_warten_oeffnen` öffnet mit `O_NONBLOCK`, der Aufrufer fragt danach `metadata()` am offenen Deskriptor, und `blockierend_stellen` nimmt `O_NONBLOCK` über `F_GETFL`/`F_SETFL` wieder ab. Damit sind zwei Dinge weg: das Fenster zwischen Prüfung und Öffnen und das Blockieren an einer benannten Röhre. `lstat(2)` und `fstat(2)` beantworten verschiedene Fragen, und wer die Prüfung an den Pfad zurückzieht, blockiert wieder. **Die Hülle hat zwei Aufrufer**, den Editor (`krk-core/src/text/datei.rs`, `oeffnen`) und seit der Runde 2 auch die Vorschau (`krk-ui/src/vorschaumodell.rs`, `bis_zur_grenze_lesen`). `krk-core` führt kein `libc`; die drei Konstanten und die variadische `fcntl`-Deklaration stehen in `verzeichnis/sys.rs`, und die Deklaration ist ausdrücklich variadisch, weil ein festes drittes Argument denselben Aufruf über den falschen Argumentweg schicken würde.

**Der Rückgängigstapel des Editors trägt ein Budget in Bytes, keine Tiefengrenze.** `STAPELBUDGET` (`krk-ui/src/appkit/editor.rs`) ist `krk_core::text::datei::EDITORGRENZE`, also 16 MB, und ein `const _: () = assert!(…)` daneben hält beide beim Übersetzen aneinander; `Stapellast` zählt die gehaltenen Bytes mit und trägt in `Drop` ab, und ein `Umkehrpunkt` trägt den geänderten Bereich statt des ganzen Standes. Die obere Schranke ist damit das Budget **plus eine Handlung**: die Handlung, die es sprengt, wird nicht abgewiesen. **`setLevelsOfUndo` steht bewusst nirgends**, und der Grund gehört dazu: eine Grenze in Handlungen ließe hundert Handlungen von je einer Dateigröße zu. Ob die Freigabe eines angemeldeten Blocks wirklich abträgt, ist in diesem Baum durch nichts gemessen; der Datensatz dazu ist als Lage angenommen geschlossen, nicht behoben (`issues/260810-1341_*_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`).

**Die Rückschritt-Taste trägt zwei Bedeutungen, und die Fallunterscheidung ist sicherheitsrelevant.** `delete` liegt in der Belegung auf „In den Papierkorb räumen", und seit dem 260817 geht jedem Räumen eine Rückfrage voraus. Steht ein Filtertext, nimmt die nackte Taste stattdessen ein Zeichen zurück und erreicht `in_papierkorb` nicht; `cmd+delete` und `f8` räumen in jeder Lage. **Die Rückfrage macht die Unterscheidung milder und nicht überflüssig**: ohne sie ginge das Blatt bei jedem berichtigten Vertipper auf, und eine Frage, die auf jeden Tippfehler folgt, wird weggeklickt statt gelesen. Die Regel steht als reine Funktion in `krk-ui/src/kommandos/rueckschritt.rs`, hat genau einen Rufer und hängt an drei Größen: ob ein Filtertext steht, ob der Anschlag aus einer Wiederholung stammt, und ob diese Wiederholung bei stehendem Filtertext begonnen hat. Die dritte ist nötig, sonst hörte auch ein gehaltener Rückschritt **ohne** Filtertext nach dem ersten Anschlag auf zu räumen.

**Sie gehört ausdrücklich nicht in `kommandos/zulaessigkeit.rs`**, obwohl das die Stelle für Zulässigkeitsfragen ist: `delete` und `cmd+delete` sind dort schon dasselbe `Kommando`, und der zweite Frager derselben Regel ist die Ausgrauung des Hauptmenüs, die überhaupt keinen Tastendruck hat. Eine Antwort dort träfe beide Wege und graute den Menüeintrag aus. Der Anschlag reist deshalb als `ereignisse::Anschlag` bis in den Ausführungszweig mit; `None` heißt „es gab keinen Tastendruck".

**`krk-ui` hat kein Bibliotheksziel, nur ein Binärziel.** `crates/krk-ui/Cargo.toml` führt allein `[[bin]] name = "krk"`, ein `src/lib.rs` gibt es nicht. Eine Datei unter `crates/krk-ui/tests/` ist deshalb eine eigene Kiste und erreicht nichts aus `krk-ui`, ob `pub` oder nicht; `tests/syntaxkiste.rs` läuft nur, weil es allein `syntect` und `two-face` anspricht. Die Proben der Oberfläche stehen darum in `#[cfg(test)]`-Modulen neben dem Code, und die, die eine `NSTextView` bauen, behaupten den Hauptfaden über `MainThreadMarker::new_unchecked` in `an_einer_flaeche` (`appkit/editor.rs`, im Prüfmodul), weil `libtest` ihn nicht hergibt. Daran hängen der Defekt `issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`, einer der zwei Wege des Defekts zum Rückgängigstapel im Absatz darüber, und die Nutzerfrage `decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`, deren Antwort einen Umbau der ganzen Kiste bedeutet. Beide sind nicht mehr offen, sondern als Lage angenommen: der Defekt geschlossen, die Frage zurückgestellt. Der Zustand am Code besteht unverändert fort.

## Technologiewahl

Getroffen am 260802-1150: **Rust mit AppKit über `objc2`**, außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26. Der Datensatz ist `decisions/260802-1134_*_sprache-und-ui-werkzeugkasten.md`, die Gegenüberstellung der Kandidaten `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`, beide im Circle der Runde 1.

**`objc2` führt keine Verfügbarkeitsangaben mit sich, und der Übersetzer hält die Untergrenze deshalb nicht.** Wer eine Methode anspricht, die nach macOS 15 hinzugekommen ist, bekommt keine Warnung, sondern einen Absturz auf dem Referenzgerät.

**Die Gegenmaßnahme ist eine Gewohnheit und kein Werkzeug**, und sie hält sich nicht von selbst: der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` im Modulkopf steht in **jeder** Datei unter `crates/krk-ui/src/appkit/` außer zweien: `koordinaten.rs` und `mod.rs`, beide begründet. Eine Quote steht hier nicht — sie ist zwischen dem 260811 und dem 260814 viermal falsch geworden, während die zwei Ausnahmen dieselben blieben (`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`). Die Deckung war zwischenzeitlich auf fünf abgesunken und ist von Hand wiederhergestellt worden. Keine Klasse im Baum liegt heute über macOS 15. Ob und wie weit die Angabe prüfbar gemacht wird, ist offen (`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`, drei Stufen mit Kosten).

## Bindende Grundlage: die Entscheidungsdatensätze

Die Entscheidungsdatensätze sind die bindende Grundlage für jede Planung und jede Implementierung. **Verbindlich ist der Dateibestand, nicht diese Aufstellung.** Den Stand trägt der Marker im Dateinamen: `_o_` offen, `_a_` beantwortet aber noch nicht in Code umgesetzt, `_i_` umgesetzt, `_d_` zurückgestellt, `_s_` überholt. `fusion-workbench/shared/decisions/` führt die projektweiten Fragen, jeder Circle seine eigenen unter `decisions/`, und **alle Speicher binden weiter** — auch die der geschlossenen Runden und die der vorgesehenen Circles, die noch keine gefahren haben.

Die Antwort steht jeweils in der Zeile `Answered:` ihres Datensatzes und ausformuliert im Spec oder im Plan; sie wird hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderläuft. Wer den Stand braucht, listet alle Speicher auf, nicht nur einen:

```sh
find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'
```

**Keine offene Frage hält einen Planschritt auf; alle binden künftige Arbeit.** Die Namen liefert das `find` darüber.

**Jedes Suchmuster dieses Projekts, das `\.md` verlangt, hat einen blinden Fleck.** Verweise in Kurzform, also `260808-1413_*_…` ohne Endung, entgehen ihm: fünf Erhebungen des Markerbefunds haben dieselben acht Stellen nicht gesehen (`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`). Wer eine Erhebung fährt, erweitert das Muster, bevor er zählt.

**Aufzeichnungen eines Standes behalten ihren damaligen Marker**, und die Ausnahme gilt je Datei nach ihrem Ort, nicht je Absatz: `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`. Die Ortsregel ist entscheidbar, eine Regel je Absatz wäre es nicht und lieferte bei jedem Durchgang einen anderen Bestand. Daneben bleibt jede Stelle stehen, an der der Marker die Aussage selbst ist — eine Befundtabelle mit den Spalten „zitiert" und „ist", die Beschreibung einer Umbenennung: dort löschte die Sternform den Inhalt.

Außerhalb aller bisher gefahrenen Runden liegen die KI-Anbindung, ein integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie die Git-Anbindung, die der Kurztext oben nennt: am 260815 trägt `Kommando` keine einzige Git-Variante. Die Abgrenzung im Einzelnen steht im Circle-Datensatz der jeweiligen Runde.

**Welche Circles vorgesehen und nicht gefahren sind, sagt `ls fusion-workbench/circles/*/_a_circle.md` und nicht diese Zeile.** Am 260815 ist es einer: der Web-Betrachter im Vorschaufenster (`260804-0933-…`). Die Statusleiste mit Bereichsschaltern (`260811-1304-…`) ist als fünfte Runde gefahren und am 260812-0820 beschränkt geschlossen.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `$FUSION_PLUGIN_ROOT/bin/fusion-rules` gibt daraufhin `fusion-workbench/stilwerk/chat-voice-de.yaml` und, für Langform-Agenten, `fusion-workbench/stilwerk/default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.

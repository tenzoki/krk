# KRK

**Language:** de

## Worum es geht

KRK ist eine native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, in der Tradition von ForkLift und Norton Commander: Lesezeichen- und Geräteleiste links, zwei Dateifenster mit je mehreren Tabs in der Mitte, Vorschaufenster rechts, dazu ein Editor mit Rohansicht und Formatansicht und eine auf vier Operationen beschränkte Git-Anbindung.

Die vollständige Directive steht im Circle-Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_*_circle.md`, Abschnitt `## Directive`. Dieser Abschnitt hier ist die Kurzfassung, nicht die verbindliche Formulierung.

Pfade der Form `planning/…`, `decisions/…`, `analyses/…` und `issues/…` sind in dieser Datei relativ zum Verzeichnis des **jeweils genannten** Circles zu lesen. Ohne Nennung gilt der Circle der Runde 2, der Editor-Runde: `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/`. Die Runde 1 liegt daneben unter `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/`, und wo diese Datei einen ihrer Datensätze meint, sagt sie es dazu.

## Maximen

Aus `idea.txt`: superschnell, supersimpel, Steuerung über die Tastatur bei zusätzlicher Maus- und Trackpad-Unterstützung.

"Superschnell" trägt in dieser Form keine Abnahmekriterien. Der Spec der Runde 1 übersetzt die Maxime in Abschnitt `### C8: Messbare Geschwindigkeit` in zehn Zeitzusagen; das Referenzgerät, auf dem sie gemessen werden, steht im Datensatz `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_*_leistungszusagen-navigator.md`.

**Die Runde 2 hat keine elfte Zahl gesetzt und keine der zehn angefasst.** Sie hat den Abnahmelauf ausgeklammert, und eine Zusage, die eine Runde nicht messen kann, wäre ein Wunsch. An ihre Stelle treten zwei ohne Messstrecke prüfbare Kriterien, und beide stehen im Spec jener Runde unter `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`. Dort stehen auch die vier Gegenstände, die die spätere Messrunde zusätzlich anzusehen hat. **Drei davon sind mit dem Lauf vom 260810 erledigt**: die Zusagen L1, L4 und L7 halten. Offen bleibt der vierte, die Geschwindigkeit der Syntaxhervorhebung aus C3 — sie gehört zu keiner der zehn Zusagen, kommt aus einer fremden Kiste und ist auf dem Referenzgerät weiterhin ungemessen.

## Projektstand

Geprüft am 260810-1930. KRK entsteht in Rust mit AppKit über `objc2`. Der Cargo-Workspace steht, das Bündel `target/KRK.app` baut und signiert, und die Anwendung trägt den Navigator der Runde 1 und den eingebauten Editor der Runde 2: Lesezeichen- und Geräteleiste, zwei Dateifenster mit Tabs, Vorschaufenster, Editor, Dateioperationen mit Fortschritt und Abbruch, Terminalaufruf im angezeigten Ordner, Belegungsansicht und ein Messmodus, der die Zeitzusagen aus C8 am laufenden Bündel abnimmt.

```
krk/
├── Cargo.toml            # Workspace mit vier Mitgliedern, Version an einer Stelle
├── rust-toolchain.toml   # Rust 1.97.1, beide Mac-Architekturen
├── .cargo/config.toml    # MACOSX_DEPLOYMENT_TARGET=15.0, Alias `cargo xtask`
├── crates/krk-core/      # Kern ohne AppKit: Verzeichnisleser, Ordnermodell,
│                         #   Tastennormalisierung, Ablage, seit Runde 2 `text/`
├── crates/krk-ui/        # Binärziel `krk`, AppKit-Anteil unter src/appkit/,
│                         #   die Modelle ohne AppKit daneben in src/
├── crates/krk-bench/     # Prüfordner-Erzeuger und kopflose Messstrecke
├── xtask/                # Bauwerkzeug: Bündel, Versionsersetzung, Signierung
├── resources/Info.plist  # Bündelbeschreibung mit Versionsplatzhalter
├── resources/default-keymap.toml  # die eine Quelle jeder Tastenbelegung
├── Makefile              # Hülle um dieselben Kommandos, setzt den PATH zu cargo selbst
├── messungen/            # Messberichte: kopflose Strecke, Durchstich, Abnahmereihen
├── spikes/fn-tasten/     # Wegwerf-Prüfcode zur Fn-Tastenfrage, nicht weitergepflegt
├── README.md             # Bauen, Signieren, Versionspflege im Einzelnen
├── idea.txt              # der ursprüngliche Entwurf, Quelle der Directive
└── fusion-workbench/     # Circles, Entscheidungen, Issues, Historie
```

**Die Runde 1 ist am 260807-1035 als beschränkter Abschluss geschlossen**, Plan und Spec auf `_c_`, der Circle-Datensatz auf `_b_`. Alle 38 Planschritte tragen `[DONE]`, am Code belegt durch den Abgleich `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260807-1022-reconciliation.md`.

**Die Runde 2 hat den Editor gebaut; alle 48 Planschritte tragen seit dem 260810-0714 `[DONE]`.** Der Editor ist der fünfte Bereich der Fensterzeile und teilt sich die Fläche zeitlich mit der Vorschau. Er nimmt Textdateien bis rund 16 MB an, trägt Rohansicht und Formatansicht mit Syntaxhervorhebung, sichert, fragt an drei Anlässen nach, bevor ein ungesicherter Stand verlorengeht, springt zu Zeilen, sucht, ersetzt und trägt Textmarken in derselben Liste und derselben Ablagedatei wie die Ordner-Lesezeichen. Drei Anzeigen sind mitgekommen, und sie gelten allen fünf Bereichen: der Fokusrahmen (`appkit/aufteilung.rs`, fünf `NSBox` statt der zwei der Runde 1), die Zeilennummern in Editor und Vorschau (`appkit/nummernspalte.rs`, **eine** Klasse für beide Flächen, ihre Zählung kommt aus `krk_core::text::zeilen`) und der volle Pfad im Fenstertitel (`krk-ui/src/fenstertitel.rs`). Neu daneben: das Kernmodul `krk-core/src/text/` mit Zeilenindex, Suche, Ersetzen, Einlesen und Sicherungsform, dazu `krk-ui/src/editormodell.rs` und `krk-ui/src/hervorhebung.rs` als die Teile des Editors, die ohne AppKit prüfbar sind. `hervorhebung.rs` **rechnet den vorigen Durchgang fort statt ihn zu wiederholen** (`3596e16`): es hebt je Zeile einen `Zerlegerstand` als `Haltepunkt` auf und steigt am letzten Haltepunkt vor der ersten Abweichung wieder ein.

**Die Runde 2 schließt als beschränkter Abschluss**, wie die Runde 1: der Circle-Datensatz geht auf `_b_`. Ihre Defekte sind abgearbeitet: ein autonomer Lauf über sechs Turns und 17 Commits (`38a02b2..0140df7`) hat 52 von ihnen geschlossen, und zwei Durchsichten sowie der Abschluss-Abgleich `history/260810-1404-reconciliation.md` haben jede behauptete Behebung einzeln gegen den Baum gelesen: keine fehlt im Code. Was offen bleibt, führt der Dateibestand; das `find` weiter unten sammelt es ein. Beschränkt ist der Abschluss aus zwei Gründen, und beide liegen beim Nutzer: der Abnahmelauf verlangt KRK im Vordergrund, und die Frage nach einem Bibliotheksziel für `krk-ui` (`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`) bedeutet einen Umbau der ganzen Kiste.

**Der Abnahmelauf der Runde 2 steht noch aus und ist Nutzerarbeit.** Der Spec führt 110 Abnahmekriterien, 108 in elf Fähigkeiten und zwei unter `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`. Kein Agent kann sie am laufenden Bündel prüfen, weil KRK dafür im Vordergrund stehen muss; der Grund steht unten unter "Was man nicht sieht". Solange der Lauf nicht gefahren ist, ist "gebaut" die richtige Aussage über diese Runde und "abgenommen" nicht.

**Nicht zu verwechseln mit dem Messlauf, der am 260810 gefahren ist.** Der nimmt die zehn Zeitzusagen aus C8 der Runde 1 ab und läuft maschinell; die 110 Kriterien der Runde 2 sind Bedienprüfungen an der Oberfläche und stehen weiterhin sämtlich auf `- [ ]`.

**Vier Aufzählungen sind in der Runde 2 gewachsen, und jede von ihnen hält den Bau an, wenn eine Stelle fehlt.** `Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) trägt sieben Werte statt vier, `Kommando` in derselben Datei 65 Varianten statt 53, `Bereich` (`krk-ui/src/fenstermodell.rs`) fünf statt vier und `Fokus` (`krk-ui/src/kommandos/fokus.rs`) fünf statt vier. Wer eine davon erweitert, bekommt vom Übersetzer die Liste der Stellen, die nachzuziehen sind; der Mechanismus dahinter steht unten unter "Was man nicht sieht".

**Zwei fremde Kisten sind dazugekommen, `syntect` und `two-face`**, beide für die Syntaxhervorhebung und beide ohne ihre Vorgabemerkmale eingebunden: der Vorgabesatz beider zöge eine Bibliothek in C herein und änderte die Bauvoraussetzungen. Es sind zwei und nicht eine, weil `syntect` kein TOML kennt und der Spec TOML ausdrücklich verlangt. Die Begründungen stehen in der Wurzel-`Cargo.toml`, wie bei den vier bestehenden. Beide ziehen weitere Pakete mit, und keines davon baut C-Code auf dem Bauziel: `Cargo.lock` führt kein `cc` und außer `windows-sys` kein `-sys`-Paket.

**Der Abnahmelauf der zehn Zusagen aus C8 ist zuletzt am 260810 gefahren, und alle zehn halten in allen fünf Runden** (`messungen/260810-1918-alle-zusagen.txt`; davor `260810-1912` mit einer Runde). Das ist der erste vollständig saubere Lauf. Der Lauf davor, `messungen/260807-1538-alle-zusagen.txt`, hielt neun von zehn und verfehlte L9.

**L9 hat sich zwischen beiden Läufen erholt, und die Ursache dafür ist so ungemessen wie die des Einbruchs.** Der Anteil im Bild ging von 65,0 Prozent auf 70 bis 90 Prozent je Runde, der höchste Einzelwert von 1,70 auf höchstens 1,26 Bildlängen — **unter höherer Systemlast als am 260807**, was eine ruhigere Maschine als Erklärung ausschließt. Zur Vorgeschichte: die Zusage stand bei 95 Prozent, ist am 260807 zweimal an einem Tag gesenkt worden, zuletzt auf **65 Prozent im ersten Bild bei höchstens zwei Bildlängen**, und die zweite Senkung geschah gegen die Empfehlung des Datensatzes. Der Defekt `shared/issues/260807-1748_*_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md` ist geschlossen, führt aber weiter beide Messreihen, drei ausgeschlossene Erklärungen und drei ungemessene Verdächtige, angeführt von `16e4558`. **Die Zahl bleibt vorerst bei 65, und zwar mit einem Ablaufdatum.** Der Nutzer hat am 260810-2140 „erst messen" gewählt (`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`, zurückgestellt): die Streuung zwischen den Runden beträgt 20 Punkte, und beide bisherigen Senkungen sind aus je einem Lauf entstanden und mussten nachgezogen werden. **Der Auslöser, der die Frage wieder aufmacht, sind weitere Abnahmeläufe an verschiedenen Tagen** — sobald mehrere vorliegen, ist zu entscheiden, ob 65 bleibt. Dieser Satz steht hier, weil ein zurückgestellter Datensatz aus der Suche nach aktiver Grundlage herausfällt: wird nie wieder gemessen, ist in der Sache „bei 65 bleiben" entschieden, ohne dass es jemand aufgeschrieben hätte.

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

Das `Makefile` im Projektwurzelverzeichnis nimmt einem genau das ab und ist eine Hülle um dieselben Kommandos, kein zweites Bauwerkzeug. `make help` listet alle Ziele; `make check` fährt die vier Abnahmekommandos in einem Zug, `make bundle` und `make run` bauen und starten, `make menue` und `make tasten` geben die beiden Protokollmodi aus, `make fixture`, `make messen` und `make durchstich` bedienen die Messstrecke. Wer lieber `cargo` tippt, verliert nichts.

`cargo xtask` ist kein eingebautes Kommando, sondern der Alias aus `.cargo/config.toml`. Der Bündelbau **verlangt eine Signaturidentität**, sucht sie in drei Stufen und bricht ohne Bündel ab, wenn keine greift; auf eine Ad-hoc-Signatur weicht er nicht aus. Die drei Stufen, das Anlegen einer Entwicklungsidentität, der Fehler `errSecInternalComponent` und die Versionspflege stehen in `README.md`.

## Was man nicht sieht, wenn man es nicht weiß

Eigenschaften, von denen jede schon einmal eine Sitzung gekostet hat. Gezählt sind sie nicht: die Liste wächst mit jeder Runde, und eine Zahl davor veraltet, wie es die Aufstellungen getan haben, die diese Datei aus demselben Grund abgelegt hat.

**Der Abnahmelauf verlangt KRK im Vordergrund.** Aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab, und die Messstrecke meldet `NICHT_IM_VORDERGRUND` statt Zahlen. Aus einem Terminalfenster im Vordergrund läuft sie durch. Kein Agent kann sie deshalb fahren; das ist Nutzerarbeit. Die Frage dazu ist offen (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`). Synthetische Tastendrücke gehören in KRKs eigene Ereignisschlange über `postEvent:atStart:` und nicht über `osascript`.

**Der Messplatz liegt unter `~/Library/Caches/krk-messplatz`**, nicht unter `/tmp`. Prüfordner einzelner Testläufe gehören dagegen nicht dorthin: sie tragen Prozesskennung und Laufnummer und räumen sich in `Drop` selbst auf. **Es gibt genau drei Fassungen, eine je Kiste, und das soll so bleiben** — `646e6a1` hat zwölf zu diesen dreien zusammengelegt: `crates/krk-core/tests/gemeinsam/mod.rs` für alle Abnahmeproben des Kerns, `crates/krk-ui/src/pruefordner.rs` für die Proben jener Kiste, `crates/krk-bench/src/wegwerfordner.rs` unter dem Namen `Wegwerfordner`. Wer einen Prüfordner braucht, nimmt die Fassung seiner Kiste, statt eine vierte zu schreiben.

**Ein `make check` löscht den Messplan eines gleichzeitig laufenden Messlaufs.** `Messplanwaechter::neu` (`krk-bench/src/messen.rs`) räumt beim Anlegen jede fremde `krk-messplan-*.toml` im Temporärverzeichnis ab. So kommt der Plan auch nach einem Strg+C weg, an das kein `Drop` heranreicht; gewählt hat das der Nutzer als Option 4 von `decisions/260810-1850_*_wie-kommt-der-messplan-bei-strg-c-weg-ohne-die-zusage-der-sitzungssicherung-zu-brechen.md`. **Vorausgesetzt ist dabei, dass nie zwei Läufe zugleich auf dieses Verzeichnis greifen**, und der zweite Greifer ist nicht nur ein zweiter Messlauf: die Probe `der_messplan_traegt_die_pruefsitzung_…` ruft `plan_schreiben`, also räumt auch `cargo test` ab (`issues/260810-1925_*_eine-probe-schreibt-ins-echte-temporaerverzeichnis-und-raeumt-dort-jetzt-fremde-messplaene-ab.md`).

**Am `Auswahlversuch` unterscheidet die Schreibweise des Aufrufs, und nichts erzwingt sie.** Ein nacktes `eintrag_waehlen(…)` heißt „`Unbekannt` kann hier nicht eintreten", ein `let _ = …` mit Begründung heißt „kann eintreten und wird bewusst verworfen". Die Regel hält heute an allen fünf Aufrufstellen, steht aber nur in zwei Aufrufkommentaren; `Auswahlversuch` (`krk-ui/src/tabs.rs`) trägt kein `#[must_use]`, ein sechster Aufrufer bricht sie also unbemerkt. Dass `#[must_use]` und diese Unterscheidung einander ausschließen, ist die eigentliche Frage darin (`issues/260810-1906_*_die-konvention-am-auswahlversuch-steht-in-kommentaren-und-wird-von-nichts-erzwungen.md`).

**Etliche Fallunterscheidungen sind vollständig und haben keinen Auffangzweig.** Das ist Absicht: eine neue Variante hält den Bau an und erzwingt eine bewusste Einordnung. Jedes neue Kommando braucht eine Zeile in `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und in `bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`); jede neue Operationsart eine in `schiebt_auffrischung_auf` (`krk-ui/src/auffrischung.rs`). **Eine Zahl steht hier nicht mehr**: sie stand bis zum 260810 bei drei, war schon damals unvollständig, und der Übersetzer nennt die Stellen ohnehin genauer als jede Aufzählung. Welche vier Aufzählungen die Runde 2 verbreitert hat, steht oben unter "Projektstand"; wer eine erweitert, baut und liest die Fehlerliste.

**Der Ereignisabgriff fragt nach der Nämlichkeit des Ersthelfers und nicht nach seiner Klasse.** `ersthelfer_gehoert_appkit` (`krk-ui/src/appkit/ereignisse.rs`) reicht jeden Tastendruck unverändert an AppKit weiter, sobald der Ersthelfer eine `NSTextView`, ein `NSTextField` oder ein `NSText` **ist**; die Textfläche des Editors ist die eine Ausnahme davon, und sie wird über die Objektgleichheit erkannt, nicht über die Klasse. Eine Frage nach der Klasse könnte sie vom Feldeditor eines Textfeldes nicht trennen, denn der ist dieselbe Klasse. **Wer eine zweite bedienbare Textfläche baut, meldet sie dort an**, sonst gehören ihre Tasten AppKit und kein Befehl von KRK wirkt darin. Der Vergleich selbst steht beim Anwendungsdelegierten, der die Fläche hält; `appkit/ereignisse.rs` kennt den Editor nicht und soll ihn nicht kennenlernen.

**Ein stehendes Blatt hält Tastenbefehle nicht über den Fokusvorbehalt an, sondern beim Anwendungsdelegierten.** `Anwendungsdelegierter::kommando_ausfuehren` (`krk-ui/src/appkit/anwendung.rs`) weist jedes Kommando außer dem Abbruch ab, solange `NSWindow::attachedSheet` ein Blatt meldet; die Abfrage dafür ist `blatt_steht` in derselben Datei, die Regel selbst eine einzige Zeile in `kommandos::operationen::waehrend_blatt_erlaubt`. `eingabe_ausfuehren` hält daneben das getippte Zeichen an, mit derselben Abfrage. **Zwei Stellen mit zwei verschiedenen Fragen**: der Vorbehalt in `appkit/ereignisse.rs` fragt, wem die Taste gehört, der Delegierte fragt, welcher Befehl jetzt zulässig ist. Wer nur `ereignisse.rs` liest, hält den Vorbehalt für die einzige Sperre und schließt daraus auf einen Defekt, den es nicht gibt. Genau so entstand der Fehlbefund `issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md`. Der Modulkopf von `appkit/ereignisse.rs` trägt die Warnung seit `8807844`.

**Jeder Wechsel des Ersthelfers geht durch die Überschreibung von `makeFirstResponder:` in `appkit/fenster.rs`.** Die Klasse `Hauptfenster` ist der eine Auslösepunkt, und sie trägt daneben `becomeKeyWindow` und `resignKeyWindow`, also auch den Vorder- und Hintergrundwechsel. **Wer eine Anzeige an den Fokus hängt, hängt sie dort an und baut keinen zweiten Beobachter**: `NSWindow` verschickt keine Benachrichtigung über den Ersthelfer, und die Beobachtung der Eigenschaft ist von Apple nicht zugesagt. Empfänger ist `Anwendungsdelegierter::fokusanzeige_nachziehen`; es schreibt ausschließlich die fünf Rahmenfarben der Aufteilung (`appkit/aufteilung.rs`, ein `NSBox` je Bereich) und den Fenstertitel (`krk-ui/src/fenstertitel.rs`, eine reine Funktion über die fünf Fokuswerte). **Es ruft weder `anwenden` noch `setHidden`**, und der Grund gehört dazu: eine ausgeblendete Ansicht, die den Ersthelfer hält, lässt AppKit den Rang neu vergeben und diese Meldung ein zweites Mal auslösen.

**Der Sortierschlüssel entsteht einmal beim Lesen** und trägt die Kollation als Bytefolge. Das ist die Voraussetzung dafür, dass L3 und L10 halten, und darf nicht in einen paarweisen Vergleich zurückfallen.

**Ein Lesevorgang leert sein Ordnermodell nicht vorab**, sondern ersetzt es mit dem ersten gelieferten Stapel (`Ordnermodell::lesevorgang_beginnen`). Wer in dieser Spanne den Bestand befragt, sieht den **alten** Ordner. Wer eine Auswahl setzen will, geht deshalb über `Tabliste::auswahl_auf_namen`: es fragt `liest()` zuerst und merkt den Namen vor, statt ihn im alten Bestand zu finden.

**Die Typprüfung vor dem Öffnen einer Textdatei steht am Deskriptor und nicht am Pfad.** `krk_core::verzeichnis::sys::ohne_warten_oeffnen` öffnet mit `O_NONBLOCK`, der Aufrufer fragt danach `metadata()` am offenen Deskriptor, und `blockierend_stellen` nimmt `O_NONBLOCK` über `F_GETFL`/`F_SETFL` wieder ab. Damit sind zwei Dinge weg: das Fenster zwischen Prüfung und Öffnen und das Blockieren an einer benannten Röhre. `lstat(2)` und `fstat(2)` beantworten verschiedene Fragen, und wer die Prüfung an den Pfad zurückzieht, blockiert wieder. **Die Hülle hat zwei Aufrufer**, den Editor (`krk-core/src/text/datei.rs`, `oeffnen`) und seit der Runde 2 auch die Vorschau (`krk-ui/src/vorschaumodell.rs`, `bis_zur_grenze_lesen`). `krk-core` führt kein `libc`; die drei Konstanten und die variadische `fcntl`-Deklaration stehen in `verzeichnis/sys.rs`, und die Deklaration ist ausdrücklich variadisch, weil ein festes drittes Argument denselben Aufruf über den falschen Argumentweg schicken würde.

**Der Rückgängigstapel des Editors trägt ein Budget in Bytes, keine Tiefengrenze.** `STAPELBUDGET` (`krk-ui/src/appkit/editor.rs`) ist `krk_core::text::datei::EDITORGRENZE`, also 16 MB, und ein `const _: () = assert!(…)` daneben hält beide beim Übersetzen aneinander; `Stapellast` zählt die gehaltenen Bytes mit und trägt in `Drop` ab, und ein `Umkehrpunkt` trägt den geänderten Bereich statt des ganzen Standes. Die obere Schranke ist damit das Budget **plus eine Handlung**: die Handlung, die es sprengt, wird nicht abgewiesen. **`setLevelsOfUndo` steht bewusst nirgends**, und der Grund gehört dazu: eine Grenze in Handlungen ließe hundert Handlungen von je einer Dateigröße zu. Ob die Freigabe eines angemeldeten Blocks wirklich abträgt, ist in diesem Baum durch nichts gemessen; der Datensatz dazu ist als Lage angenommen geschlossen, nicht behoben (`issues/260810-1341_*_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`).

**`krk-ui` hat kein Bibliotheksziel, nur ein Binärziel.** `crates/krk-ui/Cargo.toml` führt allein `[[bin]] name = "krk"`, ein `src/lib.rs` gibt es nicht. Eine Datei unter `crates/krk-ui/tests/` ist deshalb eine eigene Kiste und erreicht nichts aus `krk-ui`, ob `pub` oder nicht; `tests/syntaxkiste.rs` läuft nur, weil es allein `syntect` und `two-face` anspricht. Die Proben der Oberfläche stehen darum in `#[cfg(test)]`-Modulen neben dem Code, und die, die eine `NSTextView` bauen, behaupten den Hauptfaden über `MainThreadMarker::new_unchecked` in `an_einer_flaeche` (`appkit/editor.rs`, im Prüfmodul), weil `libtest` ihn nicht hergibt. Daran hängen der Defekt `issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`, einer der zwei Wege des Defekts zum Rückgängigstapel im Absatz darüber, und die Nutzerfrage `decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`, deren Antwort einen Umbau der ganzen Kiste bedeutet. Beide sind nicht mehr offen, sondern als Lage angenommen: der Defekt geschlossen, die Frage zurückgestellt. Der Zustand am Code besteht unverändert fort.

## Technologiewahl

Getroffen am 260802-1150: **Rust mit AppKit über `objc2`**, außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26. Der Datensatz ist `decisions/260802-1134_*_sprache-und-ui-werkzeugkasten.md`, die Gegenüberstellung der Kandidaten `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`, beide im Circle der Runde 1.

**`objc2` führt keine Verfügbarkeitsangaben mit sich, und der Übersetzer hält die Untergrenze deshalb nicht.** Wer eine Methode anspricht, die nach macOS 15 hinzugekommen ist, bekommt keine Warnung, sondern einen Absturz auf dem Referenzgerät. Die Gegenmaßnahme ist eine Gewohnheit und kein Werkzeug: jedes AppKit-Modul dieses Projekts nennt in seinem Modulkopf die Untergrenze jeder Klasse, die es anspricht.

## Bindende Grundlage: die Entscheidungsdatensätze

Die Entscheidungsdatensätze sind die bindende Grundlage für jede Planung und jede Implementierung. **Verbindlich ist der Dateibestand, nicht diese Aufstellung.** Den Stand trägt der Marker im Dateinamen: `_o_` offen, `_a_` beantwortet aber noch nicht in Code umgesetzt, `_i_` umgesetzt, `_d_` zurückgestellt, `_s_` überholt. Wer den aktuellen Stand braucht, listet alle Speicher auf, nicht nur einen:

- `fusion-workbench/shared/decisions/` — projektweite Fragen
- `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/` — Fragen der Runde 2, der Editor-Runde
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/` — Fragen der Runde 1, die weiterbinden

Die Runde 1 ist geschlossen, die Runde 2 schließt mit dieser Sitzung, und beide Speicher binden weiter. Das `find` unten deckt daneben die vorgesehenen Circles mit ab, die noch keine Runde gefahren haben.

Die Antwort steht jeweils in der Zeile `Answered:` ihres Datensatzes und ausformuliert im Spec oder im Plan; sie wird hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderläuft. Die Aufstellung der offenen Fragen stand hier bis zum 260807 namentlich und ist zweimal in vier Tagen veraltet — deshalb steht sie nicht mehr hier. Wer den Stand braucht, listet alle Speicher auf:

```sh
find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'
```

**Keine offene Frage hält einen Planschritt auf; alle binden künftige Arbeit.** Die Namen liefert das `find` drei Zeilen darüber — eine Zahl an dieser Stelle veraltet auf demselben Weg wie die Aufstellung, die hier bis zum 260807 stand.

**Jedes Suchmuster dieses Projekts, das `\.md` verlangt, hat einen blinden Fleck.** Verweise in Kurzform, also `260808-1413_o_…` ohne Endung, entgehen ihm: fünf Erhebungen des Markerbefunds haben dieselben acht Stellen nicht gesehen (`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`). Wer eine Erhebung fährt, erweitert das Muster, bevor er zählt.

**Aufzeichnungen eines Standes behalten ihren damaligen Marker**, und die Ausnahme gilt je Datei nach ihrem Ort, nicht je Absatz: `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`. Die Ortsregel ist entscheidbar, eine Regel je Absatz wäre es nicht und lieferte bei jedem Durchgang einen anderen Bestand. Daneben bleibt jede Stelle stehen, an der der Marker die Aussage selbst ist — eine Befundtabelle mit den Spalten „zitiert" und „ist", die Beschreibung einer Umbenennung: dort löschte die Sternform den Inhalt.

Außerhalb der beiden gefahrenen Runden liegen die KI-Anbindung, ein integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie Git jenseits der vier genannten Operationen. Die Abgrenzung im Einzelnen steht im Circle-Datensatz der jeweiligen Runde. Zwei Circles sind vorgesehen und nicht gefahren, der Web-Betrachter im Vorschaufenster und die Tastenbelegung als Markdown; sie tragen den Marker `_a_` und liegen unter `fusion-workbench/circles/`.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `$FUSION_PLUGIN_ROOT/bin/fusion-rules` gibt daraufhin `fusion-workbench/stilwerk/chat-voice-de.yaml` und, für Langform-Agenten, `fusion-workbench/stilwerk/default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.

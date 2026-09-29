# Durchsicht: Bildfolge nach Aufnahmedatum, Nachträge Grundabsatz und settings.toml

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `52be37a..bea5cbd`
**Not-opened:** `crates/krk-bench/src/main.rs`, `crates/krk-bench/src/messen.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/leseprofil.rs`, `crates/krk-core/tests/bild.rs`, `crates/krk-core/tests/bilder/klein-exif-am-ende.heic`, `crates/krk-core/tests/bilder/klein-exif-hinter-idat-ohne-xmp.png`, `crates/krk-core/tests/bilder/klein-meta-hinter-mdat.heic`, `crates/krk-core/tests/bilder/klein-mit-datum-ohne-xmp.png`, `crates/krk-core/tests/bilder/klein-mit-datum.bmp`, `crates/krk-core/tests/bilder/klein-mit-datum.gif`, `crates/krk-core/tests/bilder/klein-mit-datum.heic`, `crates/krk-core/tests/bilder/klein-mit-datum.jpg`, `crates/krk-core/tests/bilder/klein-mit-datum.tif`, `crates/krk-core/tests/bilder/klein-ohne-datum.heic`, `crates/krk-core/tests/bilder/klein-ohne-datum.icns`, `crates/krk-core/tests/bilder/klein-ohne-datum.jpg`, `crates/krk-core/tests/bilder/klein-ohne-datum.png`, `crates/krk-core/tests/bilder/klein-ohne-datum.tif`, `crates/krk-core/tests/bilder/klein-ohne-exif.jpg`, `260929-1437-klaerung-verengung-in-der-konfliktregel.md`, `260929-1441-klaerung-aufnahmedatum-ohne-c.md`, `260929-1141_*_der-kopf-des-konfliktblatts-nennt-opt-return-im-namensfeld-ungemessen-seit-ab8d7db-leitet-der-waechter-es-weiter.md`, `260929-1141_*_die-auslieferungsfassung-von-settings-toml-sagt-das-zuruecksetzen-nenne-bei-einem-verweis-die-zeile-zum-eintragen.md`
**Review domain:** both

Die vier Probenziele darunter sind nicht gelesen, aber gefahren: `cargo test -p krk-core --test bild --test baum --test belegung --test leseprofil`, alle grün (15, 75, 9, 82 bestanden, eine ignoriert). `crates/krk-core/tests/bild.rs` ist allein nach Probennamen und dem zählenden Leser durchsucht. `make check` ist nicht gefahren. Von den früher nicht geöffneten Dateien sind jetzt gelesen: `crates/krk-core/tests/werkszustand.rs`, `260929-1033-klaerung-vorschau-uebernimmt-profilstand.md`, `260929-1034-klaerung-ortsweg-mit-festem-ziel.md`; das Ereignisprotokoll und `6c11b1f2.md` sind überflogen.

## Summary

Kern, Leser und Konfliktregel halten, was der Plan zusagt: 256 KiB je Foto sind am Begrenzer unter dem Puffer gehalten, der neue Öffner geht über `ohne_warten_oeffnen` mit Typprüfung am Deskriptor, `cargo tree` bringt je Mac-Ziel weder `cc` noch `-sys`, `im_weg` ist eine Regelstelle mit „höchstens zwei“, die engere Funktion kommt unabhängig von der Dateireihenfolge zuerst. Ein Befund hoher Schwere liegt in der Erhebung von `Lage::bildfolge`: sie fragt allein, ob die Vorschau eine Folge zeigt, und nach einem Fensterwechsel oder während eines neuen Ladevorgangs gewinnen „Voriges Bild“ und „Zum angezeigten Bild springen“ die geteilten Tasten, obwohl vor dem Nutzer keine Bildfolge liegt.

## Totals

| Schwere | Anzahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 0 |
| Low | 3 |

Dazu eine offene Frage an den Nutzer.

## Befunde nach Thema

### 1. Zulässigkeit der drei Befehle (edb7a19, 5618322, 75468b7)

**High, code: `260929-1646_*_lage-bildfolge-fragt-allein-den-vorschauinhalt-und-lenkt-cmd-up-und-return-nach-fenster-und-auswahlwechsel-auf-die-alte-folge.md`.** `anwendung.rs:4006` erhebt `bildfolge` aus `vorschau.zeigt_bildfolge()`, und das ist `matches!(aktiver_inhalt(), Inhalt::Bildfolge(_))` (`vorschaumodell.rs:1218`). Zwei Lagen:

- Nach `FensterWechseln` (`anwendung.rs:4255`) meldet niemand die Auswahl des neuen aktiven Fensters an die Vorschau (`vorschau_fuellen`, `anwendung.rs:2374`, nimmt allein Auswahlwechsel). Im anderen Dateifenster blättert Cmd+Pfeil hoch die fremde Folge, und Return springt das jetzt aktive Fenster in den Monatsordner des fremden Fotos.
- Nach einem Auswahlwechsel steht `Inhalt::Bildfolge` bis zur Lieferung des neuen Ladevorgangs weiter da (`datei_anzeigen`, `vorschaumodell.rs:1023` bis `:1036`, setzt allein `folge = None`). Return springt zum Foto der alten Folge (`folgebild`, `:1168`), Cmd+Pfeil hoch tut nichts.

Beides verletzt C3.4 und C4.6 des Spec. Die Probe `die_bildfolge_geht_vor_und_wirkt_allein_in_der_dateiliste` prüft die Regel an einer gebauten `Lage` und sieht die Erhebung nicht.

Geprüft und ohne Befund:

- **`im_weg`** (`belegung.rs`): Träger fremder Zusteller fallen heraus, der erste unverträgliche wird genannt, bei zwei verträglichen Trägern desselben Zustellers ist der Bewerber im Weg. Vollständig und disjunkt über die drei Ausgänge. `konflikte` und `zuweisen` fragen dieselbe Funktion.
- **Engere zuerst**: `nachschlag` vertauscht, wenn die zweite gefundene Funktion die erste verengt; eine eigene `keymap.toml` mit umgestellter Reihenfolge ergibt dasselbe `Geteilt(engere, weitere)`.
- **`weiter`/`verengt`**: vollständig ohne Auffangzweig, `const fn`, Vergleich über die Stelle.
- **`folge_passt`**, **`form_passt`**, **`datei_passt`**, **`fokus::wirkt`**: je ein Arm für `Bildfolge`, ohne Auffangzweig. `jede_lage` läuft über beide Werte von `bildfolge`; `eine_verengung_ist_nie_ohne_ihren_weiteren_bereich_zulaessig` läuft über alle Paare aus `KENNUNGEN` und alle Lagen.
- **Cmd+Pfeil hoch und Return ohne Bildfolge**: `waehlen` nimmt die weitere, sobald die engere unzulässig ist. Ist keine zulässig, bleibt die engere stehen; `blattmeldung` nennt kein Kommando, also ist die Abweisung wortgleich mit vorher.
- **Pflichtstellen**: `KENNUNGEN` (103), `wirkungsbereich`, `bereich_des_kommandos` (`Funktionsbereich::Vorschau`), eigene Zweige in `kommando_ausfuehren_bei`, Einträge in `zweigproben::BEFEHLE`, `default-keymap.toml` mit nachgezählter Kopfzeile. Keiner der drei steht in `immer_erreichbar` oder `waehrend_blatt_erlaubt`.

**Offene Frage, decision: `260929-1646_*_blaettert-cmd-up-im-angezeigten-jahres-oder-monatsordner-ohne-ausgewaehlte-zeile.md`.** Wer in `Fotos/2008` hineingeht, steht ohne ausgewählte Zeile (`in_zeile_einsteigen` → `ordner_lesen(&ziel, None)`), die Vorschau beschreibt den angezeigten Ordner und zeigt die Jahresfolge, und Cmd+Pfeil hoch blättert statt aufzusteigen; auf Foto 1 tut es nichts. Der Spec deckt das, die Directive sagt es wörtlich („innerhalb eines Jahresordners“), `HowTo.md` Zeilen 665 und 704 nennen den Fall nicht. Empfehlung: so lassen und dokumentieren.

### 2. Leser des Aufnahmedatums (2b94c93)

Geprüft und ohne Befund:

- **256 KiB**: `Begrenzer` zählt gelesene Bytes, liefert danach `Err`, und er liegt unter dem `BufReader`, also zählt das Nachlesen nach einem Sprung mit (`bild/aufnahmedatum.rs`, `aus_quelle`). `HOECHSTENS_BYTES_JE_FOTO` steht einmal, in `leseprofil/mod.rs`.
- **Deskriptorregel**: `aufnahmedatum` öffnet über `ohne_warten_oeffnen` (O_NONBLOCK, danach `blockierend_stellen`) und verlangt `is_file()` am offenen Deskriptor. Eine benannte Röhre mit Endung `.jpg` liefert `None` ohne zu warten; die Erhebung nimmt ohnehin nur `Typ::Datei`. Der Doc-Kommentar von `ohne_warten_oeffnen` nennt den neuen Rufer.
- **Verknüpfungen**: `ist_foto` verlangt `Typ::Datei`, der Platzhalter greift allein `Typ::Ordner`; aufgelöst wird gegen die Wurzel über dieselbe `innerhalb` wie bei den Zeilen.
- **C-Freiheit**: `cargo tree --workspace --target aarch64-apple-darwin -e normal,build` und dasselbe für `x86_64-apple-darwin` führen weder `cc` noch ein Paket auf `-sys` (217 und 218 Einträge, wie der Kommentar in der Wurzel-`Cargo.toml` sagt). Die Zeile trägt „Namen auf `-sys`“ und ist auf `=0.6.1` festgenagelt.
- **`#![deny(unsafe_code)]`**: `bild/` steht in `krk-core` und nutzt kein `unsafe`. Neue `objc2`-Namen kommen allein in `textmerkmale.rs` (8b7d7fa) hinzu, `NSMutableCopying` steht im Untergrenzen-Abschnitt.

### 3. Erhebung, Ordnung, Abbruch, Kürzung (f011f5b, 0040151)

Geprüft und ohne Befund:

- **Kein Lesen außerhalb einer Bildfolge**: der neue Arm in `zusammenfassen_gezaehlt` fragt allein `profil.bildfolge()`; ein Profil ohne Bildfolge und ein Ordner ohne Profil nehmen den alten Weg ohne weiteren Systemaufruf. Der Startweg (`leseprofile::laden`) ist unverändert; `ausgelieferte()` hat allein die Messstrecken als Rufer.
- **Rückfall bei leerer Folge**: Profil mit Bildfolge und ohne Zeilen fällt auf `Auskunft::Default`, mit Zeilen auf `Erkannt`.
- **Abbruchmarke**: `Drop` von `Abbruchmarke` setzt die Marke; `datei_anzeigen`, `neu_laden_wo` und `zwischenablage_anzeigen` setzen `folge = None`, ein ersetzter `Ladevorgang` nimmt seine Marke mit. `nachliefern` fragt vor jeder Gruppe, `gruppe_ordnen` vor jedem Foto. Ein gescheitertes erstes `send` startet die Nachlieferung nicht.
- **L7**: `laedt_noch` bleibt bei `ladevorgang.is_some()`; der Takt hängt an `wartet_noch`.
- **7.500-Kürzung**: `gruppe_nehmen` bucht höchstens bis `HOECHSTENS_FOTOS`, `gruppe_ordnen` ordnet die ganze Kappungsgruppe und schneidet auf `beitrag`.
- **Versteckte Dateien**: `ist_foto` nimmt Namen mit führendem Punkt aus.

**Low, code: `260929-1646_*_der-zaehler-nennt-jede-gekuerzte-folge-nach-7500-fotos-gekuerzt-auch-wenn-die-ordner-oder-eintragsgrenze-kuerzt.md`.** `gekuerzt` wird an fünf Stellen gesetzt (`bildfolge.rs` Zeilen 289, 319, 326, 333, 348/351), `bildzaehler_text` (`statuszeile.rs:585`) nennt immer die Fotogrenze. Ein RAW+JPEG-Ordner mit mehr als 5.000 Aufnahmen trifft die Eintragsgrenze und zeigt „Folge nach 7.500 Fotos gekürzt“ unter 7.500 Fotos.

**Low, code: `260929-1646_*_der-ausschluss-versteckter-fotos-beruft-sich-auf-einen-nutzerentscheid-den-kein-datensatz-traegt.md`.** Der Modulkopf von `bildfolge.rs` nennt „Nutzerentscheid vom 260929“; Spec C2.1 und Plan-Entscheidung 5 sagen, versteckte Dateien zählen mit, die Frage steht im geschlossenen Plan offen, und das Ereignisprotokoll führt keine solche Antwort. Ob der Nutzer im Gespräch so entschieden hat, ist nicht geprüft.

### 4. Sprung zum Foto (75468b7)

Geprüft und ohne Befund: `zum_eintrag_springen` ist der eine Rumpf beider Sprünge; alle übrigen Rufer von `ordner_lesen` übergeben `Vormerkung::still`. `wunschauswahl_anwenden` unterscheidet `Gewaehlt`, `Ausgefiltert` (im Bestand, `zeile_von` leer) und `Fehlt`; gemeldet wird allein mit Kennzeichen, ein Lesefehler hat Vorrang, das Kennzeichen fällt nach dem Abschluss.

**Low, code: `260929-1646_*_vier-doc-kommentare-der-quelltextproben-in-anwendung-rs-haengen-am-modul-sprungproben.md`.** Vor `mod sprungproben` stehen vier aneinandergehängte Doc-Blöcke (Angleichen, Zoom, Blättern, Sprung); `angleichproben`, `zoomproben` und `blaetterproben` stehen ohne. Das Muster bestand an `52be37a` zu zweit.

### 5. Messweg (e9b3b5a)

Ohne Befund. `make fotoordner` legt rund 1 GB unter `~/Library/Caches/krk-messplatz/fotoordner` an; `README.md` sagt das („tausend Fotos zu einem MB belegen also ein GB“), der Kommentar im `Makefile` ebenso, der Hilfetext von `make help` nicht. Ein Aufräumziel gibt es nicht, wie für den übrigen Messplatz. `pruefen_dass_leer` verhindert ein Überschreiben. Der kopflose Bericht (`messungen/260929-1432-bildfolge-kopflos.txt`) liest 87 von 87 Daten aus den erzeugten Fotos; 6,2 ms Median für Erhebung plus erste Gruppe bei 1.000 Fotos.

### 6. Nachträge 8b7d7fa und b54817e

Ohne Befund. `grund_vorgeben` setzt Schrift, `defaultParagraphStyle` und den Absatzstil der `typingAttributes`; Editor (Bau, `grundschrift_setzen`) und Quicknote rufen es, keine Fläche setzt die Grundschrift daran vorbei (`beide_bearbeitbaren_flaechen_nehmen_die_vorgabe_von_hier`). Der Untergrenzen-Abschnitt nennt `typingAttributes`, `setTypingAttributes:`, `defaultParagraphStyle`, `mutableCopy` und `NSMutableCopying`. **inference:** Ob getippter Text in einer geleerten Quicknote den Absatz behält, hängt am AppKit-Verhalten bei leerem Speicher und ist am Bündel nicht gemessen. `b54817e` berichtigt allein den Kommentar in `default-settings.toml` und deckt sich mit `Werkshindernis::meldung`, Zweig `Verweis`.

### 7. Doku-Commits (f72f2db, bea5cbd, Kopf von `default-readers.toml` und `default-keymap.toml`)

Gegen den Code gelesen. Kein Widerspruch außer den zwei Stellen, die die offene Frage unter Thema 1 nennt (`HowTo.md` Zeilen 665 und 704). Der Satz „Sie stehen im Menü „Vorschau“ und wirken von dort aus“ stimmt: `validateMenuItem:` und der Klick gehen über dieselbe `Lage`. „`bildfolg` … zeigt dann seine Zeilen oder, hat es keine, allein Name und Pfad“ stimmt mit dem zweiten Arm von `zusammenfassen_gezaehlt` (Bildfolge `None`, Zeilen leer → `Erkannt` mit Kopfzeile).

### 8. Früher nicht geöffnete Dateien

`crates/krk-core/tests/werkszustand.rs`: vierzehn Proben gegen C2.1 bis C2.12, fester Zeitpunkt, eigene Ablage je Probe; ohne Befund. Die zwei Klärungen des Werkseinstellungen-Pakets: ihre Empfehlungen zum Ortsweg sind mit der Anforderungsänderung `79cbb95` gegenstandslos (`ortsziel_pruefen` gibt es nicht, `aufgeloest_erneuern` hat weiter zwei Rufer), die zum Profilstand sind in `neu_laden_wo` umgesetzt; ohne Befund.

## Cross-cutting observations

- **Die Vorschau hält Inhalt, der nicht mehr vor dem Nutzer liegt, und bis zur Bildfolge war das allein eine Anzeige.** Nach einem Fensterwechsel und während eines Ladevorgangs zeigt sie den alten Stand; der offene Defekt `260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md` beschreibt dieselbe Lücke für Start und Tabwechsel. Seit `Lage::bildfolge` den Inhalt der Vorschau liest, entscheidet dieser Stand über die Bedeutung zweier Tasten im Dateifenster. Wer künftig eine weitere Lagefrage an die Vorschau stellt, braucht dieselbe Bedingung „gehört zu dem, was das aktive Dateifenster beschreibt, und nichts wartet“.
- **Ein Grenzwert trägt zwei Aussagen.** `gekuerzt` sagt „etwas fehlt“, der Satz daraus sagt „7.500“. Dasselbe Muster wie die Zahlen in Prosa, die dieses Projekt sonst ableitet statt behauptet.

## Recommended sequencing

Vor der Auslieferung als 2.2.0 (Plan, `## Where this work stops`): der High-Befund, weil er in der Zwei-Fenster-Anordnung, der Grundform von KRK, Return in einen falschen Ordnersprung verwandelt. Die offene Frage sollte der Nutzer vorher schließen, weil sie dieselbe Stelle betrifft. Die drei Low-Befunde sind Aufräumarbeit; der Satz des Zählers geht mit jeder gekürzten Folge an den Nutzer. Der Abnahmelauf am Bündel (`make bildfolge`, Haltepunkt 3) steht aus und ist Nutzerarbeit.

# Abschlussdurchsicht: Oberfläche in Systemsprache (Deutsch, Französisch, Englisch)

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `b81a284..e46a678`
**Not-opened:** `crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/sprache.rs`, `crates/krk-ui/src/appkit/blaetter/neuerungen.rs`, `crates/krk-ui/src/appkit/blaetter/ortwahl.rs`, `crates/krk-ui/src/appkit/blaetter/pfadeingabe.rs`, `crates/krk-ui/src/appkit/blaetter/pin.rs`, `crates/krk-ui/src/appkit/blaetter/startmeldungen.rs`, `crates/krk-ui/src/appkit/blaetter/suche.rs`, `crates/krk-ui/src/appkit/blaetter/uebersprungen.rs`, `crates/krk-ui/src/appkit/blaetter/ungesichert.rs`, `crates/krk-ui/src/appkit/blaetter/zeilennummer.rs`, `crates/krk-ui/src/appkit/eintragsansicht.rs`, `crates/krk-ui/src/appkit/git.rs`, `crates/krk-ui/src/appkit/hinweis.rs`, `crates/krk-ui/src/appkit/leiste.rs`, `crates/krk-ui/src/appkit/papierkorb.rs`, `crates/krk-ui/src/appkit/quicknote.rs`, `crates/krk-ui/src/appkit/weitereinstanz.rs`, `crates/krk-ui/src/fenstertitel.rs`, `crates/krk-ui/src/kommandos/blattmeldung.rs`, `crates/krk-ui/src/kommandos/pfadeingabe.rs`, `crates/krk-ui/src/kommandos/werkseinstellungen.rs`
**Domain:** code
**Work-item:** 260930-2319-oberflaeche-lokalisierbar-deutsch-und-franzoesisch

Zur `**Not-opened:**`-Liste: von `hinweis.rs` und `sprache.rs` (Tests) sind einzelne Stellen gelesen (Signatur von `hinweis::zeigen`, die Namen der Proben), der Diff nicht. `sprache/tabelle/de.rs`, `fr.rs` und `en.rs` sind als Stichprobe gelesen und nicht vollständig, siehe Thema 4. Die drei Tabellendateien stehen deshalb nicht in der Liste, sind aber nicht Eintrag für Eintrag gelesen.

## Zusammenfassung

Das Verhalten des Betriebscodes ist an den Stellen, die die Dispatch nennt, unverändert; die Sprachlesung, `festlegen` vor `starten` und das Bündel stimmen. Ein Defekt hoher Schwere liegt in der Tabellenmechanik: Französisch setzt die Null in die Einzahl, und eine Einzahl ohne `{n}` sagt dann „eins“, sichtbar am Markierungsstand mit null Ordnern und an der Vorgangszeile während des ersten Eintrags. Dazu eine Lücke der Senkenprobe mit falscher Zusicherung im Doc-Kommentar und zwei kleine Dokumentationsfehler.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 1 |
| Low | 2 |

Jeder Befund steht als eigener Datensatz unter `issues/` dieses Arbeitspakets.

## Thema 1: Verhalten unverändert

**Belegung::bauen** (`crates/krk-core/src/tasten/belegung.rs`, `fn bauen`). Gegen `b81a284` gelesen. Für die Nutzerdatei (`wortschatz = Some`) steht die Wortschatzfrage weiter zuerst und weist eine fremde Kennung mit `Belegungsfehler::UnbekannteFunktion` ab, genau wie an `e984b3f`; die neue Frage `Funktionsschluessel::aus_kennung` danach kann für eine Kennung, die der Wortschatz kennt, nicht scheitern, weil der Wortschatz selbst nur aus Schlüsseln besteht. Die Reihenfolge Wortschatz → Schlüssel → Doppelt ist dieselbe wie Wortschatz → Doppelt vorher. Eine `keymap.toml` mit einer Kennung aus einer anderen Fassung lädt damit gleich wie vorher: abgewiesen mit Rückfall. Zwei Änderungen sind gewollt und im Modulkopf benannt: `name` ist optional statt Pflicht (eine Datei ohne `name` lädt jetzt, vorher nicht), und ein `name` aus der Nutzerdatei wird nicht mehr angezeigt. Für die Auslieferung (`wortschatz = None`) bricht eine Kennung ohne Schlüssel jetzt beim ersten Zugriff ab; vorher lief sie mit ihrem Namen aus der Datei durch. Das ist eine Verengung der Auslieferung und keine Verhaltensänderung für den Nutzer. `Funktion` vergleicht über `PartialEq` jetzt Schlüssel statt Kennung und Name; Änderungserkennung in der Belegungsansicht hängt an `Belegungsmodell::geaendert` (`belegungsmodell.rs:606`, ein Merker) und nicht an der Gleichheit, also folgt daraus nichts.

**belegungsmodell::bereich** (`crates/krk-ui/src/belegungsmodell.rs`). Vorher `Option` mit `panic!` in `nach_bereichen`, jetzt `const fn` über `Funktionsschluessel` ohne Auffangzweig. Die Zuordnung ist dieselbe: Kommando zuerst über `bereich_des_kommandos`, die sechs Textbefehle nach `Textbefehle`, `filter_einfuegen` nach `Dateilisting`. `Funktionsschluessel::aus_kennung` fragt `Kommando` vor `Zugestellt` wie vorher `bereich`. Der Fall „Funktion ohne Bereich“ ist nicht mehr erreichbar, also fällt der `panic!` zu Recht.

**leseprofil::Zeile / Beschriftung** (`crates/krk-core/src/leseprofil/mod.rs`, `defaultprofil.rs`). Nutzerzeilen tragen ihren Text, das Default-Profil den Schlüssel; der Text entsteht in `Zeile::beschriftung` bei jedem Aufruf. Damit friert das `LazyLock` `DEFAULTPROFIL` keine Sprache ein. Gleichwertig zu vorher.

**auswurfmeldung** (`crates/krk-ui/src/auffrischung.rs`). Die vier Zweige sind auf drei Formen mit zwei Zahlwörtern verteilt; Deutsch ergibt denselben Wortlaut, die Zahl ist jetzt gruppiert. `(false, 0)` war vorher „0 verdeckte Tabs“ und ist es auf Deutsch weiter; laut Doc-Kommentar unerreichbar.

**Eintragsantwort::text** (`crates/krk-ui/src/appkit/editor.rs:952-995`). Die Fallunterscheidung ist neu gruppiert (`Abgehakt | WiederOffen` für Notiz und Termin), überschneidungsfrei und vollständig; jede Kombination hat denselben deutschen Text wie vorher.

**zusammenfassung_von** (`appkit/blaetter/stapelumbenennen.rs`). Die Zahlen kommen jetzt über Zahlwörter, und „1 Einträge“ wird zu „1 Eintrag“; das ist der geschlossene Defekt `261001-0731_*_die-zusammenfassung-des-stapelumbenennens-schreibt-bei-einem-eintrag-1-eintraege.md`.

**kopieren.rs** und die übrigen Vorgänge unter `crates/krk-core/src/operation/`. Jede Änderung ersetzt ein Literal oder ein `format!` durch `text`/`satz`; Zweige, Bedingungen, Reihenfolgen und Systemaufrufe sind unverändert. `grund()` (`operation/mod.rs:595-603`) übersetzt die vier Fälle aus der Tabelle und gibt sonst den Systemtext zurück, wie vorher.

Daneben gelesen, ohne Befund: die Diffs der Ablage (`ablage/*.rs`), des Heimordners (`heimordner/*.rs`), von `text/datei.rs`, `verzeichnis/*.rs`, `tasten/parser.rs`, `git/texte.rs`, `kommandos/operationen.rs`, `kommandos/loeschwarnung.rs` (die Schwellenprüfung `jede_sprache_nennt_die_schwelle` hält jetzt alle drei Sprachen beim Übersetzen), `appkit/anwendung.rs`, `menuemodell.rs`, `belegungsausgabe.rs`, `vorschaumodell.rs`, `statuszeile.rs`, `fenstermodell.rs`, `spalten.rs`, `kontextmenue.rs`, `leistenmodell.rs`, `blaetter/konflikt.rs`, `blaetter/loeschbestaetigung.rs`, `blaetter/mod.rs`, `appkit/bereichsleiste.rs`, `appkit/belegungsansicht.rs`, `appkit/tabelle.rs`, `appkit/vorschau.rs`. `Sprache::menge` ist `operationen::menge` mit Tabelleneinheiten; unter 1.000 Bytes steht jetzt „1 Byte“ statt „1 Bytes“ (Zahlwort). Keine Beschriftung dient als Schlüssel für Sitzung, Identität oder Menübau (`itemWithTitle` und Vergleiche über Titel stehen allein in Prüfmodulen). Bereichs-, Spalten-, Wirkungsbereichs- und Funktionsbereichsnamen sind je Sprache voneinander verschieden; die Proben dafür prüfen allein Deutsch, nachgezählt am Stand `e46a678`.

**Befund (High): Französisch nimmt bei null die Einzahl, und eine Einzahl ohne `{n}` sagt dann „eins“.** `Sprache::mehrzahl` (`sprache/mod.rs:176-181`) gibt für `Fr` bei 0 Einzahl, und der Modulkopf (`:49-52`) erlaubt der Einzahl, `{n}` auszulassen. Fünfzehn französische Zahlwörter tun das (`fr.rs`, `fn zahlwort`). Mit null erreichbar: `markierungsstand_text` (`kommandos/auswahl.rs:46-58`) setzt `ordner_text(0)` ein, auf Französisch „…, dont un dossier, …“ bei null Ordnern; `Steuerung::zwischenstand` (`operation/fortschritt.rs:326-339`) meldet während des ersten Eintrags `eintraege = 0`, und `vorgangszeile` (`kommandos/operationen.rs:736`) zeigt „une entrée“; `abschlusstext` (`:791`) nach einem frühen Abbruch ebenso. Die Probe `sprache/mod.rs:492` schreibt `mit(Sprache::Fr, 0) == "Einen Eintrag umbenennen"` als Erwartung fest. Datensatz `261001-1929_*_franzoesisch-nimmt-bei-null-die-einzahl-und-eine-einzahl-ohne-n-sagt-dann-eins.md`.

## Thema 2: Sprachlesung und Bündel

- `appkit/sprache.rs`: liest `preferredLocalizations().firstObject()`, ohne Eintrag `En`; der Untergrenzen-Abschnitt nennt jede hereingeholte Klasse. Richtig.
- `main.rs:139-152`: `festlegen(vom_system())` nach der Argumentauswertung und vor `appkit::starten`. Vor dem Aufruf liest nur `messmodus::Aufgabe::aus_argumenten`, dessen Text auf die Standardfehlerausgabe geht. Kein `static`, `LazyLock` oder `const` im Betriebscode hält einen Tabellentext (geprüft über `grep` auf `LazyLock`, `OnceLock`, `static`); `Sprache::De.text` steht allein in Prüfmodulen.
- `OnceLock`/`geltende()`: threadsicher, vor dem ersten Vorgang gesetzt, also liest der Arbeitsfaden der Vorgänge die gesetzte Sprache.
- `Sprache::aus_bezeichner`: Stamm vor `-`/`_`, ohne Rücksicht auf Groß- und Kleinschreibung, sonst `En`. Passt zur Messtabelle.
- `Info.plist`: `CFBundleLocalizations` `de, fr, en`, `CFBundleDevelopmentRegion` `en`. Das ist eine gewollte Verhaltensänderung für jede Systemsprache außer den dreien: bis `e984b3f` fiel sie auf Deutsch zurück, jetzt auf Englisch. Nutzerentscheid vom 261001.
- `xtask/src/bundle.rs`: `sprachen_pruefen` läuft in `vorbereiten` vor dem Übersetzen und vergleicht `CFBundleLocalizations` mit `SPRACHEN` gleich in der Reihenfolge; `sprachordner_kopieren` läuft in `zusammensetzen` nach dem Symbol und vor der Rückkehr, also vor der Signatur in `bundle` und `release`. Das alte Bündel wird vorher ganz entfernt, ein veralteter Sprachordner bleibt also nicht stehen.
- Die drei `InfoPlist.strings`: UTF-8, fünf Schlüssel je Datei; die deutsche Fassung hält eine Probe Zeichen für Zeichen an der Plist. Vor dem Semikolon der französischen Fassung stehen 3-Byte-Zeichen, also wie angegeben U+202F.

Kein Befund.

## Thema 3: Die zwei Nahtproben und die Zerlegung

`zerlegt` (`tests/gemeinsam/mod.rs`) liest Kommentare (auch geschachtelte), Strings mit Fortsetzung, Rohstrings mit `r`/`br`/`cr`, Zeichen- und Byteliterale sowie Lebensdauern richtig; Rohbezeichner wie `r#type` fallen in den Codezweig, ohne Schaden. `betriebscode` schneidet wie angegeben. Die zwei Testdateien, die `main.rs:90-97` als `#[cfg(test)] mod …;` einbindet (`pruefordner.rs`, `quellbaum.rs`), werden als Betriebscode gelesen; das kann nur falsche Funde erzeugen, keine übersehenen.

**Befund (Medium): die Senkenprobe sieht drei Senken mit `&str`-Parameter nicht, und ihr Doc-Kommentar sagt für eine das Gegenteil.** `Statuszeile::zeigen` (`statuszeile.rs:947`), `hinweis::zeigen` (`hinweis.rs:80`) und `namenseingabe::zeigen`/`frei_zeigen` (`namenseingabe.rs:210`) reichen ihren Text intern an `setStringValue`/`setMessageText`/`Blatt::neu` weiter, dort als Variable; ein Literal an ihrer Aufrufstelle bestünde beide Nahtproben, solange es keinen Umlaut trägt. `baum.rs:1693-1695` sagt, `Statuszeile::zeigen` sei über `setStringValue` gedeckt; das hält nicht. Am Stand `e46a678` steht an keiner der Rufstellen ein Literal. Datensatz `261001-1929_*_die-senkenprobe-sieht-drei-senken-mit-str-parametern-nicht-und-ihr-doc-kommentar-sagt-fuer-eine-das-gegenteil.md`.

Daneben, ohne eigenen Datensatz: die Ausnahme „Platzhaltername“ (`baum.rs:1820-1835`) gilt für jedes Literal am Kopf eines Tupels vor einem Komma innerhalb einer Senkenargumentliste und nicht nur für die Platzhalter von `satz`/`anzahl`, wie der Doc-Kommentar (`:1755-1758`) sagt. Ein Tupel `("Abbrechen", Taste::Esc)` in einer Senke ginge durch. Heute trägt keine Senke eine solche Signatur (`Schaltflaeche::neu` nimmt den Titel als erstes Argument und nicht im Tupel). Der Befund ist ein Hinweis für den, der eine Senke mit Tupelliste baut.

## Thema 4: Übersetzungen (Stichprobe)

Gelesen: die Befehlsnamen in `fr.rs` (vollständig, `kommandoname` und `zugestellt_name`) und eine Auswahl in `en.rs`; in `fr.rs` die Einträge zu Wirkungsbereich, Ortsmangel, Vorgangsgründen, Menü, Kontextmenü, Spalten, Statuszeile, Vorgangszeile, Löschwarnung, Konflikt-, PIN- und Ungesichert-Blatt sowie die Zahlwörter; die Funktionsbereiche in allen drei. Keine Befehlsbezeichnung kommt je Sprache doppelt vor. Die Platzhalter, die Typografie und die Schwellenzahl halten Proben.

Keine sinnentstellenden Einträge gefunden, außer dem Zahlwortbefund aus Thema 1. Für die Durchsicht des Nutzers (Plan, `## Where this work stops`, Teil 3), ohne Datensatz:

- `fr.rs` übersetzt die Taste Return in Erläuterungen mit „Entrée“ (Glossar in `tabelle/mod.rs:168-169`), etwa `KonfliktTastenhinweisMehrereZiele` „Entrée ignore, Cmd+Entrée remplace, …“. In derselben Tabelle heißt „entrée“ durchgehend „Eintrag“. speculation: macOS nennt die Taste ↩ auf Französisch „Retour“ und ⌤ „Entrée“; geprüft ist das nicht.
- `fr.rs:1088-1091`, „Sélection une entrée vers le haut“ und die drei Geschwister, sind kein grammatischer Satz.
- `fr.rs:562`, `VorgangAbgebrochen` „… {uebertragen} transférés“ steht immer in der Mehrzahl, auch bei „une entrée“.

## Thema 5: CLAUDE.md, README.md, HowTo.md

Die Aussagen zu `festlegen`, `geltende()`, dem Startweg über das Bündel, den Pflichtstellen eines neuen Kommandos, `Kontextbefehl::titel` und dem Bündelaufbau stimmen am Code. `make tasten`, `make menue` und `make run` starten das Bündel (`Makefile:23`, `:97-111`).

**Befund (Low):** `CLAUDE.md:245` sagt, die Umlautprobe lese die Terminalausgaben aus der offenen Frage mit; sie liest `krk-core/src` und `krk-ui/src`, also den Messmodus, aber nicht `xtask` und `krk-bench` (`baum.rs:1522-1531`). Datensatz `261001-1929_*_claude-md-sagt-die-umlautprobe-lese-die-terminalausgaben-mit-sie-liest-xtask-und-krk-bench-nicht.md`.

**Befund (Low):** `HowTo.md:33-37` zählt das Datumsformat zu dem, was der Sprachwahl folgt; `resources/Info.plist:73-75` sagt, `NSDateFormatter` folge der Region und nicht der Sprachliste. Datensatz `261001-1929_*_howto-md-sagt-das-datumsformat-folge-der-sprachwahl-die-info-plist-sagt-es-folge-der-region.md`.

## Querschnitt

- **Eine Regel der Tabelle wird vom Rufer geschützt und nicht von der Tabelle.** Die Nullfälle im Zahlwortbefund sind an vier Stellen durch einen Zweig im Rufer abgefangen (`statuszeile.rs:544-546`, `operationen.rs:804`, `:811`, `editor.rs:859`) und an drei nicht (`auswahl.rs:55`, `operationen.rs:736`, `:791`). Dieselbe Bauart hat die Senkenprobe: sie hält die Aufrufstelle und nicht den Weg dahinter. Beide Lücken schließt kein weiterer Rufer-Zweig, sondern eine Eigenschaft der Tabelle (Thema 1) und eine Senkenliste, die die tatsächlichen `&str`-Eintritte nennt (Thema 3).
- **Die Proben auf Eindeutigkeit von Beschriftungen prüfen allein Deutsch**, weil `cargo test` die Sprache nicht umschalten kann. Für die Git-Marken ist das schon über `Sprache::ALLE` gelöst (`git/texte.rs`, Prüfmodul); für `Bereich`, `Spalte`, `Wirkungsbereich` und die Belegungsnamen nicht. Am Stand `e46a678` gibt es in keiner Sprache eine Dopplung (nachgezählt), also kein Datensatz.

## Empfohlene Reihenfolge

1. Vor jeder Auslieferung: der Zahlwortbefund (High). Er ist in der französischen Oberfläche bei jedem Kopiervorgang und jedem Markieren ohne Ordner sichtbar.
2. Mit der nächsten Arbeit an den Proben: die Senkenprobe (Medium).
3. Aufräumen: die zwei Dokumentationsbefunde (Low). Der zum Datumsformat lässt sich mit dem Abnahmelauf des Nutzers (Teil 2) entscheiden.

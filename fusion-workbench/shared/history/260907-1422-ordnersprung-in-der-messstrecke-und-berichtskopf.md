# Die Messstrecke bekommt einen Ordnersprung, und der Berichtskopf nennt die Änderung

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Zwei beantwortete Entscheidungen in Code bringen. Erstens: die Sitzungsstrecke bekommt zusätzlich einen Ordnersprung, damit die Vorschauarbeit eines Ordners überhaupt in eine gemessene Spanne fällt (`260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`, Möglichkeit 2). Zweitens: der Kopf des nächsten Messberichts nennt, dass der Lauf vom 260810 und der nächste nicht mehr dasselbe messen (`260827-1322_*_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md`, Möglichkeit 1).

## Wie die Strecke gebaut ist

Die Sitzungsstrecke (S21) läuft in `crates/krk-ui/src/messmodus.rs` als Liste von `Sitzungsschritt`: warten, ungemessen handeln, Taste drücken, aufräumen. Eine Taste mit `messung: Some(groesse)` beginnt eine Spanne; sie endet an der ersten Bildgrenze, an der `sitzungsmessung_fertig` für diese Größe wahr wird. Die Werte gehen als Zeilen `wert <name> <nanos>` an `krk-bench`, das sie in `Gesamtlauf::eine_gesamtrunde` über `hole(<name>)` einsammelt und in `Gesamtlauf::fahren` als `Zusage` mit Kennung und Abnahmemaß zusammensetzt.

Eine Zusage aus C8 und eine gemessene Spanne sind dabei nicht dasselbe: L5 trägt seit jeher zwei Spannen (Tabwechsel, Fensterwechsel), L10 ebenfalls zwei. Genau diese Bauform trägt jetzt auch L7.

## Angefasst ist L7

`Sitzungsgroesse::L7Ordner`, Messzeile `l7-ordner`, zweite Zeile im Bericht unter der Kennung `L7` — dieselben 100 ms, dasselbe Perzentilmaß. Keine neue Kennung: `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u` liefert vorher wie nachher L1 bis L10, geprüft gegen `git show HEAD:…`.

## Der gemessene Ordner, und warum dieser

Der L6-Unterordner (`<a>-l6`, 1.000 Einträge). Im Messmodus lädt die Anwendung `readers.toml` nicht (`Anwendungsdelegierter::sitzung_laden`), also erkennt kein Profil aus der Datei irgendetwas, und was jeden Ordner auswertet, ist das eingebaute Default-Profil: drei Zählzeilen auf den Ordner selbst, also ein Leselauf über seinen Inhalt. Was dieser Leselauf kostet, hängt damit allein an der Eintragszahl — und der L6-Unterordner ist der einzige Ordner des Messplatzes, dessen Eintragszahl der Messplan zusagt und die Prüfung vor dem Lauf hält. Die Unterordner in Prüfordner A sind leer; ein Sprung auf einen von ihnen misste einen Leselauf ohne Ergebnis, also fast dasselbe wie der Dateisprung darüber.

Die Anforderung des Auftrags, der gemessene Ordner solle ein Ort sein, „für den ein Leseprofil greift“, ist damit in der einzigen Lesart eingelöst, die im Messmodus erreichbar ist. Ein Profil aus `readers.toml` kann dort nicht greifen; das wäre Möglichkeit 3 des Entscheids vom 260824-1900, und die ist nicht gewählt. Die Frage ist als Entscheidungsdatensatz abgelegt (siehe unten).

## Der Ablauf des Sprungs

In den Elternordner, den Unterordner ungemessen auswählen, seine Vorschau abwarten (der Vorlauf gegen die kalte Zahl in der warmen Reihe), danach je Wiederholung: ein ungemessener Pfeil hoch, die abgewartete Vorschau des Nachbarn, der gemessene Pfeil ab zurück auf den Unterordner. Pfeil hoch und Pfeil ab landen wieder auf demselben Eintrag, gleich welcher Nachbar darübersteht; die Reihe hängt an keiner Sortierannahme.

Der Warteschritt dazwischen ist neu (`Bedingung::VorschauStehtWoanders`) und trägt den Zielpfad, weil ein bloßes „die Vorschau steht“ im Stand unmittelbar davor schon erfüllt wäre: solange der ungemessene Pfeil hoch die Ereignisschlange nicht verlassen hat, steht die Vorschau unverändert auf dem Zielordner. Die Endbedingung ist aus demselben Grund auf den Unterordner festgenagelt und nicht auf „irgendeinen ausgewählten Eintrag“: landet die Auswahl woanders, hält der Lauf nach der Geduld unter dem Namen `l7-ordner` an, statt eine Zahl auszugeben, die etwas anderes gemessen hat.

## Der Berichtskopf

Neue Kopfzeile „Messgegenstand L7“ in `gesamt_verfassen`, Text als Konstante `MESSGEGENSTAND_L7` an einer Stelle. Sie nennt beide Änderungen in einem Satz — das Default-Profil seit der Runde 19 und den Ordnersprung seit dem 260907 —, hält fest, dass die Zusage selbst unberührt ist, und sagt ausdrücklich, dass die Reihe vom 260810 und diese nicht gegeneinander zu halten sind.

## Geänderte Dateien

- `crates/krk-ui/src/messmodus.rs`
- `crates/krk-bench/src/messen.rs`
- `crates/krk-bench/src/bericht.rs`

## Was geprüft ist

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` und `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — alle fünf mit Ausgang 0.

## Was ungemessen bleibt

Der Abnahmelauf selbst. Er verlangt KRK im Vordergrund und ist Nutzerarbeit; kein Agent kann ihn fahren. Ungemessen ist damit alles, was erst am laufenden Bündel sichtbar wird: ob der Ordnersprung zwanzig Werte liefert, ob er die 100 ms von L7 hält, ob der zusätzliche Leselauf über 1.000 Einträge die Reihe verschiebt, und ob die Wartebedingung in der Praxis so lange hält, wie die Rechnung es sagt. Grün sind allein die Proben ohne Fenster.

## Abgelegte Datensätze

- `260907-1422_o_misst-der-ordnersprung-je-einen-ort-fuer-den-ein-profil-aus-readers-toml-greift.md` (Entscheidung)
- `260907-1422_o_messung-unmoeglich-faengt-eine-neue-messgroesse-still-auf-statt-den-bau-anzuhalten.md` (Defekt)

# Die Faltung des Schnitts reicht über ASCII hinaus, und der Doc-Kommentar nennt beide Ungenauigkeiten

**Status:** Complete
**Agent:** coder
**Datum:** 260825-1416
**Baumstand bei Beginn:** `863e8c5`

## Auftrag

Den letzten offenen Befund der Runde 17 beheben:
`issues/260825-1358_*_die-faltung-des-schnitts-gilt-nur-ascii-und-der-doc-kommentar-nennt-allein-die-andere-ungenauigkeit.md`,
aus der vierten Durchsicht
(`reviews/260825-1358-coderev-runde-17-vierte-durchsicht-der-behebungscommit.md`, Abschnitt 1).
Der Nutzer hat Weg 3 verlangt: die Faltung weiten **und** den Doc-Kommentar ehrlich machen.
Die Nutzerwahl vom 260825 („falten", kein Dateizugriff im Schnitt) bleibt unangetastet;
diese Behebung weitet sie und ändert sie nicht.

## Geändert

Eine Codedatei, ein Datensatz.

1. `crates/krk-ui/src/kommandos/kontextmenue.rs`
2. `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-1358_c_…` (`Resolved:` angehängt, `_o_` → `_c_`)

## Was geschehen ist

**Die Faltung.** `gleicher_eintrag` vergleicht den letzten Bestandteil jetzt über
`to_string_lossy().to_lowercase()` statt über `eq_ignore_ascii_case`. Der Elternteil bleibt
buchstabengetreu und wird weiterhin **zuerst** gefragt, sodass die zwei Umschriften nur dort
entstehen, wo sie etwas entscheiden. Die Regel steht unverändert genau einmal da; beide Rufer
erben sie. Kein Dateizugriff, kein neues fremdes Paket, L9 unberührt.

**Der Nachbar im Baum, nachgesehen wie verlangt.**
`krk_core::verzeichnis::filter::traegt_die_folge` ist der eine Vergleich des Filters und faltet
mit `to_lowercase()`, also über ganz Unicode. Die Formulierung „faltet keine Umlaute" in
`CLAUDE.md` und im Doc-Kommentar jener Funktion meint die **Normalisierung** `ä`→`a` und
`é`→`e`, nicht die Groß- und Kleinschreibung; die Probe
`der_vergleich_faltet_keine_umlaute_und_keine_akzente` (`filter.rs`) schreibt beides
nebeneinander aus: `traegt_die_folge("Äpfel.txt", "apfel")` ist falsch, `("Äpfel.txt", "äpfel")`
ist wahr. Der Filter faltet `Ä` auf `ä` also sehr wohl. Damit ist `to_lowercase` die im Baum
vorhandene Art zu falten, und eine zweite entsteht hier nicht. Der Vergleich selbst ist **nicht**
wiederverwendet: `traegt_die_folge` fragt nach einer Teilzeichenfolge, `gleicher_eintrag` nach
Gleichheit eines Pfadbestandteils. Das sind zwei Fragen, und eine Funktion für beide wäre die
Zusammenlegung zweier Regeln und nicht die Vermeidung einer zweiten.

**Der Doc-Kommentar.** Die Überschrift bei `ist_ziel_des_laufs` verspricht nicht mehr „ohne
Rücksicht auf Groß- und Kleinschreibung", sondern sagt, was der Rumpf tut: verglichen wird die
kleingeschriebene Fassung. Darunter steht neu, warum über ganz Unicode und nicht über ASCII
gefaltet wird, mit dem Fall `übersicht.zip` gegen `Übersicht.zip` und dem Verweis auf den
Datensatz. Aus der einen Ungenauigkeit sind zwei geworden, eine nach jeder Seite:

- *zu weit* auf einem groß-/kleinschreibungsempfindlich formatierten Datenträger — der bisherige
  Absatz, unverändert in der Sache;
- *zu eng* bei zusammengesetzten Zeichen: `Ü` als ein Zeichen (NFC) gegen `U` mit nachgestelltem
  Trema (NFD). Das Kleinschreiben rührt an der Zerlegung nicht, APFS in der Vorgabe hält beide für
  einen Eintrag. Die Antwort wäre eine Normalform und damit eine Zerlegungstabelle; die gäbe es
  nur als fremdes Paket, und das nimmt dieses Vorhaben dafür nicht auf.

Daneben steht, dass `Straße` gegen `STRASSE` kein Unterschied zum Bauziel ist: APFS faltet das
ebenso wenig. Bei `gleicher_eintrag` selbst steht jetzt, dass ein Name ohne gültiges UTF-8 durch
`to_string_lossy` geht und zwei solcher Namen dadurch für einen gehalten werden können — dieselbe
Seite wie die zu weite Faltung, mit demselben Ausgang, und auf dem Bauziel entsteht der Fall
nicht, weil APFS solche Namen nicht annimmt. Der Weg ist derselbe, den `archivname` und
`ordnername_zum_archiv` in dieser Datei schon gehen.

## Eine Folge, mitvermerkt statt bloß gemacht

`ohne_die_eigenen_ziele` ordnet nach der Bytelänge des Archivpfads, und der Beweis, dass ein
Beansprucher vor dem Beanspruchten drankommt, ruhte bis heute darauf, dass
`eq_ignore_ascii_case` nur gleich lange Bytefolgen trifft — die vierte Durchsicht hat genau das
als dritten Schritt nachgerechnet. Das Kleinschreiben über Unicode kann die Bytelänge ändern.
Gemessen über alle Codepunkte (`to_lowercase` gegen `len_utf8`, außerhalb des Baums gefahren):
24 Zeichen verlieren dabei Bytes, davon allein das Kelvinzeichen `K` (`U+212A`) zwei, die
übrigen je eines; drei Zeichen gewinnen eines (`İ`, `Ⱥ`, `Ⱦ`). Der Vorsprung von vier Bytes trägt
bis zu zwei solchen Zeichen in einem Namen; zwei Kelvinzeichen stellen die Längen gleich, drei
kehren die Ordnung um, und dahinter bliebe der Beanspruchte stehen — der Ausgang des Befundes
vom 260825. Der Doc-Kommentar von `ohne_die_eigenen_ziele` sagt das jetzt aus, samt dem Weg, der
die Grenze schlösse: nach der kleingeschriebenen Länge ordnen statt nach der geschriebenen.
Gebaut ist er nicht — ein Archivname mit drei Kelvinzeichen ist kein Fall dieses Vorhabens, und
die Prosa steht da, damit die nächste Runde die Ordnung nicht für bewiesen hält.

## Proben

Drei neue, im Prüfmodul von `kontextmenue.rs` neben die drei der ASCII-Faltung gestellt:

- `das_archiv_des_vorigen_laufs_faellt_auch_mit_umlaut` — im Ordner `Übersicht` liegt
  `übersicht.zip`, `archivname` rechnet `Übersicht.zip`, der Eintrag fällt aus den Quellen.
- `der_entpackschnitt_trifft_auch_mit_umlaut` — `äpfel.zip` neben `Äpfel.zip.zip`, ein Paar
  bleibt, `ausgelassen: 1`.
- `ein_zerlegt_geschriebener_umlaut_bleibt_quelle` — die verbliebene Enge, festgehalten: NFD und
  NFC sind hier zwei Einträge.

**Gegenprobe gefahren.** Die Weitung versuchsweise auf `eq_ignore_ascii_case` zurückgenommen: die
ersten zwei Proben werden rot (`test result: FAILED. 32 passed; 2 failed`), die dritte bleibt
grün, wie es sein muss — sie hält eine Grenze, die unter beiden Fassungen gilt. Danach
wiederhergestellt und der Rumpf gegengelesen.

## Prüfung

`make check` → Exit 0 (Bau, Proben, `clippy --workspace --all-targets -- -D warnings`,
`fmt --all --check`). `rustfmt` hat den Vergleich in eine Zeile gezogen; das ist die einzige
Formänderung gegenüber dem, was von Hand dastand. Die neuen Verweise im Doc-Kommentar
(`krk_core::verzeichnis::filter::traegt_die_folge`, `std::ffi::OsStr::to_string_lossy`) lösen
auf — `cargo doc -p krk-ui --document-private-items` nennt keinen davon unter seinen
Fehlern, und die Fehler, die es nennt, bestanden vorher.

## Was nicht geschehen ist

Kein Commit, kein `git add`. `CLAUDE.md` nicht angefasst: keine ihrer Aussagen wird durch diese
Änderung falsch, und die Zeile zum Filter („faltet keine Umlaute") bleibt richtig, weil sie die
Normalisierung meint. Kein neuer Datensatz für die NFD-Enge — der Nutzer hat ausdrücklich
verlangt, dass sie im Doc-Kommentar ausgeschrieben steht, und dort steht sie, dazu in einer Probe.

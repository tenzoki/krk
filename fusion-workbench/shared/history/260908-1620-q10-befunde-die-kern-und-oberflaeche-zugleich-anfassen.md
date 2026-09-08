# Q10: die offenen Befunde, deren Behebung `krk-core` und `krk-ui` zugleich anfasst

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@qantr.com>

## Was verlangt war

Die offenen Defektdatensaetze abarbeiten, deren Behebung **beide** Kisten anfasst — die Bahn,
die keine der drei Codebahnen vom 260908 nehmen konnte, ohne einer anderen in die Datei zu
greifen. Jeden gegen den heutigen Baum lesen, bevor er behoben wird. Nicht committen. HEAD
war `2f4b7b2`.

## Die Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

**124 offene Datensaetze** am 260908 gegen `2f4b7b2`, nicht mehr die 189 der Schleife vom selben Tag: die drei
Codebahnen und die Werkbankbahn haben seither 65 geschlossen.

Geschnitten wurde je Datensatz danach, ob **beide** Kistennamen darin vorkommen:

```sh
… | while read -r f; do
  c=$(grep -c 'krk-core' "$f"); u=$(grep -c 'krk-ui' "$f")
  [ "$c" -gt 0 ] && [ "$u" -gt 0 ] && echo "$f"
done
```

**34 Datensaetze**, nicht die im Auftrag geschaetzten rund 39. Der strengere Schnitt ueber die
Pfade (`crates/krk-core/` und `crates/krk-ui/`) liefert 30; die vier Datensaetze dazwischen
nennen beide Kisten im Text, ohne einen Pfad zu fuehren, und drei von ihnen liegen wirklich in
dieser Bahn. Gefahren ist deshalb der weitere Schnitt.

**Aus den drei Verlaufsprotokollen uebernommen:** die Zuordnung der 33 liegengelassenen
Datensaetze aus `260908-0838-q3-appkit-befunde.md` (Abschnitt „Fuehrt aus der Bahn hinaus"),
die Liste der in Q4 offen gelassenen mit Kernbezug und die drei, die Q2 wegen ihres
`krk-ui`-Anteils stehen liess (`freier_name`, `lesen` und der Deskriptormangel, das
Zaehlkommando). Sie sparen die Erhebung nicht, aber sie nennen den Zuschnitt.

## Behoben (17)

| Datensatz | Was gebaut ist | Dateien |
|---|---|---|
| `260812-0415` Spalten und Sortierschluessel | `schluessel_der_spalte` im Probenmodul, zwei Proben ueber beide Richtungen | `krk-ui/src/spalten.rs` |
| `260813-0715` atomar-Probe | vierte Nadel `atomar as`, die Wiederausfuhr als benannte Blindheit, der Vollstaendigkeitssatz gestrichen | `krk-core/tests/baum.rs` |
| `260813-0716` bewachte Luecke | der Kopf sagt, welche Haelfte bewacht ist und welche nicht; die Zahl durch ein Zaehlkommando ersetzt | `krk-core/src/ablage/mod.rs` |
| `260813-0719` Sitzungsschreiber | beide Stellen sagen, was der Uebersetzer haelt und was der Aufrufer halten muss | `krk-core/src/ablage/sitzung.rs`, `krk-ui/src/appkit/anwendung.rs` |
| `260813-0720` C4.6-Nadel | `krk-messplatz` als zweite Ortsnadel, die dritte Blindheit benannt | `krk-core/tests/baum.rs` |
| `260813-1110` Ueber-Dialog | Berichtigung als Nachsatz am Entscheid: die zwei untauglichen Beispiele, die Reichweite, das Erhebungskommando statt der Zahl | Circle-Datensatz |
| `260816-1934` sechs Prosastellen | fuenf behoben, zwei waren abgetragen, eine siebte derselben Bauart gefunden und mitbehoben | fuenf Dateien in beiden Kisten |
| `260823-1433` `kommando_ausfuehren` | die vierte und letzte Codestelle traegt die bedingte Fassung | `krk-ui/src/kommandos/rundweg.rs` |
| `260823-1439` Zeilenzitate | alle fuenf auf Datei plus Namen gezogen; die eine, die die Tafel als haltend fuehrte, hielt nicht mehr | `krk-core/src/tasten/belegung.rs`, `krk-ui/src/hervorhebung.rs`, `krk-ui/src/appkit/tabelle.rs` |
| `260826-1221` `Abschluss::ist_abgebrochen` | Weg 2: der Grund am Doc-Kommentar, mit Zaehlkommando und der Anweisung, wann er zu streichen ist | `krk-core/src/verzeichnis/leser.rs` |
| `260826-1223` Deskriptormangel | Feld `mangel` durch beide Aufzaehlungen, eigener Satz im Editor, Kindprobe unter `ulimit -n 64` | `krk-core/src/text/datei.rs`, `krk-core/src/ablage/mod.rs`, `krk-core/tests/text.rs` |
| `260826-1223` Zusteller | `gehalten_von` kommt aus dem Wortschatz; `name` und `reserviert_fuer` bleiben Sache der Datei | `krk-core/src/tasten/belegung.rs`, `krk-ui/src/belegungsausgabe.rs` |
| `260826-1933` `mit_zeitschranke` | beide Eigenfassungen auf die gemeinsame gezogen, die ueberholte Begruendung gestrichen | `krk-core/tests/verzeichnis.rs` |
| `260826-2156` „genau einmal" | die Vielfachheit wird gezaehlt statt als Menge verglichen; der Name traegt jetzt | `krk-core/tests/belegung.rs` |
| `260828-1046` Variantenleser | `varianten` und `codezeilen` in `quellbaum.rs`, neun Rufer, eine Zaehlprobe | vier Dateien in `krk-ui` |
| `260828-1046` http/https | `ist_webschema` im Kern, beide Wege rufen es, Probe umgezogen | `krk-core/src/zwischenablage.rs` und drei weitere |
| `260907-0858` ALLE-Listen | eine Ausnahme weniger, zwei eigene Proben, ein Blockleser fuer beide Nadeln | `krk-core/tests/{gemeinsam,baum}.rs`, `krk-ui/src/kommandos/loeschwarnung.rs` |

Die Einzelheiten stehen je im `Resolved:`-Nachsatz des Datensatzes.

## Zwei Stellen, an denen der Baum den Datensatz berichtigt hat

**`260826-1223` (Zusteller): der vorgeschlagene Schnitt war zu breit.** Der Datensatz nennt
`name`, `reserviert_fuer` und `gehalten_von` als „billigsten Schnitt", weil `bauen` die
Wortschatz-Funktion ohnehin zur Hand hat. Gebaut und zurueckgenommen: alle drei aus dem
Wortschatz zu nehmen macht zwei Proben rot, und beide zeigen einen echten Verlust — die
Maskierung des senkrechten Strichs braucht einen Namen aus der Nutzerdatei, um ueberhaupt
einen Strich in die Tabelle zu bekommen, und `krk-ui/src/belegungsmodell.rs` rechnet
ausdruecklich mit einer alten `keymap.toml`, die `reserviert_fuer` noch traegt. Gebaut ist
deshalb allein `gehalten_von`, das einzige der drei, das entscheidet, ob ein Befehl ankommt.

**`260823-1439` (Zeilenzitate): die Gegenprobe der Tafel ist selbst widerlegt.** Die Tafel
fuehrt ein fuenftes Zitat als das eine, das haelt („ein Zitat auf einen selten angefassten
Abschnitt haelt jahrelang"). Am 260908 traf auch dieses nicht mehr. Damit sind alle fuenf
falsch geworden, und das Argument der Tafel ist gegenstandslos.

## Zwei Folgen, die ueber den Datensatz hinausgehen

**Ein Auffangzweig ist unerreichbar geworden.** Mit dem Zusteller aus dem Wortschatz ist der
Weg zu `belegungsausgabe::wirkung`s Auffangzweig zu: kein Eintrag der Auslieferungsbelegung
traegt einen Zusteller **und** eine Kennung aus `Kommando::KENNUNGEN`. Der Zweig bleibt
stehen — der `match` laeuft ueber `&str` und braucht ohnehin einen —, und die Probe, die ihn
bisher ausloeste, ist durch `keine_ausgelieferte_funktion_traegt_zusteller_und_kommando`
ersetzt: sie misst die Unerreichbarkeit, statt sie zu verabreden, und wird rot, sobald eine
spaetere Runde einem gebauten Befehl ein `gehalten_von` gibt.

**Eine Probe hat den Defekt als richtiges Verhalten gemessen.**
`der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege` gab `fenster_schliessen` in
ihrer Nutzerdatei ein `gehalten_von = "menue"` und hielt danach fest, dass der Befehl kein
Kommando mehr liefert — genau die Wirkung, die `260826-1223` als Defekt fuehrt. Sie misst die
vierte Stelle der Zustellerregel jetzt an `text_alles_auswaehlen`, das den Zusteller ab Werk
traegt.

## Offen gelassen, mit Grund

**Verlangt eine Wahl, die der Datensatz offen laesst (11).**
`260812-1204` (zwei Wege, beide mit eigenem Preis) ·
`260813-0540` („der Nutzer entscheidet") ·
`260813-1110` Wegwerfordner in `xtask` („eine der beiden Fassungen waehlen, nicht beide") ·
`260815-0230` `zeile_traegt` („eine Frage an den Nutzer und keine an den Umsetzer") ·
`260816-1932` Deskriptormangel im Durchlauf (die Anzeige „unvollstaendiger Filterstand" hat
keine Gestalt) ·
`260816-2144` Leertaste (drei Wege, jeder bricht eine Zusage) ·
`260821-0142` unlesbare Ablagedatei (drei Fragen, keine aus dem Baum zu beantworten) ·
`260826-1221` `freier_name` (`Option<String>` stellt die zweite Frage mit, was das
Konfliktblatt vorschlaegt, wenn kein freier Name da ist) ·
`260826-1221` Kollisionspruefung (eigene Faltung oder ein Systemaufruf je Zeile) ·
`260826-1302` `deny(unsafe_code)` in den Probenzielen („zu waehlen ist zwischen 1 und 2; das
ist eine Nutzerfrage") ·
`260907-0858` Rechtesperre in `krk-ui` („welches von beidem, ist nicht entschieden").

**Verlangt eine Entscheidung ueber die Bauvoraussetzungen (1).**
`260905-0406` atomares Schreiben: fuenf der sechs Punkte brauchen `libc` in `krk-core`. Der
sechste — eine symbolische Verknuepfung, die das Sichern ersetzt — ist ohne `libc` behebbar,
und er ist bewusst **nicht** gebaut: eine Aenderung am Schreibweg des Editors ohne Auftrag
waere genau die Sorte Griff, gegen die die Vorgeschichte dieses Datensatzes steht.

**Braucht einen roten Lauf mit erhaltener Ausgabe (1).**
`260823-1210` `make check` mit Rueckgabewert 2.

**Ist eine Frage an die Form der Werkbankdatensaetze (1).**
`260823-1336` Zeilenzitate in Datensaetzen („das gehoert entschieden und nicht nebenbei
gegriffen").

**Gehoert dem `ontocoder` (2).**
`260827-1911` (`resources/default-readers.toml`) und `260831-1355`
(`resources/default-keymap.toml`); der zweite fuehrt „Domain: data" selbst.

**Teilweise behoben, Datensatz bleibt offen (1).**
`260826-1221` fuenf oeffentliche Namen: die zwei, fuer die der Datensatz die Antwort selbst
gibt (`MELDEABSTAND`, `HOECHSTE_STELLENZAHL`), sind aus ihrer Modulwurzel gestrichen; die drei
mit „heisst es zu entscheiden" (`Regel::ist_wirkungslos`, `Lauf::warten`,
`operation::Abschluss::ist_abgebrochen`) bleiben.

**Am Code fertig, es fehlt der Klick am Buendel (1).**
`260814-1612` Ordnerverknuepfung betreten. Unveraendert wie am 260819-1440; kein Agent kann
den Lauf fahren.

## Ohne Aenderung geschlossen

Keiner. Jeder der 17 geschlossenen Datensaetze bestand am heutigen Baum fort, und jeder ist
einzeln gegen ihn gelesen; wo eine Teilbehebung schon dastand (`260816-1934`: zwei von sechs;
`260823-1439`: drei von fuenf), ist sie im Abgleich benannt.

## Neue Datensaetze

Keine. Was neben der Bahn auffiel, ist als Abgleich an den jeweils vorhandenen offenen
Datensatz geschrieben — drei Stellen:

- `260815-0230`: die zwei Vergleiche sind seit der Runde 21 **auseinandergelaufen**, und C1.3
  der Runde 10 ist damit falsch. Der Unterschied steht jetzt am Doc-Kommentar von
  `Belegungsmodell::zeile_traegt`, statt hinter einem Kriterium zu verschwinden, das ihn
  bestreitet.
- `260816-2144`: der Punkt „Nicht geprueft" ist geprueft. Die Tippsuche der Belegungsansicht
  bekommt die Leertaste sehr wohl, weil der Faenger **vor** dem Nachschlag laeuft; der
  Dateifilter bekommt sie nie.
- `260814-1612`: unveraendert, Marker bleibt `_o_`.

## Abnahme

Am 260908 gefahren, alle fuenf:

```
cargo build --workspace                                     exit 0
cargo test --workspace                                      exit 0
cargo clippy --workspace --all-targets -- -D warnings       exit 0
cargo fmt --all --check                                     exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps   exit 0
```

Nicht committet, wie beauftragt.

## Eine Anmerkung zum parallelen Betrieb

**Der Baum war nicht allein meiner.** Waehrend dieses Durchgangs lief eine zweite Bahn in
`fusion-workbench/` (Werkbank-Buchhaltung, `260908-1539-coder-werkbank-buchhaltung-befunde-ohne-codeziel.md`);
sie hat drei Datensaetze geschlossen, drei neue abgelegt und mehrere Circle-Datensaetze
geaendert. Keine ihrer Aenderungen liegt unter `crates/`, und keiner der von ihr angefassten
Datensaetze steht in dieser Bahn. `cargo fmt --all` war deshalb unschaedlich; nachgesehen mit
`git status --porcelain crates/`, das genau die dreissig Dateien dieser Bahn nennt.

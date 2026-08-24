# Räumung der Code-Befunde aus zwei Durchsichten

**Status:** Complete
**Agent:** coder
**Datum:** 260824-1246
**Baumstand beim Beginn:** `615190a`

Kein Planschritt, sondern eine Befundräumung: der Sicherungsschalter hat bei 14 offenen
Befunden gegen 7 erledigte Schritte angeschlagen, und der Nutzer hat die Räumung gewählt. Diese
Sitzung hat die **acht Code-Befunde** genommen; die Spec- und Planbuchführung lief parallel bei
einem zweiten Agenten, und Spec wie Plan sind hier ausdrücklich nicht angefasst.

## Verifikation

```
make check — exit 0
```

Vier Kommandos grün: `cargo build --workspace`, `cargo test --workspace` (1.497 Proben),
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`.

## Die acht Befunde

| Datensatz | Gegenstand | Ausgang |
|---|---|---|
| `260824-1214` | `zusammenfassen` liefert für eine Datei `Some` | behoben, Marker `_c_` |
| `260824-1215` | „über 1" für 2.101 Einträge | behoben (Code), Buchführung bleibt offen |
| `260824-1216` | zwei Bausteintische schweigend angenommen | behoben, Marker `_c_` |
| `260824-1217` | Tippfehler im Tisch, Meldung ohne Gegenstand | behoben, ein Teil ausgelagert |
| `260824-1218` | Probe lässt zwei Ergebnisse zu | behoben, Marker `_c_` |
| `260824-0940` | `readers.toml` fällt beim `zip` still heraus | behoben, Marker `_c_` |
| `260824-1014` (Ablage) | vierzehn Prosastellen sagen „vier" | behoben, Marker `_c_` |
| `260824-1014` (`datei.rs`) | Messung im Präsens über einen Werkbankdatensatz | behoben, Marker `_c_` |

## Was gebaut wurde

### C2.6 hängt jetzt an der Stelle, die sie hält

`leseprofil::bausteine::gezaehlt` fragt nach `canonicalize` mit `std::fs::metadata`, ob der
aufgelöste Pfad ein Verzeichnis benennt, und liefert sonst `None`. Der Datensatz bot zwei Wege
an; gewählt ist der zweite, die Frage im Kern zu entscheiden, weil der erste (ein Absatz im
Doc-Kommentar) nach eigener Aussage nichts hält. Der Doc-Kommentar steht daneben und begründet,
warum die Prüfung nicht beim Aufrufer sitzt: der erste Erkennungsdurchgang sieht allein auf den
Pfadtext, also träfe ein Profil mit Pfadmuster eine Datei genauso wie den Ordner daneben.

**Am aufgelösten Pfad und nicht am ausgewählten**, damit eine Verknüpfung auf eine Datei als
Datei zählt. Ein Systemaufruf je Zusammenfassung, kein Leselauf und keine Öffnung: der Haushalt
aus C6 zählt die zwei letzteren und bleibt unberührt.

Die neue Probe `auf_eine_datei_greift_kein_profil_auch_bei_passendem_pfadmuster` ist gegen den
Baum **ohne** die Prüfung gefahren worden und wurde rot (`die Datei _t_circle.md hat eine
Zusammenfassung bekommen`).

### Eine Änderung für zwei Befunde: die Gestalt der Profilzeile

`260824-1216` und `260824-1217` hatten dieselbe Wurzel, die unmarkierte Auswahl hinter
`#[serde(flatten)]`. Beide sind mit **einer** Änderung erledigt statt mit zwei nebeneinander:

```text
vorher                                    jetzt
────────────────────────────────────      ─────────────────────────────────────
Zeilendatei                               Zeilendatei  (deny_unknown_fields)
  beschriftung: String                      beschriftung: String
  #[serde(flatten)]                         zaehlung:      Option<Zaehlungsdatei>
  baustein: Bausteindatei                   juengste:      Option<Juengstedatei>
            └ #[serde(untagged)]            feld:          Option<Felddatei>
                                            vorhandensein: Option<Vorhandenseindatei>
                                                    │
                                          Zeilendatei::zerlegen zählt sie
                                                    │
                                          genau eine ─> Bausteindatei (ohne Deserialize)
                                          keine, zwei ─> Grund mit Meldung
```

Was daraus folgt, jedes einzeln geprüft:

- Zwei Tische in einer Zeile nehmen der Zeile ihren Baustein, und die Meldung nennt Profilnamen,
  Beschriftung und die zwei gefundenen Tischnamen.
- Eine Zeile ganz ohne Tisch bekommt dieselbe Behandlung; ihre Meldung zählt die vier möglichen
  Namen auf.
- Ein zusätzlicher Schlüssel neben der Beschriftung fällt jetzt auf. Er tat es vorher nicht,
  weil `flatten` und `deny_unknown_fields` einander ausschließen.
- Die Meldung von `serde` nennt den verschriebenen Schlüssel: `unknown field \`mustre\`, expected
  \`ordner\` or \`muster\`` statt `data did not match any variant of untagged enum Bausteindatei`.
  Die unmarkierte Auswahl verwarf die Meldung des Tisches unterwegs; ohne sie kommt sie durch.
- Der Vorbehalt über die Verbindung aus `flatten` und `untagged`, den beide Modulköpfe trugen,
  ist gegenstandslos: beide Sonderwege sind weg, und die Rundreise nimmt vier gewöhnliche
  `Option`-Felder ab.

**Die Gestalt der Datei ist unverändert.** Der Nutzer schreibt weiter `zaehlung = { … }` neben
seine Beschriftung; der im Plan vorgesehene Ausweichweg (ein Feld `baustein = "zaehlung"` als
ausgeschriebene Sortenkennung) kostet eine Zeile je Profilzeile und ist nicht gezogen. Die
dritte Möglichkeit des Datensatzes — den Verlust hinnehmen und in `default-readers.toml`
ausschreiben — war damit nicht nötig.

**Die Reichweite eines Schreibfehlers bleibt die ganze Datei** und ist nach C1.6 zulässig. Der
Modulkopf von `leseprofil::datei` führt statt zwei Reichweiten **drei** und nennt diese
ausdrücklich als die weiteste, samt der Feststellung, dass ein Buchstabendreher in einem
Bausteintisch in sie fällt und nicht in die kleinste. Der Satz „fällt damit auf", der den milden
Fall nahelegte, steht nicht mehr da.

### Der Satz der abgeschnittenen Zählung

`Wert::als_text` schreibt für `UeberGrenze` jetzt
`mindestens {Treffer} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)`. Beide
Auskünfte stehen darin, und die Grenze kommt aus der Konstanten.

**„mindestens" statt „über", und das ist nicht nur Wortwahl.** „über 1" behauptet echt mehr als
einen; getroffen hat genau einer innerhalb der gelesenen Einträge, und ob hinter dem Abbruch ein
zweiter steht, ist unentschieden. Die gebaute Fassung sagte damit eine Aussage, die falsch sein
kann — das ging über den Befund hinaus, der die Zahl im Wert für die bessere Wahl hielt und
recht behält, nur nicht mit diesem Verb.

Die Probe zu `als_text` prüft den Satz jetzt an `UeberGrenze(1)` und nicht mehr an einem Wert,
der der Grenze gleicht: bei 2.000 erriete der Nutzer den Abbruch noch, und die Probe belegte den
Punkt des Befundes gerade nicht.

### Die Probe zur Teillesung

Das Vorhandensein sucht `muster = '\.md$'` statt `der-eine-treffer`, und die Zusicherung ist ein
`assert_eq!` auf `Vorhanden(true)` statt eines `matches!` über beide möglichen Werte. Bei 2.001
Dateien enthält jede Auswahl von 2.000 mindestens 1.999 Treffer; der Ausgang hängt an keiner
Lesereihenfolge mehr. `der-eine-treffer.txt` ist entfallen: die Datei diente allein dieser
Zusicherung, und ein Prüfstück ohne Aussage lädt dazu ein, eine zweite darauf zu schreiben.

### Die stille Kürzung im `zip`

Der vom Datensatz vorgeschlagene Weg — `readers.toml` in `ersetzungen_der_toml_dateien`
nachtragen — ist heute nicht gangbar: `ablage::leseprofile::laden` entsteht erst mit Schritt 8.
Gebaut ist stattdessen die benannte Ausnahme:

```rust
const OHNE_LADEWEG: [Datei; 1] = [Datei::Leser];
fn toml_dateien_mit_ladeweg() -> impl Iterator<Item = Datei>
```

Die Probe beschädigt weiter **jede** TOML-Datei und hält vor dem `zip` beide Seitenlängen
gegeneinander. Grün heute (4 gegen 4); rot, sobald eine sechste TOML-Datei hinzukommt, die
niemand einordnet, und rot, sobald jemand den Ladeweg baut, ohne den Eintrag herauszunehmen.
Nachgestellt und gemessen: mit leerer Ausnahmeliste meldet sie `left: 5, right: 4` samt dem
Satz, welche Seite zu berichtigen ist.

**Damit fällt die zweite Hälfte der `Also seen`-Zeile vom 260824-1014 weg**: die Paarung hängt
nicht mehr daran, dass `Datei::Leser` in `Datei::ALLE` als letzte TOML-Datei steht.
`OHNE_LADEWEG` filtert nach Wert und nicht nach Stelle.

### Die Prosa der Ablage

`mod.rs:1` steht auf sieben, `mod.rs:4` auf fünf, die zwölf verbleibenden „vier"-Stellen sind je
einzeln gelesen und gesetzt. Nicht mechanisch: die Überschrift bei `:59` nennt **zwei** von fünf,
weil `settings.toml` und `readers.toml` beide von Hand gepflegt werden; `:117` behält seine
zweite Vier, die die Regeln zählt und nicht die Dateien; `:143` schreibt dazu, dass
`readers.toml` `deny_unknown_fields` ebenfalls trägt, aber noch nicht über diesen Ladeweg geht.

`mod.rs:241` ist **neu begründet und nicht umgezählt**: Träger der Aussage „nur eine Zetteldatei
kann `ZuGross` tragen" ist `Zugang::text_laden` und nicht die Herkunft der Datei. Der alte
Wortlaut steht im Text als das, was er war, damit der nächste Leser die Umkehrung sieht.

**Die zwei Probennamen tragen jetzt gar keine Zahl mehr**, statt auf den Ladeweg zu warten:
`alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` und
`jede_toml_datei_mit_ladeweg_wird_bei_beschaedigung_zur_seite_gelegt`. Eine Zahl im Bezeichner
geht mit jeder neuen Ablagedatei wieder schief, und beide laufen ohnehin über `toml_dateien()`.

### Die zwei Doc-Kommentare in `text/datei.rs`

Der Modulkopf nennt allein das Verhältnis und keine Zahl. Der Doc-Kommentar von `anlesen` trägt
dasselbe Verhältnis und den Beleg daneben, einmal, mit Datum und Herkunft, samt dem Grund, warum
die Zahl kein Präsens verträgt. Die Form ist die der Kostenangaben in der Wurzel-`Cargo.toml`.

## Was offen bleibt, und warum

**Punkt 2 von `260824-1215`.** C6.5 und Planschritt 6 nennen weiter „über 2.000" als den
anzuzeigenden Satz. Spec und Plan werden in dieser Sitzung von einem zweiten Agenten geführt,
und diese Räumung fasst beide Dateien ausdrücklich nicht an. Der Zuschnitt ist derselbe wie bei
`260824-1124_*_c4-3-…`: der Bau ist entschieden, die Buchführung steht aus.

**Punkt 2 von `260824-1217`.** Die Kommentarzeilen von `resources/default-readers.toml` gehören
zu Schritt 7 und dem `ontocoder`, und die Datei steht in diesem Baum noch nicht. Neu gefiled:
`issues/260824-1242_o_die-kommentarzeilen-der-auslieferungsfassung-sagen-nicht-dass-ein-schreibfehler-die-ganze-datei-kostet.md`.

**Der Befund von `shared/issues/260821-1023`** bleibt offen und gehört ihm. Seine
Schutzanweisung für drei Stellen ist zurückgenommen — `## Nachtrag 260824-1245` in jenem
Datensatz sagt, warum sie seit Schritt 2 der Runde 16 die falsche Anweisung ist, und führt die
fünf offenen Stellen mit ihren neuen Zeilennummern und der berichtigten Zahl: **sieben**, nicht
sechs, wie der Datensatz noch zählt.

## Berührte Dateien

```
crates/krk-core/src/ablage/mod.rs
crates/krk-core/src/leseprofil/bausteine.rs
crates/krk-core/src/leseprofil/datei.rs
crates/krk-core/src/leseprofil/mod.rs
crates/krk-core/src/text/datei.rs
crates/krk-core/tests/ablage.rs
crates/krk-core/tests/leseprofil.rs
```

Kein Kommando über den ganzen Baum, kein `git add`, kein `git commit`.

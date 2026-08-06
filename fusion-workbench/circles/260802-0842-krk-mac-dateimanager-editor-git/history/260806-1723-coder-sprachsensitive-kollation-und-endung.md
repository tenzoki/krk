# Sprachsensitive Kollation und die Endung als Sortierschlüssel — 260806-1723

**Absender:** coder
**Domäne:** code
**Auftrag:** Nutzerentscheid 260806 zu `decisions/260802-1810_a_sortierung-ohne-sprachsensitive-kollation.md`, Möglichkeit 1 (beides richtigstellen)
**Defekt:** `issues/260806-1647_o_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`
**Codestand vor der Umsetzung:** `a901596`
**Status:** Complete

---

## Ergebnis in einem Satz

Die Namenssortierung ordnet sprachsensitiv, die Sortierung nach Typ ordnet nach der Dateiendung, `make check` ist grün, und die beiden Zusagen aus der Auflage — L3 und L10 — sind nachgemessen und gehalten, mit dem Faktor 9,6 beziehungsweise 8,6 an verbleibendem Spielraum.

---

## Der gewählte Weg für die Kollation

**`icu_collator` als Abhängigkeit in `krk-core`, nicht `localizedStandardCompare:` über `objc2-foundation`.**

Ausschlaggebend war nicht die Modulgrenze, sondern eine Eigenschaft, die nur eine der drei geprüften Möglichkeiten hat: `icu_collator` kann den sprachsensitiven Vergleich als **Bytefolge** ausschreiben (`Collator::write_sort_key_to`). Zwei so gebaute Schlüssel bytweise verglichen ergeben dieselbe Reihenfolge wie der sprachsensitive Vergleich der Namen.

Das hält den Zuschnitt, den `verzeichnis::eintrag` seit Schritt 2 trägt und den sein Modulkommentar begründet: der Schlüssel entsteht einmal beim Lesen, das Sortieren vergleicht nur noch Bytes. Beide Alternativen können das nicht und kennen allein den paarweisen Vergleich:

| Weg | Schlüsselbildung | Was das kostet |
|---|---|---|
| `icu_collator` | ja, `write_sort_key_to` | 100.000 Schlüssel je Sortierlauf |
| `objc2-foundation`, `localizedStandardCompare:` | nein | rund 1,7 Millionen Fremdaufrufe je Sortierlauf über 100.000 Einträge |
| `feruca` (UCA in reinem Rust) | nein — das Modul `sort_key` ist privat, nach außen steht nur `Collator::collate` | dasselbe wie oben, ohne Fremdaufruf |

Nachgesehen wurde das nicht angenommen: `feruca 0.12.0` führt in `src/lib.rs` unter seinen `pub use`-Zeilen allein `Collator`, `Locale` und `Tailoring`; `mod sort_key` steht ohne `pub`. `icu_collator 2.2.1` exportiert `CollationKeySink` in `src/lib.rs:364` ohne Merkmalsschranke, und `Vec<u8>` setzt die Eigenschaft um.

Der Fremdaufrufweg hatte zusätzlich die Modulgrenze gegen sich, und das gleich zweifach: `krk-core` ist laut `lib.rs` die Schicht ohne Fensterwerkzeug, und sein einziges Modul mit `#[allow(unsafe_code)]` ist `verzeichnis::sys`. Ein zweites danebenzustellen hätte eine Grenze aufgeweicht, die der Bau bisher erzwingt.

### Welche Ordnung

Die **Wurzelordnung von CLDR**, ohne Anpassung an eine einzelne Sprache. Für Deutsch ist das die erwartete Ordnung: die CLDR-Anpassung `de` ändert an der Wurzel nichts, sie unterscheidet sich erst in der Sonderform `de-u-co-phonebk` (Telefonbuch, `ae` für `ä`). Nachgeprüft am laufenden Code: `Apfel` < `Äpfel` < `Bäume` < `Zebra`.

Damit bekommt jeder Nutzer **eine** Ordnung, unabhängig von seinen Systemeinstellungen. Das ist eine Festlegung und keine Auslassung, und sie steht als solche im Modulkommentar von `verzeichnis::kollation`: in Schwedisch steht `ä` hinter `z`, und wer der Systemsprache folgen wollte, müsste die Ordnung von außen hereinreichen — die Systemsprache liegt in Foundation und damit in `krk-ui`. Das wäre ein eigener Entscheid; der Datensatz verlangt ihn nicht.

### Die Maxime "supersimpel"

Es gibt genau einen Sortierweg. Der Schlüssel wird nicht wahlweise billig oder sprachsensitiv gebaut, und `Sortierung::vergleiche` vergleicht in allen vier Fällen nur vorberechnete Werte.

---

## Die Endung

`Eintrag` trägt zwei neue Angaben:

- `endungsschluessel: Box<[u8]>` — derselbe Kollationsschlüssel, gebildet über die Endung. Er ist der Schlüssel der Sortierung nach Typ.
- `endung_ab: u32` — wo im Namen die Endung beginnt. Der Versatz statt einer zweiten Zeichenkette: die Endung steht bereits im Namen, und eine Kopie daneben wäre eine zweite Wahrheit. `Eintrag::endung()` liefert sie als `&str`.

Was als Endung zählt, folgt dem Finder: der **letzte** Punkt trennt (`sicherung.tar.gz` → `gz`), ein **führender** Punkt trennt nicht (`.gitignore` hat keine Endung), und ohne Punkt gibt es keine.

`Sortierung::vergleiche` vergleicht für `Schluessel::Typ` jetzt `endungsschluessel` statt `typ`. Ordner stehen weiterhin in jeder Sortierung vorn, über dieselbe Funktion `gruppe` wie bisher. Einträge ohne Endung stehen am Anfang ihrer Gruppe, weil der leere Schlüssel vor jedem anderen steht.

Zwei Nebenwirkungen, beide gewollt:

- `Typ` verliert seine Ableitungen `PartialOrd` und `Ord`. Sie hatten genau einen Nutzer, die alte Typsortierung. Eine Ordnung zu behaupten, die niemand mehr braucht, wäre eine Falle für den nächsten Leser.
- `Eintrag` lässt sich nicht mehr Feld für Feld zusammensetzen. Ein Eintrag, dessen Schlüssel nicht zu seinem Namen passt, sortiert falsch, ohne dass es auffiele; deshalb geht der Weg über `Eintrag::neu` beziehungsweise `Eintrag::mit_versteckt`. Die beiden Prüfhilfen in `verzeichnis::modell` und `krk-ui::kommandos::operationen`, die die Felder bisher von Hand füllten, sind darauf umgestellt und bauen die Schlüssel damit so, wie das Lesen es tut.

Die Variante heißt weiterhin `Schluessel::Typ` und nicht `Endung`. Ihr Name steht in `session.toml`; eine Umbenennung ließe jede bereits geschriebene Sitzung auf die Vorbelegung zurückfallen.

---

## Was die Änderung kostet

Alle Zahlen sind gemessen, nicht geschätzt. Der Bericht dazu ist `messungen/260806-1716-MacBookPro15-1-kollation-l3-l10.txt`.

### Die beiden Zusagen aus der Auflage

Gemessen auf der kopflosen Strecke, 20 Wiederholungen je Runde, 5 Runden, auf denselben Prüfordnern wie die Abnahmereihe. Angegeben ist das 95. Perzentil der schlechtesten Runde.

| Zusage | Abnahme 260805 | Basis 260806 | Neu 260806 | Zusage | Urteil |
|---|---|---|---|---|---|
| L3 — 10.000 gelesen und sortiert | 22,044 ms | 25,489 ms | **41,546 ms** | ≤ 400 ms | gehalten, alle 5 Runden |
| L10 — 100.000 gelesen und sortiert | 224,176 ms | 301,765 ms | **463,839 ms** | ≤ 4000 ms | gehalten, alle 5 Runden |

Die Spalte "Basis 260806" ist derselbe Messlauf am selben Tag mit dem Code vor der Umstellung, gebaut aus einer Kopie des Quellbaums am Stand `a901596`. Sie steht dort, weil ein Vergleich allein gegen die Abnahmereihe vom Vortag offenließe, was auf die Umstellung und was auf den Tageszustand des Geräts fällt: die Basis liegt ohne eine Zeile Codeänderung bereits 16 % (L3) und 35 % (L10) über der Abnahmereihe.

Der Preis der Umstellung, abgelesen an der Basis desselben Tages: **+63 % bei L3, +54 % bei L10**, das sind rund 1,6 Mikrosekunden je Eintrag für zwei Kollationsschlüssel. Der Aufwand fällt beim Lesen an, einmal je Eintrag. Ein Umsortieren im laufenden Betrieb kostet unverändert nichts dazu, weil das Sortieren weiterhin nur Bytes vergleicht.

Mitbetroffen ist die erste Bildschirmseite (Kernanteil von L2 und L10), weil der erste Stapel bereits fertige Einträge trägt: von 5,8–7,9 ms auf 10,5–13,3 ms, bei einer Zusage von 100 ms. Ebenfalls gehalten.

### Die Größe von `Eintrag`

**72 → 88 Bytes (+22 %).** Beide Zahlen sind gemessen, die alte am Stand `a901596`. Es sind die 16 Bytes des zweiten `Box<[u8]>`; `endung_ab` kostet kein Byte, weil der `u32` in der Lücke liegt, die die Ausrichtung der Struktur ohnehin lässt. Zwei Prüfungen in `verzeichnis::eintrag` nageln beides fest, damit ein weiteres Feld eine sichtbare Entscheidung bleibt und keine Nebenwirkung.

Auf der Halde wird es dabei leichter, ausgezählt über die 100.000 Namen des Prüfordners:

| | Bytes je Eintrag |
|---|---|
| Kollationsschlüssel des Namens | 36,2 |
| Kollationsschlüssel der Endung | 8,2 |
| zusammen | 44,4 |
| bisheriger Sortierschlüssel | 56,9 |

Der bisherige Schlüssel trug den Namen zweimal, einmal kleingeschrieben und einmal unverändert. Unter dem Strich: 16 Bytes mehr in der Struktur, 12,5 Bytes weniger auf der Halde, und eine Zuteilung mehr je Eintrag — die bei einem Namen ohne Endung entfällt.

### Das Programm

**2,15 MB → 3,41 MB (+1,26 MB).** Das sind die CLDR-Tabellen, die `icu_collator` über sein Merkmal `compiled_data` einbackt. C8 stellt keine Zusage über die Größe des Programms; die Zahl steht hier, weil sie der zweite messbare Preis der Umstellung ist.

Der Abhängigkeitsbaum wächst um rund 30 Kisten, alle aus dem ICU4X-Baum und ohne weitere Fremdbindung. `krk-core` behält `#![deny(unsafe_code)]`, und `verzeichnis::sys` bleibt das einzige Modul mit der Ausnahme.

---

## Geänderte Dateien

| Datei | Was |
|---|---|
| `Cargo.toml` | `icu_collator = "2.2"` mit der Begründung und den beiden verworfenen Wegen |
| `crates/krk-core/Cargo.toml` | die Abhängigkeit, und warum sie im Kern steht |
| `crates/krk-core/src/verzeichnis/kollation.rs` | **neu** — der Kollator und `schluessel()`, sechs Prüfungen |
| `crates/krk-core/src/verzeichnis/eintrag.rs` | die Felder `endungsschluessel` und `endung_ab`, dazu `endung()`, `neu()` und `mit_versteckt()`; `Typ` ohne `Ord`; elf Prüfungen |
| `crates/krk-core/src/verzeichnis/sortierung.rs` | `Schluessel::Typ` ordnet nach `endungsschluessel`; sechs Prüfungen |
| `crates/krk-core/src/verzeichnis/mod.rs` | das siebte Modul im Aufbaubild |
| `crates/krk-core/src/verzeichnis/modell.rs` | Prüfhilfe auf `Eintrag::neu` |
| `crates/krk-core/tests/verzeichnis.rs` | zwei Prüfungen durch den echten Leser: Umlaute und die Endungssortierung |
| `crates/krk-ui/src/kommandos/operationen.rs` | Prüfhilfe auf `Eintrag::mit_versteckt` |
| `messungen/260806-1716-MacBookPro15-1-kollation-l3-l10.txt` | **neu** — der Messbericht |

`make check` grün: 497 Prüfungen, Bau, Clippy und Formatprüfung ohne Beanstandung.

Die Prüfungen decken die drei verlangten Punkte ab: `Äpfel` vor `Bäume` (in `kollation`, in `sortierung` und durch den echten Leser hindurch), die Typsortierung nach Endung, und der Gleichstand — gleiche Endung, gleiche Größe und gleiches Datum fallen auf den Namen zurück, nicht auf die Lesereihenfolge, damit zwei Läufe dieselbe Reihenfolge ergeben.

Zwei Prüfungen verdienen einen eigenen Satz. Die eine liest die Umlautnamen über `getattrlistbulk` aus dem Dateisystem und nicht aus dem Quelltext: APFS gibt `Ä` als `A` mit Kombinationszeichen zurück, und der Kollationsschlüssel ordnet beide Schreibweisen an dieselbe Stelle. Die andere hält die Größe von `Eintrag` fest.

---

## Was offen bleibt

**Ein neuer Defekt, gemeldet statt behoben:** `issues/260806-1723_o_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md`. Die Spalte "Typ" zeigt weiterhin "Ordner", "Datei" oder "Verknüpfung", ordnet auf einen Klick aber nach der Endung. Der Nutzer klickt damit auf eine Spalte mit drei Werten und bekommt eine Ordnung nach einem vierten, den die Spalte nicht zeigt. Was eine Spalte anzeigt, ist eine sichtbare Eigenschaft der Anwendung, und der Entscheid vom 260806 trifft sie nicht; sie im Vorbeigehen umzustellen hieße, eine zweite Nutzerentscheidung ungefragt zu treffen.

**Nicht mitgenommen: die numerische Ordnung.** `localizedStandardCompare:` stellt im Finder `Datei 2` vor `Datei 10`; die Wurzelordnung von CLDR tut das ohne die Vorgabe `CollationNumericOrdering::On` nicht, und KRK setzt sie nicht. Der Datensatz nennt als Bezugspunkt allein, dass der Finder `Äpfel` vor `Bäume` sortiert. Die Ziffernfolge wäre eine zweite sichtbare Verhaltensänderung ohne Entscheid.

**Nicht angefasst, weil außerhalb des Auftrags:** der Entscheidungsdatensatz selbst und der Defekt, der ihn meldet. Der Marker `_a_` → `_i_` und die Zeile `Implemented:` gehören an einen Commit-Hash, und dieser Auftrag committet nicht. Ebenso stehen der Nachzug an Plan und Spec und die Richtigstellung von `CLAUDE.md:79` ("bindet Schritt S12" im Präsens, obwohl S12 abgenommen ist) weiterhin beim `planner`, wie der Defekt es zuweist.

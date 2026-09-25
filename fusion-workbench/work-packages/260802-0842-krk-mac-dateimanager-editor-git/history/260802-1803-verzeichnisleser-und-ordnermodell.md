# Verzeichnisleser und Ordnermodell (Schritt 2)

**Datum:** 260802-1803
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `## Implementierungsschritte`, Schritt 2
**Geänderte Dateien:** `crates/krk-core/src/verzeichnis/{mod.rs,sys.rs,leser.rs,eintrag.rs,modell.rs,sortierung.rs}` (neu), `crates/krk-core/tests/verzeichnis.rs` (neu), `crates/krk-core/src/lib.rs` (eine Zeile: `pub mod verzeichnis;`)

## Was gebaut wurde

Sechs Module, in der Reihenfolge, in der die Daten sie durchlaufen:

```
sys  ──> leser ──> eintrag ──> modell <── sortierung
```

**`sys.rs`** bindet `getattrlistbulk(2)` und ist die einzige Stelle im Kern mit einem Fremdaufruf. Sie trägt `#![allow(unsafe_code)]` als Modulattribut. Der Zerleger des Antwortpuffers ist vollständig sicherer Code: er liest jedes Feld über `slice::get` und `from_ne_bytes`, nicht über Zeiger. `unsafe` steht damit an genau zwei Zeilen, der `extern`-Deklaration und dem einen Aufruf.

**`leser.rs`** macht daraus den gestückelten Lesevorgang. Ein Arbeitsfaden liest, schneidet Stapel zu 1.024 Einträgen zu und schickt sie über einen `std::sync::mpsc`-Kanal an den Hauptfaden. Jede Meldung trägt ihre Generationsnummer. Der Abbruch läuft über ein `AtomicBool`. Der Abschluss kommt als eigene Meldung mit `Vollstaendig`, `Abgebrochen` oder `Fehler`.

**`eintrag.rs`** hält den Eintrag samt Sortierschlüssel, der beim Lesen einmal berechnet wird. **`modell.rs`** hält `Vec<Eintrag>` und `Vec<u32>` getrennt; Umsortieren ordnet nur die Indexliste. **`sortierung.rs`** liefert die acht Ordnungen, Ordner immer vor Dateien.

## Zwei Entwurfsentscheidungen, die der Plan offenließ

**Der Kanal hat die Tiefe 1.** Der Plan nennt `std::sync::mpsc`, aber nicht die Kapazität. Ein unbegrenzter Kanal hielte bei einem Ordner mit 100.000 Einträgen den ganzen Bestand ein zweites Mal im Speicher, und ein Abbruch träfe einen Arbeitsfaden, der längst durchgelaufen ist. Mit der Tiefe 1 läuft der Leser höchstens zwei Stapel vor. Genau das macht das Abnahmekriterium "mitten im Lauf abgebrochen" überhaupt prüfbar; mit einem unbegrenzten Kanal ist es bei 5.000 Einträgen nicht auslösbar.

**Der Abbruch wird auch zwischen zwei Stapeln geprüft, nicht nur zwischen zwei Systemaufrufen.** Ein einziger `getattrlistbulk`-Aufruf mit 256 KiB Puffer liefert je nach Namenslänge mehrere tausend Einträge. Die erste Fassung prüfte nur am Schleifenkopf und ließ einen abgebrochenen Ordner mit 5.000 Einträgen trotzdem vollständig durchlaufen; der Test hat das gefangen.

## Ein Fehler, den nur die Messung gefunden hat

Die erste Fassung setzte `ATTR_CMN_FLAGS = 0x00000040`. Der richtige Wert ist `0x00040000`; `0x40` ist `ATTR_CMN_OBJPERMANENTID`, ein Feld von 8 Byte. Der Kern lieferte das Feld also mit, und alles dahinter lag um 8 Byte verschoben: das Änderungsdatum war Unsinn, die Größe war der um 32 Bit verschobene richtige Wert, und das Versteckt-Kennzeichen las Reste der Nanosekunden.

Ein solcher Fehler bricht nichts, er liefert nur falsche Zahlen. Er wurde über einen Hexdump des Antwortpuffers gefunden und nicht über Nachlesen. Deshalb prüft `tests/verzeichnis.rs` jetzt jeden gelesenen Wert gegen `std::fs::symlink_metadata`: Name, Typ, Größe und Änderungsdatum, für Datei, Ordner, Verknüpfung und versteckte Datei.

Am Gerät nachgemessen und nicht nur angenommen sind außerdem:

- Der Satzaufbau: Länge, `ATTR_CMN_RETURNED_ATTRS`, dann die übrigen Attribute in aufsteigender Reihenfolge ihrer Bitwerte, dicht gepackt.
- Ein Ordner bekommt keine Dateiattribute; sein Satz ist kürzer. Der Zerleger richtet sich deshalb nach dem zurückgemeldeten Bitmuster.
- `UF_HIDDEN` (`chflags hidden`) kommt als `0x00008000` an der erwarteten Stelle an.
- `getattrlistbulk` meldet `.` und `..` nicht.
- Eine Verknüpfung kommt als `VLNK` mit der Länge ihres Ziels, dem Wert aus `symlink_metadata`.

## Abnahme

Alle Kommandos am 260802-1803 auf dem Referenzgerät ausgeführt.

| Kommando | Rückgabewert |
|---|---|
| `cargo test -p krk-core` | 0 (7 Modultests, 14 Abnahmetests) |
| `cargo test --workspace` | 0 |
| `cargo build --workspace` | 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |

Die fünf Punkte des Abnahmekriteriums:

| Punkt | Test | Ergebnis |
|---|---|---|
| 5.000 Einträge in mindestens 5 Stapeln | `fuenftausend_eintraege_kommen_in_mindestens_fuenf_stapeln` | 5.000 Einträge, 5 Stapel (4 × 1.024 + 904) |
| alle acht Sortierungen | `alle_acht_sortierungen_liefern_die_erwartete_reihenfolge` | 8 von 8, Reihenfolge je Fall ausgeschrieben |
| Ordner vor Dateien | `ordner_stehen_vor_dateien_in_jeder_sortierung` | gilt in allen acht |
| Filter für führenden Punkt | `der_filter_blendet_namen_mit_fuehrendem_punkt_aus` | blendet aus, wirft nichts weg |
| Abbruch mitten im Lauf | `abgebrochener_leser_liefert_teilbestand_und_meldet_den_abbruch` | Teilbestand, `Abschluss::Abgebrochen` |

Der Abbruchtest ist der einzige mit Zeitbezug. Er lief fünfzehnmal hintereinander grün.

Dass `#![deny(unsafe_code)]` außerhalb von `sys.rs` wirklich greift, ist nicht angenommen, sondern geprüft: ein probeweise in `modell.rs` eingesetzter `unsafe`-Block bricht den Bau mit `error: usage of an unsafe block`, mit Verweis auf `lib.rs:1`. Die Probe ist wieder entfernt, der Arbeitsbaum trägt keinen Rest.

## Ein Widerspruch im Abnahmekriterium, nicht aufgelöst

Das Kriterium verlangt wörtlich: `grep -rln 'unsafe' crates/krk-core/src` nennt genau eine Datei, `verzeichnis/sys.rs`.

Das ist so nicht erfüllbar, und der Grund liegt in Schritt 1. Dessen Abnahmekriterium verlangt `#![deny(unsafe_code)]` in `crates/krk-core/src/lib.rs`. Diese Zeile enthält die Zeichenkette `unsafe`, also nennt der `grep` immer mindestens zwei Dateien. Derselbe Wortlaut steht auch im Abnahmekriterium von Schritt 15.

Der gemeinte Sachverhalt hält. Es wurde nichts geändert, um den `grep` zu bedienen:

```
$ grep -rlnE 'unsafe (\{|extern|fn|impl|trait)' crates/krk-core/src
crates/krk-core/src/verzeichnis/sys.rs

$ grep -rn 'allow(unsafe_code)' crates/krk-core/src
crates/krk-core/src/verzeichnis/sys.rs:39:#![allow(unsafe_code)]
crates/krk-core/src/lib.rs:11://! `#[allow(unsafe_code)]`. `forbid` liesse sich ...   (Fließtext)
```

Vorschlag an den `planner`: das Kriterium in Schritt 2 und Schritt 15 auf das schärfere Muster ziehen. Entschieden ist nichts, der Plan bleibt unangetastet.

## Zwei Festlegungen, die der Plan nicht trifft

**Was "Sortierung nach Typ" heißt.** Der `Eintrag` aus `## Datenstrukturen` trägt `typ: Typ` mit den drei Werten Ordner, Datei und Verknüpfung, aber kein Feld für die Dateiendung. Die Sortierung nach Typ ordnet deshalb nach dieser Aufzählung und bei Gleichstand nach dem Namen. Da Ordner ohnehin vorne stehen, unterscheidet sie innerhalb der Dateien nur noch Datei von Verknüpfung. Ein Dateimanager sortiert an dieser Stelle üblicherweise nach der Endung. Das wäre ein zusätzliches Feld im `Eintrag` und damit eine Änderung an `## Datenstrukturen`; entschieden ist es nicht.

**Wie weit der Sortierschlüssel für Namen trägt.** Er ordnet ohne Rücksicht auf Groß- und Kleinschreibung und macht die Ordnung total. Eine sprachsensitive Kollation leistet er nicht: `Äpfel` landet hinter `Zebra`, weil der Codepunkt von `ä` über dem von `z` liegt. Für eine deutschsprachige Anwendung mit Umlauten in Dateinamen ist das sichtbar. Der Plan verlangt nur, dass der Schlüssel einmal je Eintrag berechnet wird, und begründet das mit den Kosten eines sprachsensitiven Vergleichs bei 100.000 Einträgen; welche Ordnung er herstellt, sagt er nicht. Kollationstabellen einzuführen wäre ein Vorgriff.

## Nicht gemacht

Kein Prüfordner-Erzeuger für 10.000 oder 100.000 Einträge, keine Messstrecke, kein Bericht unter `messungen/` — das ist Schritt 3. Die 5.000 Einträge der Tests entstehen im Test selbst.

`crates/krk-ui/`, `crates/krk-bench/`, `xtask/`, `spikes/` und `resources/` sind unberührt. Keine neue Abhängigkeit, `crates/krk-core/Cargo.toml` unverändert. Am Plandokument nichts geändert, auch der Schrittstatus nicht — dort arbeitet der `planner`. Kein Commit, keine Aufwandsschätzung.

# Coder: die sechs Proben zu `lesen_hoechstens` und `anlesen`

**Datum:** 2026-08-24 09:55
**Status:** Complete
**Agent:** coder
**Baumstand:** `ed893a4`

## Auftrag

Schritt 4 des Plans
`planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`,
Bündel B, in seiner zweiten Hälfte: die sechs Proben zu den zwei gedeckelten Lesewegen.
Die Quellhälfte des Schrittes — `lesen_hoechstens` samt `Lesestand` in
`verzeichnis/leser.rs`, `anlesen` in `text/datei.rs` — stand bei Beginn schon im
Arbeitsbaum, ungebunden und ungeprüft; der Baumstand oben ist der letzte Commit davor.

Arbeitsbereich nach der Antwort des Nutzers vom 260824-0952:
`crates/krk-core/src/verzeichnis/leser.rs`, `crates/krk-core/src/text/datei.rs`,
`crates/krk-core/tests/verzeichnis.rs`, `crates/krk-core/tests/text.rs`. Der Prüfordner
kommt aus `crates/krk-core/tests/gemeinsam/mod.rs`, der Fassung dieser Kiste; eine vierte
ist nicht entstanden.

## Warum die Proben hierher gehören und nicht zu Schritt 12

Der Nutzer hat am 260824-0952 Möglichkeit 1 gewählt und die Begründung mitgegeben:
`lesen_hoechstens` und `anlesen` sind die Grundlage, auf der Schritt 6 die vier Bausteine
baut. Ungeprüft in Schritt 6 zu gehen und den Beleg erst mit Schritt 12 zu holen, hieße
acht Schritte lang auf einer Behauptung zu bauen; zwei Dateien im bestehenden Testziel
derselben Kiste sind der kleinere Preis. Den dritten erwogenen Weg — Proben in `src/` —
hat er mit derselben Begründung verworfen, die die Zählprobe
`genau_drei_pruefordner_fassungen_stehen_im_baum` (`crates/krk-core/tests/baum.rs:114`)
trägt: sie steht genau dafür da, eine vierte Prüfordner-Fassung rot werden zu lassen.

## Was entstanden ist

**Drei Proben in `crates/krk-core/tests/verzeichnis.rs`**, in einem eigenen Abschnitt
„Der gedeckelte Leser" hinter dem Abschnitt „Der Leser":

- `ein_deckel_unter_dem_bestand_liefert_den_deckel_und_meldet_das_abschneiden` — fünf
  Einträge, Deckel drei, drei Einträge und `abgeschnitten == true`. Welche drei
  zurückkommen, sagt die Lesereihenfolge des Dateisystems; geprüft wird deshalb nur, dass
  jeder gelieferte Name aus dem angelegten Bestand stammt.
- `ein_deckel_genau_auf_dem_bestand_meldet_kein_abschneiden` — fünf Einträge, Deckel fünf,
  fünf Einträge und `abgeschnitten == false`. Das ist die Lage, für die der Leser einen
  `getattrlistbulk(2)` mehr ausgibt, und die Probe hält die stärkere Lesart von
  `Lesestand::abgeschnitten` fest: „es wurde etwas weggelassen" und nicht „die Zahl ist
  erreicht".
- `lesen_liefert_denselben_bestand_wie_der_hoechste_deckel` — `lesen` gegen
  `lesen_hoechstens(usize::MAX)`.

**Drei Proben in `crates/krk-core/tests/text.rs`**, in einem eigenen Abschnitt „Die dritte
Huelle um dieselbe Tuer: anlesen" am Dateiende:

- `anlesen_liefert_den_anfang_wo_die_grenze_abweist` — 100 Bytes, Zahl 10: `anlesen`
  liefert zehn Bytes, `bis_zur_grenze_lesen` liefert für dieselbe Datei und dieselbe Zahl
  `ZuGross`. Eine dritte Zusicherung hält die andere Hälfte, dass eine Datei unter dem
  Deckel ganz kommt und nicht auf den Deckel gekürzt wird.
- `ein_ordner_ist_fuer_das_anlesen_keine_datei` — `KeineDatei`, entschieden am `fstat` des
  offenen Deskriptors.
- `eine_benannte_roehre_ist_keine_datei_und_haelt_das_anlesen_nicht_an` — `KeineDatei`
  innerhalb einer Zeitschranke von fünf Sekunden. Die größere der zwei Aussagen ist, dass
  überhaupt eine Antwort fällt.

## Eine Zusammenlegung, die der Plan nicht verlangt hat

Die Röhrenprobe braucht eine Zeitschranke, und `tests/text.rs` führte davon schon zwei
Fassungen: `oeffnen_mit_zeitschranke` und `bis_zur_grenze_mit_zeitschranke`. Beide
unterschieden sich in nichts als der gerufenen Funktion und dem Meldetext. Eine dritte
Kopie daneben wäre die Bauform gewesen, gegen die dieses Projekt an mehreren Stellen
ausdrücklich schreibt — „es gibt genau eine Hülle um `NSPasteboard`", „es gibt genau drei
Prüfordner-Fassungen, und das soll so bleiben".

Entstanden ist deshalb `mit_zeitschranke<T>(was, schranke, auftrag)` als die eine Fassung;
alle drei Rufer sind jetzt Einzeiler darauf, und `was` steht im Meldetext, damit ein
Fehlschlag sagt, welche der drei Hüllen hängt. **Am Verhalten der zwei bestehenden Proben
ändert das nichts**, an ihrem Meldetext nichts, und der Netto-Zeilenstand der Datei sinkt.
Der Plan hat diese Zusammenlegung nicht verlangt; sie steht hier ausgeschrieben, damit sie
nicht als stille Zutat durchgeht.

Der Modulkopf von `tests/text.rs` zieht mit: er zählte bis dahin die vier Proben auf, die
seit der Runde 11 neben den zwölf Fällen stehen, und nennt jetzt die drei der Runde 16
daneben.

## Was nicht angefasst wurde

`crates/krk-core/src/verzeichnis/leser.rs` und `crates/krk-core/src/text/datei.rs` sind
unverändert geblieben: ihre Hälfte des Schrittes stand schon da, und die Proben haben
nichts gefunden, was daran zu berichtigen wäre. `crates/krk-core/src/verzeichnis/mod.rs`
ist ebenfalls unberührt — seine Wiederausfuhrliste führt `lesen`, aber nicht
`lesen_hoechstens`; die Proben greifen wie die übrigen dieser Datei über den vollen Pfad
`krk_core::verzeichnis::leser::…` zu, und eine Erweiterung der Liste verlangt der Plan
nicht.

## Ein Defekt, gefiltert

Die `Files:`-Zeile von Schritt 4 nannte zwei Quelldateien für einen Schritt, der vier
Dateien braucht. Es ist die zweite Fundstelle desselben Musters in diesem Plan: Schritt 8
nennt `crates/krk-core/tests/ablage.rs` nicht, obwohl der Defekt
`issues/260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`
dort das Nachziehen verlangt. Der Datensatz darüber ist
`issues/260824-0955_o_die-files-zeile-eines-planschritts-nennt-die-quelldateien-und-nicht-die-testdateien.md`;
die offenen Namen im Circle-Speicher und im gemeinsamen Speicher sind vorher durchgesehen
worden, keiner lag zu derselben Sache, also ist ein neuer Datensatz entstanden und keine
`Also seen:`-Zeile. Die `Files:`-Zeile von Schritt 4 selbst ist im Plan um die zwei
Testdateien ergänzt, mit dem Vermerk, woher der Nachtrag kommt.

## Prüfung

`make check` läuft grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check` und `cargo clippy --workspace --all-targets -- -D warnings`.
Exit-Code 0. Die sechs neuen Proben laufen mit; `tests/verzeichnis.rs` steht bei 86
bestandenen Proben, `tests/text.rs` bei 33.

## Was nicht Gegenstand war

Kein Profilmodell (Schritt 3), keine Erkennung (Schritt 5), keine Bausteine (Schritt 6).
Die Zählproben zu C6, die den Haushalt auslesen, gehören Schritt 12 und sind hier nicht
vorweggenommen; die sechs Proben dieses Schrittes zählen keine Aufrufe, sondern prüfen die
zwei Lesewege an ihren Rückgabewerten.

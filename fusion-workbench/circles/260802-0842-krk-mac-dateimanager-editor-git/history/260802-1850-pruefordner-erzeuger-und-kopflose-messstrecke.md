# Prüfordner-Erzeuger und kopflose Messstrecke (Schritt 3)

**Datum:** 260802-1850
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `## Implementierungsschritte`, Schritt 3 (Fassung des Nachzugs vom 260802-1746)
**Geänderte Dateien:** `crates/krk-bench/src/{main.rs,fixture.rs,messen.rs,bericht.rs}`, `messungen/.gitkeep`, `crates/krk-bench/Cargo.toml` (eine Abhängigkeit, siehe unten), `messungen/260802-1646-kopflos-{a,b,gross}-warm.txt` (Messbelege der Abnahme)

## Was gebaut wurde

**`fixture.rs` — der Erzeuger.** `fixture --eintraege N --seed S --out PFAD` legt einen flachen Ordner an. Die Zusammensetzung entsteht in zwei getrennten Schritten: `bauplan(anzahl, startwert)` rechnet die vollständige Liste aus Name, Art, Größe, Änderungsdatum und Verknüpfungsziel aus, ohne das Dateisystem anzufassen, und `erzeugen` legt sie hin. Diese Trennung ist der Grund, aus dem sich die Reproduzierbarkeit prüfen lässt, ohne zweimal 100.000 Dateien anzulegen.

Der Zufallsgenerator ist SplitMix64, selbst geschrieben. Eine Fremdbibliothek darf ihren Zahlenstrom mit einer neuen Hauptversion ändern; die Zusage hier lautet aber, dass Startwert 1 heute und in zwei Jahren dieselbe Liste liefert.

Gemischt sind: drei Eintragsarten (Datei, leerer Unterordner, symbolische Verknüpfung auf einen Eintrag desselben Ordners), vierzehn Endungen, vierundzwanzig Namensstämme verschiedener Länge und fünf Größenklassen von 0 Byte bis 64 MiB. Die Namenslängen streuen mit Absicht: `getattrlistbulk` packt seine Antwortsätze dicht, und die Satzlänge hängt an der Namenslänge.

**`messen.rs` — die Messstrecke.** `messen --kopflos --ordner PFAD [--kalt]` fährt zwanzig Läufe und hält je Lauf zwei Spannen fest: bis der erste Stapel im Ordnermodell steht (Anteil an L2) und bis das Modell abgeschlossen und sortiert ist (L3, L10). Ausgewiesen werden 95. Perzentil, Median und Minimum.

**`bericht.rs` — der Bedingungskopf.** Zeitpunkt, `hw.model`, `sw_vers`, Bildwiederholrate, Cache-Zustand, Wiederholungszahl, Prüfordnerpfad, Startwert, Eintragszahl und Bauart. Dazu die Einzelwerte aller zwanzig Läufe und ein Abschnitt "Lesart", der ausschreibt, was die Strecke **nicht** misst.

## Fünf Festlegungen, die der Plan offenließ

**Der Erzeuger setzt Änderungsdaten, nicht nur Namen und Größen.** Das Abnahmekriterium prüft über `ls -la <ordner> | shasum`, und `ls -la` druckt die Zeiten mit. Ohne feste Änderungsdaten ist dieses Kommando nicht durchführbar — zwei Läufe zu verschiedenen Zeitpunkten ergäben immer verschiedene Prüfsummen. Gesetzt werden deshalb die Daten aller Einträge und das des Ordners selbst, dieses zuletzt, weil jeder angelegte Eintrag es fortschreibt. Die Daten liegen in der Vergangenheit (ab 2020-01-01), weil `ls` für alles, was älter als ein halbes Jahr ist, die Jahreszahl statt der Uhrzeit druckt; damit bleibt die Ausgabe auch in einem Jahr noch dieselbe. Ohne diesen Griff wechselte die Zeilenform irgendwann und die Prüfsumme mit ihr.

Für Verknüpfungen geht das nicht über `File::set_times` — die Methode folgt der Verknüpfung und träfe das Ziel. Den Unterschied kennt auf dieser Ebene nur `lutimes`, und das hieße `libc` samt `unsafe` in einem Werkzeug, das sonst ohne beides auskommt. Stattdessen übernimmt `touch -h -r` die Zeit einer Marke, die vorher gesetzt und danach wieder gelöscht wird. Alle Verknüpfungen eines Prüfordners tragen deshalb dasselbe Datum, die Dateien gestreute.

**Dateien sind dünnbesetzt.** Nur die ersten 512 Byte werden wirklich geschrieben, der Rest entsteht über `set_len` als Loch. Der Leser fragt `ATTR_FILE_DATALENGTH` ab und sieht die volle Größe; auf der Platte liegt ein Bruchteil. Der 100.000er-Ordner nennt 197 GB und belegt 342 MB. Für eine Strecke, die Verzeichnismetadaten liest und keine Dateiinhalte, ändert das am Messwert nichts. **Wer diese Ordner später für die Messung von Kopiervorgängen (L8) benutzen will, muss das wissen: dafür taugen sie nicht.** Der Hinweis steht im Modulkopf von `fixture.rs`.

**`--kalt` ruft `purge` vor jedem einzelnen Lauf, nicht einmal vor der Reihe.** Der Plan sagt nur "ruft `purge`". Ein einziger Aufruf vor zwanzig Läufen machte neunzehn davon warm und schriebe sie trotzdem unter die Überschrift "kalt" — genau der Auswertungsfehler, den die Fn-Messung sichtbar gemacht hat und den der Plan mit seiner Berichtsdisziplin verhindern will. Umgekehrt läuft im warmen Betrieb ein **ungezählter** Vorlauf, weil "warm" laut Spec jeder *weitere* Zugriff ist; ohne ihn trüge der erste von zwanzig Läufen eine kalte Zahl in eine warme Reihe.

Geprüft wird nicht nur der Rückgabewert von `purge`, sondern auch, ob es etwas auf die Fehlerausgabe geschrieben hat. Eine warme Zahl unter der Überschrift "kalt" ist schlimmer als gar keine Zahl, weil sie ein Gate besteht, das sie nicht bestehen dürfte.

**Der Startwert kommt aus einem Steckbrief neben dem Ordner.** Der Berichtskopf muss laut Abnahmekriterium den Startwert nennen, aber `messen` bekommt nur einen Pfad, und ein Pfad trägt keinen Startwert. `fixture` legt deshalb `<name>.pruefordner.toml` **neben** den Ordner — nicht hinein, denn dann wären es 10.001 Einträge statt 10.000. Fehlt der Steckbrief, schreibt der Kopf "unbekannt" aus, statt einen Startwert zu erfinden.

**Die Bauart steht im Kopf.** Der Unterschied zwischen `debug` und `release` macht bei einem Verzeichnisleser leicht den Faktor fünf aus. Ein Bericht aus einem Bau ohne Optimierung kennzeichnet sich selbst als nicht abnahmetauglich.

## Das Perzentil

Nächster Rang, nicht interpoliert: der Wert an Position `ceil(0,95 × 20) = 19` der sortierten Reihe. Höchstens ein Lauf von zwanzig darf darüber liegen. Nicht interpoliert, weil eine Zusage gegen einen wirklich gemessenen Lauf abgenommen werden soll und nicht gegen einen gerechneten Zwischenwert. Der Median mittelt bei gerader Anzahl die beiden mittleren Werte.

Eine Reihe wird verworfen, wenn ein Lauf abbricht oder eine andere Eintragszahl liest als der erste. Ein Lauf, der weniger liest, ist nicht schnell, er misst etwas anderes.

## Prüfen statt annehmen

Der Hinweis aus S2 (`ATTR_CMN_FLAGS` fiel nur auf, weil ein Test gegen `std::fs` prüfte statt auf Plausibilität) ist beim Erzeuger genauso umgesetzt. `der_erzeugte_ordner_traegt_genau_das_was_der_bauplan_sagt` legt 600 Einträge an und prüft **jeden** davon zweifach: gegen `std::fs::symlink_metadata` auf Art, Größe, Verknüpfungsziel und Änderungsdatum, und danach noch einmal durch die Augen des Lesers aus `krk-core`, der später gemessen wird. Dazu die Eintragszahl über `read_dir`, damit ein Ordner mit 599 oder 601 Einträgen auffällt.

36 Proben in `krk-bench`, davon 13 im Erzeuger, 8 in der Messstrecke, 8 im Bericht und 7 an der Befehlszeile. Sie liegen als `#[cfg(test)]`-Module in den vier vom Plan genannten Dateien; eine eigene Testdatei entsteht nicht, weil der Plan keine nennt.

## Eine Auslassung im Plan, gemeldet und mit einer Zeile geschlossen

**Die Dateiliste von S3 nennt `crates/krk-bench/Cargo.toml` nicht**, obwohl der Schritt ohne `krk-core` als Abhängigkeit von `krk-bench` nicht baubar ist — S1 hat `krk-bench` ausdrücklich als leeren Rumpf ohne Abhängigkeiten angelegt. Das ist dieselbe Art Auslassung wie der `[alias]`-Abschnitt bei S1 (`issues/260802-1755_c_...`): kein Widerspruch, sondern eine fehlende Zeile mit genau einer möglichen Auflösung. Eingetragen ist deshalb `krk-core = { path = "../krk-core" }` und sonst nichts. Der Weg über `[workspace.dependencies]` hätte zusätzlich die Wurzel-`Cargo.toml` angefasst und damit zwei ungenannte Dateien statt einer.

## Eine Abweichung zwischen zwei Stellen des Plans

Der Absatz "Jeder Bericht trägt seine Bedingungen" (Zeile 149) verlangt acht Kopfangaben, darunter Bildwiederholrate und den **Pfad** jedes Prüfordners. Die Änderungsliste von S3 nennt sechs, und das Abnahmekriterium spricht von "den sechs genannten Angaben". Das ist kein Widerspruch, sondern eine Teilmenge; der Kopf trägt jetzt alle acht.

Die Bildwiederholrate ist auf dem Referenzgerät allerdings nicht erhebbar: `system_profiler SPDisplaysDataType` führt zum eingebauten Bildschirm des `MacBookPro15,1` keine Zeile `Refresh Rate`. Der Kopf schreibt deshalb aus, dass sie nicht gemeldet wurde, und nennt stattdessen die Auflösung. Eine erfundene 60 käme nicht in Frage, auch wenn C8 den Wert für dieses Gerät nennt. Für S21 heißt das: die Rate muss aus der laufenden Anwendung kommen (`NSScreen.maximumFramesPerSecond` oder `CADisplayLink`), nicht aus `system_profiler`.

## Wo die Prüfordner liegen

`~/Library/Caches/org.stalmann.krk/pruefordner/` — außerhalb des Repositories, auf demselben Datenträger wie das Projekt.

| Ordner | Einträge | Startwert | Belegung |
|---|---|---|---|
| `a` | 10.000 | 1 | 34 MB |
| `b` | 10.000 | 2 | 34 MB |
| `gross` | 100.000 | 3 | 342 MB |

Zusammen 410 MB. Jeder Ordner ist aus seinem Steckbrief heraus in Sekunden wiederherstellbar (2 s für 10.000, 21 s für 100.000); wer Plattenplatz braucht, kann sie folgenlos löschen.

## Abnahme

Alle Kommandos am 260802-1846 auf dem Referenzgerät `MacBookPro15,1` unter macOS 15.7.7 ausgeführt.

| Kriterium aus S3 | Ergebnis |
|---|---|
| `fixture --eintraege 10000 --seed 1` zweimal, verschiedene Ziele, `ls -la \| shasum` | identisch: `5763c0e4…` beide Male |
| dasselbe mit `--eintraege 100000` | identisch: `918315d5…` beide Male |
| `--seed 2` liefert eine andere Liste als `--seed 1` | verschieden (`5763c0e4…` gegen `26145c33…`), **null gemeinsame Namen** von 10.000 |
| `messen --kopflos` schreibt unter `messungen/` | drei Dateien, Kopf mit allen geforderten Angaben, Zahlenteil mit 95. Perzentil über 20 Läufe |
| `messen --kalt` ohne Rechte | Rückgabewert 1, Standardausgabe 0 Byte, keine Berichtsdatei |
| `cargo build --workspace` (Wirtsziel, x86_64, aarch64) | je 0 |
| `cargo test --workspace` | 57 Proben, 0 Fehlschläge |
| `cargo clippy --workspace --all-targets` | 0 Warnungen |
| `cargo fmt --all --check` | 0 |

Die Prüfsummen der beiden Zehntausenderordner sind zwischen zwei Abnahmeläufen verschieden, obwohl die Ordner es nicht sind: die Zeile `..` einer `ls -la`-Ausgabe trägt das Änderungsdatum des **Elternordners**, und das ändert sich, sobald daneben ein weiterer Ordner entsteht. Verglichen werden müssen deshalb zwei Auflistungen, die zum selben Zeitpunkt entstehen — dann stimmen sie Byte für Byte. Ohne die Zeile `..` sind die Prüfsummen auch über die Zeit stabil (`a891d00b…` für beide Ordner aus Startwert 1).

## Was nicht geprüft ist

**Der gelingende kalte Lauf.** `purge` verlangt Rechte, die dieser Lauf nicht hatte; geprüft ist nur der Abbruch. Der Weg mit `sudo` ist ungefahren. Wer ihn fährt, sollte mit Zeit rechnen: zwanzig `purge`-Aufrufe nacheinander.

**Die Messwerte selbst sind keine Abnahme.** S3 nimmt die Strecke ab, nicht die Zusagen; das tut S8. Die drei warmen Läufe liegen als Beleg unter `messungen/` und lassen die Größenordnung erkennen: erster Stapel rund 5 ms, vollständiges Lesen mit Sortierung rund 20 ms bei 10.000 Einträgen und rund 215 ms bei 100.000. Gegen L3 (400 ms warm) und L10 (4 s warm) ist das reichlich Luft, aber es ist der Kernanteil ohne Fenster, ohne AppKit und ohne Zeichendurchgang.

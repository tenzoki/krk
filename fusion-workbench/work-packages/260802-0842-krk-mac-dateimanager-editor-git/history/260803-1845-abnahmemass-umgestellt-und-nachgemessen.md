# Abnahmemaß umgestellt und nachgemessen (Schritt 8, zweiter Teil)

**Datum:** 260803-1845
**Agent:** coder
**Status:** Complete
**Auslöser:** Nutzerentscheidung vom 260803-1810, `decisions/260803-1755_i_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`; Umstellungsauftrag in `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, S8 unter `Was der coder daraufhin umzustellen hat`
**Geändert:** `crates/krk-bench/src/messen.rs`, `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S8 auf `[DONE]`, neuer Standabsatz, Defektliste), `planning/260802-1036_o_spec-navigator-geruest.md` (nur zwei Pfadverweise auf den umbenannten Datensatz), `decisions/260803-1755_a_…` → `_i_` mit `Implemented:`-Zeile und Umsetzungsabschnitt
**Neu angelegt:** `messungen/260803-1641-durchstich.txt`, `issues/260803-1845_o_l4-streut-zwischen-den-runden-viel-staerker-als-die-erste-messung-zeigte.md`, diese Datei
**Nicht angefasst:** `crates/krk-core/`, `crates/krk-ui/`, `xtask/`, `resources/`, `README.md`, `CLAUDE.md`. Kein Commit.
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen, wie für den `coder` vorgesehen. Ein Langform-Schreibprofil gibt `fusion-rules` für diesen Agenten nicht aus.

## Das Urteil zuerst

**Das Gate ist bestanden.** L1 hält das neue Maß nicht knapp, sondern vollständig: **100 von 100 Tastendrücken haben ihr nächstes Bild erreicht, in jeder der fünf Runden 20 von 20.** Erlaubt wäre eine verpasste Eingabe je Runde gewesen.

| Zusage | Maß | Gemessen über fünf Runden | Urteil |
|---|---|---|---|
| L1 Tastendruck bis Ende des Zeichendurchgangs | Anteil im nächsten Bild, mindestens 95 % je Runde | 100,0 % in jeder Runde | gehalten |
| L2 erste Bildschirmseite auf Prüfordner A | 95. Perzentil ≤ 100 ms | 41,176 bis 53,079 ms | gehalten |
| L3 vollständiges Lesen auf A, warm | 95. Perzentil ≤ 400 ms | 120,695 bis 145,780 ms | gehalten |
| L4 Prozessstart bis bedienbares Fenster | 95. Perzentil ≤ 1000 ms | 282,391 bis 715,185 ms | gehalten |
| L10 erste Bildschirmseite bei 100.000 Einträgen | 95. Perzentil ≤ 100 ms | 50,447 bis 54,486 ms | gehalten |

Bericht: `messungen/260803-1641-durchstich.txt`. Als Beigabe gemessen und vom Gate nicht abgefragt: das vollständige Lesen der 100.000 Einträge liegt bei 849 bis 1005 ms gegen zugesagte 4 s.

## Dass die Änderung nicht kosmetisch war, zeigt dieselbe Messung

Das 95. Perzentil von L1 lag in dieser Messung zwischen 14,912 und 16,633 ms. **Unter der alten Zusage von 16 ms hätte L1 in vier von fünf Runden verfehlt**, während kein einziger der hundert Tastendrücke sein Bild verpasst hat. Die beiden Maße fällen an denselben Rohdaten entgegengesetzte Urteile.

Das ist der Beleg für die Begründung, die im Entscheidungsdatensatz steht: das Perzentil einer Spanne, die an einer Bildgrenze endet, misst, an welcher Stelle des Bildes der Tastendruck eintraf, und nicht, wie schnell KRK ist. Der größte Einzelwert der Messung war 16,657 ms, also zehn Mikrosekunden unter einer Bildlänge; der kleinste 1,232 ms. Diese Streuung ist die Phase im Bild und nicht die Arbeit.

## Was ein Befund ist und kein Rundungsfehler

**L4 hat sich verschoben.** Die erste Messung wies über fünf Runden 294,555 bis 303,540 ms aus, eine Streuung von neun Millisekunden. Diese weist 282,391 bis 715,185 ms aus, und der größte Einzelwert liegt bei 916,460 ms gegen zugesagte 1000 ms.

Es ist kein einzelner Ausreißer. In Runde 2 liegen die letzten dreizehn der zwanzig Prozessstarts zwischen 420 und 916 ms, während die Runden 3, 4 und 5 durchgehend zwischen 263 und 349 ms bleiben. Ein einzelner langsamer Start hätte das 95. Perzentil nicht bewegt, weil es der neunzehnte Wert der sortierten Zwanzigerreihe ist.

**Die Codeänderung scheidet als Ursache aus.** Geändert ist allein die Rechnung, die aus vorliegenden Einzelwerten ein Urteil bildet. Weder `krk-ui` noch `krk-core` noch das Bündel sind angefasst; die Rohmessung ist dieselbe wie am 260803-1554. Nachgeprüft am Diff: `crates/krk-bench/src/messen.rs` ist die einzige geänderte Quelldatei.

`inference:` Naheliegend ist Fremdlast auf dem Gerät während der Startphase von Runde 2. Dafür spricht, dass L2 in Runde 2 mit 53,079 ms ebenfalls seinen schlechtesten Wert hatte. Dagegen spricht, dass L3 in derselben Runde mit 120,695 ms seinen besten hatte. L4 wird in einer eigenen Phase vor den übrigen gemessen, sodass ein Lastereignis die eine Phase treffen kann und die andere nicht. **Beweisen lässt sich das aus dem Bericht nicht**, weil er die Systemlast nicht mit erhebt. Gemeldet als `issues/260803-1845_o_l4-streut-zwischen-den-runden-viel-staerker-als-die-erste-messung-zeigte.md`, zu klären vor S22 und nicht jetzt: S22 nimmt L4 kalt und auf der Prüfsitzung ab, und beide Verschärfungen zehren von demselben Abstand.

L2, L3 und L10 haben sich nicht bedeutsam verschoben. L3 ist um rund 15 ms besser geworden, L10 liegt innerhalb von zwei Millisekunden auf dem alten Stand, L2 hat einen einzelnen schlechteren Rundenwert in derselben Runde 2.

## Wie das zweite Maß im Code liegt

Die Struktur `Zusage` trug mit `schwelle: Option<Duration>` genau ein Abnahmemaß, und `None` hieß "nicht abgefragt". Das trägt zwei Maße nicht: ein zweites Feld daneben hätte vier Kombinationen erlaubt, von denen zwei sinnlos sind.

An seine Stelle tritt ein Typ mit drei Fällen:

```
Abnahmemass
   ├─ Perzentil(Duration)              acht Zusagen: p95 der Runde <= Grenze
   ├─ AnteilImBild { bildlaenge }      L1, spaeter L9: >= 95 % der Werte <= Bildlaenge
   └─ Keine                            der Bericht nennt die Zahl, das Gate nicht
```

**Die Bildlänge steht im Fall selbst und nicht beim Aufrufer.** Damit trägt eine Zusage ihr Maß vollständig, und `gehalten_in` bleibt ohne Argument. Die Gegenvariante, die Bildlänge als Parameter durchzureichen, hätte acht von zehn Zusagen ein Argument aufgezwungen, das sie nicht ansehen.

`immer_gehalten` und `Durchstichergebnis::bestanden` sind unverändert geblieben. Beide fragen nur, ob eine Zusage in jeder Runde gehalten hat; **welches Maß dahinter steht, geht sie nichts an.** Das ist der Grund, aus dem die Umstellung eine Struktur berührt und nicht die Urteilskette.

**Der Anteil wird ganzzahlig verglichen**, `erreicht * 100 >= gesamt * 95`. Bei genau 19 von 20 hängt das Urteil sonst daran, ob `19.0/20.0` in der letzten Binärstelle über oder unter dem Literal `0.95` liegt. Prozentzahlen im Bericht sind Fließkomma, die Entscheidung ist es nicht.

## Wie die fehlende Bildwiederholrate strukturell abbricht

Der Plan verlangt: fehlt die Rate, bricht die Auswertung ab, statt 60 Hz zu unterstellen. Umgesetzt ist das nicht als Prüfung an der Auswertungsstelle, sondern am Typ. `Durchstichergebnis::bildwiederholrate` ist keine `Option` mehr, und `bildlaenge_bilden` ist der einzige Weg, an eine Bildlänge zu kommen. Ohne gemeldete Rate, oder bei einer Rate von null oder darunter, entsteht **gar kein Ergebnis**, das ein Urteil tragen könnte.

Damit ist auch der Zweig "nicht erhoben" aus der Kopfbeschreibung verschwunden, den es vorher gab. Ein Bericht mit einer Lücke an dieser Stelle wäre seit dem 260803-1810 kein unvollständiger Bericht mehr, sondern ein falscher: die Rate ist nicht mehr nur eine Angabe im Kopf, sondern die Grenze, gegen die L1 abgenommen wird.

## Wie der Bericht beide Maße nebeneinander lesbar macht

Die Anforderung war, dass kein Leser raten muss, nach welcher Regel eine Zeile beurteilt wurde. Drei Mittel zusammen:

- **Die Spalte `Abnahme nach`** nennt je Zeile die Regel im Klartext: `>= 95 % im Bild` bei L1, `p95 <= 100 ms` bei L2, `keine` bei der Beigabe. Sie ersetzt die alte Spalte `Zusage`, die für L1 `16 ms` nannte.
- **Die Spalte `im Bild`** trägt den Anteil der schlechtesten Runde und steht auf `-`, wo das Maß nicht gilt. Der Anteil bei L2 oder L4 auszuweisen wäre nicht neutral, sondern irreführend: er wäre bei Spannen von 45 bis 300 ms immer null.
- **Ein eigener Abschnitt** führt den Anteil Runde für Runde, mit absoluten Zahlen daneben (`100.0 % (20/20)`), so wie der vorhandene Abschnitt das Perzentil Runde für Runde führt. Darüber steht die Bildlänge, gegen die gezählt wurde.

Dazu ein Absatz über der Tabelle, der die Zweiteilung benennt und ausdrücklich sagt, dass Perzentil, Median, Minimum und Maximum für L1 Kennzahlen ohne eigenes Urteil sind. Dieselbe Aussage steht ein zweites Mal in der Überschriftszeile des Perzentilabschnitts, weil ein Leser, der dort einsteigt, den Absatz über der Tabelle nicht gelesen haben muss.

## Was weiterhin ungemessen bleibt

Die vier Einschränkungen des ersten Berichts stehen im zweiten erneut ausgeschrieben, statt wegzufallen.

**L4 ist warm gemessen, C8 sagt Kaltstart.** `purge` braucht Rechte, die diese Sitzung nicht hat (`sudo -n true` scheitert mit "a password is required"), und ein Passwortdialog lässt sich in einem Messlauf nicht beantworten. Warm ist der leichtere Fall; die Zahl ist eine Untergrenze für die Zusage, die C8 wirklich stellt. Bei der in dieser Messung gefundenen Streuung wiegt die Lücke schwerer als beim ersten Mal.

**Die Prüfsitzung aus C8 ist am Durchstich nicht herstellbar.** C8 schreibt für L4 zwei Dateifenster mit je zwei Tabs vor; Tabs gibt es erst ab S12. Gemessen ist der Start mit einem Fenster ohne wiederhergestellte Sitzung auf Prüfordner A. Die Abnahme gegen die Prüfsitzung leistet S22.

**Der körperliche Tastendruck bleibt ungemessen.** Belegt ist der Weg mit einem synthetischen `NSEvent`. Dass eine körperlich gedrückte Taste dieselben Ereignisse erzeugt, stammt weiterhin aus der Messung vom 260802-1137.

**Eine Bildgrenze ist keine Photonenmessung.** Sie ist der Zeitpunkt, an dem das System sein nächstes Bild vorbereitet, nicht der, an dem ein Pixel leuchtet. Der zweite ist aus dem eigenen Prozess heraus nicht feststellbar. **Beim neuen Maß wiegt das schwerer als beim alten**, und der Bericht schreibt das jetzt aus: die Bildlänge ist hier die Urteilsgrenze selbst, also könnte ein Wert dicht an ihr bei einer echten Bildschirmmessung auf die andere Seite fallen. In dieser Messung liegt das Feld nicht an der Grenze: 89 der 100 Einzelwerte sind kleiner als 15 ms, und kein einziger überschreitet die Bildlänge von 16,667 ms. Nachgezählt an den Einzelwerten des Berichts.

## Eine Kleinigkeit, die offenzulegen ist

Der Umzug des Entscheidungsdatensatzes von `_a_` auf `_i_` macht seinen alten Pfad ungültig. Nachgezogen sind Spec, Plan und die zwei Nennungen in `crates/krk-bench/src/messen.rs`. **Der Bericht `messungen/260803-1641-durchstich.txt` nennt im Abschnitt `Lesart` noch die `_a_`-Schreibweise**, weil er vor der Umbenennung entstanden ist. Er wird nicht nachgezogen: ein Messbericht ist ein Artefakt seines Laufs, und ihn neu zu erzeugen hieße neu zu messen und die Zahlen wegzuwerfen, über die dieses Protokoll berichtet. Die Quelle erzeugt künftige Berichte mit der richtigen Schreibweise.

Die Historiendatei `260803-1819-abnahmemass-fuer-l1-und-l9-geaendert.md` trägt den alten Pfad ebenfalls weiter und bleibt unverändert: sie hält den Stand fest, der zu ihrem Zeitpunkt galt.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo test --workspace` | Rückgabewert 0, 115 Prüfungen, davon 7 neue in `messen.rs` |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `cargo clippy --workspace --all-targets` | Rückgabewert 0, keine Warnung |
| `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/` | genau zwei Dateien, `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs` |
| `grep -cE '^[[:space:]]*use objc2' crates/krk-ui/src/messmodus.rs` | 0 |
| `cargo xtask bundle` | Rückgabewert 0, signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)", ohne Rückfrage |
| `codesign --verify --deep --strict target/KRK.app` | Rückgabewert 0 |
| `krk-bench durchstich … --runden 5` | **Rückgabewert 0**, Bericht geschrieben, alle fünf Zusagen gehalten |

Das Abnahmekriterium von S8 ist damit in beiden Teilen erfüllt. Der Bericht trägt den vollständigen Bedingungskopf mit allen acht Angaben einschließlich der aus `NSScreen` gelesenen Bildwiederholrate von 60 Hz, nennt für L1, L2, L3, L4 und L10 je einen Wert für das 95. Perzentil und für L1 zusätzlich den Anteil der Tastendrücke im nächsten Bild.

Die sieben neuen Prüfungen decken ab: dass eine Bildlänge nur aus einer gemeldeten Rate entsteht und ohne sie ein Fehler zurückkommt; dass genau eine Bildlänge noch als erreicht zählt und eine Nanosekunde mehr nicht; dass 19 von 20 hält und 18 von 20 nicht; dass gehalten auch beim Anteil in jeder Runde gehalten heißt; dass das Perzentilmaß unberührt bleibt und für seine Zusagen keinen Anteil ausweist; dass eine Zusage ohne Maß kein Urteil bekommt; und dass jedes Maß seine Beschreibung für die Berichtsspalte liefert.

## Die Messstrecke, wiederholbar

```
cargo run --release -p krk-bench -- durchstich \
    --buendel target/KRK.app/Contents/MacOS/krk \
    --ordner-a /tmp/krk-pruefordner-a \
    --ordner100k /tmp/krk-pruefordner-gross \
    --runden 5
```

`--release` gehört dazu und stand in der Protokollzeile vom 260803-1755 versehentlich nicht: ein Bau ohne Optimierung weist sich im Bedingungskopf selbst als "nicht zur Abnahme einer Zusage geeignet" aus. Beide Prüfordner liegen unter `/tmp`, überleben keinen Neustart und entstehen deterministisch neu über `krk-bench fixture` mit den Startwerten 1 und 3. Prüfordner B (Startwert 2) braucht erst S21 für L5. Ein Lauf über fünf Runden dauert rund zehn Minuten.

## Was der nächste Schritt vorfindet

S9 (Auslieferungsbelegung, `ontocoder`) ist der nächste Schritt und berührt C8 nicht.

S21 erbt die Auswertung unverändert und misst L9 gegen dasselbe `Abnahmemass::AnteilImBild`; ein zweites Verfahren entsteht dort nicht. Was S21 dazulegt, ist der Messplan als Datei, die Prüfsitzung und die Zusagen L5 bis L9.

**Offen und für S22 vorgemerkt:** der L4-Befund oben. Vierzehn Defekte waren vor dieser Bearbeitung offen; keiner davon ist angefasst, weil der Nutzer die Reihenfolge festgelegt hat. Mit dem neuen sind es fünfzehn.

# Frühmessung als Gate (Schritt 8)

**Datum:** 260803-1755
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 8
**Neu angelegt:** `crates/krk-ui/src/appkit/bildtakt.rs`, `crates/krk-ui/src/messmodus.rs`, `messungen/260803-1554-durchstich.txt`
**Geändert:** `crates/krk-ui/src/appkit/{mod.rs,anwendung.rs,ereignisse.rs,tabelle.rs}`, `crates/krk-ui/src/main.rs`, `crates/krk-ui/Cargo.toml`, `Cargo.toml` des Workspace, `Cargo.lock`, `crates/krk-bench/src/{messen.rs,bericht.rs,main.rs}`
**Nicht angefasst:** `crates/krk-core/`, `xtask/`, `resources/`, `README.md`, `CLAUDE.md`
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen, wie für den `coder` vorgesehen. Ein Langform-Schreibprofil gibt `fusion-rules` für diesen Agenten nicht aus.

## Das Urteil zuerst

**Das Gate ist nicht bestanden.** Vier der fünf abgenommenen Zusagen halten ihre Zahl in jeder Runde mit dem Faktor zwei bis drei Abstand. L1 hält sie nicht, und zwar nicht stabil: in acht von achtzehn heute gefahrenen Runden lag das 95. Perzentil über den zugesagten 16 ms.

Der Schritt endet deshalb ohne Reparaturversuch und mit einem angelegten Entscheidungsdatensatz, wie das Abnahmekriterium es vorschreibt: `decisions/260803-1755_o_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`. Der Vermerk im Plan bleibt offen.

| Zusage | Was | 95. Perzentil, bestes bis schlechtestes aus fünf Runden | Zusage | Urteil |
|---|---|---|---|---|
| L1 | Tastendruck bis Ende des Zeichendurchgangs | 13,678 bis 16,225 ms | 16 ms | verfehlt in 1 von 5 Runden |
| L2 | erste Bildschirmseite auf Prüfordner A | 43,851 bis 45,071 ms | 100 ms | gehalten |
| L3 | vollständiges Lesen auf A, warm | 143,600 bis 160,411 ms | 400 ms | gehalten |
| L4 | Prozessstart bis bedienbares Fenster | 294,555 bis 303,540 ms | 1000 ms | gehalten |
| L10 | erste Bildschirmseite bei 100.000 Einträgen | 51,445 bis 53,052 ms | 100 ms | gehalten |

Als Beigabe gemessen, vom Gate nicht abgefragt: das vollständige Lesen der 100.000 Einträge liegt bei 965 bis 1020 ms gegen zugesagte 4 s.

Bericht: `messungen/260803-1554-durchstich.txt`.

## Woran L1 scheitert, und woran nicht

**Nicht an KRK.** Drei Befunde aus derselben Messung tragen das.

In den beiden vollständigen Gate-Läufen zu je hundert Tastendrücken lag **kein einziger Einzelwert** über 16,667 ms, also über einem Bild bei 60 Hz. Über alle 320 heute protokollierten Einzelwerte waren es zwei. Die Auswahl springt praktisch immer im nächsten Bild um, und das ist der Sache nach genau das, was L1 zusagt.

Der Anteil, den KRK selbst beisteuert, liegt bei 3 bis 8 ms: das Minimum über alle Läufe ist 3,035 ms, der Median 8,007 ms. Der Rest der gemessenen Spanne ist Warten auf die nächste Bildgrenze.

Und die Messvorschrift kann die Zahl kaum halten. L1 endet laut Plan am Ende des Zeichendurchgangs, festgestellt über einen `CADisplayLink`, der bei 60 Hz alle 16,667 ms taktet. Trifft ein Tastendruck das Bild an einer zufälligen Stelle, ist die Wartezeit bis zur nächsten Bildgrenze über [0; 16,667] verteilt, und deren 95. Perzentil liegt bei 15,83 ms — für eine Anwendung, die überhaupt keine Zeit verbraucht. Die Zusage von 16 ms ist die gerundete Bildlänge, und die Rundung von 0,667 ms ist größer als der gemessene Verfehlungsbetrag von 0,225 ms. **Hielte die Zusage bei einem Bild statt bei gerundeten 16 ms, hätte L1 in allen achtzehn Runden gehalten**; der schlechteste gemessene Wert war 16,617 ms.

Der Entscheidungsdatensatz legt dem Nutzer vier Möglichkeiten vor, mit Kosten je Möglichkeit, und empfiehlt die erste: die Zusage auf ein Bild des Bildschirms stellen. Er hält auch fest, dass L9 dieselbe Herleitung trägt und in S21 auf dieselbe Grenze treffen wird.

## Warum der Bericht mehrere Runden fährt

Die erste Fassung des Werkzeugs fuhr eine Runde und fällte daraus ein Urteil. Zwei aufeinanderfolgende Läufe ergaben für L1 einmal 16,576 ms und einmal 15,196 ms, also einmal "verfehlt" und einmal "gehalten" bei unverändertem Programm. **Ein Urteil, das zwischen zwei Läufen wechselt, ist keines**, und ein Bericht über eine einzelne Runde hätte diese Eigenschaft verdeckt — je nachdem, welchen Lauf man abheftet.

Der Unterbefehl trägt deshalb `--runden N`. Eine Runde ist genau die Messung, die C8 vorschreibt: zwanzig Wiederholungen je Zusage, das 95. Perzentil darüber. Eine Zusage gilt nur dann als gehalten, wenn sie es **in jeder Runde** tut. Die Runden zusammenzuwerfen und ein Perzentil über alles zu rechnen wäre derselbe Fehler von der anderen Seite gewesen: dann wäre es nicht mehr die Messung, die C8 nennt.

## Wie die Messung gebaut ist

Zwei Aufgaben, weil die fünf Zusagen zwei verschiedene Dinge messen.

```
krk-bench durchstich
   │
   ├─ 20 × ──> krk --messmodus start   ──> "bedienbar <Uhrzeit>"     = L4
   │           (ein Prozessstart je Wiederholung)
   │
   └─  1 × ──> krk --messmodus spannen ──> je 20 Werte für L1,L2,L3,L10
               (alle vier Spannen in einem Prozess)
```

**L4 misst der äußere Aufrufer**, weil nur er den Zeitpunkt vor dem Prozessstart kennt. Er nimmt die Uhrzeit unmittelbar vor `spawn`, die Anwendung meldet die Uhrzeit, zu der ihre Oberfläche bedienbar ist. Beide Zeitpunkte kommen von derselben Uhr desselben Geräts. Der Anfang liegt einen Wimpernschlag vor dem eigentlichen Prozessstart, weil `fork` und `exec` hineinfallen; die Zahl ist damit eher zu groß als zu klein, und das ist für eine Abnahme die richtige Richtung.

**Die vier übrigen Spannen misst die Anwendung.** Jede beginnt an einem Auslöser und endet an einer Bildgrenze.

Der Auslöser hängt an einem eigenen Zeitgeber mit 97 ms und ausdrücklich nicht an der Bildgrenze. Löste die Bildgrenze selbst den Tastendruck aus, läge zwischen Druck und nächster Bildgrenze immer genau ein volles Bild, und L1 hätte bei 60 Hz konstant 16,7 ms — nicht gemessen, sondern gebaut. 97 ms sind bei 60 Hz 5,82 Bilder; über zwanzig Wiederholungen wandert der Druckzeitpunkt damit durch das Bild.

## Die Grenze zum Modul `appkit`

`crates/krk-ui/src/appkit/bildtakt.rs` hält beide Berührungen mit AppKit hinter je einer Hülle, geschnitten wie die fünf vorhandenen Module.

`Zeichenende` umschließt den `CADisplayLink` auf der Ansicht des Dateifensters. Es nimmt beim Einrichten eine gewöhnliche Rust-Senke entgegen und ruft `invalidate` in seinem `Drop`, dieselbe Form wie `Tastenabgriff` aus S7, der sich dort bei AppKit abmeldet. `bildwiederholrate` schlägt `NSWindow.screen()` auf `maximumFramesPerSecond` nach und liefert `None`, wenn das Fenster auf keinem Bildschirm steht; der Aufrufer bricht dann mit Rückgabewert 3 ab, statt auf den Hauptbildschirm auszuweichen. Die Regel steht in S21 ausgeschrieben.

`crates/krk-ui/src/messmodus.rs` hält den Ablauf, die zwanzig Wiederholungen und die Ausgabe der Einzelwerte. **Es nennt keine `objc2`-Kiste**, nachgeprüft mit `grep -n '^use objc2' crates/krk-ui/src/messmodus.rs`, das nichts liefert. Über die Grenze gehen zwei gewöhnliche Rust-Werte: die Rate als Zahl und die Zeitpunkte der Bildgrenzen als `Instant`. In die Gegenrichtung gehen drei Zahlen über den Zustand der Liste und eine `Anweisung` aus vier Fällen.

`crates/krk-ui/src/appkit/mod.rs` trägt weiterhin als einzige Datei in `krk-ui` das `#[allow(unsafe_code)]`, nachgemessen und nicht behauptet.

## Fünf Festlegungen, die der Plan offenließ

**Das Ende jeder Spanne wird an der Bildgrenze abgefragt und nicht gemeldet.** Die Alternative wäre gewesen, `tabelle.rs` eine Messsonde einzubauen, die dem Messmodus meldet, wann der erste Stapel angekommen und wann der Lesevorgang fertig ist. Das hätte dauerhaft Messmaschinerie in den Produktivcode gelegt. Stattdessen fragt der Bildtakt an jeder Bildgrenze drei nur lesende Zugriffe ab: Zeilenzahl, ob noch gelesen wird, welche Zeile ausgewählt ist. Die erste Bildgrenze, an der die Zeilenzahl über null steht, **ist** der Zeitpunkt, an dem die erste Bildschirmseite zu sehen ist; früher gibt es nichts zu sehen.

**Der Tastendruck ist synthetisch, und der Weg ist der echte.** Die Anwendung baut ein `NSEvent` vom Typ `keyDown` mit dem Tastencode 125 und stellt es über `NSApplication.postEvent:atStart:` hinten in die eigene Ereignisschlange. Von dort läuft es durch den lokalen Ereignisabgriff aus S7, die Normalisierung der Zusatztasten und den Nachschlag im Kern bis in die Datenquelle. An `behandeln` ist dafür nichts geändert. `atStart: false` hängt das Ereignis hinten an, wie das System es mit einem echten Tastendruck tut; vorn einzureihen würde die Schlange umsortieren und damit etwas anderes messen.

**Das 95. Perzentil und der Bericht liegen in `krk-bench` und nicht in `messmodus.rs`.** Der Plan sagt das andersherum. Er kann es nicht meinen: L4 beginnt in einem anderen Prozess als dem, der es beendet, also kann ein Bericht über alle fünf Zusagen nur beim äußeren Aufrufer entstehen. Dazu erhebt `crates/krk-bench/src/bericht.rs` den Bedingungskopf seit S3; dieselbe Erhebung ein zweites Mal in `krk-ui` aufzubauen wäre eine zweite Wahrheit über das Berichtsformat. Gemeldet als `issues/260803-1755_o_schritt-8-legt-perzentil-und-bericht-in-eine-datei-die-nur-eine-haelfte-kennt.md`.

**Die Schranke gegen einen hängenden Messlauf hängt an der Uhr und nicht am Bildzähler.** Die erste Fassung zählte Bildgrenzen und brach nach 600 ab. Genau daran ist sie gescheitert: in der vierten von fünf Runden hörte der `CADisplayLink` mitten in der Messreihe auf zu takten, damit zählte nichts mehr, und der Lauf stand still, bis der äußere Aufrufer ihn nach fünf Minuten abschoss. Eine Schranke, die dasselbe Ereignis zählt, dessen Ausbleiben sie abfangen soll, kann nicht greifen. Jetzt prüft der Auslösetakt die Uhr; er läuft unabhängig vom Bildtakt. Die Abbruchmeldung nennt die Zahl der seither eingegangenen Bildgrenzen und trennt damit ein stehendes Bild von einer langsamen Oberfläche.

**Der Auslösetakt fasst die Startaufgabe nicht an.** In der Startaufgabe gibt es keinen Ablauf, nur die eine Bildgrenze, an der die erste Bildschirmseite steht. Ohne eine ausdrückliche Zeile fände der Auslösetakt eine leere Schrittliste vor und meldete `Fertig`, sobald er vor der ersten Bildgrenze drankäme — ein Rennen, das ein langsamer Startordner gewinnt und das dann eine Messung ohne Zahl ausgäbe. Der Fall ist heute nicht eingetreten, weil die erste Bildgrenze nach 17 ms kommt und der Auslösetakt nach 97 ms; er wäre eingetreten, sobald jemand die Messung auf einen langsameren Ordner richtet. Prüfung: `die_startaufgabe_wartet_auf_die_bildgrenze`.

## Was ich nicht sauber messen konnte

**L4 ist warm gemessen, C8 sagt Kaltstart.** "Kalt" heißt laut C8: erster Zugriff nach dem Leeren des Dateisystem-Caches, und geleert wird er unter macOS allein von `purge`, das Rechte braucht, die diese Sitzung nicht hat (`sudo -n true` scheitert mit "a password is required"). Ein Passwortdialog lässt sich in einem Messlauf nicht beantworten. Der Bericht weist L4 deshalb als warm aus, statt eine warme Zahl unter die Überschrift "kalt" zu setzen. Warm ist der leichtere Fall; die gemessenen 303 ms sind eine Untergrenze für die Zusage, die C8 wirklich stellt. Bei 1000 ms Zusage und dem Faktor drei Abstand ist der Ausgang wahrscheinlich derselbe, gemessen ist er nicht. L2, L3 und L10 sind von der Lücke nicht betroffen, weil C8 sie ohnehin warm zusagt.

**Die Prüfsitzung aus C8 ist am Durchstich nicht herstellbar.** C8 schreibt für L4 zwei Dateifenster mit je zwei Tabs vor; Tabs gibt es erst mit S12. Gemessen ist der Start des Bündels mit einem Fenster, ohne wiederhergestellte Sitzung, auf Prüfordner A. Diese Bedingung steht im Berichtskopf. Die gemessene Zahl ist damit günstiger als die spätere: zwei sichtbare Tabs kosten zwei erste Bildschirmseiten statt einer. Die Abnahme gegen die Prüfsitzung leistet S22, wie der Plan es vorsieht.

**Der körperliche Tastendruck bleibt ungemessen.** Belegt ist der Weg mit einem synthetischen Ereignis. Dass eine körperlich gedrückte Taste dieselben Ereignisse erzeugt, stammt weiterhin aus der Messung vom 260802-1137 und nicht aus dieser.

**Das Bild selbst bleibt ungemessen.** Eine Bildgrenze ist der Zeitpunkt, an dem das System sein nächstes Bild vorbereitet, nicht der, an dem ein Pixel leuchtet. Aus dem eigenen Prozess heraus ist der zweite nicht feststellbar. Der Plan nennt die Bildgrenze ausdrücklich als die erreichbare Näherung; der Bericht schreibt sie als solche aus, statt eine Photonenmessung zu behaupten.

## Die beiden Punkte aus der Codeprüfung, die die Messung berühren

**Der Einzugstakt (M4) begründet sich jetzt mit einer erhobenen Zahl.** `crates/krk-ui/src/appkit/tabelle.rs:54-58` schreibt "Ein Sechzigstel einer Sekunde ist ein Bild auf dem Referenzgeraet". Die Rate ist mit dieser Messung erhoben: `NSScreen.maximumFramesPerSecond` meldet 60 auf dem Bildschirm des gemessenen Fensters. Der Satz stimmt also. Der Defekt `260803-1536_o_einzugstakt-begruendet-sich-mit-einer-nicht-erhobenen-bildwiederholrate.md` bleibt offen, weil der Nutzer entschieden hat, erst zu messen und die Defekte danach anzugehen; wer ihn schließt, hat mit dieser Messung den Beleg.

**Die Generationsprüfung (M1) hat die Zahlen nicht berührt.** Der Messmodus stößt einen Lesevorgang erst an, wenn der vorige abgeschlossen ist; zwei Lesevorgänge überlappen sich in keiner Runde. Der Zweig, den die Prüfung nie erreicht, wäre auch bei einer wirksamen Prüfung nie erreicht worden.

**Ein dritter Punkt, den die Prüfung nicht nennt und den die Messung sichtbar macht.** Das vollständige Lesen kostet in der Anwendung ein Vielfaches dessen, was es kopflos kostet: Prüfordner A braucht kopflos 20,2 ms und in der Anwendung 160 ms, der Ordner mit 100.000 Einträgen kopflos 214,6 ms und in der Anwendung 1020 ms. Der Grund ist die Einzugsstrecke und nicht das Dateisystem: der Kanal zwischen Lesefaden und Hauptfaden hat die Tiefe 1, und der Hauptfaden räumt ihn sechzigmal je Sekunde. Bei rund zehn Stapeln für 10.000 Einträge und rund achtundneunzig für 100.000 ergibt das die gemessene Größenordnung. Beide Zusagen halten damit bequem, L3 mit dem Faktor zweieinhalb und L10 mit dem Faktor vier. Der Punkt gehört trotzdem festgehalten: **der Abstand zur Zusage ist hier ein Entwurfsparameter und keine Eigenschaft des Geräts.** Wer die Kanaltiefe oder den Einzugstakt anfasst, verschiebt L3 und L10 unmittelbar. Kein Defekt, weil nichts kaputt ist; eine Notiz für S12 und S21.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo test --workspace` | Rückgabewert 0, 108 Prüfungen in sieben Gruppen, davon 15 neue in `messmodus.rs` |
| `cargo clippy --workspace --all-targets` | Rückgabewert 0, keine Warnung |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `grep -n '^use objc2' crates/krk-ui/src/messmodus.rs` | kein Treffer |
| `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` | genau eine Datei, `crates/krk-ui/src/appkit/mod.rs` |
| `cargo xtask bundle` | Rückgabewert 0, signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)" |
| `codesign --verify --deep --strict target/KRK.app` | Rückgabewert 0 |
| `vtool -show-build-version .../MacOS/krk` | `minos 15.0` |
| Prüfordner A | über `krk-bench fixture --eintraege 10000 --seed 1` erzeugt, 10.000 Einträge, Steckbrief daneben |
| `krk-bench durchstich … --runden 5` | Rückgabewert 1, Bericht geschrieben, L1 verfehlt |

Das Abnahmekriterium des Schrittes ist in seinem ersten Teil erfüllt: der Bericht liegt vor, trägt den vollständigen Bedingungskopf mit allen acht Angaben einschließlich einer aus `NSScreen` gelesenen Bildwiederholrate von 60 Hz, und nennt für L1, L2, L3, L4 und L10 je einen Wert für das 95. Perzentil. Sein zweiter Teil, die fünf Schwellen, ist an L1 nicht erfüllt.

## Zwei gemeldete Defekte am Schritt selbst

- `issues/260803-1755_o_dateiliste-von-schritt-8-nennt-fuenf-noetige-dateien-nicht.md` — S8 nennt neun Dateien, gebraucht wurden vierzehn. Dazugekommen sind `tabelle.rs` für die drei lesenden Zugriffe, `ereignisse.rs` für das synthetische Ereignis, `krk-bench/src/bericht.rs` für die geteilten Kopfangaben, `krk-bench/src/main.rs` für den Unterbefehl und die `Cargo.lock`.
- `issues/260803-1755_o_schritt-8-legt-perzentil-und-bericht-in-eine-datei-die-nur-eine-haelfte-kennt.md` — der Absatz zur Grenze weist `messmodus.rs` das Perzentil und den Bericht zu; beides kann dort nicht entstehen.

## Zum Stand der Entscheidungsdatensätze

`decisions/260802-1428_a_was-l4-mit-wiederhergestellten-tabs-meint.md` bleibt auf "beantwortet". Diese Messung setzt die Antwort erstmals in eine Messvorschrift um: L4 endet an der ersten Bildschirmseite und nicht am vollständig gelesenen Ordner. Sie setzt sie aber nicht vollständig um, weil die Prüfsitzung aus zwei Fenstern mit je zwei Tabs erst mit S12 herstellbar ist und die Abnahme dagegen erst S22 leistet. Erst deren Commit zieht den Datensatz auf "umgesetzt".

Neu angelegt und offen: `decisions/260803-1755_o_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`.

## Was der nächste Schritt vorfindet

Der Plan hält S9 (Auslieferungsbelegung, `ontocoder`) für den nächsten Schritt. Er berührt C8 nicht und ist von der offenen L1-Frage nicht blockiert.

Die Messstrecke steht und ist wiederholbar:

```
cargo run -p krk-bench -- durchstich \
    --buendel target/KRK.app/Contents/MacOS/krk \
    --ordner-a /tmp/krk-pruefordner-a \
    --ordner100k /tmp/krk-pruefordner-gross \
    --runden 5
```

Beide Prüfordner liegen unter `/tmp` und überleben keinen Neustart; sie entstehen deterministisch neu über `krk-bench fixture` mit den Startwerten 1 und 3. Prüfordner B (Startwert 2) braucht erst S21 für L5.

S21 erbt `bildtakt.rs` unverändert und erweitert es nicht: der `CADisplayLink` und die Bildwiederholrate stehen dort seit diesem Schritt. Was S21 dazulegt, ist der Messplan als Datei, die Prüfsitzung und die Zusagen L5 bis L9. Das synthetische Tastenereignis steht bereits in `ereignisse.rs`, dort, wo der Plan es für S21 vorsieht; S21 erweitert es um die übrigen Tasten, statt es anzulegen.

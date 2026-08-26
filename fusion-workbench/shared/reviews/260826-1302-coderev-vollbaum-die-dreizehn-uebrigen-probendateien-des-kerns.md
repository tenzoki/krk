# Vollbaum-Durchsicht R6: die dreizehn übrigen Probendateien von `krk-core`

**Reviewed-range:** `4a57028..4a57028`
**Not-opened:** none

Kein Commit-Bereich, sondern eine Vollbaum-Durchsicht der dreizehn übrigen Dateien unter
`crates/krk-core/tests/` am Baumstand HEAD `4a57028`. Beide Enden der Spanne benennen
denselben Commit, weil kein Bereich gelesen wurde, sondern ein Stand; die zwei aufgelösten
Kurzhashes stehen da, damit `bin/fusion-review-coverage` die Zeile lesen kann. Alle dreizehn
Dateien sind vollständig geöffnet, deshalb `none` — das Pflichtfeld ist maschinenlesbar, und
`bin/fusion-review-coverage` liest allein dieses eine Wort.

**Sender:** coderev
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Gelesen:** 13 Dateien, 8.687 Zeilen, jede vollständig, jede Zeilenzahl am Baum abgelesen
**Nachgemessen:** ein eigens gebautes `libtest`-Binärziel im Kratzverzeichnis, um das
Verhalten eines nicht treffenden Filters zu belegen. Kein `cargo test` und kein `cargo build`
im Projektbaum: zwei weitere Prüfer arbeiten parallel, und ein `cargo test` räumt den
Messplan eines gleichzeitig laufenden Messlaufs ab.

## Summary

Die dreizehn Dateien sind in bemerkenswertem Zustand. Ihr auffälligstes Merkmal ist, dass sie
die Frage dieser Durchsicht — misst diese Probe überhaupt etwas? — selbst schon stellen und an
über einem Dutzend Stellen beantworten: `assert!(geprueft > 0, …)`, „sonst belegt die Probe
nichts", „die Probe misst nicht, was sie messen soll". Eine Probe, die auch bei kaputtem Code
hielte, habe ich in keiner der dreizehn gefunden; `assert_eq!(x, x)`, eine leere Schleife oder
ein `is_ok()` ohne Blick auf den Wert kommt nicht vor.

Die acht Befunde liegen deshalb alle an derselben Kante: **eine Deckung, die nicht die Probe
zusagt, sondern der Text daneben.** Der schwerste ist der gemeinsame Kindstarter — sechs
Elternproben prüfen allein den Rückgabewert des Kindes, und `libtest` beendet sich mit 0, wenn
der Filter nichts trifft. Dahinter stehen genau die sechs Zusagen, die `CLAUDE.md` als „sonst
nur behauptet" führt. Daneben steht ein Befund, der die Prosa nicht nur schwächt, sondern
widerlegt: eine vierte Prüfordner-Fassung steht in `xtask/src/release.rs`, und die Zählprobe,
die „genau drei" zusagt, kann sie nicht sehen.

**Eine Voraussetzung der Aufgabe hält am Baum nicht**, und sie steht darum unten unter
„Berichtigung": `umfang.rs` ist nicht die Zähldatei dieses Projekts. Die Zählproben stehen in
`baum.rs`.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 4 |
| Low | 4 |

Dazu ein Entscheidungsdatensatz. Alle acht Befunde sind als eigene Datei in `shared/issues/`
gefiltert; kein Circle ist aktiv, also gilt die Herkunftsregel auf den gemeinsamen Speicher.
Vor dem Schreiben sind die 225 offenen Datensätze auf Doppelung geprüft; ein Treffer, dazu
unten.

## Befunde nach Thema

### Thema 1: Ein stiller Weg durch eine Probe, sechsfach

**`gemeinsam::kind_mit_deskriptorgrenze` — Medium.**
`crates/krk-core/tests/gemeinsam/mod.rs:334-351`, sechs Rufer.

Der Starter setzt diese Argumentfolge zusammen (`:344`):

```rust
"ulimit -n {grenze} && exec \"$0\" --exact --ignored --nocapture --test-threads 1 \"$1\""
```

Am 260826-1258 an einem eigens gebauten Prüfziel nachgemessen, nicht aus dem Quelltext
geschlossen:

```
$ ./probebin --exact --ignored --nocapture --test-threads 1 kind_gibt_es_nicht
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
EXIT=0
```

Jeder der sechs Rufer prüft allein `ergebnis.status.success()`; jede der sechs Kindproben
kehrt daneben bei fehlender Umgebungsvariablen still zurück. Zwei stille Wege, sechs Zusagen
dahinter: der eine Verzeichnisdeskriptor des Durchlaufs, der eine der gedeckelten Zählung, der
eine der Zusammenfassung, und dreimal „ein Mangel von außen lässt unentschieden".

`umfang.rs:266`, `:355`; `verzeichnis.rs:2571`, `:2783`, `:2883`; `leseprofil.rs:3487`.

Datensatz: `shared/issues/260826-1302_*_sechs-elternproben-am-gemeinsamen-kindstarter-bleiben-gruen-wenn-der-kindname-nicht-trifft.md`.

**Doppelung geprüft und benannt.** `shared/issues/260825-2127_*_die-kindproben-in-tests-zeit-rs-bleiben-gruen-wenn-ihr-name-nicht-trifft.md` beschreibt dieselbe Klasse an der **anderen** Starterfassung, `zeit.rs::kindprobe_in_zone`, und rät zur Lösung je Rufer nach dem Vorbild `ablage.rs`. Der gemeinsame Starter ist der Fall, in dem die Lösung an **eine** Stelle gehört und alle sechs deckt; das ist neue Auskunft, also ein eigener Datensatz plus eine `Also seen:`-Zeile am älteren.

### Thema 2: Eine Zählprobe, deren Heuhaufen kleiner ist als ihr Satz

Vier Zusagen dieses Projekts sind Aussagen über den **Baum** und keine über einen
Rückgabewert. Sie stehen in `baum.rs` und lesen ihren Stoff aus `gemeinsam::quelldateien()`
(`gemeinsam/mod.rs:272-287`), das sich auf `crates/` beschränkt. Neben `crates/` liegt `xtask/`
mit neun `.rs`-Dateien, und keine Zählprobe des Baums betritt sie.

**Eine vierte Prüfordner-Fassung steht in `xtask` — Medium.**
`xtask/src/release.rs:905-931` erklärt `Wegwerfwurzel`: `impl Drop`, `std::env::temp_dir()`,
`remove_dir_all`, mit Prozesskennung und Laufnummer im Namen. Das sind alle drei Zeichen, nach
denen `genau_drei_pruefordner_fassungen_stehen_im_baum` (`baum.rs:113-153`) sucht, und der
Doc-Kommentar der Fassung sagt es selbst: „Ein Wegwerf-Wurzelordner, **wie ihn die Proben des
Kerns benutzen**." `CLAUDE.md` sagt „genau drei Fassungen, eine je Kiste, und das soll so
bleiben"; der Baum trägt vier, und die Stelle, die das melden soll, sieht die vierte nicht.

Das ist **genau der Fehler, den dieselbe Probe schon einmal gemacht hat.** Ihr Doc-Kommentar
schreibt es aus (`baum.rs:93-107`): bis zur Runde 7 band sie an den Namen `Pruefordner` und
übersah eine vierte Fassung namens `Ordner`. Die Gegenmaßnahme war, nach dem Gegenstand statt
nach dem Namen zu suchen. Sie greift — nur nicht in einem Verzeichnis, das die Suche nie
betritt. Nicht die Nadel ist zu eng, sondern der Heuhaufen.

Die zwei anderen Zählproben sind von derselben Grenze betroffen und heute sachlich nicht
verletzt: `xtask` führt keine Abhängigkeit und kann `atomar::schreiben` nicht erreichen, und es
trägt kein `#![deny(unsafe_code)]`, dessen Ausnahme zu zählen wäre. Der Doc-Kommentar von
`quelldateien` ist an dieser Stelle genau („Jede `.rs`-Datei unter `crates/`"); die
Probennamen und Meldetexte sind es nicht und sprechen durchweg vom „Baum".

Datensatz: `shared/issues/260826-1302_*_eine-vierte-pruefordner-fassung-steht-in-xtask-und-die-zaehlprobe-c4-6-kann-sie-nicht-sehen.md`.

**`ueber_der_ablage_stehen_genau_zwei_absprachen` liest eine Datei — Low.**
`baum.rs:215-241` greift `krk-core/src/ablage/sperre.rs` aus dem Quellbaum heraus und zählt
darin Zeilen, die mit `pub const` beginnen und `.lock"` enthalten. Der Doc-Kommentar sagt „eine
dritte Absprache bräuchte eine dritte Datei, und sie fällt hier auf"; sie fällt auf, wenn sie
in **dieser** Datei als `pub const` mit dieser Endung steht. Die zwei Zusicherungen darunter
gegen die echten Konstanten decken die Hälfte „diese zwei gibt es"; ungedeckt bleibt genau die
Hälfte, für die die Quelltextzählung überhaupt da ist.

Datensatz: `shared/issues/260826-1302_*_die-probe-ueber-die-zwei-absprachen-liest-nur-sperre-rs-und-saehe-eine-dritte-daneben-nicht.md`.

### Thema 3: Eine Aufzählung, deren Vollständigkeit ein Doc-Kommentar behauptet

**Ein achter `Wirkungsbereich` übersetzt ohne Eintrag im Beschriftungsfeld — Medium.**
`belegung.rs:1892-1908`. Der Doc-Kommentar von `stelle_in_den_sieben` schließt: „sie ist
ebenfalls ohne Auffangzweig, also uebersetzt ein achter Wert erst, wenn er auch hier und damit
im Feld steht." Der erste Halbsatz stimmt, der zweite ist der ungedeckte Schritt: der
Übersetzer verlangt einen **Zweig**, und der darf `7` liefern. `SIEBEN_BESCHRIFTUNGEN` ist ein
`[(Wirkungsbereich, &str); 7]` fester Länge, das nichts mitzieht.

Alle drei Beschriftungsproben iterieren über das Feld (`:1911`, `:1936`, `:1955`), nicht über
die Varianten. Ein achter Wert bekäme damit keine Prüfung: seine Beschriftung dürfte leer sein,
dürfte einen senkrechten Strich tragen, der die Pipe-Tabelle in
`~/Downloads/KRK-Tastenbelegung.md` zerbricht, und dürfte mit der eines anderen Bereichs
übereinstimmen — die drei Dinge, die die drei Proben ausschließen sollen. Der Zweig in
`stelle_in_den_sieben` würde nie ausgeführt: sein einziger Rufer steht innerhalb der
Feldschleife (`:1912`).

Datensatz: `shared/issues/260826-1302_*_ein-achter-wirkungsbereich-uebersetzt-ohne-eintrag-im-beschriftungsfeld-der-doc-kommentar-sagt-das-gegenteil.md`.

### Thema 4: Eine Probe, deren Trennschärfe an der Zone des Geräts hängt

**Die MS-DOS-Zeitprobe misst in einer Zone ohne Sommerzeit nichts — Medium.**
`operation.rs:1370-1418`. Sie soll den Fehler fangen, den `ditto(1)` macht: einen Zonenversatz
je Lauf statt je Zeitpunkt. Die Erwartung rechnet sie mit **derselben** Funktion aus, die der
Packlauf nimmt (`:1387` gegen `src/operation/zippen.rs:701`) — richtig so, denn die Zusage
lautet „je Zeitpunkt gerechnet" und nicht „richtig gerechnet"; letzteres hält `tests/zeit.rs`
mit festen Kalenderwerten. Die Trennschärfe kommt allein daraus, dass `ortszeit(SOMMER)` und
`ortszeit(WINTER)` verschiedene Versätze liefern. Unter `TZ=UTC`, in jeder Zone ohne
Umstellung und auf einem Gerät ohne Zonendatenbank tun sie das nicht, und ein Packlauf mit dem
Fehler bestünde die Probe.

Der Doc-Kommentar nennt die Bedingung ausdrücklich — „in einer Zone mit Sommerzeit" —, und
nichts prüft sie. Die Vorlage steht daneben: `zeit.rs` startet für genau dieses Problem
Kindprozesse mit gesetztem `TZ` und begründet es im Modulkopf.

Datensatz: `shared/issues/260826-1302_*_die-msdos-zeitprobe-misst-in-einer-zone-ohne-sommerzeit-nichts-und-nichts-haelt-ihre-voraussetzung.md`.

### Thema 5: Zwei Antworten auf dieselbe Frage in einer Prüfsammlung

**Was tut eine Probe, die unter `root` nichts messen kann? — Entscheidung, kein Defekt.**
`text.rs:734-758` und `:1225-1250` schweigen und kehren zurück; `operation.rs:528-565` und
`:770-812` fielen aus; `arbeitsbaum.rs:274-287` weicht dem Fall aus und begründet das
ausdrücklich mit der `root`-Lage. Drei Antworten, keine Regel. Welche gilt, hängt daran, ob
dieser Baum je unter `root` geprüft wird, und das kann nur der Nutzer beantworten.

Datensatz: `shared/decisions/260826-1302_*_schweigt-eine-probe-die-unter-root-nichts-messen-kann-oder-faellt-sie-aus.md`.

### Thema 6: Zeitmessungen und die Sorgfalt, die ungleich verteilt ist

**Die Abbruchprobe des Stapels — Low.**
`operation.rs:715-768` hält dieselbe 100-ms-Zusage aus C4 wie ihre Nachbarin über der
500-MB-Datei, aber in **einem** Versuch, ohne `ZEITMESSUNG` und damit neben jeder anderen Probe
der Datei. Die Nachbarin trägt fünfundzwanzig Zeilen Doc-Kommentar darüber, warum genau das
nicht trägt („in 1 von 8 bis 2 von 7 Faellen"), und zieht daraus fünf Versuche und die Sperre.
Dazu: der Modulkopf sagt „`ZEITMESSUNG` laesst deshalb immer nur eine von **beiden** laufen" —
vier Proben nehmen die Sperre (`:264`, `:365`, `:459`, `:503`).

Datensatz: `shared/issues/260826-1302_*_die-abbruchprobe-des-stapels-misst-die-wanduhr-ohne-die-sperre-und-ohne-die-wiederholung-der-nachbarin.md`.

**Nicht gefiltert, weil schon gefiltert:** die Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`text.rs:824-908`) ist mit ihren 15 Sekunden und 20.000 Durchläufen die lastanfälligste Probe der dreizehn Dateien. Drei offene Datensätze behandeln sie bereits: `260825-2127_*_die-wettrennprobe-ein-wechsel-der-art-unter-dem-oeffnen-faellt-gelegentlich-aus.md`, `…_die-wettrennprobe-des-oeffnens-ist-lastabhaengig-und-ihre-marge-traegt-keinen-parallelen-bau.md` und `…_die-wettrennprobe-des-oeffnens-braucht-allein-neun-sekunden-von-fuenfzehn-und-faellt-unter-last.md`. Kein vierter.

### Thema 7: Die Grenze `deny(unsafe_code)` endet an `src/`

**Die Probenziele tragen kein `deny(unsafe_code)`, und eines führt fünf `unsafe`-Stellen — Low.**
`src/lib.rs:1` trägt das Attribut; jede Datei unmittelbar unter `tests/` ist eine eigene Kiste
und erbt es nicht. `textkopien.rs:61-73` führt `unsafe impl GlobalAlloc`, zwei `unsafe fn` und
zwei `unsafe`-Blöcke — sachlich einwandfrei und begründet, aber außerhalb jeder Buchführung.
Die Zählprobe C4.5 (`baum.rs:65-83`) sucht nach `#![allow(unsafe_code)]` und findet richtig
zwei: wo keine Sperre steht, braucht es keine Öffnung, und die Zählung schweigt. `CLAUDE.md`
sagt daneben „**Der Bau erzwingt diese Grenze**" — er erzwingt sie für `src/`.

Datensatz: `shared/issues/260826-1302_*_die-probenziele-des-kerns-tragen-kein-deny-unsafe-code-und-eines-fuehrt-fuenf-unsafe-stellen.md`.

### Thema 8: Drei kleine Textstellen

**Low, in einem Datensatz zusammengefasst**, weil es eine Korrekturrunde ist und keine drei:

- `belegung.rs:181` verweist in eckigen Klammern auf `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke` — die Probe gibt es seit dem Fall der Sprungmarke in der Runde 10 nicht mehr. Ein Intra-Doc-Link in einem Probenziel prüft niemand.
- `text.rs:205-207`: der Kommentar „Ein Bytepaar mitten in einem Mehrbytezeichen ist kein Treffer" steht über einer Zusicherung, die zählt, wie oft eine gültige Teilzeichenfolge vorkommt. Was er meint, hält die Schleife darüber.
- `operation.rs:10-11`: „Beide entstehen unter `/tmp`" — sie entstehen im `Pruefordner`, und der liegt unter `std::env::temp_dir()`, auf macOS also unter `/var/folders/…/T`. Der Modulkopf des Prüfordners hält den Unterschied ausdrücklich fest.

Datensatz: `shared/issues/260826-1302_*_drei-prosastellen-in-den-probendateien-des-kerns-behaupten-mehr-oder-anderes-als-daneben-steht.md`.

## Berichtigung einer Voraussetzung der Aufgabe

**`umfang.rs` ist nicht die Zähldatei dieses Projekts.** Der Auftrag hat sie als solche
vorgelegt („`umfang.rs` zählt. Zählproben sind die einzige Bremse gegen die Prosa"). Am Baum
misst sie `krk_core::verzeichnis::umfang`, also die gedeckelte Zählung eines Unterbaums vor dem
Löschen — Dateisystem, nicht Quelltext. Sie hält vier Zahlen, alle aus `SCHWELLE`
zurückgerechnet statt hingeschrieben (`umfang.rs:49`), und sie ist eine der sorgfältigsten
Dateien der dreizehn.

Die Zählproben über den Quelltext stehen in **`baum.rs`** (vier Stück) und in
`verzeichnis.rs` (fünf weitere Rufer von `quelldateien()`). Sie sind unter Thema 2 behandelt.
Ich nenne das hier, weil die Aufgabe die falsche Datei als „einzige Bremse gegen die Prosa"
benannt hat und ein späterer Leser sonst denselben Griff täte.

## Die Vorlagen der Aufgabe, einzeln geprüft

| Was vorgelegt war | Befund am Baum |
|---|---|
| `Kommando::KENNUNGEN` ist ungehalten, beide Proben iterieren über die Liste selbst (`260826-1223`) | **Hält unverändert.** `jedes_kommando_traegt_genau_einen_wirkungsbereich` (`belegung.rs:1697-1730`) und `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` (`:1061-1090`) laufen beide über `Kommando::KENNUNGEN.into_iter()`. Keine Stelle in den dreizehn Dateien iteriert über die Varianten. |
| Gibt es weitere Aufzählungen mit derselben Lücke? | **Ja, eine.** `Wirkungsbereich` gegen `SIEBEN_BESCHRIFTUNGEN`, Thema 3. Sie ist schwerer als `KENNUNGEN`, weil ein Doc-Kommentar die Lücke ausdrücklich als geschlossen bezeichnet. |
| `tasten.rs` prüfen | **Trägt zu `Kommando` nichts bei.** Die 102 Zeilen messen allein die Normalisierung der Modifikatorbits, ohne eine Zeile über Kommandos oder Kennungen. Fünf Proben, alle scharf; die Rohbitwerte kommen aus `normalisierung::roh` und stehen nicht zweitgeschrieben da. Kein Befund. |
| `gemeinsam/mod.rs` als wichtigster Einzelgegenstand: räumt `Drop` wirklich ab? | **Ja, zweistufig.** `abraeumen` (`:216-221`) versucht `remove_dir_all` und steigt bei Fehlschlag über `entsperren_und_loeschen` Eintrag für Eintrag hinab und dreht jedem die Rechte zurück — nötig gegen die `0o000`-Einträge der Rechteproben. Ein Socket, an dem `remove_dir_all` vorbeikäme, ist ausdrücklich mitbedacht (`:161-175`). |
| Trägt er Prozesskennung und Laufnummer? | **Ja**, beide (`:70-74`): `krk-kern-probe-{zweck}-{pid}-{laufnummer}`, Laufnummer aus einem `AtomicU64` je Prozess. |
| Können zwei gleichzeitige Läufe kollidieren? | **Nein.** Verschiedene Prozesse tragen verschiedene Kennungen; innerhalb eines Prozesses trennt die Laufnummer. Das `abraeumen(&pfad)` vor dem Anlegen (`:75`) trifft nur einen Rest desselben Prozesses. |
| Baut eine der 13 Dateien an ihm vorbei einen eigenen Ordner? | **Keine der dreizehn.** Alle acht Dateien mit Dateisystembedarf nehmen `Pruefordner`. `textkopien.rs`, `stapelumbenennen.rs`, `zwischenablage.rs`, `tasten.rs` und `zeit.rs` brauchen keinen. **Aber:** ein vierter steht in `xtask/src/release.rs`, Thema 2. |
| Schreibt eine Probe ins echte Temporärverzeichnis? (`260810-1925`, geschlossen) | **Der Fall gilt nicht mehr, und keine der dreizehn baut ihn nach.** Er betraf `krk-bench`s `plan_schreiben`; dort steht er unverändert. In den dreizehn Dateien schreibt keine Probe unmittelbar nach `temp_dir()` — alle gehen über den Prüfordner mit Kennung und Laufnummer. Der `Messplan`-Pfad wird von keiner berührt. |
| Erbt eine Probe die angehobene Deskriptorgrenze? | **Nein, keine.** Jede Zusage über Deskriptoren geht durch `kind_mit_deskriptorgrenze`. `umfang.rs` begründet in seinem Modulkopf sogar, warum es eine **tiefere** Grenze als 64 braucht (`GRENZE = 24`, weil der Deckel ohnehin auf 26 begrenzt), und das Kind rechnet nach, dass es wirklich weniger als 26 bekommt (`:391-401`) statt die Grenze zu behaupten. Vorbildlich — und genau deshalb wiegt Thema 1, das die Meldung dieses Kindes nicht liest. |
| `zeit.rs`: flatterhaft? | **Nein.** Es hängt an keiner Uhr des Wirts: fünf feste Epochensekunden gegen feste Kalenderwerte, gerechnet in Kindprozessen mit gesetztem `TZ`. Die Elternprobe, die in der Zone des Geräts läuft (`:118-141`), prüft allein Bereiche, die jede Zone erfüllt, und einen Minutenabstand. Die Sommerzeitumstellung ist mit `UMSTELLUNG` in beide Richtungen gemessen (`:205-206`). Die einzige Schwäche ist der Kindstarter, und die trägt der ältere Datensatz. |

## Querschnitt

**Diese Prüfsammlung stellt die Frage dieser Durchsicht schon selbst — an über einem Dutzend
Stellen.** `belegung.rs:786`, `:857`, `:880`, `:1000`; `umfang.rs:195`, `:391`, `:397`;
`operation.rs:1544`; `text.rs:672`; `arbeitsbaum.rs:188`, `:202`, `:296`. Die Form ist immer
dieselbe: eine Zusicherung, die nicht den Prüfling misst, sondern den Aufbau, und deren
Meldetext ausschreibt „sonst belegt die Probe nichts". Sechs der acht Befunde dieser Durchsicht
sind Stellen, an denen genau diese Form **fehlt** — nicht Stellen, an denen etwas Fremdes
geschieht. Das ist eine Aussage über die Reife der Sammlung: die Regel ist da und wird an sechs
Stellen nicht angewandt.

**Der zweite Querschnitt ist die Prosa.** Sieben der acht Befunde haben eine gemeinsame Gestalt:
nicht der Code ist falsch, sondern der Satz daneben sagt mehr, als der Code hält —
„und damit im Feld", „eine dritte fällt hier auf", „im Baum", „nur eine von beiden", „unter
`/tmp`", „in einer Zone mit Sommerzeit". Dieses Projekt hat gegen diese Klasse eine
ausgearbeitete Gewohnheit: keine Zahl in die Prosa, die eine Probe zählen kann. Die Gewohnheit
greift für Zahlen. Für **Reichweiten** — welchen Umfang liest die Nadel, welche Bedingung muss
gelten, damit die Messung trennt — greift sie noch nicht.

**Der dritte Querschnitt ist ein Verzeichnis.** `xtask/` ist der blinde Fleck jeder Zählung des
Baums, und dort steht bereits eine Übertretung. Wer eine der Zählproben erweitert, sollte zuerst
die Frage aus Thema 2 beantworten: zählt `xtask` mit?

## Was ausdrücklich nicht gefiltert ist

- **`jedes_kommando_traegt_genau_einen_wirkungsbereich`s `matches!` über die sieben Bereiche** (`belegung.rs:1704-1717`) sieht wie eine Tautologie aus und ist keine: käme ein achter Bereich und trüge ihn ein Kommando, würde sie rot. Sie ist die einzige Stelle, die das täte.
- **`der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck`** (`operation.rs:356`) ist mit seinen fünf Versuchen und der ausgeschriebenen Messreihe die sorgfältigste Zeitmessung des Baums. Kein Befund; es ist die Vorlage, an der Thema 6 gemessen wird.
- **`ein_eintrag_der_aus_dem_zielordner_herausfuehrt_entsteht_nirgends`** (`operation.rs:2206`) prüft mit `!Path::new("/absolut.txt").exists()` gegen die echte Dateisystemwurzel. Das ist eine Abhängigkeit vom Zustand des Wirts, aber die einzige Form, in der die Zusage „nicht in der Wurzel" überhaupt prüfbar ist. Nicht gefiltert.
- **Der `#[global_allocator]` in `textkopien.rs`** gilt für das ganze Binärziel; die zweite Probe der Datei könnte theoretisch mitzählen. Der Modulkopf (`:24-31`) rechnet vor, warum nicht: sie setzt `ZAEHLT` nie, und ihre Zeichenketten liegen ein Dutzend Bytes unter `ZAEHLGRENZE`. Die Rechnung trägt.

## Empfohlene Reihenfolge

Kein Befund ist ein Auslieferungshindernis; keiner beschreibt ein Fehlverhalten zur Laufzeit.

1. **Zuerst die vierte Prüfordner-Fassung** (Thema 2). Sie ist der einzige Befund, bei dem der Baum eine Zusage aus `CLAUDE.md` heute **verletzt** und nicht nur ungedeckt lässt. Sie zieht die Frage „zählt `xtask` mit?" nach sich, und diese Frage sollte vor der nächsten Zählprobe beantwortet sein.
2. **Dann der Kindstarter** (Thema 1). Eine Zeile im Starter deckt sechs Rufer und jeden künftigen; das ist der beste Aufwand-Nutzen-Schnitt der ganzen Durchsicht.
3. **Dann die zwei Doc-Kommentare, die eine Deckung behaupten** (Themen 3 und 2b). Sie schicken einen Leser aktiv in die falsche Richtung, und das ist schlimmer als eine Lücke, die dasteht.
4. **Dann die Zonenbedingung** (Thema 4) — eine Zeile, sofort.
5. **Zuletzt** die drei Textstellen, die `unsafe`-Buchführung und die Abbruchprobe des Stapels.
6. **Die Entscheidung zum `root`-Lauf** gehört dem Nutzer vorgelegt, bevor jemand an den vier Rechteproben etwas ändert.

## Verification

Dreizehn Dateien geöffnet und vollständig gelesen, 8.687 Zeilen; jede Zeilenzahl mit `wc -l`
am Baum abgelesen und nicht gerechnet. Jede Behauptung über eine Zeilennummer stammt aus
`cat -n` oder `sed -n` auf dem Stand `4a57028`. Das Verhalten von `libtest` bei einem nicht
treffenden Filter ist an einem eigens gebauten Binärziel im Kratzverzeichnis gemessen, nicht
aus der Dokumentation geschlossen. Die vierte Prüfordner-Fassung ist in `xtask/src/release.rs`
gelesen, nicht aus einem `grep`-Treffer erschlossen. Der Umfang von `quelldateien()` ist gegen
`git ls-files '*.rs'` gehalten. Keine Datei im Quellbaum ist geändert; kein `cargo build` und
kein `cargo test` im Projektbaum gefahren, weil zwei weitere Prüfer parallel arbeiten und ein
`cargo test` den Messplan eines gleichzeitigen Messlaufs abräumt. Vor dem Filtern sind die 225
offenen Defektdatensätze auf Doppelung geprüft; der eine Treffer ist benannt und hat eine
`Also seen:`-Zeile bekommen.

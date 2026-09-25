# Aufräumdurchgang: neun Code-Defekte behoben, einer liegen gelassen

---
**Agent:** coder
**Status:** Complete
**Datum:** 260805-0952
**Umfang:** nur `crates/`
**Commit:** keiner (der Orchestrator committet)

---

## Was der Auftrag war

Zehn offene Code-Defekte in einer festgelegten Reihenfolge: zuerst zwei, die zusammenhängen und S13c abnahmefähig machen, danach acht unabhängige. Grenze: nur `crates/`, kein Eingriff in `resources/`, `xtask/`, die Plandatei oder den Spec. Die `unsafe`-Grenze mit je einer Ausnahme in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs` gilt unverändert und ist eingehalten.

Neun sind behoben, einer liegt. Nach jedem Defekt und am Ende laufen die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` mit 0. Der Testlauf zählt 13 Testprogramme, alle mit 0 gescheiterten Prüfungen.

## Die neun behobenen Defekte

### 1. Der Testlauf war rot: Cmd+Q als Beispiel für eine unbelegte Kombination

`260805-0820_c_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`

Die Prüfung nennt kein Beispiel mehr. Sie heißt jetzt `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke`, sammelt jede Kombination, die eine Funktion der Auslieferungsbelegung führt, geht über alle 61 Tasten der Tabelle mal die 15 nicht leeren Zusatztastenmasken und prüft die Zusage an jeder Kombination, die dabei frei bleibt.

Die Absicherung gegen die nächste Erweiterung geht damit über die **Herkunft** des Beispiels statt über seine Wahl. Beim letzten Mal, bei `cmd+arrowleft`, ging sie über den Namen: `arrowleft` steht nicht in der Tastentabelle und darf nie hinein, weil die Schreibweise die Taste `left` nennt. Hier greift das nicht, jede Kombination darf belegt werden; jedes Ersatzbeispiel wäre dieselbe Falle eine Runde später. Die Prüfung liest deshalb die freien Kombinationen aus der Belegung, statt eine hinzuschreiben.

Zwei Vorkehrungen dazu: die Masken sind aus `ModMaske::BENANNT` gerechnet und nicht aufgezählt, damit eine fünfte Zusatztaste die Liste nicht still unvollständig macht; und eine Schlusszeile `assert!(geprueft > 0, …)` fängt den Fall ab, dass die Auslieferungsbelegung eines Tages jede Kombination mit Zusatztaste vergibt und es nichts mehr zu prüfen gibt.

### 2. Der eigene Selektor für Beenden, und damit S13c

`260805-0753_c_macos-stellt-zu-terminate-eine-zweitform-quit-and-keep-windows-auf-opt-cmd-q.md`

Der Anwendungsdelegierte trägt den Selektor `beenden:` neben `fensterEinblenden:` und `fensterSchliessen:`; der Menüeintrag trägt ihn statt `terminate:`, und der Delegierte ruft `terminate:` an `NSApplication` selbst.

**Die Zweitform verschwindet, gemessen.** Am neu gebauten, signierten Bündel, über die Bedienungshilfen an der laufenden Anwendung ausgelesen, einmal vier und einmal zwölf Sekunden nach dem Start:

```
KRK/KRK beenden  kuerzel=Q mod=0 aktiv=true
```

Das ist der einzige Eintrag des Menüs "KRK". Vorher stand "Quit and Keep Windows" mit `mod=2` daneben. Der `inference:` des Defekts, die Zweitform hänge allein an `terminate:`, ist damit nachgemessen und bestätigt, auf demselben Weg wie der ursprüngliche Befund und nicht über `--menue-protokoll`. Cmd+Q beendet unverändert: Tastendruck gesendet, danach findet `pgrep -x krk` keinen Prozess mehr.

**`NOTBEHELF_BEENDEN` und `notbehelf_befehl` sind fort**, dazu die Prüfung, die allein die Konstante las. Der Eintrag läuft über `befehl(…, "beenden")` wie die übrigen sechs. `menue.rs` legt danach **keine** Kombination mehr als Zeichenkette fest; die beiden verbliebenen Treffer eines `grep` stehen in Fließtext, der eine gemessene Eingabe und ein Übersetzungsbeispiel beschreibt. Das fehlende Abnahmekriterium von S13c trägt damit.

Dazu, wie vom `ontocoder` vermerkt: der Kopfkommentar von `systemzusaetze_unterdruecken` belegte seine Messung mit einem `plutil -extract`, das nach dem Entfernen der beiden Schlüssel aus `resources/Info.plist` nicht mehr reproduziert. Die Messung steht jetzt in der Vergangenheitsform, mit einem Absatz, der sagt, warum sie sich nicht nachstellen lässt und wo sie vollständig steht.

Ein Zusatz, den `resources/default-keymap.toml` ausdrücklich zusagt ("Solange er noch nicht steht, trägt die Funktion kein Kommando"): der Kern führt jetzt `Kommando::Beenden`, und `kommando_ausfuehren` schickt es auf dieselbe Methode wie der Menüeintrag. Am Verhalten ändert das nichts, es verschiebt nur den Weg — der Ereignisabgriff schluckt Cmd+Q jetzt selbst, statt es an das Menü weiterzureichen.

### 3. Zwei tote Generationsleser im Kern

`260803-2025_c_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md`

Weg 1 des Datensatzes: entfernt. `Meldung::generation()`, `Lesevorgang::generation()` und das damit unbenutzte Feld `Lesevorgang::generation`. Weg 2 (stehen lassen als Vorsorge für S12) trägt nicht mehr: S12 ist gebaut, jeder Tab hält seinen eigenen `Lesevorgang`, und `einzug_je_tab` liest allein aus dessen Kanal. Ein dritter Leser war ebenfalls tot und ist mitgegangen: `Lesevorgang::ist_abgebrochen()`.

Die Generationsnummer bleibt, wo sie ist, und trägt zwei Aufgaben: sie benennt den Lesefaden und sagt `Ordnermodell::leeren`, zu welchem Lauf der Inhalt gehört. Beides sind Aufgaben des Aufrufers, der sie ohnehin hält. Der Kopfkommentar von `leser.rs` schreibt diesen Zuschnitt jetzt aus, statt weiter den am 260803 entfernten Filter zu beschreiben.

### 4. Die Bildlaufposition stand am oberen Rand auf −28

`260804-1040_c_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`

Auflösung 1: beim Schreiben und Lesen um den Ursprung verschieben. `bildlauf_ursprung()` fragt ihn ab; in `session.toml` heißt 0 jetzt "ganz oben".

Der Ursprung kommt aus der **Kopfansicht der Tabelle** und nicht aus dem Inhaltsrand der Bildlaufansicht. Beides war denkbar; die erste Fassung nahm `NSScrollView::contentInsets` und verschob nichts. Eine Sonde im laufenden Bündel entschied es:

```
SONDE roh=-28 insets.top=0 kopf=Some(28.0)
```

Nachgemessen am laufenden Bündel: vorher trugen die beiden sichtbaren Tabs `bildlauf = -28.0`, nach Start und Beenden ohne Bildlauf tragen sie `0.0`. Die Sonde ist vollständig zurückgenommen.

### 5. `FSEventStreamScheduleWithRunLoop` ist seit macOS 13 abgelöst

`260804-1451_c_fseventstreamschedulewithrunloop-ist-seit-macos-13-als-veraltet-gekennzeichnet.md`

Gewechselt auf `FSEventStreamSetDispatchQueue` mit `DispatchQueue::main()`. Die zusätzliche Bindung, die der Datensatz erwartet, fällt weg: `dispatch2` geht den Weg über `_dispatch_main_q` bereits, und KRK führt die Kiste seit Schritt 16. `CFRunLoop` und `CFRunLoopMode` sind aus dem Modul verschwunden.

Nachgemessen am laufenden Bündel: eine im Terminal angelegte Datei stand nach 0,65 s in der Dateiliste, abgelesen über die Bedienungshilfen. Innerhalb der Sekunde, die S14 zusagt.

### 6. Ein verdeckter Tab auf einem ausgeworfenen Datenträger

`260804-1451_c_ein-verdeckter-tab-auf-einem-ausgeworfenen-datentraeger-behaelt-seinen-toten-pfad.md`

Möglichkeit 2 des Datensatzes: jeden Tab herunterholen, eine Meldung je Dateifenster. Die Naht in `tabs.rs` ist `Tabliste::verdeckten_tab_setzen`; sie startet ausdrücklich keinen Lesevorgang, weil auf keinem Schirm etwas steht. Die Entscheidung, welcher Tab getroffen ist, bleibt in `crate::auffrischung` und damit AppKit-frei.

Die Meldung sagt jetzt, was umgezogen ist. Der alte Satz "das Dateifenster zeigt jetzt X" wäre falsch, wenn allein ein verdeckter Tab umgezogen ist; `auswurfmeldung` unterscheidet vier Fälle.

Nachgemessen an einem eigens angelegten Datenträger (`hdiutil create -size 10m -fs APFS -volname KrkPruef`):

```
vorher  tabs:  krk-s17b[0] man[1] fotos[0] musik[0] man[0]
vorher  zeile: (leer)
$ hdiutil detach /Volumes/KrkPruef
nachher tabs:  krk-s17b[0] man[1] k1[0] k1[0] man[0]
nachher zeile: KrkPruef wurde ausgeworfen; 2 verdeckte Tabs zeigen jetzt /Users/k1
```

Danach auf einen der umgezogenen Tabs gewechselt: 12 Zeilen, keine leere Liste. Genau das war der Schaden.

### 7. Der Abbruchwunsch erreichte den Lauf erst mit der nächsten Meldung

`260804-1816_c_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md`

`Lauf::abbruchgriff() -> Abbruchgriff` gibt das Abbruchkennzeichen heraus. Entscheidend ist, was danach mit dem alten geschah: `Vorgangszustand` trug einen zweiten `AtomicBool`, den der Vermittlerfaden abfragte und weiterreichte. Der ist entfallen — ein Kennzeichen statt zwei, kein zweiter Weg neben dem ersten. Der `Receiver` bleibt ungeteilt; geteilt wird allein das `Arc<AtomicBool>`, das der Lauf ohnehin hält.

Nachgemessen mit `der_abbruchgriff_wirkt_von_einem_faden_ohne_den_lauf`: eine 500-MB-Kopie, der `Lauf` wandert auf einen zweiten Faden, der abbrechende Faden hat ihn nicht und liest keine Meldung. Der Bericht meldet `Abgebrochen`. Was **nicht** gemessen ist: der Fall, in dem die Spanne vorher wirklich groß war (`trashItemAtURL:` auf einem sehr großen Ordner). Für Kopieren und Verschieben war sie schon vorher klein.

Drei Leser sind mit dem Umweg entfallen, weil sie ihn trugen und sonst niemanden.

### 8. Die Trennung von Stamm und Endung stand an zwei Stellen

`260804-2040_c_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen.md`

`namen_teilen` ist öffentlich, `stamm_und_endung` ist fort, `Regel::anwenden` ruft `namen_teilen`. Geblieben ist die Rechnung über `rfind('.')`, weil sie geliehene Ausschnitte liefert; `Path::extension` streicht den Punkt, und ihn wieder anzusetzen kostete je Aufruf eine `String`.

Die Begründung der abgelösten Fassung ("so trennen wie `Path`") ist zur Prüfung geworden: `die_trennung_stimmt_mit_der_trennung_von_path_ueberein` rechnet beide Wege nebeneinander. Ein Probeprogramm unter `/tmp` fand über elf Namen genau eine Abweichung, `".."`, und den kann KRK nicht antreffen: `name_pruefen` weist ihn ab.

Nebenbefund gegen meine eigene Vermutung: `Path::extension("datei.")` liefert `Some("")` und damit `"."`, nicht `None`. Beide Fassungen erhalten den nachgestellten Punkt.

### 9. Zwei Module des Kerns hießen `umbenennen`

`260804-2040_c_zwei-module-des-kerns-heissen-umbenennen.md`

`krk_core::umbenennen` heißt jetzt `krk_core::stapelumbenennen`. Nicht die zweite Auflösung (`operation::stapel`): der Modulkopf begründet seit S17, warum das Modul neben `operation::umbenennen` steht und nicht darin, und diese Begründung trägt. Umbenannt ist der Name, nicht der Ort.

Am laufenden Bündel gegengeprüft, zusammen mit Defekt 8: drei Dateien, Regel `IMG_` → `Urlaub ` mit Nummer ab 7 und drei Stellen, Vorschau und Ergebnis `Urlaub a007.jpg`, `Urlaub b008.jpg`, `Urlaub c009.jpg`.

## Der eine, der liegen bleibt

`260805-0000_o_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md` bleibt offen. Der Datensatz hat einen Nachtrag bekommen, damit die nächste Runde nicht bei null anfängt.

Der Umfang des Schadens ist bestätigt, gelesen und nicht gemessen: kein Aufrufer in `krk-ui` wartet je auf einen Lesefaden, der einzige `warten`-Aufruf im Arbeitsbereich steht in `krk-bench`. Die Oberfläche bleibt also bedienbar, wie der Datensatz selbst schreibt; **die laufende Anwendung setzt sich nicht fest.** Was bleibt, ist je Navigationsversuch ein Faden in einem Systemaufruf, der nicht zurückkehrt.

Der zweite der beiden im Datensatz genannten Wege, "das Aufgeben des Fadens ohne ihn abzuwarten", ist bereits der heutige Zustand und die Ursache des Verlusts statt seine Behebung: aufgeben kann KRK den Faden, beenden nicht. Bleibt der erste, die Zeitschranke, und die bringt einen Zeitgeber in einen Pfad ohne, beendet den hängenden Faden auch dann nicht und ließe sich ohne einen eigens aufgesetzten Server nicht belegen. Das ist mehr Arbeit als die übrigen sieben Aufräumdefekte zusammen, mit einer Vermutung als Ergebnis.

## Neue Defekte

- `260805-0841_o_menue-protokoll-sieht-die-spaet-gestellten-zweitformen-nicht.md` — der zweite, kleinere Teil des Selektor-Datensatzes, den ich nicht mitgeprüft habe: `--menue-protokoll` liest unmittelbar nach `finishLaunching` aus und sieht die spät gestellten Zweitformen nicht.
- `260805-0905_o_das-merkmal-cfrunloop-in-cargo-toml-hat-keinen-nutzer-mehr.md` — Folge des FSEvents-Wechsels. `Cargo.toml` liegt im Wurzelverzeichnis und war außerhalb des Umfangs.
- `260805-0947_o_die-dateiliste-von-s17-nennt-den-alten-modulpfad-umbenennen.md` — Folge der Modulumbenennung. Die Plandatei war ausgenommen.

## Geänderte Dateien

Alle unter `crates/`. Nichts in `resources/`, `xtask/`, der Plandatei oder dem Spec.

```
krk-core/src/ablage/sitzung.rs
krk-core/src/lib.rs
krk-core/src/operation/fortschritt.rs
krk-core/src/operation/mod.rs
krk-core/src/operation/umbenennen.rs
krk-core/src/tasten/belegung.rs
krk-core/src/verzeichnis/leser.rs
krk-core/src/umbenennen/         -> krk-core/src/stapelumbenennen/   (4 Dateien, git mv)
krk-core/tests/belegung.rs
krk-core/tests/operation.rs
krk-core/tests/umbenennen.rs     -> krk-core/tests/stapelumbenennen.rs (git mv)
krk-ui/src/appkit/anwendung.rs
krk-ui/src/appkit/blaetter/stapelumbenennen.rs
krk-ui/src/appkit/fsevents.rs
krk-ui/src/appkit/menue.rs
krk-ui/src/appkit/tabelle.rs
krk-ui/src/auffrischung.rs
krk-ui/src/kommandos/operationen.rs
krk-ui/src/tabs.rs
```

Die `unsafe`-Grenze steht unverändert: `grep -rn "allow(unsafe_code)" crates/` findet die beiden erlaubten Ausnahmen in `krk-core/src/verzeichnis/sys.rs:50` und `krk-ui/src/appkit/mod.rs:1`, sonst nichts.

Prüfdaten unter `/tmp` (`krk-namenprobe`, `krk-stapelprobe`, `krk-fsevents-probe`, `krk-pruef.dmg`, zwei Sondenprotokolle) sind wieder entfernt, der Prüfdatenträger ist ausgehängt; `ls /Volumes/` zeigt nur `Macintosh SSD`. Ältere Reste unter `/tmp` aus vorigen Sitzungen sind unberührt geblieben.

## Nicht committet

Wie beauftragt. Der `[DONE]`-Vermerk von S13c bleibt dem Auftraggeber.

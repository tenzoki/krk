# Tastenereignisse und Pfeiltasten (Schritt 7)

**Datum:** 260803-1309
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 7
**Neu angelegt:** `crates/krk-core/src/tasten/{mod.rs,normalisierung.rs}`, `crates/krk-core/tests/tasten.rs`, `crates/krk-ui/src/appkit/ereignisse.rs`
**Geändert:** `crates/krk-core/src/lib.rs`, `crates/krk-ui/src/appkit/{mod.rs,anwendung.rs,tabelle.rs}`, `crates/krk-ui/src/main.rs`, `crates/krk-ui/Cargo.toml`, `Cargo.toml` des Workspace, `Cargo.lock`
**Nicht angefasst:** `xtask/`, `resources/`, `README.md`, `CLAUDE.md`, `crates/krk-bench/`, `crates/krk-core/src/verzeichnis/`
**Stilprofil:** `stilwerk/chat-voice-de.yaml` geladen, wie für den `coder` vorgesehen. Ein Langform-Schreibprofil gibt `fusion-rules` für diesen Agenten nicht aus.

## Was entstanden ist

Der Weg vom Tastendruck bis in das Ordnermodell steht, und er läuft über einen
einzigen Eintrittspunkt.

`crates/krk-core/src/tasten/normalisierung.rs` trägt die Normalisierung als reine
Funktion. `normalisieren(rohe_flaggen)` nimmt den Wert aus `NSEvent.modifierFlags`
und liefert eine `ModMaske` mit höchstens vier gesetzten Bits: `command`, `control`,
`option`, `shift`. Gelöscht werden `function`, die Feststelltaste, der Zehnerblock
und die Hilfetaste. Die rohen Bitwerte stehen in einem eigenen öffentlichen
Untermodul `roh`, damit die Prüfungen sie benennen können, statt Zahlen zu
wiederholen.

`crates/krk-core/src/tasten/mod.rs` setzt darauf den Nachschlag: ein `Tastendruck`
aus Tastencode und Maske ergibt höchstens ein `Kommando`. Die Tabelle trägt die fünf
Tasten, die der Plan nennt, mit den virtuellen Tastencodes 126 (Pfeil auf), 125
(Pfeil ab), 116 (Bild auf), 121 (Bild ab) und 36 (Return).

`crates/krk-ui/src/appkit/ereignisse.rs` hält den lokalen Ereignisabgriff. Er wird
über `NSEvent.addLocalMonitorForEventsMatchingMask:handler:` auf `keyDown`
eingerichtet, schlägt jeden Druck im Kern nach und reicht das Kommando an die
Datenquelle des Dateifensters. Trifft der Nachschlag, schluckt er das Ereignis; sonst
reicht er es unverändert weiter, damit Cmd+Q und Cmd+W ihren gewohnten Weg gehen.

`crates/krk-ui/src/appkit/tabelle.rs` legt die Kommandos aus: Auswahl um eine Zeile,
Auswahl um eine Bildschirmseite, in den ausgewählten Ordner hineinsteigen.

`crates/krk-ui/src/main.rs` wertet die Marke `--tasten-protokoll` aus und reicht sie
über `appkit::starten` bis zum Abgriff durch.

## Sechs Festlegungen, die der Plan offenließ

**Die Maske ist ein eigener Typ und keine Zahl.** `ModMaske` ist ein Neutyp über
einem Byte mit vier Konstanten und einer festen Anzeigereihenfolge. Der Plan nennt in
`## Datenstrukturen` den Nachschlag `HashMap<(u16, ModMaske), FunktionsId>`; damit war
der Name gesetzt, die Form nicht. Ein nacktes `u8` hätte an jeder Aufrufstelle die
Frage offen gelassen, ob es die rohen AppKit-Bits oder die normalisierten trägt. Der
Typ beantwortet sie einmal. Seine `Display`-Ausgabe (`command+shift`, leer als
`keine`) ist zugleich das, was der Protokollmodus schreibt, und sie hält eine feste
Reihenfolge, damit `command+shift` und `shift+command` denselben Text ergeben.

**Eine gehaltene Zusatztaste schlägt nicht durch.** `kommando` liefert nur dann
etwas, wenn die Maske leer ist. Umschalt+Pfeil ab bleibt damit unbelegt, statt wie ein
nacktes Pfeil ab zu wirken. Der Grund ist C2: die Taste gehört später der
Bereichsauswahl, und ein Nachschlag, der die Maske übergeht, hätte den Platz schon
vergeben. Die Prüfung `eine_gehaltene_zusatztaste_nimmt_der_verdrahteten_taste_ihr_kommando`
hält die Regel fest.

**Die Auslegung der Kommandos steht in der Tabelle, nicht im Abgriff.** Der Abgriff
kennt weder `NSTableView` noch Ordnermodell; er liefert ein `Kommando`. Auswahl
bewegen, Seitenhöhe erfragen und in einen Ordner hineinsteigen sind Sache der
Datenquelle, die beides ohnehin hält. Die Alternative, dem Abgriff die Tabelle
mitzugeben, hätte ihm eine zweite Zuständigkeit gegeben und die Grenze zwischen
"welche Taste" und "was tut sie" verwischt.

**Die Seitenhöhe wird erfragt und nicht gerechnet.** `rowsInRect(visibleRect)` liefert
die Zahl der sichtbaren Zeilen. Sie aus Fensterhöhe und Zeilenhöhe zu rechnen wäre
eine zweite Wahrheit neben der, die AppKit ohnehin führt, und sie ginge falsch, sobald
ein Kopf, eine Fußzeile oder ein zweiter Bereich dazukommt. Das Mindestmaß von einer
Zeile fängt den Fall ab, dass die Tabelle noch keine Größe hat; eine Seitentaste, die
um null Zeilen springt, wäre eine tote Taste.

**Am Rand bleibt die Auswahl stehen, statt umzulaufen.** Und ohne bestehende Auswahl
fängt sie an dem Rand an, aus dem die Bewegung kommt: Pfeil ab setzt auf die erste
Zeile, Pfeil auf auf die letzte. Das ist das Verhalten, das der Finder und jede
Listenansicht des Systems zeigen.

**Die Datenquelle merkt sich den angezeigten Ordner.** Ein `Eintrag` trägt nur seinen
Namen. Ohne den Ordner daneben ließe sich aus einer ausgewählten Zeile kein Ziel für
`Oeffnen` bauen. Das neue Feld `pfad` wird in `ordner_lesen` gesetzt, also an der
einen Stelle, an der der angezeigte Ordner wechselt.

## Zwei Grenzen, die dieser Schritt bewusst stehen lässt

**Aus einem Ordner führt in dieser Runde kein Weg zurück.** Der Plan nennt für S7 fünf
Tasten, und "eine Ebene höher" ist keine davon. Wer mit Return in einen Ordner
hineingeht, bleibt dort, bis er das Programm neu startet. Das ist kein Defekt, sondern
der Zuschnitt: S13 setzt die Tastaturnavigation aus C2 vollständig um, und dazu gehört
der Rückweg. Ein sechster Tastencode nebenbei wäre eine Vorwegnahme gewesen, die S13
gleich wieder anfassen müsste.

**Einer symbolischen Verknüpfung folgt `Oeffnen` nicht.** Der Leser meldet eine
Verknüpfung als Verknüpfung und nicht als das, worauf sie zeigt; eine Verknüpfung auf
einen Ordner öffnet deshalb nichts. Ob KRK ihr folgen soll, ist eine Frage an C2 und
nicht an diesen Schritt. Eine Datei öffnet ebenfalls nichts: Ansehen und Bearbeiten
sind eigene Funktionen und kommen mit dem Editor.

## Eine neue Abhängigkeit: `block2`

`NSEvent.addLocalMonitorForEventsMatchingMask:handler:` nimmt seinen Rückruf als
Objective-C-Block entgegen. Ohne die Kiste `block2` lässt sich keiner bauen, und einen
zweiten Weg zu diesem Abgriff gibt es nicht. Die Kiste lag bereits als mittelbare
Abhängigkeit im `Cargo.lock`, weil `objc2-app-kit` sie in seinen Vorgabemerkmalen
führt; neu ist allein, dass `krk-ui` sie unmittelbar nennt. Die Versionsangabe steht
in `[workspace.dependencies]`, wie S1 es für die drei objc2-Kisten hält.

Die naheliegende Alternative wäre gewesen, `NSApplication` oder die `NSTableView` zu
unterklassen und `keyDown:` zu überschreiben. Sie hätte die Abhängigkeit gespart und
dafür genau das aufgegeben, was S7 herstellen soll: einen einzigen Eintrittspunkt.
Eine Ansicht, die eine Taste selbst abfängt, ist die Sonderregel mit eigenem
Rückfallweg, die die Maxime "supersimpel" ausschließt.

## Abnahme

### Belegt, mit Ausgabe

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | Rückgabewert 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | Rückgabewert 0 |
| `cargo test --workspace` | Rückgabewert 0, 95 Prüfungen in sieben Gruppen, davon 13 neue |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `cargo clippy --workspace --all-targets` | Rückgabewert 0, keine Warnung |
| `cargo test -p krk-core --test tasten` | Rückgabewert 0, 8 von 8 Prüfungen bestanden |
| `cargo test -p krk-core tasten` | Rückgabewert 0, 6 Prüfungen, 7 gefiltert (siehe die Meldung unten) |
| `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` | genau eine Datei, `crates/krk-ui/src/appkit/mod.rs` |
| `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src` | genau eine Datei, `crates/krk-core/src/verzeichnis/sys.rs` |

Die beiden Prüfungen, die das Abnahmekriterium wörtlich verlangt, sind darunter:
`f3_mit_und_ohne_function_ergibt_dieselbe_nachschlagemaske` (Tastencode 99 mit
gesetztem `function` und ohne ergeben dieselbe Nachschlagemaske) und
`cmd_shift_k_behaelt_beide_bits`.

### Der ganze Weg, mit synthetischen Tastenereignissen belegt

Eine vorübergehende Sonde hat den Weg vom Ereignis bis in das Ordnermodell im
laufenden Programm durchgemessen. Sie baute mit
`NSEvent.keyEventWithType:location:modifierFlags:…:keyCode:` echte `keyDown`-Ereignisse
und stellte sie über `NSApplication.postEvent:atStart:` in die Ereignisschlange der
eigenen Anwendung. Sie setzte dabei `function` und `numericPad`, also genau die Bits,
die AppKit bei den Pfeiltasten setzt. Zwei Läufe, gegen `$HOME` mit 12 Zeilen und
gegen `/usr/share` mit 41 Zeilen; der zweite ist der aussagekräftigere, weil dort die
Bildschirmseite kleiner ist als die Liste:

```
PROBE 0 start           zeilen=41 auswahl= -1 seite=29 pfad=Some("/usr/share")
tastencode=125 maske=keine kommando=AuswahlRunter
PROBE 1 nach pfeil-ab   zeilen=41 auswahl=  0 seite=29
tastencode=125 maske=keine kommando=AuswahlRunter
PROBE 2 nach pfeil-ab   zeilen=41 auswahl=  1 seite=29
tastencode=126 maske=keine kommando=AuswahlHoch
PROBE 3 nach pfeil-auf  zeilen=41 auswahl=  0 seite=29
tastencode=121 maske=keine kommando=SeiteRunter
PROBE 4 nach bild-ab    zeilen=41 auswahl= 29 seite=30
tastencode=116 maske=keine kommando=SeiteHoch
PROBE 5 nach bild-auf   zeilen=41 auswahl=  0 seite=29
tastencode=125 maske=command kommando=unbelegt
PROBE 6 nach cmd+ab     zeilen=41 auswahl=  0 seite=29
tastencode=36 maske=keine kommando=Oeffnen
PROBE 7 nach return     zeilen= 0 auswahl= -1 pfad=Some("/usr/share/ans2_dummy_dir")
```

Fünf Aussagen stecken darin. Die Normalisierung arbeitet im laufenden Programm: der
Tastencode 125 kommt mit gesetztem `function` und `numericPad` an und ergibt die
Maske `keine`. Die Pfeiltasten bewegen die Auswahl um eine Zeile. Bild ab springt um
29 Zeilen, also um eine gemessene Bildschirmseite und nicht um einen festen Wert, und
Bild auf bringt zurück. Cmd+Pfeil ab bleibt unbelegt und lässt die Auswahl stehen, was
die Maskenprüfung im Betrieb zeigt. Return steigt in den ausgewählten Ordner hinein
und stößt einen neuen Lesevorgang an. Nebenbei belegen die eingestreuten Zeilen, dass
`--tasten-protokoll` jeden empfangenen Tastendruck mit Code und normalisierter Maske
schreibt.

**Was diese Messung nicht zeigt.** Sie stellt Ereignisse selbst in die Schlange. Dass
eine körperlich gedrückte Taste dieselben Ereignisse erzeugt, ist damit nicht
gemessen, sondern aus der Fn-Messung vom 260802-1137 übernommen. Und sie sagt nichts
über das Bild: dass die ausgewählte Zeile sichtbar umspringt, bleibt ungeprüft.

Die Sonde ist vollständig zurückgenommen. `grep -rn 'probe' crates/krk-ui/src
crates/krk-core/src` liefert nichts, und Bau, Prüfungen, Formatierung und Clippy sind
nach dem Rückbau erneut gelaufen; die Tabelle oben zeigt diesen Stand.

### Nicht belegt

Drei Punkte des Abnahmekriteriums brauchen das signierte Bündel und einen Menschen an
der Tastatur. Sie bleiben offen, und keiner von ihnen wird hier behauptet:

- **Dass die Pfeiltasten die Auswahl im laufenden Bündel bewegen.** Belegt ist der Weg
  mit synthetischen Ereignissen, nicht mit einem Tastendruck auf ein signiertes
  Bündel.
- **Dass Bild auf und Bild ab um eine Bildschirmseite springen, sichtbar auf dem
  Schirm.** Die Zahlen stimmen, das Bild ist ungeprüft.
- **Dass `--tasten-protokoll` bei F3, F5 und F8 die Codes 99, 96 und 100 nennt.** Die
  Codes stammen aus der Messung vom 260802-1137 und sind gegen den Produktcode nicht
  nachgeprüft; kein Teil dieser Sitzung hat eine Funktionstaste gedrückt.

**Ein Bündel ist in dieser Sitzung nicht entstanden, und zwar mit Absicht.** Auf dem
Gerät steht ein Schlüsselbund-Dialog offen, den der Nutzer noch nicht beantwortet hat;
jeder `cargo xtask bundle` bliebe im Signierschritt hängen, wie es am Ende von Schritt
6 geschehen ist. Der Aufruf ist deshalb unterblieben. `target/KRK.app` liegt seit
jenem abgebrochenen Lauf unsigniert; das Repository ist davon nicht berührt, weil
`target/` in der `.gitignore` steht.

## Vier Meldungen

- `issues/260803-1309_o_dateiliste-von-schritt-7-nennt-fuenf-noetige-dateien-nicht.md`
  — S7 nennt fünf Dateien, gebraucht wurden zehn. Dazugekommen sind die beiden
  `Cargo.toml` für `block2`, `main.rs` für die Befehlszeilenmarke, `anwendung.rs` für
  das Einrichten und Festhalten des Abgriffs und `tabelle.rs` für die Ausführung der
  Kommandos.
- `issues/260803-1309_o_abnahmekommando-von-schritt-7-filtert-nach-testnamen-statt-nach-datei.md`
  — `cargo test -p krk-core tasten` filtert nach Prüfungsnamen und lässt sieben der
  acht Prüfungen in `tests/tasten.rs` aus, darunter beide, die das Kriterium wörtlich
  verlangt. `--test tasten` trifft die Datei.
- `issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md` — eine über
  `open` gestartete Anwendung hat keine Standardausgabe. Das Protokoll ist über
  `target/KRK.app/Contents/MacOS/krk --tasten-protokoll` aus einem Terminal zu lesen.
- `issues/260803-1309_o_entscheidung-zur-unsafe-grenze-steht-noch-auf-beantwortet.md`
  — `decisions/260803-1208_a_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`
  steht auf `_a_`, obwohl Commit `569e8e0` sie umgesetzt hat. Gehört zu S6 und ist
  deshalb gemeldet statt nebenbei geändert.

## Zum Stand der Entscheidung über die Funktionstasten

`shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md` bleibt auf
`_a_`. Schritt 7 legt einen Teil ihrer Umsetzung: die Löschung von `function` aus der
Nachschlagemaske ist genau das, was das C3-Kriterium "Der Nutzer kann fn nicht als
Zusatztaste einer Belegung verwenden" verlangt. Die Antwort selbst ist damit noch
nicht realisiert, denn sie sagt aus, welche Funktion auf F3 bis F8 liegt und welches
Cmd-Kürzel daneben. Beides entsteht mit `resources/default-keymap.toml` in S9 und der
Belegungsmaschine in S11. Erst deren Commit zieht den Datensatz auf `_i_`.

## Was der nächste Schritt vorfindet

S8 misst L1 am Tastendruck. Der Eintrittspunkt dafür steht: `behandeln` in
`crates/krk-ui/src/appkit/ereignisse.rs` sieht jedes `keyDown` mit dem `NSEvent` in
der Hand, und dessen Zeitstempel ist der Anfang der Spanne, die L1 misst. Das Ende
kommt aus dem `CADisplayLink`, den S8 dazunimmt.

Die verdrahtete Tabelle in `crates/krk-core/src/tasten/mod.rs` ist der Platz, den S11
einnimmt. Was von diesem Schritt bleibt, sind `ModMaske` und `normalisieren`: der
Nachschlag der Belegungsmaschine läuft über dieselbe Maske, und die Prüfungen in
`crates/krk-core/tests/tasten.rs` gelten für sie unverändert weiter.

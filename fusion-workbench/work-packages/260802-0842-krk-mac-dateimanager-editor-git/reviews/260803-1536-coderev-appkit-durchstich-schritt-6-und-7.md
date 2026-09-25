# Codeprüfung: der AppKit-Durchstich, Schritt 6 und 7

**Datum:** 260803-1536
**Sender:** coderev
**Prüfgegenstand:** Commits `569e8e0` (S6) und `6b4fb2d` (S7)
**Umfang:** `crates/krk-ui/src/appkit/{mod,anwendung,fenster,tabelle,menue,ereignisse}.rs`,
`crates/krk-ui/src/main.rs`, `crates/krk-core/src/tasten/{mod,normalisierung}.rs`,
`crates/krk-core/tests/tasten.rs`, `crates/krk-ui/Cargo.toml`, `Cargo.toml` des Workspace
**Nicht geprüft:** `crates/krk-core/src/verzeichnis/` (nur als Kontext gelesen),
`crates/krk-bench/`, `xtask/`
**Maßstab:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitte
`## Aufbau`, `### Frage 2`, `### Frage 6`, Schritte 6 und 7; die Protokolle
`history/260803-1244-fenster-menue-und-echte-dateiliste.md` und
`history/260803-1309-tastenereignisse-und-pfeiltasten.md`

---

## Urteil

Der Durchstich trägt: die Fadenregel hält strukturell, die Eigentumsverhältnisse
sind zyklenfrei, und die `unsafe`-Stellen sind bis auf zwei Begründungen sauber
belegt. Die acht Befunde betreffen fast durchweg die Belegkette, nicht das
Verhalten; die eine Ausnahme ist die Auswahl, die das Sortieren am Ende eines
Lesevorgangs nicht übersteht.

Vor der Frühmessung aus Schritt 8 ist keiner der Befunde ein Aufhalter. Zwei von
ihnen sind es aber wert, vorher erledigt zu werden, weil sie das Muster für die
nächsten sechzehn Schritte setzen.

## Zahlen

| Gewicht | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 2 |
| Mittel | 4 |
| Niedrig | 1 |

Dazu drei Beobachtungen ohne eigenen Datensatz, unten im Abschnitt
`## Geprüft, kein Datensatz`.

---

## Befunde nach Themen

### Belegkette: was der Text behauptet und was gemessen wurde

**H1 — Der Modulkopf der Normalisierung belegt drei Aussagen mit einer Messung,
die sie nicht trägt.** *Hoch.*
`issues/260803-1536_o_normalisierung-belegt-drei-aussagen-mit-einer-messung-die-sie-nicht-traegt.md`

`crates/krk-core/src/tasten/normalisierung.rs:9-22` nennt
`spikes/fn-tasten/messung-A-neuauswertung.txt` als Beleg für drei Sätze. Die
Messdatei trägt keinen davon:

| Behauptung | Stand der Messung |
|---|---|
| "Fn+F3 und ein nacktes F3 erzeugen dasselbe Ereignis" (Zeile 14-15) | Die Neuauswertung sagt zu genau dieser Frage "NICHT MESSBAR AUF DIESEM GERÄT" (Zeile 69-72) |
| "auch bei den Pfeiltasten" (Zeile 13) | Im Rohprotokoll kommt keine Pfeiltaste vor: 17 Ereignisse, davon `a`, `b`, `c`, F3, F5, F8, fn, Shift |
| "AppKit setzt sein [Zehnerblock-]Bit auch bei den Pfeiltasten" (Zeile 21-22) | `NSEventModifierFlagNumericPad` steht in keinem gemessenen `roh=`-Wert |

Der erste Satz ist zugleich genau der Fehler, den Commit `f865fca` aus der
ursprünglichen Selbstauswertung entfernt hat. Er kehrt eine Ebene höher zurück,
im Programmtext. Derselbe Satz steht in `crates/krk-core/tests/tasten.rs:24-26`.

Das Verhalten ist richtig: AppKit setzt beide Bits tatsächlich so. Falsch ist der
Beleg, und der Modulkopf ist die Stelle, an der ein späterer Leser nachschlägt,
statt neu zu prüfen.

**M4 — Der Einzugstakt begründet sich mit einer Bildwiederholrate, die das
Projekt als unerhoben führt.** *Niedrig bis mittel.*
`issues/260803-1536_o_einzugstakt-begruendet-sich-mit-einer-nicht-erhobenen-bildwiederholrate.md`

`crates/krk-ui/src/appkit/tabelle.rs:54-58` schreibt "Ein Sechzigstel einer
Sekunde ist ein Bild auf dem Referenzgeraet". Der geschlossene Defekt
`260802-1900_c_bildwiederholrate-…` hält fest, dass die Rate am Referenzgerät
nicht erhoben ist und erst S8 sie aus `NSScreen` liest. Bei 120 Hz räumte der
Zeitgeber den Kanal nur bei jedem zweiten Bild, und der zweite Satz des
Kommentars wäre falsch. S8 misst die Rate ohnehin; der Zeitpunkt zum Nachziehen
ist genau dort.

### Verhalten: die Auswahl

**H2 — Die Auswahl des Nutzers überlebt das Sortieren am Ende eines
Lesevorgangs nicht.** *Hoch. Der einzige Befund mit sichtbarer Wirkung.*
`issues/260803-1536_o_auswahl-ueberlebt-das-sortieren-am-ende-des-lesevorgangs-nicht.md`

`einziehen` (`tabelle.rs:328-339`) ruft am Ende `reloadData`, nachdem
`stapel_uebernehmen` das Modell über `abschliessen()` neu sortiert hat
(`tabelle.rs:365`, `modell.rs:80-82`). Die Auswahl steht ausschließlich in der
`NSTableView` als Zeilennummer (`tabelle.rs:273-292`); nach dem Sortieren zeigt
dieselbe Zeilennummer auf einen anderen Eintrag.

Das Fenster für den Fehler ist die Zusage L2 selbst, die die erste Bildschirmseite
"sichtbar **und bedienbar**" verlangt, bevor die Sortierung steht. Gemessen sind
auf dem 100.000er-Prüfordner 35 ms bis zum ersten Stapel und 690 ms bis zum
Abschluss.

Der Kern hält die Lösung bereit und sagt ausdrücklich, wozu:
`Ordnermodell::eintragsindex` und `zeile_von` (`modell.rs:143-156`) sind seit
Schritt 2 da und werden von niemandem gerufen. Der Plan sagt in `### Frage 2`
dasselbe: "die Auswahl des Nutzers bleibt über einen Sortierwechsel hinweg
stabil, weil sie am Eintragsindex hängt und nicht an der Zeilennummer."

Warum jetzt und nicht in S13: S12 bringt zwei Dateifenster mit Tabs, S13 die
Tastaturnavigation, S14 die Bereichsauswahl. Jeder dieser Schritte setzt auf der
Frage auf, wo die Auswahl wohnt.

### Nebenläufigkeit: der Mechanismus, der wirklich trägt

**M1 — Die Generationsprüfung kann nicht greifen und verdeckt den wirksamen
Mechanismus.** *Mittel.*
`issues/260803-1536_o_die-generationspruefung-kann-nicht-greifen-und-verdeckt-den-wirksamen-mechanismus.md`

Die Bedingung `modell.gehoert_dazu(meldung.generation())` (`tabelle.rs:353`) ist
immer wahr. `stapel_uebernehmen` liest nur aus dem Kanal des gerade gehaltenen
Lesevorgangs (`tabelle.rs:345-352`), und `ordner_lesen` setzt Modellgeneration und
Lesevorgangsgeneration in denselben zwei Zeilen auf denselben Wert
(`tabelle.rs:224`, `tabelle.rs:227`). `modell.generation` ändert sich sonst
nirgends. Der `continue`-Zweig ist unerreichbar.

**Die im Auftrag gestellte Frage ist damit beantwortet, und die Antwort ist
gut:** ein Ordnerwechsel während eines laufenden Lesevorgangs beendet den alten
Lauf wirklich, er ignoriert ihn nicht. `tabelle.rs:223` lässt den alten
`Lesevorgang` fallen, `Lesevorgang::drop` setzt das Abbruchkennzeichen
(`leser.rs:154-164`), und spätestens das nächste `send` scheitert am
verschwundenen Empfänger (`leser.rs:242`, `leser.rs:262`). Der Lesefaden prüft das
Kennzeichen vor jedem Systemaufruf **und** zwischen zwei Stapeln
(`leser.rs:208`, `leser.rs:232`), der Abbruch greift also innerhalb von zwei
Stapeln, wie der Modulkopf des Lesers zusagt.

Der Defekt ist, dass Plan und Programmtext einen anderen Mechanismus benennen als
den wirksamen. S12 findet sonst eine Prüfung vor, die aussieht, als wäre sie
erprobt, und die nie einen Stapel verworfen hat.

### Prüfungen

**M2 — Die Prüfungen bestätigen die AppKit-Bitwerte gegen sich selbst.**
*Mittel bis hoch.*
`issues/260803-1536_o_die-pruefungen-bestaetigen-die-appkit-bitwerte-gegen-sich-selbst.md`

Die Frage aus dem Auftrag lautete, ob die Prüfungen die Zusage tragen oder die
Umsetzung nachzeichnen. Beides kommt vor.

*Sie tragen die Zusage* bei vier von acht Prüfungen in `tests/tasten.rs`:
`f3_mit_und_ohne_function_…` prüft C3 unmittelbar,
`ein_pfeil_mit_gesetztem_function_und_zehnerblock_bleibt_ein_nackter_pfeil`
prüft den Fall, für den die Normalisierung existiert,
`eine_gehaltene_zusatztaste_nimmt_der_verdrahteten_taste_ihr_kommando` prüft die
C2-Reservierung, und `cmd_shift_k_behaelt_beide_bits` prüft die Gegenrichtung.

*Sie zeichnen die Umsetzung nach* an zwei Stellen:

- `jedes_der_vier_bits_kommt_einzeln_durch` (`tests/tasten.rs:47-58`) speist
  `roh::BEFEHL` in eine Funktion, die `roh::BEFEHL` liest. Stünde die Konstante
  auf `1 << 21` statt `1 << 20`, bliebe die Prüfung grün und KRK hielte den
  Zehnerblock für die Befehlstaste. Dasselbe gilt für die beiden anderen
  Bitprüfungen.
- `die_fuenf_verdrahteten_tasten_liefern_ihr_kommando` (`tests/tasten.rs:88-101`)
  ist Zeichen für Zeichen die Tabelle `VERDRAHTET` aus `tasten/mod.rs:90-96`, in
  derselben Reihenfolge, über dieselben Konstanten.

Die Gegenprobe für die acht Bits kostet nichts: `objc2-app-kit` führt sie in
`NSEvent.rs:387-406`, und `krk-ui` kennt beide Kisten. Eine Prüfung dort
vergleicht die vorhandene Kopie mit ihrer Quelle, ohne die Architekturgrenze
anzufassen.

Nachgeprüft am 260803-1536: alle acht Werte stimmen heute. Der Befund ist die
fehlende Gegenprobe, nicht ein falscher Wert.

### Fehlerbehandlung

**M3 — Die beiden Fehlermeldungen der Oberfläche erreichen im Bündel niemanden.**
*Mittel bis hoch.*
`issues/260803-1536_o_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md`

Zwei echte Fehlerpfade melden sich allein über `eprintln!`:

- `anwendung.rs:98-104`, der nicht eingerichtete Tastenabgriff. Der Kommentar
  daneben sagt richtig, dass eine Anwendung ohne Tastatursteuerung nicht still
  ausgeliefert werden darf, und wählt dann den einen Kanal, der still ist.
- `tabelle.rs:361-364`, der nicht vollständig lesbare Ordner. Ein Ordner ohne
  Leserecht liefert `Abschluss::Fehler` schon aus `Schwungleser::oeffnen`
  (`leser.rs:201-204`), also ohne einen einzigen Eintrag; der Nutzer sieht eine
  leere Liste und kann sie von einem wirklich leeren Ordner nicht unterscheiden.

Dass Standardausgabe und Standardfehler eines über `open` gestarteten Bündels ins
Leere laufen, hält das Projekt selbst schon fest, in
`issues/260803-1309_o_tastenprotokoll-ueber-open-ist-nicht-lesbar.md`. Jener
Defekt betrifft die Abnahmevorschrift des Protokollmodus und schlägt den Start
aus dem Terminal vor; für den Alltagsbetrieb ist das keine Antwort.

### Sicherheitsbedingungen

**M5 — Zwei SAFETY-Kommentare nennen nicht die Bedingung der Bindung.** *Mittel.*
`issues/260803-1536_o_zwei-safety-kommentare-nennen-nicht-die-bedingung-der-bindung.md`

- `ereignisse.rs:76-78`: `objc2` dokumentiert für
  `addLocalMonitorForEventsMatchingMask_handler` genau eine Bedingung, "block's
  return must be a valid pointer or null"
  (`objc2-app-kit-0.3.2/src/generated/NSEvent.rs:1172-1181`). Der Kommentar
  begründet stattdessen Signatur und Lebensdauer, die beide der Übersetzer
  beziehungsweise `RcBlock` regeln. Die Bedingung ist erfüllt
  (`ereignisse.rs:66-73`), nur nicht genannt.
- `tabelle.rs:606-607`: "leben laenger als die Tabelle" stimmt beim Abbau nicht.
  `Dateifenster` hält `sicht` und `delegierter`; die Tabelle hängt an der Quelle
  und wird zuletzt freigegeben, mitten im `dealloc` von Quelle und Delegiertem.
  Getragen wird die Sicherheit von der nullenden schwachen Eigenschaft, die
  `objc2` an derselben Stelle dokumentiert
  (`NSTableView.rs:402-421`).

Beide Blöcke sind sicher. Der Defekt ist der Beleg, und er zählt, weil diese
sechs Dateien die Vorlage für jeden weiteren AppKit-Aufruf sind: S8, S12, S15,
S16 und S17 schreiben ihre SAFETY-Kommentare nach diesem Muster.

### Zuschnitt

**M6 — Nach Cmd+W bleibt KRK ohne Fenster und ohne Weg zu einem neuen.**
*Mittel.*
`issues/260803-1536_o_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md`

`performClose:` schließt das einzige Fenster (`menue.rs:33-42`), der
Fensterdelegierte bricht nur den Lesevorgang ab (`fenster.rs:52-58`), und der
Anwendungsdelegierte implementiert weder
`applicationShouldTerminateAfterLastWindowClosed:` noch
`applicationShouldHandleReopen:` (`anwendung.rs:53-59`). KRK läuft weiter,
unsichtbar, mit Menüleiste und ohne Fenster. Kein Schritt des Plans nimmt das
auf. Das Abnahmekriterium von S6 ist wörtlich erfüllt; die Sackgasse dahinter
prüft es nicht ab.

---

## Was geprüft und in Ordnung befunden wurde

Die Prüfung reichte weiter als die Befunde. Was hier steht, ist nachgelesen, nicht
angenommen.

### Die Hauptfadenregel hält, und zwar strukturell

- **Kein AppKit-Objekt verlässt den Hauptfaden.** Der einzige Arbeitsfaden im
  Programm ist der Lesefaden aus `Lesevorgang::starten` (`leser.rs:107-110`). Er
  bekommt einen `PathBuf`, ein `Arc<AtomicBool>` und einen `SyncSender<Meldung>`;
  `Meldung` trägt `Vec<Eintrag>` und `io::Error`, also reine Daten. Nichts
  Objective-C-Wertiges wird übergeben.
- **Die Übergabe passiert auf dem Hauptfaden.** Der Kanal wird ausschließlich in
  `stapel_uebernehmen` (`tabelle.rs:344`) geleert, gerufen aus `einziehen`,
  gerufen aus dem `NSTimer`-Rückruf `stapelEinziehen:` (`tabelle.rs:177-180`) an
  einer Klasse mit `#[thread_kind = MainThreadOnly]`. Der Zeitgeber hängt in
  `NSRunLoop::currentRunLoop()` (`tabelle.rs:392`), und "current" ist beim
  einzigen Aufrufweg der Hauptfaden.
- **Jeder Typ, der es voraussetzt, trägt `MainThreadOnly`.** Alle vier
  `define_class!`-Deklarationen tragen es: `Anwendungsdelegierter`
  (`anwendung.rs:45`), `FensterDelegierter` (`fenster.rs:44`),
  `DateifensterQuelle` (`tabelle.rs:170`), `DateifensterDelegierter`
  (`tabelle.rs:431`). `objc2` macht daraus maschinell `!Send + !Sync`
  (`objc2-0.6.4/src/macros/define_class.rs:608`,
  `MainThreadOnlyDoesNotImplSendSync`).
- **Die zwei Typen ohne Marker sind trotzdem fadengebunden.** `Dateifenster`
  (`tabelle.rs:574-577`) hält `Retained<NSScrollView>` und
  `Retained<DateifensterDelegierter>`, beide `!Send`. `Tastenabgriff`
  (`ereignisse.rs:45-49`) hält `Retained<AnyObject>`; `AnyObject` enthält
  `UnsafeCell<PhantomData<(*const …)>>` (`objc2-0.6.4/src/ffi/mod.rs:287`) und ist
  damit `!Send`, und `Retained<T>: Send` verlangt `T: Send + Sync`
  (`retained.rs:829`). Sein `Drop`, der `removeMonitor` ruft, kann also nicht auf
  einen anderen Faden wandern. Das trägt, ist aber nirgends ausgeschrieben.

### Die Eigentumsverhältnisse sind zyklenfrei, mit einer bewussten Ausnahme

```
Anwendungsdelegierter ─┬─> NSWindow ····> FensterDelegierter   (schwach)
                       ├─> FensterDelegierter ─> DateifensterQuelle
                       ├─> Dateifenster ─┬─> NSScrollView ─> NSTableView
                       │                 └─> DateifensterDelegierter
                       │                        └─> DateifensterQuelle ─> NSTableView
                       └─> Tastenabgriff ─> (Block bei AppKit) ─> DateifensterQuelle

NSTableView ····> DateifensterQuelle       (schwach, nullend)
NSTableView ····> DateifensterDelegierter  (schwach, nullend)
NSTimer ────> DateifensterQuelle ────> NSTimer   ← der eine Ring
```

Die drei schwachen Kanten, an denen der Ring sonst zuginge, sind belegt:
`NSTableView.dataSource` und `.delegate` sind nullende schwache Eigenschaften
(`NSTableView.rs:402-421`), `NSWindow.delegate` ebenso (`NSWindow.rs:773-778`),
und `NSApplication` hält seinen Delegierten schwach, weshalb `starten` ihn über
die ganze Laufzeit in einer lokalen Bindung hält (`anwendung.rs:125-130`).

Der einzige echte Ring ist `NSTimer` ↔ `DateifensterQuelle`. Er ist im Kommentar
benannt (`tabelle.rs:157-161`) und wird durch `invalidate` gelöst
(`tabelle.rs:399-403`), gerufen an beiden Enden eines Lesevorgangs: beim
Abschluss (`tabelle.rs:331`) und beim Fensterschließen über `lesen_abbrechen`
(`tabelle.rs:233`, `fenster.rs:56`). Ein Lesevorgang, der weder abschließt noch
abgebrochen wird, existiert nicht: der Leser sendet `Fertig` auf jedem Weg
(`leser.rs:182-187`, `leser.rs:210/218/222`).

Der Fensterdelegierte hält die Quelle, die Quelle hält die Tabelle, die Tabelle
hält keinen von beiden. Die Richtung ist über alle drei Klassen dieselbe und im
Modulkopf begründet (`tabelle.rs:6-9`).

### Kein erreichbarer `RefCell`-Doppelzugriff

Die fünf `RefCell` in `QuelleIvars` sind der Punkt, an dem ein AppKit-Rückschlag
das Programm abstürzen ließe. Ich habe jeden Aufrufweg verfolgt, der von einer
gehaltenen Ausleihe aus in AppKit läuft:

- `stapel_uebernehmen` hält `lesevorgang` **und** `modell` gleichzeitig
  (`tabelle.rs:345-372`), ruft in dieser Spanne aber keinen AppKit-Aufruf. Beide
  Ausleihen enden vor `reloadData` und `noteNumberOfRowsChanged` in `einziehen`.
- `ordner_lesen` (`tabelle.rs:215-229`) hält an keiner Zeile eine Ausleihe über
  `reloadData` hinweg; jede ist ein Ausdruckstemporär, das am Semikolon endet.
- `auswahl_verschieben` (`tabelle.rs:273-292`) liest `zeilenzahl` in einer
  eigenen Anweisung, bevor `selectRowIndexes_byExtendingSelection` und
  `scrollRowToVisible` die Delegiertenrückrufe auslösen.
- `mit_zeile` (`tabelle.rs:322-325`) gibt keine Referenz heraus, und der eine
  Aufrufer, der danach in AppKit geht, holt sich das Feld erst nach dem Rückruf
  (`tabelle.rs:475-479`). Die Begründung dafür steht im S6-Protokoll und ist
  richtig.

Das ist die sorgfältigste Stelle des ganzen Durchstichs.

### Der Inhalt der übrigen `unsafe`-Blöcke

Vierzehn `unsafe`-Stellen, zwölf davon mit einer Begründung, die die dokumentierte
Bedingung trifft. Im Einzelnen belegt: die vier `msg_send![super(this), init]`,
die vier `define_class!`-Köpfe (sie folgen wörtlich der Vorlage aus
`objc2-0.6.4/src/macros/define_class.rs:293-295`, und keine der Klassen
implementiert `Drop`), `NSWindow::initWithContentRect_…` mit dem tragenden
`setReleasedWhenClosed(false)` (`fenster.rs:77-92`),
`NSMenuItem::initWithTitle_action_keyEquivalent` (`menue.rs:77-87`),
`makeViewWithIdentifier_owner` (`tabelle.rs:553-555`), `NSEvent::removeMonitor`
(`ereignisse.rs:88-91`) und `ereignis.as_ref()` (`ereignisse.rs:63-65`). Die zwei
Ausnahmen stehen oben unter M5.

Die Grenze selbst hält: `#![deny(unsafe_code)]` in `main.rs:1` und
`crates/krk-core/src/lib.rs:1`, je genau eine Ausnahme
(`appkit/mod.rs:1`, `verzeichnis/sys.rs`). Der `coder` hat sie nachgemessen, statt
sie zu behaupten.

### Der Weg des Tastendrucks

`NSEvent` → `Tastendruck::aus_ereignis` → `tasten::kommando` →
`DateifensterQuelle::kommando_ausfuehren`. Ein Eintrittspunkt, kein zweiter Weg,
keine eigene `keyDown:`-Behandlung in irgendeiner Ansicht. Der Abgriff kennt weder
Tabelle noch Modell; die Auslegung steht dort, wo beide zu Hause sind. Das ist die
richtige Schnittlinie und deckt sich mit der Änderungsliste von S7.

Die Rückgabe an AppKit ist korrekt: `nil` beim Schlucken, sonst derselbe Zeiger,
den AppKit hereingegeben hat, ohne Besitzerwechsel (`ereignisse.rs:66-73`).
Dass `kommando` nur bei leerer Maske trifft (`tasten/mod.rs:104-112`), hält Cmd+Q
und Cmd+W frei und reserviert Umschalt+Pfeil für C2.

### Der Einzugstakt ist kein Nadelöhr

Die naheliegende Sorge bei einem Kanal der Tiefe 1, der sechzigmal je Sekunde
geleert wird, ist eine Durchsatzgrenze von rund 120 Stapeln je Sekunde. Sie greift
nicht: der `coder` hat 100.000 Einträge in 690 ms vollständig gelesen und
sortiert, das sind rund 98 Stapel in etwa 41 Takten, also 2,4 Stapel je Takt.
Die Zusage L10 nennt 4 s warm. Gemessen ist das im S6-Protokoll, nicht von mir.

### Die Ränder der Auswahlbewegung

`auswahl_verschieben` behandelt alle vier Randfälle richtig: leere Liste über
`checked_sub` (`tabelle.rs:275-277`), fehlende Auswahl über den Vergleich
`jetzt < 0`, Überlauf über `saturating_add`, Rand über `clamp`. `auswahl_oeffnen`
fängt `selectedRow() == -1` über `usize::try_from` ab (`tabelle.rs:301`).
`seitenhoehe` fängt die Tabelle ohne Größe über `.max(1)` ab (`tabelle.rs:265`).
Die Sonde des `coder` zeigt dieselben Werte im Betrieb.

### Bau, Prüfungen, Formatierung

Selbst nachgefahren am 260803-1536: `cargo test --workspace` läuft mit 95
Prüfungen in sieben Gruppen durch, `cargo clippy --workspace --all-targets` meldet
nichts, `cargo fmt --all --check` ist sauber. Die Angaben der beiden Protokolle
stimmen.

### Die beiden Punkte, die der Auftrag als bekannt ausgenommen hat

Das Bild auf dem Bildschirm und der körperliche Tastendruck am signierten Bündel
bleiben ungeprüft, und beim Lesen ist mir kein Grund begegnet, an den
Ersatzprüfungen zu zweifeln. Die Fenstergeometrie über
`CGWindowListCopyWindowInfo` und die Ereignisschlange über
`NSApplication.postEvent:atStart:` sind die richtigen Ersatzwege, und die
Protokolle schreiben aus, was sie nicht zeigen. Eine Einschränkung ist zu
ergänzen, und sie steht schon in H1: die synthetische Sonde setzte `function` und
`numericPad` selbst und kann deshalb nicht belegen, dass AppKit sie bei einer
echten Pfeiltaste setzt.

---

## Geprüft, kein Datensatz

Drei Beobachtungen, die ich absichtlich nicht als Defekt abgelegt habe, mit
Begründung.

**`let _ = ivars.…​.set(…)` dreimal in `anwendung.rs:87-89` und `96`.** Ein
verworfenes `Result` ist normalerweise ein Befund. Hier nicht: `OnceCell::set`
scheitert nur beim zweiten Setzen, und `applicationDidFinishLaunching:` läuft
genau einmal. Das anschließende `if let Some(dateifenster) = …get()`
(`anwendung.rs:91`) ist aus demselben Grund eine Verzweigung über einen
unmöglichen Fall. Stilfrage, kein Defekt.

**Kein Panikschutz an der Objective-C-Grenze.** Ein Rust-Panic in einer
Delegiertenmethode, im Zeitgeberrückruf oder im Ereignisblock bricht den Prozess
ab; `objc2` fängt ihn nur mit dem Merkmal `catch-all`, das nicht gesetzt ist
(`objc2-0.6.4/src/exception.rs:7-9`). Der einzige heute erreichbare Panikpfad ist
`Lesevorgang::starten` mit seinem `.expect` (`leser.rs:110`), gerufen aus dem
Ereignisblock über `auswahl_oeffnen`. Er löst nur bei erschöpften Systemmitteln
aus, und in dieser Lage ist die Anwendung ohnehin verloren. Kein eigener Datensatz;
wenn die Frage später doch eine Regel verdient, gehört sie in den Abschnitt
`## Aufbau` des Plans neben die `unsafe`-Grenze.

**Zwei Zeichenketten-Umwege je Zelle.** `datum_beschriften` und
`groesse_beschriften` (`tabelle.rs:524-544`) holen ein `NSString` von Foundation,
machen daraus ein Rust-`String`, und `zellenansicht` macht daraus wieder ein
`NSString` (`tabelle.rs:480`). Bei rund 120 sichtbaren Zellen je Bild sind das 240
vermeidbare Umwandlungen, geschätzt unter 100 µs je Bild. Die einheitliche
`String`-Rückgabe von `beschriften` ist es wert; ich melde das nicht, sondern
notiere es für den Fall, dass S8 bei L1 knapp wird.

**Der Widerspruch in der Kostentabelle des Plans.** Die Tabelle in
`### Wo die Kosten des Technologieentscheids anfallen` weist S7 eine "Eigene
`NSView` mit `keyDown`" zu, während der Schritt selbst den lokalen Ereignisabgriff
vorschreibt. Der `coder` ist dem Schritt gefolgt, und das war richtig. Die Tabelle
ist veraltet, das ist eine Sache des `planner` und liegt außerhalb des
Prüfgegenstands; ich nenne es hier, damit es beim nächsten Nachzug mitläuft.

---

## Was zusammen betrachtet auffällt

**Sechs von acht Befunden betreffen die Begründung, nicht das Verhalten.** H1, M1,
M4 und M5 sind alle dieselbe Form: ein Text im Programm oder im Plan behauptet
mehr, als der Beleg trägt, oder benennt einen anderen Träger als den wirklichen.
Der Programmtext dieses Durchstichs ist ungewöhnlich dicht kommentiert, und das
ist eine Stärke; die Kehrseite ist, dass ein dichter Kommentar Belege behauptet,
die ein knapper gar nicht erst behaupten würde. Für die kommenden Schritte lohnt
eine Regel: ein Kommentar, der eine Datei oder eine Messung nennt, nennt auch die
Zeile, und was dort nicht steht, steht auch nicht im Kommentar.

**Zwei Befunde folgen aus derselben Auslassung: der Kern hält etwas bereit, das
die Oberfläche nicht ruft.** H2 lässt `eintragsindex` und `zeile_von` liegen, M1
lässt `gehoert_dazu` und `Meldung::generation` unerreicht. Beide sind in Schritt 2
angelegt worden, für einen Bedarf, den Schritt 6 anders gelöst hat. Das ist kein
Vorwurf an Schritt 6, aber es lohnt ein Blick auf `krk-core`, bevor S12 dieselbe
Frage erneut stellt.

**Der Zuschnitt hat einmal geklemmt, und niemand hat es bemerkt.** M6 (Cmd+W ohne
Rückweg) und M3 (der leere Ordner ohne Erklärung) treffen sich in derselben Lücke:
S6 und S7 bauen einen Weg hinein, aber keinen heraus. Das ist im S7-Protokoll für
den Ordnerwechsel ausdrücklich als gewollt vermerkt und für das Fenster gar nicht
betrachtet.

---

## Empfohlene Reihenfolge

**Vor der Frühmessung aus Schritt 8, weil sie das Muster setzen:**

1. H1, die drei Sätze im Modulkopf der Normalisierung. Drei Sätze, kein
   Programmtext, und die Belegkette für C3 ist wieder tragfähig.
2. M5, die zwei SAFETY-Kommentare. Zwei Sätze, und die Vorlage für S8, S12, S15,
   S16 und S17 stimmt.

**Vor Schritt 12, weil sie sonst dreifach anfallen:**

3. H2, die Auswahl am Eintragsindex. Der einzige Befund mit sichtbarer Wirkung.
4. M1, die Generationsprüfung. Entweder belegen oder entfernen, und den Plan
   nachziehen.
5. M2, die Gegenprobe der acht Bitwerte in `krk-ui`.

**Wenn der Nutzer entschieden hat:**

6. M3, wie KRK dem Nutzer Fehler zeigt. Braucht die Statuszeile aus C1.
7. M6, was beim Schließen des letzten Fensters geschieht. Braucht eine Antwort
   des Nutzers.

**Mit Schritt 8 selbst:**

8. M4, der Einzugstakt. S8 misst die Bildwiederholrate ohnehin.

---

## Angelegte Datensätze

| Datei | Gewicht |
|---|---|
| `issues/260803-1536_o_normalisierung-belegt-drei-aussagen-mit-einer-messung-die-sie-nicht-traegt.md` | Hoch |
| `issues/260803-1536_o_auswahl-ueberlebt-das-sortieren-am-ende-des-lesevorgangs-nicht.md` | Hoch |
| `issues/260803-1536_o_zwei-fehlermeldungen-erreichen-im-buendel-niemanden.md` | Mittel bis hoch |
| `issues/260803-1536_o_die-pruefungen-bestaetigen-die-appkit-bitwerte-gegen-sich-selbst.md` | Mittel bis hoch |
| `issues/260803-1536_o_die-generationspruefung-kann-nicht-greifen-und-verdeckt-den-wirksamen-mechanismus.md` | Mittel |
| `issues/260803-1536_o_zwei-safety-kommentare-nennen-nicht-die-bedingung-der-bindung.md` | Mittel |
| `issues/260803-1536_o_nach-cmd-w-bleibt-krk-ohne-fenster-und-ohne-rueckweg.md` | Mittel |
| `issues/260803-1536_o_einzugstakt-begruendet-sich-mit-einer-nicht-erhobenen-bildwiederholrate.md` | Niedrig bis mittel |

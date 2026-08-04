# S14: Dateisystem-Beobachtung und Datenträgerwechsel (C9)

---
**Status:** Complete
**Agent:** coder
**Datum:** 260804-1451
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 14.`
**Spec:** `planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt `### C9`
**Geschlossener Defekt:** `issues/260803-2007_c_s14-bindet-fsevents-ohne-das-framework-coreservices-zu-verlinken.md`

---

## Was gebaut wurde

Ein `FSEventStream` über die höchstens zwei gerade sichtbaren Ordner, mit 300 ms
Sammelverzögerung, bei jeder Navigation neu aufgesetzt. Eine Funktion
`ordner_neu_lesen(pfad)` als einziger Auffrischungspfad. Eine
`NSWorkspace`-Beobachtung für `didMount`, `willUnmount` und `didUnmount`; ein
Dateifenster auf einem ausgeworfenen Datenträger meldet den Verlust und wechselt
auf das Benutzerverzeichnis.

```text
  FSEvents-Rückruf ──┐
                     ├──> auffrischung::ordner_neu_lesen(pfad)
  Abschluss einer  ──┘            │
  Operation (S16)                 └──> Dateifenster::neu_lesen
                                        (der gestückelte Leser aus S2)

  NSWorkspace willUnmount ──> auffrischung::datentraeger_verloren
  NSWorkspace didUnmount  ──┘         │
                                      └──> wechseln + melden

  jede Navigation ──> Dateisystemwache neu aufsetzen
```

## Geänderte und neue Dateien

| Datei | Was daran neu ist |
|---|---|
| `crates/krk-ui/src/auffrischung.rs` | **neu.** Die Entscheidung: welche Dateifenster ein Pfad angeht. Kein AppKit, kein `unsafe`, zehn Prüfungen. |
| `crates/krk-ui/src/appkit/fsevents.rs` | **neu.** Die Bindung an FSEvents samt `#[link]`-Attribut und der Halter `Dateisystemwache`. |
| `crates/krk-ui/src/appkit/volumes.rs` | **neu.** Die `NSWorkspace`-Beobachtung und der Halter `Datentraegerwache`. |
| `crates/krk-ui/src/appkit/anwendung.rs` | Beide Wachen als Halter, die Umsetzung von `Dateifenstersicht`, das Neuaufsetzen des Stroms. |
| `crates/krk-ui/src/appkit/tabelle.rs` | `neu_lesen`, der Rückruf `ordnerwechsel`, die Trennung von Fenster- und Tabmeldung. |
| `crates/krk-ui/src/tabs.rs` | `Tabliste::aktiven_neu_lesen`. **Nicht in der Dateiliste des Plans**, siehe unten. |
| `crates/krk-ui/src/main.rs`, `appkit/mod.rs` | `mod auffrischung;`, `mod fsevents;`, `mod volumes;` und die Modulköpfe. |
| `Cargo.toml`, `crates/krk-ui/Cargo.toml`, `Cargo.lock` | `objc2-core-foundation` mit den drei Merkmalen `CFArray`, `CFString`, `CFRunLoop`. |

## Die CoreServices-Verlinkung: Weg 1 reicht, und die Praemisse trägt nur halb

Der Nutzer hat Weg 1 entschieden, das `#[link(name = "CoreServices", kind =
"framework")]` am `extern`-Block. So ist es gebaut, und ein Bauskript ist nicht
entstanden.

Nachgeprüft wurde beides, die Notwendigkeit und die Wirkung:

- **Die Prämisse des Defekts stimmt.** Ein eigens gebautes Probeprogramm, das
  allein diesen `extern`-Block enthält und sonst nichts verlinkt, band ohne das
  Attribut nicht: `Undefined symbols for architecture x86_64:
  "_FSEventStreamCreate"`. Das Probeprogramm ist danach entfernt worden.
- **KRK bindet trotzdem auch ohne das Attribut.** `cargo clean -p krk-ui &&
  cargo build` ohne die Zeile ging durch, und `otool -L target/debug/krk`
  führte `CoreServices` auf. Der Weg ist eine Reexport-Kette: `AppKit`
  reexportiert `ApplicationServices`, und das reexportiert `CoreServices`
  (beides im Abschnitt `reexported-libraries` der jeweiligen `.tbd`).
  `xcrun dyld_info -fixups` zeigt die Bindung als
  `CoreServices/_FSEventStreamCreate`; auf der Kommandozeile des Binders steht
  `-framework CoreServices` nicht.

Das Attribut bleibt trotzdem stehen, und der Modulkopf von `fsevents.rs`
schreibt den Grund aus: ohne die Zeile hängt die Auflösung an einer Zusage, die
AppKit gibt und nicht KRK. Sie fiele still weg, wenn Apple die Kette ändert.

## Was untergehen konnte, und was daraus wurde

**Genau eine Definition von `ordner_neu_lesen`.** `grep -rn ordner_neu_lesen
crates` findet eine Definition
(`crates/krk-ui/src/auffrischung.rs:105`) und heute genau eine Aufrufstelle
(`crates/krk-ui/src/appkit/anwendung.rs:433`, der FSEvents-Rückruf). Der zweite
Auslöser aus S16 kommt an dieselbe Funktion. `krk-core` ruft sie nicht.

**Der gestückelte Lesevorgang aus S2 wird wiederverwendet.**
`Tabliste::aktiven_neu_lesen` geht durch `lesen_starten`, dieselbe private
Funktion, die jede Navigation nimmt, samt Generationszähler und Kanal. Ein
zweiter Leseweg ist nicht entstanden.

**Auswahl und Bildlaufposition überleben.** Beides über die zwei Felder, die die
Sitzungswiederherstellung schon benutzt: `wunschauswahl` trägt den **Namen** des
ausgewählten Eintrags (nicht seine Zeile, denn eine Auffrischung findet gerade
dann statt, wenn sich der Inhalt geändert hat), und `bildlauf_offen` sagt der
Ansicht, dass sie die gemerkte Position noch herstellen muss. Der Rückweg in die
Tabelle läuft über `auswahl_anzeigen` und `gemerkten_bildlauf_herstellen`, also
über die Hülle aus S12.

**Der Halter.** `Dateisystemwache` und `Datentraegerwache` hängen am
Anwendungsdelegierten und melden sich in ihrem `Drop` ab, dieselbe Form wie der
Tastenabgriff aus S7 und das `Zeichenende` aus S8. Die Dateisystemwache liegt in
einer `RefCell`, weil ein `FSEventStream` seine Pfadliste nach dem Anlegen nicht
mehr ändert und deshalb bei jeder Navigation ein neuer entsteht.

## Vier Entscheidungen, die der Plan offen ließ

**Der Pfadvergleich löst Verknüpfungen auf.** FSEvents meldet den aufgelösten
Pfad: eine Änderung unter `/tmp` kommt als `/private/tmp/…` zurück. Ein reiner
Zeichenvergleich ließe jede Auffrischung unter einem verknüpften Pfad still
ausfallen. `auffrischung::gleicher_ordner` vergleicht deshalb erst die
geschriebene Form und danach die über `canonicalize` aufgelöste. Das sind zwei
Vergleiche einer Frage und keine zwei Regeln.

**Was der Rückruf nicht auslöst.** Eine Auffrischung meldet ausdrücklich keinen
Ordnerwechsel. Zwei Gründe: der Ordner ist derselbe, und der Rückruf läuft im
Aufruf des Stroms, den er sonst mitten darin freigäbe.

**Der Messmodus bekommt keine Beobachtung.** Weder Strom noch
Datenträgerbeobachtung werden eingerichtet, wenn `--messmodus` läuft. Dieselbe
Haltung wie bei der Sitzung, die ein Messlauf weder lädt noch schreibt: eine
Beobachtung auf dem Prüfordner brächte Arbeit in die Messung, die im Betrieb an
anderer Stelle anfiele.

**`didMount` hat in dieser Runde keinen Abnehmer.** Die Meldung wird beobachtet,
wie `### Frage 3` es vorschreibt, und der Zweig im Anwendungsdelegierten ist
leer mit einem Verweis auf S18, wo die Geräteleiste aus C5 sie verbraucht.

## Ein Befund aus der Abnahme, der zu einer Änderung führte

Der erste Auswurf-Durchgang zeigte die Meldung und **verlor sie danach wieder**.
Der Grund: nach dem Wechsel auf das Benutzerverzeichnis beobachtet der Strom
`/Users/k1`, und FSEvents beobachtet rekursiv. Jede beliebige Änderung
irgendwo im Heimatverzeichnis löste eine Auffrischung aus, und die schrieb über
`meldung_anzeigen` die (leere) Meldung des Tabs über die Auswurfmeldung des
Fensters.

Die Statuszeile hatte damit zwei Schreiber ohne Regel, wer gewinnt. Sie hat
jetzt eine: `QuelleIvars::fenstermeldung` hält die Meldung, die dem Fenster
gehört, und `meldung_anzeigen` zeigt sie vor der Meldung des Tabs. Gelöscht wird
sie von jedem echten Ordner- und Tabwechsel und ausdrücklich nicht von einer
Auffrischung. Die Begründung für die Reihenfolge steht am Code: eine
Fenstermeldung beschreibt ein Ereignis, eine Tabmeldung einen Zustand, und das
Ereignis ist das Neuere.

Nach der Änderung übersteht die Auswurfmeldung eine im Heimatverzeichnis
angelegte Datei; belegt weiter unten.

## Abnahme

### Die vier üblichen Kommandos

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 248 Prüfungen (vorher 237) |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0, keine Warnung |

Elf neue Prüfungen: zehn in `crates/krk-ui/src/auffrischung.rs` (welches
Dateifenster ein Pfad angeht, Schlussstrich, verknüpfter Pfad, doppelter
Ordner, ausgeblendetes Dateifenster, Datenträger samt Namensvetter, Reihenfolge
von Wechsel und Meldung) und eine in `crates/krk-ui/src/tabs.rs` (Ordner,
Auswahlname und Bildlauf überleben `aktiven_neu_lesen`).

### Die drei Punkte des Abnahmekriteriums

**1. Eine mit `touch` angelegte Datei erscheint innerhalb von 1 s, eine mit `rm`
entfernte verschwindet ebenso.** Belegt am laufenden Bündel. Prüfordner
`~/krk-c9-links` (alpha.txt, beta.txt, gamma.txt) im linken und
`~/krk-c9-rechts` (eins.txt, zwei.txt) im rechten Dateifenster.

| Vorgang | Aufnahme nach | Ergebnis |
|---|---|---|
| `touch delta-neu.txt` | 1,02 s | vier Zeilen, `delta-neu.txt` an der sortierten Stelle |
| `touch epsilon-neu.txt` | 0,5 s | fünf Zeilen, schon da |
| `rm delta-neu.txt epsilon-neu.txt` | 1,0 s | wieder drei Zeilen |
| `touch drei-neu.txt` im **rechten** Ordner | 1,0 s | drei Zeilen rechts, beide Ordner werden beobachtet |

Kein Tastendruck und kein Mausklick dazwischen. Die halbe Sekunde ist der
schnellste gemessene Wert und passt zur Sammelverzögerung von 300 ms plus einem
Lesevorgang über drei Einträge.

**Auswahl überlebt, und zwar am Eintrag.** Mit `auswahl = "gamma.txt"` in der
Sitzung gestartet, `gamma.txt` also markiert und in Zeile 3. Dann
`touch delta2.txt`, das sich davor einsortiert. Nach der Auffrischung steht die
Markierung weiterhin auf `gamma.txt`, jetzt in Zeile 4. Die Auswahl hängt am
Eintrag und nicht an der Zeilennummer.

**2. Das Auswerfen führt zu einer Meldung und zum Wechsel auf das
Benutzerverzeichnis.** Belegt am laufenden Bündel. Das rechte Dateifenster stand
auf `/Volumes/KRKPruef/Fotos`. Nach `hdiutil detach` meldete `hdiutil` `"disk2"
ejected`, das Dateifenster zeigte `k1` (das Benutzerverzeichnis), und die
Statuszeile an seinem Fuß stand rot:

> KRKPruef wurde ausgeworfen; das Dateifenster zeigt jetzt /Users/k1

Aufgenommen 0,4 s und 2,4 s nach dem Auswurf, beide Male gleich. Im letzten
Durchgang zusätzlich eine Datei im Heimatverzeichnis angelegt und 2 s gewartet:
die Meldung stand danach unverändert. Dass der Auswurf überhaupt durchging,
belegt zugleich die Zusage "statt zu blockieren": KRK hatte den Ordner
freigegeben, sonst hätte `hdiutil detach` sich beschwert.

**Der Prüfdatenträger.** `hdiutil create -size 20m -fs APFS -volname KRKPruef
/tmp/krk-c9-pruef.dmg`, dann `hdiutil attach`. Darauf ein Ordner `Fotos` mit
zwei leeren Dateien. Ausgeworfen mit `hdiutil detach /Volumes/KRKPruef`, danach
`rm /tmp/krk-c9-pruef.dmg`. Kein fremder Datenträger wurde angefasst.

**3. Der Diff zeigt genau eine Definition von `ordner_neu_lesen`.** Belegt:
`grep -rn ordner_neu_lesen crates` liefert eine Zeile mit `pub fn`
(`auffrischung.rs:105`), eine Aufrufstelle (`anwendung.rs:433`), sechs
Verwendungen in den Prüfungen derselben Datei und drei Erwähnungen in
Kommentaren. **Ein Hinweis zur Prüfform:** `git diff` allein zeigt sie nicht,
weil `auffrischung.rs` eine neue und damit unverfolgte Datei ist. Wer am Diff
prüft, braucht `git add -N` davor oder nimmt den `grep` über den Baum.

### Die Grenzen

`grep -rn 'objc2\|unsafe' crates/krk-ui/src/auffrischung.rs` findet nichts;
die Datei liegt außerhalb von `appkit/` und hält das ein.
`grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src
crates/krk-core/src` nennt unverändert genau zwei Dateien,
`krk-ui/src/appkit/mod.rs` und `krk-core/src/verzeichnis/sys.rs`. Kein Eingriff
in `crates/krk-core/`, `crates/krk-bench/`, `xtask/`, `resources/`, den Plan
oder den Spec.

### Die vorübergehenden Hilfsmittel, und dass sie weg sind

Zwei Dinge liefen außerhalb des Projekts mit, keines davon im Quellbaum von KRK:

1. Ein Rust-Probeprogramm unter `/tmp/linkprobe`, das allein den
   `extern`-Block ohne `#[link]` enthielt, für den Nachweis oben. Entfernt.
2. Zwei kleine C-Programme unter `/tmp/krk-winid`: eines liest die Fenster-ID
   von KRK über `CGWindowListCopyWindowInfo`, das andere schneidet einen
   Ausschnitt aus einem PNG. Sie waren nötig, weil ein anderes Fenster den Fuß
   des KRK-Fensters verdeckte und `screencapture -l <id>` das Fenster samt
   Statuszeile trotzdem aufnimmt. Entfernt.

Im Quellbaum von KRK gab es keine Sonde. Die Prüfordner `~/krk-c9-links` und
`~/krk-c9-rechts`, das Abbild `/tmp/krk-c9-pruef.dmg` und alle Bildschirmfotos
sind entfernt, die Sitzungsdatei des Nutzers ist aus der Sicherung
wiederhergestellt.

## Was nicht geprüft ist

- **Netzlaufwerke.** FSEvents deckt sie nicht ab; ohne Server nicht messbar.
  `issues/260804-1451_o_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf.md`.
- **Das abgezogene Medium.** Geprüft ist der geordnete Auswurf, der
  `willUnmount` schickt. Der Fall, in dem allein `didUnmount` kommt, ist gebaut
  und im Code begründet, aber nicht ausgelöst worden: dafür hätte ein
  körperlicher Datenträger abgezogen werden müssen.
- **Der Ordner mit 100.000 Einträgen unter Auffrischung.** Die Zusage, dass eine
  Auffrischung die Eingabe nicht blockiert, folgt daraus, dass sie durch
  denselben `lesen_starten` läuft wie jede Navigation; gemessen wurde sie in
  dieser Größe nicht.

## Neu abgelegte Defekte

| Datei | Worum es geht |
|---|---|
| `issues/260804-1451_o_die-dateiliste-von-s14-nennt-tabs-rs-nicht-obwohl-der-bildlauf-dort-wohnt.md` | Die Dateiliste des Plans überschritten; ohne `tabs.rs` ist die Bildlaufzusage nicht einzulösen. |
| `issues/260804-1451_o_fseventstreamschedulewithrunloop-ist-seit-macos-13-als-veraltet-gekennzeichnet.md` | Der Plan meint die Laufschleifen-Form; Apple führt sie seit macOS 13 als abgelöst. |
| `issues/260804-1451_o_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf.md` | C9 schließt Netzlaufwerke ein, FSEvents deckt sie nicht ab. |
| `issues/260804-1451_o_ein-verdeckter-tab-auf-einem-ausgeworfenen-datentraeger-behaelt-seinen-toten-pfad.md` | Der Auswurf holt nur den sichtbaren Tab herunter. |

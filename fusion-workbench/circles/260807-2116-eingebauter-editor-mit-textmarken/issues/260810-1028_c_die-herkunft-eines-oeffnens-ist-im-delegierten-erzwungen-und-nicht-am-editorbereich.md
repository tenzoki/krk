# Die Herkunft eines Öffnens ist im Delegierten erzwungen und nicht am Editorbereich

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Behebung von `260810-0418`
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::datei_oeffnen:968`, `Ausgangsmelder:662`), `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::editor_oeffnen_lassen`)
**Cross-references:** `issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`, C2, C7

---

## Der Befund

Der Defekt `260810-0418` ist behoben, und zwar mit dem Weg, den seine Frage
„was zu prüfen wäre" als den besseren beschreibt, aber nur bis zur Grenze des
Delegierten. `Anwendungsdelegierter::editor_oeffnen_lassen` ist jetzt die eine
Stelle, an der der Delegierte den Editor eine Datei aufnehmen lässt, und sie
nimmt die Herkunft (`Oeffnungsherkunft::Befehl` oder `::Sitzung`) als
**Pflichtargument**. Ein vierter Öffnungsweg im Delegierten, der seine Herkunft
nicht nennt, übersetzt nicht.

**Was der Übersetzer weiter nicht sieht:** einen Aufruf von
`Editorbereich::datei_oeffnen`, der an dieser Stelle vorbeigeht. Heute gibt es
keinen — die vier Wege des Delegierten sind alle, die es gibt —, aber die Zusage
hängt an dieser Tatsache und nicht an einer Prüfung.

## Der Weg, der es erzwingen würde

Die Herkunft wandert in die Kette, statt neben ihr zu liegen:

1. `Editorbereich::datei_oeffnen(&self, pfad: &Path, herkunft: Oeffnungsherkunft)`
   — dann kann kein Aufrufer sie weglassen, wo auch immer er steht.
2. Der Wert wandert mit dem Ladevorgang durch `Editormodell::oeffnen` und kommt
   mit dem Ladeausgang zurück, oder — ohne das Modell anzufassen — der
   `Editorbereich` merkt sie sich zum laufenden Ladevorgang und gibt sie dem
   `Ausgangsmelder` als zweites Argument mit.

Der zweite Zuschnitt ist der kleinere und verletzt den Schnitt zwischen
`editormodell` und `appkit` nicht: das Modell müsste dann nichts von Befehlen
wissen. Er verlangt allerdings eine Änderung an `Ausgangsmelder` und damit an
jeder Stelle, die ihn einträgt.

## Warum es hier nicht gebaut wurde

Die Dateigrenze der Sitzung, in der `260810-0418` behoben wurde, lief um
`editormodell.rs` und `anwendung.rs`; an `appkit/editor.rs` arbeiteten parallel
andere. Der gebaute Weg löst den gemeldeten Fehler vollständig — ein F4 während
der Wiederherstellung erbt die Marke nicht mehr —; offen ist allein die
Erzwingung über die Kistengrenze hinweg.

## Was heute hält

Alles, was `260810-0418` benannte. Die Marke bezeichnet jetzt das **zuletzt
begonnene** Öffnen, und höchstens dieses liefert einen Ladeausgang, weil
`Editormodell::oeffnen` den laufenden Ladevorgang ersetzt und der Empfänger des
überholten Fadens fällt. Die eine Ausnahme davon führt der Datensatz
`260810-1029` daneben.

---

## Warum er am 260810-1139 offen geblieben ist

**Der Weg, den dieser Datensatz nennt, ist der richtige, und er passt in keine
Datei.** Sein zweiter Zuschnitt — die Herkunft an `Editorbereich::datei_oeffnen`,
zurück über den `Ausgangsmelder` — verlangt drei Änderungen, und zwei von ihnen
liegen in `crates/krk-ui/src/appkit/anwendung.rs`:

1. `Editorbereich::datei_oeffnen` nimmt die Herkunft als Pflichtargument.
   (`appkit/editor.rs`)
2. `Ausgangsmelder` wechselt von `Box<dyn Fn(Ladeausgang)>` auf
   `Box<dyn Fn(Ladeausgang, Oeffnungsherkunft)>`, und der Rückruf, den
   `Anwendungsdelegierter` einträgt, muss die zweite Angabe annehmen.
   (`appkit/anwendung.rs`)
3. `Oeffnungsherkunft` ist heute ein privates `enum` in `anwendung.rs`
   (Zeile 268) und müsste mindestens `pub(super)` werden oder nach `editor.rs`
   umziehen — samt `editor_oeffnen_lassen`, das dann die Herkunft weiterreicht
   statt sie in `editor_aus_sitzung` abzulegen. (`appkit/anwendung.rs`)

**Eine halbe Fassung wäre schlechter als keine.** Nur Punkt 1 zu bauen bricht den
Bau, weil `anwendung.rs` weiter mit einem Argument ruft; ein zweiter Eingang
`datei_oeffnen_mit` daneben erzwingt gar nichts und wäre genau der Kommentar, den
dieser Datensatz nicht will.

Die Dateigrenze dieser Sitzung lief um `appkit/editor.rs`, `hervorhebung.rs` und
`krk-ui/Cargo.toml`; an `anwendung.rs` arbeiteten parallel andere. Der Datensatz
bleibt deshalb unverändert offen, und die Aufstellung oben ist alles, was ihm noch
fehlt: keine Frage mehr, sondern drei benannte Änderungen in zwei Dateien.

**Nachgetragen:** wer ihn baut, hat einen vierten Punkt. `Editorbereich` muss die
Herkunft zum **laufenden** Ladevorgang merken, nicht zur letzten Anfrage, und
`Editormodell::oeffnen` ersetzt den laufenden Vorgang bei einer neuen Anfrage. Ein
`Cell<Oeffnungsherkunft>` am `Editorbereich` wäre damit dieselbe Bauart, die
`260810-0418` als Fehler geführt hat, nur eine Ebene tiefer; die Herkunft gehört
neben den Ladevorgang und nicht neben den Bereich.

---
Resolved: Alle vier Punkte gebaut, am 260810-1207. Die `Oeffnungsherkunft` ist von
`anwendung.rs` nach `crates/krk-ui/src/appkit/editor.rs` umgezogen und dort `pub`;
`Editorbereich::datei_oeffnen(&self, pfad: &Path, herkunft: Oeffnungsherkunft)`
nimmt sie als Pflichtargument, und `Ausgangsmelder` ist jetzt
`Box<dyn Fn(Ladeausgang, Oeffnungsherkunft)>`. Damit hält der Übersetzer den
vergessenen Fall an, und zwar von jeder Stelle des Programms aus: ein Aufruf von
`datei_oeffnen` ohne Herkunft übersetzt nicht, gleich wer ihn schreibt. Das Feld
`AnwendungsIvars::editor_aus_sitzung` ist samt seinem Doc-Kommentar **entfallen**;
`editorausgang_behandeln(&self, ausgang, herkunft)` bekommt die Angabe als
Argument, und `editor_oeffnen_lassen` bündelt nur noch, statt zu erzwingen.

**Punkt 4 ist beantwortet und nicht umgangen.** Die Herkunft liegt als
`Cell<Oeffnungsherkunft>` in `EditorIvars` und ist trotzdem nicht die Bauart aus
`260810-0418`: dort schrieb sie **ein** Aufrufer und jeder Ausgang las sie, hier
schreibt sie **jedes** `datei_oeffnen` und jedes `melden` liest sie. Sie bezeichnet
das zuletzt begonnene Öffnen, und höchstens dieses liefert einen Ladeausgang, weil
`Editormodell::oeffnen` den laufenden Vorgang ersetzt und die Abkürzung
`SchonOffen` ihn seit `260810-1029` aufgibt.

Die zurückgehaltene Datei aus C4 braucht damit **keine zweite Angabe daneben**, und
das ist am Code geprüft und nicht angenommen: solange ein Blatt am Fenster hängt,
weist `Anwendungsdelegierter::kommando_ausfuehren` (`anwendung.rs:2035`) jedes
Kommando ausser dem Abbruch ab (`kommandos::operationen::waehrend_blatt_erlaubt`),
also kann kein weiterer Öffnungsbefehl zwischen die Rückhaltung und die Antwort
treten. Der Nachbardatensatz `260810-1102`, der genau das Gegenteil annahm, ist
damit widerlegt.

Der zweite Zuschnitt des Datensatzes ist gebaut, der erste nicht: `krk-core` und
`editormodell.rs` sind nicht angefasst, der Schnitt zwischen Modell und `appkit`
steht unverändert. Zusätzlich trägt `Oeffnungsherkunft::ist_aus_sitzung` die eine
Fallunterscheidung über die beiden Werte, vollständig und ohne Auffangzweig; ein
`==` an der Aufrufstelle hätte einen dritten Anlass still als `Befehl` behandelt.

Abnahme: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets`, `cargo fmt --all --check` — jedes exit 0.

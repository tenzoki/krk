# S13: Tastaturnavigation vollständig (C2), dazu der Sprung aus der Zwischenablage (C10)

**Agent:** coder
**Datum:** 260804-1309
**Status:** Complete

---

## Auftrag

Zwei Teile. Erst zwei Testreparaturen, die der Belegungswechsel aus S11c überholt hatte und von denen eine den Lauf umwarf. Dann Schritt 13 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`: die acht Abnahmekriterien aus C2 als Kommandos hinter der Belegungsmaschine, dazu die zweite Funktion aus C10, der Sprung zu dem, was in der Zwischenablage steht.

---

## Teil 1: die beiden Prüfungen

`crates/krk-core/tests/belegung.rs:347` band `return` noch an das Öffnen. S11c hat `oeffnen` auf `cmd+right` umbelegt, und die Prüfung fiel damit um (`25 passed; 1 failed`). Das Beispiel steht jetzt auf `("cmd+right", Kommando::Oeffnen)`; die Zusage der Prüfung bleibt unverändert.

`die_drei_ab_werk_freien_kombinationen_kommen_nicht_vor` deckte drei Kombinationen ab, seit S11c sind es vier. Die Zahl im Namen war dabei die eigentliche Falle, dieselbe wie neulich bei den Zählprüfungen: sie bindet die Prüfung an die Größe der Liste statt an ihre Zusage und verlangt bei jedem Zuwachs eine Umbenennung. Die Prüfung heißt jetzt `die_ab_werk_freien_kombinationen_kommen_nicht_vor`, ohne Zahl, und ihr Kommentar nennt zuerst die Zusage und danach die vier Kombinationen mit je ihrem Grund.

`ctrl+b` und `ctrl+s` sind mit S11c ebenfalls unbelegt geworden und bleiben trotzdem draußen, aus zwei Gründen. Der erste steht schon im Defekt: die Liste führt Kombinationen, die ein Leser belegt erwartete und die ausdrücklich frei bleiben; `ctrl+b` und `ctrl+s` waren eine Behelfsbelegung, deren Grund weggefallen ist. Der zweite kommt dazu: die Liste sagt zu, dass eine Kombination frei **bleibt**. Für `ctrl+s` wäre das eine Zusage gegen den Editor späterer Runden, wo es die vertraute Sicherntaste ist. Beide Gründe stehen als Kommentar in der Prüfung.

Beide Defekte sind geschlossen (`_c_`), `cargo test --workspace` war danach grün.

---

## Teil 2: was gebaut wurde

### Der Kern

```text
verzeichnis/
  sprungmarke.rs   neu   Puffer der Anfangsbuchstaben, Pause, Zeichenregel
  modell.rs        +     Markierung je Eintrag, Suche nach Namen
  mod.rs           +     aufwaerts(): Elternordner und verlassener Name
zwischenablage.rs  neu   deuten(): Pfad, Web-Adresse oder nichts Verwertbares
tasten/belegung.rs +     15 neue Kommandos, KENNUNGEN von 16 auf 31
```

**Die Sprungmarke nimmt nur auf, was ein Dateiname tragen kann.** `traegt_ein_dateiname` weist zwei Klassen ab: Steuerzeichen und den Bereich `U+F700` bis `U+F8FF`, in dem AppKit die Pfeile und die Funktionstasten meldet. Ein abgewiesenes Zeichen lässt den Puffer unverändert **und startet die Pause nicht neu**; erst beides zusammen ist die Zusage, dass eine begonnene Suche einen Tastendruck übersteht, der keine Suche sein kann. Die Regel ist keine Sonderregel für die seit S11c freie Eingabetaste: sie deckt jede unbelegte Funktionstaste ab.

**Die Markierung wohnt im Ordnermodell**, als Wahrheitswert je Eintrag und parallel zu `eintraege`, nicht zur Sichtreihenfolge. Sie hängt damit am Eintragsindex und übersteht jedes Umsortieren und jedes Ein- und Ausblenden der versteckten Einträge, aus demselben Grund wie die Auswahl. "Alle markieren" und "umkehren" erfassen die **sichtbaren** Einträge, "aufheben" alle: eine Markierung, die der Nutzer beim Drücken der Taste nicht vor sich hatte, soll keine spätere Dateioperation treffen, eine stehengebliebene unsichtbare aber auch nicht.

**Der Aufstieg steht im Kern und nicht in der Oberfläche.** `verzeichnis::aufwaerts` ist reine Pfadarithmetik und liefert den übergeordneten Ordner samt dem Namen des verlassenen. Der Plan nannte für die Aufstiegsauswahl `cargo test -p krk-core --test navigation` als Abnahme; das geht nur, wenn die Rechnung im Kern liegt. `krk-ui` hängt allein die Navigation daran.

### Die Oberfläche

```text
kommandos/                neu   reines Rust, keine objc2-Zeile
  navigation.rs                 Bewegung und zielzeile: Zeile, Seite, Anfang, Ende
  auswahl.rs                    markieren_und_weiter
  pfadeingabe.rs                Ergebnis und pruefen: die eine Pfadprüfung
appkit/
  blaetter/mod.rs         neu   die gemeinsame Blatthülle, Eingabewächter
  blaetter/pfadeingabe.rs neu   das Eingabeblatt
  zwischenablage.rs       neu   NSPasteboard lesen, NSWorkspace öffnen
  ereignisse.rs           +     Fokusvorbehalt, Eingabe::Zeichen
  tabelle.rs              +     15 Kommandos, Sprungmarke, Markierungsfarbe
  anwendung.rs            +     eingabe_ausfuehren teilt Kommando und Zeichen auf
tabs.rs                   +     ordner_setzen nimmt einen Auswahlnamen entgegen
```

**Der Fokusvorbehalt sitzt im Abgriff.** `ersthelfer_nimmt_text` fragt vor dem Nachschlag, ob der Ersthelfer des **Schlüsselfensters** ein `NSTextView`, ein `NSTextField` oder ein `NSText` ist, und reicht den Tastendruck dann unverändert weiter. Gefragt ist das Schlüsselfenster und nicht das Hauptfenster: steht ein Blatt, ist dessen Panel das Schlüsselfenster, und dort sitzt das Feld. Die drei Klassen zusammen decken beide Zustände eines Textfeldes ab, das Feld selbst und den Feldeditor, an den es seinen Ersthelferrang beim Bearbeiten abgibt. Der Vorbehalt steht einmal im Abgriff und nicht je Blatt; die fünf Blätter aus S16 und S17 erben ihn.

**Ein Navigationsweg, zwei Auslöser.** `kommandos::pfadeingabe::pruefen` ist die eine Stelle, die einen Pfad prüft, und `DateifensterQuelle::pfad_anspringen` die eine, die das Ergebnis anwendet. Die Pfadeingabe von Hand ruft sie über den Rückruf des Blattes, der Sprung aus der Zwischenablage über `Ziel::Pfad`. Der Diff zeigt genau eine Definition von `pruefen`, genau eine von `pfad_anspringen` und genau zwei Aufrufstellen der zweiten.

**Die Schemata-Grenze.** `deuten` kennt drei Ausgänge und keinen Rückfallweg. `file:` zählt als Pfad, weil es dasselbe benennt und nur anders geschrieben ist; `http:` und `https:` gehen an den Systembrowser; alles andere ist nicht verwertbar, und ein `smb:` oder `ftp:` fällt deshalb ohne eigenen Zweig heraus. Der Grund ist C9: ein weitergereichtes `smb:` baute über einen Umweg die Serververbindung auf, die C9 ausschließt.

**Zwei Sorten Zwischenablage, eine Rangfolge.** `appkit::zwischenablage::lesen` fragt `NSPasteboardTypeFileURL` und danach `NSPasteboardTypeString`; die erste Sorte mit nichtleerem Inhalt gewinnt. Die Rangfolge sitzt in der Oberfläche, weil allein sie das Pasteboard kennt; die Deutung dahinter braucht keinen zweiten Zweig, weil ein Dateiverweis als `file:`-Zeichenkette ankommt. Der Entscheidungsdatensatz `decisions/260804-0830_i_was-die-zwischenablage-auswertung-liest.md` steht damit auf umgesetzt.

### Eine Zutat, die der Plan nicht vorhersah: der Eingabewächter

Ein `NSTextField` im Bearbeitungszustand verbraucht die Eingabe- und die Escape-Taste selbst: sein Feldeditor macht daraus `insertNewline:` und `cancelOperation:` und beendet damit nur die Bearbeitung. Die Schaltflächen des Blattes sehen die beiden Tasten nie. **Am laufenden Bündel gemessen:** ohne Gegenmaßnahme ließ sich das Blatt weder mit der Eingabe- noch mit der Escape-Taste schließen, und die Pfadeingabe wäre allein mit der Maus bedienbar gewesen, gegen C2.

`Eingabewaechter` in `appkit/blaetter/mod.rs` ist der Delegierte des Feldes und fängt genau diese zwei Befehle ab. Alles übrige, darunter jede Bewegung der Schreibmarke, bleibt beim Feldeditor. Dazu setzt `Blatt::neu` die beiden Tastenentsprechungen der Schaltflächen ausdrücklich: `NSAlert` gibt die Escape-Taste von sich aus allein einer Schaltfläche mit dem Titel "Cancel", und den trägt eine deutschsprachige Anwendung nicht.

---

## Abnahme

### Die vier üblichen Kommandos

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 237 Prüfungen |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |

Dazu `cargo test -p krk-core --test navigation` (12 Prüfungen) und `cargo test -p krk-core --test zwischenablage` (9 Prüfungen), beide 0. Der Deutungstest deckt die sieben Fälle ab, die der Plan einzeln nennt: Ordnerpfad, Dateipfad, `file:`-Verweis, `https:`-Adresse, `smb:`-Adresse, relativer Pfad, leere Zeichenkette.

### Die Grenzen

`grep -rn 'objc2\|AppKit' crates/krk-core/src crates/krk-ui/src/kommandos` findet nur Fließtext in Kommentaren und keine Kiste. `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src crates/krk-core/src` nennt unverändert genau zwei Dateien, `krk-ui/src/appkit/mod.rs` und `krk-core/src/verzeichnis/sys.rs`. `resources/default-keymap.toml` ist unverändert.

### Die vorübergehende Sonde

Die Tastatursteuerung von außen war nicht verfügbar: `osascript` hat auf diesem Rechner keinen Zugriff auf die Bedienungshilfen, und der Versuch löste eine Rückfrage aus, die niemand beantworten konnte. Für den Nachweis im laufenden Bündel lief deshalb eine **vorübergehende Sonde** mit, nach demselben Muster, das `ereignisse::pfeil_ab_senden` für die Messung von L1 schon kennt: sie stellt synthetische Tastenereignisse in die Ereignisschlange, die denselben Weg nehmen wie ein körperlicher Tastendruck (Schlange, lokaler Abgriff, Normalisierung, Belegung, Kommando), und schreibt nach jedem Schritt den Zustand des aktiven Dateifensters auf die Standardausgabe.

**Die Sonde ist vollständig zurückgenommen.** Entfernt sind `crates/krk-ui/src/sonde.rs`, die Zeile `mod sonde;`, `ereignisse::taste_senden`, `DateifensterQuelle::sondenbericht`, `Statuszeile::text` sowie die Sondenfelder und der Sondentakt im Anwendungsdelegierten. `grep -rn 'sonde\|Sonde\|SONDE\|VORUEBERGEHEND' crates/` findet keinen Treffer mehr, der die Sonde nennt, und das Bündel ist danach neu gebaut, signiert und gestartet worden.

Geblieben sind zwei Dinge, die die Sonde gefunden hat und die zum Auslieferungsstand gehören: der Eingabewächter und die beiden ausdrücklich gesetzten Tastenentsprechungen.

### Die acht Abnahmekriterien aus C2, im laufenden Bündel

Prüfordner `/tmp/krk-abnahme` mit drei Ordnern (Alpha, Beta, Zeta), acht Dateien und einer versteckten Datei; ausgeblendet zeigt die Liste elf Zeilen.

**1. Jede Funktion aus C1 bis C7 und aus C10 ist über mindestens einen Tastenbefehl erreichbar.** Belegt, aber nicht vollständig: `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` hält die 31 gebauten Kommandos gegen die Auslieferungsbelegung, und `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste` hält jede Funktion der Datei gegen mindestens eine Kombination. Die Richtung "jede Funktion des Specs steht in der Datei" prüft weiterhin nichts maschinell; sie ist erst mit S20 abschließend nachweisbar.

**2. Pfeiltasten, Bild auf und ab, Anfang und Ende.** Belegt. Aus Zeile −1 (keine Auswahl): `down` → Zeile 0 (Alpha), `down` → 1 (Beta), `up` → 0, `pagedown` → 10 (heinrich.txt), `pageup` → 0, `end` → 10, `home` → 0.

**3. Ein- und Aufstieg, Auswahl auf dem verlassenen Ordner.** Belegt. Auf Alpha stehend: `cmd+right` → Ordner `/tmp/krk-abnahme/Alpha`. `cmd+left` → `/tmp/krk-abnahme`, Auswahl auf `Alpha`. `cmd+up` → `/tmp`, Auswahl auf `krk-abnahme`. `return` danach: Ordner, Zeile und Auswahl unverändert.

**4. Tastenbefehle wirken nur außerhalb von Textfeldern.** Belegt. Bei offenem Eingabeblatt bewegt `cmd+left` den Ordner **nicht**; nach `esc` wirkt dasselbe `cmd+left` wieder und steigt nach `/tmp` auf. Damit ist beides gezeigt: der Vorbehalt greift, solange das Blatt steht, und er greift nicht länger.

**5. Pfadeingabe.** Teilweise belegt. Das Blatt fährt am Fenster herunter und trägt den angezeigten Ordner als Startwert (Bildschirmfoto). Die Eingabetaste bestätigt: mit dem Feldinhalt `a` meldete die Statuszeile "a ist kein absoluter Pfad", der Weg vom Feld über `pruefen` bis in die Statuszeile ist damit durchgespielt. Die Escape-Taste bricht ab (siehe Kriterium 4). **Ungeprüft geblieben ist der volle Weg "einen vorhandenen absoluten Pfad tippen und im Zielordner landen":** die Sonde bekam einzelne Zeichen nicht in den Feldeditor, und `cmd+v` scheitert an einer echten Lücke (siehe unten). Was dieser Weg tut, sobald ein gültiger Pfad im Feld steht, ist über `pruefen` in `cargo test` abgedeckt und über dieselbe Funktion aus der Zwischenablage im Bündel nachgewiesen.

**6. Tippen der Anfangsbuchstaben.** Belegt. `a` → Alpha, unmittelbar danach `n` → anton.txt (der Puffer sammelt). Nach 1,2 s Pause `b` → Beta (die Eingabe fängt von vorn an). Und der Fall, an dem die freie Eingabetaste hängt: `a` → Alpha, dann `return`, dann `n` → anton.txt. Hätte die Eingabetaste den Puffer berührt oder die Pause neu gestartet, wäre die Auswahl auf Alpha stehengeblieben oder auf ein `n` gesprungen.

**7. Mehrfachauswahl.** Belegt. Auf Zeile 0: `space` → 1 markiert, Auswahl auf Zeile 1; `space` → 2 markiert, Zeile 2; `cmd+a` → 11 markiert; `shift+cmd+i` → 0; nochmals → 11; `shift+cmd+a` → 0. Sichtbar ist die Markierung als orange Schrift über alle vier Spalten (Bildschirmfoto mit drei markierten Ordnern).

**8. Sortierung.** Belegt. `cmd+2` → Größe/aufsteigend, `cmd+2` nochmals → Größe/absteigend, `cmd+3` → Änderungsdatum/aufsteigend, `cmd+4` → Typ/aufsteigend, `cmd+r` → Typ/absteigend, `cmd+1` → Name/aufsteigend. Alle acht Ordnungen sind damit über die fünf Kombinationen erreichbar.

**9. Versteckte Dateien.** Belegt. `shift+cmd+h` → 12 Zeilen und `verstecke_ausgeblendet=false`, nochmals → 11 Zeilen und `true`.

### Die sechs Abnahmekriterien aus C10 zum Sprung, im laufenden Bündel

Jeweils Inhalt in die Zwischenablage gelegt, dann `opt+cmd+g`.

**1. Drei Ausgänge: lokaler Pfad, Web-Adresse, nichts Verwertbares.** Belegt durch die fünf Fälle unten und durch `cargo test -p krk-core --test zwischenablage`.

**2. Ein Pfad auf einen Ordner: das aktive Dateifenster wechselt hinein.** Belegt. `/tmp/krk-abnahme/Zeta` → Ordner `/tmp/krk-abnahme/Zeta`.

**3. Ein Pfad auf eine Datei: Wechsel in ihren Ordner, Auswahl auf den Eintrag; liegt sie schon da, kein Ordnerwechsel.** Belegt, beide Hälften. `/tmp/krk-abnahme/Alpha/tief.txt` → Ordner `/tmp/krk-abnahme/Alpha`, Auswahl `tief.txt`. `/tmp/krk-abnahme/emil.txt` bei angezeigtem `/tmp/krk-abnahme` → Ordner unverändert, elf Zeilen unverändert, Auswahl auf Zeile 7, `emil.txt`. Dazu der Fall des Nutzerentscheids: eine im Finder mit Cmd+C kopierte Datei legt **nur** einen Dateiverweis ab (`osascript -e 'clipboard info'` meldet `«class furl»`, `pbpaste` liefert nichts), und `opt+cmd+g` stellt die Auswahl trotzdem auf `dora.txt`.

**4. Ein Pfad, den es nicht gibt: die Statuszeile meldet den Grund.** Belegt. `/tmp/krk-abnahme/gibtsnicht` → Statuszeile "…gibt es nicht: No such file or directory (os error 2)", kein Ordnerwechsel. Es ist dieselbe Meldung, die die Pfadeingabe aus C2 kennt, weil es dieselbe Funktion ist.

**5. Eine Web-Adresse geht an den Systembrowser.** Belegt. `https://example.org/krk-abnahme-s13` → Safari kommt als neuer Prozess dazu und wird die vorderste Anwendung; KRK wechselt keinen Ordner und meldet nichts.

**6. Nichts von beidem: die Statuszeile meldet, KRK tut sonst nichts.** Belegt, drei Fälle. `smb://fileserver/freigabe` und `nur-ein-name.txt` → "die Zwischenablage trägt weder einen absoluten Pfad noch eine Web-Adresse". Leere Zwischenablage → "die Zwischenablage ist leer". In allen drei Fällen bleibt der Ordner stehen.

**KRK schreibt die Zwischenablage in keinem Fall.** `grep -rn 'setString.*forType\|writeObjects\|clearContents' crates/krk-ui/src` findet keinen Treffer.

---

## Was offen bleibt

Zwei Defekte sind neu abgelegt:

- `issues/260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md` — die eine Zusage aus C2, die mit dem heutigen Aufbau nicht zu halten ist. C2 verlangt für die Pfadeingabe ausdrücklich "tippt **oder fügt ein**". Auf dem Mac liegen `Cmd+X`, `Cmd+C`, `Cmd+V` und `Cmd+A` für Textfelder im Menü "Bearbeiten", und KRK hat keines. Am Bündel gemessen: `cmd+v` im offenen Eingabeblatt fügt nichts ein. Die Behebung berührt `menue.rs`, das die Dateiliste von S13 nicht nennt, und schafft vier weitere Menükürzel außerhalb der Konflikterkennung aus C3; das ist eine Entscheidung des Nutzers.
- `issues/260804-1309_o_die-markierung-ist-allein-an-der-farbe-erkennbar.md` — die Markierung aus C2 trägt kein zweites Kennzeichen neben der Farbe. Das trifft Nutzer mit einer Farbsehschwäche, und die Dateioperationen aus C4 wirken auf genau diese Markierung. Ein zweites Kennzeichen berührt entweder die Statuszeile oder das Aussehen der Liste; beides ist eine Festlegung und keine Nebenwirkung.

Kein weiterer offener Defekt wurde angefasst.

---

## Geänderte Dateien

Neu:

- `crates/krk-core/src/verzeichnis/sprungmarke.rs`
- `crates/krk-core/src/zwischenablage.rs`
- `crates/krk-core/tests/navigation.rs`
- `crates/krk-core/tests/zwischenablage.rs`
- `crates/krk-ui/src/kommandos/{mod.rs,navigation.rs,auswahl.rs,pfadeingabe.rs}`
- `crates/krk-ui/src/appkit/blaetter/{mod.rs,pfadeingabe.rs}`
- `crates/krk-ui/src/appkit/zwischenablage.rs`

Geändert:

- `crates/krk-core/src/lib.rs`, `crates/krk-core/src/verzeichnis/mod.rs`, `crates/krk-core/src/verzeichnis/modell.rs`, `crates/krk-core/src/tasten/belegung.rs`
- `crates/krk-core/tests/belegung.rs` (Teil 1)
- `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/tabs.rs`
- `crates/krk-ui/src/appkit/{mod.rs,anwendung.rs,ereignisse.rs,tabelle.rs}`

Workbench:

- `issues/260804-1214_c_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md` (geschlossen)
- `issues/260804-1214_c_die-pruefung-der-ab-werk-freien-kombinationen-kennt-die-vierte-nicht.md` (geschlossen)
- `decisions/260804-0830_i_was-die-zwischenablage-auswertung-liest.md` (umgesetzt)
- zwei neue Defekte, siehe oben

Nicht angefasst: `resources/default-keymap.toml`, `xtask/`, `crates/krk-bench/`, die Plandatei und der Spec.

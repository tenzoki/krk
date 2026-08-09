Eine ungesicherte Änderung ist fort, wenn die Vorschau dieselbe Datei zeigt

---

Vom Nutzer am 260809-2029 am laufenden Bündel beobachtet:

> wenn im editor was geändert wurde und man mit dem viewer die gleiche
> datei anzeigt (vor speichern) ist die änderung weg.

**Ungesicherte Arbeit geht verloren, ohne dass gefragt wird.** Das ist der
schwerste Fehler, den ein Editor haben kann.

---

## Warum das mehr ist als ein Anzeigefehler

Seit S18 (`111c72e`) schließen Editor und Vorschau einander aus: wer die Vorschau
einblendet, blendet den Editor aus. Der Weg des Nutzers geht also zwangsläufig
über einen Wechsel, und danach ist der Stand fort.

Die Entscheidung des Nutzers vom 260807-2139 sagt:
**Nachfragen beim Schließen, mit sichern, verwerfen und abbrechen.** Der Datensatz
ist `decisions/260807-2147_a_wie-greift-die-nachfrage-bei-der-sitzungssicherung.md`
in Verbindung mit Festlegung 3 der Sitzungshistorie. Hier wird nicht gefragt und
nicht gesichert, sondern verworfen.

## Was der Plan dazu schon sagt

Der Umsetzungsvermerk zu S18/S22 nennt einen benannten Zwischenstand: „ein
ungesicherter Stand fällt beim Dateiwechsel ohne Rückfrage (S28)". S28 heißt
„Drei der vier Anlässe: schließen, andere Datei, Vorschau einblenden" und hängt
über S27 an S26 und S25.

**Der beobachtete Fall ist der dritte Anlass aus S28.** Er ist also gesehen und
eingeplant, aber die Kette bis dorthin ist lang: S24, S25, S26, S27, S28.

## Was ungeprüft ist und der Bearbeiter zuerst klären sollte

`inference:`, nicht gemessen. Drei Fragen, die den Zuschnitt der Behebung
bestimmen:

1. **Fällt der Stand beim Ausblenden des Editors oder erst beim erneuten
   Einblenden?** Wenn `Editorbereich` sein `Editormodell` über den
   Sichtbarkeitswechsel hält, ist der Stand noch da und wird beim Wiederöffnen
   nur überschrieben. Dann genügt es, das Neuladen zu unterlassen, und die
   Rückfrage aus S28 kommt später obendrauf.
2. **Liest der Editor beim Einblenden neu von der Platte?** `Editormodell` trägt
   einen Stempel aus Änderungszeit und Größe (S15). Wenn der Wiedereintritt über
   `datei_oeffnen` läuft, liest er neu, und der Stempel entscheidet nichts.
3. **Zeigt die Vorschau tatsächlich den Plattenstand?** Das wäre für sich richtig,
   denn die Vorschau zeigt Dateien und nicht Editorstände. Die Frage ist nicht,
   was die Vorschau zeigt, sondern was mit dem Editorstand geschieht.

**Der billigste vollständige Weg ist vermutlich nicht die ganze Kette S24 bis
S28.** Wenn Frage 1 mit „der Stand ist noch da" beantwortet wird, ist die
Sofortmaßnahme klein und die Rückfrage bleibt S28 vorbehalten. Das ist zu messen,
bevor gebaut wird.

## Was daran hängt

Solange das offen ist, ist der Editor für wirkliche Arbeit nicht benutzbar: jeder
Blick in die Vorschau kostet die ungesicherte Änderung. Sichern gibt es
außerdem noch gar nicht, das ist S25.

**Aufgefallen bei:** eigener Bedienung durch den Nutzer am 260809-2029, unmittelbar
nach dem Übersetzungsstand `111c72e`.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` (Festlegung 3 und 9),
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260809-1921-coder-s18-s19-s20-s22-der-editor-wird-erreichbar.md`

---

## Abschlussnotiz, 260809-2148

**Geschlossen.** Die drei Fragen sind am Code beantwortet, die Ursache benannt und
die kleinste Änderung gebaut, die den Verlust beendet. Die Rückfrage aus S28 ist
nicht angefangen worden.

### Die drei Fragen, am Code beantwortet

**1. Fällt der Stand beim Ausblenden oder erst beim erneuten Einblenden?**
**Beim erneuten Einblenden, und zwar nur auf dem Weg über F4.** Das Ausblenden
ist unbeteiligt: `Aufteilung::anwenden` setzt `setHidden` auf die Unteransicht
(`crates/krk-ui/src/appkit/aufteilung.rs:201-205`), und eine ausgeblendete
`NSTextView` behält ihren Textspeicher. `Editorbereich` hält sein `Editormodell`
in einem `RefCell` über jeden Sichtbarkeitswechsel hinweg
(`crates/krk-ui/src/appkit/editor.rs:192-200`); keine Stelle im Programm ruft
`Editormodell::schliessen`. Der Verdacht des Datensatzes war richtig: *der Stand
ist noch da und wird beim Wiederöffnen überschrieben.*

**2. Liest der Editor beim Einblenden neu von der Platte?**
**Ja, und der Stempel entscheidet dabei nichts.** Der Wiedereintritt läuft über
F4, also `AnwendungsDelegierter::im_editor_oeffnen`
(`crates/krk-ui/src/appkit/anwendung.rs:2729-2763`) → `Editorbereich::datei_oeffnen`
→ `Editormodell::jetzt_oeffnen`. `jetzt_oeffnen` fragte den bisher gehaltenen
Pfad **nicht** und las bedingungslos neu (`editormodell.rs:543-550` im Stand
`111c72e`); `uebernehmen` setzte den Plattenstand ein und löschte die
Abweichungsmarke (`editormodell.rs:563-578`); `datei_oeffnen` schrieb ihn über
`stand_einsetzen` in die Textfläche. Der Stempel aus S15 wird beim Öffnen nur
**gesetzt**, nie gelesen — `fremd_geaendert` hat bis heute keinen Aufrufer.

**3. Zeigt die Vorschau den Plattenstand?**
**Ja, und das ist richtig und am Verlust unbeteiligt.** Die Vorschau liest über
`Vorschaumodell` und kennt den Editor nicht. Sie ist am Befund allein deshalb
beteiligt, weil sie den Editor nach C1 verdrängt (S18) und F4 der einzige
Befehl ist, der ihn mit seiner Datei zurückholt — `shift+cmd+e` (`fokus_editor`)
holt ihn ohne Neulesen zurück und verlor die Änderung deshalb nie.

### Die Ursache

`Editormodell::jetzt_oeffnen` behandelte „dieselbe Datei nochmal" wie „eine
andere Datei" und las neu. Der Weg des Nutzers war: F4 → tippen → `f3`/`cmd+y`
(Vorschau ein, Editor nach C1 aus) → F4 auf denselben Eintrag → Plattenstand
über das Getippte.

**Zwei Sachen mussten dafür zusammenkommen, und die zweite ist nicht behoben:**
das Neulesen, und der Umstand, dass das Getippte bis S26 überhaupt nicht im
Modell steht. `Editormodell::bearbeiten` hat im ganzen `appkit/` keinen
Aufrufer, denn der Delegierte `textDidChange:` kommt erst mit S26. Der getippte
Stand lebt allein in der `NSTextView`. Deshalb genügte es **nicht**, die
Abweichungsmarke zu befragen — sie ist immer `false`.

### Die Änderung

`Ladeausgang` bekommt den dritten Wert `SchonOffen`. `jetzt_oeffnen` kehrt damit
zurück, **bevor** es liest, wenn `haelt_bereits(pfad)` zutrifft; `datei_oeffnen`
ruft `stand_einsetzen` weiterhin allein bei `Geoeffnet` und fasst die Textfläche
damit nicht an; `im_editor_oeffnen` behandelt beide Ausgänge gleich und holt den
Editor hervor.

| Datei | Änderung |
|-------|----------|
| `crates/krk-ui/src/editormodell.rs` | `Ladeausgang::SchonOffen`, `haelt_bereits`, die Abkürzung in `jetzt_oeffnen`, zwei Proben |
| `crates/krk-ui/src/appkit/editor.rs` | Doc an `datei_oeffnen`: warum der Vergleich `Geoeffnet` namentlich nennt |
| `crates/krk-ui/src/appkit/anwendung.rs` | der Zweig nimmt `SchonOffen` mit auf |

**Der Preis, offen benannt:** F4 auf die schon gehaltene Datei liest sie auch
dann nicht neu, wenn sie sich von außen geändert hat. Ein Befehl zum Neulesen
gibt es nicht, und C2 sagt keinen zu; die Änderung von außen trägt S31.

**Was nicht behoben ist und bewusst offen bleibt:** F4 auf eine **andere** Datei
wirft den ungesicherten Stand weiterhin ohne Rückfrage — das ist der zweite
Anlass aus S28 und braucht S25. Solange nicht gesichert werden kann, ist „nicht
verlieren" das erreichbare Ziel.

**Nachgestellt** von `ein_zweites_oeffnen_derselben_datei_wirft_den_bearbeiteten_stand_nicht_weg`
(`editormodell.rs`), die ohne die Abkürzung fällt und mit ihr grün ist; gemessen
worden ist beides. `eine_andere_datei_wird_weiterhin_gelesen` hält fest, dass die
Abkürzung den S28-Anlass nicht stillschweigend mitnimmt.

**Abnahme:** `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo fmt --all --check` beenden mit 0.

**Nebenbefund, eigener Datensatz:**
`issues/260809-2148_o_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`.

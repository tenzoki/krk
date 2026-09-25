# S17b: Umbenennen eines einzelnen Eintrags in der Liste

**Status:** Complete
**Ausführender:** coder
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 17b

## Was gebaut ist

`shift+f6` und `shift+cmd+u` schalten die Namenszelle des ausgewählten Eintrags in den
Bearbeitungszustand. Return übernimmt, Escape verwirft. Ein unzulässiger oder bereits
vergebener Name benennt nichts um und meldet den Grund in der Statuszeile; die Zelle geht
dabei in den unveränderten Zustand zurück.

Kein Blatt, sondern eine bearbeitbare Zelle: `editColumn:row:withEvent:select:` stellt den
Feldeditor des Fensters in die Namenszelle, das Feld der Namensspalte trägt `isEditable`
und eine Aktion, und die Aktion wertet aus.

## Vier Abnahmekommandos, alle mit 0

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets` (0 Warnungen), `cargo fmt --all --check`. Der Testlauf zählt 367 Prüfungen,
0 gescheitert, 1 übersprungen; vor dem Schritt waren es 361.

## Die Abnahmepunkte einzeln

Alle am laufenden Programm mit `--tasten-protokoll` geprüft, Prüfordner
`/tmp/krk-s17b-pruef` mit `alt.txt`, `belegt.txt` und `weiter.txt`.

| Punkt | Beleg |
|---|---|
| `shift+f6` öffnet die Zelle | Bildschirmfoto 260805-1220: `neu.txt` steht im Feldeditor mit Fokusring, der Text ist ausgewählt. Protokollzeile `tastencode=97 maske=shift kombination=shift+f6 funktion=umbenennen` |
| `shift+cmd+u` öffnet die Zelle | Bildschirmfoto 260805-1212, Protokollzeile `tastencode=32 maske=shift+cmd kombination=shift+cmd+u funktion=umbenennen` |
| Delete löscht darin Text und keine Datei | Zwei Tastendrücke Delete bei offener Zelle: die Zelle ist danach leer, **das Tastenprotokoll bekommt keine neue Zeile**, und `ls` zeigt weiterhin alle drei Dateien. Siehe unten, "Wie der Fokusvorbehalt gemessen ist" |
| Return übernimmt den neuen Namen | `alt.txt` in `neu.txt` umbenannt; `ls` zeigt `neu.txt`, die Liste hat aufgefrischt, und die Auswahl steht auf dem umbenannten Eintrag |
| Escape verwirft ihn | Zelle geöffnet, `verworfen.txt` getippt, Escape: die Zelle zeigt wieder `neu.txt`, und `ls` zeigt `neu.txt` |
| Ein vergebener Name benennt nichts um und meldet den Grund | `neu.txt` auf `belegt.txt` umbenannt: Statuszeile `es gibt schon einen Eintrag namens „belegt.txt“`, rot, die Zelle zeigt wieder `neu.txt`, und `ls` zeigt beide Dateien unverändert |
| Der Diff zeigt die Kennung `umbenennen` in `Kommando` | `Kommando::Umbenennen` mit dem Eintrag `(Kommando::Umbenennen, "umbenennen")` in `KENNUNGEN`, 41 auf 42 Einträge. `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` läuft weiter durch |
| Der Diff zeigt genau **einen** Aufruf von `krk_core::operation::umbenennen` aus der Oberfläche | Der Diff fügt genau einen hinzu, in `umbenennen_ausfuehren`. Im Bestand stehen danach zwei: der neue und der aus `stapel_ausfuehren` von S17. **Den zweiten räumt S17c ab**; erst danach zeigt der Bestand einen Weg. Belegt mit `grep -rn "operation::umbenennen(" crates/krk-ui/src/`: `anwendung.rs:1050` (neu) und `anwendung.rs:1113` (S17) |

## Wie der Fokusvorbehalt gemessen ist

Der Vorbehalt aus S13 fragt, ob der Ersthelfer des Schlüsselfensters ein `NSTextView`,
`NSTextField` oder `NSText` ist, und gibt den Tastendruck dann unverändert an AppKit
weiter, **bevor** er ihn in der Belegung nachschlägt. Der Feldeditor einer Tabellenzelle
ist der gemeinsame `NSTextView` des Fensters, also fällt er unter die erste der drei
Klassen. Eine zweite Regel je Zelle entsteht nicht.

Gemessen mit `--tasten-protokoll`, weil das die Frage direkt beantwortet: der Abgriff
schreibt für **jeden** Tastendruck, den er nachschlägt, eine Zeile. Bei offener
Umbenennen-Zelle habe ich zweimal Delete gedrückt. Das Protokoll war vorher und nachher
drei Zeilen lang, der Text in der Zelle war weg, und die drei Dateien standen unverändert
im Ordner. Der Vorbehalt hat also gegriffen, und zwar an der Stelle, an der er greifen
soll: vor dem Nachschlag.

Damit hat der Fokusvorbehalt seine dritte und letzte Prüfung; C4 nennt neben der
Pfadeingabe und den Blättern das Umbenennen-Feld namentlich, und bis zu diesem Schritt gab
es keines.

## Der Entwurf in drei Sätzen

**Die Zeile kommt von der Tabelle und nicht aus einem gemerkten Zustand.** Die
Zellenansicht ist das Feld, das die Aktion schickt, also beantwortet `rowForView:` die
Frage "welche Zeile war das". Ein gemerkter Bearbeitungszustand hätte eine zweite
Löschregel gebraucht, für den Fall, dass die Bearbeitung ohne Aktion endet.

**Escape kommt in der Auswertung gar nicht an.** AppKit bricht die Bearbeitung über
`abortEditing` ab, stellt den alten Text her und schickt keine Aktion. Die Zusage "Return
übernimmt, Escape verwirft" kostet damit keine eigene Regel.

**Ob der Name schon vergeben ist, beantwortet das Dateisystem.** `operation::umbenennen`
scheitert mit `AlreadyExists`, und `operationen::umbenennungsfehler` macht daraus den Satz,
den auch das Anlegen sagt. Eine Vorabprüfung gegen die gelesene Liste wäre eine zweite
Wahrheit über denselben Ordner und ginge zwischen Lesen und Umbenennen ohnehin ins Leere.

## Was der Plan nicht nannte

**`crates/krk-ui/src/appkit/anwendung.rs`.** Die Umbenennung selbst läuft dort, aus einem
Grund, den S17 für das Anlegen schon festgehalten hat: `auffrischung::ordner_neu_lesen`
nimmt eine `Dateifenstersicht` entgegen, und die setzt allein der Anwendungsdelegierte um.
Von der Datenquelle aus wäre nur das eigene Dateifenster erreichbar, und das andere zeigte
den alten Namen weiter. Die Zelle sammelt und prüft den Namen, ausgeführt wird er dort;
der Weg dahin ist ein Rückruf neben den beiden vorhandenen (`aktivierung`,
`ordnerwechsel`), gesetzt in `oberflaeche_aufbauen` und den Delegierten **schwach**
haltend wie jene.

Der Befehl selbst brauchte keine Zeile in `kommando_ausfuehren` des Anwendungsdelegierten:
`Kommando::Umbenennen` gehört einem einzelnen Dateifenster und fällt in den vorhandenen
Zweig "alles Übrige gehört dem aktiven Dateifenster".

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/tasten/belegung.rs` | `Kommando::Umbenennen`, Eintrag in `KENNUNGEN` (41 → 42) |
| `crates/krk-ui/src/kommandos/operationen.rs` | `Umbenennungswunsch`, `umbenennung_pruefen`, `umbenennungsfehler`, gemeinsamer Satz `schon_vergeben` für Anlegen und Umbenennen, sechs Prüfungen |
| `crates/krk-ui/src/appkit/tabelle.rs` | bearbeitbare Namenszelle mit Aktion, `umbenennung_beginnen`, `umbenennung_beenden`, `zeile_neu_zeichnen`, Rückruf `umbenennung_setzen`, `Spalte::beschreibbar` und `NAMENSSPALTE` |
| `crates/krk-ui/src/appkit/anwendung.rs` | `umbenennen_ausfuehren` und das Setzen des Rückrufs je Dateifenster |

## Prüfdaten

`/tmp/krk-s17b-pruef` und die Bildschirmfotos unter `/tmp`, alle selbst angelegt und am
Ende der Sitzung entfernt. Das Tastenprotokoll lag unter `/tmp/krk-tasten.log`.

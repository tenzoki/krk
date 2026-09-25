# S23, S30 und S31: der Übergang aus der Vorschau, die gemerkte Datei und die Änderung von außen

---
**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Abschnitte `#### 23.`, `#### 30.`, `#### 31.`

---

## Was gebaut wurde

Drei Planschritte in einem Übersetzungsstand, weil alle drei in `anwendung.rs`
schreiben und zwei von ihnen zusätzlich in `editormodell.rs`. Einzeln gefahren
hätten sie einander überschrieben; derselbe Grund, aus dem S27 bis S29 am
260810-0021 zusammengelegt wurden.

**S23, der Übergang aus der Vorschau in den Editor.** `cmd+e` mit dem Fokus in
der Vorschau nimmt die dort angezeigte Datei und öffnet sie im Editor. Gebaut
sind ein Zweig in `kommando_ausfuehren` und `editor_aus_vorschau`, zusammen
dreizehn Zeilen. Zeigt die Vorschau den Inhalt der Zwischenablage oder nichts,
meldet die Statuszeile den Grund.

**S30, die Sitzung merkt sich die geöffnete Datei.** `Sitzung` trägt ein Feld
`editor: Option<PathBuf>`, das den Pfad hält und sonst nichts. Beim Start wird
die Datei über dieselbe Prüfung wieder geöffnet; ist sie fort oder zu groß
geworden, bleibt der Editor leer, wird ausgeblendet und meldet den Grund.

**S31, die von außen geänderte Datei.** Die bestehende Dateisystembeobachtung
beobachtet einen dritten Ordner, nämlich den der Datei, die der Editor hält.
Meldet FSEvents ihn, vergleicht der Editor seinen gemerkten Stempel und schreibt
einen Satz in die Statuszeile. Ein zweiter Strom ist nicht entstanden.

## Die Frage, die der Auftrag gestellt hat

Der Auftrag verlangte, nicht anzunehmen, sondern zu prüfen, ob S23 die Nachfrage
aus C4 wirklich ohne eine eigene Zeile erbt. Er tut es, und der Beleg ist eine
Kette mit genau einem Glied je Stufe:

```text
  editor_aus_vorschau ─┐
  im_editor_oeffnen  ──┼──> Editorbereich::datei_oeffnen ──> Editormodell::oeffnen
  editor_wiederherstellen ─┘        (drei Aufrufstellen)        (ein Aufrufer)
                                                                     │
                                                        Arbeitsfaden, Prüfung
                                                                     │
                                          Zurueckgehalten <──────────┤
                                                 │
                            editorausgang_behandeln ──> nachfrage_zeigen(AndereDatei)
```

`grep -rn 'borrow_mut().oeffnen' crates/krk-ui/src` findet neben der Tabliste
genau einen Aufrufer von `Editormodell::oeffnen`, nämlich
`Editorbereich::datei_oeffnen`; der wiederum hat in `anwendung.rs` drei
Aufrufstellen. Alle drei laufen deshalb durch dieselbe Prüfung und dieselbe
Rückhaltung. Eine zweite Abfrage des ungesicherten Standes steht nirgends, und
sie stünde vor der Prüfung, was das elfte Abnahmekriterium von C2 verbietet.

## Drei Entscheidungen, die der Plan nicht vorgezeichnet hat

**Die Wiederherstellung braucht ein unterscheidendes Kennzeichen, und das ist
kein Umweg.** Sie geht denselben einen Weg wie die beiden Einstiege, aber zwei
ihrer Ausgänge verlangen etwas anderes als bei einem Befehl. `Geoeffnet` darf
den Fokus nicht holen, weil er beim Start nach `fokus::BEIM_START` in das aktive
Dateifenster gehört; `Abgewiesen` ist die Antwort auf keinen Tastendruck und
gehört auf Rang 3 statt auf Rang 1, wo der erste Tastendruck sie wegräumte.
Gebaut ist `AnwendungsIvars::editor_aus_sitzung` nach dem Zuschnitt, den S29 für
`beenden_ohne_nachfrage` genommen hat: ein Feld, ein Schreiber, ein Leser, und
der Leser verbraucht es beim ersten Ausgang.

**Der Stempelvergleich steht außerhalb des Auffrischungsaufschubs und läuft
einmal je Stapel.** Der Aufschub beantwortet, ob eine Dateiliste neu zu lesen
ist; das ist eine andere Frage. Ein Stapel-Umbenennen, das die Datei des Editors
erwischt, soll gemeldet werden, auch während die Liste stehen bleibt. Einmal je
Stapel statt einmal je Pfad, weil die Frage „geht mich dieser Rückruf etwas an"
lautet und ein Stapel von tausend Meldungen sonst tausend `stat(2)` kostete.

**Gefragt wird über `fremd_geaendert` und nicht über den Stempel selbst.** S25
hat die Regel gezogen: eine Frage, eine Stelle. Damit hat S31 die Ankündigung an
`Editormodell::stempel` nicht eingelöst, und der Defekt
`issues/260810-0212_o_drei-stuecke-...` trägt seit heute einen Nachtrag, der aus
drei Stücken ohne Aufrufer vier macht.

## Was der Plan an Dateien nicht genannt hat

Zwei Dateien sind mitgezogen, und beide gehen nicht anders:

- **`crates/krk-ui/src/fenstermodell.rs`** (S30). `Fenstermodell::sitzung` baut
  `Sitzung` als Literal, und ein neues Feld hält dort den Bau an. Genommen ist
  der Weg, den die Funktion für die Tabs schon geht und begründet: der Wert kommt
  von außen dazu, als zweiter Parameter. Der Grund ist derselbe — die Datei wohnt
  im `Editormodell`, und das Fenstermodell kennt vom Editor allein Breite und
  Sichtbarkeit.
- **`crates/krk-ui/src/appkit/editor.rs`** (S31). Der Editorbereich hält das
  Modell; dazugekommen ist eine Zugriffsfunktion, `fremdaenderung_melden`.

## Abnahme

Die vier Kommandos laufen durch: `cargo build --workspace`,
`cargo test --workspace` (706 Proben, 0 Fehlschläge, 1 übersprungene Kindprobe),
`cargo clippy --workspace --all-targets` ohne Warnung, `cargo fmt --all --check`.

Die Greps der Abnahmekriterien:

| Kriterium | Ergebnis |
|---|---|
| Aufrufstellen von `datei_oeffnen` in `anwendung.rs` | 3 — F4, Vorschau, Sitzung |
| `FSEventStreamCreate(` in `fsevents.rs` | 1 Aufruf, unverändert |
| `kFSEventStreamCreateFlagFileEvents` | weiterhin nicht gesetzt |
| `ungesichert::zeigen` in `anwendung.rs` | 1, wie seit S28 |

Neue Proben: drei in `crates/krk-core/tests/ablage.rs` (Rundreise byteweise,
`session.toml` ohne das Feld, Pfad steht nur bei geöffneter Datei in der Datei),
zwei in `crates/krk-ui/src/auffrischung.rs` (der dritte Ordner und seine
Doppelnennungen, der gemeldete Ordner ohne die Editordatei), zwei in
`crates/krk-ui/src/editormodell.rs` (die fremde Änderung meldet sich einmal;
ohne Datei meldet sich nichts).

## Was der Nutzer prüfen muss

Am laufenden Bündel, weil kein Agent es kann:

1. Mit dem Fokus in der Vorschau auf einer Textdatei öffnet `cmd+e` sie im
   Editor, und die Vorschau verschwindet dabei.
2. Zeigt die Vorschau den Inhalt der Zwischenablage, meldet `cmd+e` den Grund
   und öffnet nichts.
3. Mit einer ungesicherten Änderung im Editor führt `cmd+e` auf eine **andere**
   Datei zur Nachfrage aus C4, und „Abbrechen" lässt den alten Stand stehen.
4. Nach Beenden und Neustart steht dieselbe Datei im Editor, ohne dass der
   Eingabefokus dort hineinspringt.
5. Wird die gemerkte Datei zwischen Beenden und Neustart gelöscht, bleibt der
   Editor leer und ausgeblendet, und die Statuszeile nennt den Grund.
6. Eine im Terminal geänderte Datei, die der Editor hält, meldet sich innerhalb
   einer Sekunde in der Statuszeile.
7. Ein `cmd+s` danach überschreibt sie nicht, sondern meldet es.
8. Eine Änderung in einem Ordner, in dem die Datei des Editors **nicht** liegt,
   löst keine Meldung des Editors aus.

## Zustand danach

S23, S30 und S31 tragen `[DONE]`. Der Datensatz
`decisions/260807-2147_i_wie-greift-die-nachfrage-bei-der-sitzungssicherung.md`
ist von „beantwortet" auf „umgesetzt" gewandert. Offen in Phase G und H bleiben
S38, S39 und S42.

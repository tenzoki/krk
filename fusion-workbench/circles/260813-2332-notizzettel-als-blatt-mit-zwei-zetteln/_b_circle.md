# Ein Notizzettel als Blatt am Hauptfenster, zwei Zettel, sichert sich selbst

---
**Domain:** code
**Status:** bounded
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_*_plan-notizzettel-als-blatt-mit-zwei-zetteln.md
**Active session history:** circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260813-2342-orchestrator-session.md

---

## Directive

KRK trägt einen Notizzettel. `f2` und `cmd+k` holen ihn als Blatt an das Hauptfenster, mittig über den fünf Bereichen; er führt zwei Zettel als anklickbare Tabs, und offen ist immer der zuletzt geöffnete, auch über einen Neustart hinweg. Die Fläche nimmt Tippen, Einfügen und Rückgängig an und sonst nichts: kein Suchen und Ersetzen, keine Zeilennummern, keine Syntaxhervorhebung, keine Textmarken. `Esc` schließt das Blatt. Gesichert wird ohne Zutun des Nutzers, an drei Punkten — beim Wechsel zwischen den beiden Zetteln, beim Schließen des Blattes und beim Beenden von KRK. Jeder Zettel liegt als eigene Datei im Ablageordner unter `~/Library/Application Support/KRK/`. Die Zulässigkeitsregel aus der achten Runde bleibt unangetastet.

## Grounding snapshot

Der Stand des Baums am 260813-2332, und die sieben Antworten aus zwei Klärungsrunden mit dem Nutzer.

### Der Zettel ist ein Blatt, und das ist die tragende Festlegung

Der Nutzer hat die Form entschieden: ein Blatt am Hauptfenster, wie KRKs neun bestehende Blätter unter `crates/krk-ui/src/appkit/blaetter/`. Ein eigenes Fenster und ein sechster Bereich der Fensterzeile sind verworfen. Der Zettel wird das zehnte Blatt und benutzt die gemeinsame Hülle aus `blaetter/mod.rs`, deren Modulkopf den Grund für eine Hülle statt neun eigener Aufbauten selbst nennt.

Damit trägt die Zulässigkeitsregel der achten Runde den Fall vollständig, und **die Runde fasst sie nicht an**. `kommandos/zulaessigkeit.rs` fragt seit dem 260813 vier Dinge, und ein anhängendes Blatt braucht in keinem davon einen Sonderfall: es **ist** das Schlüsselfenster, also sagt der vierte Bestandteil ja, und über das Blatt entscheidet allein der erste.

### Was ein stehendes Blatt anhält, und welche Folge das für den Zettel hat

`kommandos::operationen::waehrend_blatt_erlaubt` ist eine Zeile: erlaubt ist genau `Kommando::Abbrechen`. Solange der Zettel steht, wirkt also kein anderer Befehl von KRK, und dazu gehören `f2` und `cmd+k` selbst. **Der Zettel lässt sich nicht mit derselben Taste wieder schließen, mit der er gekommen ist**; heraus geht es über `Esc` und über die Schaltfläche des Blattes. Das ist keine Lücke, sondern die Folge der entschiedenen Form, und sie aufzuheben hieße die Regel anzufassen, die diese Runde ausdrücklich stehen lässt.

Der Wechsel zwischen den beiden Zetteln stößt nicht an diese Grenze, weil die Tabs **angeklickt** werden und kein Kommando sind. Genau darum kommt die Runde ohne einen Eintrag in `waehrend_blatt_erlaubt` aus.

`Esc` liegt ab Werk auf `abbrechen`, und `abbrechen` trägt `Wirkungsbereich::Ueberall` (`krk-core/src/tasten/belegung.rs`). Bei stehendem Blatt ist es der eine durchgelassene Befehl. Der offene Datensatz `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/decisions/260813-0320_*_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md` betrifft `Esc` mit dem Fokus im Editor und nicht in einem Blatt; er bindet diese Runde nicht, ist aber vor dem Bau zu lesen, weil er die einzige Stelle ist, an der `Esc` in diesem Baum schon einmal zwei Empfänger hatte.

### Die nackte Textfläche ist die Bedingung dafür, dass der Zettel ein Blatt sein kann

Der Umfang aus Antwort 4 ist keine Sparvariante. Der volle Editor öffnet für die Suche und für den Sprung zur Zeilennummer **eigene Blätter** (`blaetter/suche.rs`, `blaetter/zeilennummer.rs`), und ein Blatt über einem Blatt geht in AppKit nicht. Ein Zettel mit dem Funktionsumfang des Editors wäre deshalb kein Blatt, sondern zwänge zurück auf das verworfene eigene Fenster. Umfang und Form hängen aneinander; wer den einen später erweitert, hebt die andere auf.

Der Kern unter `crates/krk-core/src/text/` steht mit Zeilenindex, Suche, Ersetzen, Einlesen und Sicherungsform bereit. Wie viel davon ein Zettel ohne Suche und ohne Zeilennummern braucht, entscheidet der Planer.

### Zwei Vorkehrungen der bestehenden Blätter, und eine davon gilt hier anders

**Der Ersthelfervorbehalt zeigt für den Zettel in die andere Richtung als für den Editor.** `CLAUDE.md` warnt: wer eine zweite bedienbare Textfläche baut, meldet sie in `ersthelfer_gehoert_appkit` (`appkit/ereignisse.rs`) an, sonst gehören ihre Tasten AppKit. Für die Textfläche eines Blattes ist genau das erwünscht — sie **soll** die Tasten bekommen, sonst tippt niemand hinein. Die Fläche des Zettels wird dort **nicht** angemeldet, und der Grund gehört in ihren Modulkopf, weil die stehende Warnung sonst zum Gegenteil verleitet.

**Der `Eingabewaechter` der neun bestehenden Blätter fängt zwei Befehle ab**, `insertNewline:` und `cancelOperation:`, und beendet damit das Blatt. Für den Zettel gilt das nur zur Hälfte: `Esc` schließt, die Eingabetaste muss eine neue Zeile setzen. Das ist der erste Unterschied des zehnten Blattes zu den neun bestehenden und die Stelle, an der ein unbesehenes Nachbauen der Hülle den Zettel unbrauchbar machte.

### Drei Sicherungsmomente, und was dabei verloren gehen darf

Gesichert wird beim Tabwechsel, beim Schließen und beim Beenden. Der Zweisekundentakt ist ausdrücklich **nicht** gewählt.

**Die benannte Folge:** stürzt KRK bei stehendem Zettel ab, ist alles verloren, was seit dem Öffnen des Zettels getippt wurde. Der Nutzer hat das am 260813 in Kauf genommen. Das ist keine Lücke der Spezifikation, sondern eine Zusage, die diese Runde nicht macht.

### Zwei Dateien im Ablageordner, und zwei Stellen, die dabei den Bau anhalten

Der Nutzer hat zwei einzelne Dateien gewählt, **nicht** eine gemeinsame `zettel.toml`. Ein fünfter und ein sechster Eintrag im Ablageordner berühren zwei Stellen, die bei einer Auslassung nicht schweigen, sondern den Bau anhalten:

- `Datei::ALLE` ist heute `[Datei; 4]` in `crates/krk-core/src/ablage/pfade.rs`. Die Aufzählung besteht ausdrücklich, damit wer alle Ablagedateien anfassen muss, keine vergessen kann.
- Die Probe `nur_benannte_dateien_erreichen_das_atomare_schreiben` in `crates/krk-core/tests/baum.rs` zählt genau fünf Quelldateien auf, die `atomar::schreiben` erreichen dürfen. Eine sechste schreibende Datei ist dort einzutragen, mit Begründung wie die fünf bestehenden.

### Die Schreibsperre verhindert ein Gemisch, kein Überschreiben

Über dem Ablageordner stehen seit dem 260813 genau zwei Absprachen (`crates/krk-core/src/ablage/sperre.rs`): der kurzlebige `Schreibgriff` je Lesen-Ändern-Schreiben und das langlebige `Sitzungsrecht`. Der Schreibgriff sorgt dafür, dass keine halb geschriebene und keine vermischte Datei entsteht. **Er sorgt nicht dafür, dass ein Stand erhalten bleibt.**

Laufen zwei Instanzen von KRK und bearbeiten beide denselben Zettel, gewinnt die zuletzt schließende; der Stand der anderen ist weg. Der Nutzer hat diese Gefahr mit Antwort 7 in Kauf genommen. **Der Fall bleibt damit offen und ist nicht übersehen worden** — er steht hier, damit die nächste Runde ihn nicht als neuen Befund entdeckt.

### Die Taste: zwei Wege ab Werk, eine Zeile in der Belegungsansicht

`f2` und `cmd+k` lösen beide denselben Befehl aus. Das folgt dem Nutzerentscheid vom 260802-1409 für die Norton-Reihe (`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1409-shaper-fn-tasten-messung-und-cmd-kuerzel.md`, Antwort 2): zwei Wege ab Werk auf dieselbe Funktion, keine zweite Belegungsart, und in der Belegungsansicht eine Zeile je Funktion mit allen Kombinationen darin.

Der Grund für den zweiten Weg ist dort schon benannt: die Alltagstastatur hat eine echte Funktionstastenreihe, der Touch Bar steht allein beim Abnahmegerät im Weg. `cmd+k` umgeht ihn.

Am 260813 an `resources/default-keymap.toml` nachgesehen: die Datei führt 82 Belegungszeilen, `f2` ist unbelegt und `cmd+k` ebenfalls. Belegt ist `shift+cmd+k`, der zweite Weg zum Kopieren neben `f5`. Die beiden gewählten Kombinationen sind damit heute frei; ob sie es beim Bau noch sind, prüft die Runde erneut, denn die Belegung wächst mit jeder Runde.

### Welcher Zettel zuletzt offen war, ist Sitzungszustand

„Offen ist immer der zuletzt geöffnete" macht den zuletzt offenen Zettel zu einem Zustand, der die Sitzung überdauert. Der naheliegende Ort ist `Sitzung` in `crates/krk-core/src/ablage/sitzung.rs`, das ohnehin Fenster, Tabs, Auswahl, Breiten und den Pfad der Editordatei über einen Neustart trägt; jede Struktur dort trägt `#[serde(default)]`, ein neues Feld macht eine ältere `session.toml` also nicht ungültig. Das ist eine Vorgabe des Shapers, keine Nutzerentscheidung, und der Planer darf sie verwerfen.

Zwei Folgen, falls er sie behält. Erstens schreibt den Sitzungszustand nur die Instanz, die das `Sitzungsrecht` hält — bei zwei laufenden Instanzen merkt sich KRK die Zettelwahl der zweiten nicht. Zweitens läuft der `Sitzungsschreiber` in einem Takt von zwei Sekunden. **Das ist kein Widerspruch zur Absage an den Zweisekundentakt aus Antwort 5**: der Takt trüge die Merkung, welcher Zettel offen war, und nie den Text des Zettels.

### Was nicht in dieser Runde liegt

Mehr als zwei Zettel. Suchen, Ersetzen, Zeilennummern, Syntaxhervorhebung und Textmarken im Zettel. Ein Zettel als eigenes Fenster oder als sechster Bereich. Eine Änderung an `waehrend_blatt_erlaubt` oder an `zulaessigkeit::zulaessig`. Eine Auflösung der Überschreibgefahr zwischen zwei Instanzen.

## Dependencies

- `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags` — die achte Runde hat `zulaessigkeit::zulaessig` um die Schlüsselfensterfrage erweitert. Diese Regel trägt den Zettel; die Runde ist geschlossen (`_c_`), die Regel steht.
- `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` — die siebte Runde hat Schreibgriff und Sitzungsrecht getrennt und den Fall der zweiten Instanz aufgemacht. Ihr offener Datensatz zu `Esc` im Editor ist vor dem Bau zu lesen.
- `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — die sechste Runde hat die Ablage angefasst, in die die zwei Zetteldateien einziehen.

## Turn log

- Turn 1 (Sitzung 260813-2342): abgeschlossen. Alle sechzehn Planschritte gebaut, Commits 9362034, a949ff1, bfea397, dd2643e, dazu die Durchsicht. make check exit 0. Neun Befunde gefiltert, einer hoch: ein Neuoeffnen nach gescheiterter Sicherung wirft den ungesicherten Stand weg, und die Ursache liegt in C4 des Spec. Coherence-Urteil: review-needed. Sitzungsprotokoll: circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260813-2342-orchestrator-session.md

- Turn 2 (Sitzung 260813-2342): abgeschlossen. C4 des Spec nachgezogen, drei zusammenhaengende Befunde behoben, zwei Defekte geschlossen, Abgleich gefahren. Alle 16 Planschritte halten am Baum, 43 der 72 Abnahmekriterien traegt der Baum. Coherence-Urteil: review-needed wegen zweier mittlerer Befunde.

- Turn 3 (Sitzung 260813-2342): abgeschlossen. Die zwei mittleren Befunde behoben, Abnahmelauf durch den Nutzer gefahren, Abschlussabgleich. Coherence-Urteil: review-needed, weil 21 der 29 Buendelkriterien unbelegt bleiben. Der Nutzer hat den beschraenkten Abschluss gewaehlt.

## Closure note

**Geschlossen als beschraenkter Abschluss (`_b_`) am 260814-1300.** Der Baum ist fertig; was fehlt, ist Abnahme im Vordergrund.

**Was die Runde gebaut hat.** Ein Notizzettel als zehntes Blatt am Hauptfenster, geholt mit `f2` oder `cmd+k`, geschlossen mit `Esc`. Zwei Zettel als anklickbare Tabs auf einer nackten Textflaeche, gesichert an vier Momenten: Tabwechsel, `Esc`, Beenden von KRK, und `shift+cmd+w`. Zwei Dateien im Ablageordner, `note-1.txt` und `note-2.txt`; `Datei::ALLE` waechst auf sechs, und ein `Format` trennt TOML von Text. `atomar::schreiben` und `Zugang::beiseite_legen` nehmen einen Leser statt einer Zeichenkette, und dieselbe Zahl `EDITORGRENZE` begrenzt Laden und Beiseitelegen. Drei neue Dateien unter `krk-ui`, `appkit` fuehrt 29 Module.

**Warum beschraenkt und nicht kohaerent.** Die Abnahmeliste des Orchestrators war nicht an die Abnahmekriterien gebunden. Zwoelf Beobachtungen standen fuer 29 Kriterien mit Buendelanteil; belegt sind davon **8**, halb beruehrt 5, und **16 hat keine Beobachtung angefasst** — die fuenf Zwischenablagebefehle im Zettel, die sieben Textautomatiken am laufenden Programm, alle drei Beenden-Kriterien von C4. Die dem Nutzer genannte Zahl „71 von 72" stimmte an beiden Enden nicht: die Grundmenge ist seit dem C5-Nachtrag 75, mit den zwei C8-Kriterien 77.

**Der strukturelle Unterschied zur achten Runde, und er ist der eigentliche Ertrag dieser Notiz.** Die Runde 8 kennzeichnete jedes Abnahmekriterium einzeln als `(Probe)` oder `(Buendel)`: zehn Buendelkriterien, elf Beobachtungen, eine je Kriterium. Ihr „alle 59 bis auf eines" war deshalb nachrechenbar, und sie konnte kohaerent schliessen. Diese Runde fuehrt **zwei Listen je Faehigkeit**, was beim Lesen uebersichtlicher ist und die Bindung zwischen Beobachtung und Kriterium verliert. Wer die naechste Runde zuschneidet, waehlt zwischen Lesbarkeit und Nachrechenbarkeit — und diese Runde ist der Beleg, dass die zweite teurer ist, als sie aussieht.

**Was gelernt wurde und die Directive nicht vorhersagen konnte.** Zwei Zusagen von C4 hielten nicht zugleich: eine gescheiterte Sicherung wirft den Stand nicht weg, und der Zettel liest bei jedem Oeffnen neu. Die Durchsicht hat den Widerspruch gefunden, der Nutzer hat ihn entschieden (der getippte Stand gewinnt), und erst dadurch wurde ein zweiter Fall sichtbar: solange ein abweichender Stand beim Oeffnen verschwand, konnten kaum je zwei Zettel zugleich abweichen. Die Antwort auf den einen Defekt hat den zweiten erst erzeugt.

Ebenso: die Kopiergrenze. C5 sagte zu, eine zu grosse Datei werde beiseitegelegt, und legte nicht fest, wie gross „beiseite" werden darf. Der Bau nahm das woertlich und kopierte unbegrenzt auf dem Hauptfaden.

**Was offen bleibt.** 18 Defekte im Circle. Keiner betrifft das Verhalten des Zettels im gewoehnlichen Gebrauch; der Schwerpunkt liegt bei Prosa, die dem Code hinterherlaeuft, und bei der Abnahmedeckung selbst. Dazu die nicht festgehaltene Messung, welche Kante `performClose:` an einem Fenster mit anhaengendem Blatt geht — kein Abnahmekriterium haengt daran, wohl aber eine spaetere Runde am Schliessweg.

**Bilanz.** Drei Turns, zehn Commits, alle sechzehn Planschritte am Baum bestaetigt. `make check` exit 0 nach jedem Strang und nach jedem Turn. Zwei Entscheidungsdatensaetze umgesetzt. Sitzungsprotokoll: `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260813-2342-orchestrator-session.md`. Abgleiche: `history/260814-1002-reconciliation.md` und `history/260814-1247-reconciliation.md`. Abnahmeliste mit Ergebnissen: `history/260814-1100-abnahmeliste-notizzettel.md`.

**Der Weg zu einem kohaerenten Abschluss ist benannt und offen:** eine zweite Abnahmeliste, gebunden an die 21 unbelegten Kriterien, rund zwanzig Minuten im Vordergrund.

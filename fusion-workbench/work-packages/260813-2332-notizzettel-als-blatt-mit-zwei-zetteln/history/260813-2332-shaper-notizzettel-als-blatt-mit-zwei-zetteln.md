# Der Notizzettel wird ein vorgesehener Circle

**Agent:** shaper (anticipated-circle mode)
**Zeitpunkt:** 260813-2332
**Status:** Complete
**Auftrag:** Den Backlog-Eintrag `260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md` als vorgesehenen Circle anlegen, mit den sieben bereits gegebenen Antworten aus zwei Klärungsrunden.

## Der Entwurf

Der Eintrag vom 260813-2033 verlangte einen Notizzettel: eine Fläche, die auf Tastendruck mittig über den anderen Bereichen erscheint, in der der Nutzer kurz etwas ablegt, und die sich von selbst sichert. Er nannte sechs offene Punkte und begründete, warum er keine Kleinigkeit ist: eine Fläche „mittig über den anderen Bereichen" ist auf dem Mac entweder ein Blatt oder ein eigenes Fenster, und davon hängt ab, ob KRKs Tastenbefehle dahinter noch wirken.

## Was schon geklärt war

Zwei Klärungsrunden waren gelaufen, sieben Fragen beantwortet. Der Auftrag wies an, keine davon erneut zu stellen. Die Antworten in Kürze:

| | Frage | Antwort des Nutzers |
|---|---|---|
| 1 | Form | ein Blatt am Hauptfenster; eigenes Fenster und sechster Bereich verworfen |
| 2 | Sicherung | beim Schließen und beim Beenden |
| 3 | Anzahl | zwei Zettel als anklickbare Tabs, offen ist der zuletzt geöffnete |
| 4 | Umfang | nackte Textfläche: Tippen, Einfügen, Rückgängig |
| 5 | Sicherungsmomente | drei — Tabwechsel, Schließen, Beenden; kein Zweisekundentakt |
| 6 | Taste | `f2` und `cmd+k` zusammen, zwei Wege ab Werk |
| 7 | Ablage | zwei einzelne Dateien im Ablageordner, nicht eine gemeinsame Datei |

Eine achte Runde war nicht nötig. Kein tragender Punkt blieb offen, den diese sieben nicht abdecken.

## Was am Baum nachgesehen wurde

- **Die neun Blätter** unter `crates/krk-ui/src/appkit/blaetter/` und ihre gemeinsame Hülle in `blaetter/mod.rs`. Der Zettel wird das zehnte.
- **`kommandos::operationen::waehrend_blatt_erlaubt`** ist eine Zeile und erlaubt genau `Kommando::Abbrechen`. Daraus folgt: der Zettel lässt sich nicht mit derselben Taste schließen, mit der er kommt, und der Tabwechsel muss ein Klick sein und kein Kommando. Beides steht als benannte Folge in der Grounding-Aufnahme.
- **`kommandos/zulaessigkeit.rs`**, die vier Bestandteile der Regel aus der achten Runde. Ein anhängendes Blatt braucht in keinem davon einen Sonderfall.
- **Der `Eingabewaechter`** fängt bei den neun bestehenden Blättern `insertNewline:` und `cancelOperation:` ab. Für den Zettel gilt das nur zur Hälfte, weil die Eingabetaste dort eine Zeile setzen muss. Erster Unterschied des zehnten Blattes zu den neun.
- **`ersthelfer_gehoert_appkit`** in `appkit/ereignisse.rs`. Die stehende Warnung in `CLAUDE.md` zeigt hier in die andere Richtung: die Fläche des Zettels wird dort **nicht** angemeldet, sonst tippt niemand hinein.
- **`Datei::ALLE`** ist `[Datei; 4]` in `krk-core/src/ablage/pfade.rs`, und die Probe `nur_benannte_dateien_erreichen_das_atomare_schreiben` in `krk-core/tests/baum.rs` zählt fünf Quelldateien auf. Beide halten den Bau an, wenn eine Stelle fehlt.
- **`krk-core/src/ablage/sperre.rs`**: der Schreibgriff verhindert ein Gemisch, kein Überschreiben. Zwei Instanzen, die zuletzt schließende gewinnt.
- **`resources/default-keymap.toml`**: 82 Belegungszeilen, `f2` und `cmd+k` beide unbelegt, `shift+cmd+k` liegt auf dem Kopieren.
- **Der Nutzerentscheid vom 260802-1409** zur Norton-Reihe, der das Muster „zwei Wege ab Werk, eine Zeile in der Belegungsansicht" gesetzt hat, samt der Begründung über den Touch Bar des Abnahmegeräts.

## Was geschrieben wurde

- **Circle** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/` mit dem Datensatz `_a_circle.md` und den sechs Artefaktordnern. Die Grounding-Aufnahme führt neun Abschnitte; drei davon halten Punkte fest, die der Auftrag ausdrücklich verlangt hat: der Verlust des seit dem Öffnen Getippten bei einem Absturz als benannte Folge und nicht als Lücke, die Überschreibgefahr zweier Instanzen als offen und nicht übersehen, und die Verträglichkeit von Umfang und Form.
- **Backlog-Eintrag** `shared/backlog/260813-2033_c_…` geschlossen, mit `Promoted:`-Zeile auf den Circle.

## Was nicht geschrieben wurde

Kein Spec — im anticipated-circle-Modus ist der Circle-Datensatz das Artefakt. Kein Defekt zu `CLAUDE.md`: der Auftrag hat ihn ausdrücklich ausgenommen, weil der playmaker den Befund seit dem 260813-2203 als erste Warnung im Portfolio führt und `/fusion:revise-claude-md` der vorgesehene Weg ist. Ein dritter Ort für denselben Befund wäre keine Hilfe.

Ein Vorbehalt zur Vollständigkeit des Grundes: welcher Zettel zuletzt offen war, ist ein Sitzungszustand, und der Ort dafür ist eine Vorgabe des Shapers und keine Antwort des Nutzers. Sie steht als solche gekennzeichnet in der Grounding-Aufnahme, und der Planer darf sie verwerfen.

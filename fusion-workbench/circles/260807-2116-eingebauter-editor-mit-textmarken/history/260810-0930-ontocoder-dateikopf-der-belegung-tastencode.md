# Der Dateikopf der Belegung nennt den Tastencode jetzt nur noch für die Funktionstasten

**Status:** Complete
**Domäne:** data
**Ausführender:** `ontocoder`
**Aktiver Circle:** `260807-2116-eingebauter-editor-mit-textmarken` (`_t_`)
**Grundlage:** `issues/260810-0914_o_der-dateikopf-der-belegung-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`
**Abnahme:** `cargo test --workspace` mit Rückgabewert 0. 15 Testziele, alle grün, 721 bestandene Proben.

**Ein Hinweis zur Ausstattung dieses Laufs:** `fusion-rules ontocoder` gibt `chat-voice-de.yaml` aus, aber kein `default-voice-de.yaml`. Das Chat-Profil ist gelesen und angewandt; für die Langform dieses Berichts und des Defektdatensatzes gilt die Artefaktsprache aus `CLAUDE.md`, Zeile `**Language:** de`, ohne abweichende Artefaktsprache. Derselbe Befund steht im `ontocoder`-Bericht vom 260810-0914 und im `coder`-Bericht vom 260810-0822.

---

## Was zu tun war

Der Kopf von `resources/default-keymap.toml` begründete in seinem Absatz über die fn-Taste, warum sie in keiner Kombination vorkommt, und stützte das auf den Satz "KRK belegt den Tastencode". Als allgemeine Aussage ist der erste Halbsatz seit S2 falsch. Der Nachfolgebefund zu `260810-0011`, gefunden bei jener Arbeit und dort nicht mitbehoben, weil die Schreibgrenze ausdrücklich auf die beiden Kommentarblöcke bei Zeile 484 und 625 lautete.

Die Schreibgrenze dieses Laufs lautete auf `resources/default-keymap.toml` und darin auf den Dateikopf. Keine Belegungszeile, keiner der beiden nachgezogenen Kommentarblöcke, keine Datei unter `crates/**`, weil dort parallel andere Agenten arbeiteten.

## Die Behauptung am Code geprüft

Vor dem Umschreiben gelesen, in `crates/krk-core/src/tasten/parser.rs`:

- `Taste::kennung` (Zeilen 192–198) legt jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer ASCII-Ziffer auf `Tastenkennung::Zeichen`, jeden anderen Namen auf `Tastenkennung::Code`.
- `Kombination::aus_tastendruck` (Zeilen 569–576) schlägt nach Kennung nach: über das gemeldete Zeichen, wo der Tastendruck eines trägt, und über den Code sonst. Die Stellensuche über den Code filtert dabei jede Taste aus, die selbst eine Zeichenkennung trägt. Über den Code gehen damit nur noch Funktionstasten, Pfeilblock und Steuertasten.
- Der Modulkopf (Zeilen 18–51) schreibt beide Nachschlagarten und ihre Begründung aus, samt Nutzerentscheid vom 260808-0155.

Für F3 trägt die Prämisse weiter, weil F3 eine Funktionstaste ist. Falsch war allein die Reichweite. Die Schlussfolgerung des Absatzes, dass die fn-Taste in keiner Kombination steht, bleibt damit unverändert richtig, und der Messbeleg aus `spikes/fn-tasten/messung-A.txt` trägt weiter.

## Was geändert ist

Ein Halbsatz, Zeile 42. Aus

```
# Kombination vor. KRK belegt den Tastencode, und F3 mit gehaltener fn-Taste
```

wurde

```
# Kombination vor. Funktionstasten schlaegt KRK ueber den Tastencode nach, und
```

Der Rest des Absatzes ist unverändert und nur neu umbrochen, weil der längere Satzanfang die Zeilenbreite der Datei von rund 79 Zeichen sonst überschritten hätte: Verweis auf C3, Messbeleg mit den Ereignissen #03 bis #05, Einschränkung über die Touch Bar des Referenzgeräts. Kein neuer Absatz, kein zweiter Ort für die allgemeine Regel. Die steht weiter genau einmal in dieser Datei, im Block zum eingebauten Editor bei Zeile 484, und einmal im Modulkopf von `parser.rs`, auf den jener Block verweist.

**Der Kopf ist ganz durchgesehen und nicht nur an der gemeldeten Zeile.** Zeile 42 war in den Zeilen 1 bis 97 die einzige Stelle, die einen Nachschlagweg behauptet; geprüft über eine Suche nach `Tastencode`, `Stelle`, `Zeichen` und `Aufschrift` über die ganze Datei. Der neue Wortlaut ist mit den beiden Blöcken bei Zeile 484 und 625 konsistent, ohne sie anzufassen: Block 1 sagt "Buchstaben und Ziffern werden ueber das gemeldete Zeichen nachgeschlagen, alles uebrige ueber den virtuellen Tastencode", der Kopf sagt jetzt genau die Funktionstasten-Hälfte davon.

Die Formkonventionen der Datei sind gehalten: Kommentar ohne Umlaute in der Transliteration der übrigen Kopfkommentare (`schlaegt`, `ueber`), Zeilenbreite bei rund 79 Zeichen, Klammerausdruck über vier Zeilen wie vorher.

## Abnahme

```
cargo test --workspace   → exit 0
```

15 Testziele, alle grün: 55, 140, 36, 42, 15, 26, 7, 5, 22, 16, 9, 308, 5 und 35 bestandene Proben, dazu ein Doc-Test-Ziel ohne Proben; ein `ignored`, keine Fehlschläge. Zusammen 721 bestandene Proben.

Die Belegungsdatei geht über `include_str!` in den Bau, ein Formfehler hätte den Lauf angehalten. Die Probe hinter dem korrigierten Satz ist `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte`: sie geht die ausgelieferten Kombinationen durch und prüft für jede, welche der beiden Nachschlagarten sie trägt.

## Nebenwirkungen

Der Zähler im Kopf ("71 Funktionen mit zusammen 79 Kombinationen") ist nicht betroffen, weil keine Belegungszeile angefasst ist.

**Die Suche nach "belegt den Tastencode" über den Baum hat eine vierte lebende Stelle gefunden, und sie liegt außerhalb der Schreibgrenze.** `crates/krk-core/src/tasten/parser.rs:453-456` gibt als Fehlermeldung zu `fn+f3` aus: "fn ist keine Zusatztaste einer Belegung; KRK belegt den Tastencode, und F3 mit gehaltener fn erzeugt denselben wie ein nacktes F3". Dieselbe Prämisse, dieselbe Reichweitenverletzung, und von den vier Stellen die einzige, die ein Nutzer im Betrieb sieht. Die Aussage der Meldung bleibt richtig, weil F3 eine Funktionstaste ist. Geführt als `issues/260810-0935_o_die-fehlermeldung-zur-fn-taste-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`, Schwere Low, Ausführender `coder`, weil `.rs` nicht dem `ontocoder` gehört und an `crates/**` zur selben Zeit andere Agenten arbeiteten. Kein Wortlaut-Vergleich hängt daran, geprüft über `FnAlsZusatztaste` und den Meldungstext über `crates/`: die Probe in Zeile 752 vergleicht auf die Variante, nicht auf ihren Text.

Zwei weitere Fundstellen sind kein Defekt. Der Spec der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md`, Zeilen 176 und 243) führt denselben Satz zweimal, leitet beide Male etwas über F1 ab, also über eine Funktionstaste, und ist geschlossen; das Dokument beschreibt den Stand seiner Runde, in dem die Aussage allgemein zutraf. Die übrigen Treffer sind Sitzungsprotokolle und Defektdatensätze, also historische Aufzeichnungen, die nicht nachgezogen werden.

## Was offen bleibt

Die Umbenennung des Markers `_o_` → `_c_` am Defektdatensatz macht der Nutzer; der Datensatz trägt den `Resolved:`-Abschnitt.

Der neue Defekt `260810-0935` an der Fehlermeldung ist offen und wartet auf den `coder`.

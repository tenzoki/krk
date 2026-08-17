# Schritt 2 — die Schaltflächenbeschriftung des Blattes wird zum Argument

**Datum:** 260817-1104
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Bündel A, Schritt 2
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C2 und C3

---

## Was umgesetzt ist

`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs`

- `zeigen` nimmt zwei Argumente mehr: `schaltflaeche: &str` als Beschriftung der zweiten Schaltfläche und `laut: bool`. Beide stehen zwischen `erlaeuterung` und `fertig`, damit der Abschluss letztes Argument bleibt.
- `als_warnung()` steht jetzt in einem `if laut`. Ohne `laut` bleibt das Blatt ruhig und trägt kein Warnzeichen.
- Die erste Schaltfläche bleibt wörtlich `Schaltflaeche::neu("Abbrechen", Taste::Eingabe)`, die zweite behält `Taste::EingabeMitBefehl`, und die Reihenfolge im Feld ist unverändert. Die Vorbelegung auf „Abbrechen" ist nicht angefasst.
- Der Modulkopf ist umgeschrieben. Sein Gegenstand ist die eine Rückfrage vor dem Räumen in den Papierkorb, in ruhiger und in lauter Form; der Verweis auf die Texte zeigt auf `kommandos::loeschwarnung::frage_und_erlaeuterung` statt auf `operationen::loeschfrage`. Neu sind ein Abschnitt „Ruhig und laut sind dasselbe Blatt" mit der Skizze der beiden Formen und ein Absatz darüber, warum die Beschriftung der zweiten Schaltfläche als Argument hereinkommt und „Abbrechen" als einziger Wortlaut hier stehen bleibt.
- Der Abschnitt „Vorbelegt ist Abbrechen" steht **wörtlich** unverändert, samt dem Zitat aus C4 der Runde 1. Darunter steht ein neuer Absatz, dass dieselbe Forderung als Abnahmekriterium in C2 dieser Runde steht und dort für beide Formen gilt.
- Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` steht unverändert und stimmt weiter: die Datei spricht keine neue Klasse an, `NSWindow` bleibt die einzige.
- Der Verweis am Ende des Absatzes „Der Weg dahin ist bindend" zeigt auf `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md` statt auf den Datensatz von 260802, den diese Runde überholt.

`crates/krk-ui/src/appkit/anwendung.rs`

- Genau eine Zeile mehr am einzigen heutigen Aufrufer, `Anwendungsdelegierter::endgueltig_loeschen` (Zeile 4537): `"Endgültig löschen"` und `true` gehen jetzt als Argumente mit. Das Verhalten ist unverändert — dieselbe Beschriftung, dasselbe Warnzeichen, dieselbe Vorbelegung. Sonst ist in dieser Datei nichts angefasst; von Schritt 3 ist nichts vorgebaut.

## Eine Beobachtung für Schritt 3

Der Hinweistext, den `zeigen` an die Erläuterung anhängt, lautet unverändert „Return und Esc brechen ab. Zum Löschen Cmd+Return." Nach Schritt 3 steht er auch unter der Frage „Diese N Einträge in den Papierkorb räumen?", wo „Löschen" nicht ganz das trifft, was die zweite Schaltfläche tut. Der Plan sagt zu diesem Satz nichts, und dieser Schritt hat ihn deshalb nicht angefasst. Wer ihn ändern will, tut es an dieser einen Stelle.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, Proben, Clippy unter `-D warnings`, Formatprüfung.

## Was dieser Schritt nicht baut

Keinen zweiten Aufrufer, keinen gemeinsamen Rumpf, keine Rückfrage für `delete`. Der Nutzer ist nach diesem Schritt noch nicht geschützt; das leistet erst Schritt 3.

# Schritt 1 — das Modul der Löschfrage, mit der ruhigen Form

**Datum:** 260817-1028
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Bündel A, Schritt 1
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C2

---

## Was umgesetzt ist

`crates/krk-ui/src/kommandos/loeschwarnung.rs` (neu)

- `frage_und_erlaeuterung(auswahl: &Auswahl, ordner: &Path) -> (String, String)` in der ruhigen Form. Die Frage lautet bei einem Eintrag „Diesen Eintrag in den Papierkorb räumen?" und sonst „Diese N Einträge in den Papierkorb räumen?", mit der Tausendertrennung der Oberfläche über `operationen::zahl`. Die Erläuterung lautet „Geräumt wird aus <Pfad>." und hängt bei Ordnern in der Auswahl einen zweiten Absatz an: „Darunter N Ordner, jeweils mit ihrem gesamten Inhalt."
- Keine `use objc2`-Zeile, wie im ganzen Verzeichnis.
- Der Modulkopf trägt vier Abschnitte: den Gegenstand des Moduls, warum die Texte der Löschfrage eigens dastehen und nicht im Blatt, **warum der Pfad ungekürzt dasteht** (der Schadensfall vom 260817-0344 mit seinen 189 Dateien, und dass ein `~` genau die Zeichen spart, die zwei Orte unterscheiden), sowie die Aufruferzusage auf `Anwendungsdelegierter::loeschen_nach_rueckfrage` samt dem Hinweis, dass die Zählprobe dazu mit der Tafel der Auslöser in Schritt 10 kommt.
- Der Pfad entsteht über `operationen::pfadtext` und nicht über einen zweiten Formatierer. Diese Funktion hat die Entscheidung gegen `pfade::gekuerzt_fuer_anzeige` schon für die beiden Pfadkopierer getroffen und in ihrem Doc-Kommentar begründet; ein zweiter Formatierer daneben wäre die erste Abweichung, die niemand prüft.
- Fünf Proben: Einzahl, Mehrzahl (2 und 1.234, letztere für den Tausenderpunkt), voller Pfad in der Erläuterung samt Gegenprobe auf die Tilde, Ordnerzahl gesondert, und die Gegenprobe ohne Ordner.

`crates/krk-ui/src/kommandos/mod.rs`

- `pub mod loeschwarnung;`, dazu die Zeile im Modulkopfbild. „Sieben Module" wird zu acht, „vor den sechs anderen" zu sieben, „vor den fünf übrigen" zu sechs.

`crates/krk-ui/src/kommandos/operationen.rs`

- Nur die Doc-Kommentare von `zahl` und `ordner_text`: beide nennen `loeschwarnung` als neuen Abnehmer, und der von `zahl` schreibt aus, warum die Sichtbarkeit `pub(crate)` bleibt.

## Zwei Abweichungen vom Plan, beide begründet

**Die Sichtbarkeit bleibt `pub(crate)` statt `pub(super)`.** Der Plan liest `zahl` und `ordner_text` als privat; am Baum sind beide bereits `pub(crate)`, und `zahl` hat mit `crate::appkit::statuszeile:177` einen Aufrufer außerhalb von `kommandos`. `pub(super)` übersetzte dort nicht und verlangte eine vierte Datei, die der Schritt nicht anfassen soll. Die Wiederverwendung, um die es dem Schritt geht, steht ohne jede Änderung. Nur `ordner_text` zu verengen und `zahl` nicht wäre eine Ungleichheit ohne Gewinn.

**`frage_und_erlaeuterung` trägt bis Schritt 3 eine Ausnahme.** `krk-ui` ist ein Binärziel, in dem `pub` allein keine Verwendung ist; ohne Aufrufer hält `-D warnings` den Bau mit `dead_code` an. Die Funktion trägt deshalb `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]` nach dem Vorbild, das `kommandos/rueckschritt.rs` in der Runde 10 getragen hat. `expect` und nicht `allow`: mit dem Aufrufer aus Schritt 3 wird die Erwartung unerfüllt und der Bau hält an, bis die Zeilen weg sind. Der Modulkopf sagt beides, und Schritt 3 muss die Zeilen entfernen.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, Proben, Clippy unter `-D warnings`, Formatprüfung. Die fünf neuen Proben laufen unter `cargo test -p krk-ui loeschwarnung`, alle grün.

## Was dieser Schritt nicht baut

Keine laute Form, keine Zielprüfung, keine Zählung des Unterbaums, keine Papierkorbfrage. Das Blatt kennt die neue Funktion noch nicht; der Nutzer ist nach diesem Schritt noch nicht geschützt, sondern erst nach Schritt 3.

# Planungssitzung: die Bereichsleiste und die proportionale Breitenregel

**Status:** Complete
**Agent:** planner
**Circle:** `circles/260811-1304-statusleiste-mit-bereichsschaltern`
**Dispatch:** Nutzer, direkt ("Erstelle den Implementierungsplan für den aktiven Circle")

## Was entstanden ist

| Datei | Was sie trägt |
|---|---|
| `planning/260812-0415_o_bereichsleiste-und-proportionale-breitenregel.md` | der Plan: sieben Fähigkeiten mit Abnahmekriterien, acht Implementierungsschritte, drei Mermaid-Diagramme |
| `decisions/260812-0415_o_welche-kombinationen-bekommen-die-beiden-neuen-umschalter.md` | offene Nutzerfrage zur Belegung, Empfehlung Möglichkeit 1 |
| `decisions/260812-0415_o_was-tut-der-editorschalter-ohne-datei-im-editor.md` | offene Nutzerfrage, Empfehlung: ohne Meldung verwerfen |
| `decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md` | offene Nutzerfrage, Empfehlung: alle mit demselben Faktor schrumpfen |
| `issues/260812-0415_o_die-spalten-und-die-sortierschluessel-sind-zwei-aufzaehlungen-derselben-vier-dinge.md` | Defekt, in dieser Runde nicht behoben |

## Gelesene Grundlage

`CLAUDE.md`, der Circle-Datensatz `_t_circle.md`, alle elf Datensätze unter `decisions/`, der
Bericht `history/260812-0306-klaerungsrunde.md` und der Nachtrag
`issues/260811-1732_p_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`.
Am Baum gelesen: `fenstermodell.rs`, `appkit/aufteilung.rs`, `appkit/fenster.rs`,
`appkit/anwendung.rs` (Aufbau, Kommandoweg, Fokus, Nachzüge), `appkit/ereignisse.rs`,
`appkit/statuszeile.rs`, `appkit/tabelle.rs`, `appkit/belegungsansicht.rs`,
`kommandos/fokus.rs`, `belegungsmodell.rs`, `krk-core/src/ablage/sitzung.rs`,
`krk-core/src/tasten/belegung.rs`, `krk-core/tests/ablage.rs`, `resources/default-keymap.toml`,
`krk-bench/src/messen.rs` und `krk-bench/src/bericht.rs`.

## Drei Befunde, die den Zuschnitt bestimmt haben

**Die Sonderregel in `breiten_uebernehmen` reicht unter einer Anteilsregel nicht mehr.** Sie
lässt die beiden Dateifenster unangetastet, solange nur eines sichtbar ist. Unter einer
Anteilsregel bläht jedes Ausblenden alle übrigen Bereiche auf, und der Anteil des
ausgeblendeten verfiele bei jedem Nachlesen. Der Plan ersetzt die Sonderregel durch eine
Rückrechnung auf die gespeicherte Summe; damit fällt die Sonderregel weg und der Fall aus dem
Defekt vom 260804 wird ein Ergebnis der Regel statt einer Ausnahme.

**Die Abweisung an den Mindestbreiten ist ohne die Fenstergeometrie nicht entscheidbar.**
`Fenstermodell::umschalten` kennt heute weder die Fensterbreite noch die Breite einer
Trennlinie. Der Plan gibt ihm beides als Wert (`Zeilenmass`), statt aus der zuletzt
ausgelegten Breite zu schätzen. Daneben hat der Fall einen zweiten Eingang, den die
beantwortete sechste Frage nicht abdeckt: das Zusammenziehen des Fensters lässt sich nicht
abweisen. Dafür der neue Entscheidungsdatensatz.

**Die Leiste braucht keinen sechsten Fokuswert.** `ersthelferbereich` läuft über
`Bereich::ALLE` und fällt für einen Ersthelfer außerhalb der fünf Teilbäume auf
`Fokus::Dateifenster` zurück. Nähme ein Schalter den Rang an, zeigte der Fokusrahmen auf ein
Dateifenster, während die Tasten beim Schalter ankämen. Der Plan schließt den Fall aus, statt
ihn zu behandeln: die Schalter verweigern den Ersthelferrang.

## Was der Plan nicht entscheidet

Die drei offenen Datensätze oben. Keiner hält die Ausführung auf; der Plan setzt jeweils die
Empfehlung um und nennt je Frage, was eine andere Antwort kostet.

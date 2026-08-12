# Shaper-Lauf: vorgesehener Circle für Teilen, Ordnersprung, Ablagesicherung und gerenderte Vorschau

**Datum:** 2026-08-12, 10:00
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete
**Ergebnis:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/`

---

## Auftrag

Aus vier Wünschen des Nutzers vom 260812-0930 einen vorgesehenen Circle anlegen. Die Klärungsrunde war bereits gefahren: ein vorheriger Shaper-Lauf hatte vier Fragen vorgelegt, der Nutzer hatte alle vier beantwortet, und die Anweisung lautete ausdrücklich, ihn nicht erneut zu fragen. Kein Circle war aktiv, und keiner wurde aktiviert.

Die vier Wünsche im Wortlaut: Teilen über Tastenkombination und rechte Maustaste; ein Befehl, der im aktiven Dateifenster den Ordner der angezeigten Datei öffnet; die Zusicherung, dass die Lesezeichen ein Update der Anwendung überstehen; eine gerenderte Markdown-Vorschau mit formatiertem Quelltext. Der Nutzer hat entschieden, alle vier in einer Runde zu fahren.

## Die vier Antworten des Nutzers, als Festlegung übernommen

Sie stehen im Circle-Datensatz unter `### Die vier Festlegungen des Nutzers` und sind dort als Festlegung ausgeschrieben, nicht als Möglichkeit.

**A** Markdown wird voll gerendert, ohne Web-Ansicht; Quelltext läuft über die vorhandene `hervorhebung.rs` samt `syntect`. **B** Der Text erscheint sofort, die Farben ziehen nach; L7 bleibt unangetastet, hervorgehoben wird unabhängig von der Dateigröße. **C** Teilen wirkt in Dateiliste, Editor und Vorschau, über einen Mechanismus für den Begriff „die angezeigte Datei", und das Kontextmenü trägt zunächst einen Eintrag. **D** Eine beschädigte Ablagedatei wird zur Seite gelegt statt überschrieben, für alle vier Dateien unter `~/Library/Application Support/KRK/`.

## Was der Lauf am Baum erhoben hat

Der Grounding snapshot ist am Code erhoben und nennt Datei und Zeile. Vier Befunde haben den Zuschnitt verändert:

**Wunsch 3 ist zum größten Teil schon erfüllt, aber am falschen Punkt gesucht.** Die Lesezeichen liegen außerhalb des Bündels (`crates/krk-core/src/ablage/pfade.rs:79`), und `crates/krk-core/tests/ablage.rs:159` nagelt den Ort samt allen vier Dateinamen fest. Der echte Verlustweg steht im Modulkopf von `crates/krk-core/src/ablage/mod.rs:88-93`: eine beschädigte Datei bleibt zwar stehen, wird aber „beim nächsten gewöhnlichen Schreibvorgang" überschrieben. Eine Sicherungskopie gibt es nicht; `atomar.rs` schreibt in eine Nachbardatei, die niemandes Leseziel ist.

**Wunsch 2 ist billiger als erwartet, und der vorige Lauf hat den Grund nur halb gesehen.** Er hatte `Tabliste::auswahl_auf_namen` als Wiederverwendungspunkt genannt. Das trifft nur den Fall, in dem der Zielordner schon angezeigt wird. Der eigentliche Weg ist `Dateifenster::ordner_lesen(pfad, auswahl)` (`crates/krk-ui/src/appkit/tabelle.rs:628`), das an `Tabliste::ordner_setzen` (`crates/krk-ui/src/tabs.rs:508`) durchreicht und den Namen des auszuwählenden Eintrags als zweiten Parameter bereits führt. Der Aufstieg aus C2 und der Sprung aus C10 benutzen ihn schon. Der Ordnersprung ist damit ein dritter Aufrufer und kein neuer Mechanismus. Der Circle-Datensatz ist auf diesen Befund korrigiert.

**Wunsch 1 hat keinen einzigen Anknüpfungspunkt.** `NSSharingServicePicker` kommt im Baum nicht vor, und ein `menuForEvent:` steht an keiner Stelle unter `crates/krk-ui/src/appkit/`. KRK hat heute kein eigenes Kontextmenü; das im Editor gehört AppKit.

**Wunsch 4 fasst die Dreiteilung aus C6 zweimal an und trifft dabei zwei Zusagen der Runde 2.** Die Vorschau trägt Text heute als nackte Zeichenkette (`Inhalt::Text(String)`, `crates/krk-ui/src/vorschaumodell.rs:190`) an einer `NSTextView`, die weder bearbeitbar noch auswählbar ist (`appkit/vorschau.rs:574-575`). Zwei Berührungen sind neu aufgefallen und wären ohne die Erhebung erst im Plan sichtbar geworden: die Zeilennummernspalte ist **eine** Klasse für Editor und Vorschau, und ihre Zahlen stimmen neben gerendertem Markdown nicht mehr; und die Unauswählbarkeit der Vorschaufläche ist die Bedingung dafür, dass die vier Tabbefehle aus C1 in ihr wirken.

## Dreizehn offene Fragen abgelegt

Jede als eigener offener Datensatz in `decisions/` des neuen Circles, mit Möglichkeiten und den Folgen, die jede am Code auslöst. Keine ist ein Absatz im Circle-Datensatz; dort steht allein die Übersicht.

Vier davon binden über den Circle hinaus. Die Frage nach der Mindestbreite der Vorschau bindet den vorgesehenen Web-Betrachter-Circle unmittelbar, weil dessen rund 17 Punkte Luft oberhalb der heutigen 160 dieselben sind. Die Frage nach lokalen HTML-Dateien ist die zweite offene Frage jenes Circles, hierher vorgezogen. Die Frage nach dem Link im gerenderten Markdown berührt seine erste offene Frage und C9 der Runde 1. Die Frage nach den Tastenkombinationen berührt den knappen Vorrat der Auslieferungsbelegung, 79 Funktionen mit 85 Kombinationen.

## Zum Zuschnitt: was am laufenden Bündel bleibt

Der Circle-Datensatz trägt dafür einen eigenen Abschnitt mit einer Tabelle. Alle fünf gefahrenen Runden sind beschränkt abgeschlossen, weil der Abnahmelauf KRK im Vordergrund verlangt und damit Nutzerarbeit ist. Der Schnitt ist so gelegt, dass die Auswahlmenge beim Teilen, der Zielordner und der vorzumerkende Name beim Ordnersprung, das Zur-Seite-Legen samt Kollisionsfall und die Zerlegung von Markdown je ohne Bündel prüfbar sind. Was am laufenden Bündel bleibt, ist die sichtbare Wirkung: der Freigabedialog, der angezeigte Ordner, die Statuszeile und das Aussehen der Vorschaufläche.

## Was der Lauf nicht getan hat

Kein Spec geschrieben, keinen Plan, keine Aktivierung. Kein bestehender Circle angefasst, insbesondere nicht der des Web-Betrachters: die Kante zu ihm ist im neuen Datensatz beschrieben, sein eigener Abschnitt `## Dependencies` bleibt unverändert. Kein Code, keine Daten, keine Ontologie berührt. Die Aktivierung ist der nächste Schritt des Nutzers, über `/fusion:next`.

## Dateien

- `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_a_circle.md`
- `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/` — dreizehn offene Datensätze
- Die sechs Artefaktordner `planning/`, `issues/`, `decisions/`, `history/`, `reviews/`, `analyses/` sind angelegt.

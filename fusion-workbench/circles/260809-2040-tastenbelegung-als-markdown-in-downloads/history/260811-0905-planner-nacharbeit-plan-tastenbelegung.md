# Planner: Nacharbeit am Plan der Tastenbelegungs-Ausgabe

**Datum:** 2026-08-11 09:05
**Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads`
**Auftrag:** eng begrenzte Nacharbeit. Der Nutzer hat den Plan am 260811-0900 abgenommen, mit zwei Auflagen: die Pfadfrage ist entschieden und in S3 nachzuziehen, und zwei Befunde der Bewertung `reviews/260811-0853-conceptrev-plan-tastenbelegung-als-markdown-in-downloads.md` (Spruch `acceptable`) sind an den Schaubildern nachzuziehen.
**Angefasst:** allein `planning/260811-0838_o_plan-tastenbelegung-als-markdown-in-downloads.md`. Marker bleibt `_o_`. Kein Code, kein Circle-Datensatz, kein Spec, kein Commit.

## Auflage 1: die Kürzung mit Tilde

Der Nutzer hat Möglichkeit 2 gewählt, gegen die Empfehlung des Plans. Der Datensatz `decisions/260811-0838_a_schreibt-krk-einen-pfad-fuer-den-nutzer-je-gekuerzt.md` trägt die Antwort samt Folgen; der Plan zitiert ihn und wiederholt ihn nicht.

Frage 8 ist neu geschrieben und trägt jetzt drei Punkte statt einer Empfehlung. Die Kürzung bekommt eine eigene reine Funktion, `pub fn gekuerzt_fuer_anzeige(pfad: &Path, benutzerverzeichnis: Option<&Path>) -> String`, und sie wohnt in `crates/krk-core/src/ablage/pfade.rs` neben `benutzerverzeichnis()`. Der tragende Grund für den Kern ist nicht das Vorbild, sondern die Reichweite: `krk-ui` hat kein Bibliotheksziel, und eine Kürzung in `belegungsausgabe.rs` wäre für die nächste Fläche, die einen Pfad meldet, unerreichbar. Sie schriebe die Funktion dann ab. Das Benutzerverzeichnis kommt als Argument herein, damit die Funktion ohne Zugriff auf das echte Verzeichnis prüfbar bleibt; denselben Grund nennt der Modulkopf von `pfade.rs` bereits für `Ablageort`.

Die Regel für einen Pfad außerhalb des Benutzerverzeichnisses lautet: ausgeschrieben, unverändert. Sie gilt auch, wenn kein Benutzerverzeichnis übergeben wird. Die Funktion ist total und kennt vier Fälle, und der Vergleich läuft über `Path::strip_prefix` statt über eine Zeichenkette, weil ein Vergleich auf Bytes aus `/Users/kai-alt/Downloads` gegen `/Users/kai` die Antwort `~-alt/Downloads` machte. Dieser Fall steht als eigene Zusicherung in der Probe.

`crates/krk-ui/src/fenstertitel.rs` steht in S3 auf der Verbotsseite. Der Titelbalken schreibt weiter aus, die Statuszeile kürzt, und die Ungleichheit ist im Plan und im Entscheidungsdatensatz benannt statt stillschweigend hingenommen.

An S3 sind vier Stellen nachgezogen: die Dateiliste um `pfade.rs`, `crates/krk-core/tests/ablage.rs` und `fenstertitel.rs` auf der Verbotsseite; Teil b) um das Stück im Kern und um die Zusage, dass `Ausgang::meldung` jeden Pfad kürzt, während der Ausgang selbst den ungekürzten hält; das Abnahmekriterium um Punkt 11 und um die Probe der fünf Fälle; und die Diff-Prüfung um `fenstertitel.rs`. Daneben sind der Kopf (fünf `_a_`-Datensätze werden sechs), zwei Zeilen der Risikotabelle, der Eintrag unter `## Angelegte Datensätze` und die erste offene Frage nachgezogen.

## Auflage 2: die beiden Diagrammbefunde

**Befund 1, das Aufbaubild.** Es bekommt den Quellknoten `MESS` für den in S1 gemessenen Wert, als Parallelogramm und außerhalb der drei Kisten, weil sein Wert aus keiner von ihnen kommt. Die Kante nach `AUS` trägt "Spalte 3, ohne Kommando, 6 von 71"; die bestehende Kante nach `KMD` heißt jetzt "Spalte 3, mit Kommando, 65 von 71". Die dritte Spalte hat damit im Bild zwei Lieferanten, und die Zahlen stehen an beiden Kanten. In der Beschriftung von `AUS` ersetzt `wirkung` das `meldung`, das an `Ausgang` hängt und dort genannt ist. Die Prosa unter dem Bild nennt jetzt drei tragende Kanten statt zwei.

**Befund 2, der Entscheidungsknoten `P`.** `M` heißt jetzt "dieser eine der sechs vom Menue zugestellten Textbefehle", `P` fragt nach dem Selektor "dieses einen Befehls", und beide Zweige sagen "für diesen Befehl". Der Bruchzweig nennt die `match`-Verzweigung über die Kennungen ausdrücklich. Damit liest sich das Bild je Befehl, wie S3 b) es baut, und nicht mehr als Alles-oder-nichts über die Gruppe. `F` war schon vorher singulär; `M` war die Stelle, an der die Granularität kippte.

Die Befunde 3 und 4 der Bewertung sind kosmetisch beziehungsweise geringfügig und standen nicht im Auftrag. Der Zeitpunkt von `P` ist trotzdem in einem Satz unter dem zweiten Bild benannt, weil die Prosa dort ohnehin neu geschrieben wurde.

**Kein Befund liegt am Entwurf.** Beide Male sagte der Text das Richtige und das Bild etwas anderes. Der Entwurf entscheidet je Befehl und hat zwei Quellen für die dritte Spalte; die Bilder haben es verschwiegen.

## Maß

Erstes Schaubild: 14 Knoten und 13 Kanten vor dem Nachzug, 15 Knoten und 14 Kanten danach. Größter Ausgang unverändert 5 an `AUS`, größter Eingang jetzt 2 an `AUS` und an `GLI`. Zyklen 0, Waisen 0, Teilgraphen 3.

Zweites Schaubild: 9 Knoten und 10 Kanten vor wie nach dem Nachzug. Geändert haben sich vier Beschriftungen, keine Kante und kein Knoten.

Drittes Schaubild unberührt: 4 Knoten, 3 Kanten.

Alle drei Blöcke sind mit mermaid-cli 11 nach SVG und PNG gerendert und angesehen. Die Schichtung des ersten Bildes läuft weiterhin in eine Richtung, von `krk-ui/src/appkit` über die Modelle neben `appkit` in den Kern; `MESS` steht auf der Höhe der obersten Schicht und läuft nicht gegen die Ordnung.

## Was der Nutzer noch entscheiden kann

Die Statuszeile des Plankopfs sagt weiterhin "Entwurf, wartet auf die Abnahme des Nutzers". Die Abnahme ist am 260811-0900 erfolgt; die Zeile ist damit veraltet. Sie zu ändern stand nicht im Auftrag, und der Auftrag war ausdrücklich auf zwei Punkte begrenzt, deshalb steht sie unverändert.

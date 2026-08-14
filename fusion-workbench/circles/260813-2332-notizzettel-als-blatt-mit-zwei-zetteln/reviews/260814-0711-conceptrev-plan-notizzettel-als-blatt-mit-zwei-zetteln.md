# Concept Evaluation: Plan Notizzettel als Blatt mit zwei Zetteln

**Date:** 2026-08-14 07:11
**Target:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`
**Verdict:** acceptable
**Diagrams evaluated:** 3  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0, alle drei Blöcke nach SVG gerendert)
**Vorgänger:** `reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`

## Verdict

**Der Plan macht den Fehler nicht zum vierten Mal.** Seine einzige Entscheidungsraute, `BEF` in Bild 1, trägt vier ausgehende Kanten, und der Absatz darunter behauptet die Vollständigkeit ausdrücklich. Die zwei anderen Bilder kommen ohne Raute aus. Gemessen sind daneben: kein Zyklus in dreien von drei Graphen, kein Gott-Knoten (höchster Ausgangsgrad 4, und das ist die Raute selbst), kein freistehender Knoten, in allen drei Fällen der Diagrammtyp aus der Typentabelle, in allen drei Fällen sichtbare Schichtung über `subgraph`. Bild 3, der Arbeitsgraph, deckt sich Kante für Kante mit den sechzehn `Dependencies:`-Zeilen der Schrittliste, alle neunzehn; ein Arbeitsgraph, der seiner eigenen Schrittliste widerspricht, ist der häufigste Befund an dieser Stelle und liegt hier nicht vor.

Von *clean* auf *acceptable* ziehen zwei Befunde, und beide sind Auszählungen und keine Strukturfehler. Der erste steht innerhalb des Plans: der Knoten `M2` in Bild 2 zählt zwei Wege in den Abschlussblock auf, wo Schritt 11 desselben Dokuments drei nennt. Der zweite steht zwischen Plan und Spec: Bild 1 des Spec hängt das Sichern beim `Esc`-Weg an den Wächter und zeichnet es **vor** dem Schließen des Blattes, während der Plan es an den Abschlussblock hängt, also hinter das Schließen. Kein Abnahmekriterium bindet diese Reihenfolge, und die am Baum belegte Fassung ist die des Plans. Der Widerspruch bleibt trotzdem einer: zwei Bilder derselben Runde zeichnen denselben Weg verschieden.

## Per-diagram measurements

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Teilgraphen | Waisen | Urteil |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | flowchart TD | 11 | 14 | 1,27 | 4 (`BEF`) | 3 (`AB`) | 0 | 3 | 0 | acceptable |
| 2 | flowchart LR | 8 | 7 | 0,88 | 1 (alle) | 3 (`DG`) | 0 | 3 | 0 | acceptable |
| 3 | flowchart TD | 16 | 19 | 1,19 | 3 (`S3`, `S4`) | 4 (`S12`) | 0 | 6 | 0 | clean |

Bild 1 hat eine Quelle (`OEFF`) und vier Senken, Bild 2 vier Quellen und eine Senke, Bild 3 fünf Quellen und vier Senken. In Bild 2 verteilt sich der Ausgangsgrad gleichmäßig auf 1 je Knoten; ein Gott-Knoten kann bei dieser Verteilung nicht vorliegen, und der Eingangsgrad 3 an `DG` ist genau die Aussage des Bildes, nämlich drei eigene Durchgänge gegen einen bestehenden. In Bild 3 ist der Eingangsgrad 4 an `S12` der benannte Engpass und keine Verdichtung.

Sieben der vierzehn Kanten in Bild 1 tragen eine Beschriftung, in Bild 2 vier von sieben, in Bild 3 keine. Der Verzicht in Bild 3 ist richtig: in einem Abhängigkeitsgraphen der Arbeit trägt jede Kante dieselbe Relation, und eine Beschriftung „geht voraus" neunzehnmal wäre Rauschen.

## Findings

**B1 (mittel, Bild 2): `M2` zählt zwei Eingänge auf, wo Schritt 11 drei nennt.** Der Knoten trägt die Beschriftung „Abschlussblock des Blattes, Esc ueber den Waechter, Griff ueber Abbrechen". Schritt 11 des Plans sagt für dieselbe Stelle: „der Abschlussblock ist die eine Stelle, an der das Schließen des Blattes ankommt, gleich ob es über den Wächter, über die Schaltfläche oder über den Griff kam." Die Schaltfläche fehlt im Bild, und sie ist keine Erfindung des Lesers: derselbe Schritt baut das Blatt über `Blatt::mit_schaltflaechen` „mit einer Schaltfläche", und jede so angelegte Schaltfläche bekommt in `crates/krk-ui/src/appkit/blaetter/mod.rs:400-407` einen Rückgabewert, der den Abschlussblock erreicht.

Der Befund gehört in die Familie der drei vorangegangenen, unterscheidet sich aber in Form und Gewicht. Er steht in einer Beschriftung und nicht an einer Raute, und alle drei Wege laufen ohnehin in denselben Knoten; am Entwurf ändert die Auslassung nichts, weil das Sichern in allen drei Fällen stattfindet. Was sie ändert, ist die Belastbarkeit des Bildes als Beleg: wer aus `M2` die Eingänge zählt, zählt zu niedrig. Die Behebung ist ein Wort in der Beschriftung.

**B2 (mittel, Bild 2 des Plans gegen Bild 1 des Spec): Der `Esc`-Weg sichert in den zwei Dokumenten an verschiedenen Stellen und in verschiedener Reihenfolge.** Bild 1 des Spec zeichnet die Kette `WA --> SI --> ZU`, also Wächter, dann „sichern, wenn geaendert", dann „das Blatt schliesst". Der Plan setzt an dieselbe Stelle den Knoten `M2` und begründet ihn in seiner Ausgangslage: „beide Wege heraus, die Escape-Taste über den Wächter und ein Abbruch über den Griff, laufen in **denselben** Abschlussblock von `Blatt::zeigen_mit_wahl`. Das Sichern hängt deshalb am Abschlussblock und nicht am Wächter."

Am Baum gelesen, nicht aus der Prosa übernommen: der Wächterweg in `zeigen_mit_wahl` ruft `elternfenster.endSheet_returnCode(&blattfenster, antwort)` (`crates/krk-ui/src/appkit/blaetter/mod.rs:562-569`), und der Abschlussblock ist der Rückrufblock aus `beginSheetModalForWindow_completionHandler` (`:540-555`). `inference:` AppKit ruft diesen Rückruf nach dem Abräumen des Blattes, so steht es in seiner Zusage zu `beginSheetModalForWindow:completionHandler:`; in diesem Baum gemessen ist es nicht. Unter dieser Zusage kehrt sich die Reihenfolge aus Bild 1 des Spec um: das Blatt schließt, danach sichert der Abschlussblock.

Der Widerspruch wiegt leichter als der vom 260814-0000 und soll nicht schwerer gemacht werden, als er ist. Keines der Abnahmekriterien bindet die Reihenfolge auf dem `Esc`-Weg; gebunden ist allein die Reihenfolge an `fenster_schliessen`, und die zeichnen beide Dokumente gleich. Die Zusage aus C4, dass kein Weg aus dem Zettel heraus Text verliert, hält in beiden Lesarten. Auflösen lässt sich der Befund an einer Stelle und ohne einen Knoten zu verschieben: `SI` und `ZU` in Bild 1 des Spec tauschen die Plätze, und `SI` bekommt die Beschriftung des Abschlussblocks. Die Fassung des Plans ist die belegte, also ist der Spec die nachzuziehende Seite. Eine Randbemerkung dazu, die dem Planer und nicht der Zeichnung gehört: dass der Zettelwächter denselben Abschlussblock erreicht, ist eine Verdrahtung, die Schritt 11 behauptet und nicht ausschreibt, denn `zeigen_mit_wahl` verbindet heute den `Eingabewaechter` und nicht einen `NSTextViewDelegate`.

**B3 (niedrig, Bild 3): Die Prosa unter dem Arbeitsgraphen zählt drei Stränge, der Graph zeigt vier.** Der Satz lautet: „Der Engpass ist Schritt 12: er hängt an vier Vorgängern aus drei Strängen." Die vier Kanten nach `S12` kommen aus `S4` (Strang A), `S7` (Strang B), `S11` (Strang C) und `S15` (Strang E). Das sind vier Teilgraphen, und der Graph selbst zeichnet sie als vier. Die Zahl der Vorgänger stimmt, die der Stränge nicht.

**B4 (niedrig, Bild 1): Die vier Ausgänge sind vollständig, überschneidungsfrei sind sie nur über eine Reihenfolge, die das Bild nicht zeigt.** Der Absatz darunter sagt „Vier Ausgänge, überschneidungsfrei und vollständig". Eine Datei über `EDITORGRENZE`, deren Bytes zugleich kein gültiges UTF-8 sind, trägt zwei der vier Beschriftungen. Der Mechanismus entscheidet den Fall, weil die Größe am `fstat` vor dem Lesen der Bytes geprüft wird, so wie es Schritt 1 beschreibt; das Bild zeichnet diese Vorrangigkeit nicht. Eine Nummer vor jeder der vier Beschriftungen stellt sie her und kostet vier Zeichen. Der Befund steht niedrig, weil die Vollständigkeit gegeben ist und der Übersetzer die Auszählung an beiden Übersetzungen erzwingt.

**B5 (niedrig, Bild 2): Der Schreibweg hinter `zettel_sichern` ist unbedingt gezeichnet, die Bedingung steht nur im Knoten.** `ZS --> TS --> AT` trägt keine Beschriftung, während `ZS` selbst „schreibt nur bei Aenderung" sagt. Wer die Kanten liest und den Knotentext überspringt, zählt einen Schreibvorgang je Moment. Der Befund ist die halb behobene Fassung von N4 der Prüfung vom 260814-0000: die Bedingung ist im Bild angekommen, sie steht nur an der falschen Stelle. „nur bei Aenderung" an der Kante `ZS --> TS` schließt die Lücke.

**B6 (niedrig, außerhalb der Diagrammprüfung): eine Zahl in der Ausgangslage.** Der Absatz beginnt mit „**`Datei::ALLE` bleibt bei sechs Einträgen und wächst nicht auf acht.**" Am Baum steht heute `pub const ALLE: [Datei; 4]` (`crates/krk-core/src/ablage/pfade.rs:60`), und Schritt 3 sagt es richtig: „`Datei::ALLE` wird `[Datei; 6]`". Die Aufzählung wächst also von vier auf sechs. Der Punkt berührt kein Bild und steht hier, weil dieselbe Lesung ihn hervorgebracht hat.

**B7 (niedrig, Bild 2): Der Satz, der die einzige unsymmetrische Kante erklärt, ist verstellt.** Er lautet: „Die vierte Kante läuft am Durchgang vorbei und nicht daran vorbei am Schreibgriff". Gemeint ist erkennbar, dass sie den Durchgang auslässt und nicht den Schreibgriff. Die Kante `M4 --> ZS` ist die eine Stelle, an der Bild 2 von seiner eigenen Form abweicht, und ihre Begründung sollte in einem Zug lesbar sein.

**Was der Plan an seinen Bildern richtig macht und hier genannt gehört.** Der Arbeitsgraph deckt sich vollständig mit der Schrittliste: sechzehn Knoten für sechzehn Schritte, neunzehn Kanten für die neunzehn Einträge in den `Dependencies:`-Zeilen, keine Kante ohne Entsprechung und keine Abhängigkeit ohne Kante. Nachgeprüft sind auch die zwei Aussagen der Prosa über die Nebenläufigkeit: zwischen den Strängen A und B und dem Schritt 9 verläuft keine Kante, sie sind also tatsächlich unabhängig. Die drei Teilgraphen in Bild 1 tragen die These des Bildes, nämlich ein Befund und zwei Übersetzungen, und sie sind der Grund, aus dem elf Knoten mit vierzehn Kanten nicht als Knäuel erscheinen. Die vier Momente aus Bild 2 decken sich eins zu eins mit den vier Kanten aus Bild 2 des Spec, die ein Sichern zusagen: Tabklick, `Esc`, `shift+cmd+w` und `cmd+q`. `cmd+n` steht in keinem der vier Kästen, und der Plan sagt es ausdrücklich statt es wegzulassen.

**Kein fehlendes Diagramm.** Die drei strukturellen Behauptungen dieses Plans sind der Leseweg, die Sicherungsmomente und die Reihenfolge der Arbeit, und für jede liegt ein Graph vor. Ein viertes Bild für den Lebenslauf des Zettelmodells mit seinen drei `Wechsel`-Werten wäre eine Wiederholung von Bild 2 des Spec in anderer Notation.

## What a clean redraw would require

Nicht einschlägig. Das Urteil lautet *acceptable*, und kein Befund verlangt eine andere Struktur: kein Zyklus, kein Gott-Knoten, keine fehlende Schicht, kein überladenes Bild, kein Parse-Fehler. B1 bis B5 sind Beschriftungen und eine Zahl, an Ort und Stelle zu beheben, ohne einen Knoten zu verschieben oder eine Kante umzuhängen.

Für das Nutzer-Tor bleibt die Beobachtung aus der Prüfung vom 260814-0000 bestehen, in abgeschwächter Form. Der Spec ist nachgezogen worden und trägt beide Zweige an beiden Rauten; der Plan hat die Raute vollständig gezeichnet. Was jetzt noch offen ist, sind zwei Auszählungen (B1, B2), von denen eine den nachgezogenen Spec betrifft und nicht den Plan. Ob die Behebung an einen Planschritt gehört oder als Nachtrag am Dokument läuft, entscheidet der Nutzer; der Eigentümerdatensatz dazu ist `issues/260814-0628_o_diagrammbefunde-haben-keinen-eigentuemer-und-bleiben-deshalb-liegen.md`.

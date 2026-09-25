# Concept Evaluation: Spec Notizzettel als Blatt mit zwei Zetteln

**Date:** 2026-08-14 00:00
**Target:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`
**Verdict:** acceptable
**Diagrams evaluated:** 2  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0, beide Blöcke nach SVG gerendert)

## Verdict

**acceptable, und der Befund, nach dem gefragt wurde, liegt vor: der Spec macht denselben Fehler wie die achte Runde, an einer Stelle mit mehr Gewicht.** Beide Graphen sind strukturell einwandfrei. Gemessen: kein Gott-Knoten (höchster Ausgangsgrad 3 in beiden Bildern), kein freistehender Knoten, kein Knäuel (0,92 und 2,0 Kanten je Knoten), in beiden Fällen der Diagrammtyp, den die Typentabelle vorsieht. Was das Urteil von *clean* auf *acceptable* zieht, ist Bild 1: die zwei Entscheidungsrauten `Z1` und `Z2` tragen je genau eine ausgehende Kante, und der ungezeichnete Zweig an `Z2` ist keine Nachlässigkeit der Zeichnung, sondern trägt den dritten Sicherungsmoment, den Bild 2 desselben Dokuments als Kante `Zettel1 --> [*]` zusagt. Die beiden Bilder widersprechen sich damit. Die Prosa hat recht und nennt die Auflösung, `immer_erreichbar` mit seinen drei Befehlen; auf keinem der zwei Bilder steht sie. Am Entwurf ändert der Befund nichts, an der Belastbarkeit der Bilder als Beleg für C1 und C4 schon.

## Per-diagram measurements

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Geschichtet | Waisen | Urteil |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | flowchart TD | 12 | 11 | 0,92 | 3 (`TF`) | 1 | 0 | nein (kein `subgraph`) | 0 | acceptable |
| 2 | stateDiagram-v2 | 5 (3 Zustände, 2 Pseudozustände) | 10 | 2,00 | 3 (`Zu`, `Zettel1`, `Zettel2`) | 3 (`Zu`, Endpseudozustand) | 3 (erwartet, siehe unten) | entfällt beim Typ | 0 | acceptable |

Bild 1 hat eine Quelle (`K1`) und drei Senken (`TEXT`, `ZEILE`, `ZU`) und läuft als nahezu lineare Kette mit einer dreifachen Auffächerung an `TF`. Bild 2 verteilt seinen Ausgangsgrad gleichmäßig auf alle drei Zustände; ein Gott-Knoten kann bei dieser Verteilung nicht vorliegen. Der Typ passt beide Male: ein gerichteter `flowchart` für einen Kontrollfluss und ein `stateDiagram-v2` für einen Lebenszyklus stehen so in der Typentabelle.

## Findings

**N1 (mittel, Bild 1): Zwei Entscheidungsrauten mit je einem Ausgang.** `Z1` und `Z2` stehen in Rautenform, die in der Flowchart-Grammatik eine Verzweigung zusagt, und beide tragen den Ausgangsgrad 1. An `Z1` fehlt der Zweig „nein: es steht bereits ein Blatt", also genau der Fall, den C1 in der zweiten Abnahmeliste zusagt: „Der zweite Druck auf `f2` schließt ihn nicht und tut nichts." An `Z2` fehlt der Zweig „ja", und der ist der schwerere Fall.

**N2 (mittel, Bild 1 gegen Bild 2): Der ungezeichnete „ja"-Zweig an `Z2` trägt den dritten Sicherungsmoment.** Am Baum geprüft, nicht aus der Prosa übernommen. `zulaessig` berechnet `durchgelassen = immer_erreichbar(kommando) || (…)` (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:176`). Die Auslassung kurzschließt beide Bestandteile, an denen Bild 1 hängt, `blatt_steht` und `ersthelfer_gehoert_appkit`. `immer_erreichbar` führt `Beenden`, `FensterSchliessen` und `FensterEinblenden` (`:200`), alle drei tragen `Wirkungsbereich::Ueberall` (`crates/krk-core/src/tasten/belegung.rs:749-760`), und `wirkt(Ueberall, _)` liefert unbesehen `true` (`crates/krk-ui/src/kommandos/fokus.rs:336`). Das bei stehendem Blatt gemeldete `Fokus::Anderswo` (`crates/krk-ui/src/appkit/anwendung.rs:4172`) hält die drei deshalb nicht auf. Belegt sind damit `cmd+q`, `shift+cmd+w` und `cmd+n`: sie erreichen KRK, während der Zettel steht.

Bild 1 sagt für „jeder weitere Tastendruck" das Gegenteil, mit einer einzigen Kante und der Beschriftung „nein". Bild 2 zeichnet daneben `Zettel1 --> [*]: KRK beendet, sichert Zettel 1`, und diese Kante ist `cmd+q`, also einer der drei. Ein Leser, der beide Bilder nebeneinanderlegt, findet für die Kante aus Bild 2 in Bild 1 keinen Weg. Die Ausgangslage des Spec nennt `immer_erreichbar` samt seinen drei Befehlen; die Zeichnungen nehmen es nicht auf.

**Der geerbte Befund kehrt wieder, und zwar verschärft.** B2 am Spec und F1 am Plan der achten Runde beanstandeten einen Teilgraphen, der eine Weigerung an einem von vier Knoten zeichnete, während alle vier weigern können. Hier zeichnet eine Raute einen von zwei Zweigen, und der ungezeichnete trägt eine Zusage desselben Dokuments. Der Unterschied liegt in der Folge: dort war das Bild unvollständig und mit sich im Reinen, hier widerspricht es dem Nachbarbild. Beide Male ist die Wurzel dieselbe, nämlich eine im Bild unvollständig gezeichnete Fallunterscheidung, deren fehlender Zweig am Baum besteht.

Zwei saubere Auflösungen, beide ohne Verschiebung eines Knotens. Entweder bekommt `Z2` seinen zweiten Zweig, `"ja: immer_erreichbar, drei Befehle"`, mit einer Kante auf einen Knoten „KRK führt aus"; oder die Beschriftung von `K2` grenzt den gezeigten Ausschnitt ausdrücklich ab, etwa „jeder weitere Tastendruck außer den drei aus `immer_erreichbar`". Die erste Fassung ist vorzuziehen, weil sie den Weg zeichnet, auf dem die Sicherung beim Beenden überhaupt zustande kommt.

**N3 (niedrig, Folge aus N2): `shift+cmd+w` bei stehendem Zettel steht in keinem Bild und in keinem Kriterium.** `FensterSchliessen` kommt nach N2 durch und ruft `fenster.performClose(None)` am Hauptfenster (`crates/krk-ui/src/appkit/anwendung.rs:3508-3514`). Was AppKit mit `performClose:` an einem Fenster mit anhängendem Blatt tut, ist in diesem Baum nicht gemessen und hier nicht geprüft. `speculation:` es weist ab und gibt einen Signalton. Die Abnahmeliste von C1 sagt „Steht der Zettel, wirkt kein anderer Tastenbefehl von KRK" und führt drei Beispiele auf, von denen keines diesen Fall trifft. Der Punkt gehört dem Planer, nicht der Zeichnung; er steht hier, weil dieselbe Messung ihn hervorgebracht hat.

**N4 (niedrig, Bild 2): Sechs Kanten sagen „sichert" zu, wo zwei Kriterien das Schreiben an eine Bedingung hängen.** C4 sagt: „Ist der Text des Zettels derselbe, der beim Öffnen gelesen wurde, schreibt KRK nicht." C2 sagt: „Ein Wechsel auf den bereits offenen Tab schreibt nichts." Die Überschrift des Bildes lautet „Die drei Sicherungsmomente", und als Aussage über Momente trägt jede der sechs Kanten. Wer aus dem Bild Schreibvorgänge zählt, zählt zu hoch. Ein Zusatz „sichert, wenn geändert" an den sechs Stellen stellt die Deckung mit C4 her.

**N5 (niedrig, Bild 1): Die Grenze, um die es geht, ist nicht gezeichnet.** Das Bild trägt kein `subgraph`. Seine These steht im Absatz darunter: „Die Nichtanmeldung in `ersthelfer_gehoert_appkit` ist die Kante, an der dieses Bild hängt." Die Trennung zwischen KRKs Befehlsschicht (`K1`, `Z1`, `Z2`) und AppKits Textschicht (`APP`, `TF`, `TEXT`, `ZEILE`, `WA`) ist der Gegenstand des Bildes und in ihm nicht sichtbar. Zwei Teilgraphen kosten zwei Zeilen. Bei zwölf Knoten in fast linearer Kette verdeckt die fehlende Schicht nichts, weshalb der Befund niedrig steht.

**Die drei Zyklen in Bild 2 sind kein Befund, und das ist eine Feststellung und keine Nachsicht.** Der Graph trägt `Zettel1 ↔ Zettel2`, `Zu → Zettel1 → Zu` und `Zu → Zettel2 → Zu`. In einem Zustandsautomaten sind sie die Sache selbst; ein Lebenszyklus ohne Rückkehr wäre ein Einwegvorgang. Die Zyklenheuristik aus `rules/design-diagrams.md` zielt auf Abhängigkeitsgraphen, und Bild 2 ist keiner. Die Prosa erklärt daneben, warum es keinen vierten Übergang gibt.

**Was Bild 2 richtig macht und hier genannt gehört.** Die Verzweigung an `Zu` ist überschneidungsfrei und vollständig: die zwei Kanten nach `Zettel1` und `Zettel2` tragen einander ausschließende Wächter über denselben Zustand, und `Zu --> [*]: KRK beendet, nichts zu sichern` schließt den dritten Ausgang. Genau die Vollständigkeit, die Bild 1 an seinen zwei Rauten schuldig bleibt, liefert Bild 2 an seinem einzigen Verzweigungspunkt.

**Kein fehlendes Diagramm.** Die zwei strukturellen Behauptungen dieses Spec sind der Weg der Tasten und die Folge der Sicherungsmomente, und für beide liegt ein Graph vor. Ein drittes Bild für den Schreibweg aus C4 und C5, also Text über `atomar::schreiben` unter dem `Schreibgriff` in zwei Dateien, wäre vertretbar, zeichnete aber eine noch unentschiedene Struktur: ob die Aufzählung `Datei` um zwei Varianten wächst, gibt der Spec ausdrücklich an den Planer ab. Der Verzicht ist an dieser Stelle die richtige Zurückhaltung und kein Versäumnis.

## What a clean redraw would require

Nicht einschlägig. Das Urteil lautet *acceptable*, und kein Befund verlangt eine andere Struktur: kein Zyklus in einem Abhängigkeitsgraphen, kein Gott-Knoten, keine fehlende Schicht, die als Knäuel erschiene. N1 bis N5 sind an Ort und Stelle zu beheben, ohne einen Knoten zu verschieben oder eine Kante umzuhängen.

Für das Nutzer-Tor bleibt eine Beobachtung außerhalb der Diagrammprüfung. Dieselbe Fallunterscheidung ist jetzt zum dritten Mal unvollständig gezeichnet worden, und die zwei früheren Beanstandungen sind nie an ihrem Dokument behoben worden; der offene Datensatz dazu ist `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/issues/260813-1345_o_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-und-das-stationsbild-zeigt-jetzt-sechs-wo-der-baum-sieben-traegt.md`. Ob eine dritte Beanstandung derselben Art die richtige Antwort ist oder ob die Behebung an einen Planschritt gehört, entscheidet der Nutzer und nicht diese Prüfung.

---

**Abgleich 260814-1002 (reconciler, Runde 9).** Der Befund an diesem Spec ist behoben: der
Nachtrag vom 260814-0628 trägt beide Zweige an beiden Entscheidungsrauten von Bild 1 und die
zwei Teilgraphen, und Bild 2 führt die zwei Zettel als Teilzustand mit den vier Wegen heraus.
Das Muster dahinter — dieselbe unvollständige Fallunterscheidung zum dritten Mal, zweimal
unbehoben — ist nicht behoben und steht als eigener Datensatz
(`issues/260814-0628_o_diagrammbefunde-haben-keinen-eigentuemer-und-bleiben-deshalb-liegen.md`,
zu Recht offen). Keine Aussage dieser Prüfung ist widerlegt.

# Shaper: Directive-Korrektur, Referenzgerät, Diagramm-Nachbesserung

**Datum:** 2026-08-02, 11:27
**Agent:** shaper, in-Circle-Klärungsmodus mit ausdrücklicher Ausnahme für den Circle-Datensatz
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Status:** Complete

## Auftrag

Der Nutzer hat am Spec-Gate drei Entscheidungen getroffen. Zwei davon waren einzutragen: die Korrektur der Bedienzeile in der Circle-Directive und die Benennung des Referenzgeräts für die zehn Zeitzusagen aus Abschnitt C8 des Specs. Dazu kamen der eigene Anteil an einem zweiten Defekt und zwei Ungenauigkeiten, die die Konzeptprüfung `reviews/260802-1118-conceptrev-spec-navigator-geruest.md` benannt hatte.

Der Orchestrator hat die Modus-Grenze für zwei genau bezeichnete Stellen des Circle-Datensatzes aufgehoben: den Schlusssatz des Abschnitts `## Directive` und den Unterabschnitt "Offene Entscheidungen" im Abschnitt `## Grounding snapshot`. Kein anderer Abschnitt des Datensatzes wurde berührt, und kein anderer Circle wurde angefasst.

## Was geändert wurde

### Directive-Zeile

Der Schlusssatz des Abschnitts `## Directive` lautete "Jede Tastenbelegung ist frei konfigurierbar, ausgeliefert wird eine Mac-typische Vorbelegung, ergänzt um F3 bis F8 im Norton-Stil und Shift+Delete zum Löschen." Zwei Angaben darin waren durch die Antworten vom 260802-1105 überholt: die nackten Funktionstasten und Shift+Delete.

Der neue Wortlaut: "Jede Tastenbelegung ist frei konfigurierbar; ausgeliefert wird eine Mac-typische Vorbelegung, die die Norton-Reihe auf Fn+F3 bis Fn+F8 legt und die nackten Funktionstasten frei lässt. Die Taste Delete räumt in den Papierkorb, Fn+F8 löscht endgültig und fragt dabei einmal je Vorgang nach."

Der Vorschlag aus dem Defekt wurde an zwei Stellen geschärft. Er ist auf zwei Sätze verteilt, weil eine einzige Aufzählungskette vier Aussagen in einen Atemzug gepresst hätte. Und er nennt die einmalige Rückfrage vor dem endgültigen Löschen, die der Spec in C4 zusagt und die im Vorschlag fehlte.

### Referenzgerät

Der Nutzer hat das Gerät benannt, auf dem diese Sitzung läuft: ein MacBook Pro 15 Zoll von 2018, Modellkennung `MacBookPro15,1`, mit 8-Core Intel Core i9 bei 2,3 GHz, 16 GB Arbeitsspeicher, Intel UHD Graphics 630 und Radeon Pro 560X, einem Bildschirm mit 2880×1800 bei 60 Hz und macOS 15.7.7. Die Angaben stammen aus `system_profiler` auf ebendiesem Gerät.

Als eigentlichen Arbeitsrechner nennt der Nutzer einen Apple-Silicon-Mac, seiner Angabe nach einen "M2 Pro Max". Die Bezeichnung ist mehrdeutig und vermutlich als M2 Max oder M2 Pro zu lesen; sie ist in beiden Dokumenten als Aussage des Nutzers festgehalten und nicht als geprüfte Tatsache. Für die Zusagen ist das ohne Belang, weil auf dem älteren Intel-Gerät gemessen wird und die Wahl damit die strengere ist.

Die Bildwiederholrate von 60 Hz trägt die Herleitung von L1 und L9 wörtlich: 16 ms sind auf diesem Bildschirm genau ein Einzelbild. Die 120-Hz-Anmerkung des Specs greift auf dem Referenzgerät nicht und ist entsprechend umformuliert.

### Diagramme

Das erste Diagramm des Specs hat eine Kante dazubekommen, `K -->|blendet ein und aus| P2`. C7 sagt drei ausblendbare Bereiche zu, gezeichnet waren zwei. Die Selbstprüfung am Dokumentende nannte 13 Kanten bei tatsächlich 14; sie nennt jetzt 15 und dazu das Verhältnis, den Ausgangsgrad und die beiden Zyklen.

Die Umlaut-Ersetzungen sind zurückgenommen, in beiden Diagrammen. Die Konzeptprüfung hatte nachgewiesen, dass Umlaute in Mermaid fehlerfrei parsen, sowohl in einem Flowchart-Label in Anführungszeichen als auch in einem unquotierten Übergangslabel eines Zustandsdiagramms.

Von der Namensdrift gegenüber dem Circle-Datensatz sind drei Punkte aufgeräumt: der Knoten für die Dateioperationen heißt jetzt wie dort `D`, das Vorschaufenster heißt "Vorschaufenster" statt "Vorschau", und die Leiste heißt "Lesezeichen und Geräteordner". Ein vierter Punkt bleibt bewusst stehen. Der Datensatz nennt die Reihe der vier Bereiche "Oberfläche", der Spec "Fensterzeile". Der Spec verwendet "Oberfläche" an drei anderen Stellen in der allgemeinen Bedeutung, etwa in L4 und in C9. Eine Angleichung an den Datensatz hätte innerhalb des Specs eine neue Doppeldeutigkeit erzeugt, also die Regel "eine Benennung pro Sache" an anderer Stelle verletzt. "Fensterzeile" ist der präzisere Name und bleibt.

Beide Diagrammblöcke des Specs und der Block des Circle-Datensatzes wurden nach der Änderung mit Werkzeug geprüft, nicht per Lesen: `mermaid.parse()` aus mermaid 11 unter jsdom, Node v24.2.0. Ergebnis `flowchart-v2` OK, `stateDiagram` OK, `flowchart-v2` OK.

### Folgeänderungen im Spec

Drei Stellen hingen an den beiden Antworten und wären sonst falsch stehen geblieben. Der Gatehinweis am Kopf nannte zwei zu behandelnde Punkte, die jetzt beide erledigt sind. Der Abschnitt `## Offene Nutzerentscheidungen` führte das Referenzgerät als offen. Und der Abschnitt `## Abweichung zur Circle-Directive` beschrieb einen Widerspruch, den es nicht mehr gibt; er heißt jetzt `## Abgleich mit der Circle-Directive` und hält den alten wie den neuen Wortlaut fest.

## Markerwechsel

| Datei | Vorher | Nachher |
|---|---|---|
| `decisions/260802-1036_*_leistungszusagen-navigator.md` | `_o_` offen | `_a_` beantwortet |
| `issues/260802-1105_*_directive-zeile-widerspricht-loeschantwort.md` | `_o_` offen | `_c_` geschlossen |

Der Spec bleibt auf `_o_`, wie beauftragt. Es wurde nicht committet.

## Offen geblieben

`issues/260802-1105_o_beantwortete-entscheidungen-noch-als-offen-gefuehrt.md` bleibt offen. Der Defekt betrifft zwei Dateien; der Circle-Datensatz ist erledigt und im Defekt als Teil 1 vermerkt, `CLAUDE.md` bearbeitet ein anderer Agent. Der Orchestrator schließt den Defekt, wenn beide Teile vorliegen.

Drei Entscheidungsdatensätze im geteilten Speicher bleiben offen und binden die Runde 1 nicht: `shared/decisions/260802-0842_o_git-verwerfen-bedeutung.md`, `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` und `shared/decisions/260802-0842_o_code-sdk-fuer-ki-integration.md`.

Nicht angefasst, bewusst: der Abschnitt `## Activation proposal` im Circle-Datensatz nennt die beiden beantworteten Entscheidungen weiterhin unter ihrem alten Pfad. Der Abschnitt ist mit "Vorgeschlagen am: 260802-0853" datiert und hält wie eine Historiendatei den Stand seines Zeitpunkts fest. Der zugehörige Defekt schließt Historiendateien aus demselben Grund ausdrücklich aus. Ein eigener Defekt dafür wurde deshalb nicht gefiled.

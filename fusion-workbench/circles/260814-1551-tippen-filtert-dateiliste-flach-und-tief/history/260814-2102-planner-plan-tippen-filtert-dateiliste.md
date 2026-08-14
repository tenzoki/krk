# Planner-Sitzung: der Umsetzungsplan der Runde 10

**Datum:** 260814-2102
**Status:** Complete
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/` (aktiv, über den Parameter `Circle:` benannt)
**Ergebnis:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`

## Was gelesen wurde

Der Spec in seiner dritten Fassung, beide Diagrammprüfungen unter `reviews/`, alle elf Entscheidungsdatensätze dieses Circles, und der Baum auf dem Stand `6cd122c` unter `crates/` und `resources/`. Am Baum gelesen wurden: `verzeichnis/modell.rs`, `verzeichnis/leser.rs`, `verzeichnis/sprungmarke.rs`, `verzeichnis/eintrag.rs`, `verzeichnis/mod.rs`, `tabs.rs`, `appkit/tabelle.rs`, `appkit/ereignisse.rs`, `appkit/bereichsleiste.rs`, `appkit/statuszeile.rs`, `kommandos/zulaessigkeit.rs`, `kommandos/operationen.rs`, `menuemodell.rs`, die einschlägigen Teile von `appkit/anwendung.rs`, `tasten/belegung.rs`, `tasten/mod.rs`, `belegungsmodell.rs` und `resources/default-keymap.toml`.

Als Vorlage für Form und Ton diente der Plan der Runde 8, die bisher einzige kohärent geschlossene.

## Was der Plan trägt

Vierzehn Schritte in sieben Strängen, dreizehn an `coder` und einer an `ontocoder`. Vier Mermaid-Bilder, alle vier mit `@mermaid-js/mermaid-cli` gerendert, Exit 0.

Die flache Hälfte steht vor der tiefen: der Durchlauf braucht den Filter, der Filter braucht den Durchlauf nicht. Der eine Vorbedingungsschritt ist A1, der den einen Prüfschritt der Sichtbarkeit zieht und vier Felder am `Ordnermodell` anlegt.

## Die zwei Fragen, die der Spec dem Planner gegeben hat

**Fäden und Kanäle des Durchlaufs:** ein Arbeitsfaden je Tab, ein `sync_channel` mit 1.024 Plätzen je Tab, die Auftragsliste beim Start vollständig übergeben, Beginn erst nach dem Abschluss des Lesevorgangs des angezeigten Ordners. Der tragende Grund ist C3.6: unter dieser Bauart zählt „einer" Fäden, Kanäle und Durchläufe zugleich und braucht keine Lesart. Datensatz: `decisions/260814-2102_a_wie-viele-faeden-und-kanaele-benutzt-der-durchlauf-ueber-den-unterbaum.md`.

**Die Fallunterscheidung der Rückschritt-Taste:** sie gehört **nicht** in `kommandos/zulaessigkeit.rs`, und der Grund ist Entscheidbarkeit und nicht Geschmack. `resources/default-keymap.toml:156-158` legt `delete` und `cmd+delete` auf dieselbe Funktion; beide werden vor der Frage zu demselben `Kommando::InPapierkorb`, und der zweite Frager derselben Regel ist die Menü-Ausgrauung, die überhaupt keinen Tastendruck hat. Eine Antwort dort träfe beide Wege und graute den Menüeintrag aus, was C1.19 und C6.11 ausschließen. Die Regel steht deshalb als reine Funktion in einem neuen Modul neben `zulaessigkeit.rs` und wird im Ausführungszweig hinter der unveränderten Zulässigkeitsprüfung gefragt. Datensatz: `decisions/260814-2102_a_gehoert-die-fallunterscheidung-der-rueckschritt-taste-in-die-zulaessigkeitsregel.md`.

Beide Datensätze entstehen beantwortet, weil der Spec sie ausdrücklich an den Planner gerichtet hat. Sie stehen als eigene Datensätze und nicht als Absätze im Plan, weil ihre Antworten über diese Runde hinaus binden.

## Was neu abgelegt wurde

- Zwei Entscheidungsdatensätze, beide `_a_`, beide mit `Answered:`-Block auf den Plan.
- Ein Defekt, `issues/260814-2102_o_der-pruefschritt-fuer-die-sichtbarkeit-steht-im-ordnermodell-zweimal-wortgleich-da.md`. Der Prüfschritt für die Sichtbarkeit steht in `anhaengen` und in `sicht_neu_aufbauen` als zwei eigene Fassungen derselben Regel. Der Zustand ist vor dieser Runde entstanden; Schritt A1 behebt ihn nebenbei.

## Was offen bleibt

Die vier offenen Nutzerentscheidungen des Spec halten weiter keinen Schritt auf. Der Plan fährt bei allen vier auf derselben Empfehlung wie der Spec und nennt an jedem betroffenen Schritt, was sich mit einer anderen Antwort ändert: die Rangfolge der Filterzahl betrifft D1, der Filtertext beim Ordnerwechsel betrifft B2, der Gültigkeitsbereich von „Deep" betrifft E3 und F2, die Stelle des Filtertexts in den Bedeutungen von `Esc` betrifft B2.

## Zwei Aussagen, die Erschließungen sind und keine Messungen

Der Plan kennzeichnet sie als solche, weil ein Bau, der sie für geprüft hält, an der falschen Stelle spart. Erstens: dass die Verzögerung durch den Start des Durchlaufs nach dem Lesevorgang für gewöhnliche Ordner unter einem Takt liegt. Zweitens: dass AppKit `isARepeat` nur für aufeinanderfolgende Drücke derselben Taste setzt — der Plan nimmt diese Annahme mit einer Rücksetzzeile in der Senke aus der Rechnung, statt sich auf sie zu stützen.

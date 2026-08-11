# Shaper, anticipated-circle: Statusleiste mit Schaltern für die fünf Bereiche

**Datum:** 260811-1305
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete

## Auftrag

Der Nutzer hat über `/fusion:direct` einen Entwurf diktiert: eine Statusleiste am unteren Fensterrand über die volle Breite, mit fünf Schaltern für die fünf Bereiche der Fensterzeile, bedienbar per Tastatur und Mausklick, und jede Änderung löst eine Neuaufteilung **proportional zur letzten** aus. Dazu der am 260811-1240 gemeldete Defekt `shared/issues/260811-1245_o_die-breite-des-vorschaufensters-faellt-beim-navigieren-in-der-dateiliste-zurueck.md`, der vorher oder mit der Runde zu klären ist. Domäne `code`.

Der Auftrag hielt ausdrücklich fest, dass der Shaper als Unteragent kein `AskUserQuestion` hat, die Fragen im Bericht zurückgeben soll und den Circle trotzdem anlegt.

## Was entstanden ist

**Circle:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/`, Datensatz `_a_circle.md`, mit den sechs Artefaktverzeichnissen. Kein Spec: ein vorgesehener Circle hält eine Absicht fest.

**Sieben Entscheidungsdatensätze** in `decisions/` dieses Circles, alle `_o_`:

- `260811-1305_o_was-heisst-proportional-zur-letzten-aufteilung.md`
- `260811-1305_o_traegt-das-linke-dateifenster-einen-schalter.md`
- `260811-1305_o_wie-zeigen-zwei-schalter-eine-flaeche-die-nur-einer-haben-kann.md`
- `260811-1305_o_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md`
- `260811-1305_o_welchen-anteil-bekommt-ein-bereich-der-noch-nie-sichtbar-war.md`
- `260811-1305_o_was-geschieht-wenn-die-mindestbreiten-nicht-mehr-hineinpassen.md`
- `260811-1305_o_wird-der-vorschaubreiten-defekt-in-dieser-runde-behoben.md`

## Abweichung vom Prompt: wohin die Entscheidungsdatensätze gingen

Der Shaper-Prompt nennt `$OUT_DECISION` als Ablageort. Der Auflöser gab dafür `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions` aus, weil dieser Circle gerade aktiv ist. Die sieben Fragen sind aber aus der Directive des **neuen** Circles entstanden und nicht aus der des aktiven. Nach der Herkunftsregel gehören sie deshalb in `decisions/` des neuen Circles, und dort liegen sie. Der Auftrag des Nutzers sagt dasselbe: "Ungeklärtes gehört als Entscheidungsdatensatz daneben."

Diese Sitzungsdatei liegt dagegen unter `$OUT_HISTORY` wie vom Prompt verlangt, also im aktiven Circle.

## Was am Code geprüft wurde

Der Grounding-Abschnitt des Datensatzes führt die Belege einzeln. Der Befund, an dem der Umfang der Runde hängt: **`bereichsbreiten` (`crates/krk-ui/src/fenstermodell.rs:609`) verteilt heute nicht proportional.** Die drei festen Bereiche bekommen der Reihe nach ihre gespeicherte Breite in Punkten, und allein die beiden Dateifenster teilen den Rest im Verhältnis. Die Reihenfolge ist eine Festlegung des Nutzers vom 260808 und keine Nebensache: die Lesezeichenleiste steht vor dem Editor, also weicht sie nicht.

Zwei weitere Befunde haben je eine Frage ausgelöst. Die Statuszeile aus C1 sitzt **je Dateifenster** und nicht über die Fensterbreite; eine Leiste über die Fensterbreite ist damit eine zweite Fläche. Und `Sichtbarkeit` führt bewusst kein Feld für das linke Dateifenster, mit einer Begründung im Code, die ein fünfter Schalter aufheben würde.

## Was nicht gemacht wurde

Kein Spec, kein Plan, keine Änderung an Code, Daten oder einem anderen Circle. Der neue Circle ist nicht aktiviert; das bleibt beim Nutzer über `/fusion:next`.

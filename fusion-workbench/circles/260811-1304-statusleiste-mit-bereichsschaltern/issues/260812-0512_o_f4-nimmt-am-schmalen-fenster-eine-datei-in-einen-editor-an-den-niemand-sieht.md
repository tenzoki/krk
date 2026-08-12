F4 nimmt am schmalen Fenster eine Datei in einen Editor an, den niemand sieht

---

Seit Schritt 2 des Plans weist `Fenstermodell::umschalten` das Einschalten eines Bereichs ab, dessen
Mindestbreite nicht mehr in die Zeile passt. Steht das Fenster schmaler als rund 920 Punkte — und
die Mindestgröße erlaubt 780 —, dann lädt F4 auf einer Textdatei sie weiterhin in den Editor, aber
der Editorbereich kommt nicht auf den Schirm, der Fokus bleibt, wo er war, und **keine Meldung sagt
etwas**. Der Nutzer sieht auf einen Tastendruck hin nichts geschehen, während das Programm eine
Datei hält.

---

**Schwere:** mittel (kein Absturz und kein Datenverlust, aber ein stiller Zustand, der von außen
nicht zu erkennen ist)
**Gefunden:** coder, bei der Umsetzung von Schritt 2
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs`, `im_editor_oeffnen` /
`editorausgang_behandeln` / `fokus_editor_holen`
**Domain:** code

## Der Weg, am Baum gelesen

`im_editor_oeffnen` (`anwendung.rs:3543`) reicht den Pfad an den Editorbereich und prüft die Datei
nicht selbst; der Ausgang kommt in `editorausgang_behandeln` (`:3699`) an, und dort holt
`fokus_holen(Fokus::Editor)` (`:3731`) den Bereich hervor und setzt den Fokus in einem Zug. Der
erste Halbsatz geht seit Schritt 2 durch die neue Abweisung: `bereich_einblenden` →
`Fenstermodell::einblenden` → `umschalten` liefert `false`, weil Lesezeichenleiste, beide
Dateifenster und der Editor zusammen 920 Punkte verlangen. Der zweite Halbsatz scheitert danach
ebenfalls, und zwar an einer Prüfung, die es schon gab: `fokus_setzen` (`:1518`) weist jeden Fokus
auf einen ausgeblendeten Bereich ab.

Zurück bleibt ein Editormodell mit einer Datei. `fokus_editor_holen` (`:1425`) lässt den Befehl aus
C1 danach durch, weil der Editor eine Datei hält — und er läuft in dieselbe Abweisung. `opt+cmd+e`
löst die Nachfrage aus C4 für einen ungesicherten Stand aus, den niemand gesehen hat.

## Was hier nicht der Defekt ist

**Die Stille ist gewollt.** C2.5 des Plans verlangt ausdrücklich, dass keine Abweisung eine Meldung
erzeugt, und C2.4 nennt die drei Fälle. Der Defekt ist nicht, dass der Bereich nicht aufgeht,
sondern dass der Weg dorthin auf halber Strecke stehen bleibt: die Datei ist angenommen, die Fläche
nicht.

## Drei Wege, keiner in diesem Schritt

1. **Vorher fragen.** `im_editor_oeffnen` und `editor_aus_vorschau` fragen das Fenstermodell, ob der
   Editor überhaupt eingeblendet werden könnte, und lassen die Datei sonst gar nicht erst laden. Das
   ist die Reihenfolge, die das elfte Abnahmekriterium von C2 schon für die Dateiprüfung setzt
   ("erst die Prüfung, dann die Fläche"), und sie hätte hier dieselbe Form. Kosten: eine öffentliche
   Frage am Fenstermodell, die es heute nicht gibt.
2. **Die Mindestbreite des Fensters heben.** Steht `MINDESTGROESSE` in der Breite auf 940 statt 780,
   ist der Fall nicht mehr erreichbar. Das ist genau die Möglichkeit, die der Datensatz
   `decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`
   als Nutzerentscheidung führt; dieser Defekt ist ein weiteres Argument darin und keine zweite
   Frage daneben.
3. **Doch eine Meldung.** Widerspricht C2.5 und wäre eine Änderung der Zusage, nicht des Codes.

Der zweite Weg macht den ersten gegenstandslos, und deshalb ist die Reihenfolge: erst die
Nutzerfrage beantworten, dann bauen.

## Zusammenhang

Der Fall entsteht mit Schritt 2 und ist keine Altlast. Er trifft nur den Editor, weil er der einzige
Bereich ist, der eine Datei hält; die Lesezeichenleiste und die Vorschau bleiben nach einer
Abweisung ohne Rest zurück.

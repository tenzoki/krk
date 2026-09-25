# Coder: die vier Befunde der Durchsicht vom 260812-0539

**Datum:** 260812-0700
**Agent:** coder
**Status:** Complete
**Maßstab:** `reviews/260812-0539-coderev-proportionale-breitenregel-und-linkes-dateifenster.md`,
Befunde 1 bis 4
**Abnahme:** `make check` — **Exit 0**

## Auftrag

Die vier Befunde der Durchsicht beheben, alle vier an Code dieser Runde. Für Befund 1 und 2 die
**eine** Lösung suchen statt zweier Flicken; `MINDESTGROESSE` nicht anfassen, weil der Nutzer am
260812-0430 entschieden hat, dass sie bei 780 bleibt. Nicht Schritt 8, nicht committen.

Die Zahlen des Prüfers sind vor jeder Änderung mit einem eigenen Nachbau der drei Funktionen
außerhalb des Baums nachgerechnet. **Alle gehen auf**, auf die zweite Nachkommastelle: der Umlauf
1280 → 780 → 1280 liefert `[166.57, 333.13, 333.13, 0.00, 444.17]` gegen die ursprünglichen
`[155.31, 362.39, 362.39, 0.00, 396.91]`, und beide Breitenbefehle liefern unter der Mindestsumme
`[193.04, 457.14, 315.03, 260.00, 514.78]`, also 71,05 Punkte in derselben Richtung.

**Ein Zusatzbefund aus dem Nachrechnen:** der Fall aus Befund 1 braucht das Nachlesen nicht, das
der Datensatz als Ursachenkette führt. Aus den ausgelieferten Anfangsbreiten heraus liefern beide
Richtungen bei 780 Punkten `[-, 457.14, 382.86, -, -]`. Die Bedingung ist `gespeicherte Breite <
Mindestmaß × Maßstab`, und die gilt im zweiten Zweig für jeden sichtbaren Bereich, gleich woher
seine Zahl kommt. Das Nachlesen verschärft die Lage, es stellt sie nicht her.

## Die eine Naht, und was sie trägt

Der Prüfer nennt sie unter `## Was quer liegt`: drei Stellen bauen darauf, dass die Abbildung
zwischen gespeicherter Zahl und Bildschirmpunkt ein einziger Faktor ist. Das ist richtig
beschrieben, aber es ist nicht die Stelle, an der sich die beiden Befunde gemeinsam beheben lassen,
und zwar aus einem Grund, den erst das Nachrechnen zeigt: **die Frage „ist die Abbildung
umkehrbar?" ist aus den Rahmen der Ansichten nicht entscheidbar.** Ein Bereich, der genau auf
seinem Mindestmaß steht, kann dort gedeckelt worden sein oder vom Nutzer hingezogen; beide Lagen
sehen gleich aus. Wer diese Frage trotzdem stellt, muss raten, und jede der beiden Antworten kostet
etwas Echtes — entweder eine verworfene Ziehbewegung oder den Defekt.

Die gebaute Lösung stellt deshalb eine **andere** Frage, die aus denselben Eingaben entscheidbar
ist:

> **Vom Schirm wird nur zurückgelesen, was die Regel nicht selbst ausgelegt hat.**

Der Grund, aus dem das die richtige Frage ist und keine Ausweichbewegung: die Rückrechnung gibt es
allein deshalb, weil eine mit der Maus verschobene Trennlinie in den Rahmen der Ansichten steht und
nirgends sonst. Alles andere dort ist die Ausgabe von `bereichsbreiten` selbst, und sie als deren
Eingabe wieder einzuspeisen ist genau die Stelle, an der die Schleife ihre Neutralität verliert,
sobald gedeckelt wird. Ob gedeckelt wurde, muss dafür niemand wissen.

Die Regel gilt an zwei der drei Stellen, und zwar an denen, die eine ganze Zeile zurücklesen. Die
dritte, `breite_aendern`, liest keine Zeile zurück, sondern verschiebt eine Trennlinie zwischen
zwei Bereichen; dort gilt dieselbe Regel im Zuschnitt des Paares, und das ist die Feststellung, ob
die beiden Mindestmaße überhaupt nebeneinander passen.

## Was entstanden ist

**`crates/krk-ui/src/fenstermodell.rs`**

- `Fenstermodell::breite_aendern` rechnet die beiden Schranken der Deckelungskette einzeln aus und
  kehrt ohne Wirkung zurück, wenn die untere über der oberen liegt. Dann hält keine Lage der
  Trennlinie beide Mindestmaße — dieselbe Antwort, die die Funktion bei einem einzigen sichtbaren
  Dateifenster schon gibt, und dieselbe, die der Schirm gibt. **Der fehlende Boden bei 0 fällt mit
  derselben Zeile weg:** eine negative gespeicherte Breite verlangt `dort < mindestmass(bereich) -
  hier`, und das ist genau die neue Bedingung.
- `traegt_eine_ziehbewegung` (neu, privat) rechnet die Zeile aus den gehaltenen Breiten aus und
  vergleicht sie mit der gemessenen. `ZIEHSPIELRAUM` ist ein Viertelpunkt: unter dem kleinsten
  Schritt, mit dem sich eine Trennlinie ziehen lässt (ein halber Punkt auf einem Schirm mit
  doppelter Auflösung), und über dem, was ein Runden der Rahmen auf ganze Bildpunkte hinterließe.
- `Fenstermodell::breiten_uebernehmen` nimmt das `Zeilenmass` als zweiten Parameter und kehrt ohne
  Wirkung zurück, wenn die gemessene Zeile keine Ziehbewegung trägt.
- `wuensche_nachfuehren` (neu, öffentlich) beantwortet dieselbe Frage für das Auslegen nach einer
  Größenänderung. Sie steht hier und nicht in `appkit::aufteilung`, weil sie zur Breitenregel
  gehört und ohne Fenster prüfbar sein soll.
- `breite_in` (neu, privat) neben `sichtbar_in`: die eine Zuordnung von einem `Bereich` auf sein
  Feld in `Breiten`. `Fenstermodell::breite` ruft sie, statt die Fallunterscheidung ein zweites Mal
  auszuschreiben; `wuensche_nachfuehren` hätte sie sonst ein drittes Mal gebraucht.
- Die Kommentare an `breite_aendern`, `massstab` und `breiten_uebernehmen` sind mit derselben
  Änderung richtig geworden. An `breite_aendern` stand die Zusage „Am Mindestmass hoert der Schritt
  auf, statt es zu unterschreiten", die für diesen Fall das Gegenteil des Codes sagte.

**`crates/krk-ui/src/appkit/aufteilung.rs`**

- Der Delegierte hält die Wünsche jetzt selbst (`AufteilungsIvars::wuensche`), statt sie aus den
  Rahmen der Unteransichten zurückzulesen. **Der Rahmen war der falsche Speicher, und das ist die
  Ursache hinter beiden Wegen des zweiten Befunds:** er trägt unter einer Deckelung die Deckelung
  und nicht mehr den Wunsch.
- `neu_auslegen` fragt `wuensche_nachfuehren` und benutzt dafür endlich `alte_groesse`, den
  Parameter, den es bis heute ignoriert hat. Er ist tragend: die gemessenen Breiten sind unter der
  **alten** Zeilenbreite entstanden.
- `Aufteilung::anwenden` trägt den Wunsch des Fenstermodells in dasselbe Feld ein und ruft
  `sichtbar_in` statt des entfallenen `sichtbar_im`.
- Der Modulkopf sagt jetzt, wo die Wünsche stehen. **Es entsteht kein Rückweg in das Fenstermodell
  und kein Ring:** der Delegierte hält einen Wert, keine Sicht auf das Modell.

**`crates/krk-ui/src/appkit/anwendung.rs`**

- `bildschirmbreiten_uebernehmen` reicht das `Zeilenmass` mit durch, aus derselben Quelle wie die
  drei Aufrufe daneben. Der Doc-Kommentar sagt, dass nur eine wirkliche Ziehbewegung übernommen
  wird.

## Proben

Sieben, alle mit ausgerechneten Zahlen; keine schreibt nach, was der Code liefert.

| Probe | hält fest |
|---|---|
| `unter_der_mindestsumme_bleibt_der_breitenbefehl_ohne_wirkung` | Befund 1: bei 600 Punkten lassen beide Richtungen die gespeicherten Breiten unangetastet |
| `ein_gedeckelter_dritter_bereich_sperrt_den_breitenbefehl_nicht` | die Gegenprobe: bei 800 Punkten hängt nur die Leiste, und der Befehl wirkt weiter |
| `ein_zusammengezogenes_fenster_laesst_die_gespeicherten_breiten_stehen` | Befund 2, Weg über das Modell |
| `ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her` | Befund 2, Weg über den Schirm: 1280 → 600 → 1280 kommt auf `[180, 420, 420, 260]` zurück |
| `eine_mit_der_maus_verschobene_trennlinie_gilt_als_neuer_wunsch` | die Gegenprobe: eine Ziehbewegung übersteht die Größenänderung |
| `die_zuordnung_von_bereich_auf_sichtbarkeit_trifft_jedes_feld` | Befund 3 |
| `ein_bereich_ohne_fensterseite_aendert_nur_seine_eigene_breite` | Befund 4 (umbenannt) |

**Gegengeprobt, und das ist der Beleg, dass die Proben die Befunde halten.** Mit von Hand
zurückgenommenen Änderungen fallen genau die drei, die einen Befund messen, und sie fallen mit den
Zahlen der Durchsicht:

```
unter_der_mindestsumme_...        links: Some(512.0), rechts: Some(328.0)  in beiden Richtungen
ein_zusammengezogenes_fenster_... [202.11, 404.21, 404.21, 269.47]
ein_hin_und_her_am_fensterrand_... [202.11, 404.21, 404.21, 269.47] statt [180, 420, 420, 260]
```

Die beiden Gegenproben bleiben dabei grün, wie sie es sollen: sie messen, dass die Behebung nicht
zu viel abweist.

## Was nicht behoben ist

- **`issues/260812-0512_o_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`**
  bleibt offen. Er sitzt auf einem anderen Weg: der Abweisung am Schalter in
  `Fenstermodell::umschalten`, nicht an der Rückrechnung vom Schirm. Keine der beiden Änderungen
  berührt ihn.
- **Neu abgelegt:**
  `issues/260812-0700_o_der-breitenschritt-kommt-neben-einem-gedeckelten-bereich-gekuerzt-an.md`.
  Der Schritt kommt neben einem gedeckelten Bereich gekürzt an, gemessen 20,36 statt 40 Punkte bei
  800 Punkten Fensterbreite. Das ist die an `massstab` benannte und angenommene Ungenauigkeit;
  abgelegt ist sie, weil sie an C4.9 etwas kostet und diese Kosten bisher nur in einem Kommentar
  standen. Der Weg entscheidet sich am Spec.

## Was ohne Fenster nicht zu prüfen ist

Ob AppKit die Rahmen der Unteransichten unverändert stehen lässt, nachdem `auslegen` sie gesetzt
hat. Tut es das nicht, sieht jede Größenänderung wie eine Ziehbewegung aus, und das Verhalten fällt
auf das von vor dem 260812 zurück — also auf den Defekt, nicht auf einen schlimmeren. Die Wahl des
Spielraums fällt damit auf die sichere Seite, und die Zusage, dass eine Ziehbewegung die
Größenänderung übersteht, hält in beiden Fällen.

## Abnahme

`make check` — Exit 0. Nicht committet.

Die Breitenbefehle aus C7 wirken unter der Mindestsumme in die falsche Richtung

---

Steht das Fenster schmaler als die Summe der Mindestbreiten der sichtbaren Bereiche, dann tun
`opt+cmd+links` und `opt+cmd+rechts` auf einem Dateifenster **dasselbe**, und beide das Gegenteil
von dem, was ihr Name sagt: das genannte Dateifenster wird gespeichert breiter, das andere
schmaler. Auf dem Schirm bewegt sich dabei nichts, weil der zweite Zweig von `bereichsbreiten` die
Wünsche gar nicht liest. Sichtbar wird die Verschiebung erst, wenn der Nutzer das Fenster wieder
aufzieht — dann stehen die beiden Dateifenster nicht mehr gleich breit.

---

**Schwere:** mittel (kein Absturz, kein Datenverlust; ein Tastenbefehl mit umgekehrter Wirkung und
ohne Rückmeldung)
**Gefunden:** coderev, Durchsicht der Commits `5e17c9e`, `a2ea876`, `8ffaac2`
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs`, `breite_aendern` (`:598`) zusammen mit
`massstab` (`:645`) und `breiten_uebernehmen` (`:692`)
**Domain:** code

## Nachgerechnet

Nachgerechnet mit einem Nachbau der drei Funktionen aus dem Stand `8ffaac2`; die Zahlen sind
gerechnet und nicht geschätzt. Lage: Editor sichtbar, Vorschau aus, Fenster auf der Mindestgröße
von 780 Punkten, Trennlinie 1 Punkt. Die Mindestsumme der vier sichtbaren Bereiche ist 920, die
verfügbare Breite 777, also greift der zweite Zweig.

```
auf dem Schirm bei 780:            [101.35, 202.70, 202.70,   0.00, 270.26]
gespeichert nach dem Nachlesen:    [193.04, 386.09, 386.09, 260.00, 514.78]
Massstab 1.9048, skaliertes Mindestmass der Dateifenster 457.14
   -> gespeichert 386.09 liegt darunter

opt+cmd+rechts auf das linke Dateifenster:
   gespeichert [193.04, 457.14, 315.03, 260.00, 514.78]
opt+cmd+links  auf das linke Dateifenster:
   gespeichert [193.04, 457.14, 315.03, 260.00, 514.78]
```

Beide Richtungen liefern dasselbe Ergebnis, und beide verschieben um 71,05 statt um die 40 Punkte,
die C4.9 zusagt. Zieht der Nutzer das Fenster danach wieder auf 1280, kommen die beiden
Dateifenster im Verhältnis 457 zu 315 zurück statt gleich breit.

## Die Ursache, am Baum gelesen

`breite_aendern` deckelt gegen ein **skaliertes** Mindestmaß (`:606`,
`bereich.mindestbreite() * massstab`), und die Deckelung steht als Kette:

```rust
let betrag = betrag
    .min(dort - mindestmass(anderer))
    .max(mindestmass(bereich) - hier);
```

Die Kette setzt stillschweigend voraus, dass `dort - mindestmass(anderer)` nicht kleiner ist als
`mindestmass(bereich) - hier`. Sonst gewinnt `.max()` über `.min()`, und das Vorzeichen des
übergebenen Betrags spielt keine Rolle mehr.

Diese Voraussetzung hält, solange jede gespeicherte Breite über ihrem skalierten Mindestmaß liegt,
und genau das stellt `breiten_uebernehmen` normalerweise her: es schreibt `gemessen * faktor`, die
gemessene Breite liegt über dem Mindestmaß, und `faktor` ist derselbe `massstab`. **Im zweiten
Zweig gilt das nicht.** Dort ist die gemessene Breite `mindestbreite * verfuegbar / mindestsumme`,
also kleiner als das Mindestmaß, und die Rückrechnung trägt diesen Faktor ungebrochen in die
gespeicherte Zahl. Danach liegt jede gespeicherte Breite um denselben Faktor
(777/920 = 0,845) **unter** ihrem skalierten Mindestmaß, und die Kette kippt.

Der Dokumentationskommentar an `breite_aendern` sagt dazu heute "Am Mindestmass hoert der Schritt
auf, statt es zu unterschreiten." Für diesen Fall sagt er das Gegenteil des Codes.

## Wie weit die Kette trägt

Die Kette hat keinen Boden bei 0. Übersteigt `mindestmass(bereich) - hier` den Wert `dort`, schreibt
sie eine **negative** gespeicherte Breite. Nachgerechnet ist die Bedingung dafür
`mindestsumme > 2 × verfuegbar`; bei der ausgelieferten Mindestgröße von 780 Punkten und einer
größtmöglichen Mindestsumme von 920 ist sie nicht erreichbar. Erreichbar wird sie mit einer von Hand
geschriebenen `session.toml` — C7 erklärt die Datei ausdrücklich für von Hand schreibbar —, deren
Breiten stark auseinanderliegen, denn beim Start liegt noch keine gemessene Breite vor und
`breiten_uebernehmen` kehrt ohne Wirkung zurück. Eine negative Zahl prüft `Breiten` beim Einlesen
nicht ab.

## Was die Proben heute nicht messen

`der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt` (`fenstermodell.rs:1658`) misst
bei 1280, 1400 und 1920 Punkten. In allen drei Lagen hängt kein Bereich an seinem Mindestmaß, und
der zweite Zweig kommt nicht vor. C4.9 ist damit nur für den ungedeckelten Fall nachgewiesen.

## Zwei Wege, keiner im Vorbeigehen

1. **Die Kette bekommt die Voraussetzung, die sie braucht.** Vor der Deckelung feststellen, ob die
   beiden skalierten Mindestmaße überhaupt nebeneinander passen (`mindestmass(bereich) +
   mindestmass(anderer) <= hier + dort`); passen sie nicht, ist keine Verschiebung möglich und der
   Befehl bleibt ohne Wirkung, wie er es schon bei einem einzigen sichtbaren Dateifenster tut.
   Das ist dieselbe Antwort, die der zweite Zweig auf dem Schirm ohnehin gibt.
2. **Den zweiten Zweig unerreichbar machen.** `MINDESTGROESSE` in der Breite auf 940 heben. Das ist
   die Möglichkeit, die
   `decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`
   als Nutzerentscheidung führt; dieser Defekt ist ein weiteres Argument darin und keine zweite
   Frage daneben.

Der zweite Weg macht den ersten nicht gegenstandslos: die Kette bliebe auch dann ohne Boden, und
eine von Hand geschriebene `session.toml` erreicht sie weiterhin.

## Zusammenhang

Der Fall entsteht mit `a2ea876` (der Maßstab in `breite_aendern`) auf der Grundlage von `5e17c9e`
(die Rückrechnung in `breiten_uebernehmen`) und ist keine Altlast. Vor der Runde übernahm
`breiten_uebernehmen` die gemessenen Punktzahlen roh, und die liegen für einen sichtbaren Bereich
nie unter seinem Mindestmaß; die Kette konnte deshalb nicht kippen.

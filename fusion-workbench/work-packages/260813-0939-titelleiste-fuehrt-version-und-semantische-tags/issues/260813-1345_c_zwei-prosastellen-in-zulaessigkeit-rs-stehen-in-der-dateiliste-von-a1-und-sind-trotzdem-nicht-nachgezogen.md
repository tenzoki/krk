Zwei Prosastellen in `zulaessigkeit.rs` stehen in der Dateiliste von A1 und sind trotzdem nicht nachgezogen

---

Die Durchsicht von Turn 1 hat den Querschnitt „die Prosa läuft dem Code hinterher" auf eine
Ursache zurückgeführt: ein Planschritt zählt seine Dateien abschließend auf, die Aussage steht
in einer anderen Datei, und der Ausführer hält sich zu Recht an die Liste. Sechs Stellen sind
so erklärt und in zwei Datensätzen erfasst
(`260813-1420_o_vier-modulkoepfe-…` und `260813-1258_o_zwei-prosastellen-in-anwendung-rs-…`).

**Die Erklärung deckt den Querschnitt nicht vollständig ab.** Zwei weitere Stellen stehen in
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, also in genau der Datei, die A1 als einzige
nennt. Sie sind dem Ausführer nicht durch eine Schrittgrenze entgangen, sondern beim Lesen
derselben Datei.

---

**Schwere:** niedrig. Kein Verhalten, kein Bau. Beide widersprechen einer richtigen Aussage
zwanzig Zeilen weiter in derselben Datei.

**1. Die Tafel hat Achtel und keine Viertel mehr** (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:297-300`):

```
/// [`Kommando::wirkungsbereich`], und sie haelt daneben fest, dass keiner
/// der sieben eine der beiden Ausnahmen traegt. Ohne das zweite koennte ein
/// Stellvertreter die drei abweisenden Viertel der Tafel gruen faerben,
/// ohne dass die Regel sie traegt.
```

A1 hat das Viertel zum Achtel gemacht: `let achtel: [(bool, bool, bool, [[bool; 5]; 7]); 8]`
(`:410`), und `die_tafel_aus_zweihundertachtzig_faellen_geht_auf` (`:387`) prüft 280 Fälle
(`:435`). Sieben der acht Achtel weisen ab, nicht drei von vier. Der Doc-Kommentar der
Tafelprobe sagt es selbst richtig (`:384`: „Dass die sieben Achtel wirklich leer sind"); die
beiden Stellen widersprechen sich in derselben Datei.

**2. Die Regel hat vier Bestandteile** (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:459`):

```
/// **Der Fall, um dessentwillen die Regel drei Bestandteile hat.**
```

Der Satz steht am Doc der Probe `beim_umbenennen_in_der_liste_wirkt_kein_befehl_des_dateifensters`.
Gemeint ist Bestandteil (2), den der Folgesatz auch nennt; falsch ist allein die Zahl. Der
Modulkopf derselben Datei trägt seit A1 die Überschrift „# Die vier Bestandteile" (`:29`) und
zählt (4) einzeln auf (`:41-43`).

**Eine dritte Stelle, in einer anderen Datei und im Grenzbereich**
(`crates/krk-ui/src/appkit/anwendung.rs:2690-2693`): „Bis zur Runde 7 standen hier zwei
getrennte Vorbehalte … waehrend der dritte Bestandteil im Ereignisabgriff wohnte; alle drei
stehen jetzt in der einen Regel." Der Satz beschreibt einen historischen Vorgang und zählt
richtig, was damals zusammengeführt wurde; drei Zeilen darunter steht „Die vier Bestandteile"
(`:2695`). Als Umfangsangabe gelesen führt er trotzdem in die Irre. Er gehört zu den beiden
Stellen aus `260813-1258_o_zwei-prosastellen-in-anwendung-rs-…` und ist dort als dritter Punkt
mitzunehmen.

**Was zu tun ist**

Die erste Stelle auf „die sieben abweisenden Achtel" bringen, die zweite auf „vier
Bestandteile". Beide, sobald ein Schritt `zulaessigkeit.rs` ohnehin öffnet.

**Was der Befund über den Querschnitt sagt**

Die Abhilfe, die die Durchsicht vorschlägt — „ein Schritt, der eine gezählte Aussage ändert,
braucht in seiner Dateiliste die Dateien, die die Zahl nennen" — greift für diese beiden nicht.
Die Datei stand in der Liste. Was hier fehlt, ist die Suche nach der Zahl **innerhalb** der
geänderten Datei, und die kostet nichts: `grep -n 'drei\|Viertel' zulaessigkeit.rs` hätte beide
geliefert. Die Dateiliste ist damit die eine Hälfte der Abhilfe und nicht die ganze.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Berührt C5.6 nicht in der Sache, wohl aber die Genauigkeit der einen Stelle, an der die Regel
  erklärt wird.
- Schwesterbefunde: `260813-1258_o_zwei-prosastellen-in-anwendung-rs-zaehlen-noch-drei-werte-und-einen-sonderposten.md`,
  `260813-1420_o_vier-modulkoepfe-ausserhalb-der-dateiliste-von-a1-und-a2-nennen-noch-drei-bestandteile.md`.

Also seen: 260826-1417 by coderev — `zulaessigkeit.rs:587` spricht von „ein kuenftiger dritter Eintrag" der Ausnahmeliste; die Liste führt seit `FensterEinblenden` drei, gemeint ist ein vierter (dieselbe Datei, dieselbe Ursache wie Punkt 2).

---
Resolved: Behoben, beide Stellen und die dritte aus der "Also seen"-Zeile. (1) `crates/krk-ui/src/kommandos/zulaessigkeit.rs` sagt am Doc-Kommentar von `STELLVERTRETER` jetzt "die sieben abweisenden Achtel der Tafel" statt "die drei abweisenden Viertel"; das deckt sich mit dem Doc der Tafelprobe, der "Dass die sieben Achtel wirklich leer sind" schon sagte. (2) Der Doc-Kommentar von `beim_umbenennen_in_der_liste_wirkt_kein_befehl_des_dateifensters` sagt jetzt "um dessentwillen die Regel den Bestandteil (2) hat" statt "drei Bestandteile" — das ist genauer als "vier", weil der Folgesatz genau diesen Bestandteil meint. (3) Die "Also seen"-Zeile vom 260826-1417: "ein kuenftiger dritter Eintrag" heisst jetzt "ein kuenftiger weiterer Eintrag", denn vier Zeilen darueber steht "alle drei heutigen Eintraege". Die dritte Stelle in `anwendung.rs` ist mit dem Schwesterdatensatz 260813-1258_o_zwei-prosastellen-in-anwendung-rs behoben. **Daneben gefunden und mitgenommen:** vier Stellen derselben Datei und eine in `menue.rs` sprachen von einer "Tafel aus 280 Faellen" und einem "zweihundertachtzigmal"; die Tafel deckt am `2fa1d0e` acht Achtel mal acht Stellvertreter mal sechs Fokuswerte, und die Probe heisst laengst `die_tafel_aus_allen_faellen_geht_auf` und rechnet die Zahl aus dem Produkt der drei Aufzaehlungen. Alle fuenf Stellen sagen jetzt "ueber alle Faelle" und verweisen fuer die Zahl auf jene Probe. Geprueft: cargo test -p krk-ui 908 gruen, clippy/fmt/doc je Exit 0.

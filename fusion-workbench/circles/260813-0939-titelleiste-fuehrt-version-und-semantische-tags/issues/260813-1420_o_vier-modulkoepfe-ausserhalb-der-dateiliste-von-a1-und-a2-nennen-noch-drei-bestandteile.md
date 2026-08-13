Vier Stellen außerhalb der Dateiliste von A1 und A2 nennen die Regel noch mit drei Bestandteilen

---

Die Schritte A1 und A2 sind umgesetzt: `kommandos::zulaessigkeit::Lage` trägt
das vierte Feld `schluesselfenster_gehoert_krk`, `zulaessig` fragt es innerhalb
des `durchgelassen`-Ausdrucks, und die Tafel deckt 280 Fälle statt 140. Beide
Schritte nennen als Dateien nur `crates/krk-ui/src/kommandos/zulaessigkeit.rs`
und `crates/krk-ui/src/appkit/anwendung.rs`, und der Ausführer hat sich daran
gehalten.

Vier Textstellen in drei anderen Dateien beschreiben dieselbe Regel und stehen
seitdem falsch da. Keine hält den Bau an, keine ändert ein Verhalten, jede
führt den nächsten Leser in die Irre:

1. `crates/krk-ui/src/appkit/menue.rs:1110` — „die Tafel aus 140 Faellen dazu
   steht in `crates/krk-ui/src/kommandos/zulaessigkeit`". Die Tafel deckt jetzt
   280 Fälle, und die Probe heißt
   `die_tafel_aus_zweihundertachtzig_faellen_geht_auf`.
2. `crates/krk-ui/src/kommandos/mod.rs:25` — „`fokus` ist einer ihrer drei
   Bestandteile geworden". Es sind vier.
3. `crates/krk-ui/src/appkit/ereignisse.rs:90` — „die `Lage` aus Blattstand,
   Ersthelferbefund und Fokus" und „dieselben drei Werte". Die `Lage` trägt vier
   Werte; der Zeichenzweig liest weiterhin drei davon.
4. `crates/krk-ui/src/appkit/ereignisse.rs:103` — der Absatz „Ein stehendes
   Blatt und der Ersthelfer sind zwei verschiedene Fragen" zählt die
   Bestandteile (1) bis (3) auf und nennt (4) nicht. Der Plan verlangt in A2
   ausdrücklich, dass dieser Abschnitt die neue Frage nennt; die Zeile steht in
   `ereignisse.rs` und nicht in der Dateiliste von A2, die allein
   `anwendung.rs` führt.

Die Nummerierung (1) bis (3) in `ereignisse.rs` bleibt richtig: der neue
Bestandteil ist als (4) angehängt worden, damit die bestehenden Verweise nicht
verrutschen.

**Was zu tun ist**

Die vier Stellen nachziehen, sobald ein Schritt diese Dateien ohnehin öffnet.
`menue.rs` fällt in Strang C dieser Runde an, `ereignisse.rs` und
`kommandos/mod.rs` in keinem Schritt.

**Kontext**

- Gefunden beim Umsetzen von A1 und A2 der Runde 8; nicht behoben, weil beide
  Schritte ihre Dateien abschließend aufzählen und ein zweiter `coder`
  gleichzeitig im Baum arbeitet.

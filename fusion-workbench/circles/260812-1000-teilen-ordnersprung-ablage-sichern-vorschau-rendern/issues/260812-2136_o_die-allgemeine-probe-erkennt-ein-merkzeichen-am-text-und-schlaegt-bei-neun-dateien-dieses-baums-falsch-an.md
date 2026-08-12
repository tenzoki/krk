Die allgemeine Probe erkennt ein Merkzeichen am Text und schlägt bei neun Dateien dieses Baums falsch an

---

`kein_merkzeichen_liegt_im_bereich_eines_stueckes`
(`crates/krk-ui/src/markdown.rs:1327-1372`) ist der Gurt um die Klasse des
Hauptbefunds, und er hält. Sein Erkenner `beginnt_mit_merkzeichen`
(`markdown.rs:1374-1383`) entscheidet aber am **Text** und nicht am
gerenderten Merkzeichen: er fragt, ob ein Ausgabestück mit `• ` oder mit
Ziffern-Punkt-Leerzeichen anfängt. Steht dieselbe Zeichenfolge wörtlich in der
Quelle, hält er sie für ein Merkzeichen.

Damit lässt sich der Gurt nicht mit beliebigen Quellen weiten. Genau das ist
aber das Versprechen seines Doc-Kommentars: „sie faengt die ganze Klasse, ohne
dass jemand den Einzelfall nennen muesste."

---

**Gemessen.** Der Erkenner der Probe, unverändert übernommen, über alle 773
Markdown-Dateien dieses Baums (ohne `target/` und `.git/`), gegen die
Ausgabe von `markdown::rendern` aus `2c0b2a6`: **neun Dateien** lösen ihn aus,
und in allen neun zu Unrecht. Sie tragen ein wörtliches `• ` in fester
Schrift, etwa in

```
crates/krk-ui/src/markdown.rs   (im Prüfmodul zitiert)
…/issues/260812-2019_c_ein-leerer-listenpunkt-…md
…/reviews/260812-2019-coderev-turn-4-der-runde-6.md
```

Der Bereich `FesteSchrift` deckt dort `"• "`, weil in der Quelle
`` `• ` `` steht. Nähme jemand eine dieser Dateien in die dreizehn Quellen
auf — und eine Datei dieses Projekts ist die naheliegendste realistische
Quelle —, so schlüge die Probe fehl, ohne dass etwas falsch wäre.

**Der Gurt selbst hält, und das ist nachgemessen.** Über 648 systematisch
erzeugte Quellen der Klasse (sechs Merkzeichenformen mal achtzehn erste Kinder
mal sechs Umgebungen) findet die neue Fassung null Verstöße, die Fassung
`1e4e01f` dagegen 480. Über 400 000 zufällig zusammengesetzte Quellen findet
die neue Fassung null. Alle dreizehn Quellen der Probe fallen unter `1e4e01f`
durch — die Zusage „hätte den Defekt gefangen" ist also nicht nur für einige,
sondern für jede der dreizehn richtig.

**Zwei Dinge sieht der Gurt zusätzlich nicht**, und keines davon steht bei ihm:

1. **Ein Merkzeichen mitten in einem Bereich.** Geprüft wird nur der Anfang.
   Dass ein Merkzeichen heute gar nicht in die Mitte eines Bereichs geraten
   kann, hängt daran, dass jeder Vorfahre eines Listenpunkts ein Containerblock
   ist — Liste, Punkt, Zitat — und niemals eine Betonung, eine Überschrift oder
   ein Verweis. Das trägt, steht aber nirgends.
2. **Den umgekehrten Fehler.** Verlöre eine `Listenzeile` ihr eigenes
   Merkzeichen, weil der Nachzug einen Eintrag zu viel verschöbe, so bliebe der
   Gurt still: `Listenzeile` ist von der Prüfung ausgenommen. Gemessen ist
   dieser Fall nur in den acht Einzelproben daneben. (Am Baum tritt er nicht
   auf: über dieselben 773 Dateien tragen in beiden Fassungen 6 634 von 6 732
   `Listenzeile`-Bereichen ihr Merkzeichen, die 98 übrigen sind Zitatblöcke,
   und die Zahlen sind in beiden Fassungen gleich.)

**Ein Zuschnitt** (nicht gewählt): die Stellen, an denen
`Zerlegung::merkzeichen_einloesen` schreibt, im Prüfmodul mitführen und den
Gurt gegen diese Stellen fragen statt gegen den Text. Das ist eine entschiedene
Frage statt einer geratenen, und sie beantwortet zugleich Punkt 1, weil eine
Stelle mitten in einem Bereich genauso auffiele wie eine an seinem Anfang. Zu
prüfen wäre, wie die Stellen aus `rendern` herauskommen, ohne dass die
Auslieferung ein Feld dafür trägt.

**Gewicht: niedrig.** Heute ist nichts falsch, und der Gurt hält für alles, was
er heute misst. Der Befund ist, dass er nicht weiter zu füllen ist als bis zu
Quellen, die kein Merkzeichen wörtlich enthalten — und dass sein Doc-Kommentar
das nicht sagt.

**Herkunft:** Circle der Runde 6, Turn 5, `2c0b2a6`.

Quelltext, der kein Ereignis mit Zeichen erzeugt, verschwindet spurlos aus der gerenderten Vorschau

---

Das dritte Abnahmekriterium von C4 sagt zu: „Alles außerhalb dieses Umfangs
erscheint als der Quelltext, der dasteht." Die Auffangregel in
`crate::markdown` hängt an einem **Ereignis** des Zerlegers. Quelltext, zu dem
`pulldown-cmark` gar kein Ereignis liefert oder ein Ereignis ohne Zeichen,
fällt durch sie hindurch und erscheint überhaupt nicht — weder gerendert noch
als sein Quelltext.

---

**Zwei Fälle, beide am Baum gemessen** (mit `markdown::rendern` aus
`crates/krk-ui/src/markdown.rs:104`, unverändert in ein Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`):

1. **Eine Verweisdefinition erzeugt kein einziges Ereignis.**

   ```
   Quelle : "Text davor.\n\nSiehe [den Text][ref] hier.\n\n[ref]: https://example.com \"Titel\"\n"
   Ausgabe: "Text davor.\n\nSiehe den Text hier."
   ```

   Die Zeile `[ref]: https://example.com "Titel"` ist aus der Anzeige
   verschwunden. Der Zerleger verbraucht sie beim Auflösen des Verweises und
   meldet sie nicht; die Schleife in `rendern` (`markdown.rs:105-142`) bekommt
   nichts, worauf die Auffangregel greifen könnte.

   Der schärfste Fall ist eine Datei, die nur aus Definitionen besteht:

   ```
   Quelle : "[ref]: https://example.com\n[zwei]: https://b.example\n"
   Ausgabe: ""
   ```

   Die Vorschau zeigt eine leere Fläche für eine Datei mit Inhalt.

2. **Ein Verweis ohne Text erzeugt ein Ereignis ohne Zeichen.**

   ```
   Quelle : "Siehe [](https://example.com) dort.\n"
   Ausgabe: "Siehe  dort."
   ```

   `Zerlegung::schliessen` (`markdown.rs:632-660`) trägt bei
   `laenge == 0` nichts ein, und geschrieben hat der Verweis auch nichts. Die
   23 Zeichen `[](https://example.com)` sind weg.

**Warum das mehr ist als eine Schönheitsfrage.** Der Plan begründet die Wahl
von `pulldown-cmark` gegen `syntect` ausdrücklich damit, dass bei jenem fremde
Sprachdefinitionen entschieden, welche Zeichen verschwinden: „ein fälschlich
ausgeblendetes Zeichen ist eine falsche Auskunft über den Inhalt einer Datei"
(Wurzel-`Cargo.toml`, Begründung an der Versionsangabe; Plan,
`## Womit die Vorschau Markdown zerlegt`). Genau diese Wirkung tritt hier ein,
nur an einer anderen Stelle.

**Warum die Totalitätszusage sie nicht abdeckt.** Der Modulkopf von
`markdown.rs:41-47` und die `Decidability`-Zeile des Plans sagen, die
Fallunterscheidung sei total, weil alles außerhalb des Umfangs als sein
Quellbereich erscheine. Die Totalität gilt über `Event` und `Tag`. Sie gilt
**nicht** über die Zeichen der Datei: es gibt Quellbytes, die in keinem
Ereignis vorkommen, und diese Lücke ist in Plan und Modulkopf nicht benannt.

**Zwei verschiedene Ursachen, ein Kriterium.** Fall 1 hat kein Ereignis, an das
sich eine Regel hängen ließe — die Deckung müsste über die Quellbereiche der
gelieferten Ereignisse laufen (was zwischen dem Ende eines Bereichs und dem
Anfang des nächsten liegt, ist wörtlich auszugeben). Fall 2 ist eine Zeile in
`schliessen`: ein Element ohne Zeichen gibt seinen Quellbereich aus, statt zu
verschwinden. Beide gehören in eine Antwort, sonst wird die halbe gebaut.

**Gewicht:** mittel. Kein Absturz und kein Datenverlust, aber die Vorschau gibt
über den Inhalt einer Datei falsche Auskunft, und der Fall mit den
Verweisdefinitionen kommt in jeder README vor, die Verweise in Kurzform führt.

**Herkunft:** Circle der Runde 6, Planschritt 8 (C4.3).

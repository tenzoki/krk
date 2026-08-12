Die Deckungszusage von `markdown.rs` gilt nicht innerhalb eines Elements, das Zeichen geliefert hat — eine Verweisdefinition im Listenpunkt verschwindet weiterhin

---

Der Modulkopf von `crates/krk-ui/src/markdown.rs` (Abschnitt
`# Die Deckung: kein Quellbyte faellt heraus`, Zeilen 56–86) sagt zu, zwei
Sätze träfen „zusammen jedes Byte". Sie tun es nicht. Es gibt einen dritten
Fall, den keiner von beiden erreicht: **eine Lücke innerhalb eines Elements,
das Zeichen geliefert hat.** Quelltext dort verschwindet spurlos, genau wie vor
der Behebung von `260812-1805`.

---

**Zwei Fälle, am Baum gemessen** (`markdown::rendern` aus
`crates/krk-ui/src/markdown.rs:152`, unverändert in ein Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`, Tafel Hell):

1. **Eine Verweisdefinition in einem Listenpunkt.**

   ```
   Quelle : "- [ref]: http://a.example\n"
   Ausgabe: "• "
   ```

   Die Zeile ist weg; übrig bleibt das Merkzeichen, das diese Runde erst
   eingeführt hat. Vor `a9e1149` lautete die Ausgabe `""` — der Verlust ist
   also nicht neu, aber die Zusage, dass es ihn nicht mehr gibt, ist neu.

2. **Eine Verweisdefinition in einem Zitatblock.**

   ```
   Quelle : "> Zitat\n>\n> [ref]: http://a.example\n"
   Ausgabe: "Zitat"
   ```

   Vor `a9e1149` dieselbe Ausgabe.

**Warum beide Sätze der Deckung danebengreifen.**

- Satz 1 steht in `Zerlegung::luecke_bis` (`markdown.rs:514-527`) und beginnt
  mit `if !self.offen.is_empty() { return; }`. Im Listenpunkt und im
  Zitatblock steht ein Element offen, also greift er nicht.
- Satz 2 steht in `Zerlegung::schliessen` (`markdown.rs:613-645`) und gibt den
  Quellbereich nur bei `laenge == 0` heraus. Der Punkt hat sein Merkzeichen
  geschrieben, also ist `laenge` gleich 2, und der Zitatblock hat „Zitat"
  geliefert.

**Die Prämisse, auf der Satz 1 seine Grenze zieht, ist zu stark.** Der
Modulkopf begründet die Einschränkung „nur auf Dokumentebene" so: „hat es
Zeichen geliefert, sind die Luecken darin seine Auszeichnungszeichen und
gehoeren weg" (`markdown.rs:76-79`). Für einen Verweis stimmt das — das `[`
und das `][ref]` gehören weg. Für einen **Containerblock** stimmt es nicht:
zwischen den Kindern eines Listenpunkts oder eines Zitats kann Quelltext
stehen, der keine Auszeichnung ist und den die Kiste dennoch nicht meldet.
Der Unterschied ist der zwischen einem Element, dessen Quellbereich es selbst
ausfüllt, und einem, das andere enthält.

**Die Gegenprobe hält weiterhin.** `die_zeichen_eines_gerenderten_elements_bleiben_weg`
(`markdown.rs:984`) prüft den Verweis in Kurzform, und der muss weiter
verschwinden. Ein Zuschnitt, der Satz 1 einfach auf alle Ebenen ausdehnte,
bräche diese Probe. Die tragfähige Unterscheidung ist die zwischen einem
Element, das Text **enthält**, und einem, das Text **ist**; im Zuschnitt der
Datei ist das die zwischen `Behandlung::Block`/`Zitat`/`Liste`/`Punkt` und
`Stueck`/`Verweis`.

**Was zu tun ist**, in dieser Reihenfolge:

1. **Die Zusage im Modulkopf berichtigen**, damit sie nicht behauptet, was
   nicht gilt. Das ist derselbe Fehlertyp, den `CLAUDE.md` unter „Was man nicht
   sieht" führt: eine Zusicherung im Code, die eine Unmöglichkeit behauptet,
   die eintritt. Sie kostet den nächsten Leser die Nachprüfung.
2. **Entscheiden, ob die Lücke geschlossen wird.** Der eine mechanische
   Zuschnitt wäre, `luecke_bis` nicht am leeren `offen` festzumachen, sondern
   an der Art des innersten offenen Elements: ein Element, das Kinder
   enthält, deckt seine Zwischenräume, eines in der Zeile nicht. Das ist eine
   Bedingung und keine Liste von Ereignisarten, bleibt also so mechanisch wie
   die heutige Regel.

**Gewicht:** mittel. Kein Absturz, aber die Vorschau gibt über den Inhalt einer
Datei falsche Auskunft, und der Modulkopf sagt ausdrücklich zu, dass sie es
nicht mehr tut. Die betroffenen Quellen sind seltener als die des Defekts
`260812-1805` — eine Verweisdefinition steht meist am Dateiende, nicht in einem
Listenpunkt —, aber die falsche Zusage wiegt unabhängig von der Häufigkeit.

**Herkunft:** Circle der Runde 6, Turn 3, Behebung von
`260812-1805_c_quelltext-ohne-ereignis-verschwindet-spurlos-aus-der-gerenderten-vorschau.md`.

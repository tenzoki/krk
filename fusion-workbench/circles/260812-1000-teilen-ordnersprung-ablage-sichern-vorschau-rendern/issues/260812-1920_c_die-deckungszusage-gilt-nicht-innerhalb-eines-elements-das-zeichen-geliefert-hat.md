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

---

**Resolved 260812** — beides ist getan: die Luecke ist geschlossen, soweit sie
mechanisch zu schliessen war, und der Modulkopf sagt jetzt genau, wo die
Deckung endet.

**Die Unterscheidung, die traegt, ist die von CommonMark** zwischen einem
Containerblock und einem Blattblock. Sie steht als `Inhalt` an jedem `Offen`:
`Inhalt::Bloecke` fuer Zitatblock, Liste und Listenpunkt, `Inhalt::Zeichen`
fuer Absatz, Ueberschrift, Quelltextblock, Betonung und Verweis. `luecke_bis`
fragt nicht mehr, ob `offen` leer ist, sondern nach dem `Inhalt` des innersten
Elements; `schliessen` gibt bei `Inhalt::Bloecke` zusaetzlich heraus, was
zwischen dem letzten Kind und dem Ende ungelesen blieb. Letzteres braucht es,
weil beim Endereignis `luecke_bis` nicht greift: der Quellbereich eines
Endereignisses beginnt am **Anfang** des Elements.

**Die Gegenprobe haelt.** `die_zeichen_eines_gerenderten_elements_bleiben_weg`
laeuft unveraendert durch, denn ein Verweis traegt `Inhalt::Zeichen`.

**Beide gemessenen Faelle, am Baum nachgemessen:**

```
"- [ref]: http://a.example\n"            -> "- [ref]: http://a.example\n"
"> Zitat\n>\n> [ref]: http://a.example\n" -> "Zitat\n\n[ref]: http://a.example"
```

Der erste kommt nicht aus der neuen Regel, sondern aus dem aufgeschobenen
Merkzeichen (Datensatz
`260812-1920_c_in-einer-losen-liste-steht-das-merkzeichen-allein-auf-seiner-zeile.md`): der Punkt
liefert kein Zeichen, also tritt nach dem dritten Satz der Deckung sein
Quellbereich an seine Stelle, mit dem `- ` der Quelle. Ein noch ausstehendes
Merkzeichen faellt mit dem Punkt weg, sonst stuende `• - [ref]: …` da.

**Das `>` eines Zitats faellt zeilenweise weg** (`ohne_umgebungszeichen`).
Ohne diesen Griff stuende zwischen zwei Absaetzen eines Zitats das nackte `>`
seiner Leerzeile; die Probe
`ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text` haelt
das fest. Das Merkzeichen einer Liste steht **nicht** in dieser Menge, denn
`-` und `1.` koennen der Anfang einer Zeile sein, die dasteht.

**Wo die Deckung jetzt endet, und das ist genau eine Stelle:** der Vorspann
eines Containerblocks, also alles von seinem Anfang bis zum ersten Byte, das
darin gelesen wird. Dort steht sein Merkzeichen, und es gehoert weg — aber
eine Verweisdefinition **vor** dem ersten Absatz eines Punktes faellt mit
heraus:

```
"- [ref]: http://a.example\n\n  Text\n"   -> "• Text"
"> [ref]: http://a.example\n>\n> Zitat\n" -> "Zitat"
```

Das ist keine Verschlechterung gegenueber `a9e1149` — dort war der ganze
Container ungedeckt —, sondern der Rest, und er ist an drei Stellen benannt:
im Modulkopf unter „Wo die Deckung endet", am Doc-Kommentar von `luecke_bis`
und in der Probe `im_vorspann_eines_elements_endet_die_deckung`, die beide
Ausgaben oben festschreibt. Eine Zusage, die weiter reicht als der Code, gibt
es damit nicht mehr; wer die Grenze verschieben will, braucht eine Regel, die
das Merkzeichen des Containers vom Quelltext davor trennt, und die ist nicht
mechanisch zu haben.

**Neue Proben in `crates/krk-ui/src/markdown.rs`**:
`ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen`,
`eine_verweisdefinition_am_ende_eines_zitats_bleibt_stehen`,
`ein_zitat_aus_zwei_absaetzen_traegt_seine_zeichen_nicht_in_den_text`,
`im_vorspann_eines_elements_endet_die_deckung` und
`eine_verweisdefinition_hinter_dem_absatz_eines_punktes_bleibt_stehen`.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0. Das Binaerziel `krk` steht bei 466 Proben statt 457.

---

**Nachtrag 260812-2140 zur Abschlussnotiz: die Aufzählung heißt `Inhaltsart`
und nicht `Inhalt`.** Die Notiz darüber nennt sie an sechs Stellen `Inhalt`
und ihre Werte `Inhalt::Bloecke` und `Inhalt::Zeichen`. Im Baum heißt sie seit
`c35f8b1` `Inhaltsart`, und der Name trägt sein `-art` aus einem genannten
Grund: `crate::vorschaumodell::Inhalt` bezeichnet in derselben Kiste etwas
anderes, nämlich die Art dessen, was die Vorschau anzeigt. Wer im Text oben
nach `Inhalt` greift, landet beim falschen Typ. Zu lesen ist überall
`Inhaltsart`, `Inhaltsart::Bloecke` und `Inhaltsart::Zeichen`.

Der Text der Notiz bleibt stehen, wie er geschrieben wurde: dieser Datensatz
ist die Aufzeichnung eines Standes, und `CLAUDE.md` hält solche Aufzeichnungen
unverändert. Berichtigt wird deshalb hier und nicht dort.

**Zwei Sätze der Notiz haben sich seither überholt**, und zwar durch die
Behebung von `260812-2019_c_ein-leerer-listenpunkt-zeigt-sein-rohes-bindestrich-zeichen-und-verliert-seinen-einzug.md`:

- „Der erste kommt nicht aus der neuen Regel, sondern aus dem aufgeschobenen
  Merkzeichen … Ein noch ausstehendes Merkzeichen fällt mit dem Punkt weg" —
  das gilt weiterhin für einen Punkt, dessen Quellbereich mehr trägt als sein
  Merkzeichen. Trägt er nichts weiter, wird der Wunsch jetzt doch eingelöst,
  und ein leerer Punkt steht als `• ` da statt als rohes `- `.
- „wer die Grenze verschieben will, braucht eine Regel, die das Merkzeichen
  des Containers vom Quelltext davor trennt, und die ist nicht mechanisch zu
  haben" — für den Fall, dass hinter dem Merkzeichen **nichts** steht, ist sie
  mechanisch zu haben und steht als `traegt_nur_sein_merkzeichen` im Baum. Für
  einen Punkt mit Inhalt gilt der Satz unverändert.

Die beiden oben gemessenen Ausgaben dieses Datensatzes sind davon nicht
betroffen und stehen unverändert.

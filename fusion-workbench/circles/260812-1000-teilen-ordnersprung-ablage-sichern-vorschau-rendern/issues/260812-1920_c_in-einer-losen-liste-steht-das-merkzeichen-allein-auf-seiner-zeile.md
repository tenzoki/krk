In einer losen Liste steht das Merkzeichen allein auf seiner Zeile, durch eine Leerzeile von seinem Text getrennt

---

Steht zwischen zwei Listenpunkten eine Leerzeile — in CommonMark eine „lose"
Liste, und die häufigste Form in jeder README —, dann schreibt
`Zerlegung::punkt_oeffnen` das Merkzeichen, und der Absatz des Punktes drängt
sich danach mit zwei Umbrüchen dazwischen. Das Merkzeichen bleibt allein auf
seiner Zeile stehen.

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:152`,
unverändert in ein Prüfprogramm kopiert, `pulldown-cmark 0.13.4`):

```
Quelle : "- eins\n\n- zwei\n"
Ausgabe: "• \n\neins\n\n• \n\nzwei"
```

Dargestellt heißt das sechs Zeilen für zwei Listenpunkte:

```
•

eins

•

zwei
```

**Derselbe Fehler bei jedem Punkt mit mehr als einem Block:**

```
Quelle : "- Punkt\n\n  > Zitat\n"
Ausgabe: "• \n\nPunkt\n\nZitat"

Quelle : "- Punkt\n\n  [ref]: http://a.example\n"
Ausgabe: "• \n\nPunkt"
```

**Es ist eine Verschlechterung durch `a9e1149` und kein Altbestand.** Vor dem
Commit gab es kein Merkzeichen, und die Ausgabe war `"eins\n\nzwei"` — ohne
Merkzeichen, aber ohne verwaiste Zeile. Gemessen mit derselben Datei aus
`94a81bd`.

**Die Ursache.** `punkt_oeffnen` (`markdown.rs:578-589`) schreibt das
Merkzeichen unmittelbar in den Text. Bei einer **straffen** Liste liefert
`pulldown-cmark` danach den Inhalt als `Event::Text`, und alles steht in einer
Zeile. Bei einer **losen** Liste liefert die Kiste ein `Tag::Paragraph`, und
`behandlung` macht daraus `Behandlung::Block { umbrueche: ABSATZABSTAND }`
(`markdown.rs:289-292`). `rendern` ruft dafür `zerlegung.trennen(2)`
(`markdown.rs:162`), und weil der Text nach dem Merkzeichen nicht mehr leer
ist, wird der Wunsch nach zwei Umbrüchen eingelöst — zwischen Merkzeichen und
Text.

**Die Auszeichnung ist damit ebenfalls falsch.** Die `Listenzeile` deckt
`(0,8)`, also Merkzeichen, Leerzeile und Text; AppKit dehnt ein Absatzmerkmal
auf den ganzen Absatz aus, und das sind hier drei Absätze statt einem.

**Keine Probe fängt es.** Alle Listenproben in `markdown.rs` — von
`ein_listenpunkt_traegt_den_einzug_und_behaelt_sein_zeichen`
(`markdown.rs:809`) bis `eine_verschachtelte_liste_traegt_ihre_tiefe`
(`:843`) — benutzen straffe Listen. Eine lose kommt in keiner vor.

**Zwei Zuschnitte, keiner ist hier gewählt:**

1. **Der erste Absatz eines Punktes trennt nicht ab.** Ein Punkt merkt sich,
   dass er gerade eröffnet hat, und der erste `Behandlung::Block` darin ruft
   `trennen(0)` statt `trennen(2)`. Das ist eine Bedingung an einer Stelle und
   trifft straffe wie lose Listen gleich.
2. **Das Merkzeichen wird erst vor dem ersten Zeichen des Punktes
   eingelöst**, so wie `trennung` heute schon ein Wunsch und kein Text ist.
   Näher an der vorhandenen Mechanik, aber ein zweiter aufgeschobener Wunsch
   neben dem der Umbrüche.

**Gewicht:** hoch. Die lose Liste ist die verbreitetere der beiden Formen —
jede Liste, deren Punkte mehr als eine Zeile tragen, ist eine —, und das
Ergebnis ist auf den ersten Blick als kaputt zu erkennen. C4.2 sagt Listen
ausdrücklich zu.

**Herkunft:** Circle der Runde 6, Turn 3, `a9e1149` (Behebung von
`260812-1805_c_listen-verlieren-merkzeichen-nummerierung-und-verschachtelungstiefe.md`).

---

**Resolved 260812** — gewaehlt ist **Zuschnitt 2**: das Merkzeichen ist jetzt
ein Wunsch und kein Text. `Zerlegung::punkt_oeffnen` schreibt es nicht mehr,
sondern merkt es am `Offen` des Punktes vor (`Offen::merkzeichen`);
`Zerlegung::schreiben` loest es ein, und zwar **nach** `absetzen` und **vor**
dem Text. Damit steht es in jeder Liste unmittelbar vor dem Zeichen, zu dem es
gehoert, und liegt weiter innerhalb des Bereichs seiner Listenzeile — der
Einzug nimmt es mit, wie `einzugsmerkmal` es zusagt.

**Zuschnitt 1 ist verworfen**, weil er den zweiten Datensatz dieser Durchsicht
nicht mit erledigt haette: ein Punkt, der nur eine Verweisdefinition enthaelt,
liefert kein Zeichen, und nur ein **nicht** geschriebenes Merkzeichen laesst
den zweiten Satz der Deckung an ihn heran.

**Alle drei gemessenen Faelle des Datensatzes, am Baum nachgemessen:**

```
"- eins\n\n- zwei\n"                    -> "• eins\n\n• zwei"
"- Punkt\n\n  > Zitat\n"                -> "• Punkt\n\nZitat"
"- Punkt\n\n  [ref]: http://a.example\n" -> "• Punkt\n\n[ref]: http://a.example"
```

Die Leerzeile zwischen den Punkten der losen Liste bleibt: die Quelle traegt
sie, und `pulldown-cmark` verlangt sie ueber den Absatz des Punktes. Der
dritte Fall ist ueber den zweiten Satz der Deckung erledigt, siehe den
Datensatz `260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`.

**Die Auszeichnung stimmt wieder.** Die `Listenzeile` deckt `• eins` und
`• zwei` statt `(0,8)` ueber drei Absaetze; die Probe prueft die Bereiche
einzeln nach.

**Der eigentliche Befund des Datensatzes — keine Probe benutzte eine lose
Liste — ist behoben.** Neue Proben in `crates/krk-ui/src/markdown.rs`:
`eine_lose_liste_haelt_ihr_merkzeichen_bei_seinem_text` (mit den zwei
Bereichen), `eine_lose_geordnete_liste_haelt_ihre_nummer_bei_ihrem_text`,
`ein_punkt_aus_zwei_bloecken_haelt_sein_merkzeichen`,
`zwei_punkte_uebereinander_tragen_beide_ihr_merkzeichen` (`- - tief`, zwei
Wuensche vor demselben Zeichen) und
`eine_verweisdefinition_hinter_dem_absatz_eines_punktes_bleibt_stehen`.

Der Modulkopf sagt den Wunsch jetzt an der Stelle, an der er die
Einrueckebenen beschreibt, samt Grund.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0. Das Binaerziel `krk` steht bei 466 Proben statt 457.

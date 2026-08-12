Listen verlieren Merkzeichen, Nummerierung und Verschachtelungstiefe

---

`Auszeichnung::Listenzeile` ist ein fester Einzug von 20 Punkten und sonst
nichts. Damit verliert eine gerenderte Liste drei Dinge auf einmal: das
Aufzählungszeichen, die Nummer einer geordneten Liste und jede
Verschachtelungstiefe. Eine dreistufige Liste steht danach als drei Zeilen mit
demselben Einzug und ohne Merkzeichen da, und eine geordnete Liste ist von
einer ungeordneten nicht mehr zu unterscheiden.

---

**Gemessen am Baum** (`markdown::rendern` aus
`crates/krk-ui/src/markdown.rs`, unverändert in ein Prüfprogramm kopiert):

```
Quelle : "1. eins\n2. zwei\n3. drei\n"
Ausgabe: "eins\nzwei\ndrei"
          Listenzeile (0,4), (5,4), (10,4)
```

```
Quelle : "- eins\n- zwei\n  - drunter\n    - noch tiefer\n"
Ausgabe: "eins\nzwei\ndrunter\nnoch tiefer"
          Listenzeile (0,4), (5,24), (10,19), (18,11)
```

Die vier Bereiche im zweiten Fall überlappen einander, tragen aber alle
dasselbe Merkmal: `einzugsmerkmal`
(`crates/krk-ui/src/appkit/textmerkmale.rs:412-424`) baut einen frischen
`NSMutableParagraphStyle` mit `firstLineHeadIndent = headIndent = LISTENEINZUG`
(20,0), und `addAttributes:range:` ersetzt den Absatzstil, statt ihn zu
addieren. Vier Stufen bekommen deshalb genau einen Einzug.

**Warum das kein bloßes Umsetzen des Plans ist.** Der Plan schreibt
`List`/`Item` auf `Auszeichnung::Listenzeile` fest, und der Code tut das
richtig. Der Entscheidungsdatensatz
`decisions/260812-1000_a_welchen-umfang-von-markdown-rendert-die-vorschau.md`
nennt **verschachtelte Listen** aber ausdrücklich unter den drei teuren
Bestandteilen, die Möglichkeit 1 **nicht** enthält, mit der Begründung, sie
brauchten „eine Einrücktiefe, die die vorhandene Auszeichnungsmechanik nicht
kennt"; und Möglichkeit 1 sagt für alles Weitere zu: „erscheint als der Text,
der dasteht". Die Umsetzung tut ein Drittes: sie rendert die verschachtelte
Liste und macht sie dabei flach. Weder Plan noch Modulkopf halten diese Wahl
fest; ihre einzige Spur ist der Name der Probe
`eine_verschachtelte_liste_haengt_nicht_aneinander` (`markdown.rs:697`), die
das flache Ergebnis als richtig festschreibt.

**Was der Nutzer davon sieht.** Eine README mit einer nummerierten
Installationsanleitung steht in der Vorschau ohne Nummern; eine gegliederte
Liste steht ohne Gliederung. Der Editor derselben Runde zeigt beide **mit**
ihren Zeichen — die Vorschau soll nach dem Datensatz mehr können als er, nicht
weniger.

**Drei Zuschnitte sind erkennbar, keiner ist hier gewählt:**

1. **Das Merkzeichen behalten.** `Tag::Item` schreibt vor den Inhalt des
   Punktes ein „• " beziehungsweise die Nummer aus `Tag::List(Some(n))`. Kostet
   wenige Zeilen in `behandlung`/`Zerlegung` und löst Nummerierung und
   Unterscheidbarkeit, nicht die Tiefe.
2. **Die Tiefe in die Auszeichnung nehmen.** `Listenzeile { tiefe: u8 }`, und
   `einzugsmerkmal` rechnet `tiefe * LISTENEINZUG`. Berührt `hervorhebung.rs`,
   `markdown.rs` und `textmerkmale.rs`, also die Naht, die Schritt 7 gerade
   gezogen hat.
3. **Beim gewählten Umfang bleiben und es aufschreiben.** Dann gehört der Satz
   „Listen erscheinen eingerückt, ohne Merkzeichen, ohne Nummer und ohne
   Tiefe" in den Modulkopf von `markdown.rs` und in den Datensatz zum Umfang,
   damit die Abweichung eine Wahl ist und kein Nebenprodukt.

**Gewicht:** mittel. Kein Absturz, aber der gerenderte Text sagt über die
Struktur der Datei etwas anderes, als in ihr steht.

**Herkunft:** Circle der Runde 6, Planschritt 8 (C4.2, C4.3).

---

**Resolved 260812** — alle drei verlorenen Dinge sind zurueck: Merkzeichen,
Nummer und Tiefe. Gewaehlt sind **Zuschnitt 1 und 2 zusammen**; Zuschnitt 3,
die Beschraenkung aufzuschreiben, faellt damit weg.

**Nachgemessen.** Beide Ausgaben des Datensatzes stimmen am Baum genau so, die
vier ueberlappenden Bereiche `(0,4)`, `(5,24)`, `(10,19)`, `(18,11)`
eingeschlossen.

**Zuschnitt 1, das Merkzeichen.** `Zerlegung::punkt_oeffnen` schreibt vor den
Inhalt eines Punktes `• ` oder die Nummer seiner geordneten Liste. Die Nummer
steht in der `Ebene` der Liste, wird vom Punkt verbraucht und zaehlt weiter;
eine Liste, die bei 3 anfaengt, faengt bei 3 an. Das Merkzeichen liegt
**innerhalb** des Bereichs der Listenzeile, damit der Einzug es mitnimmt.

**Zuschnitt 2, die Tiefe.** `Auszeichnung::Listenzeile` traegt jetzt
`tiefe: u8`, und `einzugsmerkmal` in `crates/krk-ui/src/appkit/textmerkmale.rs`
rechnet `tiefe * LISTENEINZUG`. Gezaehlt wird die Tiefe aus den offenen
Elementen (`Zerlegung::tiefe`) und nicht in einem zweiten Zaehler daneben:
jedes einrueckende Element traegt eine `Ebene`, ihre Zahl ist die Tiefe. Ein
Zitatblock zaehlt mit, weil er genauso einrueckt.

**Der Einzug ist bei acht Ebenen gedeckelt** (`EINZUGSGRENZE`). Acht mal 20
Punkte sind 160, die Mindestbreite eines Bereichs der Fensterzeile; darueber
stuende die Zeile ganz ausserhalb der Vorschau, und Markdown laesst beliebig
tiefe Verschachtelung zu.

**Die Formatansicht des Editors gibt immer `tiefe: 1`**
(`crates/krk-ui/src/hervorhebung.rs`), denn dort steht die Einrueckung noch im
Text; ein zweiter Einzug darueber verschoebe die Zeile gegen ihre eigenen
Zeichen.

**Die ueberlappenden Bereiche tragen jetzt verschiedene Tiefen und kommen
richtig heraus**, weil die Sortierung in `Zerlegung::abschliessen` aussen vor
innen stellt und der tiefere Bereich zuletzt setzt. Bei gleichem Anfang **und**
gleicher Laenge — `> - Punkt im Zitat` liefert genau das — entscheidet der neue
dritte Schluessel, der Rang beim Oeffnen. Naeheres im Datensatz
`260812-1805_*_der-ueberschneidungssatz-in-textmerkmale-anwenden-gilt-seit-markdown-rs-nicht-mehr.md`.

**Das geht ueber den beschlossenen Umfang hinaus, und zwar bewusst.** Der
Datensatz `decisions/260812-1000_a_welchen-umfang-von-markdown-rendert-die-vorschau.md`
nennt verschachtelte Listen unter den drei teuren Bestandteilen, die
Moeglichkeit 1 nicht enthaelt, mit der Begruendung, sie brauchten „eine
Einruecktiefe, die die vorhandene Auszeichnungsmechanik nicht kennt". Die
Mechanik kennt sie jetzt, und sie hat zwei Zeilen gekostet. Der Umfangsdatensatz
ist damit in einem Punkt ueberholt; ihn nachzuziehen ist Sache des Nutzers, denn
die Wahl war seine.

**Neue Proben in `crates/krk-ui/src/markdown.rs`**:
`eine_geordnete_liste_behaelt_ihre_nummern`,
`eine_geordnete_liste_beginnt_bei_ihrer_eigenen_zahl`,
`eine_verschachtelte_liste_traegt_ihre_tiefe` (der gemessene Fall des
Datensatzes, mit den Tiefen 1, 1, 2, 3) und
`ein_punkt_im_zitat_liegt_eine_ebene_tiefer`. Vier vorhandene Proben tragen die
Merkzeichen jetzt in ihrer Erwartung, darunter
`eine_verschachtelte_liste_haengt_nicht_aneinander`, die das flache Ergebnis
festschrieb.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0.

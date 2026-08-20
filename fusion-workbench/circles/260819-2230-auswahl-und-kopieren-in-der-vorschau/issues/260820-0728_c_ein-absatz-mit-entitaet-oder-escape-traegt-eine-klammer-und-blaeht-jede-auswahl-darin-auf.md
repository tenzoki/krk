Ein Absatz mit Entität oder Escape trägt eine Klammer und bläht jede Auswahl darin auf die verworfene Möglichkeit 3 auf

---

`Zerlegung::klammer_verbuchen` (`crates/krk-ui/src/markdown.rs:1044-1075`) setzt die Klammer
eines Elements, sobald **irgendwo** in seinem Quellbereich Bytes abgetragen werden, die im
Text nicht wiederkehren und nicht bloß Leerraum sind. Ein gewöhnlicher Absatz erfüllt das,
sobald er eine HTML-Entität, ein Backslash-Escape oder einen Backslash-Zeilenumbruch trägt.
Damit fährt bei jeder Auswahl in einem solchen Absatz der **ganze Absatz** mit — genau die
Möglichkeit 3, die der Nutzer am 260819-2242 nicht gewählt hat
(`shared/decisions/260819-2216_a_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md`).

---

**Gemessen, nicht erschlossen.** Der Baum auf dem Stand `b28cdd6` wurde in eine Kopie unter
`/private/tmp` gelegt und dort um eine Probe erweitert; `cargo test -p krk-ui` liefert:

```
"Ein &amp; hier im Absatz mit vielen Woertern.\n"   Auswahl "vielen"
  -> "Ein &amp; hier im Absatz mit vielen Woertern.\n"      Klammer des Absatzes: true
"Ein \* Stern im Absatz mit vielen Woertern.\n"     Auswahl "vielen"
  -> "Ein \* Stern im Absatz mit vielen Woertern.\n"        Klammer des Absatzes: true
"Ein gewoehnlicher Absatz mit vielen Woertern.\n"   Auswahl "vielen"
  -> "vielen"                                              Klammer des Absatzes: false
```

Ein dritter Fall trifft denselben Weg: `"Zeile eins\\\nZeile zwei.\n"`, also der harte
Umbruch mit Backslash, setzt die Klammer des Absatzes ebenfalls auf `true`. Der harte
Umbruch mit zwei Leerzeichen tut es nicht, weil der Leerraum abgezogen wird.

**Warum es geschieht.** `pulldown-cmark` meldet für eine Entität ein eigenes `Event::Text`
mit dem Quellbereich der **ungelösten** Entität: `Text("&")` über `4..9` für `&amp;`.
`Zerlegung::schreiben` (`markdown.rs:1149-1167`) vergleicht `quelle[gelesen..bis]` mit dem
geschriebenen Stück, findet Ungleichheit und legt einen Abschnitt der Art
`Abschnittsart::Ersetzt` an; `kacheln` ruft `klammer_verbuchen`, das innerste offene Element
ist der Absatz, und `"&amp;".trim()` ist nicht leer. Beim Backslash-Escape liefert die Kiste
`Text("* Stern …")` über `5..23`, während der Lesestand bei `4` steht, also enthält der
Abschnitt das `\` und dasselbe geschieht.

**Was der Baum selbst dazu sagt.** Der Doc-Kommentar von `klammer_verbuchen`
(`markdown.rs:1055-1063`) begründet den Leerraum-Halbsatz ausdrücklich damit, dass sonst
„jeder Absatz eine Klammer trüge, und wieder wäre es Möglichkeit 3". Der Halbsatz fängt den
Zeilenumbruch am Absatzende ab, aber nicht die Entität und nicht das Escape. Die Probe
`eine_auswahl_in_einem_langen_absatz_liefert_nicht_den_absatz` (`markdown.rs:2646-2657`)
prüft genau diese Zusage — an einem Absatz ohne Entität und ohne Escape, und deshalb bleibt
sie grün.

**Wurzel und Richtung.** Der Begriff „Klammer" heißt im Plan und im Modulkopf, dass ein
Element an seinen **Rändern** Zeichen trägt, die beim Zerschneiden unbalanciert
zurückblieben. Umgesetzt ist statt dessen „irgendwo verdeckte Bytes". Eine Entität mitten in
einem Absatz zerschneidet nichts; sie steht ganz in dem Stück, das die Auswahl ohnehin
liefert. Die Richtung wäre, die Klammer an die Ränder des Elements zu binden — an einen
Vorspann vor dem ersten geschriebenen Zeichen oder einen Nachspann hinter dem letzten —
statt an jedes verdeckte Byte darin. Dabei ist mitzuprüfen, dass die heute richtigen Fälle
richtig bleiben: Überschrift, Betonung, Verweis, Listenpunkt, Zitat und Quelltextblock
tragen ihre Zeichen sämtlich an den Rändern.

**Der Befund `260820-0731_o_…` in diesem Speicher ist die Gegenrichtung derselben Stelle**
und gehört mit ihr zusammen behoben.

**Schwere:** hoch. Kein Absturz, aber das Ergebnis widerspricht der Nutzerentscheidung, es
ist still, und `&amp;`, `&nbsp;`, `&#8212;` und `\*` stehen in gewöhnlichem Markdown.
**Baumstand:** `b28cdd6`.

---
Resolved: 260820-0803, coder, `crates/krk-ui/src/markdown.rs`. Die Wurzel ist behoben und nicht das
Symptom: die Klammer hängt jetzt am **Vorspann und am Nachspann** eines Elements — den Bytes vor
dem ersten und hinter dem letzten Ereignis in seinem Quellbereich — und nicht mehr an der Art eines
beliebigen Abschnitts in seinem Inneren. `Zerlegung::klammer_verbuchen` ist durch
`Zerlegung::ereignis_verbuchen` und die reine Funktion `klammer_der_raender` ersetzt;
`Abschnittsart::verdeckt_quelle` ist mit weggefallen, weil die Art eines Abschnitts über die
Klammer nichts mehr entscheidet. Dazu trägt `Zerlegung::luecke_bis` die Lücken innerhalb eines
Elements aus Zeichen jetzt ebenso ab wie den Vorspann eines Elements aus Blöcken — ohne etwas zu
schreiben, aber so, dass das Stück dahinter Zeichen für Zeichen an seiner Quelle steht; ohne diesen
Teil lieferte die Auswahl im Escape-Fall weiterhin den halben Absatz statt der zwei Wörter.

Gemessen: `"Ein &amp; hier im Absatz mit vielen Woertern.\n"` mit der Auswahl `"vielen"` liefert
`"vielen"`, ebenso der Escape `\*` und der harte Umbruch mit Backslash. Die Probe
`eine_entitaet_oder_ein_escape_im_absatz_blaeht_die_auswahl_nicht_auf` hält alle drei und die
Klammer des Absatzes; sie ist vor der Behebung gefahren worden und war rot. `make check` exit 0.
Protokoll: `history/260820-0803-coder-klammer-an-den-raendern.md`.

Der Befund `260820-0731_*_eine-ueberschrift-die-mit-einem-kind-beginnt-…` ist mit derselben
Änderung geschlossen, wie hier vorgesehen.

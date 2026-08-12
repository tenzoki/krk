Der geschlossene Deckungs-Datensatz nennt die Aufzählung fünfmal `Inhalt`, im Baum heißt sie `Inhaltsart`

---

Die Abschlussnotiz von
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`
nennt die neue Aufzählung durchweg `Inhalt` und ihre Werte `Inhalt::Bloecke`
und `Inhalt::Zeichen`. Im Baum heißt sie `Inhaltsart`, und der Name trägt sein
`-art` aus einem genannten Grund: `crate::vorschaumodell::Inhalt` bezeichnet in
derselben Kiste etwas anderes. Wer den Datensatz liest und `Inhalt` sucht,
findet den falschen Typ.

---

**Gelesen** (Datensatz gegen Baum, Stand `c35f8b1`):

| Datensatz sagt | Baum trägt |
|---|---|
| „Sie steht als `Inhalt` an jedem `Offen`" | `Inhaltsart` (`markdown.rs:446`), Feld `inhalt` (`markdown.rs:483`) |
| „`Inhalt::Bloecke` fuer Zitatblock, Liste und Listenpunkt" | `Inhaltsart::Bloecke` |
| „`Inhalt::Zeichen` fuer Absatz, …" | `Inhaltsart::Zeichen` |
| „fragt … nach dem `Inhalt` des innersten Elements" | fragt nach `eintrag.inhalt`, Typ `Inhaltsart` |
| „gibt bei `Inhalt::Bloecke` zusaetzlich heraus" | `Inhaltsart::Bloecke` |
| „denn ein Verweis traegt `Inhalt::Zeichen`" | `Inhaltsart::Zeichen` |

Sechs Stellen in einer Notiz. Ein bestehender Typ desselben Namens:

```
grep -rn "enum Inhalt" crates/krk-ui/src/
-> crates/krk-ui/src/vorschaumodell.rs  (die Art dessen, was die Vorschau anzeigt)
-> crates/krk-ui/src/markdown.rs        enum Inhaltsart
```

Der Doc-Kommentar von `Inhaltsart` benennt die Verwechslungsgefahr
ausdrücklich (`markdown.rs:437-439`): „Der Name traegt sein `-art`, weil
`crate::vorschaumodell::Inhalt` in derselben Kiste etwas anderes heisst."

**Die Commit-Nachricht von `c35f8b1` ist richtig** und nennt die Aufzählung
gar nicht beim Namen; die Abweichung steht allein in der Abschlussnotiz des
Datensatzes.

**Es ist kein Fall der Marker-Ausnahme.** `CLAUDE.md` hält fest, dass
Aufzeichnungen eines Standes ihren damaligen Marker behalten — das betrifft
Zustandsmarker in Dateinamen, nicht einen Typnamen, der zu keinem Zeitpunkt so
hieß. Der Name im Datensatz war schon beim Schreiben falsch.

**Was zu tun ist:** die sechs Stellen der Abschlussnotiz auf `Inhaltsart`
ziehen.

**Gewicht: niedrig.** Buchführung. Kostet den nächsten Leser einen
fehlgeschlagenen `grep` und führt ihn auf `vorschaumodell::Inhalt`.

**Herkunft:** Circle der Runde 6, Turn 4; aufgefallen beim Abgleich der
geschlossenen Datensätze gegen den Baum.

---

**Resolved 260812** — berichtigt, aber nicht so, wie der Datensatz es unter
„Was zu tun ist" vorschlug.

Der Vorschlag lautete, „die sechs Stellen der Abschlussnotiz auf `Inhaltsart`
zu ziehen", also den Text zu ändern. Das ist nicht getan. Ein Defektdatensatz
ist die Aufzeichnung eines Standes, und `CLAUDE.md` hält fest, dass solche
Aufzeichnungen stehen bleiben — die Ortsregel nennt `issues/` ausdrücklich.
Sie gilt der ganzen Datei nach ihrem Ort und nicht dem einzelnen Absatz; wer
in einer Abschlussnotiz sechs Namen austauscht, macht aus einer Aufzeichnung
eine Fassung und nimmt dem nächsten Leser die Möglichkeit zu sehen, was
damals dastand.

Statt dessen trägt
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`
jetzt einen datierten Nachtrag am Ende: er nennt den richtigen Namen, den
Grund für das `-art`, den Typ, mit dem `Inhalt` zu verwechseln wäre, und sagt,
dass überall `Inhaltsart` zu lesen ist. Der Nachtrag hält daneben fest, welche
zwei Sätze jener Notiz sich durch die Behebungen dieses Turns überholt haben —
das war nicht Gegenstand dieses Datensatzes, fiel aber beim Abgleich derselben
Notiz gegen den Baum an und wäre sonst der nächste Fehlbefund.

**Am Baum ist nichts zu ändern.** `Inhaltsart` heißt schon so, und ihr
Doc-Kommentar nennt die Verwechslungsgefahr; die Aufzählung hat mit diesem
Turn eine Methode `deckt_luecken` bekommen
(`260812-2019_c_die-aufzaehlung-inhaltsart-wird-nur-ueber-matches-gelesen-und-haelt-den-bau-nicht-an.md`),
die den Namen ein weiteres Mal trägt.

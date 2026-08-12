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

Ein Zuträg des ontorev an einen Datensatz der Runde 6 ist nirgends eingetragen

---

Die Durchsicht `reviews/260813-0532-ontorev-belegungsdatei-weitere-instanz.md` führt vier
Zuträge zu bestehenden Datensätzen und sagt ausdrücklich, sie seien „hier eingetragen, nicht
ein zweites Mal gemeldet". Einer davon ist an keiner Stelle außerhalb des Durchsichtsberichts
angekommen.

**Der Zuträg.** Unter `### opt+cmd: der neue Block ist der dritte Gegenbeleg zu einer
Reihenordnung, die die Datei behauptet` stellt der `ontorev` fest, dass `weitere_instanz` nach
`opt+cmd+delete` und `opt+cmd+e` der **dritte** Gegenbeleg zu dem Satz in
`resources/default-keymap.toml:246-249` ist, die `opt+cmd`-Reihe trage, „was einen Ordner
herstellt oder liefert". Der neue Kommentar begründet stattdessen über den Grundbuchstaben und
lässt die Datei mit **drei** Lesarten ihrer eigenen Ordnung zurück, wo der Datensatz der Runde
6 erst zwei gezählt hatte. Die Empfehlung lautet, die Berichtigung zu `260812-1527` auf den
neuen Block auszudehnen.

**Am 260813 nachgesehen:**
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1527_*_die-zwei-neuen-kommentare-verengen-die-reihenordnung-und-widersprechen-der-datei.md`
endet unverändert mit dem Abschnitt „Nicht betroffen". Weder `weitere_instanz` noch der dritte
Gegenbeleg noch die Durchsicht vom 260813 kommen darin vor.

---

**Schwere:** gering. Kein Code betroffen. Der Zuträg lebt allein im Durchsichtsbericht, und
Durchsichtsberichte sind Aufzeichnungen eines Standes: wer `260812-1527` später behebt, liest
den Datensatz und nicht die Durchsicht einer fremden Runde. Der dritte Gegenbeleg fiele dann
unter den Tisch, und die Berichtigung wäre beim Schreiben schon unvollständig.

**Warum er nicht behoben wurde.** Der Turn 2 der Runde 7 hatte den Rust-Anteil und die
Kommentare der Belegungsdatei zum Auftrag. `260812-1527` liegt im Speicher der Runde 6, und
diese Runde hat ihn nicht angefasst — richtig so. Der Zuträg ist deshalb nicht falsch
behandelt, sondern nur nicht abgelegt.

**Herkunft:** Der Befund entsteht aus der Durchsicht der Runde 7 und liegt deshalb in deren
Speicher, obwohl sein Ziel der Datensatz der Runde 6 ist.

**Gefunden:** reconciler, Abgleich der Runde 7

**Betroffen:**
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1527_*_die-zwei-neuen-kommentare-verengen-die-reihenordnung-und-widersprechen-der-datei.md`,
`reviews/260813-0532-ontorev-belegungsdatei-weitere-instanz.md` (Abschnitt `opt+cmd`)

**Domain:** data — die Behebung fasst `resources/default-keymap.toml` an und gehört dem `ontocoder`

## Behebung

Den Absatz aus der Durchsicht an `260812-1527` anhängen, mit Datum und Herkunft, damit die
Berichtigung jener Datei den dritten Gegenbeleg mitnimmt. Das ist eine Zeile Arbeit und keine
Entscheidung; die Entscheidung, welche Lesart der `opt+cmd`-Reihe gilt, liegt weiterhin in
`260812-1527` selbst.

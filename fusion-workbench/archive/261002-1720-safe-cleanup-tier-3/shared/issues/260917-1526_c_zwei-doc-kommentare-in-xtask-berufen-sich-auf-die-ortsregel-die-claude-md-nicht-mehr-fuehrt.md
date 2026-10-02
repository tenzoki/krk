Zwei Doc-Kommentare in `xtask/src/release.rs` berufen sich auf die Ortsregel, die `CLAUDE.md` nicht mehr führt

---

`CLAUDE.md` hat am 260917 die Ortsregel für den Marker in Zitaten durch eine Gestaltregel ersetzt (Eintrag L11 des Kuratorlaufs `260917-1450-curator-run.md`, Stufe 2, Beleg `260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`). Zwei Doc-Kommentare im Prüfmodul von `xtask/src/release.rs` nennen jene Regel weiter beim Namen und begründen mit ihr, warum `fusion-workbench/` aus den beiden Quellbaum-Läufen herausfällt.

---

**Filed by:** curator, Kai Stalmann <kai@stalmann.org>

**Betroffene Stellen**

- `xtask/src/release.rs:1492`, im Kopf von `der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr`: „die behalten nach der Ortsregel aus `CLAUDE.md` ihren damaligen Wortlaut".
- `xtask/src/release.rs:1526`, im Kopf von `NICHT_BETRETEN`: „die Werkbank haelt nach der Ortsregel aus `CLAUDE.md` ihren damaligen Wortlaut".

Erhoben mit ``grep -rn 'Ortsregel' --exclude-dir=fusion-workbench --exclude-dir=target --exclude-dir=.git .`` → genau diese zwei Zeilen, keine dritte.

**Was falsch ist und was nicht**

Falsch ist der Name, nicht der Schluss. `CLAUDE.md` sagt weiter, dass Aufzeichnungen eines Standes ihren damaligen Marker behalten; was gefallen ist, ist die Ausnahme nach dem Speicherort und damit der Begriff „Ortsregel". Die beiden Läufe dürfen die Werkbank aus demselben Grund draußen lassen wie bisher, nur trägt dieser Grund jetzt eine andere Begründung: die Sternform bindet jedes Zitat, und stehen bleibt, was keine Adresse ist, sondern eine Aussage über eine.

**Wer es ändert**

`coder`. Die Datei liegt unter `xtask/` und ist Anwendungscode; der Kuratorlauf ändert dort nichts.

**Abnahme**

Beide Doc-Kommentare nennen den Grund, ohne einen Regelnamen zu führen, den `CLAUDE.md` nicht mehr trägt; ``grep -rn 'Ortsregel' --exclude-dir=fusion-workbench --exclude-dir=target --exclude-dir=.git .`` gibt nichts mehr aus, und `cargo test -p xtask` bleibt grün.

---

Resolved: Beide Doc-Kommentare begründen den Ausschluss der Werkbank jetzt aus der Sache statt aus einem Regelnamen: eine Aufzeichnung wird nicht nachgezogen, wenn der Baum weiterrückt, weil ihr Wortlaut sagt, was am Tag ihrer Niederschrift galt, und ein Angleichen an den heutigen Stand genau diese Auskunft löschte. Der Schluss und der Umfang der beiden Läufe sind unverändert; `NICHT_BETRETEN` führt weiter dieselben vier Ordner. `grep -rn 'Ortsregel' --exclude-dir=fusion-workbench --exclude-dir=target --exclude-dir=.git .` gibt nichts mehr aus, `cargo test -p xtask` mit Exit 0 (171 Proben).

Die Probe zu C5.10 liest den ganzen Text und misst die Hälfte des Kriteriums nicht

---

C5.10 verlangt Kommentarzeilen, „die alle vier Bausteine benennen **und je einen an einem
Beispiel zeigen**". Die Probe `die_auslieferungsfassung_nennt_jeden_bausteinnamen`
(`crates/krk-core/src/ablage/leseprofile.rs:144`) sucht die vier Namen im **ganzen**
`AUSLIEFERUNGSTEXT` und nicht in seinen Kommentarzeilen. Jeder der vier Namen steht ohnehin in
den Profilblöcken der Datei, also bestünde die Probe auch an einer Fassung ohne eine einzige
erklärende Kommentarzeile.

---

**Die Sache stimmt, gemessen am 260824-1852.** `resources/default-readers.toml` erklärt jeden
der vier Bausteine im Kommentarkopf und zeigt je ein Beispiel: `zaehlung` (`:81-87`), `juengste`
(`:89-96`), `feld` (`:98-110`), `vorhandensein` (`:112-120`). Das Kriterium ist eingelöst.

**Was die Probe hält und was nicht.** Sie hält, dass die vier Bausteinnamen irgendwo in der
Datei vorkommen, und sie zählt daneben die Kommentarzeilen gegen eine Untergrenze von 100
(`:156`). Die zwei Aussagen zusammen sind ein Näherungswert für „die Datei ist zur Hälfte
Kommentar", nicht ein Nachweis für „jeder Baustein ist im Kommentar erklärt". Ein Räumen, das
den Bausteinabschnitt entfernt und die Vorrangregel stehen lässt, ließe beide Zusagen grün.

**Der Name der Probe ist ehrlich**, ihr Doc-Kommentar ist es nicht: er schreibt „(C5.10)" an
eine Messung, die C5.10 nur zur Hälfte trifft. Das ist derselbe Befundtyp, den diese Runde
viermal geräumt hat.

**Abstellen:** die Suche auf die Kommentarzeilen einschränken und je Baustein eine
Beispielzeile fordern, also die Zeilen mit führendem `#` einsammeln und in ihnen suchen. Arbeit
für den `coder`.

Gefunden beim Abgleich zum Abschluss der Runde 16, 260824-1852.

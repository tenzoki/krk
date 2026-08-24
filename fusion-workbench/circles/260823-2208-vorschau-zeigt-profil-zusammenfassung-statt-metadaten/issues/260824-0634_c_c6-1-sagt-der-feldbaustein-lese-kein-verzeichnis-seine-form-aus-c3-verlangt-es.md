C6.1 sagt, der Feldbaustein lese kein Verzeichnis; seine Form aus C3 verlangt es

---

Der Spec `planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
traegt zwei Aussagen ueber den Baustein B3, die zusammen nicht gelten koennen.

C3 beschreibt seine Form: „**B3 — Ein Feld aus einer Datei.** Nennt eine Datei ueber einen
Ausdruck auf dem Dateinamen und einen zweiten Ausdruck mit genau einer Fanggruppe."

C6.1 beschreibt seine Kosten: „Ein Baustein loest hoechstens einen Verzeichnisleselauf aus.
Der Feldbaustein loest keinen aus."

Ein Ausdruck auf dem Dateinamen laesst sich nur gegen Namen halten, die jemand zuvor
aufgezaehlt hat. Wer eine Datei ueber ein Muster benennt, liest damit das Verzeichnis, in dem
sie liegt. Der zweite Satz von C6.1 ist in dieser Allgemeinheit falsch.

---

**Warum es zaehlt.** C5.6 verlangt die Directive aus dem Circle-Datensatz, und dessen Name
traegt den Zustandsmarker: `_a_circle.md`, `_t_circle.md`, `_c_circle.md` und drei weitere
Schreibweisen. Ein fester Dateiname erreicht ihn nicht, ein Muster schon. Der Fall, um
dessentwillen der Nutzer am 260824-0555 den regulaeren Ausdruck gewaehlt hat, ist also genau
der, der ein Verzeichnis lesen muss.

**Der Plan loest den Widerspruch, ohne ein Kriterium fallen zu lassen, und hebt ihn nicht
auf.** Der Umsetzungsplan vom 260824 liest den erkannten Ordner hoechstens einmal je
Zusammenfassung und laesst jeden Baustein, der ihn nennt, diese eine Lesung benutzen; der
Feldbaustein loest damit fuer jedes der fuenf mitgelieferten Profile keinen eigenen
Verzeichnisleselauf aus. Fuer einen Feldbaustein, der eine Datei in einem **Unterordner**
benennt, faellt genau ein Leselauf an, und dafuer ist der zweite Satz von C6.1 weiterhin
falsch. Die genaue Fassung waere: „Der Feldbaustein loest keinen eigenen Verzeichnisleselauf
aus, solange seine Datei in einem Ordner liegt, der ohnehin gelesen wird; der erkannte Ordner
ist immer einer davon."

**Was nicht beruehrt ist.** Die Zahlen aus C6.7 halten mit dem Plan, gerechnet am groessten
mitgelieferten Profil: fuenf Verzeichnisleselaeufe und elf Dateioeffnungen gegen die dort
zugesagten hoechstens sieben und hoechstens elf. Der Widerspruch kostet die Runde also keine
Arbeit, sondern eine Berichtigung des Wortlauts.

**Filed by:** planner

---
Resolved: C6.1 des Specs traegt seit dem 260824-1224 die genaue Fassung, die dieser Datensatz nennt: der Feldbaustein loest keinen **eigenen** Verzeichnisleselauf aus, solange seine Datei in einem Ordner liegt, der ohnehin gelesen wird, und der erkannte Ordner ist immer einer davon. Dazu steht, was allgemein gilt und was nicht: fuer die fuenf mitgelieferten Profile gilt der Satz durchweg, fuer einen Feldbaustein in einem Unterordner faellt genau ein Leselauf an. Die Berichtigung unter der Kriterienliste von C6 fuehrt den Grund und die Zahlen aus C6.7, die unberuehrt sind. Der Punkt unter `## Open Questions` des Plans ist abgehakt. Die Berichtigung aendert ein freigegebenes Abnahmekriterium inhaltlich und ist dem Nutzer vorzulegen.

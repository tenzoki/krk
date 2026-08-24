# Der Titel aus der Überschriftenzeile erreicht keinen einzigen Defektdatensatz: bleibt es dabei?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_was-heisst-die-juengsten-zehn-und-was-ist-ihr-titel.md`, `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`

---

## Question

Der Nutzer hat am 260824-0555 gewählt: sortiert wird nach Änderungsdatum, und der Titel ist
die erste Überschriftenzeile, ersatzweise der Dateiname. Nachgemessen am Bestand der Werkbank
trifft der Ersatzfall nicht wenige Dateien, sondern eine ganze Sorte: **kein einziger
Defektdatensatz trägt eine Markdown-Überschrift.** Das Dateiformat für Defekte schreibt eine
nackte Titelzeile ohne `#` vor, so festgelegt in `rules/fusion-workbench-conventions.md` unter
`## Issue and Decision Filing`; Entscheidungs-, Verlaufs-, Analyse- und Planungsdatensätze
beginnen dagegen sämtlich mit `# `.

Gezählt am 260824-0600: 82 Dateien in `shared/issues/`, 157 im größten Speicher eines Circles,
und in jeder von ihnen steht der Titel in der ersten Zeile, nur ohne Doppelkreuz. Ein Profil,
das über einem Defektspeicher die jüngsten zehn Titel zeigt, zeigte damit zehn Dateinamen der
Form `260823-1445_o_die-neue-regel-verweist-jeden-rufer-an-sich-selbst-einer-der-fuenf-sagt-dort-nichts.md`,
also genau die Anzeige, gegen die der Nutzer sich mit Möglichkeit 2 entschieden hat. Die
Antwort gehört vor die Abnahmekriterien, weil vier der sechs skizzierten Zusammenfassungen den
Baustein „jüngste zehn mit Titel" tragen.

## Options

1. **Es bleibt bei der Überschriftenzeile** — Erste Zeile mit `#` ist der Titel, sonst der
   Dateiname.
   - Pros: Die Antwort vom 260824-0555 steht unverändert, und die Regel ist die knappste:
     Markdown-Überschrift oder Name. Der Zustandsmarker eines Defekts bleibt in der Anzeige,
     weil sie dort den Dateinamen zeigt.
   - Cons: Für den Speicher, der die meisten Datensätze führt, liefert der Baustein
     Dateinamen. Der Nutzer sieht Zeitstempel und Bindestriche statt Sätzen, und zwar überall
     dort, wo die Werkbank am dichtesten belegt ist.
2. **Die erste nicht leere Zeile ist der Titel, ein führendes `#` wird abgeräumt** —
   Dieselbe Regel für beide Sorten, ohne Fallunterscheidung nach Format.
   - Pros: Liefert für **jede** Sorte der Werkbank einen echten Titel, Defektdatensätze
     eingeschlossen, und löst damit ein, wofür Möglichkeit 2 am 260824-0555 gewählt wurde.
     Eine Regel statt zweier, und sie kennt kein Markdown.
   - Cons: In einem Ordner mit Dateien, deren erste Zeile kein Titel ist, zeigt die Anzeige
     die erste Zeile und nicht den Namen; ein Quelltext zeigte seine erste Zeile, eine
     TOML-Datei ihre erste Kommentarzeile. Der Rückweg auf den Dateinamen entfällt außer für
     die vollständig leere Datei.
3. **Erste Überschriftenzeile, ersatzweise die erste nicht leere Zeile, zuletzt der
   Dateiname** — Drei Stufen statt zweier.
   - Pros: Trägt beide Sorten der Werkbank und behält den Dateinamen als letzten Rückweg.
   - Cons: Drei Stufen für eine Frage, die zwei beantworten. Der mittlere Fall ist gegenüber
     Möglichkeit 2 nur dort unterscheidbar, wo eine Datei überhaupt eine Überschrift trägt,
     und dann sind beide gleich; die dritte Stufe greift allein bei der leeren Datei.

## Constraints

Gelesen wird über den vorhandenen Weg `krk_core::text::datei::bis_zur_grenze_lesen`. Die zehn
Dateiöffnungen je Zusammenfassung sind mit der Antwort vom 260824-0555 bereits in Kauf
genommen; keine dieser drei Möglichkeiten ändert ihre Zahl, sie unterscheiden sich allein in
dem, was aus den gelesenen Bytes wird. `Eintrag` trägt den Änderungszeitpunkt schon aus dem
Verzeichnislesen (`krk-core/src/verzeichnis/eintrag.rs:47`), das Sortieren kostet also keinen
zusätzlichen Systemaufruf.

## Recommendation

Möglichkeit 2. Der Beispielfall der Runde ist die Werkbank, und in ihr ist die erste nicht
leere Zeile jeder Datensatzsorte der Titel; das Doppelkreuz ist die Auszeichnung und nicht die
Auskunft. Der Preis ist benannt: über einem Ordner ohne Titelzeilen zeigt die Anzeige erste
Zeilen statt Namen. Wer das nicht will, wählt Möglichkeit 1 und nimmt die Dateinamen über den
zwei größten Speichern der Werkbank in Kauf.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:88 — Erste nicht leere Zeile ist der Titel (Möglichkeit 1); berichtigt die Titelhälfte der Antwort vom 260824-0555.

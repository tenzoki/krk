# Wie heißt das Archiv, wenn Zip auf eine einzelne Datei mit Endung wirkt?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_a_circle.md` (Directive, Zip-Teil)

---

## Question

Der Nutzer hat entschieden, dass das Archiv bei mehreren markierten Einträgen den Namen des angezeigten Ordners trägt. Für den Fall eines einzelnen Eintrags sagt der Entwurf „benamt nach Ordner bzw. Datei“, und bei einer Datei mit Endung ist damit noch nicht entschieden, ob die Endung stehen bleibt. Aus `bericht.txt` wird entweder `bericht.zip` oder `bericht.txt.zip`. Die Frage muss vor dem Bau beantwortet sein, weil die Namensbildung eine reine Funktion ist, die genau einmal dasteht, und weil beide Formen unterschiedliche Kollisionen erzeugen: `bericht.zip` kollidiert mit einem gleichnamigen Archiv, `bericht.txt.zip` nicht, dafür sieht der Nutzer eine doppelte Endung.

## Options

1. **Endung anhängen: `bericht.txt.zip`** — der volle Dateiname bleibt erhalten, die Endung `.zip` tritt dahinter.
   - Pro: Der Ursprungsname ist im Archivnamen vollständig ablesbar, auch wenn zwei Dateien gleichen Stamms und verschiedener Endung im selben Ordner liegen (`bericht.txt` und `bericht.md` ergeben zwei verschiedene Archive statt einer Kollision).
   - Contra: Doppelte Endung, die manche Nutzer als unsauber empfinden.
2. **Endung ersetzen: `bericht.zip`** — der Stamm bleibt, die alte Endung fällt.
   - Pro: Kürzerer, ruhigerer Name.
   - Contra: Zwei Dateien gleichen Stamms erzeugen denselben Archivnamen, also planmäßig eine Konfliktrückfrage bei einer Bewegung, die für den Nutzer nach zwei verschiedenen Vorgängen aussieht.
3. **Ordner und Datei verschieden behandeln** — ein Ordner `Projekte` ergibt `Projekte.zip`, eine Datei `bericht.txt` ergibt `bericht.txt.zip`.
   - Pro: Ein Ordnername hat keine Endung, die Frage stellt sich dort nicht; die Regel ist damit genau die aus Möglichkeit 1, nur ohne Sonderfall.
   - Contra: Nichts über Möglichkeit 1 hinaus; sie ist dieselbe Regel, anders beschrieben.

## Constraints

Die Namensbildung steht als eine reine Funktion an einer Stelle und ist ohne AppKit prüfbar, wie `kommandos::rueckschritt` und `verzeichnis::filter` es vormachen. Was immer gewählt wird, gilt auch für den Fall aus Runde 2, in dem der Name vom angezeigten Ordner kommt; ein Ordnername trägt in der Regel keine Endung, sodass beide Möglichkeiten dort dasselbe Ergebnis liefern.

## Recommendation

Möglichkeit 1. Sie hält den Ursprungsnamen vollständig und vermeidet die Kollision zweier Dateien gleichen Stamms, die sonst eine Rückfrage auslöst, ohne dass der Nutzer einen Grund dafür sieht. Es ist zugleich das Verhalten, das die Nutzer vom Komprimieren im Finder kennen.

---
Answered: shared/history/260824-2120-orchestrator-session.md:31 — Moeglichkeit 1, Endung anhaengen: aus bericht.txt wird bericht.txt.zip.
Implemented: 423d5f2, 3d8332a — kontextmenue::archivname haengt die Endung an; ordnername_zum_archiv kehrt es um, und die Probe archivname_und_ordnername_kehren_einander_um haelt das Paar.
Deferred:
Superseded by:
Retired:

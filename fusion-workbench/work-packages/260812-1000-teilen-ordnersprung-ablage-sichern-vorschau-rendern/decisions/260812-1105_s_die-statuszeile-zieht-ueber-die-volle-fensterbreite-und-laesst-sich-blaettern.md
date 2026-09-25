# Zieht die Statuszeile über die volle Fensterbreite, und wie erfährt der Leser, welches Dateifenster gemeint ist?

---
**Domain:** code
**Status:** superseded
**Filed by:** orchestrator (Klärungsrunde, auf Vorgabe des Nutzers)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md` (überholt durch diesen Datensatz), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` (C1), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-1832_*_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`, `crates/krk-ui/src/appkit/statuszeile.rs`, `crates/krk-ui/src/appkit/aufteilung.rs:486`

---

## Question

Die Statuszeile sitzt heute **je Dateifenster** am Fuß des Bereichs, 18 Punkte hoch und so breit
wie ihr Fenster (`aufteilung.rs:486-489`). Es gibt also zwei davon. Der Nutzer hat am 260812-1105
festgestellt, dass das für die meisten Meldungen zu schmal ist, und verlangt: **die Statusmeldung
nutzt die volle Fensterbreite, und es muss möglich sein, nach rechts zu blättern.**

Damit ersetzt eine Zeile zwei, die heute je einem Fenster gehören. Zu klären ist, woher der Leser
dann weiß, welches Fenster gemeint ist — besonders beim Fortschritt einer Dateioperation, den der
Nutzer am 260804-1832 ausdrücklich dem jeweiligen Fenster zugeordnet hat.

## Options

1. **Die Meldung nennt ihr Fenster im Text.** Eine Zeile über die volle Breite; wo die Herkunft
   nicht ohnehin klar ist, steht sie im Satz („Rechts: 3 von 12 kopiert").
   - Pros: kein neues Bedienelement, die fünf Ränge aus C1 bleiben unangetastet.
   - Cons: bei zwei gleichzeitigen Meldungen aus beiden Fenstern gewinnt eine, und welche, ist zu
     klären.
2. **Die Zeile beschreibt immer das aktive Fenster.**
   - Pros: klare Regel ohne Zusatztext.
   - Cons: ein Kopiervorgang links läuft unsichtbar, solange rechts gearbeitet wird — und genau
     dagegen hat der Nutzer den Fortschritt am 260804 aus dem Blatt in die Zeile geholt.
3. **Zweigeteilte Zeile mit je einer Hälfte pro Fenster.**
   - Cons: jede Hälfte ist wieder halb so breit. Das ist genau die Enge, die behoben werden soll,
     nur eine Ebene tiefer.

## Constraints

- C1 der Runde 1 sagt zu, dass die Statuszeile der **eine** Weg für laufende Meldungen ist und
  dass KRK keine Meldung über die Standardfehlerausgabe gibt. Eine Zeile statt zweier bricht das
  nicht — zwei Meldeflächen nebeneinander täten es.
- Die fünf Ränge (Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung, Markierungsstand)
  sind heute **je Dateifenster** gerechnet; drei ihrer Quellen hängen an einem Fenster, der
  Markierungsstand kommt aus dem Ordnermodell des sichtbaren Tabs.
- L9 aus C8 misst den Anteil der Dateiliste im ersten Bild. Eine Zeile weniger je Dateifenster
  gibt der Liste Höhe zurück, eine Zeile über die volle Breite nimmt sie einmal statt zweimal.
- Die Bereichsleiste der Runde 5 sitzt bereits am Fensterfuß. Wo die neue Statuszeile relativ zu
  ihr liegt, gehört in den Plan.

## Antwort 260812-1105

**Möglichkeit 1, Nutzerentscheid.** Eine Statuszeile über die volle Fensterbreite; wo die Herkunft
nicht ohnehin klar ist, nennt die Meldung ihr Fenster im Text. Die Zeile lässt sich nach rechts
blättern, damit eine lange Meldung nicht abgeschnitten wird.

**Dieser Datensatz überholt eine Entscheidung der Runde 5.** Dort ist am 260812-0306 Möglichkeit 1
gewählt worden — die neue Leiste trägt ausschließlich Schalter, die beiden Statuszeilen bleiben,
wo sie sind — und Möglichkeit 2 jenes Datensatzes, genau der Umbau, um den es hier geht, ist
ausdrücklich als eigene Runde vertagt worden. Der Nutzer hat am 260812-1105 entschieden, ihn in
diese Runde zu nehmen. Der damalige Datensatz geht deshalb von umgesetzt auf überholt.

**Was das kostet, ausdrücklich benannt und vom Nutzer angenommen:** die Runde fasst C1 der Runde 1
an, also eine abgenommene Fähigkeit. Die Rangfolge in `statuszeile::zeile` ist neu zu fassen, und
die Zuordnung des Vorgangsfortschritts zu seinem Dateifenster — eine Entscheidung des Nutzers vom
260804-1832 — wird von einer räumlichen zu einer sprachlichen: sie steht danach im Satz statt in
der Lage der Zeile.

**Offen und in den Plan zu tragen:** was geschieht, wenn beide Dateifenster zugleich eine Meldung
haben. Der Plan entscheidet es an der bestehenden Rangfolge, statt eine zweite daneben zu setzen.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Nutzerentscheid vom 260812-1105, vorgelegt mit drei Möglichkeiten und ihren Folgen; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by: `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1809_*_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md` — nur die Blätter-Hälfte ist ersetzt. Eine zu lange Meldung wird künftig über einen Kurzhinweis beim Verweilen lesbar statt über eine Bildlaufansicht, weil das Blättern zwei Kosten erzeugt hat, die hier niemand vorhergesehen hatte: den Gestenklau am Fensterfuß und die `NSScroller`, die C5.11 breiter machen. Alles Übrige dieses Datensatzes gilt fort und ist mit `baf8660` gebaut — eine Zeile statt zweier, über die volle Fensterbreite, mit der Zuordnung zum Dateifenster im Satz und der zweistelligen Ordnung aus Rang und aktiver Seite. Der Nachfolger nennt beides ausdrücklich, damit die fortgeltende Hälfte nicht mit der ersetzten verlorengeht.

---
Der README-Kopf sagte, der Editor folge noch in späteren Runden

---

Der Kopf der `README.md` lautete bis zum Umbau vom 260821:

> KRK ist ein Dateimanager für macOS in der Tradition der
> Norton-Commander-Bedienung: zwei Dateifenster nebeneinander, alles über die
> Tastatur erreichbar. **Editor und Git-Anbindung folgen in späteren Runden.**

Der zweite Satz ist zur Hälfte falsch. Der eingebaute Editor ist der Gegenstand
der Runde 2 (`circles/260807-2116-eingebauter-editor-mit-textmarken`), steht seit
dem als fünfter Bereich der Fensterzeile im Baum und ist seither in zwölf
weiteren Runden fortgeschrieben worden. Allein die Git-Anbindung ist offen; am
260821 trägt `Kommando` keine einzige Git-Variante.

Der Satz stammt aus der Zeit der Runde 1 und ist beim Bau der Runde 2 nicht
nachgezogen worden. Er ist an der einen Stelle stehengeblieben, die Fremde zu
lesen bekommen, und widersprach seit der Auslieferung von v0.5.6 dem
Releasetext, den dasselbe Projekt auf jede Releaseseite schreibt: `RELEASETEXT`
in `xtask/src/veroeffentlichung.rs` nennt KRK „ein Dateimanager mit Editor für
macOS".

**Zweiter Befund, kleiner:** derselbe Kopf trug zwei Aussagen nicht, die der
`RELEASETEXT` trägt und die der Nutzer des Zips braucht — die Untergrenze
macOS 15 (sie stand allein in der Tabelle `## Voraussetzungen`, also im
Entwicklerteil) und dass das Bündel beglaubigt ist und ohne Rückfrage startet.
Die beiden Flächen deckten sich damit in acht von zehn Aussagen.

Gefunden vom `coder` beim Umbau der `README.md` (Nutzerauftrag vom 260821, erst
der Nutzer und dann der Entwickler), der den neuen Kopf gegen den `RELEASETEXT`
gegengelesen hat.

Herkunft: Circle-Speicher. Der Befund entsteht aus der Directive dieser Runde —
sie schreibt die Datei um, in der er steht.

---
Resolved: Der Kopf der `README.md` ist im selben Umbau neu geschrieben. Er nennt
KRK jetzt „ein Dateimanager mit eingebautem Editor für macOS" und sagt von der
Git-Anbindung, sie sei „vorgesehen und noch nicht gebaut". Die zwei fehlenden
Aussagen stehen ebenfalls im Kopf: macOS 15 als Voraussetzung und das
beglaubigte Bündel, das ohne Rückfrage startet. Damit deckt sich der Kopf in
allen zehn Aussagen mit `RELEASETEXT`.

# Welche Form bekommt das zweite Kennzeichen der Markierung, und in welchen Rang der Statuszeile gehört ihre Zahl?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1309_c_die-markierung-ist-allein-an-der-farbe-erkennbar.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C1, C2), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`

---

## Question

Die Markierung aus C2 ist heute allein an der Farbe erkennbar: S13 färbt die Zellen eines markierten Eintrags orange, und Schrift, Hintergrund, Zeilenhöhe und Text bleiben gleich. Für einen Nutzer mit einer Farbfehlsichtigkeit zeigt eine markierte Liste damit dasselbe Bild wie eine unmarkierte, während die Dateioperationen aus C4 auf ebendiese Markierung wirken. Der Nutzer hat am 260805-0000 zwei Abhilfen zugleich zugesagt, ein zweites Kennzeichen neben der Farbe und Zahl und Gesamtgröße der markierten Einträge in der Statuszeile, und zwei Fragen dem Planner überlassen: welche Form das zweite Kennzeichen bekommt, ohne die vier Spalten aus C1 zu sprengen, und ob die Markierungszahl in den vierten Rang der Statuszeile passt oder einen fünften braucht.

## Options

**Zur Form des zweiten Kennzeichens:**

1. **Fette Schrift** in allen vier Spalten, neben der Farbe.
   - Pros: eine Form und keine Farbe, wirkt also bei jeder Farbfehlsichtigkeit; braucht keine Fläche und lässt die vier Spalten unberührt; geht denselben Weg wie die Farbe, die `zellenansicht` in `crates/krk-ui/src/appkit/tabelle.rs` ohnehin in jedem Durchgang setzt.
   - Cons: eine schwache Auszeichnung, wenn sie allein stünde; sie steht aber neben der Farbe und neben der Zahl in der Statuszeile.
2. **Eine fünfte Spalte** mit einem Markierungszeichen.
   - Pros: unmissverständlich, die Form der Dateimanager mit Textoberfläche.
   - Cons: sprengt die vier Spalten aus C1, die der Nutzer ausdrücklich erhalten wissen will.
3. **Ein Zeichen vor dem Namen** in der Namensspalte.
   - Pros: dieselbe Tradition, ohne fünfte Spalte.
   - Cons: der angezeigte Name wäre nicht mehr der wirkliche; in einer Proportionalschrift verschöben sich die Namen markierter und unmarkierter Zeilen gegeneinander, und eine feste Rinne davor verlangte eine Schrift mit fester Breite oder ein eigenes Feld.

**Zum Platz der Markierungszahl:**

1. **Rang 4**, zusammen mit der Tabmeldung.
   - Pros: kein fünfter Rang; beide beschreiben einen Zustand des sichtbaren Tabs.
   - Cons: die beiden können zugleich zutreffen, und das Feld bekäme zwei Löschregeln. Die Tabmeldung trägt einen Ordner, der sich nicht lesen ließ, und muss stehen bleiben, während der Nutzer markiert und die Markierung wieder aufhebt.
2. **Ein fünfter Rang**, unter der Tabmeldung.
   - Pros: eine Lebensdauer je Feld, wie S16b es für die vier vorhandenen Ränge durchgehalten hat; kein Fehler verdrängt eine Markierungszahl und umgekehrt.
   - Cons: ein Rang mehr in einer Tabelle, die schon vier hat.

## Constraints

- Die vier Spalten aus C1 bleiben vier.
- Die Statuszeile trägt einen Text; steht mehr als eine Aussage an, gewinnt die höchstrangige, und keine geht dabei verloren (C1).
- Die Größe eines Ordners lässt sich nur durch Durchlaufen ermitteln, und diesen Vorabdurchlauf schließt `### Frage 6` des Plans aus.
- Die Maxime "supersimpel" schließt eine Lösung aus, die eine Fähigkeit mit einer eigenen Sonderregel und einem eigenen Rückfallweg erkauft.

## Recommendation

Fette Schrift, und ein fünfter Rang.

---
Answered: planner am 260805-0000, im Auftrag des Nutzers, der beide Teilfragen ausdrücklich überlassen hat — **fette Schrift** und **ein fünfter Rang**.

**Zur Form.** Ein markierter Eintrag steht in allen vier Spalten fett und bleibt orange. Der Eingriff ist eine Zeile neben einer vorhandenen: `zellenansicht` setzt heute in jedem Durchgang die Textfarbe, weil die Zellenansichten wiederverwendet werden und eine ungesetzte Eigenschaft die des vorigen Eintrags bliebe; die Schriftstärke geht denselben Weg und aus demselben Grund. Es kommt eine Eigenschaft dazu und kein Mechanismus. Möglichkeit 2 scheidet an der Auflage des Nutzers aus. Möglichkeit 3 scheidet daran aus, dass sie den angezeigten Namen vom wirklichen unterscheidbar machte; der Nutzer sucht mit der Sprungmarke aus C2 nach dem wirklichen.

**Zum Rang, und warum der fünfte seinen Preis wert ist.** Der Nutzer hat richtig gesehen, dass die Markierungszahl ein Zustand ist wie die Tabmeldung und kein Ereignis. Sie ist trotzdem nicht dieselbe Sorte Zustand, und der Unterschied ist die Lebensdauer: die Tabmeldung fällt mit dem Tab, die Markierungszahl steht und fällt mit jedem der vier Markierungsbefehle. Beide in ein Feld zu legen gäbe diesem Feld zwei Löschregeln, und genau diesen Sonderfall hat S16b bei der Trennung von Befehlsantwort und Fenstermeldung schon einmal ausgeschlossen. Der neue Rang steht **unter** der Tabmeldung, weil ein nicht lesbarer Ordner ein Fehler ist und eine Markierungszahl keiner; er ist der Ruhezustand der Zeile, und ein Ruhezustand ist der unterste Rang.

Der Preis ist kleiner als ein Feld, denn er ist keines. Die vier vorhandenen Quellen halten je einen Text, den jemand setzt und eine Regel löscht. Der Markierungsstand wird stattdessen bei jedem Schreiben der Zeile aus dem Ordnermodell des sichtbaren Tabs errechnet, das ohnehin im Speicher steht. Ein Feld hätte vier Schreiber — die vier Markierungsbefehle, die Auffrischung, den Tabwechsel und den Sortierwechsel — und damit vier Gelegenheiten, veraltet zu sein. Die Funktion `zeile` bleibt eine reine Funktion und bekommt den fertigen Text als fünften Parameter.

**Was der Rang zeigt.** Zahl der markierten Einträge, davon gesondert die Zahl der Ordner, und die Summe der Größen der markierten **Dateien**; ein Ordner zählt in der Zahl und nicht in der Größe, weil seine Größe einen Durchlauf verlangte. Dieselbe Trennung zieht die Größenspalte heute schon, indem sie bei einem Ordner `--` zeigt. Die Form der Angabe folgt der Rückfrage vor dem endgültigen Löschen aus C4, die die Ordner ebenfalls gesondert nennt. Im Kern liefert `Ordnermodell::markierungsstand` die drei Werte in einem Durchlauf, statt drei Zähler nebeneinanderzustellen; `markierungszahl` wächst dazu.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C1 (Rangfolge auf fünf, neues Kriterium zum Markierungsstand, vier neue Festlegungen) und C2 (neues Kriterium, eine Festlegung); `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` als neuer Schritt S16c.
Implemented: <offen — S16c>

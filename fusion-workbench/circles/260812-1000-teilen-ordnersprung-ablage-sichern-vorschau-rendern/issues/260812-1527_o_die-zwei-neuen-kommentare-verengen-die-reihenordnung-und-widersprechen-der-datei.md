Die zwei neuen Kommentare verengen die Reihenordnung ihres Entscheidungsdatensatzes und widersprechen damit dem Rest der Belegungsdatei

---

Beide neuen Blöcke aus Commit `95b2dfa` begründen ihre Kombination mit einer
Reihenordnung. Beide geben die Ordnung als **eingliedrig** wieder, während der zitierte
Entscheidungsdatensatz sie **zweigliedrig** formuliert. In der verengten Form ist sie
durch die Datei widerlegt.

**`opt+cmd` — `resources/default-keymap.toml:246-249`**

> Die opt+cmd-Reihe traegt in diesem Programm, was einen Ordner herstellt oder
> liefert: opt+cmd+c kopiert den Pfad des angezeigten Ordners, opt+cmd+g springt zur
> Adresse aus der Zwischenablage, opt+cmd+l, opt+cmd+d, opt+cmd+b, opt+cmd+left und
> opt+cmd+right **schalten Bereiche ein und aus**.

Der Satz widerlegt sich in seiner eigenen Aufzählung: fünf der sieben genannten
Beispiele schalten Bereiche und liefern keinen Ordner. Dazu kommen zwei `opt+cmd`-Einträge,
die die Aufzählung übergeht:

- `opt+cmd+delete` → `endgueltig_loeschen` (`:130-133`) — weder Ordner noch Bereich,
- `opt+cmd+e` → `editor_schliessen` (`:685-688`).

Die Datei beschreibt dieselbe Reihe an anderer Stelle bereits anders: der Kommentar an
`editor_schliessen` (`:689-690`) nennt sie die **Umschaltfamilie**
(„Die Umschaltfamilie steht auf opt+cmd+<Buchstabe>"). Nach dem Commit stehen zwei
unvereinbare Beschreibungen derselben Reihe in einer Datei.

**`shift+cmd` — `resources/default-keymap.toml:611-613`**

> Die shift+cmd-Reihe traegt, was auf die betroffenen Eintraege wirkt: shift+cmd+c
> kopiert ihren Pfad, shift+cmd+a hebt die Markierung auf, shift+cmd+i kehrt sie um.

Die Datei führt 17 `shift+cmd`-Kombinationen. Mindestens zehn wirken nicht auf die
betroffenen Einträge: `shift+cmd+d`, `shift+cmd+e`, `shift+cmd+l`, `shift+cmd+y`
(Fokus), `shift+cmd+g` (Pfadeingabe), `shift+cmd+h` (versteckte Einträge),
`shift+cmd+n` (Ordner anlegen), `shift+cmd+r` (Ersetzen im Editor), `shift+cmd+w`
(Fenster schließen), `shift+cmd+z` (Wiederholen).

**Was der Datensatz schreibt**

`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`,
Abschnitt `## Question`, führt beide Reihen zweigliedrig und trifft die Datei damit:

- „**`shift+cmd+X` wirkt auf Eintrag, Auswahl und Fokus.**"
- „**`opt+cmd+X` wirkt auf Ordner und Bereiche.**"

In dieser Form bleibt allein `opt+cmd+delete` außerhalb; in der verengten Form der
Kommentare fallen zwölf Einträge heraus.

---

**Warum ein Datensatz und nicht zwei:** ein Fehler, eine Ursache, eine Berichtigung —
beide Kommentare geben denselben Satz desselben Datensatzes verkürzt wieder, und eine
Berichtigung, die nur einen der beiden anfasst, hinterlässt die Datei mit zwei
verschiedenen Auskünften über ihre eigene Ordnung.

**Herkunft:** Directive dieser Runde, Commit `95b2dfa` (Schritte 2 und 4 des Plans
`.../planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`).

**Gewicht:** die Reihenordnung ist der ausdrückliche Grund, aus dem der Nutzer am
260812-1105 Möglichkeit 1 gewählt hat („die Ordnung ist nirgends aufgeschrieben und lebt
allein davon, dass sie eingehalten wird", Möglichkeit 3 desselben Datensatzes). Sie ist
mit diesem Commit zum ersten Mal aufgeschrieben — und in einer Form, die die nächste
Runde an der Datei prüfen und verwerfen wird.

**Empfehlung:** beide Kommentare auf die Formulierung des Datensatzes zurückführen
(`opt+cmd`: Ordner **und Bereiche**; `shift+cmd`: Eintrag, Auswahl **und Fokus**) und
`opt+cmd+delete` als benannte Ausnahme mitführen, statt es zu übergehen. Ob die
Umschaltfamilie am `editor_schliessen` daneben stehen bleibt oder in dieselbe
Beschreibung aufgeht, gehört in dieselbe Berichtigung.

**Nicht betroffen:** die Wahl der beiden Kombinationen und der Platz der beiden Blöcke.
Beide Befehle liegen nach der zweigliedrigen Lesart richtig.

# Was bedeutet das "revert" aus dem Entwurf: Änderungen der Datei verwerfen oder einen Commit zurücknehmen?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`

---

## Question

Der Entwurf in `idea.txt` nennt für den Editor "Git add/commit/revert" und daneben "Versionen anzeigen/auschecken (Slider zur Steuerung)". Das Wort "revert" trägt in Git zwei Bedeutungen, die nichts miteinander zu tun haben. `git checkout -- <datei>` und `git restore <datei>` werfen die ungespeicherten Änderungen einer Datei weg und holen den zuletzt committeten Stand zurück. `git revert <commit>` erzeugt dagegen einen neuen Commit, der die Änderungen eines früheren rückgängig macht, und lässt die Historie unangetastet. Beide sind plausibel gemeint, und die Wahl bestimmt, was der Knopf im Editor tut und wie gefährlich er ist. Da der Versions-Schieberegler das Ansehen und Auschecken älterer Stände bereits abdeckt, liegt die Lesart "Änderungen verwerfen" näher, aber sie ist nicht belegt.

## Options

1. **Nur Änderungen der Datei verwerfen** — der Befehl setzt die geöffnete Datei auf den zuletzt committeten Stand zurück.
   - Pro: passt zum Editor-Kontext, in dem das Wort in fast jedem Werkzeug so gemeint ist. Deckt den häufigen Fall ab, eine Änderung wegzuwerfen, ohne die Historie anzufassen.
   - Contra: löscht ungespeicherte Arbeit unwiederbringlich, sofern KRK keinen eigenen Rückweg anbietet.

2. **Nur einen Commit zurücknehmen** — der Befehl erzeugt einen Gegen-Commit zu einem im Versionsverlauf ausgewählten Stand.
   - Pro: verlustfrei, weil die Historie erhalten bleibt und der Schritt selbst wieder rückgängig zu machen ist. Ergänzt den Versions-Schieberegler, der die Auswahl des Commits ohnehin bereitstellt.
   - Contra: deckt den häufigsten Wunsch im Editor nicht ab, nämlich die eigene ungespeicherte Änderung wegzuwerfen.

3. **Beides, als zwei getrennte Befehle** — "Änderungen verwerfen" im Editor, "Commit zurücknehmen" im Versionsverlauf am Schieberegler.
   - Pro: jeder Befehl sitzt dort, wo der Nutzer ihn sucht, und beide Bedeutungen sind sauber getrennt statt in einem Wort vermischt.
   - Contra: mehr Oberfläche und mehr Erklärbedarf als der Entwurf vorsah.

## Constraints

- Der Umfang dieses Circles endet bei hinzufügen, committen, verwerfen und Versionen ansehen oder auschecken. Branches, Merges, Remotes, Push und Pull bleiben draußen, gleich wie diese Frage ausgeht.
- Der Versions-Schieberegler existiert unabhängig von der Antwort und deckt Ansehen und Auschecken bereits ab.
- Was auch immer gewählt wird, es muss auch für eine Datei greifen, die gar nicht in einem Git-Repository liegt. Für sie ist der Befehl schlicht nicht verfügbar.

## Recommendation

Option 3 trennt die beiden Bedeutungen, statt sie unter einem mehrdeutigen Wort zu verstecken, und ordnet jede der Stelle zu, an der der Nutzer sie erwartet. Der zusätzliche Aufwand ist gering, weil der Schieberegler die Commit-Auswahl ohnehin mitbringt. Empfehlung, keine geprüfte Aussage.

---
Answered:
Implemented:
Deferred:
Superseded by:

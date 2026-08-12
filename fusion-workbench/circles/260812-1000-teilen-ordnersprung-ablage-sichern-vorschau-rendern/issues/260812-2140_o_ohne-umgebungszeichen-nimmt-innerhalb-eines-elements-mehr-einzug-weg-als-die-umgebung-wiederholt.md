`ohne_umgebungszeichen` nimmt innerhalb eines Elements mehr Einzug weg, als die Umgebung wiederholt

---

`ohne_umgebungszeichen` (`crates/krk-ui/src/markdown.rs`) sagt zu, es nehme
einer Lücke, „was ihre **Umgebung** auf jeder ihrer Zeilen wiederholt". Es
nimmt aber jeden führenden Leerraum weg und nicht nur so viel, wie die
Umgebung wiederholt. Steht in einem Listenpunkt mit zwei Leerzeichen Einzug
eine Zeile mit sechs, so verliert sie alle sechs statt zwei.

---

**Gemessen** (`crates/krk-ui/src/markdown.rs`, Stand nach der Behebung von
`260812-2019_c_ohne-umgebungszeichen-laeuft-auch-auf-dokumentebene-und-nimmt-dort-einzug-weg-der-inhalt-ist.md`):

```
Quelle : "- Text\n\n  [ZIEL]:\n      http://z.example\n"
Ausgabe: "• Text\n\n[ZIEL]:\nhttp://z.example"
```

Die vier zusätzlichen Leerzeichen der Fortsetzungszeile gehören zur
Verweisdefinition und nicht zum Einzug des Punktes.

**Die Ursache.** `trim_start_matches([' ', '\t', '>'])` schneidet den ganzen
führenden Lauf und nicht eine gezählte Menge. Wie viel die Umgebung
wiederholt, weiß die Funktion nicht: sie bekommt die Lücke und sonst nichts.

**Ein Zuschnitt** (nicht gewählt): den kürzesten führenden Lauf über alle
**nichtleeren** Zeilen der Lücke bestimmen und nur diesen von jeder Zeile
abziehen. Das bliebe eine Regel ohne Fallaufzählung und käme ohne die Kenntnis
der Umgebung aus. Zu prüfen wäre, ob die Leerzeile eines Zitats — sie trägt
nur `>` und keinen Leerraum — das Mindestmaß so weit drückt, dass das `>` der
Nachbarzeilen stehen bliebe; nach der Regel „nur nichtleere Zeilen zählen"
tut sie es nicht, gemessen ist es nicht.

**Gewicht: niedrig.** Kosmetisch, sehr seltene Quelle, kein Inhaltsverlust —
die Zeichen stehen da, nur der zusätzliche Einzug nicht. Der Befund ist die
verbliebene Abweichung zwischen dem Doc-Kommentar und dem, was die Funktion
tut. Die Dokumentebene, die den größeren Teil derselben Abweichung ausmachte,
ist behoben.

**Herkunft:** Circle der Runde 6, Turn 5; abgetrennt beim Beheben von
`260812-2019_c_ohne-umgebungszeichen-laeuft-auch-auf-dokumentebene-und-nimmt-dort-einzug-weg-der-inhalt-ist.md`.

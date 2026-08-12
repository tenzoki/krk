Ein leerer Listenpunkt zeigt sein rohes `- ` samt Zeilenumbruch und verliert seinen Einzug

---

Ein Listenpunkt ohne Inhalt gibt seit `c35f8b1` seinen Quellbereich wörtlich
heraus. Für einen Punkt, der **nur eine Verweisdefinition** trägt, ist das der
gewollte Zuschnitt und durch eine Probe festgeschrieben. Für einen Punkt, der
**gar nichts** trägt, ist es eine Verschlechterung: statt eines gerenderten
`• ` steht das rohe Markdown-Zeichen `- ` da, dazu der Zeilenumbruch der
Quelle, und die `Listenzeile` fehlt ganz, also rückt die Zeile nicht ein.

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:182`,
beide Fassungen unverändert in dasselbe Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`):

```
Quelle : "- \n"
f401dcc: "• "        Listenzeile{1} ueber "• "
c35f8b1: "- \n"      keine Auszeichnung

Quelle : "-\n"
f401dcc: "• "
c35f8b1: "-\n"

Quelle : "- eins\n- \n"
f401dcc: "• eins\n• "
c35f8b1: "• eins\n- \n"      eine Liste, zwei verschiedene Merkzeichen

Quelle : "- \n- zwei\n"
f401dcc: "• \n• zwei"
c35f8b1: "- \n• zwei"

Quelle : "- \n\nAbsatz\n"
f401dcc: "• \n\nAbsatz"
c35f8b1: "- \n\nAbsatz"

Quelle : "-\n" x 2000
c35f8b1: 4000 Zeichen roher Quelltext, 0 Auszeichnungen
```

**Drei Dinge auf einmal.**

1. **Das rohe Zeichen.** In derselben Liste steht `• ` neben `- `. Welches der
   beiden ein Leser sieht, hängt davon ab, ob der Punkt Text trägt.
2. **Der Zeilenumbruch der Quelle.** `Offen::quelle` eines Punktes schließt
   sein `\n` ein, und `Zerlegung::woertlich` schreibt es mit. Damit umgeht der
   Punkt die Abstandsrechnung aus `Zerlegung::absetzen`, die für jeden anderen
   Block gilt — der Modulkopf nennt sie unter „Die Abstaende zwischen den
   Bloecken" und begründet sie mit dem Auffüllen statt Anhängen. Der letzte
   Fall oben endet deshalb mit einem Umbruch, was sonst keine Ausgabe tut.
3. **Der fehlende Einzug.** Der wörtliche Zweig in `Zerlegung::schliessen`
   (`markdown.rs:827-829`) liegt im `else` zu `laenge > 0`, und nur der
   `if`-Zweig trägt die `Auszeichnungsstelle` ein. Ein leerer Punkt bekommt
   also keine `Listenzeile` und rückt in AppKit nicht ein — auch dann nicht,
   wenn er auf Ebene drei steht.

**Die Ursache.** `Zerlegung::punkt_oeffnen` schreibt das Merkzeichen nicht
mehr, sondern merkt es vor (`markdown.rs:758-760`). Ein Punkt ohne Inhalt löst
den Wunsch nie ein, also bleibt `laenge == 0`, und `schliessen` nimmt den
wörtlichen Zweig. Für den Verweisdefinitions-Fall ist genau das gewollt und
notwendig — nur so kommt die Definition überhaupt heraus. Der leere Punkt
trägt aber nichts, was herauszugeben wäre, außer seinem eigenen Merkzeichen.

**Es ist im Modulkopf benannt** („Ein Punkt ohne jeden Inhalt fällt nicht
darunter — er hat kein Zeichen geliefert und gibt nach Satz 3 seinen
Quellbereich wörtlich heraus, sein `- ` eingeschlossen", `markdown.rs:103-105`),
also kein Widerspruch zwischen Zusage und Baum. Es ist gemessen aber keine
Probe da, die den **leeren** Punkt festhält:
`ein_punkt_ohne_ein_einziges_zeichen_bleibt_als_sein_quelltext_stehen`
(`markdown.rs:1249`) misst `"- [ref]: http://a.example\n"`, also den Fall mit
Inhalt.

**Ein Zuschnitt** (nicht gewählt): der wörtliche Zweig könnte greifen, sobald
der Quellbereich **mehr** trägt als das Merkzeichen des Elements, und sonst
den Wunsch einlösen. Ob diese Frage mechanisch entscheidbar ist, ohne die
Merkzeichenlängen der Quelle zu vermessen, ist hier nicht geprüft — der
Modulkopf sagt an anderer Stelle, eine Regel, die das Merkzeichen eines
Containers vom Quelltext davor trennt, sei nicht mechanisch zu haben.

**Gewicht: mittel.** Ein leerer Listenpunkt ist selten, aber die Ausgabe ist
auf den ersten Blick als roh zu erkennen, und die Liste zeigt zwei
verschiedene Merkzeichen nebeneinander. Kein Absturz und keine falsche
Auskunft über den Inhalt.

**Herkunft:** Circle der Runde 6, Turn 4, `c35f8b1`.

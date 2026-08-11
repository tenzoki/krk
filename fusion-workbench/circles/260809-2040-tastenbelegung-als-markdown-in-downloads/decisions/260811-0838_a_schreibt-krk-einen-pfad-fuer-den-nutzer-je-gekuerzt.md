# Schreibt KRK einen Pfad für den Nutzer je gekürzt, oder immer ausgeschrieben?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0838_*_plan-tastenbelegung-als-markdown-in-downloads.md` (Frage 8 unter `## Antworten auf die acht Punkte`), `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md` (`## Offen für den Planner`, letzter Punkt, und C4), `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` (C11, der Fenstertitel)

---

## Question

Die Erfolgsmeldung aus C4 nennt den vollen Pfad der geschriebenen Datei. Der Nutzer hat sie am 260811-0115 in der Form "Tastenbelegung geschrieben: ~/Downloads/KRK-Tastenbelegung.md" genannt, also mit der Tilde für das Benutzerverzeichnis. Der Spec überweist die Darstellungsform dem Planner und nennt sie eine Kleinigkeit.

Sie ist die Kleinigkeit, die der Spec sieht, und dazu eine Festlegung, die er nicht sehen konnte. KRK hat die Frage nämlich schon einmal beantwortet, an der anderen Fläche desselben Fensters: **der Fenstertitel aus C11 der Runde 2 kürzt ausdrücklich nicht.** Sein Modulkopf schreibt es aus, und zwar als Nutzerverlangen vom 260809 nach dem absoluten Pfad: "Kein Ersetzen des Benutzerordners durch eine Tilde, kein Auslassen von Zwischenordnern" (`crates/krk-ui/src/fenstertitel.rs:37-40`). Am Code geprüft am 260811-0838: `grep -rn '~' crates/krk-ui/src/` liefert außerhalb von Kommentaren genau einen Treffer, und der ist eine Probe, die die **Abwesenheit** der Tilde im Fenstertitel festhält (`fenstertitel.rs:170`). KRK erzeugt heute an keiner Stelle einen gekürzten Pfad.

Die Frage lautet damit nicht "welche Zeichenkette steht in dieser einen Meldung", sondern: **schreibt KRK einen Pfad für den Nutzer künftig an einer Fläche gekürzt und an einer anderen ausgeschrieben, oder gilt eine Form für beide?** Titelbalken und Statuszeile stehen im selben Fenster übereinander, und die Ausgabe ist die erste Gelegenheit, an der beide denselben Pfad zeigen könnten.

## Options

1. **Ausgeschrieben, ohne Tilde.** Die Meldung lautet "Tastenbelegung geschrieben: /Users/<name>/Downloads/KRK-Tastenbelegung.md".
   - Pro: eine Form für beide Flächen. Der Pfad ist auswählbar, kopierbar und in jedem Terminal ohne Übersetzung brauchbar. Es entsteht kein zweiter Mechanismus und keine Regel für den Fall, dass das Benutzerverzeichnis nicht als Präfix passt.
   - Contra: die Meldung wird länger, und der Nutzer hat sie anders genannt. Der Name des Benutzers steht in der Statuszeile, was bei einer Bildschirmaufnahme mitgeht.
2. **Mit Tilde, wie der Nutzer die Meldung genannt hat.** Die Meldung lautet "Tastenbelegung geschrieben: ~/Downloads/KRK-Tastenbelegung.md".
   - Pro: kürzer, entspricht dem Wortlaut der Nutzerantwort vom 260811-0115, und `~/Downloads` ist die Schreibweise, in der der Ordner in der Directive und im Spec durchgehend vorkommt.
   - Contra: KRK bekommt zwei Formen für denselben Pfad an zwei Flächen desselben Fensters. Die Kürzung braucht eine eigene reine Funktion samt einer Regel für den Fall, dass der Pfad nicht unter dem Benutzerverzeichnis liegt; in dieser Runde kann er es nicht, in einer späteren mit einstellbarem Zielordner schon.
3. **Beide Flächen kürzen, also auch der Fenstertitel.** Eine gemeinsame Kürzungsfunktion, von Titel und Statuszeile benutzt.
   - Pro: eine Form für beide Flächen, und es ist die kürzere.
   - Contra: hebt eine Nutzerentscheidung vom 260809 auf und ändert eine Fähigkeit der Runde 2, die abgenommen ist. Das liegt außerhalb der Directive dieses Circles.

## Constraints

- Die Directive dieser Runde sagt eine Ausgabedatei zu und keine Änderung an einer bestehenden Anzeige. Möglichkeit 3 überschreitet sie und ist hier nur der Vollständigkeit halber genannt.
- Der Wortlaut der Erfolgsmeldung ist im Übrigen entschieden: **eine** Meldung für die neu entstandene wie für die ersetzte Datei, mit dem vollen Pfad. Diese Frage betrifft allein die Schreibweise des Pfades darin.
- Die Antwort hält keinen Schritt des Plans auf. Sie kostet eine Zeile in einer reinen Funktion in `crates/krk-ui/src/belegungsausgabe.rs` und eine Probe daneben, und zwar in jede Richtung.

## Recommendation

**Wir empfehlen Möglichkeit 1, den ausgeschriebenen Pfad.** Der Grund ist nicht die Länge der Meldung, sondern die Zahl der Formen: KRK hat für "wie schreibt man dem Nutzer einen Pfad hin" bereits eine Antwort, sie ist vom Nutzer selbst gesetzt, und sie steht am selben Fenster. Eine zweite Form daneben ist dieselbe Sorte Kosten, die dieser Circle an anderer Stelle ausdrücklich vermeidet, wenn seine Directive eine zweite Aufbereitung ausschließt.

**Was daran eine Auslegung ist:** ob der Nutzer die Tilde als Wortlaut gemeint hat oder als Abkürzung im Gespräch. Wer beim Diktieren einer Meldung "~/Downloads" sagt, meint möglicherweise nur den Ordner und nicht die Schreibweise. Das kann nur er sagen, und deshalb steht die Frage hier statt still im Plan.

**Folgen jenseits dieser Runde.** Fällt die Antwort auf Möglichkeit 2, ist die Kürzung von da an KRKs Form für Meldungen, und die nächste Fläche, die einen Pfad meldet, erbt die Frage erneut, weil der Fenstertitel weiterhin ausschreibt. Fällt sie auf Möglichkeit 1, ist die Form für beide Flächen dieselbe, und ein späterer Wunsch nach Kürzung ist eine eigene, dann gemeinsame Entscheidung für Titel und Statuszeile.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Moeglichkeit 2, mit Tilde** — Nutzerantwort am 260811-0900, **gegen die Empfehlung
dieses Datensatzes**. Die Meldung lautet "Tastenbelegung geschrieben:
~/Downloads/KRK-Tastenbelegung.md".

Damit ist die Auslegungsfrage entschieden, die der Datensatz oben offengelassen hat: der Nutzer
hat die Tilde als **Wortlaut** gemeint und nicht als Abkuerzung im Gespraech.

**Was die Antwort kostet, und es ist mehr als eine Zeichenkette.** KRK bekommt zwei Formen fuer
denselben Pfad an zwei Flaechen desselben Fensters: der Fenstertitel schreibt aus, die
Statuszeile kuerzt. Die Kuerzung braucht eine eigene reine Funktion, und die braucht eine Regel
fuer den Fall, dass der Pfad nicht unter dem Benutzerverzeichnis liegt. In dieser Runde kann er
es nicht — das Ziel ist fest der Downloads-Ordner —, in einer spaeteren mit einstellbarem
Zielordner schon. Die Regel gehoert trotzdem jetzt gebaut, weil eine Funktion, die einen Fall
nicht kennt, ihn beim ersten Auftreten falsch beantwortet.

**Was daraus fuer spaeter folgt, und der Datensatz hat es vorhergesagt.** Die Kuerzung ist von
nun an KRKs Form fuer Meldungen. Die naechste Flaeche, die einen Pfad meldet, erbt die Frage
erneut, weil der Fenstertitel weiterhin ausschreibt. Wer diese Ungleichheit spaeter aufloesen
will, hebt entweder die Nutzerentscheidung vom 260809 zum Fenstertitel auf oder nimmt die
Kuerzung hier zurueck; beides ist dann eine eigene Entscheidung.

**Die Entscheidung vom 260809 zum Fenstertitel bleibt unberuehrt.** Moeglichkeit 3, beide
Flaechen zu kuerzen, ist nicht gewaehlt worden; `crates/krk-ui/src/fenstertitel.rs` wird von
dieser Runde nicht angefasst.

Der Plan `260811-0838_*_plan-tastenbelegung-als-markdown-in-downloads.md` zieht die Kuerzung in
Schritt S3 nach.

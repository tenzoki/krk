Der Kurzhinweis der Statuszeile veraltet bei einer Fensteränderung

---

`Statuszeile::zeigen` (`crates/krk-ui/src/appkit/statuszeile.rs`) misst beim
Setzen des Textes, ob er in die Zeile passt, und setzt danach den Kurzhinweis
oder nimmt ihn weg. **Die Breite, gegen die gemessen wird, ändert sich aber
auch ohne neue Meldung**: sie steht im Rahmen des Textfeldes, und den zieht die
Autogröße bei jeder Fensteränderung nach. Zwischen zwei Meldungen kann der
Hinweis deshalb falsch stehen.

---

**Zwei Richtungen, beide erreichbar:**

1. Eine lange Meldung steht, der Nutzer zieht das Fenster **breiter**. Der Text
   passt jetzt hinein, der Hinweis steht weiter da und wiederholt beim
   Verweilen genau den Satz, der ohnehin zu lesen ist. Das ist das Rauschen,
   das der Entscheid vom 260812 ausdrücklich vermeiden wollte
   (`decisions/260812-1809_*_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`,
   Möglichkeit 2: „gesetzt nur dann, wenn der Text abgeschnitten ist").
2. Eine kurze Meldung steht, der Nutzer zieht das Fenster **schmaler**. Sie
   wird jetzt gekürzt und hat keinen Hinweis; genau in der Lage, für die er
   gebaut ist, fehlt er.

Die nächste Meldung zieht ihn in beiden Fällen nach. Betroffen sind allein die
Ränge, die stehen bleiben — die Tabmeldung, die Fenstermeldung und die
Befehlsantwort halten ihren Text bis zum nächsten Ereignis.

**Warum der Nachzug nicht mitgebaut ist.** Der eine Auslösepunkt einer
Breitenänderung am Feld ist `setFrameSize:`, und ihn zu überschreiben verlangte
eine eigene Klasse über `NSTextField`. Die ließe sich nicht mehr über
`labelWithString:` bauen — und genau dieser Erzeuger ist die ganze Grundlage,
auf der C5.11 heute ruht („a non-wrapping, non-editable, non-selectable text
field", `NSTextField.h:87-93`). Der Nachzug kostete also die Zusage, die er
begleiten soll. Das ist derselbe Tausch, den der Entscheid vom 260812 für die
Bildlaufansicht abgelehnt hat, nur an einer anderen Stelle.

**Drei Zuschnitte, keiner ist hier gewählt:**

1. **So lassen und benannt stehen lassen.** Der Modulkopf von `statuszeile.rs`
   sagt es aus. Preis: das Rauschen aus Fall 1 tritt auf, wenn auch selten und
   ohne Schaden.
2. **Der Kurzhinweis wird lazy beantwortet.** `addToolTipRect:owner:userData:`
   fragt seinen Besitzer über `view:stringForToolTip:point:userData:` erst in
   dem Augenblick, in dem der Hinweis erscheinen soll; dann ist die Antwort
   immer die richtige, ohne jeden Nachzug. Preis: eine eigene Klasse als
   Besitzer, also eine Bauart, die im Baum bisher nicht vorkommt, und ein
   Mittel neben `setToolTip:` statt desselben. C5.11 bliebe unberührt, weil
   das Feld unverändert aus `labelWithString:` kommt.
3. **Ein Nachzug am Fenster.** `windowDidResize:` am `FensterDelegierter`
   (`crates/krk-ui/src/appkit/fenster.rs`) meldet an den
   Anwendungsdelegierten, der nur den Hinweis neu setzt. Preis: ein zweiter
   Anlass an einer Stelle, die heute genau eine Aufgabe hat, und ein Ruf je
   Bild einer laufenden Ziehbewegung.

**Gewicht:** gering. Kein falscher Satz, keine verlorene Auskunft — der Hinweis
ist entweder überflüssig oder fehlt, und die nächste Meldung räumt beides auf.

**Herkunft:** Circle der Runde 6, Turn 3, Rücknahme von Schritt 11 (C5.10).

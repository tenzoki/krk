Die Directive des aktiven Circles sagt weiterhin eine blätterbare Statuszeile zu, obwohl der Nutzer das Blättern zurückgenommen hat

---

`fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_t_circle.md:14`
trägt im Abschnitt `## Directive` den Satz: „**Fünftens zieht die Statuszeile
über die volle Fensterbreite und lässt sich nach rechts blättern.**" Das
Blättern ist am 260812 vom Nutzer zurückgenommen worden, und `df4ec00` hat es
aus dem Baum entfernt.

---

**Der Stand am Baum.** `crates/krk-ui/src/appkit/statuszeile.rs` hält wieder
allein ein `NSTextField` aus `labelWithString:`
(`statuszeile.rs:492-494, 507`); `NSScrollView` kommt in der Datei nur noch in
Prosa vor, die die Rücknahme erklärt. Nachgezählt über den ganzen Baum: keine
`NSScrollView` in `statuszeile.rs`, kein `breite_nachziehen`, kein
`an_den_anfang`.

**Der Stand in den Datensätzen.** Der Nutzerentscheid steht in
`decisions/260812-1809_i_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`
und sagt ausdrücklich: „**C5.10 ist damit überholt.** Der Wortlaut „Die Zeile
lässt sich nach rechts blättern" wird ersetzt". Der Vorgänger
`decisions/260812-1105_s_…` steht auf überholt.

**Die Directive ist nicht mitgezogen worden.** Sie ist der Maßstab, an dem der
Circle beim Abschluss gemessen wird, und sie steht nicht in einem der Speicher,
für die `CLAUDE.md` die Ausnahme „Aufzeichnungen eines Standes behalten ihren
damaligen Marker" zieht — jene Ausnahme gilt für `history/`, `reviews/`,
`analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/`, nicht für den
Circle-Datensatz selbst.

**Dass die Directive nachgezogen wird, ist an diesem Circle schon geübt.**
Derselbe Satz trägt den Halbsatz „Diese fünfte Fähigkeit ist am 260812-1105 auf
Vorgabe des Nutzers hinzugekommen" — die Directive ist also bereits einmal
während der Runde geändert worden, als der Nutzer die Fähigkeit vorgab. Jetzt
fehlt der Gegenzug.

**Der Turn-Eintrag desselben Datensatzes weiß es.** `_t_circle.md:171` hält für
Turn 2 fest: „der Nutzer hat waehrend des Turns C5.10 ueberholt (Kurzhinweis
statt Blaettern) … Schritt 11 ist damit zurueckzunehmen." Der Turnlog ist eine
Aufzeichnung eines Standes und richtig; die Directive darüber ist es nicht.

**Was zu tun ist.** Den Halbsatz „und lässt sich nach rechts blättern" in
`_t_circle.md:14` durch die Fassung ersetzen, die der Entscheid vom 260812-1809
gewählt hat: eine Meldung, die breiter ist als das Fenster, wird über einen
Kurzhinweis beim Verweilen vollständig lesbar. Der Rest des Satzes gilt fort
und darf nicht mit weggeschrieben werden — eine Zeile statt zweier, über die
volle Breite, mit der Zuordnung im Text.

**Gewicht:** mittel. Kein Code betroffen, aber der Circle wird an dieser
Directive abgeschlossen, und ein Abschluss gegen eine zurückgenommene Zusage
liest sich später wie ein verfehltes Kriterium.

**Herkunft:** Circle der Runde 6, Turn 3, Rücknahme von Schritt 11 (C5.10).

---
Resolved: Die Directive traegt die Aenderung jetzt selbst. Auf Nutzerentscheid vom 260812 sind
zwei Stellen nachgezogen worden: die fuenfte Faehigkeit sagt statt der blaetterbaren Statuszeile
den Kurzhinweis beim Verweilen zu, und der Zaehlsatz nennt fuenf Dinge statt vier. Ein Nachtrag
am Ende der Directive haelt fest, was geaendert wurde und warum, und verweist auf den Entscheid
`decisions/260812-1809_*_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md` sowie
auf seinen ueberholten Vorgaenger.

Eine Notiz allein in der Abschlussnotiz haette nicht genuegt, und das Argument stammt aus dem
Abgleich `history/260812-2253-reconciliation.md`: `portfolio.md` zitiert die Directive-Zeile ohne
die Notiz, und der Abnahmelauf geht von der Kriterienliste aus, nicht vom Dateiende. Die Regel,
die den Weg oeffnet, steht in `rules/circle-records.md`, wo die Directive ausdruecklich als
ueber das Rebalance revidierbar gefuehrt wird: eigenmaechtig darf der Orchestrator sie nicht
umschreiben, mit Zustimmung des Nutzers schon.

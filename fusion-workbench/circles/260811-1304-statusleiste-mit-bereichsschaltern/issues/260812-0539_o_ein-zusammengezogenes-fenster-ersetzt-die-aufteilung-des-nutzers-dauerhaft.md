Ein zusammengezogenes Fenster ersetzt die Aufteilung des Nutzers dauerhaft

---

Zieht der Nutzer das Fenster so schmal, dass ein sichtbarer Bereich an seinem Mindestmaß hängt,
wird die gedeckelte Breite zum neuen Wunsch dieses Bereichs. Beim Wiederaufziehen kommt seine alte
Aufteilung nicht zurück. Unter der Anteilsregel trifft das nicht mehr einen Bereich, sondern alle
zugleich: fällt das Fenster unter die Summe der Mindestbreiten, ersetzt ein einziges Hin und Her
am Fensterrand die Aufteilung des Nutzers durch das Verhältnis der Mindestbreiten. Kein Tastenbefehl
ist dafür nötig.

---

**Schwere:** mittel (kein Absturz, kein Datenverlust; eine vom Nutzer eingestellte Aufteilung geht
ohne sein Zutun und ohne Meldung verloren)
**Gefunden:** coderev, Durchsicht der Commits `5e17c9e`, `a2ea876`, `8ffaac2`
**Betroffen:** `crates/krk-ui/src/appkit/aufteilung.rs`, `neu_auslegen` (`:151`) und
`crates/krk-ui/src/fenstermodell.rs`, `breiten_uebernehmen` (`:692`)
**Domain:** code

## Nachgerechnet

Zwei Wege führen dorthin, und beide sind mit einem Nachbau der Rechnungen aus dem Stand `8ffaac2`
nachgerechnet. Lage in beiden Fällen: Editor sichtbar, Vorschau aus, Auslieferungsbreiten, Fenster
von 1280 auf 780 Punkte und zurück.

**Der Weg über den Schirm, ganz ohne Befehl.** `splitView:resizeSubviewsWithOldSize:`
(`aufteilung.rs:151`) speist bei jedem Bild die **gemessenen** Breiten wieder als Wünsche ein. Das
ist unschädlich, solange nichts gedeckelt ist, denn dann ist die Abbildung ein einheitlicher Faktor
und damit idempotent. Sobald gedeckelt wird, ist sie es nicht mehr:

```
bei 1280:          [155.31, 362.39, 362.39, 0.00, 396.91]
auf  780 gezogen:  [101.35, 202.70, 202.70, 0.00, 270.26]
wieder auf 1280:   [166.57, 333.13, 333.13, 0.00, 444.17]
haette sein sollen:[155.31, 362.39, 362.39, 0.00, 396.91]
```

Die Dateifenster verlieren 8,1 Prozent, der Editor gewinnt 11,9. Das neue Verhältnis ist genau das
der Mindestbreiten (120 : 240 : 240 : 320) und trägt von der Einstellung des Nutzers nichts mehr.

**Der Weg über das Modell.** Der nächste Befehl ruft `bildschirmbreiten_uebernehmen`
(`anwendung.rs:2614`), und `breiten_uebernehmen` schreibt die gemessenen Breiten auf die
gespeicherte Summe zurückgerechnet in das Modell. Aus 180/420/420/460 werden dabei
193/386/386/515. Damit steht die verlorene Aufteilung auch in `session.toml` und übersteht den
Neustart.

## Was das Kriterium C4.7 dazu sagt

C4.7 des Plans lautet: "Das Vergrößern des Fensters ändert keine gespeicherte Breite." Die Probe
dazu, `das_vergroessern_des_fensters_laesst_die_gespeicherten_breiten_stehen`
(`fenstermodell.rs:1869`), misst von 1280 auf 2000 Punkte, also in einer Lage, in der kein Bereich
gedeckelt ist; dort hält die Zusage. Sie hält nicht, sobald das Verkleinern davor einen Bereich auf
sein Mindestmaß gedrückt hat, und den Fall misst keine Probe.

## Was hier nicht neu ist, und was doch

**Der Kern ist als Risiko benannt und angenommen.** Die Risikotafel des Plans führt in ihrer ersten
Zeile: "Die gedeckelte Breite eines Bereichs wird beim nächsten Nachlesen sein neuer Wunsch, und
beim Vergrößern des Fensters kehrt er nicht auf seine alte Zahl zurück." Zwei Dinge stehen dort
nicht:

- **Der Umfang.** Die Zeile spricht von *einem* Bereich. Im zweiten Zweig sind es alle sichtbaren
  zugleich, und das Ergebnis ist nicht eine verschobene Zahl, sondern eine Aufteilung, die von der
  des Nutzers nichts mehr enthält. Der Kommentar an `MINDESTGROESSE` (`fenster.rs:110`) hält den
  Unterschied für den Schirm ausdrücklich fest ("Bis zur Bereichsleisten-Runde traf es allein den
  Editor ... schickt dann **alle vier**"); dass er auch für die gespeicherten Zahlen gilt, steht
  nirgends.
- **Der Weg.** Die Zeile setzt ein "Nachlesen" voraus, also einen Befehl. Der Weg über
  `neu_auslegen` braucht keinen: das bloße Ziehen am Fensterrand genügt, und der Befehl schreibt
  danach nur noch fest, was der Schirm schon verloren hat.

## Drei Wege, keiner im Vorbeigehen

1. **Den zweiten Zweig unerreichbar machen.** `MINDESTGROESSE` in der Breite auf 940 heben, dann
   passen die Mindestbreiten immer. Der gedeckelte Einzelfall aus der Risikotafel bliebe bestehen.
   Das ist die Möglichkeit, die
   `decisions/260812-0415_o_was-geschieht-wenn-das-fenster-unter-die-summe-der-mindestbreiten-faellt.md`
   als Nutzerentscheidung führt.
2. **Den Rückweg abschneiden.** `neu_auslegen` speist nicht mehr die gemessenen Breiten ein, sondern
   die gespeicherten des Modells. Das kostet die Zusage aus dem Modulkopf von `aufteilung.rs`, dass
   eine mit der Maus verschobene Trennlinie die nächste Fenstergrößenänderung übersteht, und ist
   deshalb keine reine Reparatur, sondern eine Änderung an einer Zusage.
3. **Nichts tun und die Grenze schreiben.** Wenn die Lage angenommen bleibt, gehört ihr Umfang in
   die Kommentare an `breiten_uebernehmen` und `neu_auslegen`, denn beide sagen heute die
   Idempotenz zu, ohne ihre Grenze zu nennen.

Weg 1 und Weg 2 schließen einander nicht aus, und Weg 1 ist bereits eine offene Nutzerfrage. Die
Reihenfolge ist deshalb: erst die Nutzerfrage beantworten, dann bauen.

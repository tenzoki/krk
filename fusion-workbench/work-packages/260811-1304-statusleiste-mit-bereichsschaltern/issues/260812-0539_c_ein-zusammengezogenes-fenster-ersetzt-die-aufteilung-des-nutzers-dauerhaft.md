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

---

Resolved: 260812-0700, coder. **Ein vierter Weg, und er ist keiner der drei benannten.** Die drei
lauteten: den zweiten Zweig unerreichbar machen (vom Nutzer am 260812-0430 abgelehnt), den Rückweg
abschneiden (kostet die Zusage, dass eine Ziehbewegung die Größenänderung übersteht), oder nichts
tun. Der gegangene Weg schneidet den Rückweg nicht ab, sondern **fragt vorher, ob überhaupt etwas
zurückzulesen ist**.

## Die Regel, in einem Satz

*Vom Schirm wird nur zurückgelesen, was die Regel nicht selbst ausgelegt hat.*

Der Grund, aus dem das die richtige Frage ist: die Rückrechnung gibt es allein deshalb, weil eine
mit der Maus verschobene Trennlinie in den Rahmen der Ansichten steht und nirgends sonst. Alles
andere auf dem Schirm ist die Ausgabe von `bereichsbreiten` selbst, und die als deren Eingabe
wieder einzuspeisen ist genau die Stelle, an der die Schleife ihre Neutralität verliert, sobald
gedeckelt wird.

**Die Frage ist damit entscheidbar, und die naheliegende wäre es nicht.** „Ist die Abbildung
zwischen Wunsch und Bildschirmpunkt umkehrbar?" lässt sich aus den Rahmen allein nicht beantworten:
ein Bereich, der genau auf seinem Mindestmaß steht, kann dort gedeckelt worden sein oder vom Nutzer
hingezogen. Die engere Frage „steht dort etwas anderes, als das Auslegen hingeschrieben hat?" kennt
diesen Zweifel nicht.

## Was gebaut ist

- `fenstermodell::traegt_eine_ziehbewegung` (neu, privat) rechnet die Zeile aus den gehaltenen
  Breiten aus und vergleicht sie mit der gemessenen. Der Spielraum ist ein Viertelpunkt
  (`ZIEHSPIELRAUM`): unter dem kleinsten Schritt, mit dem sich eine Trennlinie ziehen lässt, und
  über dem, was ein Runden der Rahmen hinterließe.
- `Fenstermodell::breiten_uebernehmen` nimmt jetzt das `Zeilenmass` als zweiten Parameter und kehrt
  ohne Wirkung zurück, wenn die gemessene Zeile keine Ziehbewegung trägt. Das schließt den **Weg
  über das Modell**.
- `fenstermodell::wuensche_nachfuehren` (neu, öffentlich) beantwortet dieselbe Frage für das
  Auslegen und steht im Fenstermodell, damit sie ohne Fenster prüfbar ist.
- `appkit::aufteilung`: der Delegierte hält die Wünsche jetzt selbst
  (`AufteilungsIvars::wuensche`), statt sie aus den Rahmen der Unteransichten zurückzulesen.
  `neu_auslegen` fragt `wuensche_nachfuehren` und benutzt dafür endlich `alte_groesse` — die
  gemessenen Breiten sind unter der **alten** Zeilenbreite entstanden, und nur an ihr gemessen
  lässt sich sagen, ob sie von der Regel stammen. Das schließt den **Weg über den Schirm**.
  `Aufteilung::anwenden` trägt den Wunsch des Fenstermodells in dasselbe Feld ein.

**Der Rahmen war der falsche Speicher, und das ist die Ursache hinter beiden Wegen.** Er trägt
unter einer Deckelung die Deckelung und nicht mehr den Wunsch. Es entsteht dadurch kein Rückweg vom
Delegierten in das Fenstermodell und kein Ring: er hält einen Wert, keine Sicht auf das Modell.

**Die Zusage aus dem Modulkopf bleibt.** Eine mit der Maus verschobene Trennlinie steht anders im
Rahmen, als die Regel sie hingeschrieben hat, gilt also als Ziehbewegung und wird übernommen —
auch dann, wenn sie an einem Mindestmaß endet.

## Nachgerechnet und geprobt

Die Zahlen des Datensatzes sind vor der Änderung mit einem eigenen Nachbau außerhalb des Baums
nachgerechnet und gehen auf: 1280 → 780 → 1280 liefert `[166.57, 333.13, 333.13, 0.00, 444.17]`
statt `[155.31, 362.39, 362.39, 0.00, 396.91]`, die Dateifenster also 8,1 Prozent schmaler und der
Editor 11,9 Prozent breiter.

Drei Proben in `crates/krk-ui/src/fenstermodell.rs`:

- `ein_zusammengezogenes_fenster_laesst_die_gespeicherten_breiten_stehen` — bei 600 Punkten steht
  das 600/760-Fache der Mindestbreiten auf dem Schirm; das Nachlesen lässt das Modell unangetastet.
  Gegengeprobt: ohne die Frage schreibt es `[202.11, 404.21, 404.21, 269.47]`.
- `ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her` — 1280 → 600 → 1280 über
  dieselbe Folge von Aufrufen, die die Aufteilung fährt, nur ohne Fenster. Es kommt
  `[180, 420, 420, 260]` zurück. Gegengeprobt: ohne die Frage `[202.11, 404.21, 404.21, 269.47]`.
- `eine_mit_der_maus_verschobene_trennlinie_gilt_als_neuer_wunsch` — die Gegenprobe zur
  Über-Vorsicht: eine um 60 Punkte verschobene Linie überlebt den Sprung auf 1600 Punkte
  (`[225, 600, 450, 325]`).

**Was ohne Fenster nicht zu prüfen ist**, steht hier ausdrücklich: ob AppKit die Rahmen der
Unteransichten unverändert stehen lässt, nachdem `auslegen` sie gesetzt hat. Tut es das nicht,
sieht jede Größenänderung wie eine Ziehbewegung aus, und das Verhalten fällt auf das von vor dem
260812 zurück — also auf diesen Defekt, nicht auf einen schlimmeren. Die Wahl des Spielraums fällt
damit auf die sichere Seite.

Abgenommen mit `make check`, exit 0.

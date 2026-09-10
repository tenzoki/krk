# Was zeigt das Blatt auf Abruf, wenn der Start nichts erhoben hat?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Cross-references:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md` Schritte 4, 6 und 9; `260910-0818_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l4.md` (die Zusage, an der die Sparsamkeit hängt); `crates/krk-ui/src/appkit/blaetter/neuerungen.rs`, `crates/krk-ui/src/appkit/anwendung.rs` (`AnwendungsIvars::neuerungen`)

---

## Question

Schritt 4 hält den erhobenen `Bestand` in einem `ivar`, und Schritt 9 soll das Blatt „mit
dem gehaltenen `Bestand`" zeigen. **Beim häufigsten Start ist keiner gehalten.**

Der Grund ist gewollt: die Erhebung steht hinter dem Fassungsvergleich, damit im
Dauerbetrieb keine der drei Dateien ein zweites Mal geöffnet wird — genau die Bedingung,
unter der der L4-Datensatz dieser Runde mit Möglichkeit 1 beantwortet ist. Stimmt der
Merker, wird nichts erhoben, und der `ivar` steht auf `None`. Das trifft **jeden zweiten
und jeden weiteren Start derselben Fassung**, also fast jeden.

`None` heißt deshalb „nicht erhoben" und nicht „kein Unterschied". Ein erhobener Bestand
ohne Unterschied steht als `Some` da und trägt die vollen Pfade; die zwei Auskünfte sind
auseinanderzuhalten, und der `ivar` hält sie auseinander. Was der Befehl aus Schritt 9 im
`None`-Fall zeigt, ist damit nicht entschieden.

Schritt 6 hat das Blatt gebaut und dabei ausdrücklich festgehalten, dass es **auch ohne
einen einzigen Unterschied** aufgeht und die drei vollen Pfade zeigt: dort meldet KRK nicht
ungefragt, sondern der Nutzer hat gefragt. Dieselbe Erwägung stellt sich hier ein zweites
Mal, eine Ebene tiefer.

## Options

1. **Der Befehl erhebt auf Verlangen, wenn beim Start nichts erhoben wurde.**
   - Pro: Der Nutzer hat gefragt, also kostet das Lesen kein L4: die Zeitzusage misst den
     Start und nicht einen Tastendruck im laufenden Betrieb.
   - Pro: Das Blatt zeigt dann immer dasselbe, gleich ob es der erste Start dieser Fassung
     war oder der zwanzigste. Eine Auskunft, die davon abhinge, wann der Nutzer sie abruft,
     wäre schwer zu erklären.
   - Pro: Es ist die Auskunft, um die der Nutzer bittet. Ein Blatt, das ihm den vollen Pfad
     zeigt und die Namen verschweigt, beantwortet die Frage halb.
   - Contra: Die Erhebung braucht einen `Zugang`, also die Schreibsperre. Sie steht
     während des Betriebs zur Verfügung (`unter_der_sperre`), aber der Befehl bekäme damit
     zwei Ausgänge — Blatt und Sperrhindernis —, und jeder Rufer von `unter_der_sperre`
     entscheidet die zwei Fälle einzeln.
   - Contra: Der gezeigte Stand ist dann **nicht** mehr der vom Start. Der Plan begründet
     unter „Where this Circle stops" ausdrücklich, warum das Blatt den Startstand zeigt:
     die Leseprofile und die Belegung, mit denen KRK arbeitet, sind die vom Start, und ein
     Blatt, das die Platte neu läse, zeigte einen Bestand, den die laufende Anwendung gar
     nicht benutzt. Diese Möglichkeit nimmt das zurück — und zwar nur für den Fall, in dem
     beim Start nichts erhoben wurde, also asymmetrisch.

2. **Das Blatt zeigt die drei vollen Pfade und dazu den Satz, dass für diese Fassung schon
   gemeldet ist.**
   - Pro: Bleibt bei der Zusage, dass das Blatt den Stand vom Start zeigt, ohne Ausnahme.
   - Pro: Kostet keinen Zugang und keine Sperre; Schritt 9 bleibt ein reiner
     Ausführungszweig.
   - Contra: Der Befehl heißt „Neuerungen anzeigen" und zeigt fast immer keine. Das ist
     schwer als Zusage zu lesen und leicht als Defekt.
   - Contra: Der Nutzer, der die Startzeile gesehen und weggeklickt hat, kommt an die Namen
     nicht mehr heran — bis zur nächsten Fassung. Das ist genau der Weg, den die Directive
     mit „auf Verlangen im Einzelnen ansehen" eröffnen wollte.

3. **Beim Start wird immer erhoben, und der Merker steuert allein die Meldung.**
   - Pro: Der `ivar` steht dann immer auf `Some`, Schritt 9 wird trivial, und die
     Startzeile bleibt „einmal je Fassung".
   - Contra: **Bricht die Bedingung, unter der der L4-Datensatz beantwortet ist.** Dort
     steht die Annahme, dass im Dauerbetrieb keine der drei Dateien ein zweites Mal geöffnet
     wird, und eine Probe hält sie. Diese Möglichkeit macht die Probe rot und den Datensatz
     falsch; sie wäre nur zu haben, wenn der L4-Datensatz neu beantwortet wird.

## Constraints

- Die Zusage aus dem L4-Datensatz: ohne Fassungswechsel wird beim Start keine der drei
  Dateien ein zweites Mal geöffnet. Eine Probe hält sie
  (`bei_gleichem_merker_wird_keine_der_drei_dateien_geoeffnet`).
- KRK schreibt keine der drei Dateien. Keine Möglichkeit hier rührt daran.
- Das Blatt aus Schritt 6 nimmt einen `&Bestand` und setzt seinen Text daraus zusammen; es
  kennt keinen zweiten Eingang. Möglichkeit 2 bräuchte deshalb entweder einen leeren
  Bestand mit den Pfaden oder einen zweiten Text.

## Recommendation

**Möglichkeit 1.** Die Zusage „das Blatt zeigt den Stand vom Start" ist begründet mit dem,
was KRK **benutzt** — und für die Frage „was bringt diese Fassung mit, das Ihre Dateien
noch nicht führen" ist der benutzte Stand nicht der Gegenstand: gefragt ist der Stand der
Dateien, und der liegt auf der Platte. Die Sparsamkeit gehört an den Start, wo sie L4
schützt, und nicht an einen Tastendruck, den der Nutzer selbst ausgelöst hat.

Möglichkeit 3 scheidet aus, solange der L4-Datensatz steht.

Die Frage bindet Schritt 9 und sonst nichts. Sie hält ihn nicht auf: der Zweig lässt sich
für beide Antworten in derselben Stunde bauen.

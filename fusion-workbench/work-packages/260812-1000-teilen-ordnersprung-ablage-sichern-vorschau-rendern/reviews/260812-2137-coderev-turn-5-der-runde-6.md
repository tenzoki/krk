# Durchsicht Turn 5 der Runde 6 — das Merkzeichen gehört seinem Punkt

**Sender:** coderev
**Reviewed-range:** `1e4e01f..2c0b2a6`
**Not-opened:** none
**Datum:** 260812-2137
**Circle:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`

## Zusammenfassung

Der Hauptbefund ist behoben, und zwar nicht nur für die acht gemessenen Fälle,
sondern für die Klasse: über 648 systematisch erzeugte Quellen der Klasse und
über 400 000 zufällig zusammengesetzte Quellen findet die neue Fassung keinen
einzigen Verstoß, wo die Vorfassung 480 von 648 durchfallen lässt. Der
dritte Reparatur-Turn in Folge hat als erster keine sichtbare Verschlechterung
im Ausgabetext eingeschleppt: über alle 773 Markdown-Dateien dieses Baums sind
die Texte beider Fassungen zeichengleich, und 3 571 von 73 447 Bereichen sind
verschoben — genau die Verschiebung, die der Turn wollte.

Vier Befunde, keiner davon am Hauptbefund. Der schwerste ist eine
Geschwindigkeitsregression, die die Messung des Turns nicht sehen konnte, weil
sie nur die Länge einer Quelle variiert hat und nicht ihre Tiefe.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 1 |
| Niedrig | 3 |

## Wie geprüft wurde

`markdown::rendern` aus `1e4e01f` und aus `2c0b2a6`, beide unverändert in
dasselbe Prüfprogramm kopiert und gegeneinander gefahren; `pulldown-cmark
0.13.4`, `Options::empty()`, Profil `release`, Tafel Hell. `hervorhebung`
steht als Stellvertreter daneben, weil `linkfarbe` nur ein `Some` liefern
muss. Fünf Läufe:

1. **Klasse, systematisch** — sechs Merkzeichenformen (`- `, `* `, `+ `,
   `1. `, `7. `, `1) `) mal achtzehn erste Kinder mal sechs Umgebungen
   (bloß, im Zitat, im äußeren Punkt, hinter einem Absatz, vor einem Absatz,
   in einer losen Liste) = 648 Quellen.
2. **Zufall** — 400 000 Quellen aus 48 Bausteinen, Länge 1 bis 24 Bausteine.
   Geprüft auf: kein Bereich außer `Listenzeile` beginnt mit `• `; jede
   Stelle liegt innerhalb der gemeldeten Länge; keine Stelle schneidet ein
   UTF-16-Ersatzpaar; keine Stelle der Länge null; Sortierung nach Anfang.
3. **Korpus** — alle 773 Markdown-Dateien des Baums, Text und Bereiche beider
   Fassungen verglichen, dazu die Verschachtelung der `Listenzeile`-Bereiche.
4. **Geschwindigkeit** — die 1,05-MiB-Musterquelle des Turns, eine
   1,05-MiB-Quelle aus echten Dateien dieses Baums, eine flache Liste aus
   20 000 Punkten und sechs verschieden tief verschachtelte Listen.
5. **Baum** — `cargo fmt --all --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`: alle drei Exit 0,
   478 Proben im Binärziel `krk`.

## Was hält

**Der Hauptbefund ist geschlossen, und die Klasse mit ihm.**

| Lauf | `1e4e01f` | `2c0b2a6` |
|---|---|---|
| 648 Klassenquellen | 480 mit Verstoß | 0 |
| 400 000 Zufallsquellen | — | 0 |
| die 13 Quellen der neuen Probe | 13 von 13 mit Verstoß | 0 |

Der Doc-Kommentar der Probe sagt, sie hätte den Defekt zur Zeit von `c35f8b1`
gefangen. Das ist nachgemessen und gilt für **jede** der dreizehn Quellen,
nicht nur für einige.

**„Innere" stimmt, und es wird kein Eintrag übersprungen oder doppelt
gerückt.** `Zerlegung::offen` ist nach `anfang` aufsteigend sortiert, denn
`oeffnen` schreibt `self.stelle`, und `stelle` wächst nur. Die Prüfung
`eintrag.anfang == vorher` trifft deshalb genau die Einträge, die noch kein
Zeichen bekommen haben; ein Eintrag mit `anfang < vorher` trägt schon Text und
darf nicht rücken, `anfang > vorher` kann es nicht geben. Wird ein Eintrag bei
zwei aufeinanderfolgenden Stufen zweimal gerückt, so liegt er in beiden
Punkten und gehört hinter beide Merkzeichen. Die Einträge vor `stufe` sind
Vorfahren des Punktes und sollen sein Merkzeichen mitnehmen — jeder Vorfahre
eines Listenpunkts ist ein Containerblock (Liste, Punkt, Zitat) und trägt
entweder `Abschluss::Nichts` oder eine `Listenzeile`, für die der Einzug gilt.
Am Korpus gemessen: in beiden Fassungen tragen 6 634 von 6 732
`Listenzeile`-Bereichen ihr Merkzeichen, die 98 übrigen sind Zitatblöcke, und
kein `Listenzeile`-Bereich der Tiefe *d+1* liegt außerhalb eines Bereichs der
Tiefe *d*.

**`traegt_nur_sein_merkzeichen` ist mechanisch.** Die tragende Tatsache ist,
dass der Quellbereich eines Punktes bei seinem Merkzeichen anfängt. Das ist
eine Eigenschaft von `pulldown-cmark` und nicht von CommonMark, und sie ist
gemessen: bei `"> -\n"` liegt der Bereich des Punktes bei `2..4` und nicht bei
`0..4`, das `>` des Zitats steht also nicht darin; bei `"- a\n  - b\n"` liegt
der innere Punkt bei `6..10`, die zwei Leerzeichen Einzug stehen nicht darin.
Für die Funktion selbst reicht ohnehin weniger, als ihr Doc-Kommentar
behauptet: das Merkzeichen enthält keinen Leerraum, jeder Inhalt ist
mindestens ein Stück, also ist „nichts als das Merkzeichen" gleichbedeutend
mit „genau ein Stück". Bräche die Annahme, wäre die Folge ein ausbleibender
Fix und keine neue Verschlechterung — der Punkt fiele auf den wörtlichen
Zweig zurück, also auf das Verhalten von `c35f8b1`.

**`Inhaltsart::deckt_luecken` hält den Bau an.** Genau ein `match` im ganzen
Baum (`markdown.rs:519-523`), erschöpfend, ohne `_`-Zweig; kein `matches!`
über `Inhaltsart` ist übrig (der einzige verbliebene `matches!` im Modul steht
im Prüfmodul und fragt nach `Auszeichnung::Listenzeile`). Zwei Rufer,
`luecke_bis` und `schliessen`, eine Entscheidung.

**`ohne_umgebungszeichen` hat genau einen Rufer**, und der liegt im
Nicht-Dokumentebenen-Zweig (`markdown.rs:761`). Die Trennung hängt an
demselben `self.offen.is_empty()`, an dem Satz 1 und Satz 2 der Deckung ohnehin
auseinandergehen — die Zusage stimmt.

**Keine vorhandene Probe ist abgeschwächt.** Die Löschzeilen des Diffs
berühren ausschließlich Doc-Kommentare und drei Rümpfe; im Prüfmodul steht
keine einzige.

**Der abgetrennte Rest ist richtig beschrieben.** Die Messung in
`260812-2140_o_…` ist exakt nachvollzogen:

```
"- Text\n\n  [ZIEL]:\n      http://z.example\n"  ->  "• Text\n\n[ZIEL]:\nhttp://z.example"
```

Er beschreibt aber nur die Hälfte dessen, was noch fehlt: er grenzt sich
ausdrücklich auf „innerhalb eines Elements" ein, und auf Dokumentebene ist mit
diesem Turn die gegenläufige Ungleichheit entstanden (Befund 2).

**Beide datierten Nachträge stimmen.** Alle vier Ausgaben, die der Nachtrag in
`260812-1920_c_die-deckungszusage-…` als unverändert bezeichnet, sind am Baum
nachgemessen und unverändert:

```
"- [ref]: http://a.example\n"             -> "- [ref]: http://a.example\n"
"> Zitat\n>\n> [ref]: http://a.example\n" -> "Zitat\n\n[ref]: http://a.example"
"- [ref]: http://a.example\n\n  Text\n"   -> "• Text"
"> [ref]: http://a.example\n>\n> Zitat\n" -> "Zitat"
```

Ebenso die drei Formen aus
`260812-2019_c_das-merkzeichen-eines-aeusseren-punktes-…`: `"- -\n"` gibt
jetzt `"• • "`, `"- - [ZIEL]: …"` und `"- >\n"` sind unverändert. Die Grenze,
die die Nachträge nennen — mechanisch nur dort, wo hinter dem Merkzeichen
nichts steht — stimmt mit dem Baum überein.

## Befunde

### Mittel

**1. `merkzeichen_einloesen` kostet bei tiefer Verschachtelung das
Zweieinhalbfache, und L7 wird jetzt schon bei 12 kB verfehlt.**
`markdown.rs:679-695`. Datensatz
`issues/260812-2133_o_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-das-zweieinhalbfache-und-verfehlt-l7-frueher.md`.

Der innere Nachzug läuft je eingelöstem Merkzeichen über den ganzen Rest des
Stapels; bei einer tief verschachtelten Liste ist der Rest die Tiefe selbst.
Median aus 15 Läufen, Profil `release`, Quelle `"- "` mal Tiefe:

| Tiefe | Quelle | `1e4e01f` | `2c0b2a6` |
|---|---|---|---|
| 6 000 | 12 kB | 38,0 ms | **95,5 ms** |
| 8 000 | 16 kB | 69,1 ms | **163,8 ms** |
| 10 000 | 20 kB | **113,4 ms** | **253,7 ms** |
| 20 000 | 40 kB | 460,9 ms | 1 075,5 ms |

Die 100 ms von L7 werden jetzt ab rund 6 100 Ebenen verfehlt statt ab rund
9 500. Die Komplexitätsklasse ändert sich nicht — `tiefe()` und `absetzen()`
laufen ohnehin je Ereignis über `offen` —, wohl aber der Faktor. Flache Listen
und die 1,05-MiB-Musterquelle sind in der neuen Fassung schneller; der
schlechteste Fall ist in die andere Richtung gegangen und war ungemessen.

### Niedrig

**2. Auf Dokumentebene verliert die erste Zeile einer Lücke ihren Einzug, jede
folgende behält ihn.** `markdown.rs:758-762`. Datensatz
`issues/260812-2134_o_auf-dokumentebene-verliert-die-erste-zeile-einer-luecke-ihren-einzug-und-jede-folgende-behaelt-ihn.md`.

```
Quelle : "  [a]: http://a.example\n  [b]: http://b.example\n"
1e4e01f: "[a]: http://a.example\n[b]: http://b.example"
2c0b2a6: "[a]: http://a.example\n  [b]: http://b.example"
```

`trim()` schneidet an den Enden der ganzen Lücke und nicht je Zeile. Zwei
Zeilen mit demselben Einzug kommen verschieden heraus, und der Einzug, den die
zweite behält, ist einer, den CommonMark nicht trägt. Neu mit diesem Turn und
von 260812-2140 nicht abgedeckt, weil der sich auf „innerhalb eines Elements"
begrenzt.

**3. Der Doc-Kommentar von `luecke_bis` sagt weiter, der Leerraum falle
zeilenweise weg.** `markdown.rs:727-731` gegen `:754-762`. Datensatz
`issues/260812-2135_o_der-doc-kommentar-von-luecke-bis-sagt-weiter-der-leerraum-falle-zeilenweise-weg.md`.

Modulkopf und `ohne_umgebungszeichen` sind mit demselben Commit nachgezogen,
diese eine Stelle nicht. Doc-Kommentar und Rumpfkommentar derselben Funktion
sagen jetzt Verschiedenes.

**4. Die allgemeine Probe erkennt ein Merkzeichen am Text und schlägt bei neun
Dateien dieses Baums falsch an.** `markdown.rs:1374-1383`. Datensatz
`issues/260812-2136_o_die-allgemeine-probe-erkennt-ein-merkzeichen-am-text-und-schlaegt-bei-neun-dateien-dieses-baums-falsch-an.md`.

`beginnt_mit_merkzeichen` fragt den Text und nicht die Stelle, an der ein
Merkzeichen geschrieben wurde. Eine Quelle mit wörtlichem `` `• ` `` löst ihn
aus; neun der 773 Markdown-Dateien dieses Baums tun das. Der Gurt hält für
alles, was er heute misst, ist aber nicht mit beliebigen Quellen zu weiten —
und genau das verspricht sein Doc-Kommentar. Zwei Dinge sieht er zusätzlich
nicht: ein Merkzeichen mitten in einem Bereich (heute strukturell unmöglich,
aber nirgends gesagt) und den umgekehrten Fehler, dass eine `Listenzeile` ihr
eigenes Merkzeichen verliert.

## Die Geschwindigkeitszahl des Turns

Der Turn berichtet 20,9 ms auf 1,05 MiB gegen 23,0 ms vor dem Turn. Das ist
mit den Prüfprogrammen des Turns reproduzierbar (21,4 bis 22,6 ms gegen 23,0
bis 28,0 ms in vier Wiederholungen) und im eigenen Prüfprogramm dieser
Durchsicht der Richtung nach bestätigt: Median aus 15 Läufen, 29,3 ms für
`1e4e01f` gegen 25,9 ms für `2c0b2a6`. Das Band „19 bis 30 ms" im Modulkopf
hält in jeder Messung.

**Ein Vorbehalt zur Zahl selbst, kein Befund:** die 23,0 ms für den Stand vor
dem Turn sind das Beste aus fünf Läufen einer Verteilung, deren
Wiederholungen bis 28,0 ms reichen. Der berichtete Gewinn von 2,1 ms liegt
innerhalb der Streuung jener einen Messung; bestätigt ist er erst durch den
Median über fünfzehn Läufe. Die Richtung stimmt, die Genauigkeit der Zahl
nicht.

Auf einer 1,05-MiB-Quelle aus echten Dateien dieses Baums kostet der Durchgang
6,4 ms — die Musterquelle des Turns ist rund viermal dichter an Auszeichnungen
als der Baum selbst und damit die vorsichtigere Messung.

## Querschnitt

**Alle vier Befunde sind Buchführung oder Randfälle, keiner ist der
Hauptbefund** — das ist der Unterschied zu den Durchsichten von Turn 3 und
Turn 4, die je eine Verschlechterung im Ausgabetext gefunden haben. Der
Ausgabetext ist über alle 773 Dateien des Baums in beiden Fassungen
zeichengleich; über Zufallsquellen unterscheidet er sich nur dort, wo der
leere Listenpunkt jetzt sein gerendertes Merkzeichen zeigt, und jede der 283
gefundenen Unterschiedsformen führt auf diese eine Änderung zurück.

**Zwei der vier Befunde stammen aus derselben Wurzel:** die Frage „wie viel
Leerraum wiederholt die Umgebung" ist auf beiden Seiten der Grenze
`self.offen.is_empty()` nur genähert — innerhalb eines Elements zu großzügig
(der abgetrennte Rest 260812-2140), auf Dokumentebene uneinheitlich zwischen
erster und folgender Zeile (Befund 2). Der Zuschnitt, den 260812-2140 für die
eine Seite erwägt — der kürzeste führende Lauf über alle nichtleeren Zeilen —
löst beide Seiten mit einer Regel. Wer eines von beiden anfasst, sollte das
andere mitnehmen, sonst entsteht die dritte Näherung.

**Der Turn hat gemessen, was er geändert hat, und nicht, wogegen er es
gemessen hat.** Die Geschwindigkeitsmessung variiert die Länge einer Quelle
und nicht ihre Tiefe; die Änderung liegt aber in einer Schleife über die
Tiefe. Das ist der Mechanismus hinter Befund 1 und die einzige Stelle dieser
Durchsicht, an der die Prüfstrategie des Turns eine Lücke hat.

## Empfohlene Reihenfolge

1. **Befund 3** (Doc-Kommentar) — eine Zeile, kein Verhalten, und es ist genau
   der Befundtyp, den dieser Turn sonst ausgeräumt hat.
2. **Befund 1** (Geschwindigkeit bei Tiefe) — die einzige Stelle mit einer
   verfehlten Zusage. Der Zuschnitt ist eine mitgeführte Indexvariable statt
   eines inneren Durchlaufs.
3. **Befund 2 zusammen mit dem offenen Rest 260812-2140** — eine Regel für
   beide Seiten, nicht zwei.
4. **Befund 4** (Gurt) — zuletzt, weil der Gurt heute hält und der Zuschnitt
   die Auslieferung berührt.

Kein Befund hält eine Freigabe auf; Befund 1 betrifft eine Zusage, die schon
vor diesem Turn nur mit pathologischen Quellen zu brechen war.

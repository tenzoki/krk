# Durchsicht Turn 4 der Runde 6: das Merkzeichen am Text und die Deckung im Container

**Sender:** coderev
**Reviewed-range:** `f401dcc..c35f8b1`
**Not-opened:** none
**Date:** 2026-08-12
**Circle:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`
**Commits:** `c6bf13d` (reine Werkbank), `c35f8b1` (eine Codedatei)
**Datei im Umfang:** `crates/krk-ui/src/markdown.rs` (+370/−32)

---

## Zusammenfassung

Der Umbau tut, was er zusagt: die drei gemessenen Fälle der losen Liste und die
zwei der Deckungszusage sind behoben, jeder mit einer eigenen Probe, und keine
vorhandene Probe musste nachgezogen werden. Er hat dabei **eine Verschlechterung
eingeschleppt, und sie ist häufig**: beginnt ein Listenpunkt unmittelbar mit
einer Auszeichnung, deckt deren Bereich das Merkzeichen mit ab, und der
Aufzählungspunkt wird fett, kursiv, festbreit oder eingefärbt. Im Baum dieses
Projekts trifft das 3721 Zeilen in 507 Dateien.

Sechs Defekte abgelegt: einer hoch, zwei mittel, drei niedrig. Der Bereich ist
gegen die Vorfassung gemessen worden, nicht gelesen — beide Fassungen von
`markdown::rendern` unverändert in ein Prüfprogramm kopiert, gegeneinander
gefahren, mit 60 000 zufällig zusammengesetzten Quellen und mit gezielten
Proben belastet.

## Zahlen

| | |
|---|---|
| Defekte abgelegt | 6 (hoch 1, mittel 2, niedrig 3) |
| Proben im Binärziel `krk` | 466, vorher 457 — nachgezählt, stimmt |
| Vorhandene Proben abgeschwächt | keine — im Prüfmodul steht keine einzige gelöschte Zeile |
| Gemessene Fälle der zwei geschlossenen Datensätze | 5 von 5 als Probe vorhanden |
| `cargo build` / `fmt` / `clippy -D warnings` / `test` | alle vier Exit 0, am Baum nachgefahren |

## Wie gemessen wurde

`markdown::rendern` aus `f401dcc` und aus `c35f8b1` je unverändert in dasselbe
Prüfprogramm kopiert, die Typen aus `crate::hervorhebung` durch einen Stub
ersetzt (nur `linkfarbe` liefert statt `syntect` eine feste Farbe; alle übrigen
Typen sind wörtlich übernommen), `pulldown-cmark 0.13.4` wie im Baum ohne
Vorgabemerkmale. Gefahren wurde:

1. **60 000 zufällig zusammengesetzte Quellen** aus 41 Bausteinen, beide
   Fassungen nebeneinander. Geprüft auf Programmabbruch, auf den Gürtel (jede
   Stelle innerhalb der gemeldeten Länge, Länge gleich UTF-16-Länge des
   Textes), auf ein allein stehendes Merkzeichen und auf Unterschiede.
   Ergebnis: 0 Abbrüche, 0 Gürtelbrüche, 0 allein stehende Merkzeichen,
   7522 Unterschiede.
2. **40 000 Quellen mit einer eindeutigen Marke** in einer Verweisdefinition an
   wechselnder Stelle, um Doppelausgabe und Inhaltsverlust zu messen. Ergebnis:
   **0 doppelt** und **0 in `c35f8b1` verloren, was `f401dcc` noch zeigte**.
   Die Frage „kann beim Schließen etwas doppelt herauskommen" ist damit
   gemessen verneint, nicht nur gelesen.
3. **Gezielte Proben** zu den fünf Punkten des Auftrags, einzeln unten.
4. **Pathologische Quellen**: Verschachtelungstiefe 300 in Liste und Zitat,
   2000 leere Punkte, 2000 nackte `>`, CRLF, Emoji, Umlaut, `u64::MAX` als
   Listennummer, Tabulator-Einzug. Kein Abbruch, kein Gürtelbruch.

## Befunde nach Thema

### Das Merkzeichen und die Bereiche der Auszeichnungen

**1. Das Merkzeichen liegt im Bereich des ersten Kindes — hoch.**
`260812-2019_o_das-merkzeichen-liegt-im-bereich-des-ersten-kindes-und-wird-fett-kursiv-fest-oder-eingefaerbt.md`

`- **fett**` liefert `StarkeBetonung` über `"• fett"` statt über `"fett"`.
Dasselbe für `*kursiv*`, `` `code` ``, `[Verweis](…)`, `# Titel` und den
Quelltextblock; bei `- - **fett**` über beide Merkzeichen. Ursache:
`Zerlegung::oeffnen` (`markdown.rs:696`) setzt `Offen::anfang` auf `self.stelle`,
und `merkzeichen_einloesen` (`markdown.rs:600-616`) erhöht `self.stelle` später,
**ohne einen `Offen::anfang` nachzuziehen** — anders als `absetzen`
(`markdown.rs:572-576`), das genau das für die Umbrüche tut. In AppKit ist die
Wirkung voll: `textmerkmale.rs:206-217` setzt für vier der fünf Auszeichnungen
eine Schrift über den Bereich, für den Verweis Farbe und Unterstreichung.

**2. Das Merkzeichen eines äußeren Punktes wird vom wörtlichen Quelltext eines
inneren eingelöst — mittel/niedrig.**
`260812-2019_o_das-merkzeichen-eines-aeusseren-punktes-wird-vom-woertlichen-quelltext-eines-inneren-eingeloest.md`

`"- - [ref]: http://z\n"` ergibt `"• - [ref]: http://z\n"`, also gerendertes
`• ` neben rohem `- `. Der Doc-Kommentar an `schliessen` (`markdown.rs:785-787`)
sagt ausdrücklich zu, dass genau diese Form nicht entsteht. Sie tut es für die
**Vorfahren** des schließenden Punktes, denn `woertlich` ruft `schreiben`, und
das löst alle noch offenen Wünsche ein.

**3. Ein leerer Listenpunkt zeigt sein rohes `- ` — mittel.**
`260812-2019_o_ein-leerer-listenpunkt-zeigt-sein-rohes-bindestrich-zeichen-und-verliert-seinen-einzug.md`

`"- eins\n- \n"` ergibt `"• eins\n- \n"`: eine Liste, zwei verschiedene
Merkzeichen. Dazu kommt der Zeilenumbruch der Quelle, der die Abstandsrechnung
aus `absetzen` umgeht, und die fehlende `Listenzeile`, weil der wörtliche Zweig
in `schliessen` im `else` zu `laenge > 0` liegt und dort keine
`Auszeichnungsstelle` eingetragen wird. Der Modulkopf benennt das Verhalten
(`markdown.rs:103-105`) — der Befund ist, dass der **leere** Punkt von keiner
Probe gehalten wird, die vorhandene misst den Punkt mit Verweisdefinition.

### Die Deckungsgrenze

**Die Zusage „alles außer dem Vorspann eines Containers" hält.** Zwölf
gezielte Stellungen einer Verweisdefinition gemessen — Dokumentebene, im Punkt
davor und dahinter, im Zitat davor und dahinter, zwischen zwei Punkten, vor dem
ersten Punkt, zwei Ebenen tief in beiden Lagen, Zitat im Zitat, Punkt im Zitat,
hinter dem letzten Punkt. Verloren gehen genau die Vorspann-Fälle. Die
40 000-Marken-Reihe hat keinen weiteren gefunden.

**Eine Lücke außerhalb des Vorspanns ist nicht gefunden worden**, und das ist
ein Befund im Sinne des Auftrags: die Zusage reicht diesmal nicht weiter als
der Baum. Die Frage, ob C4.3 nachgezogen wird, steht als
`decisions/260812-2002_o_bleibt-der-vorspann-eines-containers-…` und wird hier
nicht wiederholt.

**4. `ohne_umgebungszeichen` läuft auch auf Dokumentebene — niedrig.**
`260812-2019_o_ohne-umgebungszeichen-laeuft-auch-auf-dokumentebene-und-nimmt-dort-einzug-weg-der-inhalt-ist.md`

Die Funktion nimmt zeilenweise Leerraum und `>` weg. `luecke_bis` ruft sie
unbedingt, also auch bei leerem `offen`, wo keine Umgebung etwas wiederholt.
Eine mehrzeilige Verweisdefinition verliert dort den Einzug ihrer
Fortsetzungszeile. Ein `>` **mitten** in einer Zeile bleibt stehen (gemessen);
ein Zitat im Zitat und ein Quelltextblock im Zitat kommen richtig heraus
(gemessen).

### Die Aufzählung `Inhaltsart`

**Die Zuordnung ist vollständig und richtig.** `Bloecke` für Zitatblock, Liste
und Listenpunkt, `Zeichen` für Absatz, Überschrift, Quelltextblock, Betonung,
Verweis und den Quelltext in der Zeile. Das sind genau die drei Containerblöcke
von CommonMark gegen die Blattblöcke. Alles Übrige geht über
`Behandlung::Woertlich` (`markdown.rs:355`) und wird gar nicht erst geöffnet,
bekommt also keine `Inhaltsart` — die Zuordnung hat damit kein Loch.

**5. Sie wird nur über `matches!` gelesen — niedrig.**
`260812-2019_o_die-aufzaehlung-inhaltsart-wird-nur-ueber-matches-gelesen-und-haelt-den-bau-nicht-an.md`

Die Aufzählung hat keinen Auffangzweig, aber ihre beiden Lesestellen
(`markdown.rs:658` und `:796`) sind `matches!`, und ein `matches!` trägt einen
stillen `_ => false`. Eine dritte Variante hielte den Bau nicht an, sondern
liefe still als „nicht gedeckt". Das ist der Mechanismus, den `CLAUDE.md` unter
„Was man nicht sieht" für die vier gewachsenen Aufzählungen beschreibt und den
`Auszeichnung` in `hervorhebung.rs:279-281` ausdrücklich zusagt.

### Buchführung

**6. Der geschlossene Deckungs-Datensatz nennt die Aufzählung `Inhalt` —
niedrig.**
`260812-2019_o_der-geschlossene-deckungs-datensatz-nennt-die-aufzaehlung-inhalt-sie-heisst-inhaltsart.md`

Sechs Stellen in der Abschlussnotiz von
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-…`. Im
Baum heißt sie `Inhaltsart`, und `vorschaumodell::Inhalt` gibt es wirklich.

**`c6bf13d` ist geprüft und in Ordnung.** Die drei Zeigerstellen tragen die
Sternform, die zwei Stellen, an denen der Marker die Aussage ist, bleiben
ausgeschrieben, und die drei geschlossenen Datensätze verweisen aufeinander.
Der `find`-Lauf über offene Punkte liefert keine vertagte Arbeit mehr.

## Was nachgeprüft und für gut befunden wurde

- **Das Merkzeichen steht in jeder Liste unmittelbar vor seinem Zeichen** —
  lose, straff, verschachtelt, geordnet, im Zitat, Punkt im Zitat, drei Ebenen
  lose. In 20 000 Zufallsquellen kein einziges allein stehendes Merkzeichen.
  Der Punkt ohne Text ist die Ausnahme und Gegenstand von Befund 3.
- **Keine Doppelausgabe** aus `schliessen`, in 40 000 markierten Quellen
  gemessen.
- **Kein Inhaltsverlust gegenüber der Vorfassung** in derselben Reihe.
- **Keine vorhandene Probe ist abgeschwächt worden.** Das Prüfmodul trägt im
  Bereich ausschließlich hinzugefügte Zeilen; die Gegenprobe
  `die_zeichen_eines_gerenderten_elements_bleiben_weg` läuft unverändert.
- **Jeder gemessene Fall der zwei geschlossenen Datensätze** hat eine Probe:
  drei aus dem Merkzeichen-Datensatz, zwei aus dem Deckungs-Datensatz, dazu die
  Vorspann-Grenze und die `>`-Gegenprobe.
- **Robustheit**: Tiefe 300, 2000 leere Elemente, CRLF, Emoji, `u64::MAX` als
  Nummer — kein Abbruch, Gürtel hält überall.

## Beobachtung ohne Datensatz: die Laufzeit

Der Durchgang ist gegenüber `f401dcc` um rund 17 Prozent langsamer geworden:
auf einer 1,05-MB-Quelle 24,8 ms gegen 29,0 ms (beste von fünf Läufen, Profil
release, dieselbe Maschine, derselbe Stub). Die Ursache liegt nahe: `luecke_bis`
kehrt nicht mehr sofort zurück, sobald ein Element offen ist, und
`ohne_umgebungszeichen` legt für jede Lücke einen `Vec<&str>` und einen
`String` an, wo vorher ein `trim()` ohne Allokation stand.

**Das ist kein Defekt:** der Modulkopf nennt „19 bis 30 ms fuer 1,05 MB"
(`markdown.rs:176-177`), und 29,0 liegt darin. Der Abstand zur oberen Kante ist
aber auf etwa eine Millisekunde geschrumpft, und die Zahl im Modulkopf stammt
aus einer Messung vor diesem Umbau. Wer die Vorschau das nächste Mal anfasst,
sollte sie nachmessen, bevor er sie zitiert.

## Empfohlene Reihenfolge

1. **Befund 1** (das Merkzeichen im Bereich des Kindes) vor dem Rundenschluss.
   Er trifft die häufigste Listenzeile überhaupt, er ist eine Verschlechterung
   dieses Turns, und er ist an einer Stelle zu beheben.
2. **Befund 3** (der leere Punkt) und **Befund 2** (`• - `) zusammen — beide
   hängen am wörtlichen Zweig in `schliessen`.
3. **Befund 5** (`matches!`) und **Befund 6** (`Inhalt`/`Inhaltsart`) sind
   Aufräumarbeit und halten nichts auf.
4. **Befund 4** (`ohne_umgebungszeichen` auf Dokumentebene) ist entweder eine
   Zeile im Doc-Kommentar oder eine Fallunterscheidung, die der Modulkopf
   ausdrücklich vermeiden will. Er gehört zur Frage aus
   `decisions/260812-2002_o_…` mit auf den Tisch, ohne in sie hineingeschrieben
   zu werden.

## Deckung des Bereichs

`f401dcc` selbst ist von keiner Durchsicht erfasst: Turn 3 endete bei
`df4ec00`, dieser Bereich beginnt bei `f401dcc` und schließt ihn damit aus. Der
Commit trägt allein die Turn-3-Durchsicht, also keinen Code — eine Durchsicht
der eigenen Durchsicht ist keine, die etwas fände. Festgehalten ist es hier,
damit der nächste Deckungslauf die Lücke nicht als Versäumnis liest.

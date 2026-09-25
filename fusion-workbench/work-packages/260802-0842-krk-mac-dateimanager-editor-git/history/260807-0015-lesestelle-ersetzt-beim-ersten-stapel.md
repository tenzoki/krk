# Die Lesestelle ersetzt beim ersten Stapel, statt vorab zu leeren

**Agent:** coder
**Aufgabe:** D6 aus Turn 25
**Status:** Complete

---

## Auftrag

Der Nutzerentscheid vom 260806-2345 zum Defekt
`issues/260806-1445_*_ein-schnelles-verschieben-koennte-dieselbe-meldelawine-ausloesen-wie-das-stapel-umbenennen.md`:
nicht den Vorbehalt messen, sondern die Ursache angehen. Ein Lesevorgang soll
sein Ordnermodell erst mit dem ersten gelieferten Stapel ersetzen, statt es
vorab zu leeren. `schiebt_auffrischung_auf` bleibt unveraendert; ob der Aufschub
danach entfallen kann, ist zu pruefen und zu berichten, nicht zu aendern.

## Was umgesetzt ist

**Der Kern.** `Ordnermodell::leeren` ist entfallen. An seiner Stelle steht
`lesevorgang_beginnen(generation)`: es setzt die Generation und merkt den Ersatz
des Bestands nur vor. Eingeloest wird er von `anhaengen` mit dem ersten Stapel
oder, wenn der Ordner keinen liefert, von `abschliessen`. Ein neues Feld
`ersatz_ausstehend` traegt den Zustand, `ersatz_einloesen` die eine Stelle, an
der Eintraege, Sicht, Markierung und Auswahl fallen. Dazu die Abfrage
`ersetzt_beim_naechsten_stapel`, an der die Ansicht ablesen kann, dass sie die
Tabelle neu holen muss.

**Die Oberflaeche.** `Tabliste::lesen_starten` ruft `lesevorgang_beginnen` statt
`leeren`. `Tabliste::aktiven_neu_lesen` tauscht den Tab nicht mehr gegen einen
frischen aus — das war der zweite Ort, an dem der Bestand vorab fiel —, sondern
setzt allein Auswahlname und offene Bildlaufposition zurueck. `Einzug` bekommt
das Feld `ersetzt`; `DateifensterQuelle::einziehen` laesst darauf die Tabelle neu
holen statt nur eine neue Zeilenzahl zu melden.

## Die vier Pruefpunkte des Auftrags

1. **Auswahl und Markierung** fallen mit dem Ersatz, also mit dem ersten Stapel
   oder mit dem Abschluss, nicht mehr beim Start des Lesevorgangs. Solange die
   alten Zeilen stehen, zeigen beide auf genau die Eintraege, die der Nutzer
   gerade sieht. Auf einen Eintrag des neuen Ordners springen kann keiner von
   beiden, weil `ersatz_einloesen` sie in derselben Anweisung wegwirft, in der
   die alten Eintraege fallen.
2. **Ein leerer oder unlesbarer Ordner** liefert keinen Stapel. Der Auffangfall
   ist `abschliessen`: der Leser meldet seinen Abschluss in jedem Ausgang
   (vollstaendig, abgebrochen, gescheitert), und `einzug_je_tab` ruft
   `abschliessen` auf jede `Meldung::Fertig`. Es gibt keinen Ausgang, der den
   Ersatz schuldig bleibt.
3. **Ein abgebrochener Lesevorgang** hinterlaesst keinen Mischzustand. Getragen
   wird das nicht von `gehoert_dazu`, sondern davon, dass `lesen_starten` den
   alten `Lesevorgang` fallen laesst; damit faellt sein Empfaenger, und ein
   Stapel des alten Laufs kann gar nicht mehr ankommen. `gehoert_dazu` traegt
   unveraendert weiter, was es traegt: es beantwortet "gehoert dieser Stapel zum
   laufenden Lesevorgang", und der einzige Aufrufer in `krk-bench` prueft genau
   das.
4. **L2** wird nicht spaeter erreicht. Der erste Stapel steht unveraendert
   sofort in der Sicht; die Umstellung verschiebt allein das Freigeben der alten
   Eintraege vom Start des Lesevorgangs zum ersten Stapel, und beide Zeitpunkte
   liegen innerhalb derselben Spanne, die L2 misst.

## Die Frage nach dem Aufschub

Der Aufschub kann **nicht** entfallen. Die leere Liste kaeme ohne ihn nicht
zurueck, aber eine andere Fehlfunktion: FSEvents sammelt 300 ms, ein
vollstaendiger Lesevorgang braucht 492 ms fuer 100.000 Eintraege. Ueber der
daraus folgenden Schwelle setzte jede Meldung den Lesevorgang neu auf, bevor er
fertig ist, und die Liste zeigte fuer die ganze Laufzeit nur den Anfang des
Ordners in Lesereihenfolge, also unsortiert und unvollstaendig.

Die Frage liegt als Datensatz beim Nutzer:
`decisions/260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`.
`schiebt_auffrischung_auf` ist unveraendert; geaendert ist allein der Kommentar
an der Funktion, der die Kante jetzt als geschlossen beschreibt.

## Nachmessung

`messungen/260807-0002-MacBookPro15-1-ersatz-beim-ersten-stapel-l2-l3-l10.txt`.
Fuenf Runden zu zwanzig Wiederholungen, abwechselnd gegen eine Basisreihe aus
`2fbab30`. L2, L3 und L10 halten in jeder Runde; die Abstaende zwischen den
Reihen liegen unter der Streuung der Basisreihe selbst und zeigen in beide
Richtungen.

## Geaenderte Dateien

- `crates/krk-core/src/verzeichnis/modell.rs` — Modulkopf, Feld
  `ersatz_ausstehend`, `lesevorgang_beginnen`, `ersetzt_beim_naechsten_stapel`,
  `ersatz_einloesen`, `anhaengen`, `abschliessen`, acht Proben
- `crates/krk-core/src/verzeichnis/leser.rs` — Modulkopf, ein Verweis
- `crates/krk-ui/src/tabs.rs` — `Einzug::ersetzt`, `aktiven_neu_lesen`,
  `lesen_starten`, `einzug_je_tab`, eine Probe
- `crates/krk-ui/src/appkit/tabelle.rs` — Modulkopf, `neu_lesen`,
  `nach_lesebeginn`, `einziehen`
- `crates/krk-ui/src/auffrischung.rs` — allein der Kommentar an
  `schiebt_auffrischung_auf`
- `crates/krk-ui/src/kommandos/operationen.rs` — ein ueberfluessiger
  `leeren`-Aufruf in einer Probenhilfe entfaellt

## Abnahme

`make check` gruen: Bau, 510 Pruefungen, Formatierung, clippy ohne Warnung.

## Was daneben auffiel

- Die Generationsnummer wird an drei Stellen erklaert: im Modulkopf von
  `leser.rs` (warum es sie gibt), an `Ordnermodell::generation` (was sie sagt)
  und im Modulkopf von `appkit/tabelle.rs` (was einen Ordnerwechsel mitten im
  Lesen stattdessen traegt). Die drei sagen Verschiedenes und widersprechen sich
  nicht; alle drei sind mit dieser Aenderung nachgezogen. Kein Defekt, sondern
  eine Notiz fuer den naechsten, der eine davon anfasst: es sind drei.
- Eine Auffrischung wirft die Markierung des Nutzers weg. Das war vorher so und
  ist es weiter, nur faellt sie jetzt spaeter. Die Auswahl ueberlebt ueber ihren
  Namen, die Markierung hat keinen solchen Weg. Als Frage abgelegt unter
  `decisions/260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md`.

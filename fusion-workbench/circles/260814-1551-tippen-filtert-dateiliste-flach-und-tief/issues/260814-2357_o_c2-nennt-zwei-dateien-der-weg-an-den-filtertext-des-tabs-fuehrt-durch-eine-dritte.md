# C2 nennt zwei Dateien, der Weg an den Filtertext des Tabs führt durch eine dritte

**Status:** Open
**Domain:** Plan der Filter-Runde, Strang C
**Filed by:** coder, beim Umsetzen von C2
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt C2; `issues/260814-2303_o_e1-und-e3-nennen-drei-dateien-der-weg-an-das-tabmodell-fuehrt-durch-eine-vierte.md` (derselbe Befund an Strang E)

## Befund

Schritt C2 nennt `crates/krk-ui/src/appkit/ereignisse.rs` und
`crates/krk-ui/src/appkit/anwendung.rs` und schreibt vor, der Zweig rufe
`rueckschritt` "mit ‚steht ein Filtertext‘ aus dem Modell des sichtbaren Tabs des
aktiven Dateifensters" und lasse `ZeichenZurueck` dann `letztes_zeichen_weg` rufen.
Aus den zwei genannten Dateien ist dieses Modell nicht erreichbar.

Es ist derselbe Zugriffsweg, den `issues/260814-2303_o_e1-und-e3-nennen-drei-dateien-…`
für Strang E schon beschreibt: er läuft über `self.dateifenster(seite).quelle()`, also
über `DateifensterQuelle` aus `crates/krk-ui/src/appkit/tabelle.rs`, deren Tabliste im
modulprivaten Ivar `QuelleIvars::tabs` steht. Am 260814-2357 trägt `DateifensterQuelle`
weder einen Leser für den Filtertext noch einen Setzer, der sein letztes Zeichen
zurücknimmt: Schritt B1, der `sprungmarke_tippen` zu `filterzeichen_tippen` umbaut, ist
noch nicht gefahren.

## Was beim Umsetzen von C2 daraus geworden ist

Zwei öffentliche Methoden an `DateifensterQuelle`, unmittelbar neben
`tiefe_suche_umschalten` aus E1 und in derselben Bauart:

- `filter_steht() -> bool` — liest `Ordnermodell::filter_steht` am sichtbaren Tab.
- `letztes_filterzeichen_weg()` — ruft `Ordnermodell::letztes_zeichen_weg`, verbraucht
  dessen `#[must_use]`-Wert für die Entscheidung, ob die Anzeige nachzuziehen ist, und
  ruft dafür `umsortiert`, wie `tiefe_suche_umschalten` es tut.

Damit weicht der Schritt von seiner Dateiliste ab, und die Abweichung steht hier, statt
still zu bleiben.

## Warum das kein Streit mit Strang B ist

Schritt B1 baut in derselben Datei `sprungmarke_tippen` zu `filterzeichen_tippen` um und
entfernt den Ivar `sprungmarke`. Die beiden neuen Methoden berühren weder das eine noch
das andere. Sie rufen `umsortiert`, das B1 ohnehin anfasst; B1 wird dessen
Auswahlnachzug für C1.11 erweitern, und die beiden Methoden erben ihn dann, ohne selbst
eine Zeile zu brauchen.

## Vorschlag

Die Dateiliste von C2 um `crates/krk-ui/src/appkit/tabelle.rs` ergänzen. Der Befund fällt
mit dem von E1 und E3 zusammen: **drei Schritte in zwei Strängen nennen ihre Dateien ohne
die Datei, durch die der Weg an das Tabmodell führt.** Eine gemeinsame Berichtigung ist
billiger als drei einzelne.

## Nachtrag vom 260815, beim Umsetzen von B1

**Derselbe Befund ein viertes Mal, und diesmal in schwächerer Form.** Schritt B1 nennt
unter `Files:` `crates/krk-ui/src/appkit/tabelle.rs` und
`crates/krk-ui/src/appkit/anwendung.rs`. Geändert sind drei Dateien: die dritte ist
`crates/krk-ui/src/kommandos/navigation.rs`, in die die reine Funktion `ersatzzeile` für
C1.11 gezogen wurde.

**Diese dritte Datei ist keine Entdeckung des Umsetzers.** Der letzte Aufzählungspunkt
unter B1 `Changes:` schreibt sie ausdrücklich vor: „die Rechnung ‚welche Zeile bekommt
die Auswahl, wenn ihre weggefallen ist‘ wird dafür als reine Funktion nach
`crate::kommandos::navigation` gezogen, wo `zielzeile` schon steht". Die Zeile `Files:`
und der Abschnitt `Changes:` desselben Schritts widersprechen sich also, und der Fehler
sitzt allein in der Zeile.

Damit trennt sich der Befund in zwei Sorten, und eine gemeinsame Berichtigung braucht
beide:

- **C2, E1, E3** — die Datei steht in keinem Teil des Schritts, und der Weg an das
  Tabmodell führt trotzdem durch sie. Der Schritt ist ohne sie nicht ausführbar.
- **B1** — die Datei steht im Fließtext des Schritts und fehlt in seiner Dateiliste. Ein
  Leser, der nur `Files:` liest, hält die Abweichung für eine Eigenmächtigkeit des
  Umsetzers.

**Was `Files:` in diesem Plan misst, ist damit nicht verlässlich.** Wer die Zeile als
Umfangsgrenze liest — für eine Zuständigkeitsprüfung, für eine Abgleichszählung —,
bekommt bei vier der vierzehn Schritte eine zu kurze Antwort.

## Nachtrag vom 260815, beim Umsetzen von D1

**Ein fünfter Fall, und er hat eine dritte Ursache.** Schritt D1 nennt unter `Files:`
`crates/krk-ui/src/appkit/statuszeile.rs`, `crates/krk-ui/src/appkit/tabelle.rs` und
`crates/krk-ui/src/appkit/anwendung.rs`. Geändert sind fünf Dateien; die zwei weiteren
sind `crates/krk-ui/src/kommandos/auswahl.rs` und `crates/krk-ui/src/appkit/editor.rs`,
und in beiden ist es **eine Zeile Modulkopf beziehungsweise Doc-Kommentar**.

Der Grund liegt nicht am Zugriffsweg wie bei C2, E1 und E3 und nicht an einem
widersprüchlichen Schritt wie bei B1, sondern an der Rangstelle selbst. D1 setzt den
neuen Rang **zwischen** Tabmeldung und Markierungsstand (C4.1). Damit rückt der
Markierungsstand vom fünften auf den sechsten Rang, und jede Stelle im Baum, die ihn
beim alten Namen nennt, wird in demselben Zug falsch:

- `kommandos/auswahl.rs:31` — „Der fünfte Rang der Statuszeile: was im sichtbaren Tab
  markiert ist (C2)" über `markierungsstand_text`, der reinen Funktion für genau diesen
  Rang.
- `appkit/editor.rs:171` — „auf den obersten ihrer fünf Ränge"; eine Zählung, keine
  Rangnummer.

**Das ist keine Abweichung vom Schritt, sondern seine Folge.** Wer einen Rang in die
Mitte einer Rangfolge einfügt, verschiebt die Nummern aller darunter, und die Nummern
stehen in diesem Baum in Prosa und nicht in einer Aufzählung, die der Übersetzer
nachzieht. Innerhalb der drei genannten Dateien waren es weitere elf Stellen
(„fünf Ränge", „zehn Bewerber", „zehn Quellen", „zehn Aussagen"), alle mitgezogen.

## Was der Befund inzwischen misst

Fünf der vierzehn Schritte, und drei verschiedene Ursachen:

- **C2, E1, E3** — die Datei steht in keinem Teil des Schritts; der Weg an das Tabmodell
  führt trotzdem durch sie. Der Schritt ist ohne sie nicht ausführbar.
- **B1** — die Datei steht im Fließtext des Schritts und fehlt in seiner Dateiliste.
- **D1** — die Dateien sind vollständig für die *Änderung*, aber nicht für ihre *Folge*:
  eine Rangverschiebung entwertet Prosa außerhalb der geänderten Dateien.

Für die dritte Ursache hilft eine Ergänzung der Dateilisten nur begrenzt weiter. Eine
Rangnummer in Prosa ist im Baum von keiner Prüfung gehalten; sie veraltet auf demselben
Weg, auf dem die Zahlen in `CLAUDE.md` viermal in vier Tagen veraltet sind. Wer die
Berichtigung fährt, sollte dabei entscheiden, ob eine Dateiliste diesen Fall überhaupt
tragen soll oder ob die Nummern aus der Prosa verschwinden.

## Nachtrag vom 260815, beim Umsetzen von E3

**Der sechste Fall, und er trägt beide bekannten Ursachen auf einmal.** Schritt E3
nennt unter `Files:` `crates/krk-ui/src/appkit/bereichsleiste.rs` und
`crates/krk-ui/src/appkit/anwendung.rs`. Geändert sind sieben Dateien.

**Erste Ursache, der Zugriffsweg — wie bei C2, E1 und E3 vorhergesagt.** Der Schritt
verlangt, `bereichsleiste_nachziehen` hole den Wert „aus dem Modell des sichtbaren Tabs
des aktiven Dateifensters". Aus den zwei genannten Dateien ist dieses Modell nicht
erreichbar; der Weg läuft über `self.dateifenster(seite).quelle()`, also über
`DateifensterQuelle` in `crates/krk-ui/src/appkit/tabelle.rs`. Dazugekommen ist dort
**eine** öffentliche Methode, unmittelbar über `filter_steht` und in derselben Bauart:

- `tiefe_suche_steht() -> bool` — liest `Ordnermodell::tief` am sichtbaren Tab. Die
  Leseseite von `tiefe_suche_umschalten` aus E1, dieselbe Adresse (das **aktive**
  Dateifenster) für Schreiben und Lesen.

Damit ist der Befund für alle drei vorhergesagten Schritte eingetroffen: C2, E1 und E3
haben ihre dritte Datei jeweils gebraucht.

**Zweite Ursache, die Folge der Änderung — wie bei D1.** Die Zahl „acht Schalter" steht
in diesem Baum in Prosa, und der neunte Schalter macht jede Nennung in demselben Zug
falsch. Innerhalb von `bereichsleiste.rs` waren es sechs Stellen. Außerhalb waren es
vier, in vier weiteren Dateien, und keine davon steht in einer Dateiliste dieses Plans:

- `crates/krk-ui/src/appkit/mod.rs:79` — „acht Ankreuzfelder, fuenf fuer die Bereiche
  und drei fuer die schaltbaren Spalten", die Übersicht über den AppKit-Anteil.
- `crates/krk-ui/src/spalten.rs:73` und `crates/krk-ui/src/fenstermodell.rs:244` — die
  gleichlautende Begründung dafür, warum `beschriftung` kurze Namen liefert: „weil die
  Leiste 18 Punkte hoch ist und acht Schalter nebeneinander traegt".
- `crates/krk-ui/src/appkit/tabelle.rs:247` — dieselbe Begründung ein drittes Mal, für
  den abweichenden Namen der Datumsspalte.

Alle vier sind mitgezogen. **Die drei letzten sind dabei dieselbe Aussage an drei
Stellen** und damit ein eigener, kleinerer Befund neben diesem: eine Begründung, die in
drei Dateien wörtlich wiederholt steht, veraltet dreifach. Ein Datensatz dafür ist
nicht angelegt — er gehört nicht in diese Runde, und diese Nennung hier reicht, um ihn
bei der Berichtigung mit aufzugreifen.

**Die siebte Datei ist die Probe, und sie hat einen dritten Grund.**
`crates/krk-core/tests/verzeichnis.rs` trägt jetzt
`ohne_filtertext_aendert_die_tiefe_suche_nichts` für die eine Hälfte von C2.4, die im
Kern zu prüfen ist: ohne Filtertext entscheidet der Befund über keine Zeile. Die andere
Hälfte, dass über die Zulässigkeit der Wirkungsbereich entscheidet, steht in
`bereichsleiste.rs`. **Ein Kriterium, dessen zwei Hälften in zwei Kisten fallen, sprengt
jede Dateiliste, die einen Schritt einer Kiste zuordnet** — das ist weder der
Zugriffsweg noch eine Folge, sondern der Zuschnitt des Kriteriums.

**Ein Anlass, den der Schritt vorschreibt, war schon da.** E3 nennt drei neue Anlässe
für `bereichsleiste_nachziehen`: Tabwechsel, Wechsel des aktiven Dateifensters,
Ordnerwechsel. Gebraucht wurde **eine** Zeile, und sie deckt zwei davon ab: der
Rückruf `ordnerwechsel_setzen` in `oberflaeche_aufbauen` wird von
`ordnerwechsel_melden` in `tabelle.rs` sowohl beim Tabwechsel (`tab_gewechselt`) als
auch beim Ordnerwechsel (`ordner_lesen`, `sichtbaren_lesen`) ausgelöst, mit der Maus
wie mit der Taste. Der dritte, der Wechsel des aktiven Dateifensters, braucht keine
Zeile: er läuft über `aktives_setzen` (Mausklick) oder über `Kommando::FensterWechseln`
(Tastenbefehl), und beide rufen `aufteilung_nachziehen`, das
`bereichsleiste_nachziehen` schon enthält. **Das ist keine Abweichung vom Schritt,
sondern seine Erfüllung mit weniger Code**, und es steht hier, weil ein Abgleich, der
drei neue Aufrufstellen sucht, sonst eine fehlende fände.

## Was der Befund inzwischen misst

Sechs der vierzehn Schritte, und weiterhin drei Ursachen:

- **C2, E1, E3** — die Datei steht in keinem Teil des Schritts; der Weg an das
  Tabmodell führt trotzdem durch sie. Alle drei Vorhersagen sind eingetroffen.
- **B1** — die Datei steht im Fließtext des Schritts und fehlt in seiner Dateiliste.
- **D1, E3** — die Dateien sind vollständig für die *Änderung*, aber nicht für ihre
  *Folge*: eine Rangverschiebung bei D1, eine Zahl in Prosa bei E3.

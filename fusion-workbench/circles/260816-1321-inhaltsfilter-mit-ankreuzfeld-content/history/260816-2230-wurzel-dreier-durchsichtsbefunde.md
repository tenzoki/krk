# Die gemeinsame Wurzel dreier Durchsichtsbefunde: eine Frage, einmal gestellt

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Durchsicht:** `reviews/260816-1936-coderev-inhaltsfilter-der-dateiliste.md`, Abschnitt `## Übergreifende Beobachtungen`
**Befunde:** `issues/260816-1930`, `issues/260816-1931`, `issues/260816-1933` — alle drei geschlossen
**Nutzerentscheid:** 260816-2130, „nicht die drei einzeln, sondern ihre Wurzel"
**Baumstand vor der Arbeit:** `f9ca26f`

## Die Wurzel, wie sie sich hat fassen lassen

Die Durchsicht sagt: eine Erwägung der Runde 10 ist mit verändertem Preisschild
weitergereicht worden. Am Code liest sich das so:

**Dieselbe Frage stand an drei Stellen in drei Fassungen, und eine Antwort wurde
ohne ihre Frage aufbewahrt.**

- `Ordnermodell::sichtbar` entschied, wessen Zeile an einem Befund hängt.
- `krk-ui`s freie Funktion `auftraege` entschied, wer einen Auftrag verdient —
  dieselbe Frage, zweite Fassung, und ihr fehlte der erste Zweig des
  Prüfschritts (das Ausblenden).
- `tief_setzen` und `inhalt_setzen` entschieden jede für sich, wann ein Befund
  verfällt — beide unsymmetrisch, beide mit derselben Begründung „weil ihn dann
  niemand liest", die seit der Runde 11 für einen Ordner nicht mehr stimmte.

Solange ein Befund einen Metadatengang kostete und nur Ordner betraf, konnten
die Fassungen auseinanderlaufen, ohne dass es etwas kostete. Seit dem
Inhaltsfilter kostet jede Abweichung ein `open(2)`, bis zu 1 MB gelesene Bytes
oder eine falsche Zeile.

## Der eine Eingriff

**Der Prüfschritt liefert einen Wert, und alle drei Frager lesen ihn.**

`Zeilengrund` (`krk-core/src/verzeichnis/modell.rs`, privat) hat drei Werte:
`Steht`, `FaelltWeg`, `UnterVorbehalt(Auftragsart)`. `zeilengrund_von` rechnet
ihn — das ist der bisherige Prüfschritt ohne seinen letzten Schritt —,
`grund_neu_rechnen` legt ihn je Eintrag ab, und drei Stellen lesen ihn:

| Frager | vorher | nachher |
|---|---|---|
| `sichtbar` | ganzer Prüfschritt je Eintrag | ein Blick in `grund` und in `befund` |
| `steht_wegen_des_inhalts` | fünf Vorbedingungen, darunter die Namensfrage, je gezeichneter Zelle | ein Vergleich |
| `auftraege` | zweite Fassung des Prüfschritts, in `krk-ui` | ein Gang über `grund` |

Daran hängen alle drei Befunde:

**1930.** Der Befundvektor gehört zu einer Frage, und die Frage ist der
kleingeschriebene Filtertext samt der Angabe, ob der Inhalt mitzählt — genau die
beiden Werte, mit denen `Durchlauf::starten` losläuft. `schalter_setzen` ist die
eine Stelle, die das misst: `inhalt_wirkt()` vorher merken, nachher vergleichen,
bei Abweichung zurücksetzen. Die zwei unsymmetrischen Zweige sind weg, und das
Ausschalten von „Content" nimmt die Ordnerzeile jetzt sofort mit (C2.9).

**1931.** Ein ausgeblendeter Eintrag hat `Zeilengrund::FaelltWeg` und bekommt
damit ohne eigene Regel keinen Auftrag mehr. Dafür ist das Ein- und Ausblenden
eine Eingabe der Auftragsliste geworden: `verstecke_umschalten`
(`krk-ui/src/appkit/tabelle.rs`) zieht `durchlauf_nachziehen` und
`meldung_gewechselt` nach, in derselben Bauart wie die beiden anderen Schalter.
**Der Abstieg in versteckte Ordner bleibt unverändert** — dazu unten.

**1933.** Die Namensfrage hat einen Rufer statt dreier, und der Zeilengrund hängt
nicht am Befund: ein eintreffender Befund baut die Sicht neu auf, ohne die Frage
noch einmal an 100.000 Einträge zu stellen. Vorher lief dieser Gang bei **jedem**
Einzugstakt, während eines Durchlaufs also bis zu sechzigmal in der Sekunde.
Dazu trägt `Auftrag` nur noch `index` und `art`; den Namen schlägt der Durchlauf
im Bestand nach, den `Ordnermodell::bestand` als `Arc<Vec<Eintrag>>` mitreicht.

## Was geändert ist

| Datei | Was |
|---|---|
| `crates/krk-core/src/verzeichnis/modell.rs` | `Zeilengrund`, `zeilengrund_von`, `grund`, `grund_neu_rechnen`, `schalter_setzen`, `auftraege`, `bestand`; `eintraege` als `Arc<Vec<Eintrag>>`; `inhalt_entscheidet` gefallen |
| `crates/krk-core/src/verzeichnis/durchlauf.rs` | `Auftrag` ohne `name`; `starten` nimmt den Bestand; `Auftragslage` bündelt die fünf unveränderlichen Argumente des Fadens |
| `crates/krk-core/src/verzeichnis/filter.rs` | Prosa: der Rufer des Vergleichs heißt jetzt anders |
| `crates/krk-ui/src/tabs.rs` | freie Funktion `auftraege` gefallen; `durchlauf_nachziehen_an` nimmt Liste und Bestand aus dem Modell |
| `crates/krk-ui/src/appkit/tabelle.rs` | `verstecke_umschalten` zieht den Durchlauf nach |
| `crates/krk-core/tests/verzeichnis.rs` | vier neue Proben, zwei umgeschriebene, `bestand_aus` als Hilfsmittel |
| `messungen/260816-abnahme-inhaltsfilter.md` | Beobachtungen 26 und 27 neu, 17 und 21 nachgezogen, C2.9 und C4.2 im Nachweisverzeichnis |

## Was prüfbar gemacht ist

Alle in `crates/krk-core/tests/verzeichnis.rs`, alle ohne Fenster:

- `das_ausschalten_des_inhaltsfilters_nimmt_auch_die_ordnerzeile_sofort_weg` — die Zusage aus C2.9 für den Ordner.
- `das_ausschalten_nimmt_auch_eine_namentlich_begruendete_ordnerzeile_mit` — ihr Preis, gemessen statt behauptet.
- `ein_befund_gilt_nur_zu_seiner_frage` — die Regel dahinter, mit der Gegenprobe, dass „Deep" allein keine Antwort wegwirft.
- `ein_ausgeblendeter_eintrag_bekommt_keinen_auftrag` — beide Hälften des umgedrehten Handels in einem Zug.
- `die_auftragsliste_traegt_je_typ_die_richtige_art` — der Schnitt über den Typ.
- `die_namensfrage_des_filters_hat_einen_rufer` — Zählprobe im Stil des Projekts; sie fällt, sobald ein zweiter Ort die Frage wieder stellt.

Dazu hält der Übersetzer, dass ein `Auftrag` keinen Namen mehr trägt.

## Was ausdrücklich nicht geändert ist

**Der Abstieg in versteckte Ordner.** Ein Treffer unter einem versteckten Ordner
ist ein Treffer unter dem sichtbaren Ordner darüber; ihn zu übergehen wäre eine
neue Regel und keine Ersparnis und änderte die Bedeutung von C3.1. Der Datensatz
`260816-1931` trennt die beiden Hälften selbst so. Die Begründung steht am
Doc-Kommentar von `Ordnermodell::auftraege`, damit die nächste Durchsicht die
Frage nicht noch einmal aufmacht.

**Die drei anderen Befunde der Durchsicht.** `260816-1932` (Deskriptormangel und
Lesehinweis), `260816-1934` und `260816-1935` (Prosa) sind nicht angefasst, wie
beauftragt. Ebenso wenig die fünf älteren offenen Datensätze.

**Die Zahl der Zeitzusagen.** Es ist keine elfte gesetzt. Was `1933` kostet, ist
in Gängen über den Bestand gezählt und nicht in Millisekunden am Referenzgerät;
die Lage ist dieselbe wie beim Namensfilter der Runde 10.

## Abnahme

`export PATH="$HOME/.cargo/bin:$PATH" && make check` — exit 0. Die
Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist
durchgelaufen; sie ist nicht angefasst.

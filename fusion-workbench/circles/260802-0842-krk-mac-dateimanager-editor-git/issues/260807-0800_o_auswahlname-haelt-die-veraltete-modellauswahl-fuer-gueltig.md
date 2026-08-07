auswahlname hält die veraltete Modellauswahl für gültig und überschreibt damit eine vorgemerkte wunschauswahl

---

`Tabinhalt::auswahlname` zieht die Auswahl des Ordnermodells der `wunschauswahl`
vor. Seit `5f2e45d` ist diese Modellauswahl zwischen `lesevorgang_beginnen` und
dem ersten gelieferten Stapel veraltet: sie zeigt auf einen Eintrag des vorigen
Laufs. `aktiven_neu_lesen` liest sie in dieser Spanne aus und schreibt sie in
die `wunschauswahl` — und überschreibt damit einen Namen, den ein Aufrufer
kurz zuvor dort vorgemerkt hat.

---

## Die Stellen

- `crates/krk-ui/src/tabs.rs:192-201` — `auswahlname` nimmt `wunschauswahl` nur,
  wenn `modell.auswahl()` `None` ist. Bei vorgemerktem Ersatz ist sie `Some`,
  und der Name gehört dem alten Ordner.
- `crates/krk-ui/src/tabs.rs:476-486` — `aktiven_neu_lesen` setzt
  `tab.wunschauswahl = auswahlname()` bedingungslos.
- `crates/krk-core/src/verzeichnis/modell.rs` — die Probe
  `ein_neuer_ordner_hebt_die_auswahl_auf` hält ausdrücklich fest, dass
  `auswahl()` nach `lesevorgang_beginnen` noch `Some` ist.

## Der Ablauf

```
ordner_neu_lesen  ──> aktiven_neu_lesen  ──> wunschauswahl := "alt.txt"
                                             ersatz vorgemerkt, auswahl = alt.txt
eintrag_waehlen("neu.txt")               ──> Vorgemerkt, wunschauswahl := "neu.txt"
zweite Auffrischung vor dem ersten Stapel ─> aktiven_neu_lesen liest auswahl()
                                             = alt.txt, wunschauswahl := "alt.txt"
erster Stapel / Abschluss                 ─> Auswahl steht auf alt.txt
```

Der neu angelegte oder umbenannte Eintrag bekommt die Auswahl nicht, obwohl
`crates/krk-ui/src/appkit/anwendung.rs:1884-1885` (Anlegen) und `:1907-1908`
(Umbenennen) sie ihm zusagen.

Vor dem 260807 konnte das nicht eintreten: `leeren` hatte `auswahl` beim Start
des Lesevorgangs auf `None` gesetzt, `auswahlname` fiel deshalb auf die
`wunschauswahl` zurück, und der vorgemerkte Name überlebte jede Zahl von
Auffrischungen.

## Reichweite

*inference, nicht beobachtet.* Der Fall verlangt zwei `neu_lesen`-Aufrufe
innerhalb derselben Spanne, also bevor der erste Stapel angehängt ist. Zwei
Wege dorthin:

1. Der FSEvents-Rückruf arbeitet die gemeldeten Pfade in einer Schleife ab
   (`crates/krk-ui/src/appkit/anwendung.rs:1236-1241`) und ruft
   `ordner_neu_lesen` je Pfad. Nennt ein Bündel denselben Ordner zweimal,
   laufen zwei Auffrischungen im selben synchronen Aufruf.
2. `crates/krk-ui/src/appkit/anwendung.rs:2303-2305` läuft über
   `vorgang.ordner()`; stehen dort zwei Einträge, die auf denselben angezeigten
   Ordner zeigen, gilt dasselbe.

Zwischen zwei FSEvents-Bündeln liegen dagegen mindestens die 0,3 s
Sammelverzögerung aus `crates/krk-ui/src/appkit/fsevents.rs:94`; über diesen
Weg allein ist der Fall auf kleinen Ordnern nicht zu erreichen.

## Denkbarer Weg

`auswahlname` fragt das Modell, ob sein Bestand noch dem vorigen Lauf gehört,
und fällt dann auf die `wunschauswahl` zurück. Dieselbe Frage, die der Befund
`260807-0800_o_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-…`
braucht; beide sind dieselbe Ursache an zwei Lesestellen, und eine gemeinsame
Antwort ist einer zweiten Sonderfallzeile vorzuziehen.

## Dringlichkeit

Mittel. Nicht beobachtet, aber der Bruch sitzt in derselben Zeile, die die
Auffrischung über C9 trägt, und die Umstellung vom 260807 hat ihn erst
geschaffen. Keine der zehn Zeitzusagen aus C8 ist berührt.

**Betrifft:** `crates/krk-ui/src/tabs.rs`.

**Aufgefallen bei:** der inkrementellen Durchsicht nach Turn 25 der Sitzung
260806-2257, Diff `f9a0462..HEAD`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0800_o_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-und-die-auswahl-faellt-danach-ersatzlos.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md`

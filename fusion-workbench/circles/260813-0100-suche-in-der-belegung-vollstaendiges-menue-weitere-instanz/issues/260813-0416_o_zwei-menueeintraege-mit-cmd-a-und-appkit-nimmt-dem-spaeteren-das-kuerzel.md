Zwei Menüeinträge mit Cmd+A, und AppKit nimmt dem späteren das Kürzel

---

Seit das Hauptmenü jede Funktion der Belegung trägt (C2.1), stehen `alle_markieren` und
`text_alles_auswaehlen` beide als Menüeintrag da, und beide tragen `cmd+a`. Eine Menüleiste
verträgt dieselbe Tastenentsprechung nicht zweimal: **AppKit nimmt sie dem später stehenden
Eintrag still weg.** Gemessen am 260813 über `--menue-protokoll` am Bau von S6, vor der
Gegenmaßnahme:

```
menue="Dateilisting" eintrag="Alle Einträge markieren"  kombination=cmd+a    kuerzel="a" zusatztasten=1048576
menue="Bearbeiten"   eintrag="Alles auswählen"          kombination=(keines) kuerzel=""  zusatztasten=1048576
```

Die Maske bleibt stehen, das Zeichen ist fort. Derselbe Lauf gegen `HEAD` (`9da33bc`, zehn
Menüeinträge) zeigt für „Alles auswählen" `kuerzel="a"`.

**Die Folge wäre ein Bruch von C2.18 gewesen.** Ohne Menükürzel erreicht `cmd+a` den
Feldeditor eines Textfeldes auf keinem Weg — das ist am 260804-1309 gemessen und der Grund,
aus dem es das Menü „Bearbeiten" überhaupt gibt
(`issues/260804-1309_*_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`).
`Belegung::nachschlag` überspringt zugestellte Funktionen, der Ereignisabgriff kennt
`text_alles_auswaehlen` also nicht. In einem Textfeld wäre Cmd+A nach dieser Runde wirkungslos
geworden.

---

**Schwere:** hoch, wäre er stehengeblieben (ein Mac-Standardbefehl fällt in jedem Textfeld
aus); **behoben** im selben Schritt, siehe unten
**Gefunden:** coder, beim Bauen von S6 der Runde 7 am 260813-0416, über
`cargo run -p krk-ui --bin krk -- --menue-protokoll`
**Betroffen:** `crates/krk-ui/src/menuemodell.rs`, `resources/default-keymap.toml`
(`alle_markieren`, `text_alles_auswaehlen`)
**Domain:** code

## Warum der Fall neu ist und nicht schon immer bestand

Der Nutzerentscheid vom 260805
(`decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`) erlaubt
dieselbe Kombination bei zwei Funktionen, sofern **verschiedene Zusteller** sie tragen. Die
Begründung steht im Kopf von `resources/default-keymap.toml`: „zwei Funktionen mit
verschiedenen Zustellern begegnen einander nie", weil der Fokusvorbehalt jeden Tastendruck
genau einem von beiden zuteilt.

**In der Belegungsdatei stimmt der Satz weiterhin. In der Menüleiste stimmt er seit dieser
Runde nicht mehr.** Bis dahin trug das Menü zehn Einträge, und `alle_markieren` war keiner
davon; die zwei begegneten einander tatsächlich nie. Mit C2.1 stehen beide in derselben
Leiste, und dort entscheidet nicht der Fokusvorbehalt, sondern AppKit nach der Stellung.

Ausgeliefert ist es **genau eine** Kombination. Nachgezählt am 260813 über alle 81 Funktionen
von `resources/default-keymap.toml`: `cmd+a` ist die einzige, die zweimal vorkommt.

## Was gebaut wurde

`crates/krk-ui/src/menuemodell.rs` gibt das Menükürzel dem Zusteller: `zugestellte_kuerzel`
sammelt die ersten Kombinationen der sechs zugestellten Funktionen, und ein Befehlseintrag,
dessen erste Kombination darunter steht, bekommt keine. Am Bau danach gemessen:

```
menue="Dateilisting" eintrag="Alle Einträge markieren"  kombination=(keines) kuerzel=""
menue="Bearbeiten"   eintrag="Alles auswählen"          kombination=cmd+a    kuerzel="a"
```

**Warum in diese Richtung und nicht in die andere:** ein Befehl von KRK braucht sein
Menükürzel nicht. Der Ereignisabgriff sieht jeden Tastendruck **vor** dem Menü und führt ihn
aus, wo er zulässig ist; im Dateifenster verbraucht `alle_markieren` das `cmd+a`, bevor das
Menü es sieht. Eine zugestellte Funktion hat diesen zweiten Weg nicht.

**Der Preis** ist die Anzeige: „Alle Einträge markieren" steht im Menü ohne `Cmd+A`, obwohl
`Cmd+A` es auslöst. Die Belegungsansicht und die Markdown-Ausgabe zeigen die Kombination
unverändert — sie fragen die Belegung und nicht das Menümodell.

Zwei Proben halten es: `bei_einer_doppelten_kombination_traegt_der_zusteller_das_kuerzel` sucht
sich den Fall selbst und schlägt fehl, wenn es keinen mehr gibt;
`keine_zwei_eintraege_tragen_dieselbe_kombination` hält die eigentliche Zusage und ist
unabhängig davon, wie viele Doppelungen es gibt.

## Was offen bleibt

**Die Richtung ist eine Nutzerentscheidung und liegt als Datensatz vor:**
`decisions/260813-0430_o_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`.
Die Runde fährt auf ihrer Empfehlung, also auf dem oben Gebauten.

**Ungemessen bleibt, was ein ausgegrauter Eintrag mit einer Tastenentsprechung tut.** Nach der
Gegenmaßnahme trägt in der Leiste nur noch **ein** Eintrag `cmd+a`, und die Frage stellt sich
für diesen Fall nicht mehr. Sie steht daneben trotzdem im Raum: ob AppKit nach einem
abgewiesenen Eintrag weitersucht, ist am Baum nicht entscheidbar. Am Bündel zu sehen: `cmd+a`
mit der Schreibmarke in der Pfadeingabe und beim Umbenennen in der Liste.

**Der billigere Ort wäre die Belegungsdatei.** Trüge `alle_markieren` eine zweite Kombination
ohne Doppelung, entfiele der Fall ganz. Das ist eine Änderung an
`resources/default-keymap.toml` und gehört dem `ontocoder`; sie ändert daneben, was der Nutzer
gewohnt ist, und ist deshalb keine Aufräumarbeit, sondern eine eigene Frage.

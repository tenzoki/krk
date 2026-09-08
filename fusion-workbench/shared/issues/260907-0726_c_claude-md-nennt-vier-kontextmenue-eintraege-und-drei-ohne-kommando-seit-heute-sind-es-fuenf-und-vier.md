CLAUDE.md nennt vier Kontextmenü-Einträge und drei ohne `Kommando` — seit heute sind es fünf und vier

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Baumstand:** `354dac3` plus die Arbeit vom 260907-0726 (noch nicht committet)

---

Der Eintrag „Im Finder anzeigen" ist heute als fünfter Eintrag des Kontextmenüs der Dateiliste
gebaut (Nutzeranforderung vom 260907-0703, festgehalten in
`260905-2008-orchestrator-session.md`, Abschnitt
`## Fuenf weitere Entscheidungen am 260907-0703 beantwortet`). Damit stehen drei Angaben in
`CLAUDE.md` neben dem Baum, alle drei im Absatz **„Seit der Runde 17 führt ein zweiter Weg in die
Anwendung hinein, und er hat keine Taste"** unter `## Was man nicht sieht, wenn man es nicht
weiß`:

| zitiert | ist |
|---|---|
| „Das Kontextmenü der Dateiliste trägt vier Einträge" | fünf: Teilen, Zip, Unzip, „Im Finder öffnen", „Im Finder anzeigen" |
| „daneben Zip, Unzip und „Im Finder öffnen", die keines sind. **Die drei** tragen weder eine Tastenkombination noch einen Hauptmenüeintrag" | vier, mit „Im Finder anzeigen" |
| „Ein **vierter** Wert hält damit den Bau an, statt still nichts zu tun" (über `Kontextbefehl`) | ein fünfter |

Erhoben wird der Bestand mit
``awk '/pub enum Kontextbefehl/,/^}/' crates/krk-ui/src/kommandos/kontextmenue.rs`` und an
`Kontextbefehl::ALLE` in derselben Datei; die Titel stehen in `Kontextbefehl::titel`, die Tafel
der Probe daneben in deren Prüfmodul.

**Die Aussage des Absatzes bleibt richtig, allein die Zahlen fallen.** Der zweite Weg in die
Anwendung hat weiterhin keine Taste, die Einträge hängen weiterhin weder an
`Kommando::wirkungsbereich` noch an `bereich_des_kommandos`, und `Kontextbefehl` ist weiterhin
die Sperre gegen den wirkungslosen Menüeintrag. Es ist derselbe Befund wie bei den Aufzählungen,
die `CLAUDE.md` aus genau diesem Grund nicht mehr beziffert (`Bereich`, `Fokus`, `Spalte`,
`Wirkungsbereich`, `Kommando`, `Art`): eine Zahl in der Prosa, die mit der nächsten Runde
falsch wird.

**Zwei weitere Stellen im Baum trugen dieselben Zahlen und sind mit dieser Arbeit nachgezogen**,
nicht neu beziffert: die Modulköpfe von `crates/krk-ui/src/appkit/tabelle.rs` und
`crates/krk-ui/src/kommandos/mod.rs` sagen jetzt „die eigenen Einträge" statt „die drei".
Dieselbe Form ist die Abhilfe für `CLAUDE.md`.

**Abnahmebedingung:** keine Stelle in `CLAUDE.md` nennt eine Zahl der Kontextmenü-Einträge oder
der `Kontextbefehl`-Werte; der Absatz verweist stattdessen auf `Kontextbefehl::ALLE` und nennt den
Erhebungsbefehl, wie es die Absätze zu `Wirkungsbereich` und `Kommando` vormachen.

---
Resolved: Der Absatz nennt keine Zahl der Kontextmenü-Einträge und keine der
`Kontextbefehl`-Werte mehr. Er sagt, dass das Menü das Teilen und daneben die
eigenen Einträge trägt, zeigt für deren Bestand auf `Kontextbefehl::ALLE` und
nennt den Erhebungsbefehl
``awk '/pub enum Kontextbefehl/,/^}/' crates/krk-ui/src/kommandos/kontextmenue.rs`` —
dieselbe Form, die die Absätze zu `Wirkungsbereich` und `Kommando` schon tragen.
Der Satz „mit den drei eigenen Einträgen des Kontextmenüs" in `CLAUDE.md:86`
heißt jetzt „mit den eigenen Einträgen"; das ist die Form, die die Modulköpfe von
`appkit/tabelle.rs` und `kommandos/mod.rs` vorgemacht haben.

Zwei Angaben sind dabei über den Befund hinaus berichtigt worden, weil sie
derselbe Satz trug: „Ein vierter Wert hält damit den Bau an" ist zu „Ein weiterer
Wert" geworden, und daneben steht jetzt, was der Modulkopf ausdrücklich sagt und
`CLAUDE.md` verschwieg — **die Feldbreite der Liste `ALLE` hält den Bau nicht
an**; gehalten wird die Liste von der Probe
`jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`.

Die eine Zahl, die stehen bleibt, ist datiert und deshalb unverfälschbar: „die
Zahl stand von der Runde 17 bis zum 260907 auf drei". Denselben Satz führt der
Modulkopf von `kontextmenue.rs`.

Belegt am Baum vor der Änderung: `Kontextbefehl` trägt `Zippen`, `Entpacken`,
`ImFinderOeffnen`, `ImFinderAnzeigen`.

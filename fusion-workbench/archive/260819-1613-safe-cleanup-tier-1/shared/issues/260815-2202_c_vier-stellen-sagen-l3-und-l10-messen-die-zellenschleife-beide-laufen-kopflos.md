Vier Stellen sagen, L3 und L10 messen die Zellenschleife — beide laufen kopflos

---

Der Einbau des Ordnerzeichens begründet an vier Stellen, warum eine Verknüpfung auf einen
Ordner kein Zeichen bekommt: „das Verweisziel zu erfragen hieße ein `stat` je sichtbarer
Zeile, und genau diese Schleife messen L3 und L10." **Der Satz ist falsch.** L2, L3 und L10
werden auf der kopflosen Strecke gemessen; sie baut keine `NSTableView`, ruft
`zellenansicht` nie und sieht kein einziges `NSTextField`. Ein `stat` je sichtbarer Zeile
liefe unter **keiner** der zehn Zusagen aus C8 auf.

---

**Schwere:** mittel. Kein Fehlverhalten am Code — die getroffene Entscheidung (kein `stat`
je Zeile) ist richtig. Falsch ist ihre Begründung, und sie ist die Begründung, auf die
sich der nächste Entwurf stützen wird: Option 1 des Entscheids (Sinnbild in der
Namensspalte) ist mit demselben Satz eingeklammert worden.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:325-346` und `:3281-3283`,
`fusion-workbench/shared/decisions/260815-2056_i_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`
(Abschnitte `## Randbedingungen` und `### Was daraus folgt`), die Commit-Nachricht von
`3b128c3`, `fusion-workbench/shared/history/260815-2110-coder-…`
**Domain:** code

## Was am Baum steht

`crates/krk-bench/src/messen.rs:1199-1202`:

```
// Die kopflose Strecke aus S3: L2 und L3 auf Pruefordner A, L10 auf
// dem grossen Ordner, beide warm, wie C8 die Zusagen stellt.
let reihe_a = Messreihe::fahren(&self.ordner_a, Cache::Warm, self.wiederholungen)?;
let reihe_gross = Messreihe::fahren(&self.ordner100k, Cache::Warm, self.wiederholungen)?;
```

`Messreihe::fahren` fährt `einen_lauf_fahren` (`messen.rs:203`), und dessen Doc-Zeile sagt,
was gemessen wird: „Ein einzelner Lauf: Ordner lesen, Modell fuellen, Sortierung
herstellen." Die drei Beschriftungen im Gesamtergebnis sagen dasselbe noch einmal
(`messen.rs:1073-1082`, `:1153-1160`):

| Zusage | Beschriftung im Bericht |
|---|---|
| L2 | „Pruefordner A: erste Bildschirmseite (**Kernanteil, kopflos**)" |
| L3 | „Pruefordner A: vollstaendig gelesen und sortiert (**kopflos**, warm)" |
| L10 | „100.000 Eintraege: erste Bildschirmseite" — aus derselben kopflosen Reihe |

Damit gilt: L3 endet, wenn das Modell vollständig gelesen und sortiert ist, also **bevor**
irgendeine Zelle entsteht. L2 und L10 messen den Kernanteil bis zum ersten Stapel, ebenfalls
ohne Zeichendurchgang. `namensform` und `zellenansicht` liegen auf keiner der drei Strecken.

## Warum die Berichtigung die Vorsicht verschärft und nicht lockert

Der Satz suggeriert, ein Plattenzugriff je Zeile würde von einer bestehenden Messstrecke
gefangen. Er würde von keiner gefangen. Die richtige Fassung lautet: **die Zeichenschleife
der Dateiliste ist in diesem Baum ungemessen**, und ein `stat` je sichtbarer Zeile käme
deshalb ohne Abnahmekriterium in den Baum. Das ist der stärkere Grund, ihn zu unterlassen,
und er gehört an die vier Stellen statt des heutigen.

## Vorschlag

Die vier Stellen auf eine Formulierung ziehen, die den Baum trifft, etwa: „ein `stat` je
sichtbarer Zeile stünde in der Zeichenschleife der Dateiliste, und die misst keine der zehn
Zusagen aus C8 — L2, L3 und L10 laufen kopflos (`krk-bench/src/messen.rs:1199`)."
Der Entscheidungsdatensatz trägt denselben Satz und ist mit zu berichtigen; er ist `_i_` und
damit endständig, die Berichtigung gehört deshalb als Nachtrag an sein Ende, nicht in seinen
Text.

---
Resolved: Die zwei Codestellen sind nachgezogen (260816, coder), beide in
`crates/krk-ui/src/appkit/tabelle.rs`. Am Doc-Kommentar von `namensform` steht jetzt zuerst,
was die Sache entscheidet — die Gleichheit mit dem `--` der Spalte `Größe` —, und darunter in
einem eigenen Absatz, warum das Verweisziel nicht erfragt wird: ein `stat` je sichtbarer
Zeile stünde in der Zeichenschleife der Dateiliste, und die misst **keine** der zehn Zusagen
aus C8, weil L2, L3 und L10 auf der kopflosen Strecke laufen
(`krk-bench/src/messen.rs:1199`). Der Absatz sagt ausdrücklich, dass das der stärkere Grund
ist und nicht der schwächere, und nennt den bis zum 260816 dort stehenden falschen Satz samt
diesem Befund und dem Entscheid. Der Kommentar in der Probe
`allein_ein_ordner_traegt_den_schraegstrich` sagt dasselbe in einem Satz und verweist für die
Begründung auf `namensform`.

Die zwei übrigen der vier Stellen sind nicht von dieser Arbeit berührt und brauchten es
nicht: der Entscheidungsdatensatz `shared/decisions/260815-2056_i_…` trägt die Berichtigung
seit dem 260815-2210 als Nachtrag an seinem Ende und in seinem Abschnitt
`## Randbedingungen`, und die Commit-Nachricht von `3b128c3` ändert niemand mehr.

Gegengeprüft am ganzen Baum: außer den zwei berichtigten Stellen führt keine Code- oder
Prosadatei den Satz mehr. Die drei verbleibenden Nennungen von „L3 und L10" in
`crates/` — `tabelle.rs:1500`, `krk-core/src/verzeichnis/eintrag.rs:157`,
`krk-core/src/verzeichnis/verweisziel.rs:23` — meinen den Lesevorgang und den
Sortierschlüssel und sind richtig: die Strecke misst sie, und zwar kopflos.

Verification: `make check` — exit 0.

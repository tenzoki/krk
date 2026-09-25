# Coder: Schritt 8 — die Bereichsleiste, acht Schalter, Nachzug

**Datum:** 260812-0712
**Agent:** coder
**Status:** Complete
**Maßstab:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`,
Schritt 8 unter `## Implementierungsschritte`, dazu `## Warum die Leiste keinen sechsten
Fokuswert bekommt`
**Abnahme:** `make check` — **Exit 0**; `make bundle` — **Exit 0**
**Ausgangsstand:** HEAD `026c665`, Schritte 1 bis 7 gebaut und eingetragen

## Auftrag

Der letzte Schritt der Runde und der einzige, der eine neue Fläche baut: eine Leiste über die
volle Fensterbreite am unteren Rand, darin acht Ankreuzfelder — fünf für die Bereiche der
Fensterzeile, drei für die schaltbaren Spalten der Dateilisten. Dazu der Nachzug, der die acht
Zustände aus dem Modell schreibt, und die Hebung der Mindesthöhe des Fensters um genau die Höhe
der Leiste. Nicht committen; das Bündel bauen, KRK aber nicht starten.

## Was entstanden ist

**Neu: `crates/krk-ui/src/appkit/bereichsleiste.rs`** (564 Zeilen mit Modulkopf und Proben)

- `HOEHE = statuszeile::HOEHE`, nicht eine zweite 18 daneben. Ebenso `EINZUG`; neu sind allein
  `ABSTAND` (10 pt zwischen zwei Schaltern derselben Gruppe) und `GRUPPENABSTAND` (24 pt zwischen
  den fünf Bereichs- und den drei Spaltenschaltern).
- `Bereichsleiste::bauen(mtm)` legt eine `NSView` an und darin acht Ankreuzfelder über
  `NSButton::checkboxWithTitle_target_action`. Jedes bekommt `setControlSize(Small)`, danach die
  kleine Systemschrift (die Reihenfolge trägt: `setControlSize:` zieht die Schrift nach),
  `setToolTip` mit dem langen Namen, **`setRefusesFirstResponder(true)`** und seine `tag`.
- Feldbreiten `[Retained<NSButton>; 5]` und `[Retained<NSButton>; 3]`, dasselbe Muster wie
  `Aufteilung::rahmen`.
- `Leistenquelle` über `define_class!` mit `bereichGedrueckt:` und `spalteGedrueckt:`. Sie hält
  einen `Kommandomelder` (`Box<dyn Fn(Kommando)>`), den der Anwendungsdelegierte einträgt und der
  ihn **schwach** hält — wie die fünf anderen Melder dieses Projekts.
- `zustaende_setzen(&Sichtbarkeit, &Spaltensichtbarkeit)` schreibt alle acht Zustände. Sie ruft
  weder `anwenden` noch `setHidden` und fasst den Ersthelfer nicht an.

**`crates/krk-ui/src/fenstermodell.rs`**

- `Bereich::beschriftung` („Lesezeichen", „Links", „Rechts", „Vorschau", „Editor") und
  `Bereich::langname` (der Hinweistext), beide vollständige Fallunterscheidungen ohne
  Auffangzweig. `Bereich` trägt damit sieben davon.
- Probe `jeder_bereich_hat_eine_eigene_beschriftung`: beide Texte für alle fünf nicht leer und
  paarweise verschieden.

**`crates/krk-ui/src/appkit/fenster.rs`**

- Neue freie Funktion `fensterinhalt(mtm, fensterzeile, leiste)`. Sie legt beide übereinander:
  die Leiste unten mit `ViewWidthSizable | ViewMaxYMargin`, die Fensterzeile darüber mit
  `ViewWidthSizable | ViewHeightSizable`. `hauptfenster` behält seine Signatur und bekommt das
  Ergebnis als Inhaltsansicht.
- `MINDESTGROESSE` steigt in der Höhe von 300 auf `300.0 + bereichsleiste::HOEHE`. **Als Summe
  hingeschrieben und nicht als 318**, damit die beiden Zahlen nicht auseinanderlaufen können.
  Die Breite bleibt bei 780 (Nutzerentscheid vom 260812-0430).
- Modulkopf: die Inhaltsansicht ist nicht mehr die Aufteilung; die vier neuen Berührungen stehen
  im Abschnitt zur macOS-Untergrenze.

**`crates/krk-ui/src/appkit/anwendung.rs`**

- Neues Ivar `bereichsleiste: OnceCell<Bereichsleiste>`.
- Im Aufbau: Leiste bauen, `fensterinhalt` bilden, Melder eintragen, festhalten.
- Der Melder ruft `kommando_ausfuehren(kommando)` und **danach in jedem Fall**
  `bereichsleiste_nachziehen()` — außerhalb jeder Bedingung.
- `bereichsleiste_nachziehen` neu, gerufen aus `aufteilung_nachziehen` (und damit nach jedem
  ausgeführten Kommando) und aus dem Melder.

**`crates/krk-ui/src/appkit/mod.rs`**: `mod bereichsleiste;`, Überblick und Prosa nachgezogen.

## Die drei Fallen, und was mit ihnen geschehen ist

**Der Ersthelfer.** Jeder der acht Schalter trägt `setRefusesFirstResponder(true)`. `Fokus` hat
keinen sechsten Wert bekommen, `ersthelferbereich` ist unangetastet, und die Leiste liegt
ausdrücklich **neben** der `NSSplitView` und nicht darin — sonst wäre sie entweder ein sechster
Bereich oder ein blinder Fleck im Durchgang über `Bereich::ALLE`. Die Begründung steht im
Modulkopf der neuen Datei und nicht nur im Plan.

`inference:` Dass `refusesFirstResponder` den Rang bei eingeschalteter vollständiger
Tastaturbedienung verlässlich verhindert, ist in diesem Baum weiterhin **nicht gemessen**.
Kriterium C1.4 misst es am laufenden Bündel; das ist Nutzerarbeit.

**Der Nachzug.** `bereichsleiste_nachziehen` schreibt acht `setState` und sonst nichts. Kein
`anwenden`, kein `setHidden`, keine Berührung des Ersthelfers — derselbe Grund wie bei
`fokusanzeige_nachziehen`.

**Die macOS-Untergrenze.** Am SDK nachgelesen statt aus dem Plan übernommen, und dabei **eine
Abweichung gefunden**: der Plan führt `controlSize` unter den Berührungen ohne
Verfügbarkeitsangabe. `NSControl.h:32` trägt `API_AVAILABLE(macos(10.10))`. Der Modulkopf nennt
sie deshalb; die höchste Untergrenze der Datei bleibt `checkboxWithTitle:target:action:` mit
10.12 (`NSButton.h:59`). Geprüft und ohne eigene Angabe: `refusesFirstResponder`
(`NSControl.h:30`), `tag` (`NSControl.h:26`), `toolTip` (`NSView.h:310`), `sizeToFit`
(`NSControl.h:44`), `font` (`NSControl.h:60`), `state` (`NSButton.h:151`) und die drei Werte von
`NSControlStateValue` (`NSCell.h:71-74`). Das Bündel zielt auf 15.0.

## Drei Entscheidungen, die der Plan offenließ

**Die Aufbautabelle ist ein `match` und keine Feldtabelle.** Der Plan sagt „je Schalter eine
Zeile"; gebaut sind es zwei vollständige Fallunterscheidungen, `kommando_des_bereichs(Bereich) ->
Kommando` und `kommando_der_spalte(Spalte) -> Option<Kommando>`. Der Grund ist der Mechanismus,
an dem dieses Projekt hängt: ein `const [(Bereich, Kommando); 5]` hielte den Bau bei einem
sechsten Bereich **nicht** an, ein `match` ohne Auffangzweig tut es. `kommando_der_spalte` ist
zugleich die eine Stelle, die sagt, welche Spalten überhaupt einen Schalter tragen — die
Namensspalte liefert `None`.

**Die Stelle eines Spaltenschalters wird gerechnet.** `spaltenfach(Spalte) -> Option<usize>`
filtert `Spalte::ALLE` über dieselbe Funktion, aus der `bauen` die drei Schalter erzeugt. Drei
feste Zahlen daneben wären eine dritte Aufzählung gewesen. Das `try_into` auf `[_; 3]` hängt
damit an einer geprüften Aussage: `genau_drei_spalten_sind_schaltbar`.

**Keine Probe mit einer Ansicht.** Der Plan sieht vor, wo sie stünde, verlangt aber keine. Sie
ist nicht entstanden, und das ist eine Wahl mit einem Preis auf beiden Seiten: die vier
bestehenden Instanzproben behaupten den Hauptfaden über `MainThreadMarker::new_unchecked`, den
`libtest` ihnen nicht gibt (`issues/260810-1001_*`, als Lage angenommen und nicht behoben), und
ob sie in ein eigenes Prüfziel umziehen, ist zurückgestellt (`decisions/260810-1044_*`, ein Umbau
der ganzen Kiste). Eine fünfte Stelle derselben Bauart vergrößerte genau diesen Umbau. Was ohne
Fenster prüfbar ist, ist geprüft; was eine Ansicht braucht — die Höhe der Leiste, die Breite der
acht Schalter nebeneinander, das Zurückspringen nach einem abgewiesenen Klick — steht ohnehin auf
der Bündelliste des Plans.

## Proben

Sechs neu, alle ohne Fenster:

| Probe | Ort | Was sie hält |
|---|---|---|
| `jeder_bereich_hat_eine_eigene_beschriftung` | `fenstermodell.rs` | fünf Aufschriften und fünf Hinweistexte, nicht leer, paarweise verschieden |
| `die_leiste_traegt_acht_schalter` | `bereichsleiste.rs` | fünf plus drei |
| `genau_drei_spalten_sind_schaltbar` | `bereichsleiste.rs` | die Feldbreite `; 3` und C3.1 |
| `jeder_schalter_nennt_genau_ein_eigenes_kommando` | `bereichsleiste.rs` | kein Kommando zweimal |
| `jeder_schalter_wirkt_aus_jedem_fokus` | `bereichsleiste.rs` | alle acht `Wirkungsbereich::Ueberall` (C2.6) |
| `jede_schaltbare_spalte_hat_ihr_eigenes_fach` | `bereichsleiste.rs` | `spaltenfach` lässt kein Fach frei |

## Abnahme

```
make check   → Exit 0   (build, test, fmt --check, clippy -D warnings)
make bundle  → Exit 0   (target/KRK.app, signiert mit der Entwicklungsidentität)
```

KRK ist **nicht** gestartet worden. Der Augenschein an der laufenden Anwendung — C1.1, C1.2,
C1.4, C2.1 bis C2.5, C3.1, C3.2, C3.4, C5.1 und C6.3 — verlangt den Vordergrund und ist
Nutzerarbeit; der Plan führt ihn unter `## Abnahme am laufenden Bündel`.

## Was offen bleibt

- Kein neuer Defekt und keine neue Frage aus diesem Schritt. Die drei Wahlpunkte unter
  `## Offene Fragen` des Plans sind unverändert; keiner ist von diesem Schritt berührt.
- **Der nächste Abnahmelauf der zehn Zeitzusagen ist fällig.** Die Leiste nimmt der Fensterzeile
  18 Punkte Höhe, und L9 liegt auf dem gemessenen Weg. Der Plan setzt dafür keine neue Zahl.
- Nicht committet, wie beauftragt.

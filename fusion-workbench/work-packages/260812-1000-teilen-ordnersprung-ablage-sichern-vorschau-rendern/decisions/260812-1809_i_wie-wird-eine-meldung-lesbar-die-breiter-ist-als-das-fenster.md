# Wie wird eine Meldung lesbar, die breiter ist als das Fenster?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (auf Nachfrage des Nutzers)
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1105_*_die-statuszeile-zieht-ueber-die-volle-fensterbreite-und-laesst-sich-blaettern.md` (von diesem Datensatz überholt); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_c_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` (C5.10, C5.11, Schritt 11); `crates/krk-ui/src/appkit/statuszeile.rs`; `crates/krk-ui/src/appkit/bereichsleiste.rs:474` (der eine vorhandene Kurzhinweis)

---

## Question

Schritt 11 der Runde 6 hat die Statuszeile blätterbar gemacht, wie C5.10 es verlangt, und dabei
zwei Kosten erzeugt, die vorher niemand benannt hatte. Mit dem Zeiger über den achtzehn Punkten
am Fensterfuß bewegt ein Zweifingerstrich jetzt die Zeile und nicht die Liste darüber. Und die
Bildlaufansicht bringt `NSScroller` mit, also Steuerelemente derselben Art, für die C5.11 seit der
Runde 5 offen ist: ob sie bei eingeschalteter vollständiger Tastaturbedienung den Ersthelferrang
fernhalten, ist ungemessen.

Der Nutzer hat daraufhin am 260812 gefragt, ob das Blättern nicht zu viel sei, und einen Klick mit
Overlay vorgeschlagen. Die Frage ist damit allgemeiner als sein Vorschlag: **mit welchem Mittel
wird eine zu lange Meldung lesbar, und was kostet es?**

## Options

1. **Blättern, wie gebaut.** Die Zeile sitzt in einer `NSScrollView`.
   - Pros: gebaut und committet (`05797d7`), erfüllt C5.10 wörtlich.
   - Cons: der Gestenklau am Fensterfuß; C5.11 wird breiter statt enger.

2. **Kurzhinweis beim Verweilen** über `setToolTip:`, gesetzt nur dann, wenn der Text abgeschnitten
   ist.
   - Pros: das Mittel liegt im Baum, `bereichsleiste.rs:474` benutzt es für die Schalter der
     Leiste. Keine neue Ansicht, keine Ereignisbehandlung, kein Gestenklau. C5.11 bleibt so eng
     wie vor Schritt 11: die Zeile bleibt ein nicht auswählbares Textfeld. Der Hinweis bricht
     langen Text mehrzeilig um und zeigt ihn damit besser als eine einzeilige Bildlaufansicht.
   - Cons: der Text ist nicht markierbar und nicht kopierbar, und er verschwindet, sobald der
     Zeiger weggeht. Er erscheint erst nach einer kurzen Verweildauer, die das System bestimmt.

3. **Klick und `NSPopover`.** Die Zeile wird klickbar, ein Popover zeigt den vollen Text.
   - Pros: markierbar und kopierbar, bleibt stehen, bis man ihn schließt.
   - Cons: `NSPopover` kommt im ganzen Baum nicht vor, das wäre ein neues Mittel. Ein Klick
     braucht ein Ziel, und `labelWithString:` nimmt von sich aus keine Mausereignisse an; die
     Zeile müsste in die Ereigniskette. C5.11 wird damit deutlich breiter als beim Blättern.

4. **Klick und Blatt.** Ein Blatt aus `appkit/blaetter/` zeigt den Text.
   - Pros: zehn Blätter im Baum, das Muster ist eingeführt.
   - Cons: ein stehendes Blatt sperrt jeden Befehl außer dem Abbruch. Für das Nachlesen eines
     Satzes ist das unverhältnismäßig.

## Constraints

- Es bleibt bei **einer** Meldefläche (C5.9).
- `Fokus` bekommt keinen sechsten Wert (C5.12).
- Die Zeile bleibt achtzehn Punkte hoch; die Mindestfensterhöhe von 336 Punkten bleibt.

## Antwort 260812

**Möglichkeit 2, der Kurzhinweis.** Der Nutzer hat sie am 260812 gewählt, nachdem ihm die vier
Möglichkeiten mit ihren Kosten vorgelegt worden waren, und ausdrücklich gegen seinen eigenen
Vorschlag (Möglichkeit 3), weil dieser mehr Maschinerie ist und nicht weniger.

**C5.10 ist damit überholt.** Der Wortlaut „Die Zeile lässt sich nach rechts blättern" wird
ersetzt: eine Meldung, die breiter ist als das Fenster, wird über einen Kurzhinweis beim
Verweilen vollständig lesbar. Schritt 11 der Runde 6 wird zurückgenommen; die `NSScrollView`
verschwindet wieder aus `statuszeile.rs`.

**C5.11 kehrt damit auf seine Grundlage vor Schritt 11 zurück.** Ohne Bildlaufansicht gibt es
keine `NSScroller`, und die Frage betrifft wieder allein das Textfeld, das über `labelWithString:`
entsteht und nicht auswählbar ist. Sie bleibt offen und ist am Bündel abzunehmen, aber sie ist
wieder so eng wie in der Runde 5.

**Der Preis ist benannt und angenommen:** der Text ist nicht kopierbar. Wer eine Meldung
weitergeben will, tippt sie ab oder liest sie im Protokoll. Kopierbarkeit war in keiner Fassung
von C5 zugesagt.

**Was nicht überholt ist:** alles andere aus dem Datensatz vom 260812-1105. Die Zeile zieht über
die volle Fensterbreite, es gibt eine statt zweier, die Zuordnung zum Dateifenster steht im Satz,
und die zweistellige Ordnung aus Rang und aktiver Seite bleibt. Das ist mit `baf8660` gebaut und
bleibt stehen.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812` — Nutzerentscheid vom 260812, vorgelegt mit vier Möglichkeiten und ihren Folgen.
Implemented: `crates/krk-ui/src/appkit/statuszeile.rs`. Die `NSScrollView` samt `breite_nachziehen` und `an_den_anfang` ist weg, `Statuszeile` hält wieder allein ihr `NSTextField`, und `sicht` gibt es heraus. An ihre Stelle treten zwei Methoden: `Statuszeile::abgeschnitten` misst mit demselben `sizeToFit`, das Schritt 11 zum Setzen der Breite benutzte, wie breit der Text wäre, nimmt den Rahmen unmittelbar danach wieder zurück und vergleicht mit der Breite, die das Feld im Fenster hat; `Statuszeile::kurzhinweis_nachziehen` setzt `setToolTip:` genau dann und nimmt ihn sonst weg. Gerufen wird es einmal in `Statuszeile::zeigen`, hinter beiden Zweigen: eine geleerte Zeile hat nichts abzuschneiden, also räumt derselbe Ruf den Hinweis dort ab. Der Zuschnitt ist von `crates/krk-ui/src/appkit/bereichsleiste.rs:474` übernommen, dem einen vorhandenen Kurzhinweis des Baums; eine zweite Art, einen Hinweis zu setzen, entsteht nicht. `crates/krk-ui/src/appkit/fenster.rs` ist auf seinen Stand vor Schritt 11 zurückgenommen (Skizze im Modulkopf, zwei Doc-Absätze an `fensterinhalt`); Code stand dort ohnehin keiner.

Der Modulkopf von `statuszeile.rs` sagt beides aus: dass C5.11 ohne Bildlaufansicht wieder allein das nicht auswählbare Textfeld aus `labelWithString:` betrifft und **offen** bleibt, abzunehmen am Bündel mit eingeschalteter vollständiger Tastaturbedienung, und dass der Text nicht kopierbar ist. Die Angaben zu `NSScrollView`, `NSClipView` und `NSBorderType` sind aus dem Untergrenzen-Abschnitt heraus; an ihrer Stelle stehen `toolTip` (`NSView.h:310`), `stringValue` (`NSControl.h:36`), `sizeToFit` (`NSControl.h:44`) und `frame` (`NSView.h:129`), alle vier am SDK gegengelesen und ohne eigene Angabe, also seit 10.0. Die höchste Untergrenze der Datei ist wieder `labelWithString:` mit 10.12.

**Ein Preis ist beim Bauen sichtbar geworden, den der Datensatz nicht nennt.** Gemessen wird beim Setzen des Textes; zieht der Nutzer das Fenster danach breiter oder schmaler, ohne dass eine neue Meldung kommt, steht der Hinweis oder fehlt er nach der alten Breite. Er ist als `issues/260812-1854_*_der-kurzhinweis-der-statuszeile-veraltet-bei-einer-fensteraenderung.md` aufgeschrieben, samt dem Grund, warum der Nachzug hier nicht gebaut ist: der eine Auslösepunkt wäre `setFrameSize:` am Feld, und ihn zu überschreiben verlangte eine eigene Klasse über `NSTextField`, die sich nicht mehr über `labelWithString:` bauen ließe — also genau die Grundlage kostete, auf der C5.11 heute ruht.

Abnahme am 260812: `cargo build --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace` je Exit 0. 457 Proben im Binärziel `krk` gegenüber 454 vorher; die drei neuen gehören zur zweiten Aufgabe desselben Laufs, diese hier ist am Bündel abzunehmen und trägt keine. Noch nicht committet, der Nutzer committet nach der Aufgabe.
Deferred:
Superseded by:

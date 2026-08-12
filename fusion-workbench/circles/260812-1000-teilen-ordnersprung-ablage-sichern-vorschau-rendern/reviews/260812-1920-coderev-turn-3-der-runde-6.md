# Durchsicht Turn 3 der Runde 6

**Sender:** coderev
**Reviewed-range:** `94a81bd..df4ec00`
**Not-opened:** none

---

## Zusammenfassung

Die drei geschlossenen Defekte sind nachgemessen: alle acht Ausgaben, die ihre
Datensätze nennen, stimmen am Baum genau so. Die Statuszeile ist sauber auf
ihren Stand vor Schritt 11 zurückgenommen, C5.11 ruht wieder allein auf dem
Textfeld aus `labelWithString:`, und die neunzehn alten Proben sind
vollständig erhalten, eine davon verschärft.

**Die Behebung des Inhaltsverlusts hält aber nur für die zwei gemessenen Fälle,
nicht für die Zusage, die der Modulkopf daraus macht.** Es gibt einen dritten
Fall, den beide Deckungssätze verfehlen, und er ist gemessen. Dazu kommt eine
Verschlechterung, die diesen Turn erst entstanden ist: in einer losen Liste
steht das neue Merkzeichen allein auf seiner Zeile.

## Zahlen

| Gewicht | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 4 |
| Gering | 1 |

Alle sechs sind als eigene Datensätze unter `issues/` abgelegt, Zeitstempel
`260812-1920`.

## Was nachgemessen ist und hält

`markdown::rendern` (`crates/krk-ui/src/markdown.rs:152`) ist unverändert in
ein Prüfprogramm kopiert und gegen `pulldown-cmark 0.13.4` gefahren, so wie
beim Finden der Defekte in Turn 2.

**Die drei Fälle des Inhaltsverlust-Datensatzes sind behoben:**

```
"Text davor.\n\nSiehe [den Text][ref] hier.\n\n[ref]: https://example.com \"Titel\"\n"
  -> "Text davor.\n\nSiehe den Text hier.\n\n[ref]: https://example.com \"Titel\""
"[ref]: https://example.com\n[zwei]: https://b.example\n"
  -> "[ref]: https://example.com\n[zwei]: https://b.example"
"Siehe [](https://example.com) dort.\n"
  -> "Siehe [](https://example.com) dort."
```

**Die Auffangregel ist mechanisch.** `Zerlegung::luecke_bis` wird vor dem
`match` gerufen (`markdown.rs:158`), also ohne die Art des Ereignisses zu
kennen; `Zerlegung::schliessen` fragt allein `laenge == 0`
(`markdown.rs:618`). Nirgends steht eine Liste von Ereignisarten in der Regel.
Die `match`-Anweisung über `Event` in `rendern` (`markdown.rs:159-220`) trägt
keinen Auffangzweig — eine neue Variante hält den Bau an, wie es die Bauart
dieses Projekts verlangt.

**Die Tiefenzählung stimmt.** `Zerlegung::tiefe` (`markdown.rs:548-551`) zählt
`self.offen.iter().filter(|e| e.ebene.is_some()).count()`; ein zweiter Zähler
existiert nicht. Jenseits von acht Ebenen wächst der **Wert** weiter (bis 255,
danach gesättigt), und gedeckelt wird erst beim Verbraucher:
`einzugsmerkmal` rechnet `LISTENEINZUG * f64::from(tiefe.clamp(1, EINZUGSGRENZE))`
mit `EINZUGSGRENZE = 8` (`textmerkmale.rs:143, 447`). Gemessen an zehn Ebenen:
die Auszeichnungen tragen die Tiefen 1 bis 10, der Einzug bleibt ab 8 bei 160
Punkten. Das entspricht dem Datensatz.

**Kein Doppelschreiben und keine Bereichsüberschreitung.** 300 000 zufällig
zusammengesetzte Quellen aus 31 Markdown-Bausteinen gefahren: kein Absturz,
kein Fall, in dem `formatierung.laenge` von der wirklichen UTF-16-Länge
abweicht, kein Bereich außerhalb des Textes, und kein alphanumerisches Zeichen
in der Ausgabe, das nicht in der Quelle stand — mit einer Ausnahme, die keine
ist: die Neuvergabe der Nummern einer geordneten Liste (`1. … 1.` wird
`1. … 2.`), wie jeder CommonMark-Betrachter es tut.

**Die Statuszeile ist vollständig zurückgenommen.** `NSScrollView`,
`NSClipView`, `NSBorderType`, `breite_nachziehen` und `an_den_anfang` kommen in
`statuszeile.rs` nur noch in erklärender Prosa vor; die `use`-Zeile führt
`{NSColor, NSFont, NSTextAlignment, NSTextField, NSView}`, und `Statuszeile`
hält ein Feld: `feld: Retained<NSTextField>` (`statuszeile.rs:486-488`). Der
Kurzhinweis hängt an einer Messung und nicht am Vorhandensein eines Textes:
`kurzhinweis_nachziehen` setzt `setToolTip:` über
`self.abgeschnitten().then_some(&*voll)` (`statuszeile.rs:556-559`), und der
`None`-Zweig räumt einen alten Hinweis ab.

**C5.11 steht wieder auf seiner Grundlage vor Schritt 11.** Es gibt eine
Ansicht statt zweier, sie kommt aus `labelWithString:`, und der Kurzhinweis
fügt der Ereigniskette nichts hinzu. Ob sie den Ersthelferrang bei
eingeschalteter vollständiger Tastaturbedienung wirklich fernhält, ist damit
unverändert offen und am Bündel abzunehmen; hier ist es nicht beantwortet.

**Die zweistellige Ordnung in `statuszeile::zeile` bleibt vollständig und ohne
Gleichstand.** Die Sichtbarkeit ist ein **Filter vor** der Ordnung und keine
dritte Ordnungsstelle (`statuszeile.rs:420-424`); über den überlebenden
Bewerbern gilt die alte Regel unverändert. Der Fall „gar kein Bewerber, weil
beide Dateifenster ausgeblendet sind" ist unerreichbar, und zwar an beiden
Wegen ins Modell: `Fenstermodell::umschalten` weist das Ausblenden ab, solange
das andere schon aus ist (`fenstermodell.rs:649-652`), und
`Fenstermodell::aus_sitzung` blendet das linke wieder ein, wenn eine von Hand
geschriebene `session.toml` beide ausschaltet (`fenstermodell.rs:420-422`).

**Die neunzehn alten Proben sind inhaltlich erhalten**, keine ist schwächer
geworden. Achtzehn haben allein das vierte Argument bekommen; die neunzehnte,
`steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz`, ist **verschärft**:
sie übergibt dem ausgeblendeten Dateifenster jetzt einen Rang, der über dem des
sichtbaren liegt, statt wie vorher `Quellen::default()`. Drei neue kommen dazu.

**Der Untergrenzen-Abschnitt ist vollständig.** 36 Dateien unter
`crates/krk-ui/src/appkit/` rekursiv gezählt, `blaetter/` eingeschlossen; 34
tragen `# Ab welchem macOS die angesprochenen Klassen stehen`, ohne ihn sind
die zwei begründeten Ausnahmen `koordinaten.rs` und `mod.rs`.

**Die vier Abnahmekommandos laufen.** `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo build` und
`cargo test --workspace` je Exit 0; 457 Proben im Binärziel `krk`, wie der
Datensatz sagt.

## Befunde

### Deckung und Anzeige der Vorschau

**1. Die Deckungszusage gilt nicht innerhalb eines Elements, das Zeichen
geliefert hat.** Gewicht mittel.
`issues/260812-1920_o_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md`

Der Modulkopf sagt zu, zwei Sätze träfen „zusammen jedes Byte"
(`markdown.rs:56-86`). Es gibt einen dritten Fall, den beide verfehlen:

```
"- [ref]: http://a.example\n"              -> "• "
"> Zitat\n>\n> [ref]: http://a.example\n"  -> "Zitat"
```

Satz 1 (`luecke_bis`, `markdown.rs:515`) kehrt bei `!self.offen.is_empty()`
sofort zurück, Satz 2 (`schliessen`, `markdown.rs:618`) greift nur bei
`laenge == 0`, und der Listenpunkt hat sein Merkzeichen geschrieben. Die
Prämisse hinter Satz 1 — „hat es Zeichen geliefert, sind die Luecken darin
seine Auszeichnungszeichen" — trägt für einen Verweis und nicht für einen
Containerblock. Der Verlust selbst ist nicht neu (vor `a9e1149` derselbe), die
Zusage, dass es ihn nicht mehr gibt, schon.

**2. In einer losen Liste steht das Merkzeichen allein auf seiner Zeile.**
Gewicht hoch.
`issues/260812-1920_o_in-einer-losen-liste-steht-das-merkzeichen-allein-auf-seiner-zeile.md`

```
"- eins\n\n- zwei\n"    -> "• \n\neins\n\n• \n\nzwei"
"- Punkt\n\n  > Zitat\n" -> "• \n\nPunkt\n\nZitat"
```

Vor `a9e1149` lautete die erste Ausgabe `"eins\n\nzwei"` — eine
Verschlechterung dieses Turns. Bei einer losen Liste liefert die Kiste ein
`Tag::Paragraph`, `behandlung` macht daraus einen Block mit `ABSATZABSTAND`
(`markdown.rs:289-292`), und `trennen(2)` löst zwischen Merkzeichen und Text
aus, weil `punkt_oeffnen` das Zeichen schon geschrieben hat
(`markdown.rs:578-589`). Die `Listenzeile` deckt danach drei Absätze statt
einem. Keine der Listenproben in `markdown.rs` benutzt eine lose Liste.

**3. Eine Auszeichnung in einer Überschrift verliert deren Schriftgröße.**
Gewicht mittel.
`issues/260812-1920_o_eine-auszeichnung-in-einer-ueberschrift-verliert-deren-schriftgroesse.md`

`# Titel **fett** danach` liefert `Ueberschrift{1}(0,17)` gefolgt von
`StarkeBetonung(6,4)`; die Schleife in `anwenden` setzt zuerst
`boldSystemFontOfSize(grundgroesse * 1.7)` und danach
`boldSystemFontOfSize(grundgroesse)` (`textmerkmale.rs:206-215`). „fett"
verliert 41 Prozent seiner Höhe. Dasselbe für Code und Kursiv in einer
Überschrift.

**Damit beantwortet sich die Frage nach dem offenen Überschneidungsdatensatz:
er beschreibt nicht vollständig, was noch fehlt.** Sein Absatz „Was fehlt:
Punkt 2" nennt zwei Paarungen, „Fett **und** kursiv oder feste Schrift **und**
fett", und keine Überschrift. Der dort genannte Weg — `NSFontDescriptor`
beziehungsweise `applyFontTraits:range:` — legt **Schnitte** zusammen und
nicht **Größen** und behebt diesen Fall folglich nicht. Punkt 1 und Punkt 3 des
Datensatzes sind dagegen nachgeprüft erledigt: der Kommentar sagt jetzt, was
gilt, und `Offen::rang` macht die Ordnung bei gleichem Bereich total
(`markdown.rs:387, 663-670`).

### Die Datensätze zur Rücknahme von Schritt 11

**4. Die Directive des aktiven Circles sagt weiterhin eine blätterbare
Statuszeile zu.** Gewicht mittel.
`issues/260812-1920_o_die-directive-des-aktiven-circles-sagt-weiterhin-eine-blaetterbare-statuszeile-zu.md`

`_t_circle.md:14` trägt „**Fünftens zieht die Statuszeile über die volle
Fensterbreite und lässt sich nach rechts blättern.**" Der Code ist
zurückgenommen, der Entscheid sagt „C5.10 ist damit überholt", der Turnlog
desselben Datensatzes weiß es (`:171`) — die Directive nicht. Sie steht nicht
in einem der Speicher, für die `CLAUDE.md` die Ausnahme „Aufzeichnungen eines
Standes behalten ihren damaligen Marker" zieht, und sie ist an diesem Circle
schon einmal während der Runde geändert worden.

**5. Dieselbe verbleibende Arbeit steht zweimal, einmal offen und einmal
zurückgestellt.** Gewicht mittel.
`issues/260812-1920_o_dieselbe-verbleibende-arbeit-steht-zweimal-einmal-offen-und-einmal-zurueckgestellt.md`

Der offene Überschneidungsdatensatz bleibt allein wegen Punkt 2 offen; genau
diesen Punkt hat der Nutzer als `260812-1851_d_…` zurückgestellt. Keiner nennt
den anderen (`grep 260812-1851` im offenen liefert nichts). Der `find`-Lauf
über `*_o_*.md`, den `CLAUDE.md` als den Weg zum Stand nennt, liefert damit
Arbeit, die der Nutzer am selben Tag vertagt hat.

**6. Ein `Superseded by:`-Verweis zeigt auf einen nicht mehr vorhandenen
Dateinamen.** Gewicht gering.
`issues/260812-1920_o_ein-verweis-im-ueberholten-statuszeilen-datensatz-zeigt-auf-einen-nicht-mehr-vorhandenen-dateinamen.md`

`260812-1105_s_…` nennt seinen Nachfolger als `260812-1809_a_…`; die Datei
heißt seit `df4ec00` `260812-1809_i_…`. Der Schwesterdatensatz
`260812-1000_s_…` nennt seinen Nachfolger mit dem heutigen Marker, löst also
auf. Zwei Konventionen in einem Speicher.

## Was quer liegt

**Die beiden Behebungen sind gegeneinander gelaufen, und niemand hat es
gemessen.** Der Verlust-Datensatz hängt seine Auffangregel daran, ob ein
Element Zeichen geliefert hat. Der Listen-Datensatz lässt den Listenpunkt ein
Merkzeichen schreiben — also Zeichen liefern. Damit fällt der Listenpunkt aus
dem zweiten Deckungssatz heraus, und Befund 1 ist genau das. Beide Datensätze
sind einzeln nachgemessen worden, keiner gegen den anderen. Das ist die
Lehre für den nächsten Reparatur-Turn: bei zwei Behebungen in derselben Datei
gehört eine Messung ihrer Schnittmenge dazu.

**Drei der sechs Befunde betreffen keinen Code, sondern die Datensätze.** Die
Rücknahme eines Planschritts erzeugt Buchhaltung an mehr Stellen, als der
Entscheid nennt: der Entscheid selbst, der überholte Vorgänger, die Directive,
der Turnlog und die Datei. Vier davon sind gezogen, die Directive nicht.

**Das Muster „ein Datensatz nennt zwei Fälle, gemessen sind drei" kommt
zweimal vor** — bei den Schriftschnitten (Befund 3) und bei der Deckung
(Befund 1). Beide Male ist die Zahl der gemessenen Fälle zur Grenze der
Aussage geworden, statt die Aussage über die Fälle hinaus zu formulieren.

## Reihenfolge

**Vor dem Abschluss der Runde:**

1. Befund 2, die lose Liste. Sie ist eine Verschlechterung dieses Turns, sie
   ist auf den ersten Blick als kaputt zu erkennen, und C4.2 sagt Listen zu.
2. Befund 4, die Directive. Der Circle wird an ihr gemessen.

**Vor dem nächsten Reparatur-Turn:**

3. Befund 1, mindestens der erste Punkt: die Zusage im Modulkopf berichtigen.
   Ob die Lücke geschlossen wird, ist eine zweite Frage.
4. Befund 5, die doppelte Buchführung. Sie kostet sonst den nächsten Lauf
   über die offenen Punkte.

**Aufräumen:**

5. Befund 3, die Beschreibung dessen, was bei den Schriftschnitten fehlt. Die
   Behebung selbst ist zurückgestellt; die Beschreibung ist es nicht.
6. Befund 6, der tote Verweis.

## Belege

- Prüfprogramm: `markdown.rs` aus `df4ec00` und aus `94a81bd`, je unverändert
  kopiert, neben einem Stummel für die acht Typen aus `crate::hervorhebung`,
  gegen `pulldown-cmark 0.13`.
- 300 000 zufällige Quellen aus 31 Bausteinen, geprüft auf Absturz,
  Längenabweichung, Bereichsüberschreitung und Zeichenzuwachs.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`: je Exit 0, 457 Proben im Binärziel.

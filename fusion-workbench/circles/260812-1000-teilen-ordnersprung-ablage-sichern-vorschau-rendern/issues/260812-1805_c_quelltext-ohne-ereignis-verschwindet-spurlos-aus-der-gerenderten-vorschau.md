Quelltext, der kein Ereignis mit Zeichen erzeugt, verschwindet spurlos aus der gerenderten Vorschau

---

Das dritte Abnahmekriterium von C4 sagt zu: „Alles außerhalb dieses Umfangs
erscheint als der Quelltext, der dasteht." Die Auffangregel in
`crate::markdown` hängt an einem **Ereignis** des Zerlegers. Quelltext, zu dem
`pulldown-cmark` gar kein Ereignis liefert oder ein Ereignis ohne Zeichen,
fällt durch sie hindurch und erscheint überhaupt nicht — weder gerendert noch
als sein Quelltext.

---

**Zwei Fälle, beide am Baum gemessen** (mit `markdown::rendern` aus
`crates/krk-ui/src/markdown.rs:104`, unverändert in ein Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`):

1. **Eine Verweisdefinition erzeugt kein einziges Ereignis.**

   ```
   Quelle : "Text davor.\n\nSiehe [den Text][ref] hier.\n\n[ref]: https://example.com \"Titel\"\n"
   Ausgabe: "Text davor.\n\nSiehe den Text hier."
   ```

   Die Zeile `[ref]: https://example.com "Titel"` ist aus der Anzeige
   verschwunden. Der Zerleger verbraucht sie beim Auflösen des Verweises und
   meldet sie nicht; die Schleife in `rendern` (`markdown.rs:105-142`) bekommt
   nichts, worauf die Auffangregel greifen könnte.

   Der schärfste Fall ist eine Datei, die nur aus Definitionen besteht:

   ```
   Quelle : "[ref]: https://example.com\n[zwei]: https://b.example\n"
   Ausgabe: ""
   ```

   Die Vorschau zeigt eine leere Fläche für eine Datei mit Inhalt.

2. **Ein Verweis ohne Text erzeugt ein Ereignis ohne Zeichen.**

   ```
   Quelle : "Siehe [](https://example.com) dort.\n"
   Ausgabe: "Siehe  dort."
   ```

   `Zerlegung::schliessen` (`markdown.rs:632-660`) trägt bei
   `laenge == 0` nichts ein, und geschrieben hat der Verweis auch nichts. Die
   23 Zeichen `[](https://example.com)` sind weg.

**Warum das mehr ist als eine Schönheitsfrage.** Der Plan begründet die Wahl
von `pulldown-cmark` gegen `syntect` ausdrücklich damit, dass bei jenem fremde
Sprachdefinitionen entschieden, welche Zeichen verschwinden: „ein fälschlich
ausgeblendetes Zeichen ist eine falsche Auskunft über den Inhalt einer Datei"
(Wurzel-`Cargo.toml`, Begründung an der Versionsangabe; Plan,
`## Womit die Vorschau Markdown zerlegt`). Genau diese Wirkung tritt hier ein,
nur an einer anderen Stelle.

**Warum die Totalitätszusage sie nicht abdeckt.** Der Modulkopf von
`markdown.rs:41-47` und die `Decidability`-Zeile des Plans sagen, die
Fallunterscheidung sei total, weil alles außerhalb des Umfangs als sein
Quellbereich erscheine. Die Totalität gilt über `Event` und `Tag`. Sie gilt
**nicht** über die Zeichen der Datei: es gibt Quellbytes, die in keinem
Ereignis vorkommen, und diese Lücke ist in Plan und Modulkopf nicht benannt.

**Zwei verschiedene Ursachen, ein Kriterium.** Fall 1 hat kein Ereignis, an das
sich eine Regel hängen ließe — die Deckung müsste über die Quellbereiche der
gelieferten Ereignisse laufen (was zwischen dem Ende eines Bereichs und dem
Anfang des nächsten liegt, ist wörtlich auszugeben). Fall 2 ist eine Zeile in
`schliessen`: ein Element ohne Zeichen gibt seinen Quellbereich aus, statt zu
verschwinden. Beide gehören in eine Antwort, sonst wird die halbe gebaut.

**Gewicht:** mittel. Kein Absturz und kein Datenverlust, aber die Vorschau gibt
über den Inhalt einer Datei falsche Auskunft, und der Fall mit den
Verweisdefinitionen kommt in jeder README vor, die Verweise in Kurzform führt.

**Herkunft:** Circle der Runde 6, Planschritt 8 (C4.3).

---

**Resolved 260812** — beide gemessenen Faelle nachgeprueft und behoben, mit
**einem** Mechanismus statt zweier Sonderregeln.

**Nachgemessen.** Alle drei Ausgaben des Datensatzes stimmen am Baum genau so,
wie er sie nennt: `"Text davor.\n\nSiehe den Text hier."`, `""` und
`"Siehe  dort."`.

**Der Mechanismus ist ein Stand in der Quelle und keine Liste von
Ereignisarten.** `Zerlegung::gelesen` (`crates/krk-ui/src/markdown.rs`) haelt,
bis wohin die Quelle abgetragen ist. Daran haengen zwei Saetze, die zusammen
jedes Byte treffen, und keiner von beiden fragt nach der Art eines Ereignisses:

1. **Auf Dokumentebene** — also wenn `Zerlegung::offen` leer ist — gibt
   `luecke_bis` vor jedem Ereignis heraus, was seit dem letzten Stand
   ungelesen blieb; nach dem Durchgang ebenso bis `str::len`. Eine Luecke aus
   reinem Leerraum faellt weg, weil die Abstaende zwischen den Bloecken schon
   `absetzen` rechnet.
2. **Innerhalb eines Elements** deckt das Element seinen ganzen Quellbereich.
   Hat es Zeichen geliefert, sind die Luecken darin seine
   Auszeichnungszeichen und gehoeren weg. Hat es **kein** Zeichen geliefert,
   gibt `schliessen` seinen Quellbereich woertlich heraus.

Der erste Satz fragt allein, ob ein Element offen ist, der zweite allein, ob
die Laenge null ist. Fall 1 des Datensatzes faellt unter den ersten, Fall 2
unter den zweiten; eine kuenftige Fassung von `pulldown-cmark`, die ein Element
anders meldet, aendert daran nichts.

**Die Kehrseite ist mitgeprueft.** Ohne die Grenze „nur auf Dokumentebene"
truege die Regel das `[` und das `][ref]` eines Verweises in Kurzform wieder in
den Text. Die Probe `die_zeichen_eines_gerenderten_elements_bleiben_weg` haelt
das fest.

**Neue Proben in `crates/krk-ui/src/markdown.rs`**, je eine fuer jeden
gemessenen Fall: `eine_verweisdefinition_bleibt_als_ihr_quelltext_stehen`,
`eine_datei_aus_lauter_verweisdefinitionen_bleibt_sichtbar`,
`ein_verweis_ohne_text_erscheint_als_sein_quelltext`, dazu die Gegenprobe
oben. Der Abschnitt `# Die Deckung: kein Quellbyte faellt heraus` im Modulkopf
schreibt die Zusage auf und benennt die Luecke, die die Totalitaetszusage ueber
`Event` und `Tag` offenliess.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0.

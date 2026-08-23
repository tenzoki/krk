Der Rundweg erhebt den Fokus ein zweites Mal, gegen die Zusage der einen Erhebung

---

`Anwendungsdelegierter::editor_rundweg` (`crates/krk-ui/src/appkit/anwendung.rs:7025-7034`) fragt
`self.fokus()` selbst, obwohl der Wert acht Zeilen über dem Zweig, der es ruft, schon erhoben in
der Hand liegt. Genau diesen Griff verbietet der Kommentar an der Stelle, die ihn hält.

---

**Aus dem Baum gelesen.** `inference:` Am laufenden Bündel dürften die beiden Erhebungen heute
dieselbe Antwort geben; der Befund ist eine Verletzung einer schriftlichen Zusage, kein
gemessener Auseinanderlauf.

## Die Zusage

`crates/krk-ui/src/appkit/anwendung.rs:2994-2997`, unverändert seit vor diesem Commit:

```rust
// Derselbe Wert, jetzt als Adresse und nicht mehr als Vorbehalt; siehe
// den Modulkopf und `Self::lage`. Ein zweites `self.fokus()` waere eine
// zweite Erhebung desselben Augenblicks.
let fokus = lage.fokus;
```

Und an `Self::lage` (`anwendung.rs:2896-2900`): „Zwei Erhebungen desselben Augenblicks koennten
auseinanderlaufen; eine kann es nicht."

## Der Griff

`anwendung.rs:7025-7034`:

```rust
fn editor_rundweg(&self) -> bool {
    let Some(weg) = rundweg(self.fokus()) else {
```

`self.fokus()` ruft `self.fokus_bei(self.schluesselfenster())` — also eine zweite Abfrage von
`keyWindow` und ein zweiter Gang durch den Ansichtsbaum, nachdem `zulaessigkeit::zulaessig`
denselben Befehl bereits auf `lage.fokus` zugelassen hat.

## Drei Geschwister im selben `match` machen es anders

Alle drei reichen den einmal erhobenen Wert als Argument durch:

- `Kommando::TabSchliessen => self.tab_schliessen(fokus)` (`anwendung.rs:3198`)
- `Kommando::Teilen => self.teilen(fokus)` (`anwendung.rs:3226`)
- `andere => self.bereichskommando(fokus, andere)` (`anwendung.rs:3228`)

`bereichskommando` schreibt die Regel sogar aus (`anwendung.rs:3232-3235`): „**Keine zweite
Fokusabfrage.** Der Wert kommt aus der einen Abfrage in [`Self::kommando_ausfuehren`] und
beantwortet hier eine andere Frage: nicht, **ob** der Befehl wirkt … sondern **wohin** er geht."

Der Rundweg beantwortet genau dieselbe zweite Frage und ist der einzige Zweig, der sie mit einem
eigenen Griff beantwortet.

## Ein Nebenbefund derselben Wurzel

Der Doc-Kommentar an `Self::fokus_bei` (`anwendung.rs:5670-5673`) sagt: „[`Self::fokus`] bleibt
fuer die **fuenf** uebrigen Aufrufer stehen, die den Wert nicht schon in der Hand haben."

Vor `28cbb7b` traf die Zahl zu: `git show a8be186:crates/krk-ui/src/appkit/anwendung.rs |
grep -n 'self\.fokus()'` liefert fünf Aufrufe (Zeilen 1331, 1876, 4300, 6423, 6893; die zwei
weiteren Treffer stehen in Kommentaren). Heute sind es sechs (1353, 1898, 4334, 6474, 6966,
7026). Die Zahl ist mit diesem Commit falsch geworden — und sie wird von selbst wieder richtig,
sobald der Griff verschwindet.

## Empfehlung

`editor_rundweg` nimmt `fokus: Fokus` als Argument, und der Zweig ruft
`self.editor_rundweg(fokus)`. Das ist eine Zeile in jede Richtung, stellt den Befehl neben seine
drei Geschwister und macht den Nebenbefund gegenstandslos, ohne dass jemand eine Zahl nachziehen
muss.

**Schwere:** Medium. Kein beobachteter Fehler, aber die Verletzung einer Zusage, die dieser Baum
an drei Stellen schriftlich führt, in dem einen Zweig, der sie am nötigsten hat: liefen die
beiden Erhebungen auseinander, öffnete `cmd+e` eine Datei, wo es den Editor schließen sollte.

**Filed by:** coderev

---

In Arbeit: 260823-1137 durch coder. `Anwendungsdelegierter::editor_rundweg` nimmt
`fokus: Fokus` als Argument, der Zweig ruft `self.editor_rundweg(fokus)`, und der Rumpf
gibt den Wert an `rundweg` weiter statt `self.fokus()` ein zweites Mal zu rufen. Der
Doc-Kommentar traegt jetzt denselben Absatz „Keine zweite Fokusabfrage" wie die drei
Geschwister. Der Nebenbefund ist damit gegenstandslos: `self.fokus()` hat wieder genau
fuenf Aufrufer (Zeilen 1353, 1898, 4334, 6474, 6966), und die Zahl an `fokus_bei` stimmt
ohne Nachzug. Bleibt zum Schliessen mit dem Commit.

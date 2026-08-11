Der Auffangzweig in `wirkung` ist erreichbar: `bereich` und `wirkung` fragen nicht dasselbe

---

`crates/krk-ui/src/belegungsausgabe.rs:266` trägt `_ => ""`. Der Kommentar darüber
(`:258-265`) nennt den Zweig unerreichbar und begründet es so: `markdown` laufe zuvor durch
`nach_bereichen`, und das breche laut ab, sobald `belegungsmodell::bereich` eine Kennung nicht
einordnen könne — „also fuer jede Kennung, die weder ein Kommando traegt noch eine der sechs
oben ist".

**Die Begründung trägt nicht.** Die beiden Stellen stellen zwei verschiedene Fragen, und für
eine Funktion mit `gehalten_von` auf einer Kommandokennung fallen die Antworten auseinander.

---

**Schwere:** Mittel
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
**Betroffen:** `crates/krk-ui/src/belegungsausgabe.rs` (`wirkung`), im Verbund mit
`crates/krk-ui/src/belegungsmodell.rs` (`bereich`, `nach_bereichen`) und
`crates/krk-core/src/tasten/belegung.rs` (`Funktion::kommando`)
**Domain:** code

## Die beiden Fragen

`belegungsmodell::bereich` (`belegungsmodell.rs:148-150`) fragt über **`Kommando::aus_kennung`**:

```rust
pub fn bereich(kennung: &str) -> Option<Funktionsbereich> {
    if let Some(kommando) = Kommando::aus_kennung(kennung) {
        return Some(bereich_des_kommandos(kommando));
    }
    match kennung { /* die sechs zugestellten */ _ => None }
}
```

`belegungsausgabe::wirkung` (`belegungsausgabe.rs:213`) fragt über **`Funktion::kommando`**:

```rust
if let Some(kommando) = funktion.kommando() {
    return kommando.wirkungsbereich().beschriftung();
}
```

Und `Funktion::kommando` (`crates/krk-core/src/tasten/belegung.rs:757-762`) ist **nicht**
`Kommando::aus_kennung`, sondern trägt die Zustellerregel mit:

```rust
pub fn kommando(&self) -> Option<Kommando> {
    if self.gehalten_von.is_some() {
        return None;
    }
    Kommando::aus_kennung(&self.kennung)
}
```

Der Doc-Kommentar von `wirkung` nennt diesen Unterschied ausdrücklich als Vorzug — und genau
er reißt das Loch: `bereich` sieht `gehalten_von` nicht, `wirkung` schon.

## Der erreichbare Fall, am Baum gemessen

Eine `keymap.toml` des Nutzers, die einer Funktion **mit** Kommando einen Zusteller gibt:

```toml
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["f5"]
gehalten_von = "menue"
```

Gemessen am 260811-0955 gegen `krk-core` als Abhängigkeit einer Wegwerfkiste, damit kein
Prüfcode im Baum entsteht:

```
angenommen: kommando()=None gehalten_von=Some("menue") aus_kennung=Some(Kopieren)
```

Damit gilt der Reihe nach:

1. `Belegung::vom_nutzer` **nimmt die Datei an.** `Belegung::bauen` (`belegung.rs:984-1015`)
   prüft allein `eintrag.id` gegen den Wortschatz der Auslieferungsbelegung und übernimmt
   `gehalten_von` unverändert; `Eintrag` liest das Feld aus dem TOML (`belegung.rs:1225-1230`).
   `konflikte` (`:947-965`) vergleicht nur innerhalb desselben Zustellers, also entsteht auch
   kein Konflikt.
2. `bereich("kopieren")` findet über `Kommando::aus_kennung` den Bereich `Dateioperationen`.
   **`nach_bereichen` bricht nicht ab.**
3. `wirkung(funktion)` bekommt von `funktion.kommando()` ein `None`, fällt in den `match` über
   die Kennung, trifft keinen der sechs Zweige und landet in `_ => ""`.

Die Funktion steht damit in der Datei, im richtigen Abschnitt, mit ihren Kombinationen — und
mit **leerer** dritter Zelle.

## Warum das mehr ist als ein unerreichbarer Zweig

**Die leere Zelle hat in dieser Datei bereits eine Bedeutung**, und zwar eine bewusst gesetzte:
`text_alles_auswaehlen` bleibt leer, weil die Messung aus S1 die Ableitung gebrochen hat
(`issues/260811-0930_*_die-ableitung-textfelder-und-editor-bricht-fuer-alles-auswaehlen-*.md`).
Der Auffangzweig liefert denselben Ausgang für einen ganz anderen Sachverhalt: „hier ist nichts
entschieden" und „hier hat niemand nachgesehen" sind in der Ausgabe nicht mehr unterscheidbar.
Das ist der Fall, den der Spec unter `## Was die Abnahme mitentscheidet` als teuersten benennt.

**Die Vollständigkeitsprobe fängt es nicht.**
`jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` (`belegungsausgabe.rs:681`) läuft über
`Belegung::auslieferung()` und nicht über eine Nutzerbelegung. Ihr Doc-Kommentar sagt trotzdem:
„Eine Funktion, die weder das eine noch das andere traegt, faengt diese Probe, bevor sie eine
leere Zelle in der Datei erzeugt." Für eine Belegung des Nutzers gilt dieser Satz nicht.

## Was der Auffangzweig hier *nicht* kostet

Der Vollständigkeit halber, weil die Projektregel aus `CLAUDE.md` hier nicht greift: der `match`
in `wirkung` läuft über `&str` und nicht über eine Aufzählung dieses Projekts. Ein `&str`-`match`
ist in Rust **nie** ohne Auffangzweig übersetzbar. Es geht hier also nicht um eine verlorene
Bauunterbrechung — die gäbe es an dieser Stelle ohnehin nicht —, sondern allein darum, was der
Zweig tut, wenn er greift.

## Behebungsrichtungen

Zwei Wege, und der erste ist der integrale.

**a) Die beiden Fallunterscheidungen deckungsgleich machen.** `wirkung` fragt dieselbe Frage wie
`bereich`, also `Kommando::aus_kennung(funktion.kennung())`. Danach ist der Auffangzweig
tatsächlich durch den lauten Abbruch in `nach_bereichen` gedeckt, und er kann dann so laut
abbrechen wie dieser, statt still eine leere Zelle zu liefern. Der Preis: die Zusage hängt
wieder daran, dass `Kommando::KENNUNGEN` die sechs Textbefehle nicht nennt — genau das, wogegen
der Doc-Kommentar von `wirkung` heute argumentiert. Diese Abwägung ist eine Nutzerfrage und
keine Entscheidung dieses Datensatzes.

**b) Den Zweig eine eigene, unterscheidbare Auskunft geben.** Nicht `""`, sondern ein Text, der
nicht mit der bewussten Leerzelle verwechselbar ist. Löst das Kernproblem — die Verwechslung —
ohne die Frage aus a) zu beantworten, lässt die Ungleichheit der beiden Fallunterscheidungen
aber stehen.

**Ein Abbruch (`panic!`) ohne a) wäre falsch:** er brächte KRK an einer vom Nutzer von Hand
geschriebenen, formal zulässigen `keymap.toml` zum Absturz.

Was in jedem Fall mitgeht: der Kommentar `belegungsausgabe.rs:258-265` behauptet heute eine
Unerreichbarkeit, die der Programmtext nicht hergibt, und der Satz im Doc-Kommentar von
`jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` reicht weiter als die Probe.

---
Nachtrag 260811-1045: Weg b) ist gewaehlt und gebaut, der Datensatz bleibt offen.

**Was behoben ist.** Der Zweig traegt nicht mehr die leere Zelle, sondern
`NICHT_EINGEORDNET = "(von KRK nicht eingeordnet)"`. Damit fallen "hier ist nichts entschieden"
und "hier hat niemand nachgesehen" in dieser Datei nicht mehr in derselben Zelle zusammen; die
leere Zelle bleibt `text_alles_auswaehlen` allein vorbehalten. Der Text nennt bewusst keinen Ort,
steht als einzige Zelle der Spalte in Klammern und benutzt mit "eingeordnet" das Wort der
Rechnung statt des Gegenstands. Eine Probe misst den Fall, statt ihn zu behaupten
(`eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig`), und der Kommentar an
`belegungsausgabe.rs` traegt die Messung vom 260811-0955 im Wortlaut statt der widerlegten
Unerreichbarkeitsbehauptung.

**Was offen bleibt, und dieser Datensatz bleibt deswegen offen.** Die Ungleichheit selbst steht
unveraendert: `bereich` fragt ueber `Kommando::aus_kennung` und sieht `gehalten_von` nicht,
`wirkung` fragt ueber `Funktion::kommando()` und sieht es. Der Nutzer hat am 260811-1005
ausdruecklich Weg b) gewaehlt und die beiden anderen abgelehnt — die Lesestellen deckungsgleich
zu machen, was die Kopplung an `KENNUNGEN` zurueckbraechte, und eine Abweisung an der
Eingangsstelle ueber einen vierten `Belegungsfehler`. Beide bleiben moeglich; die Wahl war, jetzt
nur die Verwechslung zu beheben.

Kein `panic!` gebaut: es stuerzte an einer formal zulaessigen, von Hand geschriebenen
`keymap.toml` ab.

Die neun berichtigten Zahlen stehen weiter unverankert, und die benannte Ursache trägt keinen eigenen Datensatz

---

`ea5f23e` hat neun Prosastellen in `crates/krk-ui/src/appkit/menue.rs` und
`crates/krk-ui/src/belegungsausgabe.rs` von 79/73 auf 84/78 gebracht. Die Zahlen sind
richtig, dreifach nachgezählt:

```
grep -c '^id = ' resources/default-keymap.toml            -> 84
grep -c '^gehalten_von = ' resources/default-keymap.toml  ->  6
awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs \
  | grep -cE '^    [A-Z][A-Za-z0-9]*,'                    -> 78
```

Der zugehörige Datensatz
`shared/issues/260813-1345_c_fuenf-stellen-nennen-79-funktionen-und-73-mit-kommando-die-belegung-fuehrt-82-und-76.md`
ist als `_c_` geschlossen und sagt in seinem eigenen Abschluss:

> **Der Befund ist damit behoben und die Ursache nicht.** Die Zahlen stehen weiter von Hand
> in der Prosa und veralten mit der nächsten Runde, die eine Funktion hinzufügt.

Die Ursache ist damit benannt und trägt keinen eigenen Datensatz. Ein geschlossener
Datensatz fällt aus jeder Suche nach offener Arbeit heraus; die Feststellung steht in einer
Datei, die niemand mehr aufschlägt.

---

**Schwere:** mittel. Kein Verhalten, kein Bau. Der Schaden ist die Wiederkehr: die Zahlen
sind in diesem Baum viermal in vier Tagen falsch geworden, und beim letzten Mal noch
bevor jemand die vorige Berichtigung fahren konnte.
**Gefunden von:** coderev, Durchsicht des Bereichs `cd0b5b7..093a6f4`
**Betroffen:** `crates/krk-ui/src/appkit/menue.rs:128,799,867`,
`crates/krk-ui/src/belegungsausgabe.rs:45,48,56,256,725,726`
**Domain:** code

## Warum es der Rede wert ist

Die Familie ist belegt und wächst:

- `shared/issues/260812-2253_c_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`
- `shared/issues/260812-1438_c_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`
- `shared/issues/260813-1345_c_…-die-belegung-fuehrt-82-und-76.md` — Zielzahl beim
  Aufschreiben 82/76, beim Abgleich der Runde 9 schon 83/77, beim Beheben 84/78
- `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0810_o_die-zahl-39-im-kopf-der-belegungsdatei-steht-im-praesens-und-ist-ungeprueft.md`
  — offen, dieselbe Sorte

`CLAUDE.md` hat für dieselbe Zahl bereits die Konsequenz gezogen und führt sie nicht mehr,
sondern nennt das `awk`, mit dem man sie zählt. Im Code steht sie neunmal.

**Die Runde hat den einen Fall dieser Familie mit eigenem Datensatz weitergetragen** — den
Hilfetext, `shared/issues/260815-1436_o_…` — und den anderen nicht. Die Ungleichbehandlung
ist der eigentliche Befund.

## Der Ort steht schon fest

Beide, der Datensatz `260813-1345` und der Modulkopf von `belegungsausgabe.rs:45-49`,
nennen dieselbe Probe: `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`
(`belegungsausgabe.rs:732-756`). Sie zählt bereits:

```rust
let mut mit_kommando = 0;
for funktion in belegung.funktionen() { … mit_kommando += 1; … }
assert_eq!(mit_kommando, Kommando::KENNUNGEN.len(), "…");
```

Die Beziehung zwischen Belegung und Aufzählung ist damit schon geprüft. Was fehlt, ist die
Brücke zu den Zahlen in der Prosa. Zwei Zeilen reichen:

```rust
assert_eq!(belegung.funktionen().len(), 84, "der Modulkopf und menue.rs nennen 84");
assert_eq!(mit_kommando, 78, "der Modulkopf und menue.rs nennen 78");
```

Damit hält der Bau an, sobald eine Funktion hinzukommt, und die Fehlermeldung nennt die
Stellen, die nachzuziehen sind — dasselbe Muster, das dieses Projekt für die vier
gewachsenen Aufzählungen schon fährt.

**Die Alternative wäre, die Zahlen aus der Prosa zu nehmen**, wie `CLAUDE.md` es getan hat.
Sie ist billiger und verliert eine Auskunft, die die Modulköpfe heute tragen: dass es
überhaupt genau sechs Ausnahmen gibt. Die Entscheidung zwischen beiden Wegen gehört dem
Nutzer.

## Abgrenzung

Die neun Zahlen sind heute richtig; dieser Datensatz behauptet nichts anderes. Er betrifft
allein die Verankerung. Nachgeprüft: `grep -n '\b79\b\|\b73\b'` findet in beiden Dateien
keine Fundstelle dieser Bedeutung mehr, und im übrigen Baum steht keine.

## Herkunft

Gemeinsamer Speicher. Die Zahlen sind älter als jede der betroffenen Runden und betreffen
eine projektweite Gewohnheit.

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und die Zahlen sind seit der Ablage ein weiteres Mal von Hand nachgezogen worden.** `crates/krk-ui/src/belegungsausgabe.rs:45,48,256,730` und `crates/krk-ui/src/appkit/menue.rs:128,799,867` tragen heute 85 und 79, wo sie bei der Ablage 84 und 78 trugen. Die einzige Zusicherung, die eine dieser Zahlen hält, ist unverändert `mit_kommando == Kommando::KENNUNGEN.len()` (`belegungsausgabe.rs:756-758`), und sie hält keine der sieben Prosastellen. Der Datensatz zählt damit den fünften Durchgang von Hand.

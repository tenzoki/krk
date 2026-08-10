# Die Laufzeitprobe bindet den Bau an die macOS-Version des prüfenden Geräts

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:563-583` (`keine_unbekannte_automatik_steht_an_der_textflaeche`)
**Cross-references:** `issues/260809-1650_c_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`, `issues/260810-0416_o_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`, Commit `f7ef6c5`, CLAUDE.md („Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26")

---

## Der Befund

Die Probe zählt zur Laufzeit die Selektoren der Form `set…Enabled:` an der
Klasse `NSTextView` auf und verlangt Mengengleichheit mit `ABGESCHALTET` ∪
`GEDULDET`:

```rust
assert_eq!(
    getragen, eingeordnet,
    "NSTextView traegt einen Schalter, den weder ABGESCHALTET noch GEDULDET kennt, oder \
     umgekehrt — wer ihn ergaenzt, beantwortet zuerst, ob er Zeichen aendert (C4)"
);
```

`getragen` kommt aus der **Laufzeit des prüfenden Geräts**, nicht aus dem SDK
und nicht aus dem Zielsystem. `cargo test` misst damit das macOS, auf dem es
gerade läuft. KRK sagt Unterstützung von macOS 15 bis macOS 26 zu. Zwei Folgen:

1. **Derselbe unveränderte Quelltext ist auf einem anderen macOS rot.** Fügt
   Apple einen Schalter hinzu, hält der Bau an, obwohl an KRK niemand etwas
   geändert hat. Das ist die erklärte Absicht der Probe und in dieser Richtung
   richtig.
2. **Er ist auch dann rot, wenn ein Schalter verschwindet.** Verliert eine
   spätere Fassung von `NSTextView` etwa `setIncrementalSearchingEnabled:` aus
   `GEDULDET`, schlägt dieselbe Zusicherung fehl. Dieser Fall gefährdet die
   Zusage aus C4 in keiner Weise: ein Schalter, den es nicht mehr gibt, kann
   keine Zeichen ändern.

## Warum der Fehlschlag heute nicht aussagekräftig ist

Die Meldung sagt „einen Schalter, den weder ABGESCHALTET noch GEDULDET kennt,
oder umgekehrt" und lässt damit offen, welcher der beiden grundverschiedenen
Fälle vorliegt. Der Leser findet den Unterschied erst, wenn er die beiden
`BTreeSet`-Ausgaben von `assert_eq!` von Hand gegeneinanderhält. Die beiden
Fälle verlangen entgegengesetzte Antworten:

```
  neu und uneingeordnet  ──> C4 ist offen: einordnen und ggf. abschalten
  eingeordnet und weg    ──> nichts ist offen: Eintrag streichen
```

## Was zu prüfen wäre

Die Richtungen zu trennen statt Mengen zu vergleichen:

- `getragen \ eingeordnet` ist der gefährliche Fall und gehört weiter in eine
  Zusicherung, die den Bau anhält, mit den Namen in der Meldung.
- `eingeordnet \ getragen` ist der harmlose. Ob er den Bau anhalten soll, ist
  eine Entscheidung und keine Selbstverständlichkeit: eine Meldung auf der
  Standardfehlerausgabe hielte den Hinweis, ohne eine grüne Reihe auf einem
  unterstützten System rot zu färben.

Gemessen am 260810-0416 auf macOS 15.7.7 (Build 24G720): `NSTextView` trägt
genau die zwölf Selektoren, die die beiden Aufstellungen führen. Die Probe ist
auf diesem Gerät grün; der Befund ist eine Aussage über die anderen
unterstützten Systeme, nicht über dieses.

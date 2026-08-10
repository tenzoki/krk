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

---
Resolved: Die Richtungen sind getrennt. Nur `getragen \ eingeordnet` hält den Bau
an und nennt die Namen; `eingeordnet \ getragen` ist ein Hinweis auf der
Standardfehlerausgabe und färbt keine grüne Reihe rot. Die Probe bleibt, aber sie
sagt jetzt zu, was sie halten kann.

**Die Probe bleibt — und die Entscheidung ist begründet, nicht stillschweigend.**
Der Datensatz stellte zur Debatte, ob eine Probe dieser Bauart überhaupt der
richtige Ort ist. Sie ist es, aber nur als **Stolperdraht**, nicht als
Vollständigkeitsbeweis. Drei Grenzen stehen dem entgegen, und keine davon ist zu
schließen; sie stehen deshalb jetzt ausformuliert im Modulkopf von
`crates/krk-ui/src/appkit/editor.rs`, damit der nächste Leser sie nicht
zurückschließen muss.

1. **Die Aufzählung ist zur Übersetzungszeit nicht erzwingbar.** Rust sieht die
   Kopfdateien des SDK nicht, `objc2` bildet keine Verfügbarkeitsgrenze ab, und
   `AnyProtocol` führt in `objc2` 0.6 keine Mitgliederliste — sonst wäre
   `NSTextInputTraits` der sachliche statt des namensbasierten Schnitts. Geprüft
   an `objc2-0.6/src/runtime/mod.rs:1045-1090`: `get`, `protocols`,
   `adopted_protocols`, `conforms_to`, `name` — kein
   `protocol_copyMethodDescriptionList`. Der Weg dorthin führte über rohes FFI
   und damit `unsafe`, das `krk-ui` außerhalb von `appkit/mod.rs` verbietet.
2. **Die Namensform ist nicht der Schnitt, den die Sache verlangt.** Das ist mit
   `260810-0416` von der anderen Seite belegt: zehn Paare tragen zwei Namen für
   eine Sache. "Alles, was den Textspeicher anfassen kann" ist an einem
   Selektornamen nicht entscheidbar.
3. **Sie misst das prüfende Gerät.** Das bleibt so, weil eine Laufzeitaufzählung
   nichts anderes messen kann. Der Modulkopf sagt es jetzt: eine Einstellung, die
   Apple in macOS 26 dazulegt, fällt erst dem auf, der auf macOS 26 prüft.

**Was die Zusage aus C4 trägt, sind deshalb nicht die Proben**, sondern die
sieben Zeilen in `textflaeche_bauen` und die Prüfung am laufenden Bündel. Auch
das steht jetzt im Modulkopf, an der Stelle, an der vorher die Probe als das
Haltende beschrieben war.

**Die zweite Folge des Befunds ist behoben.** Ein verschwundener Schalter hält
den Bau nicht mehr an:

```rust
let verschwunden: Vec<&str> = eingeordnet.difference(&getragen).map(String::as_str).collect();
if !verschwunden.is_empty() {
    eprintln!(
        "Hinweis: {verschwunden:?} steht in EINSTELLUNGEN, aber nicht mehr an NSTextView \
         dieses Systems. C4 ist davon nicht beruehrt — was es nicht gibt, aendert keine \
         Zeichen. Wer aufraeumt, streicht den Eintrag."
    );
}
```

**Die zweideutige Meldung ist fort.** Der gefährliche Fall nennt jetzt Zahl und
Namen, statt zwei `BTreeSet` gegeneinanderzuhalten. Beide Richtungen sind
gegengeprüft: ein aus der Aufstellung entfernter Eintrag bricht die Probe mit
`NSTextView traegt 1 Einstellung(en), die EINSTELLUNGEN nicht kennt:
["setContentType:"]`; ein Eintrag ohne Entsprechung an der Klasse läuft grün
durch und schreibt den Hinweis.

**`assert_eq!` auf Mengengleichheit ist damit fort**, und mit ihm der Grund, aus
dem die Meldung zweideutig war: die Probe stellt jetzt zwei Fragen statt einer.

Die neue Gesamtzahl acht ist eine Aufruferzaehlung ohne Kriterium und benennt zwei der acht falsch

---

`der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten`
(`crates/krk-ui/src/appkit/menue.rs:1207-1234`) trägt seit Turn 2 eine zweite Zusicherung:

```rust
let alle: usize = dateien
    .iter()
    .map(|(_, inhalt)| crate::quellbaum::aufrufstellen(inhalt, name))
    .sum();
assert_eq!(alle, 8, ...);
```

Zwei Einwände, und sie stehen unabhängig voneinander.

**Erstens: die Zahl acht sagt kein Abnahmekriterium zu.** Der Kopf von
`crates/krk-ui/src/quellbaum.rs:41-47`, im selben Commit geschrieben, sagt dazu:

> Eine Aufruferzaehlung steht deshalb nur dort, wo ein Abnahmekriterium die Zahl selbst zusagt,
> und nirgends als Stellvertreter fuer „es gibt keinen Doppelbau".

Die Drei ist zugesagt (C2.14, ein Ausführungsweg mit drei Fragern). Die Acht ist es nicht: sie
zählt drei Frager und fünf Weiterreichungen zusammen. Der Doc-Kommentar nennt als ihren Zweck
ausdrücklich „ein neunter macht die Probe rot" (`:1203-1205`), also genau den Dienst als
Stellvertreter, den der Absatz oben ausschließt. Der billigste Weg zurück ins Grüne, wenn eine
Fläche eine sechste Weiterreichung bekommt, ist das Hochzählen der Acht.

**Zweitens: die Erklärung stimmt für zwei der acht nicht.** Der Kommentar sagt „die drei oben
und fuenf Weiterreichungen an Tabelle, Leiste und Vorschau" (`:1201-1203`). Nachgezählt sind es

| Stelle | Was sie ist |
|---|---|
| `anwendung.rs:684`, `:985`, `:2491` | die drei Frager am Delegierten |
| `anwendung.rs:2787`, `:2792`, `:2819` | die drei Weiterreichungen aus `bereichskommando` |
| `anwendung.rs:5496`, `:5524` | `messhandlung` ruft `Tabellenquelle::kommando_ausfuehren` **direkt** |

Die letzten zwei sind keine Weiterreichungen. Sie stehen im Messmodus und gehen an
`Anwendungsdelegierter::kommando_ausfuehren` und damit an der Zulässigkeitsregel vorbei — das
ist gewollt, weil der Messlauf eine Handlung setzt und keinen Befehl auslöst, aber es ist
etwas anderes als eine Fortsetzung des einen Ausführungswegs. Wer die Acht später prüft, liest
die falsche Auskunft darüber, woraus sie sich zusammensetzt.

---

**Schwere:** gering. Die Probe ist grün und richtig gerechnet; sie ist brüchig gegen jede
zulässige Änderung, und ihre Beschriftung trifft zwei ihrer acht Fundstellen nicht.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0718

**Betroffen:** `crates/krk-ui/src/appkit/menue.rs:1188-1234`

**Domain:** code

## Vorschlag

Die zweite Zahl streichen und die erste ehrlich beschriften. Was sie leistet, sagt der
Datensatz `260813-0540_c_zwei-aufruferzaehlungen-…` selbst: sie benennt die drei Frager. Dass
ein vierter unter anderem Bindungsnamen ihr entginge, gehört als benannte Blindheit an den
Doc-Kommentar — Folgerung 3 aus dem Kopf von `quellbaum.rs`.

Wer die zweite Zahl behalten will, nimmt ihr die Rolle der Wache: die Erwartung auf die drei
Frager plus die drei Weiterreichungen begrenzen (also die zwei Messmodus-Aufrufe aus der
Zählung ausschließen, so wie `zulaessigkeit.rs` seine eigene Datei ausschließt) und im
Kommentar sagen, dass die Zahl eine Beobachtung ist und keine Zusage.

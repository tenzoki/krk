Die Probe zum einen Rufer des Weitergabehinweises liest drei von sechs Modulen und nicht den Zweig

---

`allein_der_unterbefehl_bundle_gibt_den_hinweis_aus` (`xtask/src/sign.rs:632-643`) hält die
Freihaltung von `release` fest:

```rust
let nadel = concat!("weitergabe", "hinweis(");
for (name, quelle) in [
    ("release.rs", include_str!("release.rs")),
    ("messen.rs",  include_str!("messen.rs")),
    ("bundle.rs",  include_str!("bundle.rs")),
] {
    assert!(!quelle.contains(nadel), "{name} gibt den Hinweis aus");
}
assert_eq!(include_str!("main.rs").matches(nadel).count(), 1);
```

Ihr Doc-Kommentar sagt zu, sie halte fest, „dass es genau einen gibt und wo er nicht liegt"
(`sign.rs:629-631`). Das hält sie nicht. Sie hält fest, dass **drei benannte** Dateien den
Namen nicht tragen und dass `main.rs` ihn **einmal** trägt.

---

**Schwere:** niedrig bis mittel. Kein Verhalten, kein Bau. Der Befund ist eine Probe, die
weniger trägt als ihr Kommentar zusagt, und die genau die Bewegung durchlässt, gegen die sie
geschrieben ist.
**Gefunden von:** coderev, Durchsicht des Bereichs `cd0b5b7..093a6f4`
**Betroffen:** `xtask/src/sign.rs:625-643`
**Domain:** code

## Die zwei Lücken

**Erstens: drei von sechs Geschwistern.** `xtask/src/` führt `bundle.rs`, `git.rs`,
`main.rs`, `messen.rs`, `release.rs`, `sign.rs`. Die Probe liest drei davon. `version.rs`
und `git.rs` stehen in keiner Schleife; ein Aufruf von dort liefe durch. Dass die beiden
heute keinen Grund hätten, den Hinweis auszugeben, ist wahr und ist keine Zusicherung —
genau diese Art Argument soll die Probe ja ersetzen. `sign.rs` selbst ist mit Grund nicht
dabei: dort steht die Funktion und dort rufen sie vier Proben.

**Zweitens, und das ist die tragende: der Zweig ist nicht festgeschrieben.** Die
Unterbefehlsverteilung steht in `main.rs:135-180`, und `release` erreicht sie über
`"release" => release::ausfuehren(&argumente[1..])` (`main.rs:161`). Wer die drei Zeilen
des Aufrufs (`main.rs:151-158`) aus dem `"bundle"`-Zweig in den `"release"`-Zweig
verschöbe, ließe die Zahl in `main.rs` bei 1 und die drei anderen Dateien bei 0 — die Probe
bliebe grün, und der Hinweis stünde an der Stelle, an der der Quelldatensatz ihn
ausdrücklich nicht haben will.

Der Satz aus
`shared/issues/260812-1628_c_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`
— „`release` bleibt strukturell frei, nicht über eine Abfrage" — stimmt für die eine
Hälfte: `release::ausfuehren` (`release.rs:155-191`) ruft `bundle::bauen` nachweislich
nicht, es setzt das Bündel über `bundle::vorbereiten`, `bundle::uebersetzen` und
`vorlage.zusammensetzen` selbst zusammen. Nachgezählt hat `bundle::bauen` genau zwei Rufer,
`main.rs:140` und `messen.rs:45`. Die andere Hälfte — dass der Hinweis nicht doch im
`release`-Zweig landet — ist nicht strukturell, sondern eine Position innerhalb eines
`match`, und die trägt keine Probe.

## Was zu tun wäre

Die Probe an den Zweig heften statt an die Datei. Zwei Wege:

1. **Textlich, in derselben Form wie jetzt:** den `"bundle"`-Zweig aus `main.rs` als
   Abschnitt herausschneiden (von `"bundle" =>` bis zum nächsten `=>` auf derselben Ebene)
   und die Nadel darin zählen. Bleibt eine Textprobe, hält aber die Stelle.
2. **Strukturell, und dann trägt sie von selbst:** den Zweigkörper in eine eigene Funktion
   ziehen — etwa `fn bundle_fahren() -> Result<(), Abbruch>` —, und die Probe zählt die
   Nadel in dieser Funktion. Dann ist der Ausgabeort ein benannter Ort und keine Position.

Die Erweiterung der Dateiliste auf alle Geschwister ist billig und sollte in beiden Fällen
mitlaufen; besser noch ein Verzeichnisdurchlauf statt einer Liste, die beim nächsten neuen
Modul stumm bleibt.

## Herkunft

Gemeinsamer Speicher. Betrifft `xtask` und den Bauweg des ganzen Projekts.

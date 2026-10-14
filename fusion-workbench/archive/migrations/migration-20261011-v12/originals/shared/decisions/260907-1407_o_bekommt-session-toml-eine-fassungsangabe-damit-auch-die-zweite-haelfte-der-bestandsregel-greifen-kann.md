# Bekommt `session.toml` eine Fassungsangabe, damit auch die zweite Hälfte der Bestandsregel greifen kann?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md` — der Entscheid, dessen eine Hälfte hier übrig bleibt; `260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md` — der Defekt, aus dem die Bestandsregel stammt; `crates/krk-core/src/ablage/sitzung.rs` (`Sitzung`), `crates/krk-core/src/ablage/pfade.rs` (`Datei::leerbefund`)

---

## Question

Die Bestandsregel der Ablage hat zwei Hälften. Die erste fragt „steht in der Datei ein
oberster Schlüssel, den der Leser nicht kennt" und wird mit
`#[serde(deny_unknown_fields)]` an der jeweiligen Struktur beantwortet. Die zweite fragt
„steht in der Datei überhaupt ein oberster Schlüssel" und wird je Datei in
`Datei::leerbefund` beantwortet.

Der Nutzerentscheid vom 260907 stellt `session.toml` streng — und bindet die Strenge
zugleich an die Bedingung, dass eine `session.toml` aus einer **späteren** Fassung von KRK
in einer früheren die Sitzung nicht kostet, auch nicht mit Meldung. Damit ist die zweite
Hälfte gebaut (`Datei::Sitzung` trägt `Leerbefund::Beschaedigt`) und die erste nicht:
`deny_unknown_fields` an `Sitzung` verwürfe genau die Datei, die die Bedingung schützt.

**Die Frage dahinter ist heute nicht entscheidbar.** Ein unbekannter oberster Schlüssel in
`session.toml` hat zwei Ursachen, die die Datei nicht auseinanderhält: ein Feld, das eine
spätere Fassung geschrieben hat, und ein Tippfehler des Nutzers, der die Datei nach C7 von
Hand liest und ändert. Beide sehen gleich aus. Solange das so bleibt, ist die erste Hälfte
für diese Datei nicht zu haben, und die Frage lautet, ob KRK dem Mechanismus die fehlende
Eingabe verschafft oder auf die Hälfte verzichtet.

## Options

1. **Es bleibt dabei: `Sitzung` bekommt nie `deny_unknown_fields`.** Die heutige Fassung
   wird zur Antwort erklärt.
   - Pro: Nichts zu bauen, nichts zu pflegen. Der Verlust, gegen den die Bestandsregel
     gebaut ist, trifft `session.toml` in seiner schweren Gestalt ohnehin nicht mehr: die
     Datei ohne obersten Schlüssel ist seit dem 260907 ein Befund, und ein einzelner
     unbekannter Schlüssel neben lauter bekannten kostet den Rest der Sitzung nicht,
     sondern nur sich selbst.
   - Contra: Ein vertippter oberster Schlüssel — `[fenstre]` statt `[[fenster]]` — bleibt
     still. Der Nutzer sieht seine Tabs verschwinden und bekommt keine Meldung darüber,
     warum. Genau diese stille Gestalt hat der Defekt `260820-2235` an `bookmarks.toml`
     gemessen.

2. **`session.toml` bekommt eine Fassungsangabe**, etwa ein oberstes Feld `fassung`, das
   KRK beim Schreiben setzt und beim Lesen gegen die eigene hält. Ein unbekannter
   Schlüssel in einer Datei mit **höherer** Fassung wird getragen, in einer mit gleicher
   oder niedrigerer ist er ein Befund.
   - Pro: Macht die undentscheidbare Frage entscheidbar, statt sie zu nähern. Danach ist
     `deny_unknown_fields` — oder was an seine Stelle tritt — für diese Datei zu haben,
     ohne die Rückwärtsrichtung zu bezahlen.
   - Contra: Eine `session.toml` aus der Zeit vor dem Feld trägt keine Fassung, und die
     Regel braucht dafür einen weiteren Zweig. Und `serde` kann die Fallunterscheidung
     nicht allein tragen: `deny_unknown_fields` ist eine feste Marke und kennt keinen
     Vorbehalt, also entstünde ein Leseweg, der die Fassung vor der Struktur liest. Das
     wäre eine zweite Stelle neben `Zugang::laden`, und der Ausgangsdatensatz schließt
     einen zweiten Mechanismus aus.

3. **Der unbekannte oberste Schlüssel wird gemeldet, ohne den Bestand zu kosten.** Der
   Ladeweg liest das Dokument zusätzlich als `toml::Table`, hält dessen oberste Schlüssel
   gegen die bekannten und meldet, was übrig bleibt — der gelesene Wert bleibt stehen.
   - Pro: Trifft beide Ursachen richtig, ohne eine von ihnen zu erkennen: der Tippfehler
     wird sichtbar, die spätere Fassung kostet nichts.
   - Contra: Braucht eine Namensliste der obersten Schlüssel neben der Struktur, und eine
     solche zweite Liste ist in diesem Baum die bekannte Falle (`Kommando::KENNUNGEN`).
     Und die Meldung ist keine `Ersetzung`: es wird nichts ersetzt. Es entstünde ein
     weiterer Meldungswert neben `Grund` und `Beiseite`, also der zweite Mechanismus, den
     Möglichkeit 2 an anderer Stelle auch bräuchte.

## Constraints

- Kein zweiter Mechanismus neben `Zugang::laden` und `atomar::beiseitepfad`. Diese
  Randbedingung stammt aus dem Ausgangsdatensatz und gilt unverändert; sie ist der Grund,
  aus dem die Möglichkeiten 2 und 3 teuer sind.
- Die Rückwärtsrichtung bleibt gesichert: eine `session.toml` aus einer späteren Fassung
  darf in einer früheren die Sitzung nicht kosten. Gehalten wird das heute von der Probe
  `eine_session_toml_aus_einer_spaeteren_fassung_behaelt_ihre_sitzung`
  (`crates/krk-core/tests/ablage.rs`); jede Antwort hält sie grün oder ersetzt sie durch
  eine, die dieselbe Zusage genauer fasst.
- `Datei::leerbefund` bleibt eine vollständige Fallunterscheidung ohne Auffangzweig.

## Recommendation

Möglichkeit 1, solange niemand den vertippten obersten Schlüssel als wirklichen Verlust
gemessen hat. Die zwei anderen kosten je einen zweiten Mechanismus, den der
Ausgangsdatensatz ausschließt, und kaufen dafür eine Meldung über einen Fall, für den in
diesem Projekt keine Messung vorliegt — anders als bei `bookmarks.toml`, wo der Defekt
`260820-2235` ihn an zwei Gestalten wirklich gefangen hat. Wer Möglichkeit 2 oder 3 will,
hebt zuerst die Randbedingung „kein zweiter Mechanismus" auf; im Vorbeigehen ist keine von
beiden zu haben.

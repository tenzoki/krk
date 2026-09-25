Der Modulkopf der Bausteine sagt, die abgeschnittene Zählung dürfe „mehr" sagen; `Wert::UeberGrenze` schreibt das Gegenteil aus

---

Zwei Doc-Kommentare derselben Kiste sagen Entgegengesetztes über dieselbe Anzeige. Der Modulkopf von
`bausteine.rs` trägt die Regel in ihrer Fassung von vor dem 260824-1243; die Fassung, nach der
gebaut ist, steht am Wert in `mod.rs`.

---

## Die zwei Stellen

`crates/krk-core/src/leseprofil/bausteine.rs:47-48`:

```rust
//! - Die Zaehlung liefert [`Wert::UeberGrenze`] statt [`Wert::Zahl`]. Sie kann
//!   sagen, dass es mehr sind als die gezaehlten, und sonst nichts.
```

`crates/krk-core/src/leseprofil/mod.rs:526-528`:

```rust
/// 2.000 offene Defekte" und damit eine falsche Aussage. „Mindestens 1" ist
/// wahr — und „ueber 1" waere es nicht, denn ein weiterer Treffer hinter
/// dem Abbruch ist moeglich und nicht gesichert.
```

Beide beschreiben `Wert::UeberGrenze`, und beide stehen im selben Modulbaum. Der Modulkopf erlaubt
der Zählung genau die Aussage, die der Wert ihr verbietet.

## Welche Fassung gilt

Die des Wertes, und der Bau folgt ihr: `Wert::als_text` schreibt `mindestens {Treffer} (Lesung bei
{HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)` und nicht „über". Der Grund steht im Datensatz
`issues/260824-1215_c_die-abgeschnittene-zaehlung-zeigt-ueber-treffer-und-c6-5-verlangt-ueber-2000.md`:
getroffen hat eine bestimmte Zahl **innerhalb** der gelesenen Einträge, und ob hinter dem Abbruch
ein weiterer Treffer steht, ist unentschieden. „Mehr als die gezählten" ist damit keine gesicherte
Aussage, sondern die, die der Bau ausdrücklich nicht mehr macht.

Der Satz ist nicht bloß ungenau. Der Modulkopf ist die Stelle, an der ein Leser die Regel über die
Teillesung nachschlägt, und die Regel darüber lautet: **es wird nur gesagt, was die Teillesung
entscheidet.** Der Aufzählungspunkt darunter gibt für die Zählung ein Beispiel, das gegen die Regel
verstößt, unter der es steht.

## Woher der Satz stammt

Aus dem Plan, `### Was eine unvollständige Lesung sagen darf`. Er lautete dort wörtlich genauso und
ist am 260824-1722 berichtigt worden, zusammen mit C6.5 und der Festlegung A5
(`issues/260824-1651_c_c6-5-a5-und-planschritt-6-sagen-weiter-ueber-2-000-und-kein-offener-datensatz-traegt-es.md`).
Die Code-Hälfte blieb dabei stehen, weil dieser Lauf keine Datei unter `crates/` anfassen durfte.

## Was zu tun ist

Den Aufzählungspunkt im Modulkopf auf die Fassung des Wertes ziehen: die Zählung sagt, dass es
**mindestens** so viele sind wie die getroffenen, und dass die Lesung abgebrochen ist. Die zwei
übrigen Punkte des Modulkopfs, Vorhandensein und jüngste N, sind unberührt und stimmen.

**Schwere:** gering. Kein Fehlverhalten und keine Abweichung von einem Abnahmekriterium, aber zwei
Doc-Kommentare derselben Kiste, die einander widersprechen, an der Stelle, an der die Regel
nachgeschlagen wird.

**Gefunden:** analyst, beim Berichtigen von C6.5, A5 und der zwei Planstellen am 260824-1722.

**Betroffen:** `crates/krk-core/src/leseprofil/bausteine.rs` (Modulkopf, Abschnitt
`# Was eine unvollstaendige Lesung sagen darf`)

**Domain:** code

---
Resolved: 260824 durch `coder`. Der Aufzaehlungspunkt im Modulkopf von
`crates/krk-core/src/leseprofil/bausteine.rs` (Abschnitt `# Was eine unvollstaendige Lesung sagen
darf`) sagt jetzt dasselbe wie der Doc-Kommentar von `Wert::UeberGrenze` und wie die berichtigte
Fassung von C6.5: die Zaehlung sagt zweierlei und sonst nichts — dass es **mindestens** so viele
sind wie die getroffenen, und dass die Lesung abgebrochen wurde; *mehr* als die getroffenen sagt
sie ausdruecklich nicht, weil ein weiterer Treffer hinter dem Abbruch moeglich und nicht gesichert
ist. Der Grund dafuer, dass die Zahl die der Treffer ist und nicht die der Grenze, steht weiterhin
nur an einer Stelle; der Modulkopf verweist auf sie, statt sie zu wiederholen.

Die Fassung von `Wert::UeberGrenze` ist beim Lesen die bessere und ist unveraendert geblieben: sie
traegt die Begruendung an dem Wert, den sie beschreibt, und der Bau folgt ihr — `Wert::als_text`
setzt `mindestens {Treffer} (Lesung bei {HOECHSTENS_EINTRAEGE} Eintraegen abgebrochen)`. Angeglichen
wurde deshalb allein der Modulkopf.

Kein Verhalten geaendert; die Aenderung ist ein Kommentar. Eine Datei angefasst,
`crates/krk-core/src/leseprofil/bausteine.rs`. `make check` gruen (Exit 0).

# Der Modulkopf der Bausteine zieht auf die Fassung von `Wert::UeberGrenze` nach

**Datum:** 260824-2100
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Auftrag:** Dispatch, ein Defektdatensatz — `260824-1722_*_der-modulkopf-der-bausteine-sagt-die-abgeschnittene-zaehlung-duerfe-mehr-sagen-der-wert-widerspricht.md`
**Baumstand vorher:** `8433935` auf HEAD, daneben die Arbeit des `ontocoder` an `resources/default-readers.toml`

---

## Auftrag

Ein Befund, gering: zwei Doc-Kommentare desselben Moduls widersprachen einander an
der Stelle, an der ein Leser die Regel über die Teillesung nachschlägt. Nur
`crates/krk-core/src/leseprofil/bausteine.rs` anfassen, `resources/default-readers.toml`
nicht — dort arbeitet parallel der `ontocoder`.

## Was dastand

`bausteine.rs`, Modulkopf, Abschnitt `# Was eine unvollstaendige Lesung sagen darf`,
Zeilen 47–48:

```rust
//! - Die Zaehlung liefert [`Wert::UeberGrenze`] statt [`Wert::Zahl`]. Sie kann
//!   sagen, dass es mehr sind als die gezaehlten, und sonst nichts.
```

Das ist der Wortlaut aus dem Plan, `### Was eine unvollstaendige Lesung sagen darf`,
in seiner Fassung von vor dem 260824-1243. Der Doc-Kommentar von `Wert::UeberGrenze`
(`crates/krk-core/src/leseprofil/mod.rs:526-528`) schreibt aus, dass genau diese
Aussage **nicht** wahr ist, und der Bau folgt ihm: `Wert::als_text` (`mod.rs:574-578`)
setzt `mindestens {gezaehlt} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)`
und nicht „über".

Der Punkt stand damit als Gegenbeispiel unter der Regel, für die er ein Beispiel sein
sollte: *es wird nur gesagt, was die Teillesung entscheidet.*

## Welche Fassung die bessere ist

Die des Wertes, und sie ist unverändert geblieben. Sie trägt ihre Begründung an dem
Wert, den sie beschreibt — warum die Zahl die der Treffer ist und nicht die der
Grenze, warum „mindestens 1" wahr ist und „über 1" nicht, und warum die Zeile den
Abbruch ausdrücklich nennt statt ihn der Zahl zu überlassen. Der Auftrag ließ offen,
beide anzugleichen; angeglichen ist allein der Modulkopf.

## Was jetzt dasteht

```rust
//! - Die Zaehlung liefert [`Wert::UeberGrenze`] statt [`Wert::Zahl`]. Sie sagt
//!   zweierlei und sonst nichts: dass es **mindestens** so viele sind wie die
//!   getroffenen, und dass die Lesung abgebrochen wurde. *Mehr* als die
//!   getroffenen sagt sie nicht — ein weiterer Treffer hinter dem Abbruch ist
//!   moeglich und nicht gesichert. Warum die Zahl die der Treffer ist und
//!   nicht die der Grenze, steht bei [`Wert::UeberGrenze`].
```

Das „und sonst nichts" der alten Fassung ist erhalten, denn es ist die Regel selbst;
was sich ändert, ist die Aussage, auf die es sich bezieht. Die Begründung steht
weiterhin an genau einer Stelle: der Modulkopf verweist auf sie, statt sie zu
wiederholen — zwei Fassungen derselben Begründung wären der Defekt, der gerade
geräumt wird. Die zwei übrigen Punkte des Abschnitts, Vorhandensein und jüngste N,
sind unberührt und stimmen.

## Verhalten

Unverändert. Die Änderung ist ein Kommentar; kein `#[doc]`-Attribut, kein `cfg`, kein
Doktest.

## Prüfung

`make check` — Exit 0, alle vier Kommandos grün (`build`, `test`, `fmt --check`,
`clippy -D warnings`).

## Geändert

- `crates/krk-core/src/leseprofil/bausteine.rs` (Modulkopf, sechs Zeilen statt zwei)
- `fusion-workbench/circles/260823-2208-…/issues/260824-1722_c_…md` (`_o_` → `_c_`, `Resolved:` gesetzt)

Nicht angefasst: `resources/default-readers.toml`, `crates/krk-core/src/leseprofil/mod.rs`,
Spec und Plan. Nichts committet.

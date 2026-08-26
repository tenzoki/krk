Die Abhilfe in `pruefordner_pruefen` nennt einen `fixture`-Aufruf ohne `--seed`, und der bricht ab

---

Beide Fehlermeldungen von `pruefordner_pruefen` (`crates/krk-bench/src/messen.rs:1594`, `:1603`) enden auf die Anweisung ``die drei Pruefordner aus C8 erzeugt `krk-bench fixture --eintraege {erwartet} --out {ordner}` ``. Dieser Aufruf läuft nicht: `fixture_bauen` verlangt alle drei Angaben und bricht ohne `--seed` mit „--seed fehlt" ab (`crates/krk-bench/src/main.rs:163`, gehalten von der Probe `fixture_verlangt_alle_drei_angaben`, `main.rs:421-440`). Wer die Meldung befolgt, steht vor dem nächsten Abbruch.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-bench/src/messen.rs:1590-1606`
**Cross-references:** `shared/issues/260826-1301_c_kein-pruefordner-ausser-dem-l6-unterordner-wird-gegen-seine-zugesagte-eintragszahl-gehalten.md` (behoben in `960900d`; dieser Befund steckt in der Behebung)

## Der Befund

`crates/krk-bench/src/messen.rs:1590-1598`:

```rust
Some(brief) => Err(io::Error::other(format!(
    "{} traegt laut Steckbrief {} Eintraege statt der zugesagten {erwartet}. \
     Loesche den Ordner samt Steckbrief; den L6-Unterordner legt der Lauf \
     dann selbst neu an, die drei Pruefordner aus C8 erzeugt \
     `krk-bench fixture --eintraege {erwartet} --out {}`.",
```

Dieselbe Zeile ein zweites Mal im `None`-Zweig (`:1603`).

`crates/krk-bench/src/main.rs:162-164`:

```rust
let eintraege = eintraege.ok_or_else(|| Abbruch::Aufruf("--eintraege fehlt".to_owned()))?;
let startwert = startwert.ok_or_else(|| Abbruch::Aufruf("--seed fehlt".to_owned()))?;
let ziel = ziel.ok_or_else(|| Abbruch::Aufruf("--out fehlt".to_owned()))?;
```

## Warum der Startwert nicht Beiwerk ist

Der Modulkopf von `main.rs:16-18` schreibt die drei Aufrufe mit je einem eigenen Startwert aus:

```
krk-bench fixture --eintraege 10000  --seed 1 --out <pfad>/a
krk-bench fixture --eintraege 10000  --seed 2 --out <pfad>/b
krk-bench fixture --eintraege 100000 --seed 3 --out <pfad>/gross
```

A und B tragen dieselbe Eintragszahl und unterscheiden sich allein im Startwert. Eine Meldung, die den Startwert wegläßt, kann für B keinen richtigen Aufruf nennen — und der Steckbrief, den die Meldung gerade zum Löschen auffordert, trägt den bisherigen Startwert als einzige Auskunft darüber, welcher es war (`fixture.rs:526-543`, Zeile `startwert = …`).

## Vorschlag

Den Startwert aus dem gelesenen Steckbrief in die Meldung nehmen, wo es einen gibt, und im `None`-Zweig auf den Modulkopf von `main.rs` verweisen statt einen unvollständigen Aufruf zu nennen. Die Form, die `fixture::steckbrief_schreiben` (`fixture.rs:532-534`) in den Steckbrief selbst schreibt, ist vollständig und steht schon da: `cargo run -p krk-bench -- fixture --eintraege N --seed S --out PFAD`.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.

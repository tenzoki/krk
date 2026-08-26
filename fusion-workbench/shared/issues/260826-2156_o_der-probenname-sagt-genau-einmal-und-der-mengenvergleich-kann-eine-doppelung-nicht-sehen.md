Der Probenname sagt „genau einmal", und der Mengenvergleich kann eine Doppelung nicht sehen

---

`jede_variante_von_kommando_steht_genau_einmal_in_kennungen` (`crates/krk-core/tests/belegung.rs:1760-1789`) vergleicht zwei `BTreeSet<String>` in beide Richtungen. Eine Menge kennt keine Vielfachheit: stünde eine Variante zweimal in `KENNUNGEN`, fielen beide Vorkommen im `gefuehrt`-Set zu einem zusammen, und die Probe bliebe grün. Sie hält die Vollständigkeit, nicht die Eindeutigkeit — ihr eigener Doc-Kommentar sagt das sechs Zeilen darüber ausdrücklich (`:1743-1745`), und ihr Name sagt das Gegenteil.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-core/tests/belegung.rs:1736-1789`; zitiert in `crates/krk-ui/src/appkit/menue.rs:441-450` und in `CLAUDE.md:133`
**Cross-references:** `shared/issues/260826-1223_c_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`

## Der Befund

```rust
fn jede_variante_von_kommando_steht_genau_einmal_in_kennungen() {
    let varianten: BTreeSet<String> = varianten_der_aufzaehlung(…).into_iter().collect();
    let gefuehrt: BTreeSet<String> = Kommando::KENNUNGEN.into_iter()
        .map(|(kommando, _)| format!("{kommando:?}")).collect();
    // difference in beide Richtungen
```

Der Doc-Kommentar darüber (`:1743-1745`): „Jene läuft über `KENNUNGEN` und hält die **Eindeutigkeit**; die Vollständigkeit kann sie nicht halten … Diese hier läuft über die **Varianten** …". Also: die Eindeutigkeit hält `jedes_kommando_traegt_genau_einen_wirkungsbereich` (`:1708-1735`, `assert_ne!` über jedes Paar), die Vollständigkeit hält die neue. Zusammen ergeben sie „genau einmal"; die neue allein nicht.

Das ist kein Loch im Baum — beide Proben stehen da — sondern ein Name, der mehr sagt als sein Rumpf. In diesem Projekt ist genau das die Fehlerklasse, aus der der behobene Datensatz `260826-1223` entstanden ist: dort hat ein Doc-Kommentar „jedes Kommando genau einmal" versprochen und der Rumpf nur die Eindeutigkeit gehalten. Der Name der Nachfolgerin wiederholt die Wendung eine Ebene tiefer.

Der Name reist außerdem: `menue.rs:443` und `CLAUDE.md:133` zitieren ihn wörtlich. `CLAUDE.md:133` sagt daneben richtig, was die Probe tut („liest die Varianten aus dem Quelltext der Aufzählung und hält beide Mengen gegeneinander"), führt aber im selben Satz den Namen mit dem Anspruch.

## Vorschlag

Umbenennen auf das, was der Rumpf hält, etwa `jede_variante_von_kommando_steht_in_kennungen_und_jeder_eintrag_benennt_eine`. Der Name ist an drei Stellen zitiert, die mitziehen müßten (`menue.rs:443`, `CLAUDE.md:133`, `belegung.rs:1705`). Alternativ die Vielfachheit wirklich prüfen: statt der zweiten `BTreeSet` eine Zählung je Variante über `KENNUNGEN`, dann trägt der Name.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.

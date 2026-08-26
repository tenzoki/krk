# Die Generationsprobe filtert über eine Schleifeninvariante und misst kein Verwerfen

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Medium
**Affected:** `crates/krk-core/tests/verzeichnis.rs:619-634`
**Tree state:** `4a57028`
**Cross-references:** `crates/krk-core/src/verzeichnis/modell.rs:397-399` (`gehoert_dazu`), `:467` (`anhaengen`, ohne Generationsfrage); `crates/krk-core/tests/verzeichnis.rs:60-88` (`stapelweise_lesen`, prüft die Generation schon selbst)

---

## Was ist

```rust
#[test]
fn das_modell_verwirft_stapel_einer_alten_generation() {
    let ordner = ordner_mit_dateien("generation", 10);
    let modell = Ordnermodell::neu(4);

    assert!(modell.gehoert_dazu(4));
    assert!(!modell.gehoert_dazu(3));

    let (stapel, _) = stapelweise_lesen(ordner.pfad(), 3);
    let veraltet: usize = stapel
        .iter()
        .filter(|_| !modell.gehoert_dazu(3))
        .map(|s| s.len())
        .sum();
    assert_eq!(veraltet, 10, "die Generationspruefung greift nicht");
}
```

`modell` ist nicht `mut` und wird nach `neu(4)` nie angefasst. Das Prädikat im
`filter` sieht seinen Eintrag gar nicht an (`|_|`) und ruft eine
Schleifeninvariante, die zwei Zeilen darüber schon zugesichert ist. Es ist damit
für jeden Stapel wahr, und `veraltet` ist die Gesamtzahl der gelesenen Einträge.

Die letzte Zusicherung sagt also: **der Leser hat zehn Einträge geliefert.** Das
ist derselbe Satz, den `fuenftausend_eintraege_kommen_in_mindestens_fuenf_stapeln`
(`:95`) für 5.000 hält, und er hat mit dem Namen der Probe nichts zu tun.

## Was der Name zusagt und was ihn hielte

Verworfen wird in diesem Baum nicht vom Modell, sondern vom Rufer:
`Ordnermodell::anhaengen` (`modell.rs:467`) fragt nach keiner Generation, und die
Oberfläche stellt die Frage selbst, wie
`ein_grosser_ordner_laeuft_stapelweise_ins_modell` (`tests/verzeichnis.rs:654`)
sie vorführt — dort allerdings mit **passender** Generation, sodass auch dort der
verwerfende Zweig nie läuft.

Es gibt in dieser Datei folglich keine Stelle, an der ein Stapel fremder
Generation tatsächlich abgewiesen wird. Ein `anhaengen`, das den fremden Stapel
aufnähme, ließe beide Proben grün.

Zwei Wege, und der zweite ist der ehrlichere:

1. **Die Zeilen 627-633 streichen** und die Probe nach dem benennen, was die zwei
   verbleibenden Zusicherungen halten, nämlich `gehoert_dazu`. Der Lesevorgang
   trägt dann nichts mehr bei und fällt mit weg; `stapelweise_lesen` prüft die
   Generation ohnehin schon selbst (`:73`, `:80`).
2. **Das Verwerfen wirklich fahren:** den Stapel der Generation 3 durch dieselbe
   Abfrage schicken, die die Oberfläche stellt, und danach `modell.zeilenzahl()`
   auf 0 halten. Erst das misst den Namen.

**Schwere:** mittel. Nichts am Code ist heute kaputt, und die Prüfung von
`gehoert_dazu` ist echt; die Probe sagt aber Deckung für ein Verhalten zu, das
sie an keiner Stelle berührt.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.

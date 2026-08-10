S11 ändert eine Kernschnittstelle, deren Aufrufstellen der Plan erst S38 und S39 zuweist

---

Nach der Umsetzung von S11 hielt `cargo build --workspace` mit zwei Fehlern an,
beide in `crates/krk-ui/src/leistenmodell.rs`:

```
error[E0609]: no field `ordner` on type `krk_core::ablage::Lesezeichen`
   --> crates/krk-ui/src/leistenmodell.rs:352:49
error[E0308]: mismatched types (expected `Ziel`, found `&Path`)
   --> crates/krk-ui/src/leistenmodell.rs:382:42
```

**Der Arbeitsbereich war damit zwischen zwei Schritten nicht baubar**, und zwar
nicht für einen Augenblick, sondern bis zu Schritten, die der Plan dreißig
Positionen später führt.

---

## Was der Plan zusagt und was eintrat

S11 nennt als Dateien allein `crates/krk-core/src/ablage/`. Die beiden
Aufrufstellen in `krk-ui` weist er S38 (Anlegen) und S39 (Sprung) zu. Beide
hängen über lange Ketten an S16, S22 und S12 und wären in Turn 3 oder später
gelandet.

Zwischen S11 und S38 liegen nach der Abhängigkeitsordnung 26 Schritte. Jeder von
ihnen hätte gegen einen Arbeitsbereich gearbeitet, der nicht übersetzt, und
`cargo build --workspace` sowie `cargo test --workspace` stehen in `CLAUDE.md`
als die Abnahmekommandos des Projekts.

## Was getan wurde

Ein eigener Nachziehschritt hat die beiden Aufrufstellen minimal auf die neue
Schnittstelle gehoben, ohne Verhalten zu ändern und ohne die zweite Sorte
vorzubauen. `Leistenmodell::gewaehlt` liest den Pfad aus `Ziel::Ordner` und
liefert für eine Textmarke `None`; `Leistenmodell::anlegen` verpackt den Pfad in
`Ziel::Ordner` und behält seine Signatur. Beide Stellen tragen einen Kommentar,
dass S38 und S39 sie ablösen. Der Nachzug ist mit S11 zusammen committet
(`65c8efa`).

Danach: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` alle
sauber.

## Warum das als Defekt festgehalten wird

Nicht wegen der zwei Zeilen, die zu ändern waren. Sondern weil der Plan eine
Regel nicht führt, die er führen müsste: **ein Schritt, der eine Schnittstelle
ändert, zieht ihre Aufrufstellen im selben Schritt nach.** Der Abschnitt
`### Was die Dateiliste eines Schrittes zusagt` beschreibt, was eine Dateiliste
bedeutet, aber nicht, dass sie vollständig sein muss, damit der Bau steht.

`inference:` Ungeprüft ist, ob weitere der 42 Schritte dieselbe Lücke tragen.
Kandidaten sind alle, die eine öffentliche Form in `krk-core` ändern, während
`krk-ui` sie benutzt: S3 (`Wirkungsbereich` wächst um drei Werte), S5
(`Kommando` wächst um zwölf, `KENNUNGEN` von 53 auf 65) und S9. Bei S3 und S5
nennt der Plan die `krk-ui`-Seite ausdrücklich mit, dort greift die Lücke also
nicht.

**Zuständig:** der nächste Planungsdurchgang oder der `reconciler` am Ende der
Runde. Kein laufender Schritt ist blockiert.

**Aufgefallen bei:** Turn 1 der Editor-Runde, unmittelbar nach der Umsetzung von
S11 am 260808-0930.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`,
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260808-0929-coder-leistenmodell-auf-die-neue-lesezeichenschnittstelle.md`

---
Resolved: Der Plan trägt die fehlende Regel jetzt. Der Abschnitt
`### Was die Dateiliste eines Schrittes zusagt` hat als dritte Herleitungsregel
die **Schnittstellen-Regel** bekommen: ein Schritt, der eine öffentliche Form
ändert, zieht ihre Aufrufstellen im selben Schritt nach, auch wenn ein späterer
Schritt sie ohnehin anfassen wird. Sie steht auf der bindenden Verbotsseite und
nicht bei der Lese- und Begründungsliste, weil nicht die Vollständigkeit der
Liste geschuldet ist, sondern ein übersetzbarer Arbeitsbereich zwischen zwei
Schritten. Die Regel verweist auf die beiden Abschnitte des Nachtrags, die sie
für die sechs Schritte von S43 bis S48 schon anwenden
(`### Wie diese sechs Schritte geschnitten sind`, `### Wo diese Schritte eine
öffentliche Form ändern`), statt neben ihnen eine zweite Fassung derselben Lehre
aufzumachen. Der `inference:`-Absatz dieses Datensatzes ist mitgenommen: die
Regel benennt S3 und S5 als geprüfte Nicht-Fälle.

S11 selbst ist nachgezogen, ohne den `[DONE]`-Marker zu berühren: die Dateiliste
nennt `crates/krk-ui/src/leistenmodell.rs` (nachgezogen: `gewaehlt` und
`anlegen`), das Abnahmekriterium verlangt `cargo build --workspace` und
`cargo test --workspace` statt allein `cargo test -p krk-core`, und ein
Umsetzungsvermerk hält fest, was `65c8efa` an den zwei Stellen getan hat und
dass der Bau ohne den Nachzug bis S38 rot gestanden hätte. Der Commit ist am
Repository nachgeprüft (`git show --stat 65c8efa`), er trägt den Nachzug in
seiner Botschaft.

Kein Code ist angefasst. `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden
mit 0.

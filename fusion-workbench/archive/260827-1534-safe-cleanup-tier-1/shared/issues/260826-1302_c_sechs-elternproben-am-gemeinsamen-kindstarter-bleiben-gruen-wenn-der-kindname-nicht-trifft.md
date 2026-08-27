Sechs Elternproben am gemeinsamen Kindstarter bleiben grün, wenn der Kindname nicht trifft

---

`gemeinsam::kind_mit_deskriptorgrenze` (`crates/krk-core/tests/gemeinsam/mod.rs:334-351`) hat sechs Rufer, und **jeder** von ihnen prüft allein `ergebnis.status.success()`. `libtest` beendet sich mit 0, wenn der Filter kein Verfahren trifft; jede der sechs Kindproben kehrt daneben bei fehlender Umgebungsvariablen still zurück. Damit stehen zwei Wege offen, auf denen eine Probe grün wird, ohne gelaufen zu sein — und dahinter stehen genau die Zusagen, die `CLAUDE.md` als „sonst nur behauptet" bezeichnet.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/gemeinsam/mod.rs:334-351`; `crates/krk-core/tests/umfang.rs:266-280`, `:355-369`; `crates/krk-core/tests/verzeichnis.rs:2571-2585`, `:2783-2797`, `:2883-2897`; `crates/krk-core/tests/leseprofil.rs:3487-3501`
**Cross-references:** `shared/issues/260825-2127_*_die-kindproben-in-tests-zeit-rs-bleiben-gruen-wenn-ihr-name-nicht-trifft.md` — dieselbe Klasse an der **anderen** Kindstarter-Fassung (`zeit.rs::kindprobe_in_zone`); dieser Datensatz betrifft den gemeinsamen Starter unter `tests/gemeinsam/` und seine sechs Rufer.

## Der Befund, nachgemessen

Ein `libtest`-Binärziel mit einem Filter, der nichts trifft, beendet sich mit 0. Am 260826-1258 an einem eigens gebauten Prüfziel nachgemessen (nicht aus dem Quelltext geschlossen):

```
$ ./probebin --exact --ignored --nocapture --test-threads 1 kind_gibt_es_nicht
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
EXIT=0
```

Der Starter setzt genau diese Argumentfolge zusammen (`gemeinsam/mod.rs:344`).

## Die sechs Rufer, jeder mit derselben einen Zusicherung

| Datei:Zeile | Kindprobe | Zusage dahinter |
|---|---|---|
| `umfang.rs:266` | `kind_laesst_den_umfang_bei_deskriptormangel_unentschieden` | ein Mangel von außen lässt die Zählung unentschieden |
| `umfang.rs:355` | `kind_zaehlt_die_tiefe_kette_mit_einem_deskriptor` | die gedeckelte Zählung hält **einen** Deskriptor |
| `verzeichnis.rs:2571` | `kind_meldet_bei_deskriptormangel_ueber_einer_datei_nichts` | der Inhaltsfilter entscheidet bei Mangel nicht negativ |
| `verzeichnis.rs:2783` | `kind_entscheidet_die_tiefe_kette` | der Durchlauf hält **einen** Verzeichnisdeskriptor |
| `verzeichnis.rs:2883` | `kind_meldet_bei_deskriptormangel_nichts` | ein Mangel von außen lässt den Auftrag unentschieden |
| `leseprofil.rs:3487` | `kind_fasst_mit_einem_freien_deskriptor_zusammen` | die Zusammenfassung hält **einen** Deskriptor |

Der Rumpf ist bei allen sechs derselbe:

```rust
assert!(
    ergebnis.status.success(),
    "…\n--- stdout ---\n{}\n--- stderr ---\n{}", …
);
```

Keine prüft die Ausgabe, keine prüft eine Spur im Dateisystem.

## Der zweite stille Weg

Jede der sechs Kindproben beginnt so (`umfang.rs:285-287`, gleichlautend in den fünf anderen):

```rust
let Some(ordner) = std::env::var_os(AUFTRAG_MANGEL) else {
    return;
};
```

Läuft das Kind mit der falschen oder ohne Variable, ist es **grün**, und der Elternteil sieht `success()`. Der Auftragsname steht je Datei als eigene Konstante; ein Schreibfehler auf einer der beiden Seiten wird von nichts gefangen.

## Warum das gerade hier zählt

`CLAUDE.md` sagt unter „Was man nicht sieht": „Gemessen wird beides von Kindproben unter `ulimit -n 64`, weil `cargo test` sonst die angehobene Grenze der Sitzung erbt und die Zusage nur behauptet." Der Modulkopf des Starters sagt dasselbe (`gemeinsam/mod.rs:34-42`). Fällt die Kindprobe still aus, ist die Zusage wieder genau das: behauptet. Sechs Zusagen auf einmal, und keine davon hat eine zweite Messstelle.

Der Baum ist heute grün und die sechs Namen stimmen — deshalb Medium und nicht höher. Der Schaden entsteht bei der nächsten Umbenennung.

## Richtung

Der Starter ist die eine Stelle, an der es einmal statt sechsmal steht. Er kennt den Namen der Kindprobe schon als Argument und kann seine Ausgabe selbst halten: `--nocapture` ist gesetzt, `stdout` trägt bei einem gelaufenen Kind die Zeile `1 passed`. Eine Zusicherung im Starter deckt alle sechs Rufer und jeden künftigen — anders als bei `zeit.rs`, wo die Vorlage `ablage.rs` eine Spur je Rufer schreibt und der Aufwand deshalb je Probe anfällt.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.

---

## Nachtrag 260826-1310, aus der parallelen Durchsicht R5

Dieser Datensatz ist am 260826-1302 gefiltert worden, und die Durchsicht R5 der drei größten
Probendateien hat denselben Befund eine Minute später unabhängig erhoben. Der zweite
Datensatz ist gelöscht statt gespeichert; was er hinzufügt, steht hier.

**Es gibt einen dritten stillen Weg, und er ist der einzige, den kein Blick auf die
Umgebungsvariable und kein Blick auf den Namen fängt: ein verlorenes `#[ignore]`.** Der
Starter setzt `--ignored`, und dieses Argument fährt **nur** die stillgelegten Proben.
Verliert eine Kindprobe ihren Vermerk — beim Aufräumen, beim Umbau, beim Vereinheitlichen mit
einer Probe, die keinen braucht —, filtert `libtest` sie weg. Am 260826-1259 an einem eigenen
Prüfziel außerhalb des Projektbaums nachgemessen:

```
$ # dieselbe Datei, kind_da OHNE #[ignore], Rumpf panic!("das Kind ist gelaufen")
$ cargo test --test t -- --exact --ignored --nocapture --test-threads 1 kind_da
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
EXIT=0
```

Der Name trifft hier, die Umgebungsvariable steht, und die Elternprobe ist trotzdem grün.
Von den drei Wegen schließt die Prüfung auf `"1 passed"` in der Ausgabe alle drei; eine
Prüfung, die nur den Namen absichert, schließt keinen davon ganz.

**Zur Schwere.** R5 hat den Befund als **High** eingestuft, dieser Datensatz als **Medium**.
Der Unterschied liegt nicht in der Sache, sondern in der Gewichtung: die Kinder selbst messen
sorgfältig, ob `ulimit` überhaupt gegriffen hat (`verzeichnis.rs:2819-2828`,
`leseprofil.rs:3546-3555`), und genau diese Sorgfalt verpufft, wenn der Rumpf nie erreicht
wird — dahinter stehen die einzigen Träger von vier Zusagen, die `CLAUDE.md` unter „Was man
nicht sieht" als **gemessen** führt. Das Feld `**Severity:**` oben bleibt unverändert, weil
es dem filternden Prüfer gehört; wer den Datensatz abarbeitet, sollte die zweite Lesart
kennen.

**Cross-reference:** `shared/reviews/260826-1303-coderev-die-drei-groessten-probendateien-des-kerns.md`, Abschnitt „A1".

---
Resolved: crates/krk-core/tests/gemeinsam/mod.rs, kind_mit_deskriptorgrenze — der Starter haelt status.success() und "test result: ok. 1 passed;" in stdout, und es gibt nur noch einen Auftragsnamen KRK_KINDPROBE_AUFTRAG; beide Mutationen (Kindname, entferntes ignore) rot am Gate, gruen ohne (Plan 260826-1811 Schritt 3, Sitzungseintrag 260826-2010).

Reconciled: 260826-2205 — gegen den Baum `bc5991d` geprueft und zutreffend: der Commit ist `17e5e4e`, das Gate steht an `crates/krk-core/tests/gemeinsam/mod.rs:527-537`, der eine Auftragsname an `:471` und sein einer Leser an `:480`; nachgezaehlt: sechs Rufer von `kind_mit_deskriptorgrenze` in `umfang.rs`, `verzeichnis.rs` und `leseprofil.rs`, keine `AUFTRAG_`-Konstante mehr ausser den drei fremden. `make check` ueber `bc5991d` gruen. Nachlage: die sechs fachlichen `assert!` der Rufer sind seit dem Gate unerreichbar (`shared/issues/260826-2152_*_die-sechs-fachlichen-assert-der-kindstarter-rufer-sind-seit-dem-gate-unerreichbar.md`).

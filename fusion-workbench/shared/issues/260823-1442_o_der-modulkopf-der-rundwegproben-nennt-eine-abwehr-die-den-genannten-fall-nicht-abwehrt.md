# Der Modulkopf der `rundwegproben` nennt eine Abwehr, die den genannten Fall nicht abwehrt

---

Der Modulkopf der vier neuen Quelltextproben aus `52fba42` benennt seine eigene Lücke — einen
dritten Rufer von `editor_schliessen` — und sagt dann, wogegen die halte. Die genannte Probe hält
etwas anderes. Sie zählt die Rufer von `rundweg`, nicht die von `editor_schliessen`. Ein dritter
Rufer von `editor_schliessen` bestünde sie unverändert und bestünde auch alle vier neuen Proben.

---

**Am Baum gelesen.**

## Der Satz

`crates/krk-ui/src/appkit/anwendung.rs:8405-8409`, im Doc-Kopf von `mod rundwegproben`:

```
/// **Was sie nicht sehen:** einen dritten Rufer von
/// [`Anwendungsdelegierter::editor_schliessen`], der einen eigenen Wert
/// uebergibt. Dagegen haelt, dass die Regel des Rundwegs genau einen Aufrufer
/// hat (`crate::kommandos::rundweg::tests::die_regel_hat_genau_einen_aufrufer`).
```

## Was die genannte Probe zählt

`crates/krk-ui/src/kommandos/rundweg.rs:182-194`:

```rust
fn die_regel_hat_genau_einen_aufrufer() {
    let zuhause = "krk-ui/src/kommandos/rundweg.rs";
    let name = concat!("rund", "weg");
    let aufrufe: usize = quelldateien()
        .iter()
        .filter(|(datei, _)| datei != zuhause)
        .map(|(_, inhalt)| aufrufstellen(inhalt, name))
        .sum();
    assert_eq!(aufrufe, 1, …);
}
```

Die Nadel ist `rundweg(`. Gezählt werden die Rufer der reinen Funktion `rundweg`, und deren gibt es
genau einen, nämlich `editor_rundweg`. Über `editor_schliessen` sagt die Probe nichts.

## Der Fall, den der Satz abzuwehren vorgibt

`editor_schliessen` hat heute zwei Rufer (`grep -rn editor_schliessen crates/krk-ui/src`):

- `crates/krk-ui/src/appkit/anwendung.rs:3170` — `Kommando::EditorSchliessen => self.editor_schliessen(false)`
- `crates/krk-ui/src/appkit/anwendung.rs:7083` — `Rundweg::ZurueckInDieDateiliste => self.editor_schliessen(true)`

Genau diese zwei Zeilen halten die vier neuen Proben. Käme ein dritter Rufer dazu — ein neuer
Zweig in `kommando_ausfuehren`, ein Melder, eine Fortsetzung —, dann gilt:

- `die_regel_hat_genau_einen_aufrufer` bleibt grün, denn `rundweg` hat weiterhin einen Rufer.
- `opt_cmd_e_schliesst_ohne_die_vorschau_danach` bleibt grün, denn Zeile 3170 ist unverändert.
- `der_rueckweg_schliesst_mit_der_vorschau_danach` bleibt grün, denn Zeile 7083 ist unverändert.
- Die beiden Nachfrageproben bleiben grün, denn sie lesen `anlass_ausfuehren` und
  `anlass_unterbleibt`.

Der dritte Rufer könnte `true` oder `false` übergeben, ohne dass irgendetwas rot wird. Das ist
genau die Lage, die der Modulkopf als abgewehrt ausschreibt.

## Das Werkzeug dafür steht schon im Baum

`crate::quellbaum::aufrufstellen` (`crates/krk-ui/src/quellbaum.rs:132-150`) zählt Aufrufstellen
unabhängig von der Schreibweise und zieht Fundstellen mitten in einem Namen, die Erklärung selbst
und Kommentarzeilen ab. Ein Empfängerpunkt davor bleibt drin, `self.editor_schliessen(` zählt also
mit. Eine Zählprobe auf `editor_schliessen` mit dem erwarteten Wert 2 ist damit dieselbe Bauform,
die `rundweg.rs` schon benutzt, und sie schließt die Lücke wirklich.

## Vorschlag

Eine von zwei Sachen, nicht beide:

1. **Die Zählprobe nachziehen.** `aufrufstellen(inhalt, "editor_schliessen") == 2` über
   `quelldateien()`, mit derselben Begründung, die `rundweg.rs` für seine Zählung führt. Danach
   stimmt der Satz im Modulkopf, ohne ihn zu ändern — bis auf den Namen der Probe, auf die er
   verweist.
2. **Den Satz streichen.** Die Lücke steht dann als Lücke da, wie der Modulkopf sie zuerst
   benennt, und behauptet keine Abwehr. Das ist die billigere Antwort und die ehrlichere, wenn
   niemand die Zählung will.

Der erste Weg ist vorzuziehen: der Modulkopf hat die Lücke selbst gesehen, das Werkzeug liegt
daneben, und eine Aufruferzählung ist in diesem Baum die eingeführte Antwort auf genau diese Frage.

**Schwere:** Low. Kein Verhalten ist betroffen, und der dritte Rufer ist heute nicht da. Der Satz
sagt aber einem künftigen Leser, er sei gedeckt, wo er es nicht ist, und das ist die Sorte Zusage,
gegen die `52fba42` selbst geschrieben ist.

**Gefunden:** coderev, Auslieferungsdurchsicht `28cbb7b..b58e9d1`, Baumstand `b58e9d1`

**Domain:** code

**Cross-references:** `shared/issues/260823-1034_c_das-neue-feld-vorschau-danach-haelt-keine-probe-*`

---
Resolved:

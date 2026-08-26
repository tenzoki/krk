Die zweite Uebertragungsart verliert COPYFILE_EXCL und mit ihm die Zusage der Konfliktregel

---

Die Zusage "ueber ein vorhandenes Ziel entscheidet die Konfliktregel und nicht `copyfile(3)`"
haengt an `COPYFILE_EXCL`, und dieses Kennzeichen kommt allein ueber `COPYFILE_CLONE` herein.
Mit `Uebertragungsart::ImmerBytes` steht es nicht da, und ein vorhandenes Ziel wird
ueberschrieben statt abgewiesen. Die Uebertragungsart kommt aus dem `Auftrag`, also aus dem
Umfang dieser Durchsicht, und ist ueber `Auftrag::mit_uebertragung` oeffentlich einstellbar.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/verzeichnis/sys.rs:699-702`:

```rust
let kennzeichen = match art {
    Uebertragungsart::KlonenWennMoeglich => COPYFILE_ALL | COPYFILE_CLONE,
    Uebertragungsart::ImmerBytes => COPYFILE_ALL,
};
```

`COPYFILE_ALL` ist `0x0000_000F` (`sys.rs:468`) und traegt kein `COPYFILE_EXCL`. Dass das
Kennzeichen ueber `COPYFILE_CLONE` hereinkommt, sagt der Doc-Kommentar an `sys.rs:473-476`
selbst.

## Die zwei Doc-Kommentare, die mehr zusagen als der Code haelt

- `sys.rs:638-639` an `datei_kopieren`: "Ein vorhandenes Ziel laesst den Aufruf scheitern. Ueber
  ein vorhandenes Ziel entscheidet die Konfliktregel, nicht diese Funktion." Ohne Einschraenkung
  auf eine der zwei Arten.
- `sys.rs:473-476` an `COPYFILE_CLONE`: "Genau das ist gewollt, denn ueber ein vorhandenes Ziel
  entscheidet die Konfliktregel und nicht `copyfile(3)`." Hier steht die Bedingung, dort nicht.

## Warum das in diesen Umfang gehoert

Die Wahl faellt im `Auftrag`: `Auftrag::mit_uebertragung`
(`crates/krk-core/src/operation/auftrag.rs:210-214`) setzt das Feld, `Auftrag::neu`
(`auftrag.rs:193-200`) laesst die Vorgabe stehen, und `kopieren::datei`
(`crates/krk-core/src/operation/kopieren.rs:85`) reicht es unveraendert weiter. Wer die zweite
Art waehlt, hebt damit unbemerkt die Konfliktbehandlung fuer die einzelne Datei auf: die Antwort
`UmbenennenIn` mit einem selbst getippten Namen und der Ausfall des freien Namens nach tausend
Versuchen fuehren beide auf einen belegten Zielpfad, und `copyfile` schreibt dann darueber.

## Wie weit es heute reicht

Heute nur so weit wie die Proben: `grep -rn "Uebertragungsart::\|mit_uebertragung" crates/`
findet ausserhalb von `sys.rs` genau vier Stellen, davon zwei in
`crates/krk-core/tests/operation.rs:379` und `:465` und zwei in `auftrag.rs` selbst. **Kein
Rufer der Anwendung waehlt `ImmerBytes`.** `Uebertragungsart::default()` ist
`KlonenWennMoeglich` (`sys.rs:562-568`), und dort haelt die Zusage.

Damit ist das hier kein Defekt am laufenden Buendel, sondern eine oeffentliche Schnittstelle, die
eine Zusage still aufhebt, sobald sie zum ersten Mal in der Anwendung gebraucht wird — und ihre
Doc-Kommentare sagen das Gegenteil.

## Zwei moegliche Antworten

1. `COPYFILE_EXCL` als eigene Konstante fuehren und in **beide** Zweige aufnehmen. Dann sagt der
   Kommentar an `datei_kopieren` die Wahrheit, und der Verweis auf `COPYFILE_CLONE` faellt weg.
2. `Auftrag::mit_uebertragung` und `Uebertragungsart::ImmerBytes` streichen, solange kein Rufer
   sie braucht. Dann faellt der Fall mit der Schnittstelle.

## Umfang

`krk-core`, `verzeichnis/sys.rs` (die Kennzeichen und zwei Doc-Kommentare) und
`operation/auftrag.rs` (die Schnittstelle). **Hinweis fuer den Bearbeiter:** `verzeichnis/sys.rs`
lag am 260826 im Umfang einer parallelen Durchsicht; ein zweiter Datensatz zu derselben Zeile
kann dort entstanden sein.

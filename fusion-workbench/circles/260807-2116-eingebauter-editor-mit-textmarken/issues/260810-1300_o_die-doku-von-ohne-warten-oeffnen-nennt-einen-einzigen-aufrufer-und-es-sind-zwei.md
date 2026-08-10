Die Doku von `ohne_warten_oeffnen` nennt einen einzigen Aufrufer, und es sind zwei

---

`krk_core::verzeichnis::sys::ohne_warten_oeffnen` hat seit der Behebung des Defekts `260810-1247` einen zweiten Aufrufer: den Leseweg des Vorschaumodells (`crates/krk-ui/src/vorschaumodell.rs`, Funktion `bis_zur_grenze_lesen`). Drei Stellen in `krk-core` sagen weiter, es sei einer, und eine von ihnen sagt es als Zusage.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung des Defekts `260810-1247`
**Betroffen:** `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/src/verzeichnis/mod.rs`
**Domain:** code
**Zusammenhang:** `issues/260810-1247_*_die-typpruefung-am-pfad-ist-im-vorschauweg-geblieben-und-dort-blockiert-sie.md`

## Die drei Stellen, mit ihrem jetzigen Wortlaut

1. `crates/krk-core/src/verzeichnis/sys.rs:736` — die tragende, denn sie steht als Aussage und nicht als Skizze:

   ```
   /// Der eine Aufrufer ist [`crate::text::datei::oeffnen`]. Der Defekt, der die
   /// Funktion verlangt hat, ist `260809-1652`.
   ```

2. `crates/krk-core/src/verzeichnis/sys.rs:15` — der Modulkopf, in seinem Datenflussbild:

   ```
   //! fcntl(2)           ──> ohne_warten_oeffnen  ──> text::datei::oeffnen
   ```

3. `crates/krk-core/src/verzeichnis/mod.rs:14` — der Modulkopf des Verzeichnisteils:

   ```
   //! `260809-1652` `fcntl(2)` fuer `ohne_warten_oeffnen`, den Eingang von
   //! `text::datei::oeffnen`.
   ```

## Warum das mehr als eine Formsache ist

Stelle 1 begruendet mit der Einzigkeit des Aufrufers, dass die Entscheidung "was ein gueltiges Ziel ist" beim Aufrufer liegt und nicht in `ohne_warten_oeffnen`. Diese Begruendung haelt weiter, und zwar fuer beide Aufrufer: die Vorschau nimmt eine benannte Roehre genauso wenig an wie der Editor, sagt es aber mit einer anderen Antwort (`Inhalt::Metadaten` statt `Abweisung::KeinGueltigesZiel`). Wer die Zeile liest und daraus schliesst, er duerfe die Funktion auf den Editorfall zuschneiden, nimmt der Vorschau den Schutz, den sie seit dem 260810 hat.

## Vorgeschlagene Behebung

Aus "der eine Aufrufer" zwei machen, an allen drei Stellen, und an Stelle 1 dazusagen, was beide gemeinsam brauchen und wo sie sich unterscheiden. Die Zahl "vier Schnittstellen und acht gebundene Funktionen" in `mod.rs` bleibt richtig und ist nicht betroffen.

## Zustaendigkeit

`coder`. Nicht in derselben Aenderung behoben, weil die Dateigrenze der Behebung von `260810-1247` `crates/krk-ui/src/vorschaumodell.rs` und `crates/krk-core/src/tasten/belegung.rs` war; `sys.rs` und `mod.rs` lagen ausdruecklich ausserhalb.

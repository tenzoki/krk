Die Zusicherung gegen ein Blatt ohne ungefährlichen Ausgang greift in keinem Bau, den KRK herstellt

---

`Blatt::mit_schaltflaechen` fängt ein Blatt, dessen Schaltflächen alle etwas ausführen, mit
einem `debug_assert!` ab. Der Auslieferungsbau übersetzt es weg, und der Probenbau erreicht es
nicht, weil keine Probe ein `Blatt` baut. Die Prosa an `abbruchstelle` sagt trotzdem
unbedingt, die Zusicherung lasse den Fall auffliegen.

---

**Schwere:** Mittel. Kein Fehlverhalten am heutigen Baum: alle elf Aufrufstellen tragen eine
Schaltfläche mit `Wirkung::Liegenlassen`, und das ist nachgezählt. Der Befund ist die
Sicherung selbst — sie ist die eine, die ein künftiges Blatt vor dem zerstörenden Rückfall
bewahren soll, und sie ist unwirksam.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/mod.rs:404-409`, `:532-537`, `:810-814`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

```rust
// crates/krk-ui/src/appkit/blaetter/mod.rs:532-537
debug_assert!(
    schaltflaechen
        .iter()
        .any(|schaltflaeche| schaltflaeche.wirkung == Wirkung::Liegenlassen),
    "das Blatt \"{frage}\" traegt keine Schaltflaeche, die alles liegen laesst"
);
```

**Im Auslieferungsbau ist die Zeile nicht vorhanden.** `cargo xtask bundle` übersetzt mit
`--profile release` (`xtask/src/bundle.rs:60` setzt `PROFIL = "release"`, `:472` reicht
`--profile` durch). Weder die Wurzel-`Cargo.toml` noch eine Kisten-`Cargo.toml` noch
`.cargo/config.toml` führen einen Abschnitt `[profile.…]`; nachgezählt mit
`grep -rn '^\[profile' Cargo.toml crates/*/Cargo.toml xtask/Cargo.toml .cargo/config.toml`,
kein Treffer. Cargos Vorgabe für `release` ist `debug-assertions = false`.

**Im Probenbau ist die Zeile vorhanden und wird nie erreicht.** Alle elf Stellen, die ein
`Blatt` bauen, liegen im Nicht-Probencode:

```
Blatt::mit_schaltflaechen  belegungsansicht.rs:747, blaetter/mod.rs:504 (aus Blatt::neu),
                           blaetter/{zettel:408, konflikt:82, uebersprungen:38,
                           ungesichert:86, loeschbestaetigung:128}
Blatt::neu                 blaetter/{zeilennummer:68, pfadeingabe:69, namenseingabe:114,
                           stapelumbenennen:397, suche:134}
```

`krk-ui` hat kein Bibliotheksziel, also erreicht kein Testziel diese Funktionen, und ein
`#[cfg(test)]`-Modul, das sie ruft, gibt es nicht: `grep -rn "blaetter::\|loeschbestaetigung::zeigen" crates/krk-ui/src | grep -i test`
liefert nichts. `make check` lief am 260817-1418 grün, ohne dass die Zusicherung einmal
ausgeführt wurde.

## Was die Prosa dazu sagt

Zwei Stellen, und sie sind ungleich ehrlich.

- `:404-409`, im Doc-Kommentar von `abbruchstelle`: „**Die zweite Zeile ist ein Blatt, das es
  nicht geben soll**, und [`Blatt::mit_schaltflaechen`] laesst es im Probenbau auffliegen."
  Unbedingt formuliert und am Baum nicht wahr.
- `:810-814`, im Doc-Kommentar der Zählprobe: „der `debug_assert!` in
  [`Blatt::mit_schaltflaechen`] deckt den Rest ab, **sobald das Blatt im Probenbau wirklich
  aufgeht**." Die Bedingung steht da, und sie tritt nicht ein.

Wirksam ist damit allein `jedes_blatt_nennt_seine_liegenlassende_schaltflaeche` (`:819`), und
die prüft je Datei: sie liest, ob eine Datei, die `mit_schaltflaechen` ruft, irgendwo die
Zeichenfolge `Wirkung::Liegenlassen` enthält. Ihre eigene Blindheit steht bei ihr (`:810-812`)
und trifft namentlich `blaetter/mod.rs` selbst, die einzige Datei mit mehr als einem Blattbau.

## Richtung

Die Frage „welche Schaltfläche ist die ungefährliche, wenn keine es ist" hat keine Antwort,
und `unwrap_or(0)` ist eine Näherung darauf. Sie ist heute unauffällig, weil die Stelle 0 in
der Rückfrage vor dem Räumen die abbrechende ist; in einem Blatt mit ausführender erster
Schaltfläche wäre sie der zerstörende Ausgang, also genau der Fehler, den `260817-1106`
behoben hat.

Der Mechanismuswechsel wäre, eine liegenlassende Schaltfläche am **Typ** zu verlangen, statt
sie zu prüfen: `mit_schaltflaechen(mtm, frage, ausfuehrende: &[Schaltflaeche], liegenlassende: Schaltflaeche)`
oder ein Bauer, der sie als eigenes Argument nimmt. Dann ist `abbruchstelle` total, der
Rückfall entfällt, und der Übersetzer trägt die Zusage statt einer Zeichenfolgensuche.

Die kleine Form, falls der Umbau nicht gewollt ist: die beiden Prosastellen auf das
zurückziehen, was hält — die Zählprobe trägt die Zusage, das `debug_assert!` trägt sie nicht —
und die Zählprobe auf `Blatt::neu` mit ausweiten.

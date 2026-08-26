Der Arbeitsfaden einer Dateioperation wird mit expect gestartet und reisst die Anwendung mit

---

`operation::starten` bricht mit `expect` ab, wenn sich der Arbeitsfaden nicht starten laesst. Das
ist der einzige `expect` ausserhalb der Pruefmodule in beiden Modulen des Umfangs, und er sitzt
an einem Fehler, den das System wirklich liefert: `EAGAIN`, wenn die Fadengrenze des Prozesses
erreicht ist. Gerufen wird `starten` vom Hauptfaden, ein Panik dort nimmt die Anwendung mit —
samt jeder ungesicherten Sitzung.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-core/src/operation/mod.rs:158-165`:

```rust
let faden = thread::Builder::new()
    .name("krk-operation".to_owned())
    .spawn(move || { … })
    .expect("Arbeitsfaden fuer eine Dateioperation laesst sich nicht starten");
```

`thread::Builder::spawn` liefert ausdruecklich ein `io::Result`, gerade damit dieser Fall
behandelt werden kann; `thread::spawn` daneben paniert von selbst. Die gewaehlte Fassung holt sich
den Rueckgabewert und wirft ihn wieder fort.

## Warum das neben dem uebrigen Modul aus der Reihe faellt

Dieselbe Datei uebersetzt jeden anderen Systemfehler in eine Zeile der Abschlussliste
(`grund`, `mod.rs:476-484`), und der Modulkopf schreibt aus, dass eine gescheiterte
Einzelposition den Stapel nicht abbricht (`mod.rs:50-54`). Ein Vorgang, der gar nicht erst
anlaeuft, ist der aeusserste Fall derselben Frage — und er ist der einzige, der nicht gemeldet,
sondern beendet wird.

## Was daran haengt

`starten` liefert `Lauf` und kann heute nicht sagen, dass nichts angelaufen ist. Eine Behebung
aendert also die Signatur, oder sie liefert einen `Lauf`, der sofort `Meldung::Fertig` mit einer
uebersprungenen Zeile meldet — die zweite Fassung laesst jeden Rufer unveraendert und nutzt den
Meldeweg, den es schon gibt.

## Umfang

`krk-core`, `operation/mod.rs`, und je nach Fassung die Rufer von `operation::starten` in
`krk-ui`.

---
Also seen: 260826-1221 by coderev — dieselbe Form an zwei weiteren Stellen, `verzeichnis/leser.rs:117` und `verzeichnis/durchlauf.rs:277`, gemeldet in `shared/issues/260826-1221_*_zwei-fadenstarts-des-verzeichnisbaums-brechen-mit-panik-ab-*`.

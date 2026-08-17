# Zwei Stellen nennen den Melder der Bereichsleiste als Weg in den Papierkorb

**Datum:** 260817-1109
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 4
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/kommandos/loeschwarnung.rs`, `crates/krk-ui/src/appkit/anwendung.rs`
**Baumstand:** `472eb81`

## Der Befund

`kommandos/loeschwarnung.rs:46-50` sagt über `in_den_papierkorb`:

> die beiden Tasten, der Menueeintrag und der Melder der Bereichsleiste laufen durch ihn
> hindurch

Der Melder der Bereichsleiste kann `Kommando::InPapierkorb` nicht senden. `bereichsleiste.rs`
kennt genau elf Kommandos, und alle elf sind Umschalter: fünf Bereiche (`:164-168`), drei
Spalten (`:182-184`), die Tiefe (`:195`), der Inhalt (`:214`).

Dieselbe Aufzählung steht seit vor diesem Bündel im Doc-Kommentar von
`papierkorb_oder_zeichen_zurueck` (`anwendung.rs:4479-4483`), von wo der neue Modulkopf sie
übernommen hat.

## Warum es zählt

Der Modulkopf ist die Stelle, an der der nächste Leser die Aufrufer zählt, und die Runde
stützt auf diese Zählung ihre Zusage, dass es genau einen Löschweg gibt. Eine genannte Quelle,
die es nicht gibt, macht die Zählung unprüfbar: wer sie nachvollzieht, sucht einen Weg, der
nie bestand, und weiß am Ende nicht, ob er ihn übersehen hat.

## Richtung

Beide Stellen auf das kürzen, was der Baum trägt: die beiden Tasten und den Menüeintrag. Nach
Bündel D kommt `f8` als dritte Taste hinzu.

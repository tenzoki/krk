# Die drei neuen Einträge ziehen die zwei Zählstände im Kopf der Auslieferungsbelegung nach

---
**Domain:** ontology
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**For:** ontocoder
**Cross-references:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Schritt 9); `resources/default-keymap.toml:34`; `crates/krk-core/src/tasten/belegung.rs` (Probe `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`); `history/260831-0230-coder-schritt-8-die-drei-kommandos-und-die-einhaengung.md`

---

## Befund

Zeile 34 von `resources/default-keymap.toml` lautet heute

```
# Ausgeliefert sind 88 Funktionen mit zusammen 93 Kombinationen.
```

und die Probe `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs`) hält beide Zahlen gegen die Datei.
Der Plantext von Schritt 9 nennt die drei `[[funktion]]`-Blöcke und diese Zeile
nicht.

**Gemessen und nicht geraten:** ich habe die drei Blöcke am 260831 versuchsweise
eingetragen, `cargo test --workspace` gefahren und die Datei danach unverändert
zurückgestellt (Prüfsumme gleich). Ohne Nachzug der Zeile ist die Probe rot, und
zwar als einzige, die es nach Schritt 9 noch wäre.

## Was zu tun ist

Mit den drei Einträgen steigt die Zahl der Funktionen um drei und die der
Kombinationen um zwei — `spalte_marke_umschalten` trägt `tasten = []`. Die
Zeile ist im selben Schritt nachzuziehen, in dem die Blöcke entstehen; die
gerechnete Zahl steht in der Fehlermeldung der Probe, falls sie danebenliegt.

## Was schon erledigt ist

Die zwei Ausnahmelisten für Funktionen ohne Kombination sind bereits nachgezogen
und liegen beide in Rust, also außerhalb dieses Datensatzes:
`OHNE_KOMBINATION_AB_WERK` in `crates/krk-core/tests/belegung.rs` und das
Literal in `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
(`crates/krk-ui/src/belegungsausgabe.rs`) führen `spalte_marke_umschalten`
seit Schritt 8, jeweils unmittelbar hinter `spalte_typ_umschalten`.

---
Resolved: Die Kopfzeile von `resources/default-keymap.toml` nennt seit dem 260831-0808
91 Funktionen mit zusammen 95 Kombinationen. Gezählt und nicht gerechnet: die Zahlen
stammen aus einem Durchlauf über `^[[funktion]]` und die Einträge jeder `tasten`-Zeile
derselben Datei, nicht aus 88+3 und 93+2.
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` ist grün, `make check`
exit 0.

Im selben Absatz stand eine zweite Kombinationszahl („heute sind es 90", geschrieben am
260818 in `18af77f`). Sie war schon vor dieser Runde um drei zu niedrig und wäre mit den
drei neuen Einträgen um fünf danebengelegen. Sie ist nicht nachgezogen, sondern gefallen:
an ihrer Stelle steht der Zeiger auf die Kopfzeile, die eine Probe hält. Aus demselben
Grund nennt der Einleitungsabsatz der Spaltenschalter keine „drei" mehr, die dieser
Schritt auf vier gebracht hätte.

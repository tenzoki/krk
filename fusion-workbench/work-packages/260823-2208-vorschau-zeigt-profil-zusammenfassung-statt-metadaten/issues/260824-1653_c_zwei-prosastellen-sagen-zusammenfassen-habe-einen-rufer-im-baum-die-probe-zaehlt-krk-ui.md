Zwei Prosastellen sagen, `zusammenfassen` habe einen Rufer „in diesem Baum"; die Probe zählt allein `krk-ui`

---

`crates/krk-ui/src/vorschaumodell.rs` sagt an zwei Stellen, `laden` sei **der eine Aufrufer von
`krk_core::leseprofil::zusammenfassen` in diesem Baum**. Im Baum stehen elf Aufrufstellen. Die
Zählprobe, die die Aussage halten soll, filtert ausdrücklich auf `krk-ui/` und sagt das in
ihrem eigenen Doc-Kommentar auch — nur die zwei Stellen darüber sagen es nicht.

---

## Die drei Stellen

```
crates/krk-ui/src/vorschaumodell.rs:90   //! Daraus folgt C4.7 ohne eine eigene Vorkehrung: [`laden`] ist der eine
crates/krk-ui/src/vorschaumodell.rs:91   //! Aufrufer von [`krk_core::leseprofil::zusammenfassen`] in diesem Baum, und
```

```
crates/krk-ui/src/vorschaumodell.rs:672  /// **Der eine Aufrufer von [`krk_core::leseprofil::zusammenfassen`] in diesem
crates/krk-ui/src/vorschaumodell.rs:673  /// Baum** (C4.7).
```

```
crates/krk-ui/src/vorschaumodell.rs:1430 .filter(|(datei, _)| datei.starts_with("krk-ui/"))
```

Gezählt am 260824-1648: `zusammenfassen(` steht zehnmal in
`crates/krk-core/tests/leseprofil.rs` und einmal in `crates/krk-ui/src/vorschaumodell.rs`.

## Warum das zählt

„Im Baum" ist in diesem Projekt ein festgelegter Umfang und keine Redewendung: `quellbaum`
liest `crates/` mit allen Testzielen, und sein Modulkopf hält seit der Runde 7 fest, dass genau
diese Verengung schon einmal einen Doppelbau verdeckt hat
(`issues/260813-0540_*_die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste.md`).
Der Befund hier ist die Umkehrung: die Probe liest richtig eng, und die Prosa spricht weit.

Der Doc-Kommentar der Probe selbst
(`crates/krk-ui/src/vorschaumodell.rs:1407-1411`) ist bereits genau: „In `crates/krk-ui` wird
… an genau einer Stelle gerufen … Gezählt wird allein diese Kiste." Die zwei Stellen darüber
sind ihm nachzuziehen.

## Was zu tun ist

An beiden Stellen „in diesem Baum" durch „in `krk-ui`" oder „im ausgelieferten Programm"
ersetzen. Kein Bau ändert sich, und die Zusage C4.7 bleibt, was sie ist: die zehn Rufer in
`krk-core/tests` laufen in keinem Vorschaufenster.

**Schwere:** niedrig.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1648.

**Betroffen:** `crates/krk-ui/src/vorschaumodell.rs` (Modulkopf Zeile 90-91, Doc-Kommentar von `laden` Zeile 672-673)

**Domain:** code

---
Resolved: 260824-1740 vom coder. Beide Stellen in `crates/krk-ui/src/vorschaumodell.rs` sagen jetzt „im ausgelieferten Programm" statt „in diesem Baum"; der Doc-Kommentar an `laden` schreibt dazu aus, warum „im Baum" zu weit greift (zehn Rufe in `krk-core/tests`, keiner in einem Vorschaufenster) und dass die Zählprobe deshalb `crates/krk-ui` liest. Kein Bau geändert.

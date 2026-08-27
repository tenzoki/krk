`Kommando::KENNUNGEN` ist die programmweite Kommandoliste, und nichts hält sie vollständig

---

`Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs:695-798`) wird im ganzen Baum als **die** Aufzählung aller Kommandos benutzt. Weder der Übersetzer noch eine Probe hält, dass jede Variante von `Kommando` darin steht. Eine Variante, die hinzukommt und den Eintrag nicht bekommt, übersetzt, besteht jede Probe, ist für Menü, Belegung und Belegungsausgabe unsichtbar — und bringt zwei `expect`/`panic!` zum Auslösen, sobald sie doch jemand nachschlägt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Was den Übersetzer hält und was nicht

`Kommando::wirkungsbereich` (`belegung.rs:849-1104`) ist eine vollständige Fallunterscheidung ohne Auffangzweig — am 260826 gegen den Baum nachgezählt: 79 Varianten, 79 in den Zweigen genannt, kein `_ =>`. Diese Hälfte hält der Übersetzer, und die Zusage von `CLAUDE.md` trägt.

`KENNUNGEN` hält er **nicht**. Die Deklaration lautet `[(Kommando, &'static str); 79]`; die Längenangabe zwingt zu 79 Einträgen und sagt nichts darüber, **welche** 79. Eine achtzigste Variante zwingt zu einer Zeile in `wirkungsbereich` und zu keiner hier.

## Was die genannte Probe wirklich prüft

`jedes_kommando_traegt_genau_einen_wirkungsbereich` (`crates/krk-core/tests/belegung.rs:1696-1722`) läuft über `Kommando::KENNUNGEN.into_iter()` und prüft paarweise, dass **kein Kommando zweimal** und **keine Kennung zweimal** darin steht. Sie kann eine fehlende Variante nicht sehen: sie iteriert über die Liste, deren Vollständigkeit die Frage ist. Ihr eigener Doc-Kommentar sagt es genauer, als der Rumpf hält — „dass `Kommando::KENNUNGEN` jedes Kommando genau einmal führt" ist eine Aussage über zwei Hälften, und geprüft ist nur die zweite.

`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (`belegung.rs:1707-1718`) iteriert ebenfalls über `KENNUNGEN` und prüft die Gegenrichtung: jede genannte Kennung steht in der Auslieferungsbelegung. Auch sie sieht eine fehlende Variante nicht.

Damit gibt es im ganzen Baum keine Stelle, die über die **Varianten** iteriert.

## Die Stelle, die die ungehaltene Hälfte als gehalten zitiert

`crates/krk-ui/src/appkit/menue.rs:437-440`:

```
/// Der Index in [`Kommando::KENNUNGEN`]. Die Liste fuehrt jedes Kommando genau
/// einmal — `jedes_kommando_traegt_genau_einen_wirkungsbereich` in
/// `krk-core/tests/belegung.rs` haelt das fest —, und sie ist zur Uebersetzzeit
/// festgelegt; der Index ist damit im Prozess stabil.
```

Der Beleg trägt die zitierte Aussage nicht. Er trägt „genau einmal, wenn überhaupt".

## Was eine fehlende Zeile kostet

1. **Zwei Absturzstellen.** `Kommando::kennung()` (`belegung.rs:1107-1117`) endet auf `panic!("jedes Kommando steht in KENNUNGEN")`; `tag_des_kommandos` (`menue.rs:441-446`) auf `.expect("jedes Kommando steht in KENNUNGEN")`. Beide laufen auf dem Hauptfaden.
2. **Das Kommando ist unbelegbar.** `Kommando::aus_kennung` (`belegung.rs:805-810`) sucht in `KENNUNGEN`; ohne Eintrag liefert es für jede Kennung der `keymap.toml` `None`, und `Funktion::kommando()` antwortet `None`. Der Befehl steht in der Belegungsansicht und tut nichts — genau die Falle, die `CLAUDE.md` unter „Was man nicht sieht" für den Ausführungszweig beschreibt, an einer zweiten Stelle.
3. **Es fällt aus jedem Rundumlauf heraus.** `KENNUNGEN` ist die Kommandoliste auch in `krk-ui/src/kommandos/zulaessigkeit.rs:592,620,668,768`, `kommandos/fokus.rs:587,763`, `kommandos/operationen.rs:1474`, `belegungsausgabe.rs:758` und `appkit/menue.rs:1069,1085,1109`. Ein Kommando ohne Eintrag wird von keiner dieser Erhebungen gezählt, auch nicht von denen, die sich als Vollständigkeitsproben verstehen.

## Vorschlag

Eine Probe, die über die **Varianten** läuft, nicht über die Liste. Ohne Ableitung (`strum` ist nicht eingebunden und wäre eine fremde Kiste für eine Zeile) bleibt die Form, die dieses Projekt an anderer Stelle schon fährt: ein `const ALLE: [Kommando; N]` neben der Aufzählung, dessen Vollständigkeit ein `match` ohne Auffangzweig erzwingt, und eine Probe `ALLE` gegen `KENNUNGEN`. Alternativ eine `const fn`, die zur Übersetzungszeit für jede Variante `kennung()` auswertet — dann wird aus dem `panic!` in `kennung()` ein Übersetzungsfehler, und die Zusage steht dort, wo sie behauptet wird.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`, am Baum geprüft. Gehört nach der Herkunftsregel in den gemeinsamen Speicher: kein Circle ist aktiv, und der Befund betrifft jede Runde, die ein Kommando hinzufügt.

Also seen: 260826-1416 by coderev — in `appkit/menue.rs` liegen elf `KENNUNGEN`-Zeilen (`:44,351,437,442,445,446,459,1069,1080,1085,1109`): vier Prosa, vier im Code (`tag_des_kommandos` mit zwei `expect`, `kommando_zum_tag`), drei in Proben; `:437-440` zitiert `jedes_kommando_traegt_genau_einen_wirkungsbereich` für die Eindeutigkeit, nicht für die Vollständigkeit.

Also seen: 260826-1417 by coderev — die Probe `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` (`zulaessigkeit.rs:666-692`) iteriert über `KENNUNGEN` und nicht über die vier, die sie kennt; sie wird für jeden fünften Eintrag rot, der in `KENNUNGEN` steht, und erbt für einen, der dort fehlt, genau diesen blinden Fleck.

---
Resolved: 260826-2135 — die Probe jede_variante_von_kommando_steht_genau_einmal_in_kennungen haelt die Varianten aus dem Quelltext der Aufzaehlung gegen KENNUNGEN, in beiden Richtungen; Helfer varianten_der_aufzaehlung ohne Aufzaehlungsnamen, damit der zweite Plan ihn fuer Wirkungsbereich nimmt; Mutation rot vor der Behebung (Plan 260826-1811 Schritt 4).

Reconciled: 260826-2205 — gegen den Baum `bc5991d` geprueft und zutreffend: der Commit ist `9a4e495`, die Probe steht an `crates/krk-core/tests/belegung.rs:1760` und vergleicht beide Richtungen, der Helfer an `crates/krk-core/tests/gemeinsam/mod.rs:411` fuehrt keinen Aufzaehlungsnamen; nachgezaehlt: 79 datenlose Varianten gegen `[(Kommando, &'static str); 79]` an `crates/krk-core/src/tasten/belegung.rs:697`. `make check` ueber `bc5991d` gruen. Der Hash steht hier als Abgleichsbeleg, nicht als Berichtigung der `Resolved:`-Zeile (`shared/issues/260826-1933_*_die-zwei-resolved-zeilen-der-schritte-1-und-2-tragen-den-sitzungsstempel-statt-des-commits.md`).

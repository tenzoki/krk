Vier öffentliche Lesezugänge von `Bestand` und `Neuerungen` haben keinen Rufer im Betriebscode

---

`Bestand::ordner`, `Bestand::dateien`, `Bestand::fuer`, `Bestand::traegt_unterschied` und
`Neuerungen::traegt_unterschied` sind `pub` und tragen je ein `#[must_use]`. Gerufen werden
sie allein aus `crates/krk-core/tests/ablage.rs`; im Betriebscode ruft sie niemand, denn die
zwei Formatierer desselben Moduls greifen die Felder unmittelbar.

---

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** Durchsicht der Runde 24, Bereich `feecd6c..d9535c4`
**Cross-references:** `260826-1221_o_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`,
`260826-1225_o_zwei-oeffentliche-zugaenge-der-ablage-haben-im-ganzen-arbeitsbereich-keinen-rufer.md` — dieselbe Klasse
**Betroffen:** `crates/krk-core/src/ablage/neuerungen.rs:258-282` (`Bestand`),
`crates/krk-core/src/ablage/neuerungen.rs:238-245` (`Neuerungen::traegt_unterschied`)

## Erhebung

`grep -rn "traegt_unterschied\|\.fuer(\|\.dateien()\|\.ordner()" crates/*/src crates/*/tests`
gegen den Stand `d9535c4`: jeder Treffer liegt in `crates/krk-core/tests/ablage.rs`.
`startzeile` und `blatttext` greifen `bestand.dateien` und `bestand.ordner` als Felder
(`neuerungen.rs:484`, `:497`, `:543`), `krk-ui` reicht den `Bestand` nur weiter.

## Was daran zählt

Der Übersetzer sieht es nicht: die Namen sind `pub` in einer Bibliothekskiste, also gilt
jeder als benutzt. Eine Probe, die die Zugänge ruft, hält sie am Leben, ohne dass jemand sie
braucht — und die nächste Änderung an `Bestand` zieht sie mit, weil sie nach ihrer Form
Schnittstelle sind und nach ihrer Benutzung keine.

## Abnahme

Entweder fällt, was keinen Rufer hat, oder es hat einen. Beides ist vertretbar; abgenommen
ist der Zustand, in dem
`grep -rn "\.traegt_unterschied()\|\.fuer(\|\.dateien()\|\.ordner()" crates/*/src` für
dieses Modul entweder leer ist und die Namen weg sind, oder je Name eine Zeile im
Betriebscode nennt.

---
Resolved: 2183df9 und 00fb99a — nach dem Massstab
`260912-1149_*_was-geschieht-mit-einem-oeffentlichen-namen-ohne-rufer-im-betriebscode.md`.
`Bestand::ordner` hatte gar keinen Rufer und ist gefallen; `Bestand::dateien`, `Bestand::fuer`
und `Bestand::traegt_unterschied` bleiben `pub` und tragen je einen Absatz, der die Probe
namentlich nennt und sagt, warum sie den Namen nicht anders erreicht.
`Neuerungen::traegt_unterschied` hat seit `ad43d87` einen Betriebsrufer und war damit schon
kein Befund mehr, als dieser Datensatz noch offenstand.

**Eine Behauptung dieses Datensatzes war falsch.** Er nennt fuer alle vier Zugaenge gemeinsam
"jeder Treffer liegt in tests/ablage.rs" und hat nicht je Name geprueft: fuer `ordner` gab es
ueberhaupt keinen Treffer, auch am Stand `d9535c4` nicht, gegen den erhoben wurde. Die
Erhebungsregel, die daraus folgt — je Name einzeln, ueber `crates` und `xtask`, und nach der
Kiste des Rufers unterschieden —, steht im Nachtrag des Entscheids. `make check` mit Exit 0.

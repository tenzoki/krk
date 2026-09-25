# Coder-Sitzung: Schritt 7, das Kommando im Kern

**Date:** 2026-09-10, 260910-1215
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 7
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

`Kommando::NeuerungenZeigen` steht im Kern, mit Kennung, Wirkungsbereich und
Funktionsbereich. Der Ausführungszweig fehlt bewusst; er ist Schritt 9.

### `crates/krk-core/src/tasten/belegung.rs`

Drei Stellen, die drei Pflichtstellen eines neuen Kommandos:

- Die Variante `NeuerungenZeigen` am Fuß der Aufzählung `Kommando`, wo dieser Baum jedes
  Kommando einer neuen Runde anfügt (zuletzt `GitBereichUmschalten`, `FokusGit` und
  `SpalteMarkeUmschalten` aus der Runde 23). Ihr Doc-Kommentar sagt, dass das Blatt den
  Bestand vom Start zeigt und die drei Dateien nicht neu liest, und verweist für den
  Wirkungsbereich auf `Kommando::Notizzettel`.
- Die Zeile `(Kommando::NeuerungenZeigen, "neuerungen_zeigen")` in `Kommando::KENNUNGEN`,
  und die Längenangabe der Liste von 86 auf 87. Die Zeile ist die Pflichtstelle, die der
  Übersetzer nicht hält: ohne sie stürzen `kennung()` und `tag_des_kommandos` ab und
  `aus_kennung` liefert `None`.
- `Kommando::wirkungsbereich` gibt `Wirkungsbereich::Ueberall`, im selben Zweig wie
  `Notizzettel`, mit eigener Begründung darüber: das Blatt hängt am Hauptfenster, sein
  Gegenstand ist die Ablage und keiner der Bereiche der Fensterzeile.

### `crates/krk-ui/src/belegungsmodell.rs`

`bereich_des_kommandos` gibt `Funktionsbereich::Anwendung`, im Zweig neben
`BelegungAnsehen`, `BelegungsdateiAnsehen`, `Beenden`, `WeitereInstanz` und `Notizzettel`.
Der Kommentar sagt zusätzlich, warum der Befehl **nicht** unter `Vorschau` steht, obwohl die
Belegungsdatei dort erscheint: er zeigt nicht eine der drei Dateien, sondern den
Unterschied.

## Was nicht angefasst ist

`resources/default-keymap.toml`. Der Plan sperrt sie für diesen Schritt und gibt sie
Schritt 8; die Reihenfolge ist nicht umkehrbar, weil `nach_bereichen` an einer Funktion
ohne Kommando mit `panic` abbricht.

## Die erwartete Zwischenlage, und zwei Stellen mehr als der Plan nennt

Der Plan sagt für die Spanne zwischen Schritt 7 und Schritt 8 genau eine rote Probe voraus,
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` in `krk-core`. Die steht
rot, wörtlich mit der vorhergesagten Meldung. **Es sind aber drei**, gezählt mit
`cargo test --workspace --no-fail-fast`, und alle drei haben dieselbe Wurzel: die Kennung
`neuerungen_zeigen` steht noch in keiner Auslieferungsbelegung.

| Ziel | Probe |
|---|---|
| `-p krk-core --lib` | `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` |
| `-p krk-core --test belegung` | `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` |
| `-p krk-ui --bin krk` | `belegungsausgabe::tests::die_dritte_spalte_haelt_die_begruendungslagen_auseinander` |

Die zweite verlangt zusätzlich eine **Kombination** und nicht nur einen Eintrag: sie lässt
eine Funktion ohne Kombination nur durch, wenn ihre Kennung in `OHNE_KOMBINATION_AB_WERK`
steht (`crates/krk-core/tests/belegung.rs`). Der Nutzer hat `opt+cmd+i` festgelegt, also
trägt Schritt 8 eine Kombination ein und die Ausnahmeliste bleibt unberührt; wer den Eintrag
ohne Kombination schriebe, holte diese Probe nicht zurück.

Die Acceptance von Schritt 8 verlangt `cargo test --workspace` vollständig grün und deckt
alle drei ab. Der Ausführende sollte trotzdem wissen, dass er drei Proben zurückholt und
nicht eine: bleibt nach seinem Eintrag eine rot, ist das kein neuer Befund, sondern eine der
drei.

Alles andere ist grün, 948 Proben allein im Binärziel. Namentlich grün sind die drei
Vollständigkeitsproben, die dieses Kommando betreffen:
`jede_variante_von_kommando_steht_genau_einmal_in_kennungen` (die Variante steht genau
einmal in `KENNUNGEN`), `jedes_kommando_traegt_genau_einen_wirkungsbereich` (sie trägt einen
der acht Bereiche) und `jedes_kommando_ueberlebt_den_weg_durch_den_tag`
(`tag_des_kommandos` und `kommando_zum_tag` beantworten sie in beide Richtungen).

## Prüfung

`make check` — Rückgabewert 2. Es hält am zweiten seiner fünf Kommandos an, `cargo test
--workspace`, an der oben genannten Probe des Kerns; die drei dahinter fährt es dann nicht
mehr. Einzeln gefahren sind sie grün: `cargo clippy --workspace --all-targets` 0,
`cargo fmt --all --check` 0, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` 0.
`cargo build --workspace` ist 0 und damit das erste Abnahmekriterium des Schrittes erfüllt:
die zwei vollständigen Fallunterscheidungen sind beantwortet.

Die Rotlage ist die, die der Plan für diese Spanne vorsieht, und sie geht mit Schritt 8 weg.

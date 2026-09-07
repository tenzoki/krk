# Die Liste der ab Werk tastenlosen Funktionen steht an einer Stelle, und beide Prüfrichtungen hängen an ihr

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Auftrag

Aufgabe K9. Die Aufzählung der Funktionen, die ohne Tastenkombination ausgeliefert werden, stand zweimal im Baum: als `OHNE_KOMBINATION_AB_WERK` in `crates/krk-core/tests/belegung.rs` und als Literal im Rumpf von `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` (`crates/krk-ui/src/belegungsausgabe.rs`). Der Nutzer hat am 260907 Möglichkeit 3 aus `260814-2326_*_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md` gewählt: die Liste bleibt an einer Stelle im Prüfcode, und die zweite Prüfrichtung zieht mit um.

## Erhebung vor dem Umbau

Gezählt mit

```sh
grep -rn 'OHNE_KOMBINATION_AB_WERK\|"spalte_groesse_umschalten"' crates/ --include='*.rs'
```

Betroffen waren drei Proben, wie der Datensatz vom 260814 sagt:

1. `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` (`crates/krk-core/tests/belegung.rs`) — liest die Konstante als Ausnahmezweig, hält die eine Richtung.
2. `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` (dieselbe Datei) — liest sie als Ausnahme zur Nachschlagzusage.
3. `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` (`crates/krk-ui/src/belegungsausgabe.rs`) — trug das zweite Literal und mit ihm die Gegenrichtung.

Der dritte Treffer des `grep`, `crates/krk-core/src/tasten/belegung.rs:855`, ist `Kommando::KENNUNGEN` und keine dritte Fassung der Liste.

Die Liste selbst stimmt: sieben Einträge je Seite, inhalts- und reihenfolgegleich, und dieselben sieben trägt die Auslieferungsdatei. Gezählt mit

```sh
awk '/^\[\[funktion\]\]/{id=""} /^id *=/{id=$0} /^tasten *= *\[\]/{print id}' resources/default-keymap.toml
```

## Was gebaut ist

**`crates/krk-core/tests/belegung.rs`** hält die Liste weiterhin allein und trägt jetzt beide Richtungen. Neu ist `ab_werk_traegt_genau_diese_liste_keine_kombination`: ein `assert_eq!` der ausgerechneten unbelegten Funktionen gegen `OHNE_KOMBINATION_AB_WERK`, mit Reihenfolge. Der Vergleich sagt in einem Zug, dass keine andere Funktion tastenlos ist und dass jede genannte wirklich keine Taste trägt. Der Meldetext nennt beide Abweichungsrichtungen einzeln (`ohne Kombination und nicht in der Liste`, `in der Liste und mit Kombination`), damit ein Leser nicht zwei siebenstellige Listen von Hand gegeneinander hält.

Die Reihenfolge ist Teil der Zusage geblieben, weil die Begründung dafür (die Reihenfolge ist die der Belegungsdatei) mit umgezogen ist und ohne die Zusage neben nichts stünde.

**`crates/krk-ui/src/belegungsausgabe.rs`** verliert das Literal und den dritten Teil der Probe. Die zwei verbliebenen Teile rechnen die unbelegten Funktionen aus der Belegung aus und schreiben sie in ihren Meldetext; die Auskunft „welche Funktion fällt aus der Datei" steht damit beim Fehlschlag weiter da, ausgerechnet statt hingeschrieben und deshalb nie veraltet. Der Kommentar in der Nachbarprobe `innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei`, der bisher auf den entfallenen dritten Teil zeigte und die Liste ein drittes Mal unvollständig nachbetete, zeigt jetzt auf `OHNE_KOMBINATION_AB_WERK`.

## Der Durchlauf über die `ALLE`-Listen

`jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` (`crates/krk-core/tests/baum.rs`, seit `cf232e2`) fasst diese Liste nicht, und das ist richtig: der Durchlauf greift Listen namens `ALLE` und hält sie gegen den Quelltext ihrer Aufzählung. Hinter `OHNE_KOMBINATION_AB_WERK` steht keine Aufzählung, sondern die Kennungen einer Datei; die zweite Quelle ist `resources/default-keymap.toml`, und die liest die neue Probe über `Belegung::auslieferung()`. Eine Umbenennung auf `ALLE` brächte die Liste in seinen Griff und zugleich um ihre Zusage, weil er nach einer Aufzählung suchte, die es nicht gibt. Der Modulkopf schreibt das aus.

## Befund

Der Widerspruch in den umgezogenen Begründungen ist als eigener Datensatz abgelegt: `260907-1225_o_die-begruendung-am-kopf-von-ohne-kombination-ab-werk-nennt-sechs-eintraege-einer-offen-gelassenen-wahl-vier-davon-folgen-einer-getroffenen.md`. Der Text ist beim Umzug nicht umformuliert worden.

## Abnahme

Alle fünf Kommandos grün, dazu eine Gegenprobe: mit `"belegungsdatei_ansehen"` durch `"oeffnen"` ersetzt fällt die neue Probe und nennt beide Abweichungen namentlich; danach zurückgesetzt.

- `cargo build --workspace` — exit 0
- `cargo test --workspace` — exit 0
- `cargo clippy --workspace --all-targets -- -D warnings` — exit 0
- `cargo fmt --all --check` — exit 0 (nach einem `cargo fmt --all`, das allein `belegung.rs` angefasst hat)
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — exit 0

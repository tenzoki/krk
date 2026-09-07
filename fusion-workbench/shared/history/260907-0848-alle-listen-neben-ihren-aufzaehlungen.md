# Ein Durchlauf haelt jede Liste ALLE gegen die Varianten ihrer Aufzaehlung

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Auftrag K6: Proben halten jede Liste neben ihrer Aufzaehlung vollstaendig.
Grundlage ist
`260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`,
am 260907 vom Nutzer mit Moeglichkeit 1 beantwortet: eine Probe liest die
Varianten aus dem Quelltext, keine fremde Kiste.

## Die Erhebung

Gezaehlt mit `grep -rn 'const ALLE' crates/*/src xtask/src`, HEAD `90f352d`.
Fuenfzehn Treffer, davon zwei keine `ALLE`-Liste:
`ALLES_ABGEWIESEN` (`krk-ui/src/kommandos/zulaessigkeit.rs`) faellt unter das
Praefix, und `Bereich::ALLE` selbst ist die Aufzaehlungsseite und nicht die
Liste daneben. Dreizehn Listen bleiben, eine davon
(`Loeschzielbefund::ALLE`) im Pruefmodul ihrer Datei.

Daneben zwei Listen derselben Art unter anderem Namen, beide schon gehalten:
`Kommando::KENNUNGEN` (`crates/krk-core/tests/belegung.rs`) und das
Beschriftungsfeld von `Wirkungsbereich` (dieselbe Datei). Zusammen die
fuenfzehn Listen, von denen der Datensatz spricht.

Die vier Bauformen, die `CLAUDE.md` nennt, sind eine **zweite** Familie: nicht
`ALLE` selbst, sondern Felder parallel zu `Bereich`, gegriffen ueber
`Bereich::index()`. Gezaehlt mit
`grep -n '; 6\]' crates/krk-ui/src/fenstermodell.rs crates/krk-ui/src/appkit/aufteilung.rs`:
neun Stellen, davon vier in Pruefmodulen.

## Was gebaut ist

**Ein Durchlauf statt dreizehn Proben.**
`jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`
(`crates/krk-core/tests/baum.rs`) sucht jede Zeile `const ALLE: [` unter
`crates/`, liest den Listenrumpf und die Varianten der gleichnamigen
Aufzaehlung derselben Datei und haelt beide gegeneinander: fehlende Variante,
Doppelung, ueberzaehliger Eintrag, Reihenfolge — je eine Zusicherung, die die
Namen ausschreibt. Die Variantenseite kommt aus
`gemeinsam::varianten_der_aufzaehlung`, dem Helfer der Runde 22, unveraendert.

Der Durchlauf statt einer Probe je Liste ist die Wahl gegen die Luecke eine
Ebene hoeher: eine vierzehnte Liste ist vom Tag ihrer Entstehung an gehalten,
ohne dass jemand daran denkt. Er erreicht auch `krk-ui`, das kein
Bibliotheksziel hat, weil er beide Seiten aus dem Quelltext liest und keine
verlinkt; sechs der elf gedeckten Listen liegen dort.

Zwei Listen stehen mit Grund in `UNLESBARE_ALLE_LISTEN` und sind uebergangen
(`260907-0858_*_zwei-alle-listen-bleiben-vom-durchlauf-ungedeckt-und-eine-davon-kann-keine-nadel-lesen.md`).
Der Durchlauf haelt jede Ausnahme gegen den Baum: eine, deren Fundstelle
verschwindet, laesst ihn rot werden.

**Die Feldlaengen leiten sich ab.** `Aufteilung::rahmen`,
`Aufteilung::gemessene_breiten`, `Fenstermodell::breiten_uebernehmen`,
`bereichsbreiten`, `anteilig` und `traegt_eine_ziehbewegung` tragen die Laenge
`Bereich::ALLE.len()` statt einer `6` im Quelltext, ebenso die vier Stellen in
den Pruefmodulen von `fenstermodell.rs`. Die Bauform ist die von
`zettel.rs:147` (`[&str; Zettel::ALLE.len()]`) und von
`Bereichsleiste::bereichsschalter`. Fuer diese Familie ist eine Probe die
falsche Antwort: welches Feld parallel zu einer Aufzaehlung liegt, ist an
seinem Typ nicht abzulesen, also wird nicht geraten, sondern die Zahl
abgeleitet. `Bereichsleiste::bereichsschalter` bleibt unangetastet — sein Feld
entsteht ueber `Bereich::ALLE.map(…)` und ist schon gehalten;
`spaltenschalter` ebenso, dort haelt die Zaehlprobe
`genau_vier_spalten_sind_schaltbar`.

## Gegenproben

Vier Mutationen, jede gefahren und zurueckgenommen:

- `Bereich::Git` in `Bereich::ALLE` durch `Bereich::Editor` ersetzt →
  „diese Varianten von Bereich stehen in keinem Eintrag von Bereich::ALLE
  (krk-ui/src/fenstermodell.rs): Git".
- `Editor` und `Git` vertauscht → Reihenfolgemeldung.
- Eine Ausnahme auf einen Namen gesetzt, den der Baum nicht traegt → rot.
- `Bereich::ALLE` auf fuenf Werte gekuerzt → `cargo build -p krk-ui` meldet
  „expected an array with a size of 5, found one with a size of 6" und nennt
  `crates/krk-ui/src/appkit/aufteilung.rs:319`.

Der Zerleger wird je Datei gegen die rohe Zahl der Zeilen mit `const ALLE: [`
gehalten; ohne das liesse eine geaenderte Schreibweise ihn still nichts mehr
finden. Die Nadel steht mit `concat!` zusammengesetzt da, weil diese Datei in
dem Baum liegt, den sie liest.

## Verifikation

    cargo build --workspace                              — exit 0
    cargo test --workspace                               — exit 0
    cargo clippy --workspace --all-targets -- -D warnings — exit 0
    cargo fmt --all --check                              — exit 0
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps — exit 0

## Abgelegte Datensaetze

- `260907-0858_*_zwei-alle-listen-bleiben-vom-durchlauf-ungedeckt-und-eine-davon-kann-keine-nadel-lesen.md`
- `260907-0859_*_claude-md-nennt-drei-feldlaengen-als-ungehalten-die-der-uebersetzer-seit-dem-260907-haelt.md`
- `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`
  von `_a_` auf `_i_`.

# Coder: Die Ausdruckskiste kommt in den Arbeitsbereich, und ihr Preis wird gemessen

**Datum:** 2026-08-24 09:19
**Status:** Complete
**Agent:** coder
**Baumstand:** `c15f99b`

## Auftrag

Schritt 1 des Plans
`planning/260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`,
Bündel A: `regex = "1"` unter `[workspace.dependencies]` der Wurzel-`Cargo.toml`
aufnehmen, mit einer Begründung in der Form der übrigen Einträge dieser Datei, und die
Kiste in `crates/krk-core/Cargo.toml` unmittelbar nennen. Der Preis wird in diesem
Schritt gemessen und in keinem späteren.

## Was entstanden ist

Die Wurzel-`Cargo.toml` trägt `regex = "1"` hinter `icu_collator` und davor eine
Begründung von 60 Zeilen, die vier Dinge nennt: die Laufzeitzusage aus C2.8 und warum
endliche Automaten sie geben; warum `fancy-regex` trotz seiner Anwesenheit im Baum
ausscheidet; dass keine bestehende Abhängigkeit einen Mustervergleich leistet; und was
mitkommt, gezählt. Dazu ein Absatz, warum die Vorgabemerkmale hier anbleiben, anders als
bei den meisten Einträgen dieser Datei.

`crates/krk-core/Cargo.toml` nennt die Kiste hinter `icu_collator`, mit sechs Zeilen
Begründung: die Auswertung liegt im Kern, weil C6.8 Proben ohne Fenster verlangt und
`krk-ui` kein Bibliotheksziel hat.

## Die Messung

Erhoben mit `cargo tree -p krk-core -e normal` und dem Vergleich der Paketlisten aus
`Cargo.lock` vor und nach der Aufnahme.

| Frage | Erwartung des Plans | Gemessen |
|---|---|---|
| Wie viele Einträge kommen dazu? | ein einziges neues Paket | ein einziges: `regex` 1.13.1, `Cargo.lock` wächst von 97 auf 98 |
| Welche kommen auf dem Bauziel an? | — | das eine; es hängt an keinem Zielvorbehalt |
| `cc` oder ein `-sys`-Paket? | keines | keines; `windows-sys` steht wie zuvor allein in `Cargo.lock` |

Die vier Kisten, auf denen `regex` aufsetzt, standen bereits in genau diesen Fassungen im
Baum, sämtlich über `fancy-regex`: `regex-automata` 0.4.18, `regex-syntax` 0.8.11,
`aho-corasick` 1.1.5 und `memchr` 2.8.3. Die Erwartung des Plans hält damit ohne
Abweichung; die gemessene Zahl und die erwartete sind dieselbe.

Zwei Angaben in der Begründung sind an der Quelle geprüft und nicht übernommen:
`fancy-regex` 0.16.2 setzt `backtrack_limit` in der Vorgabe auf 1.000.000 Schritte und
liefert bei Überschreitung `BacktrackLimitExceeded` (`src/lib.rs:582`, `:672`), und
`regex` 1.13.1 nennt `rust-version = "1.65"`.

## Prüfung

`make check` läuft grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check` und `cargo clippy --workspace --all-targets -- -D warnings`.
Exit-Code 0.

Ein unbenutzter Eintrag unter `[dependencies]` löst in diesem Baum keine Warnung aus:
`unused_crate_dependencies` ist nirgends eingeschaltet. Die Kiste steht damit bis
Schritt 3 ohne Rufer da, ohne dass der Bau daran hängt.

## Was nicht Gegenstand war

Kein Code, der die Kiste nennt: `leseprofil::datei::pruefen` entsteht in Schritt 3. Der
Entscheidungsdatensatz `260824-0541_a_…` und der Defekt `260824-0600_o_…` gehören zu
Schritt 13 und bleiben unberührt.

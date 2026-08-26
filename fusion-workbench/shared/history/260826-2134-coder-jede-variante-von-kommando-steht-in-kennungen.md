# Jede Variante von `Kommando` steht in `KENNUNGEN`

**Agent:** coder
**Datum:** 260826-2134
**Auftrag:** Schritt 4 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`; Datensatz `shared/issues/260826-1223_o_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`
**Ausgangsstand:** HEAD `17e5e4e`
**Status:** Complete

## Was geändert ist

- `crates/krk-core/tests/gemeinsam/mod.rs`: `pub fn varianten_der_aufzaehlung(datei: &str, name: &str) -> Vec<String>`. Nimmt die Datei aus `quelldateien()` (Pfad unterhalb von `crates/`), sucht die Zeile `pub enum <name> {` in Spalte 0, liest bis zur ersten schließenden Klammer in Spalte 0 und liefert je Zeile den Bezeichner vor dem Komma; Doc-Kommentare, gewöhnliche Kommentare, Attribute und Leerzeilen fallen aus. Drei Zusicherungen: die Kopfzeile steht da, der Block ist geschlossen, die Liste ist nicht leer. Eine Zeile, die keine datenlose Variante ist, bricht mit ihrem Wortlaut ab statt still übersprungen zu werden. Der Helfer trägt **keinen** Namen einer bestimmten Aufzählung, damit der zweite Plan ihn für `Wirkungsbereich` wieder aufnehmen kann. Seine drei Blindheiten stehen im Doc-Kommentar, nach dem Vorbild des Modulkopfs von `tests/baum.rs`.
- `crates/krk-core/tests/belegung.rs`: neue Probe `jede_variante_von_kommando_steht_genau_einmal_in_kennungen`. Sie hält die Menge der Varianten aus dem Quelltext gegen die Menge `format!("{kommando:?}")` über `Kommando::KENNUNGEN`, in beiden Richtungen, und nennt die fehlenden Namen in der Meldung. Zwei neue Einbindungen: `std::collections::BTreeSet` (feste Reihenfolge in der Meldung) und `gemeinsam::varianten_der_aufzaehlung`.
- `crates/krk-core/tests/belegung.rs`: der Doc-Kommentar von `jedes_kommando_traegt_genau_einen_wirkungsbereich` sagt jetzt, dass sie die **Eindeutigkeit** hält, warum sie die Vollständigkeit nicht halten kann (sie läuft über die Liste, deren Vollständigkeit die Frage ist) und wer sie hält. Die Probe selbst ist unverändert.
- `crates/krk-ui/src/appkit/menue.rs`: der Doc-Kommentar von `tag_des_kommandos` zitiert für „jedes Kommando genau einmal" beide Proben mit Namen und trennt, welche welche Hälfte trägt. Bis zum 260826 zitierte er allein die Eindeutigkeitsprobe als Beleg für die Vollständigkeit, an der das `expect` darunter hängt.

## Rot vor grün

**Die Mutation des Plans.** `KENNUNGEN` auf `[(Kommando, &'static str); 78]` gesetzt und die Zeile `(Kommando::Notizzettel, "notizzettel"),` (`crates/krk-core/src/tasten/belegung.rs:797`) entfernt. `cargo test -p krk-core --test belegung jede_variante_von_kommando` wörtlich:

```
running 1 test
test jede_variante_von_kommando_steht_genau_einmal_in_kennungen ... FAILED

failures:

---- jede_variante_von_kommando_steht_genau_einmal_in_kennungen stdout ----

thread 'jede_variante_von_kommando_steht_genau_einmal_in_kennungen' (2586434) panicked at crates/krk-core/tests/belegung.rs:1774:5:
diese Varianten von Kommando stehen in keiner Zeile von KENNUNGEN und sind damit unbelegbar: Notizzettel
```

**Die zweite Richtung ist ebenfalls gemessen, und sie braucht eine andere Mutation.** Ein Eintrag in `KENNUNGEN`, der keine Variante benennt, übersetzt in Rust nicht; die Richtung fängt deshalb nicht diesen Fall, sondern einen Helfer, der eine Variante überliest. Gemessen mit `varianten.pop()` am Ende von `varianten_der_aufzaehlung`, bei unverändertem `belegung.rs`:

```
thread 'jede_variante_von_kommando_steht_genau_einmal_in_kennungen' (2587248) panicked at crates/krk-core/tests/belegung.rs:1785:5:
diese Eintraege von KENNUNGEN benennen keine Variante der Aufzaehlung: Notizzettel
```

Beide Mutationen sind vollständig zurückgenommen: `crates/krk-core/src/tasten/belegung.rs` aus der Sicherung wiederhergestellt (`git diff` gegen HEAD leer, Zeile 697 trägt wieder `; 79]`, Zeile 797 den Eintrag), `varianten.pop()` entfernt (`grep -c` liefert 0). Der Baum trägt keine Mutation.

## Prüfung

- `cargo test -p krk-core --test belegung jede_variante_von_kommando` am unveränderten Baum: `1 passed`.
- `cargo test -p krk-core -p krk-ui`: alle Ziele grün, darunter `belegung` mit 50 und `krk-ui` mit 816 Proben.
- `cargo clippy -p krk-core -p krk-ui --all-targets`: ohne Befund.
- `make check` — exit 0, „alle vier gruen".

Ein erster Lauf von `make check` brach im Bau von `krk-bench` ab (`ueber_runden_einig` nicht gefunden, `messen.rs:1105`). Das ist der Arbeitsstand des parallel laufenden Schritts 6 und keine Wirkung dieses Schritts; der Wiederholungslauf nach dessen Landung ist grün.

## Nicht getan

Kein Commit; der Datensatz `260826-1223_o_…` bleibt auf `_o_` und bekommt seine `Resolved:`-Zeile beim Commit durch den Orchestrator. Schritt 5 (`CLAUDE.md` nennt die dritte Pflichtstelle) ist nicht Teil dieses Auftrags; `CLAUDE.md` ist unberührt. Ebenso unberührt: `crates/krk-bench/src/messen.rs` und `bericht.rs`.

# Schritt 8 der Runde 20: der siebte Rang der Statuszeile

**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 8

## Was getan ist

- `crates/krk-ui/src/appkit/statuszeile.rs`
  - `Rang::Seitenzaehler` zwischen `Filterstand` und `Markierungsstand`; `ALLE` ist `[Rang; 7]` in der Reihenfolge aus A5; `art` liefert `Vorgang`.
  - Neue Aufzaehlung `Herkunftsart { Dateifenster, Vorschau }` und `Rang::herkunft`, vollstaendig ohne Auffangzweig; allein `Seitenzaehler` ist `Vorschau`.
  - `Quellen::text` antwortet fuer `Seitenzaehler` mit `None`, Kommentar dazu an der Stelle; `Quellen` behaelt seine sechs Felder.
  - Neue Aufzaehlung `Herkunft { Dateifenster(Fensterseite), Vorschau }`; `Meldung.seite` ist `Meldung.herkunft`.
  - `zeile` traegt den fuenften Parameter `vorschau: Option<&str>` und verzweigt je Rang ueber `herkunft()`: Dateifenster-Raenge wie bisher (aktive Seite zuerst, `sichtbar_in(..., Bereich::von_seite)`), der Vorschau-Rang mit `sichtbar_in(sichtbar, Bereich::Vorschau)` und einem Bewerber.
  - `zeilentext` stellt den Seitennamen genau dann voran, wenn `Herkunft::Dateifenster(seite)` mit `seite != aktiv`; eine Vorschau-Meldung traegt nie einen Zusatz.
  - `pub fn seitenzaehler_text(aktuell, gesamt) -> String` neben `filterstand_text`, ueber `zahl` (Tausenderpunkte).
  - Doc-Kommentare von `Rang`, `ALLE`, `Quellen`, `Meldung`, `zeile` und der Modulkopf ohne Rangzahl; die Bewerber stehen als "zwei je Dateifenster-Rang und einer fuer den Vorschau-Rang". Die Skizze in `zeile` traegt die siebte Zeile.
  - Proben: die zwei `ALLE.len() == 6` stehen auf 7 mit Filterstand < Seitenzaehler < Markierungsstand; `jeder_der_sechs_raenge_hat_genau_ein_feld` heisst `jeder_dateifenster_rang_hat_genau_ein_feld_und_der_vorschau_rang_keines`; Helfer `dateifenster_raenge()` (aus `Rang::herkunft` gelesen) fuer die drei Schleifen, die `nur(rang, ..)` ueber alle Raenge fahren, und `nur` bricht fuer `Seitenzaehler` ausdruecklich ab. Jeder `zeile`-Aufruf der Bestandsproben traegt `None` als fuenftes Argument. Acht neue Proben: Reihenfolge und Herkunft, Verdraengung durch Filtertext und Rueckkehr (C4.5), Vorrang ueber Markierungsstand, Vorgang/Befehlsantwort/Fenstermeldung/Tabmeldung ueber dem Zaehler auch aus dem inaktiven Fenster und `Art::Vorgang` (C4.6), kein Seitenname bei beiden aktiven Seiten, ausgeblendete Vorschau bewirbt sich nicht und kommt zurueck, ohne PDF meldet der Rang nichts (C4.4), `seitenzaehler_text(1, 9)` und `(1200, 3400)` mit Tausenderpunkten (C4.1).
- `crates/krk-ui/src/appkit/anwendung.rs`
  - `statuszeile_nachziehen` holt `self.ivars().vorschau.get().and_then(|v| v.seitenzaehler())` und reicht `vorschau.as_deref()` als fuenfte Eingabe an `statuszeile::zeile`; Doc-Kommentar nennt die fuenfte Eingabe und den dritten Anlass, zwei "zwoelf"-Nennungen in demselben Kommentar sind ohne Zahl.
  - `oberflaeche_aufbauen` setzt unmittelbar nach `Vorschaufenster::bauen` den Seitenmelder mit `objc2::rc::Weak` auf den Delegierten, der `statuszeile_nachziehen` ruft; Muster und Begruendung wie `fenster.melder_setzen`.
  - `kommando_ausfuehren` unberuehrt (Schritt 9).
- `crates/krk-ui/src/appkit/vorschau.rs`: die private `seitenzaehler_text` gestrichen, `seitenzaehler` ruft `super::statuszeile::seitenzaehler_text`; die zwei `#[allow(dead_code)]` mit Ablaufdatum "Schritt 8" an `seitenzaehler` und `seitenmelder_setzen` samt Kommentar entfernt. Die Ausnahmen an `zoomen` und an `Zoom` (Ablauf Schritt 9) stehen noch.
- Plan Schritt 8 auf `[DONE]`.

## Abweichungen vom Plan, mit Grund

1. **Die Probe `ueber_alle_zwoelf_bewerber_gewinnt_genau_eine_aussage` behaelt ihren Namen**: sie setzt alle sechs Dateifenster-Raenge auf beiden Seiten, das sind weiterhin zwoelf Bewerber; die Vorschau tritt in ihr nicht an. Eine Umbenennung haette nichts geprueft.
2. **Zwei Nennungen von "zwoelf" ausserhalb meiner Dateiliste stehen noch**: `appkit/tabelle.rs:19` und `:3152` (Modulkopf und ein Doc-Kommentar) sowie `anwendung.rs:1358` (ein Kommentar in `oberflaeche_aufbauen`, ausserhalb der zwei Stellen, die der Schritt nennt). Sie sprechen von den Bewerbern der Dateifenster und sind nicht falsch, aber nicht mehr vollstaendig; wer die Zahl streicht, tut es in einem Schritt, der jene Dateien anfasst.

## Verifikation

- `cargo fmt --all --check` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` exit 0; `cargo build -p krk-ui` exit 0.
- `cargo test -p krk-ui statuszeile`: 44 von 44 gruen.
- `cargo test --workspace --no-fail-fast`: genau die drei roten Proben, die auf die Belegungseintraege des Ontocoders warten (`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`, `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`). Sonst gruen; krk-ui 837 von 838.
- `make check` exit 2 (bricht im Ziel `test` an der ersten der drei roten Kernproben ab).

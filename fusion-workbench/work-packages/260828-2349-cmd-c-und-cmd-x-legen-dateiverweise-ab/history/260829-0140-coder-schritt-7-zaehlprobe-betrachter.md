# Coder: Schritt 7, die Zählprobe im Betrachter zieht nach

**Circle:** 260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab
**Plan:** planning/260829-0006_p_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md, Schritt 7
**Status:** Complete

## Geändert

- `crates/krk-ui/src/appkit/betrachter.rs`, allein diese Datei.
  - Die Probe `nspasteboard_steht_nicht_im_betrachter_und_copy_genau_einmal` heißt jetzt `nspasteboard_steht_nicht_im_betrachter_und_copy_und_cut_stehen_an_genannten_stellen`. Die erste Hälfte (keine Codezeile nennt `NSPasteboard`) ist unverändert. Die Sammlung ist die Hilfsfunktion `stellen_von(nadel)`, zweimal gerufen: `copy:` erwartet `[("krk-ui/src/appkit/anwendung.rs", 1), ("krk-ui/src/appkit/betrachter.rs", 1)]`, `cut:` erwartet `[("krk-ui/src/appkit/anwendung.rs", 1)]`, beide in der Sortierung von `quelldateien` (sie sortiert, `quellbaum.rs:107`).
  - Der Doc-Kommentar nennt beide Stellen mit ihrem Zweck und sagt, dass die Erwartung die Lage am 260828 ist und keine Zusage über spätere Runden (A5).
  - Der Modulkopf (`# Das Kopieren geht durch die eine Huelle`) nennt den neuen Probennamen und die zweite Antwort beim Delegierten.

## Verifikation

- `cargo test -p krk-ui -- betrachter`: exit 0, nachdem Schritt 5 `copy:` und `cut:` in `anwendung.rs` gespeichert hatte; davor war allein diese Probe rot, wie der Plan es vorsieht.
- `make check`: exit 2. Rot sind drei Proben aus Schritt 3, keine aus meiner Datei: `appkit::zwischenablage::proben::der_zweite_ausgang_legt_verweise_und_namen_ab`, `ein_zweites_ablegen_ersetzt_das_erste`, `eine_verknuepfung_wird_als_verknuepfung_abgelegt`. Allein gefahren (`cargo test -p krk-ui -- zwischenablage`) laufen dieselben drei grün; im vollen Lauf fallen sie, also greifen sie wohl nebeneinander auf dieselbe Ablage. Das ist ein Befund an `zwischenablage.rs`, nicht an Schritt 7. Alle übrigen Ziele von `make check` sind grün (Build, clippy, fmt, alle anderen Kisten).
- Plan Schritt 7 steht auf `[DONE]`.

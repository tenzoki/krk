# Coder: Schritt 9 der Runde 21 — die Proben des Kerns und die Zählprobe

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 9

## Was gebaut ist

`crates/krk-core/tests/verzeichnis.rs`, allein diese Datei. Keine Codezeile darin nennt den Vergleich beim Namen; die Zählprobe liest jede `.rs` unter `crates/`.

Zählprobe:
- `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` heißt jetzt `die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei`. Die Zeichenregel erwartet `krk-core/src/zwischenablage.rs`, `krk-ui/src/appkit/tabelle.rs`, `krk-ui/src/belegungsmodell.rs`; der Vergleich unverändert `durchlauf.rs`, `inhalt.rs`, `modell.rs`. Die Doc sagt, warum die Reinigung `filtertext_aus` der dritte Rufer ist und dass sie im Kern steht (C4.3, C7.1, B8).

Neue Proben je Kriterium:
- C1.1, C1.2: `ein_eingefuegter_text_ist_derselbe_filtertext_wie_fuenf_getippte_zeichen` (Helfer `einfuegemodell`; `n`, `o` getippt, `tiz` eingefügt = `notiz`; Sicht, Filtertext und Muster gleich dem getippten Weg; `tiz.txt` steht nicht; leerer Text ändert nichts, zweites Einfügen hängt weiter an).
- C1.5: `der_rueckschritt_nach_einem_einfuegen_nimmt_ein_zeichen` (`letztes_zeichen_weg` lässt `noti`, `filter_leeren` leert).
- C1.7: `ein_eingefuegter_name_von_fuenf_zeichen_stoesst_den_inhaltsfilter_sofort_an` (tief an, Content an, `text_anhaengen("hallo")` → `inhalt_wirkt`; vier Zeichen nicht; `steht_wegen_des_inhalts` nach gesetztem Befund).
- C2.10, C5.1: `ein_eingefuegter_marker_findet_beide_marker` (ganzer Dateiname mit `_*_` und die Kurzform `260503-1144_*_f1` gegen `_d_`, `_c_`, ohne Marker, andere Nummer).
- C5.4: `ein_stern_am_rand_aendert_die_sicht_nicht` (`*abc`, `abc*`, `*abc*` gegen `abc` über `abc`, `xabc`, `abcx`, `xabcx`, `axbc`).
- C6.1: `der_durchlauf_versteht_das_muster` (`a*z` findet `anzeige.txt` im Unterbaum, nicht `zebra.txt`).
- C6.2: `der_inhalt_versteht_das_muster_ueber_zeilenenden` (`fn*main` über Zeilenenden; `main` vor `fn` trägt nicht; eine Zeile trifft).
- C6.3, B5: `der_name_und_der_inhalt_geben_dieselbe_antwort` läuft mit zwölf Mustern ein zweites Mal (`b*n`, `b**e`, `*nan*`, `ä*l`, `a*l`, `c*é`, `c*e*`, `a*a*a`, `b*a*c`, `*`, `x*z`, `e*b`).
- C6.4, B6: `das_sternchen_zaehlt_nicht_zur_schwelle` (`ab*` nicht, `ab*c` flach; `ab*cd` nicht, `ab*cde` tief; `*****` nie, `filter_steht` dabei).
- C6.6: `ein_einzelnes_sternchen_stoesst_den_durchlauf_an_und_entscheidet_jeden_ordner_mit_dem_ersten_eintrag` — siehe Befund unten.
- `der_kleingeschriebene_filtertext_laeuft_mit` bekommt den Fall `Ab*Cd`: Filtertext behält das `*`, `muster()` ist die Zerlegung, ungleich `Muster::aus("AbCd")`.

## Befund zu C6.6

Der Spec (B6) sagt: „ein einzelnes `*` stößt den Durchlauf an, und weil jeder Name das Muster trägt, entscheidet er jeden Ordner mit dem ersten Eintrag." Am Ordnermodell stimmt der erste Halbsatz nicht: `zeilengrund_von` (`modell.rs:740-773`) stellt den Kurzschluss des Namens vor den Unterbaumzweig, bei `*` trägt jeder Ordnername das Muster, jede Zeile steht wegen ihres Namens, und `auftraege()` ist leer. Der Durchlauf bekommt nichts. Die Probe hält beides: am Modell `filter_steht()` wahr, `auftraege()` leer, jede Zeile steht; am `Durchlauf` mit von Hand vergebenen Aufträgen und `Muster::aus("*")` jeder Ordner `treffer: true` und `zu_gross() == 0` trotz Dateien über der Inhaltsgrenze in jedem Ordner und Unterordner (der erste Eintrag entscheidet, nichts wird gelesen). Das Verhalten ist günstiger als der Spec beschreibt und kein Defekt; die Prosa von B6 ist an dieser Stelle ungenau. Kein Datensatz angelegt.

## Verifikation

- `cargo test -p krk-core` — exit 0, alle Ziele grün (tests/verzeichnis.rs: 97 passed, 3 ignored).
- `make check` — exit 0, nichts rot außerhalb der Datei.

## Anmerkung

`cargo fmt --all` ist zweimal gelaufen und hat den ganzen Arbeitsbereich formatiert, nicht nur diese Datei. Ob es fremde, parallel bearbeitete Dateien berührt hat, habe ich nicht geprüft, weil ein baumweites git-Kommando untersagt war. `make check` (mit `cargo fmt --all --check`) ist danach grün.

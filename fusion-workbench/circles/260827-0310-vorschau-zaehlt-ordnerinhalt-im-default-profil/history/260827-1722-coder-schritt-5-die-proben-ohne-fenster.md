# Coder: Schritt 5 des Plans der Runde 19, die Proben ohne Fenster

**Status:** Complete
**Agent:** coder
**Zeit:** 260827-1722
**Plan:** `planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, Schritt 5

## Was gebaut ist

Fünf Dateien, ausschließlich Proben; kein Produktionscode angefasst, weil keine Probe einen Defekt aufgedeckt hat.

`crates/krk-core/tests/leseprofil.rs`

- Rundreise über die vier Bausteine nimmt `typ = "datei", versteckt = true` auf und hält beide Felder.
- `ein_unbekannter_typ_oder_ein_nicht_wahrheitswert_fuer_versteckt_kostet_die_ganze_datei` (C3.6, C3.1): vier falsche Typwerte, vier Nicht-Wahrheitswerte, sieben gültige Formen.
- Neuer Abschnitt „Die drei Zählzeilen des eingebauten Default-Profils" mit dem Prüfordner `zaehlbestand` (vier Dateien, drei Ordner, drei Verknüpfungen, je eine versteckt, hundert Dateien darunter):
  - `die_drei_zaehlzeilen_zaehlen_nach_typ_flach_und_beziffern_die_versteckten` (C2.1, C2.3, C2.4, C2.8, C2.9, C2.6 erste Hälfte); Summe gegen `read_dir`, Text gegen `zeilen_als_text`.
  - `ein_leerer_ordner_zeigt_drei_zeilen_mit_null_und_null` (C2.5).
  - `ein_ueber_chflags_versteckter_eintrag_zaehlt_wie_einer_mit_punkt` (C2.6 zweite Hälfte, Datei und Ordner).
  - `ueber_der_schranke_sagen_die_drei_zeilen_mindestens_und_tragen_keine_klammer` (C2.10, C4.4).
  - `ein_ordner_ohne_leserecht_zeigt_drei_platzhalter_unter_ihren_beschriftungen` (C2.11).
  - `die_zaehlung_nimmt_typ_und_versteckt_an_und_zaehlt_ohne_sie_wie_zuvor` (C3.1, C3.2, C3.3).
  - `ein_eigenes_profil_liefert_dieselbe_zeile_wie_das_default_profil` (C3.5).
  - `die_drei_zaehlzeilen_kosten_einen_leselauf_und_keine_oeffnung_auch_nach_der_erkennung` (C4.1, C4.2, C4.5): vier Profilsätze, je ein Lauf, null Öffnungen.
  - `eine_verknuepfung_auf_einen_ordner_bekommt_keine_zaehlzeilen` (C1.7, A4) samt Gegenprobe über ein Pfadmuster.
- Kindprobe `kind_fasst_mit_einem_freien_deskriptor_zusammen` auf den Rückfallweg ausgedehnt (C4.3): ohne freien Deskriptor keine Zahl, mit genau einem alle drei.

`crates/krk-core/tests/baum.rs`

- `genau_drei_dateien_lesen_das_kennzeichen_versteckt_und_fragen_nach_dem_typ` (C3.7): Nadeln `.versteckt` und eine der vier Typfragen, unter `crates/*/src`, die drei Dateien beim Namen.
- `keine_code_zeile_unter_leseprofil_erreicht_den_ausblendeschalter` (C2.7, strukturelle Hälfte): `verstecke_*` und `Ordnermodell` fehlen unter `leseprofil/`, Gegenprobe an `modell.rs`.

`crates/krk-core/src/ablage/leseprofile.rs`, Prüfmodul

- `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` (C3.4): Nicht-Kommentarzeilen ohne `typ =`/`versteckt =`, zwölf Profile, keine geprüfte Zählung mit Typ oder Klammer.

`crates/krk-ui/src/vorschaumodell.rs`, Prüfmodul

- `ein_ordner_mit_leerem_profilsatz_traegt_drei_zaehlzeilen_unter_seinen_metadaten` (C1.1, C2.1, C2.2).
- `eine_verknuepfung_und_eine_datei_tragen_keine_zaehlzeile` (C1.6, C1.7).

`crates/krk-ui/src/appkit/vorschau.rs`, Prüfmodul

- `die_zaehlzeilen_folgen_in_metadaten_text_auf_die_zeile_typ` (C2.1, C2.2, strukturell): ein Rufer von `zeilen_als_text`, im Rumpf von `metadaten_text`, Formatzeile endet auf `Typ: {}{}`.

## Was anders gebaut ist als im Plan genannt

- C3.7: die Probe zählt unter `crates/*/src` und nicht über den ganzen Baum. Über den ganzen Baum fand die Nadel als vierte Datei `krk-core/tests/verzeichnis.rs`, die beide Fragen an gelesene Einträge stellt und nichts gruppiert. Der Plan sagt selbst „unter `crates/*/src`"; die erste Fassung der Probe hatte das nicht eingegrenzt.
- C2.1/C2.2 in `vorschau.rs`: der fertige Text der sechs Zeilen mit den drei darunter ist ohne Instanz nicht zu prüfen (die Formatierer von AppKit hängen an `ivars`), und der Kopf des Prüfmoduls dort schließt Proben aus, die den Hauptfaden behaupten. Gebaut ist deshalb eine strukturelle Probe; der gerenderte Text bleibt Schritt 8.

## Abnahme

`make check` — exit 0 (260827-1722). Neue Proben je Kiste grün: `cargo test -p krk-core --test leseprofil --test baum` 57 + 6 bestanden, 1 Kind ignoriert und vom Elternteil gefahren.

## Nicht angefasst

`resources/default-readers.toml`, alle Datensätze unter `fusion-workbench/` außer diesem Eintrag und dem Marker `[DONE]` an Schritt 5 des Plans. Keine git-Kommandos.

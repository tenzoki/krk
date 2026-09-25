# Schritt 8: die drei fehlenden Probenpflichten der Ablagehälfte

**Datum:** 260824-1457 bis 260824-1520
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel C, Schritt 8
**Baumstand vorher:** der Produktionscode von Schritt 8 lag unversioniert im Arbeitsbaum, `make check` grün

---

## Auftrag

Die drei Probenpflichten nachtragen, die Schritt 8 offen ließ: C1.1 mit C1.2, C1.7 und die
Hälfte `Grund::NichtLesbar` von C1.6. Am Produktionscode nichts ändern.

## Was entstanden ist

Alle drei Proben stehen in `crates/krk-core/tests/ablage.rs`, in einem neuen Abschnitt
`Die von Hand gepflegten Leseprofile (C1 der Runde 16)` unmittelbar hinter dem Abschnitt
der von Hand gepflegten Einstellungen, dessen Proben ihre Vorlage sind.

| Probe | Zeile | Kriterium |
|---|---|---|
| `eine_fehlende_readers_toml_entsteht_byteweise_und_bleibt_beim_zweiten_start_liegen` | 1951 | C1.1 und C1.2 |
| `eine_nicht_anlegbare_readers_toml_meldet_sich` | 2017 | C1.7 |
| `eine_nicht_lesbare_readers_toml_ergibt_kein_profil_und_eine_meldung` | 2059 | C1.6, Hälfte `NichtLesbar` |

### C1.1 und C1.2

Der Byte-für-Byte-Vergleich geht gegen `krk_core::ablage::leseprofile::AUSLIEFERUNGSTEXT`
und nicht über eine Zählung von Kommentarzeilen; der Doc-Kommentar sagt, warum die Vorlage
`eine_fehlende_settings_toml_liefert_die_vorbelegung_und_entsteht_mit_kommentaren` zählen
darf und diese Probe nicht. Daneben stehen zwei Zusicherungen, die die erste Abweichung aus
dem Modulkopf von `ablage/leseprofile.rs` messen: die angelegte Datei entsteht ohne Meldung,
und schon die Sitzung, die sie anlegt, arbeitet mit ihren fünf Profilen.

Der zweite Teil ist der **veränderte** Fall aus C1.2. Der geleerte steht in
`eine_leere_datei_meldet_bei_den_vier_uebrigen_toml_dateien_nichts` (Zeile 3306) und wird
hier nicht wiederholt.

### C1.7

Der Weg ist der von `eine_nicht_anlegbare_settings_toml_meldet_sich`: der Ablageordner
verschwindet zwischen `Ablage::oeffnen` und dem Laden, damit die Probe ohne entzogene
Rechte auskommt. Der Doc-Kommentar schreibt die Abweichung gegenüber den Einstellungen aus
und verweist für ihre Begründung auf den Modulkopf von `ablage/leseprofile.rs`, Abschnitt
„Zweite Abweichung"; zitiert wird sie nicht. Gemessen wird die Abweichung an
`profile.wert.zahl() == 0`.

### C1.6, Hälfte `NichtLesbar`

Nachgeprüft: keine vorhandene Probe trug sie für `readers.toml`. Die zwei Stellen im Baum,
die `Grund::NichtLesbar` prüfen, nehmen `bookmarks.toml` (Zeile 1069, über `pruefe_meldung`)
und `session.toml` (Zeile 1296). Die beschädigte Hälfte von C1.6 nimmt dagegen
`jede_toml_datei_wird_bei_beschaedigung_zur_seite_gelegt` (Zeile 1145) über `toml_dateien`
schon mit; ein Ordner an der Stelle der Datei lässt sich nicht über alle fünf Dateien
setzen, deshalb steht die nicht lesbare Hälfte als eigene Probe da.

## Eine Änderung außerhalb der drei Proben

`LESEPROFILTEXT` (Zeile 407) trug bis heute allein die Kommentarzeile
`# von Hand gepflegt\n`. Damit hätte der „veränderte" Fall aus C1.2 dieselben null Profile
geliefert wie der geleerte und wäre von ihm nicht zu unterscheiden gewesen. Die Konstante
trägt jetzt ein vollständiges Profil `eigener Ordner` mit einer Zeile `Dateien`; ihr
Doc-Kommentar schreibt den Grund aus. Für den Rundlauf
`alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` ist der Inhalt weiterhin bloße
Nutzlast, die Probe prüft Bytegleichheit und läuft unverändert grün.

Der Import `use krk_core::leseprofil::Profile;` ist zu `{Profil, Profile}` geworden: die
Probe zu C1.2 liest die Namen der geladenen Profile.

## Was nicht geändert wurde

Am Produktionscode nichts. Keine der drei Proben hat einen Fehler aufgedeckt; alle drei
liefen im ersten Lauf grün.

## Abnahme

`make check` — exit 0 (`cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`).
Das Testziel `ablage` fährt 74 Proben, davon die drei neuen.

## Ein Befund am Rande, nicht behoben

`Ersetzung`s `Display` (`crates/krk-core/src/ablage/mod.rs:392`) schreibt in jedem seiner
fünf Zweige „und wird durch den Auslieferungszustand ersetzt". Für `readers.toml` ist das
der Sache nach unzutreffend: dort tritt nach der zweiten Abweichung **kein** Profil an die
Stelle der Datei und nicht die Auslieferungsfassung. Der Satz ist der gemeinsame Satz aller
sieben Ablagedateien und war schon vor dieser Runde so; die vorhandene Probe
`eine_kaputte_datei_fuehrt_zum_auslieferungszustand_und_zu_einer_meldung` nimmt ihn für
`readers.toml` bereits ab. Eine Änderung daran ist Prosaarbeit an einer Nutzermeldung und
gehört nicht in diesen Auftrag.

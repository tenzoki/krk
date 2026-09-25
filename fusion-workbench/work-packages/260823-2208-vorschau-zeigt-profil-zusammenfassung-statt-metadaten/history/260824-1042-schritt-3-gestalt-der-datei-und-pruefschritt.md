# Schritt 3 der Runde 16: die Gestalt der Datei und der Prüfschritt dahinter

**Status:** Complete
**Datum:** 260824-1042
**Agent:** coder
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel B, Schritt 3
**Spec:** `planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, C1, C2, C3

---

## Was gebaut ist

Das Modul `krk_core::leseprofil` steht, in zwei Dateien und ohne einen Rufer außerhalb der
Proben. `mod.rs` trägt die Werttypen aus `## Data Structures` des Plans — `Profile`, `Profil`,
`Zeile`, `Baustein`, `Ortsangabe`, `Zusammenfassung`, `Zusammenfassungszeile`, `Wert`,
`Haushalt` — samt den fünf Zahlen des Haushalts als Konstanten. `datei.rs` trägt die Gestalt
der TOML-Datei (`Profildatei`, `Profilblock`, `Zeilendatei`, `Bausteindatei` als unmarkierte
Auswahl über `#[serde(untagged)]`, eingebettet über `#[serde(flatten)]`, dazu die vier
Bausteintische) und den Prüfschritt `pruefen(Profildatei) -> (Profile, Vec<String>)`. `lib.rs`
bekommt `pub mod leseprofil;` und einen Absatz im Modulkopf über die neue Schicht.

`pruefen` ist die eine Stelle, an der ein Muster zu einem `Regex` wird. Die Übersetzung selbst
steht als `uebersetzen` an genau einer Stelle in `datei.rs`; jeder der vier Rufer darunter
(`erkennungsmuster`, `muster`, `wahlfreies_muster`, `feldmuster`) setzt allein seinen Satz für
die Meldung davor.

**Die drei Dinge, die der Modulkopf ausschreibt**, stehen als drei eigene Abschnitte: warum die
Auswertung im Kern liegt und nicht in `krk-ui` (C6.8 verlangt Proben ohne Fenster, `krk-ui` hat
kein Bibliotheksziel), warum die Bausteinauswahl unmarkiert ist und wo ihre Vorlage steht
(`ablage::lesezeichen::Ziel`, samt dem Vorbehalt zu `flatten` und der Rundreise, die ihn
abnimmt), und warum jede Prüfung beim Laden läuft und nicht beim Anzeigen.

## Die Reichweite der Abweisungen

Zwei Reichweiten, und die Umsetzung ordnet den Plan als Regel und nicht als Liste ein:

- **Das ganze Profil fällt weg**, wenn eines seiner beiden Erkennungsmuster sich nicht
  übersetzen lässt (C2.7) oder wenn es keines von beiden nennt. Die übrigen Profile bleiben.
- **Die Zeile behält ihre Beschriftung und verliert ihren Baustein**, wenn ein Muster darin
  sich nicht übersetzen lässt, wenn das Feldmuster nicht genau eine Fanggruppe trägt (C3.10)
  oder wenn die Ortsangabe schon am Text herausführt (C3.13, textliche Hälfte).

`anzahl` über 10 wird gekappt und nicht abgewiesen (C6.3), ohne Meldung.

Jede Meldung nennt den Profilnamen, bei einer Zeile deren Beschriftung, und den Grund. Die
mehrzeilige Fehlerbeschreibung von `regex` wird auf eine Zeile gebracht, weil die Statuszeile
eine Zeile ist.

## Der Risikoposten des Plans ist abgetragen

Der erste Eintrag unter `## Risks & Mitigations` lautet: „`#[serde(flatten)]` über einer
unmarkierten Auswahl mit vier Varianten trägt in `toml` nicht so weit wie über einer mit zwei."
Die Rundreise `eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte` läuft
grün, und zwar über eine Datei, in der jede der vier Sorten genau einmal steht. Der im Plan
benannte Ausweg (ein ausgeschriebenes `baustein = "zaehlung"` und ein von Hand geschriebener
Prüfschritt) wird nicht gebraucht; er bleibt im Modulkopf von `mod.rs` stehen, damit er nicht
neu zu suchen ist, falls die Probe eines Tages fällt.

## Proben

Neu: `crates/krk-core/tests/leseprofil.rs` mit sieben Proben ohne Fenster und ohne Dateisystem,
dazu drei `#[cfg(test)]`-Proben in `leseprofil/mod.rs` zu `Ortsangabe` und `Haushalt`.

| Probe | Kriterium |
|---|---|
| `eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte` | die Gestalt der Datei, alle vier Bausteine |
| `ein_unuebersetzbares_pfadmuster_nimmt_nur_sein_eigenes_profil_weg` | C2.7 |
| `ein_profil_ohne_erkennung_faellt_weg_und_die_uebrigen_bleiben` | die zweite Profilabweisung |
| `ein_feldmuster_ohne_genau_eine_fanggruppe_nimmt_der_zeile_ihren_baustein` | C3.10, in beiden Abweichungen (zwei Gruppen und keine) |
| `eine_ortsangabe_die_herausfuehrt_nimmt_der_zeile_ihren_baustein` | C3.13, textliche Hälfte, über sechs Angaben |
| `eine_anzahl_ueber_der_grenze_wird_gekappt_und_nicht_abgewiesen` | C6.3 |
| `eine_datei_ohne_profilblock_liefert_keine_profile_und_keine_meldung` | C1.5 |
| `eine_ortsangabe_traegt_gewoehnliche_namensbestandteile` (mod.rs) | die Zerlegung der Angabe |
| `eine_ortsangabe_die_herausfuehrt_wird_schon_am_text_abgewiesen` (mod.rs) | die drei Mängelsorten einzeln |
| `der_haushalt_zaehlt_die_tatsaechlichen_laeufe_und_nicht_die_versuchten` (mod.rs) | der Haushalt |

## Was dieser Schritt nicht tut

Die Auswertung selbst — Erkennung, Bausteine, `zusammenfassen`, `als_text` — ist Sache der
Schritte 5 und 6. `Haushalt` trägt hier seine zwei Zähler und seine Grenzen, aber keinen Rufer;
`Zusammenfassung` und `Wert` stehen als Typen da und werden von nichts erzeugt.

Kein Entscheidungsdatensatz geht auf `_i_`. Die drei, die der Plan diesem Schritt zuordnet
(`260824-0541_a_wie-zieht-der-baustein-…`, `260824-0541_a_was-zeigt-die-zusammenfassung-…`,
`260824-0600_a_welche-form-hat-das-pfadmuster-…`), teilt er sich jeweils mit Schritt 5 oder 6;
ihre Antwort ist damit erst nach jenen ganz in Code umgesetzt.

`bis_zur_grenze_lesen` und `anlesen` kommen in diesem Schritt nicht vor: er liest keine Datei.
Der Datensatz `issues/260824-1014_o_c3-14-nennt-bis-zur-grenze-lesen-…` bekommt deshalb keine
`Also seen:`-Zeile von hier.

## Abgelegte Befunde

- `issues/260824-1042_o_schritt-3-zaehlt-vier-abweisungen-auf-ein-unuebersetzbares-muster-in-einem-baustein-ist-eine-fuenfte.md`
  — der Plan zählt vier Abweisungen auf; ein unübersetzbares Muster **innerhalb** eines
  Bausteins ist ein fünfter Fall, der am Baum zwangsläufig auftritt. Die Umsetzung ordnet ihn
  als Zeilenabweisung ein und begründet das im Modulkopf von `datei.rs`.

## Prüfung

`make check` (`cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`) — Exit 0.

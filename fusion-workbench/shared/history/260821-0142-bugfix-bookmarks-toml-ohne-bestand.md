# Bugfix: eine `bookmarks.toml`, die serde toleriert aber nicht versteht

**Date:** 2026-08-21 01:42
**Status:** Complete
**Trigger:** Issue file — `shared/issues/260820-2235_*_eine-bookmarks-toml-die-serde-toleriert-aber-nicht-versteht-wird-still-als-leer-gelesen.md`, vom Nutzer am 260820-2250 zur Behebung ausgewählt

## Error

Die Zusage der Runde 6 — eine Ablagedatei, die KRK nicht versteht, wird über
`atomar::beiseitepfad` gesichert, bevor der Auslieferungszustand einspringt — deckte für
`bookmarks.toml` nur den syntaktischen Fehlschlag. Zwei Gestalten kamen daran vorbei und
führten zu einer stummen leeren Liste, die der nächste Lesezeichenbefehl festschrieb.

Gemessen am Baumstand `01d2365`, Tabelle im Defektdatensatz. Zeile 2: oberster Schlüssel heißt
anders, etwa `[[lesezeichen]]` — null Einträge, `Ok`, keine Meldung, nichts beiseitegelegt.
Zeile 3: Datei ist 0 Bytes — dasselbe.

## Root Cause

Der Ladeweg stellte die falsche Frage, und die Wurzel liegt an zwei Stellen, nicht an einer.

**`crates/krk-core/src/ablage/lesezeichen.rs`, `Lesezeichenliste`.** Die Struktur trug
`#[serde(default)]` ohne `#[serde(deny_unknown_fields)]`. serde übergeht damit jeden obersten
Schlüssel, den sie nicht kennt, und setzt für das fehlende `eintraege` die leere Liste ein. Eine
`bookmarks.toml`, deren Bestand unter einem anderen Namen steht, ist damit **gültiges TOML mit
null Einträgen** und kein Fehler. `Belegungsdatei` (`tasten/belegung.rs:1587`) und
`Einstellungsdatei` (`ablage/einstellungen.rs:125`) tragen den Vermerk seit jeher; die
Lesezeichenliste war die Ausnahme, und dass sie es war, stand nirgends.

**`crates/krk-core/src/ablage/mod.rs`, `Zugang::laden`.** Der `Ok`-Zweig fragte nichts weiter.
Eine Struktur, deren Felder alle einen Auslieferungswert haben, nimmt auch das leere Dokument
widerspruchslos an, und danach ist der gelesene Wert von einem echten Bestand nicht mehr zu
unterscheiden. `beiseite_legen` steht ausdrücklich allein im Zweig `Grund::Beschaedigt`, und
dorthin kam keine der zwei Gestalten.

**Beleg für „kein erster Start":** eine leere `Lesezeichenliste` serialisiert zu
`eintraege = []`, also zu einem obersten Schlüssel. Gemessen mit einem Wegwerf-Prüfprogramm über
`toml::to_string`, danach als Probe
`eine_leere_liste_steht_als_oberster_schluessel_in_der_datei` in den Baum gestellt. KRK schreibt
`bookmarks.toml` nie ohne obersten Schlüssel; eine Datei ohne einen einzigen stammt nicht aus
seiner Feder.

## Fix

Der Ladeweg fragt jetzt „hat die gelesene Datei den Bestand hergegeben, den sie trägt". Die
Frage hat zwei Hälften, und beide münden in denselben Zweig `Grund::Beschaedigt` und damit in
`Zugang::beiseite_legen`. Ein zweiter Mechanismus daneben entsteht nicht, und
`nur_benannte_dateien_erreichen_das_atomare_schreiben` in `krk-core/tests/baum.rs` zählt
unverändert fünf Dateien.

| File | Change |
|------|--------|
| `crates/krk-core/src/ablage/lesezeichen.rs:329` | `#[serde(default, deny_unknown_fields)]` an `Lesezeichenliste`, mit dem Grund im Doc-Kommentar: warum an der Liste und nicht am Eintrag, und warum er am Eintrag technisch nicht zu haben wäre (`deny_unknown_fields` schließt `#[serde(flatten)]` aus) |
| `crates/krk-core/src/ablage/pfade.rs` | Neu: `enum Leerbefund { Vorgabe, Beschaedigt }` und `Datei::leerbefund`, eine vollständige Fallunterscheidung ohne Auffangzweig nach dem Vorbild von `Datei::format`. `bookmarks.toml` trägt `Beschaedigt`, die fünf übrigen `Vorgabe` |
| `crates/krk-core/src/ablage/mod.rs` | `Zugang::laden` prüft vor dem Lesen des Wertes, ob die Datei bei `Leerbefund::Beschaedigt` überhaupt einen obersten Schlüssel trägt; neue Hilfsfunktion `ohne_obersten_schluessel`, die die Frage am TOML-Dokument stellt und ungültiges TOML dem bestehenden Zweig überlässt. `Leerbefund` wird mit exportiert |
| `crates/krk-core/src/ablage/mod.rs` (Modulkopf) | Neuer Abschnitt „Beschädigt heißt nicht ‚ungültiges TOML'". Der Absatz „alle vier … haben dort keinen eigenen Zweig" stimmte nach der Änderung nicht mehr und ist berichtigt; `Grund::Beschaedigt` und die Doku von `Zugang::laden` ziehen nach. Der Abschnitt benennt auch, was die Zusage weiterhin **nicht** deckt |
| `crates/krk-core/tests/ablage.rs` | Sechs Proben plus eine gemeinsame Prüfstelle |

### Was bewusst nicht mitgekommen ist

**Gestalt 2 ist nicht behoben.** „Eine Datei, die sich nicht lesen ließ, darf nicht überschrieben
werden" ist schmal formuliert und in der Wirkung nicht schmal: der Lesezeichenbefehl des Nutzers
täte dann nichts, und was er stattdessen sieht, ist zu entscheiden. Drei Folgefragen hängen
daran und sind aus dem Baum nicht zu beantworten. Der Befund steht mit dem Gemessenen als
eigener Datensatz.

**Die Verallgemeinerung auf `session.toml` und `keymap.toml` ist nicht im Vorbeigehen getroffen.**
Der Defektdatensatz nennt sie ausdrücklich als zu entscheiden. Für den Lauf steht die Fassung,
die nichts am Verhalten ändert; die Frage liegt als Entscheidungsdatensatz vor, und eine Probe
schreibt die heutige Antwort aus, damit sie nicht stillschweigend wandert.

**Ein unbekanntes Feld im einzelnen Eintrag bleibt getragen** — die vierte Zeile der Messtabelle
ist unverändert. Das ist die Vorsorge, die der Kopf von `Lesezeichen` beschreibt: eine
`bookmarks.toml` aus einer späteren Runde bleibt in einer früheren lesbar.

## Verification

- [x] Original error resolved — die zweite und dritte Zeile der Messtabelle führen jetzt zu
      `Grund::Beschaedigt`, einer Meldung und einer Sicherung unter `atomar::beiseitepfad`; die
      Datei selbst bleibt liegen
- [x] Full test suite passes — `make check`, Exit 0 (Bau, Proben, clippy mit `-D warnings`, fmt)
- [x] No regressions introduced — `eine_bookmarks_toml_aus_der_zeit_vor_den_textmarken_bleibt_lesbar`
      und `eine_rundreise_ueber_beide_sorten_liefert_dieselbe_datei` laufen unverändert; die drei
      Nachsichtsproben von `session.toml` (`ablage.rs:511`, `:551`, `:611`) ebenso

## Unrelated Issues Found

`shared/issues/260821-0142_*_eine-nicht-lesbare-ablagedatei-wird-nicht-gesichert-und-vom-naechsten-schreibvorgang-ueberschrieben.md`
— Gestalt 2 desselben Verlusts, abgetrennt statt behoben, mit den drei Folgefragen, die sie zu
einer Nutzerentscheidung machen.

`shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`
— die Frage, die der Defektdatensatz als zu entscheiden bezeichnet, mit drei Optionen und der
Messung, die jede von ihnen voraussetzt.

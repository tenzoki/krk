# Die Beispielzahl „vier" des Projektwurzelprofils hält jetzt eine Probe

**Agent:** coder
**Datum:** 2026-08-26, ab 01:40
**Aufgabe:** S-2, Runde 3 der Sitzung zur Runde 18 — Befund N3 der Nachdurchsicht
`fusion-workbench/shared/reviews/260825-2233-ontorev-nachdurchsicht-der-leseprofile-nach-der-behebungsrunde.md`,
Datensatz `shared/issues/260825-2233_*_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md`
**Status:** Complete

## Was geändert ist

Eine Datei: `crates/krk-core/tests/leseprofil.rs`. Möglichkeit 1 des Datensatzes.
`resources/default-readers.toml` ist nicht angefasst.

**Ein vierter Fall in `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`.**
Er misst das Profil `Projektwurzel mit fusion-Werkbank` an einem Prüfordner und hält vier
Aussagen: die drei Orte, die das Profil nennt; dass keiner davon der erkannte Ordner selbst ist;
`(4, 5)` als Leseläufe und Öffnungen genau; und die Vier ein zweites Mal als `orte.len() + 1`.
Die zwei Fassungen der Vier stehen nebeneinander, weil sie verschiedene Dinge sagen: die eine
zählt, die andere leitet her.

**Der Prüfordner ist eine Ebene höher als bei der Werkbankwurzel.** Das Profil erkennt über
`kennzeichen = '^fusion-workbench$'`, also über einen Eintrag im ausgewählten Ordner. An der
Wurzel steht deshalb nichts als dieser eine Ordner: der zweite Erkennungsdurchgang nimmt das
erste Profil mit Treffer, und `^\.fusion-setup$` wie `^_._circle\.md$` stehen vor
`^fusion-workbench$`.

**Der Bestand steht jetzt an einer Stelle statt an zweien.** `werkbankwurzel` schrieb ihn bisher
selbst; er ist als `werkbankbestand(&Path)` herausgezogen, und die zwei Erzeuger unterscheiden
sich allein darin, wohin sie ihn legen. Die sieben Zeilen des Projektwurzelprofils sind die des
Wurzelprofils, jede mit `fusion-workbench/` vor der Ortsangabe; zwei von Hand gepflegte Bestände
liefen auseinander, und die zwei Messungen verglichen dann nicht mehr dieselbe Gestalt.

**`profil_der_auslieferung(&profile, name)`** löst `speicherprofil_der_auslieferung` ab, das jetzt
darauf aufsetzt. Ein Profil wird über seinen Namen gegriffen und nicht über seine Nummer in der
Datei.

## Gemessen, nicht übernommen

Alle Zahlen über `zusammenfassen_gezaehlt` an Prüfordnern gemessen, am 260826 gegen den Baum
`20eccd4` plus die parallele Arbeit an `default-readers.toml`:

| Gestalt | Leseläufe | Öffnungen |
|---|---|---|
| Projektwurzel, voller Bestand | 4 | 5 |
| dieselbe ohne `.active-circle` | 4 | 4 |
| Projektwurzel mit leerem `fusion-workbench` | 2 | 0 |
| dieselbe mit `.fusion-setup` an der Wurzel | 1 | 3 |

**Die Vier stimmt, die Öffnungszahl der Aufgabenstellung nicht.** Die Aufgabe nannte vier
Leseläufe und vier Öffnungen aus der Kostenmessung `shared/analyses/260825-2107-…`. Vier Öffnungen
sind die Zahl der **wirklichen** Werkbank: `krk/fusion-workbench` führt kein `.active-circle`, und
eine Zeile, die ihre Datei nicht findet, öffnet nichts. Der Prüfordner trägt den vollen Bestand,
also fällt die fünfte Öffnung an. Zeile 2 der Tabelle ist die Gegenprobe dazu: dieselbe Gestalt
ohne diese eine Datei liefert die vier der Kostenmessung. Die Probe hält deshalb `(4, 5)`, und die
Herleitung steht im Doc-Kommentar.

**Zeile 3 sagt, warum der Prüfordner Inhalt braucht.** Ein leeres `fusion-workbench` kostet zwei
Läufe: ein Ort, den es nicht gibt, wird nicht gelesen. Die Vier steht nur an einer eingerichteten
Werkbank.

**Zeile 4 sagt, warum die Beschriftungsliste hier nicht der ganze Ausweis ist.** Die drei Fälle
davor weisen das gegriffene Profil über die Beschriftungen der Zusammenfassung aus. Bei diesem
vierten reicht das nicht: das Wurzelprofil führt dieselben sieben Beschriftungen, es sind dieselben
sieben Zeilen. Erst die Werte trennen die zwei — an einer Projektwurzel sieht das Wurzelprofil in
den ausgewählten Ordner selbst und fände dort nichts. Die Werteliste steht deshalb im vierten Fall
ausgeschrieben und **nicht** als Vergleich gegen `wurzelwerte`: was die zwei Profile aneinanderhielte,
wäre eine Zusage, die `default-readers.toml` bei `:637-639` für sich ausdrücklich nicht gibt
(„Sie stehen zweimal in dieser Datei und können auseinanderlaufen; nichts hält sie aneinander").

## Gegenproben

Drei Verstellungen, je einzeln gefahren und wieder zurückgenommen:

| verstellt | Ergebnis |
|---|---|
| Erwartung `(4, 5)` auf `(4, 4)` | rot, `left: (4, 5)` gegen `right: (4, 4)` |
| Herleitung auf `projektorte.len()` ohne das `+ 1` | rot, `left: 4` gegen `right: 3` |
| `.active-circle` aus `werkbankbestand` gestrichen | rot, und zwar zuerst am **Wurzel**fall (`(3, 4)` statt `(3, 5)`) |

Die dritte belegt zweierlei: die fünfte Öffnung ist ein wirklicher Treffer und keine Buchung, und
die zwei Erzeuger teilen sich tatsächlich einen Bestand.

## Abnahme

`make check` — Rückgabewert 0, alle vier Kommandos grün. `cargo test -p krk-core --test leseprofil`:
47 Proben grün, dieselbe Zahl wie vorher; der vierte Fall steht in einer bestehenden Probe und
legt keine neue an.

## Was ausdrücklich nicht angefasst wurde

`resources/default-readers.toml` (ein `ontocoder` arbeitet parallel darin; der feste Stand ist über
`git show HEAD:resources/default-readers.toml` gelesen), `crates/krk-core/src/`, `crates/krk-ui/`,
`Cargo.toml`, `CLAUDE.md`. Kein Commit, kein Git-Kommando über den ganzen Baum.

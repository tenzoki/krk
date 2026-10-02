# Fünf Befunde an `default-readers.toml` behoben

**Agent:** ontocoder
**Datum:** 2026-08-25, 22:25 bis 22:40
**Aufgabe:** R-1, Runde 2 der Sitzung zur Runde 18 — die fünf Datensätze
`fusion-workbench/shared/issues/260825-2126_*_…` aus der Durchsicht
`shared/reviews/260825-2126-ontorev-die-zwoelf-leseprofile-der-auslieferungsfassung.md`.
**Status:** Complete

## Was geändert ist

Eine Datei im Baum: `resources/default-readers.toml` (58 Zeilen dazu, 17 weg). Ein Muster
und fünf Absätze des Handbuchteils.

1. **Sitzung.** Das `feldmuster` der Zeile „Sitzung" trägt an beiden Stellen (Wurzelprofil
   und Projektwurzelprofil) das `(?m)^` vor `## Current`; der Kommentar über dem
   Wurzelprofil sagt, was es abhält.
2. **Öffnung der Zeile „Projekt".** „teilt sich ihre Öffnung, also nichts" ist ersetzt:
   geteilt wird der Leselauf über einen Ort, jeder Feldbaustein öffnet selbst, drei Zeilen
   über `.fusion-setup` kosten drei der vierundzwanzig Öffnungen.
3. **Reichweiten.** Reichweite 1 nennt die Orte des unbekannten Schlüssels (oberste Ebene,
   Zeile, Bausteintisch), den fehlenden `name` und den unbekannten Wert für `zeigt`;
   Reichweite 3 das leere Stück und den zweiten Platzhalter. Ein neuer Absatz sagt, dass
   der `[[profil]]`-Block einen unbekannten Schlüssel übergeht und was das je nach
   Schreibfehler kostet. Der Platzhalter-Absatz nennt die Reichweite des zweiten Sterns.
4. **Leselaufregel.** Die Regel zählt den Erkennungslauf mit: plus einen Lauf, wenn das
   Profil über sein `kennzeichen` erkannt wurde und keine Zeile den erkannten Ordner
   nennt; `pfad` kostet nichts. Mit den Zahlen 3 und 4 als Beispiel; der Kommentar des
   gemeinsamen Speichers sagt, dass seine Erkennung über `pfad` keinen Lauf kostet.
5. **Projektwurzelprofile.** Der Zuschnitt bleibt (Möglichkeit 1 des Datensatzes). Der
   Kommentar über dem fusion-Projektwurzelprofil nennt den Preis: ein `kennzeichen` sieht
   nicht in den Ordner, ein leeres `fusion-workbench` oder eine Datei dieses Namens
   liefert sieben Platzhalter statt der Metadaten; warum das in Kauf genommen ist und wie
   man es abwählt. Der flight-Block verweist auf denselben Preis.

Die fünf Datensätze tragen je einen `Resolved:`-Vermerk mit den Messwerten und den
Marker `_c_`.

## Wie gemessen

Nicht durch Hinsehen. Das Wegwerfpaket der Durchsicht im Sitzungsverzeichnis
(`scratchpad/profilprobe`, Pfadabhängigkeit auf `krk-core`, um `--toml <pfad>` erweitert)
lädt die Datei über `toml::from_str` und `leseprofil::datei::pruefen` und fährt
`zusammenfassen_gezaehlt`:

| Fall | Ergebnis |
|---|---|
| `### Current` vor dem echten Abschnitt | Sitzung: `RICHTIG` |
| `siehe ## Current` im Fließtext | Sitzung: `RICHTIG` |
| CRLF | Sitzung: `--` |
| `fusion-workbench` dieses Projekts | Leseläufe 3, Öffnungen 4 |
| Projektwurzel `krk` | Leseläufe 4, Öffnungen 4 |
| `fusion-workbench/shared` | Leseläufe 10, Öffnungen 0 |
| `example`, `example/flight-workbench` | Leseläufe 6 und 5 |
| leeres `fusion-workbench`, Datei `fusion-workbench`, leeres `flight-workbench` | je sieben `--` |
| `kennzeichnen` | 11 Profile, Meldung „weder ein Pfadmuster noch eine Kennzeichendatei" |
| `nane`, `zeigt = "beides"` | Datei fällt |
| `zeilen`, Fremdschlüssel im Block | 12 Profile, keine Meldung |
| `circles/`, `*/*/x` | „leeres Stueck", „mehr als einen Platzhalter" |

`cargo test -p krk-core --lib leseprofile` Exit 0 (schließt
`die_eingebettete_fassung_besteht_ihre_eigene_pruefung` ein).

## Was offen bleibt

- Der erste `make check` (22:33) war rot an `ein_elfter_unterspeicher_kostet_einen_elften_leselauf`
  in `crates/krk-core/tests/leseprofil.rs`, einer neuen, nicht committeten Probe des parallel
  arbeitenden Coders. Ihr Szenario liefert im Wegwerfpaket auf der HEAD-Fassung und auf der
  geänderten Fassung der Datei dieselbe Ausgabe („Elfter" als erste Zeile); die Ursache lag
  nicht in `default-readers.toml`. Der zweite Lauf (22:41) ist grün: `make check` Exit 0.
- Der gleichlautende Satz „kostet eine Öffnung, die sie sich mit zwei anderen Zeilen
  teilt, also nichts" steht weiter in
  `shared/issues/260825-2044_o_die-zeile-projekt-…` unter „Möglichkeiten" Punkt 1. Der
  Datensatz ist offen und nicht Teil dieser Aufgabe; wer ihn beantwortet, streicht den
  Halbsatz.

# Coder: der Leerbefund-Zweig belegt den einen Sicherungsplatz nicht mehr

**Date:** 2026-08-21 11:00
**Status:** Complete
**Trigger:** Issue file — `shared/issues/260821-1023_*_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`, Befund „Hoch" der Durchsicht `shared/reviews/260821-1023-coderev-eine-bookmarks-toml-ohne-bestand-ist-kein-erster-start.md`
**Baumstand bei Beginn:** `a8da5a5`

## Auftrag

Der Zweig, den `073448e` in `Zugang::laden` eingezogen hat, fängt eine `bookmarks.toml` ohne
einen einzigen obersten Schlüssel und ruft für sie `beiseite_legen`. Eine solche Datei kann
`eintraege` definitionsgemäß nicht tragen; die Sicherung ist wertlos und sperrt den einen
Sicherungsplatz gegen die spätere, die den Bestand enthielte. Die Durchsicht schlägt
`Beiseite::Nicht` vor. Der Vorschlag war zu prüfen, nicht zu übernehmen.

## Prüfung des Vorschlags

Der Vorschlag ist nicht aus dem Text übernommen, sondern zuerst am gebauten Ergebnis
nachgestellt worden: die neue Probe für die gemessene Reihenfolge ist **vor** der Codeänderung
in den Baum gestellt und gefahren worden. Sie scheitert dort an genau der Zeile, die der
Datensatz als Schritt 1 beschreibt:

```
nach_einem_leerbefund_bleibt_der_sicherungsplatz_fuer_den_echten_bestand_frei ... FAILED
  panicked at crates/krk-core/tests/ablage.rs:2916:5:
  der Leerbefund hat den einen Sicherungsplatz belegt
```

Daneben fielen die zwei Proben des Ausgangsdefekts, die den Leerbefund-Zweig treffen, mit
`left: Gesichert(".../bookmarks.toml.beschaedigt")  right: Nicht`. Die dritte,
`ein_fremder_oberster_schluessel_in_bookmarks_toml_gilt_als_beschaedigt`, blieb grün — sie geht
über den Parse-Zweig darunter und ist von der Änderung nicht berührt. Damit war die
Fallunterscheidung vor der Änderung gemessen und nicht abgeleitet.

## Änderung

**`crates/krk-core/src/ablage/mod.rs`, `Zugang::laden`.** Der Leerbefund-Zweig gibt
`Beiseite::Nicht` zurück, statt `beiseite_legen` zu rufen. Eine Zeile weniger, kein zweiter
Mechanismus. Die Erkennung bleibt unangetastet: `Ersetzung`, `Grund::Beschaedigt`,
Auslieferungszustand — alles wie mit `073448e` gebaut.

**Die Meldung ist mitgezogen, ohne dass `Display` angefasst wurde.** Mit `Beiseite::Nicht`
greift ein anderer Zweig von `melden`. Am gebauten Ergebnis gemessen, über `Ablage::oeffnen`,
`durchgang` und `laden`, für die 0-Byte-Datei und für die Datei aus lauter Kommentaren, für
beide gleich:

```
beiseite=Nicht
meldung=.../bookmarks.toml ist beschaedigt und wird durch den Auslieferungszustand
        ersetzt: die Datei traegt keinen einzigen obersten Schluessel, und KRK
        schreibt sie nie so
platz_belegt=Ok(false)
```

Kein Satz über eine Sicherung, die es nicht gibt. Die Zusage im Doc-Kommentar an `Beiseite`
hält damit wieder.

**Vier Prosastellen**, die die Änderung falsch gemacht hätte, sind mitgezogen: die erste der
vier Regeln im Modulkopf, der Absatz „Zwei Stellen beantworten die weitere Frage", der
Doc-Kommentar an `Beiseite::Nicht` (zwei Fälle, jetzt drei, samt der Begründung, warum der
dritte nicht doch sichert) und der Doc-Kommentar an `Zugang::laden`.

## Proben

**Neu:** `nach_einem_leerbefund_bleibt_der_sicherungsplatz_fuer_den_echten_bestand_frei`
(`crates/krk-core/tests/ablage.rs`) — die vier Schritte des Datensatzes als vier Abschnitte.
Die eine Probe im Baum, die zweimal lädt; genau der blinde Fleck, den die Durchsicht an den
sechs Proben des Ausgangsdefekts benannt hat.

**Angepasst:** der gemeinsame Rumpf `beschaedigte_lesezeichen` nimmt die erwartete
`Sicherungslage` als Argument — `Wortlaut` oder `Frei`, zwei Werte und kein Wahrheitswert. Die
drei Proben des Ausgangsdefekts bleiben stehen; für den `Frei`-Fall prüft der Rumpf zusätzlich,
dass die Meldung den Beiseitepfad nicht nennt.

## Bindungen, gegengeprüft

- `nur_benannte_dateien_erreichen_das_atomare_schreiben` (`crates/krk-core/tests/baum.rs`)
  zählt unverändert fünf Dateien. `beiseite_legen` bleibt stehen, es hat einen Rufer weniger.
- `Datei::leerbefund` ist nicht angefasst.
  `eine_leere_datei_meldet_bei_den_drei_uebrigen_toml_dateien_nichts` läuft grün: für die drei
  übrigen TOML-Dateien und die zwei Zettel läuft nichts anders.
- Kein Kommando über den ganzen Baum. Gegen HEAD gemessen mit `git show HEAD:<pfad>`.
- Keine der sieben Prosastellen des Befunds „Niedrig" ist im Wortlaut berührt. Ihre
  Zeilennummern haben sich verschoben (`mod.rs:142`→`:150`, `:425`→`:460`, `:427`→`:462`,
  `:467`→`:502`, `:508`→`:543`); im Defektdatensatz steht es.

## Abnahme

`make check` — Exit 0, `alle vier gruen`, 21 grüne Testläufe. `crates/krk-core/tests/ablage.rs`
zählt jetzt 71 statt 70 Proben, 5 ignoriert.

**Ein Fehlschlag unterwegs, und er gehört nicht zu dieser Änderung.** Ein Lauf von `make check`
brach mit Exit 2 ab an
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` (`crates/krk-core/tests/text.rs:870`,
„die Durchlaeufe sind nach 15 Sekunden nicht fertig geworden"). Die Probe hängt an einer
Zeitschranke über einer benannten Röhre, liegt in `text.rs` und berührt die Ablage nicht; der
Lauf davor und die zwei danach waren grün. Es ist ein zeitabhängiger Ausfall unter Last und
kein Befund dieser Änderung — festgehalten, weil er sonst beim nächsten Auftreten neu untersucht
würde.

## Dateien

- `crates/krk-core/src/ablage/mod.rs`
- `crates/krk-core/tests/ablage.rs`
- `fusion-workbench/shared/issues/260821-1023_c_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`

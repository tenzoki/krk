# Ontocoder — zwei Leseprofile für `archive/` und `shared/` — 260831-1353

**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Auftrag:** Zwei neue Leseprofile in `resources/default-readers.toml` anlegen, eines für
`fusion-workbench/archive/` (Zahl der Einträge in beiden Lesarten, Datum der letzten
Archivierung) und eines für `fusion-workbench/shared/` (je Speicher eine Zählzeile).
**Herkunft:** außerhalb der laufenden Runde 23, deshalb liegen die Artefakte im
gemeinsamen Speicher und nicht im aktiven Circle.
**Status:** Complete — ohne Änderung an der Zieldatei, mit einem Entscheidungsdatensatz.

## Ergebnis in einem Satz

`resources/default-readers.toml` ist unverändert geblieben, weil beide beauftragten
Profile schon darin stehen und die eine Erweiterung, die sie nicht tragen, mit dem
festgelegten Bausteinsatz nicht zu haben ist.

## Was am Bestand steht

Die Datei führt zwölf Profile, und zwei davon sind die beauftragten:

| Profil | Erkennung | Zeilen |
|---|---|---|
| `fusion-Werkbank: der Ablagespeicher` | `pfad = 'fusion-workbench/archive$'` | „Läufe" (`zaehlung = { }`), „Zuletzt abgelegt" (`juengste = { anzahl = 1, zeigt = "datum" }`) |
| `fusion-Werkbank: der gemeinsame Speicher` | `pfad = 'fusion-workbench/shared$'` | je Speicher eine Zähl- und eine Datumszeile |

Die Speicher unter `fusion-workbench/shared/` sind am Bestand des 260831 nachgezählt:
`analyses`, `backlog`, `consult`, `decisions`, `history`, `investigations`, `issues`,
`memos`, `planning`, `reviews`. Das Profil führt genau diese und keinen daneben. Sein
Kommentar rechnet seine Kosten vor, zehn Leseläufe für zwanzig Zeilen, und weist auf die
Grenze hin, an die ein elfter Speicher heranrückt.

## Die zwei Mehrdeutigkeiten des Auftrags

**„Anzahl der Archiv-Einträge".** Die Lesart „Archivläufe" steht als Zeile „Läufe" da. Die
Lesart „archivierte Dateien" ist nicht zählbar. Die Ablage liegt drei Ebenen tief,
`archive/<lauf>/shared/<speicher>/<datensatz>.md`, die Zählung läuft flach über eine Ebene,
und eine Ortsangabe nimmt höchstens einen Platzhalter an (`Ortsmangel::MehrerePlatzhalter`).
Gemessen am 260831: fünf Läufe, 167 Dateien darunter, keine unmittelbar in `archive/`. Eine
Zeile `zaehlung = { ordner = "*/shared" }` läge dazwischen und zählte fünfzehn abgelegte
Speicherordner, also weder das eine noch das andere.

**„Datum der letzten Archivierung".** Die Wahl `zeigt = "datum"` steht schon da und ist die
einzig mögliche. `bausteine.rs:699` filtert die Titelform auf `eintrag.typ == Typ::Datei`,
und `archive/` trägt unmittelbar nur Ordner; `zeigt = "titel"` lieferte dort den
Platzhalter `--` und nicht den Ordnernamen mit seinem Zeitstempel. Der Kommentar am Profil
schreibt genau das bereits aus.

## Warum kein Edit

Ein dreizehntes `[[profil]]` wäre mit einer Änderung an dieser Datei allein nicht zu haben.
Drei Proben halten die Zahl zwölf: `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
und `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` in
`crates/krk-core/src/ablage/leseprofile.rs`, dazu `ausgelieferte()` in
`crates/krk-core/tests/leseprofil.rs`. Die zweite verlangt außerdem, dass keine
mitgelieferte Zeile `typ =` oder `versteckt =` trägt. Prüfcode gehört dem `coder`.

## Artefakte

`shared/decisions/260831-1353_o_bekommt-das-ablageprofil-eine-zweite-umfangszeile-obwohl-die-dateizahl-nicht-zaehlbar-ist.md`
legt die offene Frage mit drei Möglichkeiten vor und empfiehlt, es bei der einen Zeile
„Läufe" zu belassen und den Grund im Kommentar auszuschreiben.

## Prüfung

```
make check                                        exit 0
cargo test -p krk-core --lib ablage::leseprofile   exit 0 (3 passed)
```

Gefahren am unveränderten Baum, HEAD `63aa690`. Der zweite Lauf ist der Beleg, dass die
Auslieferungsfassung durch dieselbe `datei::pruefen` läuft, die auch die Nutzerdatei
durchläuft, und dabei nicht beanstandet wird.

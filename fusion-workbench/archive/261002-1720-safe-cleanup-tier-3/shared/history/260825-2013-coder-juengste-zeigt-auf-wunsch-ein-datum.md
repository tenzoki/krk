# `juengste` zeigt auf Wunsch ein Änderungsdatum statt eines Titels

**Datum:** 2026-08-25
**Agent:** coder
**Status:** Complete
**Auftrag:** Schritt 6, Strang 2 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` — „`juengste` zeigt
auf Wunsch ein Änderungsdatum statt eines Titels"
**Grundlage:** `shared/decisions/260825-1725_a_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile.md`,
Möglichkeit 1 mit ihren drei Festlegungen, freigegeben am 260825-1740

## Was gebaut ist

**`crates/krk-core/src/leseprofil/mod.rs`.** Die neue Aufzählung `Anzeige` trägt zwei Werte,
`Titel` und `Datum`, vollständig und ohne Auffangzweig; `Baustein::Juengste` bekommt das Feld
`zeigt: Anzeige`. Der Bausteinsatz bleibt bei vier — Festlegung A7 der Runde 16 ist unberührt,
und `Wert` trägt weiter sechs Werte.

Die Dokumentation von `Anzeige` stellt die drei Unterschiede als Tabelle nebeneinander
(Öffnungen, gesehene Einträge, gelieferter `Wert`) und führt alle drei auf den **einen**
Unterschied zurück, dass die eine Form Dateien liest und die andere nicht. Der dritte Wert
„Titel und Datum" ist dort ausdrücklich als nicht gebaut vermerkt, wie der Entscheid es sagt.

`Wert::Text` heißt jetzt „ein Text" und nicht mehr „ein aus einer Datei gezogenes Feld": die
Variante beschreibt die **Gestalt** eines Wertes und nicht seine Herkunft, und sie hat seit
dieser Runde zwei Quellen. Die Doku nennt beide und sagt dazu, dass über den Zeilenumbruch
`Zusammenfassung::als_text` entscheidet — am Text und nicht an der Variante.

**`crates/krk-core/src/leseprofil/datei.rs`.** `Juengstedatei` bekommt `zeigt:
Option<Anzeigedatei>`; die Aufzählung `Anzeigedatei` trägt `#[serde(rename_all = "lowercase")]`
und damit die zwei Namen `titel` und `datum`. Ein dritter Wert lässt `serde` die **ganze
Datei** abweisen (C1.6, die weiteste der drei Reichweiten) — dieselbe Reichweite wie ein
verschriebener Bausteintisch. Die Doku sagt daneben, warum hier nicht gekappt wird wie bei
`anzahl`: eine überhöhte Zahl ist eine Angabe, die mehr verlangt, als die Zusammenfassung
hergibt; `zeigt = "titelchen"` ist ein Vertipper, und ihn still auf „titel" zu bringen hieße,
dem Nutzer etwas anderes zu zeigen, als er geschrieben hat.

**Ein eigener Typ neben `Anzeige` und nicht dieser selbst**, obwohl beide dieselben zwei Werte
tragen: `serde` wohnt in `datei.rs` und nicht im Elternmodul, so wie dort die Zeichenkette
steht und im Elternmodul das übersetzte Muster. Der Preis ist die Zuordnung `anzeige`, und sie
ist vollständig ohne Auffangzweig — ein dritter Wert hält den Bau dort an. Dieselbe Funktion
trägt zugleich den Vorgabewert: fehlt der Schlüssel, sind Titel gemeint, denn eine
`readers.toml` von gestern soll zeigen, was sie gestern gezeigt hat.

**`crates/krk-core/src/leseprofil/bausteine.rs`.** `Lauf::juengste` nimmt `zeigt` entgegen und
verzweigt an zwei Stellen: beim Filtern der Kandidaten (`Titel` nimmt allein Einträge vom Typ
Datei, `Datum` jeden Typs) und beim Bauen des Wertes. Die Reihenfolge, die Kappung auf N und
die Abbruchregel stehen **vor** der Verzweigung und sind für beide Formen dieselben.

Zwei neue freie Funktionen im neuen Abschnitt „Aus Zeitpunkten ein Kalendertext":

- `daten` legt die Änderungsdaten untereinander in **einen** `Wert::Text`. Kein siebter `Wert`
  und keine neue Regel in `als_text`: die vorhandene entscheidet am Zeilenumbruch, also steht
  ein einzelnes Datum von selbst neben seiner Beschriftung und mehrere von selbst darunter.
- `kalendertext` formt `JJJJ-MM-TT HH:MM` über `krk_core::verzeichnis::sys::ortszeit` aus
  `c0050bf`. Die Sekunde fällt weg; sie beantwortet keine Frage, die jemand an eine
  Zusammenfassung stellt.

**Ein Zeitpunkt, den der Kalender nicht trägt, kostet die ganze Zeile und nicht nur seinen
eigenen Eintrag.** Das ist derselbe Satz, den die Titelform mit ihren Öffnungen hält: eine
Liste, in der einer von drei Einträgen mit einem Ersatztext dasteht, läse sich unter der
Beschriftung „die jüngsten drei" falsch. Die Lage ist die dritte des Platzhalters aus C3.12.

**Die Modulköpfe.** `bausteine.rs`: der Abschnitt „Was ein Name entscheidet und was eine
Datei" sagt jetzt, dass die Naht **am Lesen** entlangläuft und nicht am Baustein, und dass
beides seit dieser Runde auseinanderfällt; der Abschnitt zur Teillesung sagt, dass die
Abbruchregel für beide Formen gilt; der Satz „in einem Zug oder gar nicht" nennt seine
Entsprechung in der Datumsform. `mod.rs`: `HOECHSTENS_OEFFNUNGEN` sagt, dass die Datumsform
keine Öffnung kostet, und `Baustein` sagt, warum ein Speicher aus lauter Ordnern nur so
antworten kann. `datei.rs`: die erste der drei Reichweiten nennt den neuen Fall.

## Was gemessen ist

Sechs Kriterien des Plans hängen an fünf neuen Proben in `crates/krk-core/tests/leseprofil.rs`
und einer erweiterten alten:

| Probe | was sie abnimmt |
|---|---|
| `zeigt_datum_liefert_ein_kalenderdatum_und_oeffnet_keine_datei` | **null** Öffnungen über `zusammenfassen_gezaehlt`, daneben die **eine** derselben Zeile als Titelform; `zeigt = "titel"` liefert Wert **und** Haushalt wie eine Zeile ohne den Schlüssel |
| `die_datumsform_traegt_vier_zahlen_an_festen_stellen` | sechzehn Zeichen, vier Trenner an festen Stellen, sonst Ziffern |
| `ein_ordner_aus_lauter_ordnern_liefert_ein_datum_und_keinen_titel` | ein Archivordner aus zwei Unterordnern: die Datumsform antwortet, die Titelform setzt weiter ihren Platzhalter |
| `drei_daten_stehen_untereinander_und_eines_daneben` | `anzahl = 3` eingerückt unter der Beschriftung, `anzahl = 1` daneben — beides aus derselben Regel |
| `ein_dritter_wert_fuer_zeigt_kostet_die_ganze_datei` | drei ungültige Werte, je mit einer Meldung, die `zeigt`, `titel` und `datum` nennt; die drei gültigen Schreibweisen kommen durch |
| `eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet` (erweitert) | eine fünfte Zeile: auch als Datum sind die jüngsten zehn einer Teilliste nicht die jüngsten zehn |

Dazu die Rundreise `eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte`: sie
zerlegt `Baustein::Juengste` und hält jetzt fest, dass ohne den Schlüssel `Anzeige::Titel`
dasteht. Die vorhandenen Proben zu `juengste` sind Zahl für Zahl unverändert grün, insbesondere
`die_zahl_der_oeffnungen_folgt_der_bausteinsorte` mit ihren acht Fällen.

**Die erwartete Zeichenkette steht nicht im Quelltext**, sondern kommt aus `ortszeit`: eine
feste Zahl darin wäre die Zeitzone des Geräts, auf dem die Probe geschrieben wurde. Dieselbe
Wahl trifft `tests/operation.rs::das_msdos_feld_traegt_die_ortszeit_des_quelldatums`. Die
**Form** dagegen steht ausgeschrieben da, denn sie ist die Zusage.

**Gegenprobe zur Null.** In den Datumszweig eine Buchung von einer Öffnung eingesetzt, die es
nicht gibt; `zeigt_datum_liefert_ein_kalenderdatum_und_oeffnet_keine_datei` wird rot mit
`left: 1, right: 0`. Danach aus der Sicherungskopie wiederhergestellt (nicht über
`git checkout`) und wieder grün. Die Zahl ist damit gemessen und nicht behauptet: die Probe
zählt am Haushalt des Laufs, den die Vorschau fährt, und die Eins der Titelform daneben belegt,
dass an dieser Stelle überhaupt etwas zu sparen war.

## Was aufgefallen ist

**Kein Datensatz gefiled.** Drei Beobachtungen, die alle drei keine offene Frage und kein
Defekt sind:

- **`juengste` weist einen Platzhalter auch in der Datumsform ab**, obwohl diese Form keine
  Datei liest und einen zusammengelegten Lesestand also tragen könnte. Der Plan hebt die
  Abweisung nicht auf, und sie steht als Zurückstellung mit ihrem Grund in der Dokumentation
  von `Ortsangabe`: die Grenze bleibt an einer Stelle ablesbar („`juengste` nimmt keinen
  Platzhalter an"), solange niemand die Auskunft braucht, die erst der Platzhalter hergibt.
  Für den Wunsch des Nutzers ist sie ohne Belang — eine Sammlung über `*` legte die Einträge
  aller Unterordner zusammen und lieferte **ein** Datum und nicht eines je Unterordner. „Je
  Unterordner" heißt eine Zeile je Unterordner, und die schreibt Schritt 8.
- **Ein einzelner Titel steht unter seiner Beschriftung, ein einzelnes Datum daneben.**
  `als_text` entscheidet an `Wert::Titel(_)` unabhängig von der Zahl der Einträge, an
  `Wert::Text` am Zeilenumbruch. Das ist genau, was der Entscheid verlangt, und keine
  Unstimmigkeit, die aufzulösen wäre; es steht hier, damit ein späterer Leser es nicht für
  eine hält.
- **Der offene Befund aus Schritt 5** (`shared/issues/260825-1953_o_ein-platzhalterlauf-…`)
  ist von dieser Arbeit unberührt und wird von ihr nicht verschärft: `juengste` erreicht einen
  Platzhalterlauf nicht, und die Datumsform senkt die Zahl der Öffnungen, statt sie zu heben.

## Was nicht angefasst ist

`resources/default-readers.toml` — die Zeilen mit `zeigt = "datum"` schreibt Schritt 8. Nichts
unter `crates/krk-ui/`, `crates/krk-core/src/operation/` oder `verzeichnis/`; `Cargo.toml`
unverändert, keine neue fremde Kiste. `HOECHSTENS_LESELAEUFE`, `HOECHSTENS_OEFFNUNGEN` und
`HOECHSTENS_EINTRAEGE` stehen auf ihren Zahlen. Kein Commit: die Änderungen stehen im
Arbeitsbaum.

## Abnahme

`make check` — exit 0.

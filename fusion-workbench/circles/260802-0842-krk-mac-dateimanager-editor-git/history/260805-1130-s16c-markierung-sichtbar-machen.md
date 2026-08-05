# S16c: die Markierung sichtbar machen

**Status:** Complete
**Ausführender:** coder
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 16c
**Datensatz:** `decisions/260805-0000_a_zweites-kennzeichen-der-markierung-und-ihr-platz-in-der-statuszeile.md`

## Was gebaut ist

Zwei Änderungen, die dasselbe sichtbar machen. Ein markierter Eintrag steht seit diesem
Schritt in allen vier Spalten **fett** und bleibt orange; die Statuszeile bekommt als
fünften und untersten Rang den Markierungsstand des sichtbaren Tabs.

`Ordnermodell::markierungszahl` ist zu `Ordnermodell::markierungsstand` gewachsen und
liefert Zahl, Ordnerzahl und Größensumme in einem Durchlauf als `Markierungsstand`. Die
Größensumme zählt allein Dateien: `Eintrag.groesse` ist für einen Ordner ohne Aussage, und
sie zu ermitteln hieße ihn zu durchlaufen, was `### Frage 6` ausschließt. Dieselbe
Trennung zieht die Größenspalte, die bei einem Ordner `--` zeigt.

Der fünfte Rang ist die einzige Quelle der Zeile **ohne Feld**. Er wird bei jedem Aufruf
von `meldung_anzeigen` aus dem Ordnermodell des sichtbaren Tabs gerechnet. `zeile` bleibt
eine reine Funktion und nimmt den fertigen Text als fünften Parameter. Er trägt
`Art::Vorgang`: eine Markierungszahl ist kein Fehler und wird nicht rot.

## Vier Abnahmekommandos, alle mit 0

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets`, `cargo fmt --all --check`. Der Testlauf zählt 13 Testprogramme mit
zusammen 361 Prüfungen, davon 0 gescheitert und 1 übersprungen; vor dem Schritt waren es
348.

## Die Abnahmepunkte einzeln

| Punkt | Beleg |
|---|---|
| `cargo test -p krk-core --test navigation` mit 0, deckt `markierungsstand` ab | 15 Prüfungen, davon drei neue: `alle_markieren_zaehlt_ordner_gesondert_und_summiert_allein_die_dateien` (1.000 Einträge, 100 Ordner, Summe gegen die gerechnete Byte-Zahl), `ein_markierter_ordner_erhoeht_die_groessensumme_nicht`, `ohne_markierung_sind_alle_drei_werte_null` |
| `cargo test -p krk-ui` mit 0, deckt `zeile` ab | 102 Prüfungen, davon zwei neue: `der_markierungsstand_steht_hinter_der_tabmeldung`, `der_markierungsstand_gilt_nicht_als_fehler`; dazu vier neue in `kommandos::auswahl` zur Wortform |
| Drei markierte Einträge stehen fett und orange, die übrigen weder noch | Bildschirmfoto am laufenden Bündel, 260805-1128: `Ordner-1` bis `Ordner-3` fett und orange, `alpha.txt` bis `gamma.txt` in gewöhnlicher Schrift |
| Die Statuszeile nennt Zahl, Ordnerzahl und Größe | Am laufenden Bündel: `7 markiert, davon 3 Ordner, 10 KB` nach `cmd+a` im Prüfordner mit 3 Ordnern und 4 Dateien zu 1.000, 2.000, 3.000 und 4.000 Bytes |
| Ein Ordner ohne Leserecht zeigt weiter seine Meldung | Am laufenden Bündel: `/tmp/krk-s16c-gesperrt laesst sich nicht lesen: Permission denied (os error 13)`, rot. **Die Verbindung "unlesbar und zugleich markiert" ist am laufenden Bündel nicht baubar**, weil ein Ordner ohne Leserecht keine Einträge liefert, die sich markieren ließen. Den Vorrang belegt stattdessen die Prüfung `der_markierungsstand_steht_hinter_der_tabmeldung` |
| Ein Bildschirmfoto in Graustufen zeigt den Unterschied | Erzeugt mit `screencapture` und nach `Generic Gray Gamma 2.2` umgewandelt (`sips -m`), geprüft: `samplesPerPixel: 2`, `space: Gray`. Im Graustufenbild sind die drei markierten Zeilen allein an der Schriftstärke von den vier unmarkierten zu unterscheiden. Die Bildschirmaufnahme war freigegeben; ein ungeprüfter Punkt bleibt nicht |

## Was der Plan nicht nannte

**`crates/krk-ui/src/kommandos/auswahl.rs` und `crates/krk-ui/src/kommandos/operationen.rs`.**
Die Wortform des fünften Rangs ("12 markiert, davon 3 Ordner, 4,2 MB") steht in
`auswahl.rs`, weil sie zu C2 gehört und nicht zu C4, und weil sie dort ohne AppKit prüfbar
ist. Ihre beiden Bausteine leiht sie sich aus `operationen.rs`: `zahl` für die
Tausenderpunkte und `ordner_text` für "3 Ordner" beziehungsweise "ein Ordner". Beide sind
dafür von privat auf `pub(crate)` gegangen und sonst unverändert. Ein zweites Mal
geschrieben wären sie zwei Schreibweisen für dieselbe Zahl.

**`crates/krk-core/src/verzeichnis/mod.rs`.** Die neue Struktur `Markierungsstand` wird
dort ausgeführt, wo `Ordnermodell` schon steht.

## Zwei Stellen, die der Plan so nicht vorhergesehen hat

**Die Markierungsbefehle mussten das Zeichnen anstoßen.** Der Plan leitet richtig her,
dass ein Feld vier Schreiber hätte und deshalb keines entsteht. Gerechnet wird der Stand
aber erst beim nächsten Aufruf von `meldung_anzeigen`, und den rief bis dahin kein
Markierungsbefehl. Beim ersten Lauf am Bündel blieb die Zeile deshalb leer. `markieren_und_weiter`
und `markierung_aendern` rufen ihn jetzt, so wie sie die Tabelle neu laden lassen. Das ist
etwas anderes als ein Feld mit vier Schreibern: verpasst einer den Aufruf, steht ein alter
Text in der Zeile bis zum nächsten Zeichenanlass, und nirgends ein falscher Zustand.
Umsortieren und Ein- und Ausblenden brauchen den Aufruf nicht, weil sie die Markierung
nicht anfassen und der Stand über alle gelesenen Einträge zählt.

**Der Größenformatierer ist von der Delegierten zur Quelle gezogen.** Der Plan verlangt
für den Markierungsstand denselben `NSByteCountFormatter`, der die Größenspalte
beschriftet. Der stand in `DelegiertenIvars`, und `meldung_anzeigen` läuft an der Quelle.
Er wohnt jetzt in `QuelleIvars`; der Delegierte erreicht ihn über `quelle()`, und die
starke Richtung geht ohnehin von ihm zur Quelle. Ein zweiter Formatierer wäre eine zweite
Schreibweise für dieselbe Zahl gewesen.

## Ein Defekt angelegt

`issues/260805-1130_o_der-groessenformatierer-schreibt-zero-kb-auf-englisch.md` — für null
Bytes schreibt `NSByteCountFormatter` "Zero KB" statt einer deutschen Wendung, sichtbar in
der Größenspalte seit S12 und seit diesem Schritt zusätzlich in einem deutschen Satz. Die
beiden möglichen Auflösungen berühren `resources/Info.plist` oder die Einstellung des
gemeinsamen Formatierers, also beide Male mehr als diesen Schritt.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/verzeichnis/modell.rs` | `Markierungsstand`, `markierungszahl` → `markierungsstand` |
| `crates/krk-core/src/verzeichnis/mod.rs` | Ausfuhr von `Markierungsstand` |
| `crates/krk-core/tests/navigation.rs` | neun Aufrufstellen umgestellt, drei Prüfungen und zwei Prüfordner-Hilfen dazu |
| `crates/krk-ui/src/appkit/statuszeile.rs` | fünfter Parameter von `zeile`, Rangtabelle im Modulkopf, zwei Prüfungen dazu |
| `crates/krk-ui/src/appkit/tabelle.rs` | fette Schrift in `zellenansicht`, `markierungsstand_text`, `groesse_beschriften` an der Quelle, zwei Aufrufe von `meldung_anzeigen` |
| `crates/krk-ui/src/kommandos/auswahl.rs` | `markierungsstand_text` mit vier Prüfungen |
| `crates/krk-ui/src/kommandos/operationen.rs` | `zahl` und `ordner_text` auf `pub(crate)` |

## Prüfdaten

`/tmp/krk-s16c-pruef` (drei Ordner, vier Dateien bekannter Größe), `/tmp/krk-s16c-gesperrt`
(Rechte 000) und die Bildschirmfotos unter `/tmp`. Alle selbst angelegt und am Ende der
Sitzung entfernt.

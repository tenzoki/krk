# Eine abgewiesene Auswahl bricht die Messstrecke ab (D7, Turn 25)

**Agent:** coder
**Status:** Complete
**Quelle:** `issues/260806-1304_o_der-sitzungslauf-blieb-einmal-von-drei-malen-bei-l6-stehen.md`

## Was umgesetzt wurde

Die ungemessene Auswahl-Vorbereitung der Sitzungsstrecke warf den Rückgabewert von `Tabellenquelle::eintrag_waehlen` weg. Ging sie ins Leere, drückte der nächste Schritt `oeffnen` auf einen anderen Eintrag oder auf keinen, die Endbedingung von L6 stand nie, und der Lauf endete zehn Sekunden später mit einer Geduldsmeldung über L6, die den Grund nicht nennen konnte.

Jetzt liefert `Anwendungsdelegierter::messhandlung` ein `Result<(), String>`. Bei `Handlung::Auswaehlen` entscheidet der `Auswahlversuch`, und die drei Fälle werden getrennt behandelt:

| Fall | Behandlung | Warum |
|---|---|---|
| `Gewaehlt` | `Ok` | Die Auswahl steht auf dem Eintrag. |
| `Vorgemerkt` | `Ok` | Es läuft noch ein Lesevorgang; die Auswahl springt mit seinem Abschluss auf den Namen. Der gewöhnliche Weg, kein Fehlschlag. |
| `Unbekannt` | `Err` | Die Liste ist fertig gelesen und kennt den Namen nicht. Der endgültige Fehlschlag. |

Der Grund geht über `Messlauf::vorbereitung_gescheitert` in den Messlauf zurück und kommt am nächsten Auslösetakt (97 ms) als `Anweisung::Abbruch` wieder heraus, über **denselben** Abbruchweg wie jeder andere Abbruch der Strecke. Die Prüfung steht am Kopf von `naechster_schritt`, vor jeder Aufgabe und jedem Schritt: der Abbruch fällt, bevor der Schritt `oeffnen` seine Taste absetzt und bevor eine Messung beginnt.

Die Meldung nennt Namen, Ordner, den Lesestand und die Zeilenzahl:

```
krk: die Vorbereitung sollte <name> in <ordner> auswaehlen, aber der Name steht
dort nicht: die Liste ist fertig gelesen, es laeuft kein Lesevorgang mehr, und
sie traegt <n> Zeilen. Das ist ein Fehler der Strecke und keine langsame
Oberflaeche. Es wird keine Zahl ausgegeben.
```

Dass kein Lesevorgang mehr läuft, ist keine Ablesung, sondern die Bedeutung von `Unbekannt` selbst: `eintrag_waehlen` liefert diesen Fall nur, wenn `liest` falsch ist. Ein zweites Flag daneben wäre eine Konstante mit einer Möglichkeit zum Auseinanderlaufen. Die Zeilenzahl trennt die beiden Fälle, in die ein Fehlschlag zerfällt: null Zeilen heißt, der Ordner kam gar nicht an; eine gefüllte Liste, dass der Name in einem gelesenen Bestand fehlt.

## Geänderte Dateien

| Datei | Zeilen | Was |
|---|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs` | 198 | `Auswahlversuch` mit importiert |
| `crates/krk-ui/src/appkit/anwendung.rs` | 2519–2528 | Aufrufstelle: ein `Err` geht an `vorbereitung_gescheitert` statt in ein `exit` an Ort und Stelle |
| `crates/krk-ui/src/appkit/anwendung.rs` | 2543–2587 | `messhandlung` liefert `Result`; die drei `Auswahlversuch`-Fälle samt Begründung je Fall |
| `crates/krk-ui/src/messmodus.rs` | 726–750 | `auswahl_ohne_eintrag`: die Meldung, prüfbar ohne AppKit |
| `crates/krk-ui/src/messmodus.rs` | 893–899 | Feld `vorbereitungsfehler` |
| `crates/krk-ui/src/messmodus.rs` | 950–961 | `vorbereitung_gescheitert` |
| `crates/krk-ui/src/messmodus.rs` | 969–977 | Prüfung am Kopf von `naechster_schritt` |
| `crates/krk-ui/src/messmodus.rs` | 2127–2185 | Test `eine_abgewiesene_auswahl_bricht_den_lauf_ab` |

## Warum der Umweg über den Messlauf

Ein `eprintln!` samt `exit(4)` unmittelbar in `messhandlung` wäre kürzer gewesen und hätte den Abbruch um eine Zehntelsekunde vorgezogen. Dagegen sprechen zwei Dinge. Die Strecke hat mit `Anweisung::Abbruch` genau einen Abbruchweg, und ein zweiter Ausstieg mitten im AppKit-Teil wäre der Anfang einer zweiten Wahrheit darüber, wie ein Lauf endet. Und die Entscheidung wäre dort nicht prüfbar: `messhandlung` hängt am Anwendungsdelegierten und ist ohne Fenster nicht erreichbar. Gegen zehn Sekunden Geduld ist ein Takt von 97 ms kein Unterschied, den jemand bemerkt.

## Prüfung

Der Test `eine_abgewiesene_auswahl_bricht_den_lauf_ab` spult die Schrittliste bis zur ersten Auswahl-Vorbereitung der L6-Reihe vor, prüft, dass dort tatsächlich `Handlung::Auswaehlen(<Unterordnername>)` steht, meldet den Fehlschlag über `vorbereitung_gescheitert` zurück und belegt, dass der nächste Takt `Anweisung::Abbruch` liefert statt der gemessenen Taste `oeffnen` — und dass die Meldung Namen, Ordner und Lesestand trägt. Ein zweiter Takt bleibt beim Abbruch: ein Fehlschlag geht nicht dadurch weg, dass die Zeit vergeht.

Was der Test **nicht** deckt: welcher der drei `Auswahlversuch`-Fälle als Fehlschlag gilt. Diese Zuordnung steht in `messhandlung` am Anwendungsdelegierten und ist ohne Fenster nicht erreichbar; die Ebene darunter ist geprüft.

`make check` läuft grün: Bau, Tests im ganzen Arbeitsbereich, `clippy -D warnings`, `fmt --check`.

## Der zweite Verdacht: geprüft, ausgeräumt

Der Defekt nennt neben der verworfenen Auswahl ein Rennen zwischen dem Warteschritt auf den Elternordner und der Auswahl. Am Programmtext ist es ausgeschlossen.

Die L6-Reihe fährt je Wiederholung `Handeln(AktivLesen(eltern))`, `Warten(AktivZeigt(eltern))`, `Handeln(Auswaehlen(name))`, `Taste oeffnen`. Die Bedingung `AktivZeigt` verlangt drei Dinge zugleich: der sichtbare Tab zeigt den Ordner, es läuft kein Lesevorgang, und er trägt Zeilen (`messmodus.rs:584`).

`messen_weiter` führt die Handlung **im selben Auslösetakt** aus, in dem `sitzung_weiter` sie ausgibt; erst der nächste Takt wertet den Warteschritt aus. In diesem selben Takt setzt `Tabliste::ordner_setzen` (`tabs.rs:439`) einen frischen `Tabinhalt` mit leerem Modell an die Stelle des alten und `lesen_starten` (`tabs.rs:581`) setzt `lesevorgang = Some(...)`. Wenn die Bedingung das erste Mal geprüft wird, sind damit zwei ihrer drei Teile falsch: `zeilen_aktiv == 0` und `liest_aktiv == true`. Sie kann nicht auf dem Stand von vor dem Lesevorgang durchlaufen.

`liest_aktiv` wird erst falsch, nachdem `einzug_je_tab` (`tabs.rs:610`) die Meldung `Fertig` gesehen und `Ordnermodell::abschliessen` gerufen hat, das die Sichtreihenfolge neu aufbaut (`krk-core/src/verzeichnis/modell.rs:218`). Gelesen **und** sortiert also, bevor die Auswahl kommt. Eine gleichzeitige Auffrischung aus C9 kann den Warteschritt nur verzögern, nicht früher durchlassen: `aktiven_neu_lesen` setzt `liest` auf wahr und lässt die alten Zeilen stehen.

Ein eigener Defekt dazu wurde deshalb nicht angelegt.

## Was offen bleibt

Der Defekt bleibt offen (`_o_`). Welcher der beiden Fälle der Abbruch vom 260806 war, verlangt Wiederholungsläufe der vollen Strecke, und die verlangt KRK im Vordergrund aus einem Terminalfenster; sie wurde in dieser Sitzung nicht gestartet. Der nächste vollständige Lauf beantwortet die Frage von selbst: bricht er mit der Meldung oben ab, war es die verworfene Auswahl, und Ordner und Zeilenzahl sagen, woran sie scheiterte. Läuft er wieder in die Geduld über L6, war es nicht die Auswahl.

## Nicht zur Aufgabe gehörig, aber aufgefallen

- Drei weitere Aufrufer von `eintrag_waehlen` werfen den Rückgabewert ebenfalls weg: `anlegen_ausfuehren` (`anwendung.rs:1885`), `umbenennen_ausfuehren` (`anwendung.rs:1908`) und die Auswahlnachführung des Stapel-Umbenennens (`anwendung.rs:2316`). Alle drei treffen eine Liste, die gerade neu gelesen wird, also den Fall `Vorgemerkt`; ein `Unbekannt` wäre dort trotzdem eine Auskunft an den Nutzer wert, wie es der Kopfkommentar von `eintrag_waehlen` selbst schreibt. Kein Defekt der Messstrecke, deshalb hier nur notiert.
- `messen_weiter` trägt zwei Stellen, die wortgleich `eprintln!("krk: {…}. Es wird keine Zahl ausgegeben.")` und `exit(4)` schreiben (Funktionstaste und `Anweisung::Abbruch`). Eine dritte kam mit dieser Änderung ausdrücklich nicht dazu. Zusammenzuziehen wären die beiden trotzdem.

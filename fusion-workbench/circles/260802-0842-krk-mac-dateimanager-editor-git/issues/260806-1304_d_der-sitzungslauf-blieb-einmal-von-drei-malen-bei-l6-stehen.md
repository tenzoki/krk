Der Sitzungslauf blieb einmal von drei Malen bei L6 stehen

---

Beim Nachprüfen des Defekts `260806-1235_o_der-sitzungslauf-der-abnahmestrecke-bricht-bei-l5-tab-ab-und-gibt-keine-zahl-mehr-aus.md`
lief die Sitzungsstrecke am 260806 dreimal aus einem Terminalfenster im
Vordergrund, also unter der Bedingung, unter der sie überhaupt misst. Zwei der
drei Läufe kamen vollständig durch. Der **erste** brach ab:

```
krk: die Messung l6 ist nach 10 s nicht am Ziel; seit dem Beginn sind 581
Bildgrenzen eingegangen. Es wird keine Zahl ausgegeben.
```

L1, L7, L5-Tab und L5-Fenster hatten in diesem Lauf ihre zwanzig Werte; erst
L6 blieb stehen. Der zweite, wortgleiche Lauf lieferte für L6 zwanzig Werte,
und der volle `make alle RUNDEN=1` danach nahm L6 mit einem 95. Perzentil von
47,1 ms ab (Zusage 100 ms).

Der Bildtakt lief während der zehn Sekunden weiter (581 Bildgrenzen sind rund
58 je Sekunde); es stand also nicht die Oberfläche.

**Was L6 misst.** Je Wiederholung: den Elternordner in den sichtbaren Tab
lesen, auf ihn warten, den Unterordner am Namen auswählen (`Handlung::Auswaehlen`,
ungemessen), dann `oeffnen` gemessen, bis `ordner_aktiv == unterordner`, der
Lesevorgang beendet ist und Zeilen dastehen
(`crates/krk-ui/src/messmodus.rs`, `sitzungsschritte` und
`sitzungsmessung_fertig`).

`inference:`, nicht gemessen: die wahrscheinlichste Stelle ist die ungemessene
Auswahl. `Tabellenquelle::eintrag_waehlen` liefert `Auswahlversuch::Unbekannt`,
wenn der Name im Modell nicht steht und **kein** Lesevorgang mehr läuft; der
Rückgabewert wird an der Aufrufstelle in `Anwendungsdelegierter::messhandlung`
verworfen. Trifft dieser Fall zu, drückt der nächste Schritt `oeffnen` auf
einen anderen Eintrag oder auf keinen, die Endbedingung steht nie, und die
Geduld läuft ab. Ein zweiter Kandidat ist ein Rennen zwischen dem Warteschritt
auf den Elternordner und der Auswahl.

**Was zu tun bleibt:** feststellen, welcher der beiden Fälle es war, und den
Fehlschlag sichtbar machen statt ihn zu verwerfen — ein abgewiesener
`Auswahlversuch` in einer Vorbereitung der Messstrecke ist ein Fehler der
Strecke und gehört abgebrochen, nicht in eine Zehn-Sekunden-Geduld.

**Aufgefallen bei:** der Gegenmessung zum L5-Tab-Defekt am 260806-1250.
Adressat: `coder`.

---

## Gebaut am 260807: der Fehlschlag bricht ab, statt verworfen zu werden

**Was jetzt steht.** `Anwendungsdelegierter::messhandlung` liefert ein
`Result<(), String>` statt nichts. Bei `Handlung::Auswaehlen` entscheidet der
`Auswahlversuch` darüber: `Gewaehlt` und `Vorgemerkt` sind der gewöhnliche Weg
und gehen als `Ok` durch — `Vorgemerkt` heißt, es läuft noch ein Lesevorgang
und die Auswahl springt mit seinem Abschluss auf den Namen, und das ist kein
Fehler. Allein `Unbekannt`, der endgültige Fehlschlag in einer fertig
gelesenen Liste, wird zu `Err`. Der Grund geht über
`Messlauf::vorbereitung_gescheitert` in den Messlauf zurück und kommt am
nächsten Auslösetakt als `Anweisung::Abbruch` wieder heraus, also über
denselben Abbruchweg wie jeder andere Abbruch der Strecke; ein zweiter
Ausstieg in `appkit` entsteht nicht.

**Was der nächste Lauf sieht.** Statt einer Geduldsmeldung über L6 nach zehn
Sekunden steht rund eine Zehntelsekunde nach dem Fehlschlag:

```
krk: die Vorbereitung sollte <name> in <ordner> auswaehlen, aber der Name
steht dort nicht: die Liste ist fertig gelesen, es laeuft kein Lesevorgang
mehr, und sie traegt <n> Zeilen. Das ist ein Fehler der Strecke und keine
langsame Oberflaeche. Es wird keine Zahl ausgegeben.
```

Der Abbruch fällt, **bevor** der Schritt `oeffnen` seine Taste absetzt: die
Prüfung steht am Kopf von `naechster_schritt`, vor jeder Aufgabe und jedem
Schritt. Die Zeilenzahl trennt die beiden Fälle, in die ein Fehlschlag
zerfällt: null Zeilen heißt, der Elternordner kam gar nicht an, eine gefüllte
Liste, dass der Name in einem gelesenen Bestand fehlt.

**Der zweite Verdacht ist am Programmtext ausgeräumt.** Geprüft wurde die
Kette `Handlung::AktivLesen` → `Dateifensterquelle::ordner_lesen`
(`crates/krk-ui/src/appkit/tabelle.rs:604`) → `Tabliste::ordner_setzen`
(`crates/krk-ui/src/tabs.rs:439`) → `lesen_starten`
(`crates/krk-ui/src/tabs.rs:581`). Der Warteschritt kann nicht auf dem Stand
von vor dem Lesevorgang durchlaufen: `ordner_setzen` setzt einen frischen
`Tabinhalt` mit leerem Modell an die Stelle des alten und `lesen_starten` setzt
`lesevorgang = Some(...)`, beides **synchron im selben Auslösetakt**, in dem
`messen_weiter` die Handlung ausführt. Am nächsten Takt, an dem die Bedingung
`AktivZeigt(eltern)` erstmals geprüft wird, sind damit zwei ihrer drei Teile
falsch (`zeilen_aktiv == 0` und `liest_aktiv == true`). Und `liest_aktiv` wird
erst falsch, nachdem `einzug_je_tab` die Meldung `Fertig` gesehen und
`Ordnermodell::abschliessen` gerufen hat, das die Sichtreihenfolge neu aufbaut
(`crates/krk-core/src/verzeichnis/modell.rs:218`) — gelesen **und** sortiert,
bevor die Auswahl kommt. Eine gleichzeitige Auffrischung aus C9 kann den
Warteschritt nur verzögern, nicht früher durchlassen: sie setzt `liest` auf
wahr und lässt die alten Zeilen stehen.

**Was zu tun bleibt.** Welcher der beiden Fälle der Abbruch vom 260806 war,
ist weiter unbeantwortet, und der Defekt bleibt deshalb offen. Die Frage
beantwortet der nächste vollständige Sitzungslauf von selbst: bricht er mit
der Meldung oben ab, war es die verworfene Auswahl, und der Ordner und die
Zeilenzahl in der Meldung sagen, woran sie scheiterte. Läuft er wieder zehn
Sekunden in die Geduld über L6, war es nicht die Auswahl, und der Verdacht
richtet sich auf die Messung selbst. Die Strecke verlangt KRK im Vordergrund
aus einem Terminalfenster; aus dem Hintergrund weist die
Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab.

**Geändert:** `crates/krk-ui/src/appkit/anwendung.rs` (Aufrufstelle und
`messhandlung`), `crates/krk-ui/src/messmodus.rs` (`auswahl_ohne_eintrag`,
`vorbereitungsfehler`, `vorbereitung_gescheitert`, Prüfung am Kopf von
`naechster_schritt`, Prüfmodul-Test
`eine_abgewiesene_auswahl_bricht_den_lauf_ab`). `make check` grün.

---
Deferred: der naechste vollstaendige Sitzungslauf mit KRK im Vordergrund — Nutzerarbeit.
Zurueckgestellt am 260810-1717 auf ausdrueckliche Wahl des Nutzers, in der Sitzung
`shared/history/260810-1647-orchestrator-session.md`. Die codierbare Haelfte des Befunds ist
am 260807 gebaut: ein abgewiesener `Auswahlversuch` in der Vorbereitung der Messstrecke bricht
den Lauf ab, statt in die Zehn-Sekunden-Geduld zu laufen. Offen bleibt allein, welcher der
beiden Faelle der Abbruch vom 260806 war, und das entscheidet keine Codeaenderung, sondern eine
Messung. Aus dem Hintergrund meldet die Messstrecke `NICHT_IM_VORDERGRUND` statt Zahlen; kein
Agent kann den Lauf fahren. Die zugehoerige offene Frage ist
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`.

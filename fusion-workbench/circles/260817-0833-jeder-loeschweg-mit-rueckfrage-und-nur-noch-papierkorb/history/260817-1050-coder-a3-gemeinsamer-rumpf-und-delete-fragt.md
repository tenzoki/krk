# Schritt 3 — der gemeinsame Rumpf, und `delete` fragt

**Datum:** 260817-1050
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Bündel A, Schritt 3
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C2

> Zur Reihenfolge der Dateinamen: das Protokoll von Schritt 2 heißt `260817-1104-…`, ist aber laut Dateisystem um 1036 geschrieben. Dieses hier trägt die Uhrzeit, die die Uhr beim Schreiben zeigte. Nach Namen sortiert steht es deshalb vor Schritt 2, der Sache nach dahinter.

---

## Was umgesetzt ist

`crates/krk-ui/src/appkit/anwendung.rs`

- Neuer Rumpf `loeschen_nach_rueckfrage(&self, art, frage, erlaeuterung, schaltflaeche, laut) -> bool` mit der Reihenfolge, die der Plan vorgibt: laufender Vorgang, leere Auswahl, Blatt, und bei Bestätigung der Auftrag. Der laufende Vorgang wird damit **vor** dem Blatt gemeldet und nicht mehr nach der Bestätigung; die Begründung steht als Absatz an der Funktion, denn eine Rückfrage, deren Ja folgenlos bleibt, gewöhnt den Nutzer an das Wegdrücken.
- Neuer `loeschauftrag_stellen(&self, art, auswahl, quellordner)`. Er baut den `Auftrag` aus der Auswahl, die im Blatt stand, und reicht ihn an `auftrag_starten`. Ein zweites `betroffene_eintraege()` nach der Bestätigung gibt es nicht mehr. Der Absatz an der Funktion schreibt den behobenen Defekt aus: zwischen der alten ersten und zweiten Lesung stand das Blatt, und ein Blatt hält weder FSEvents noch ein fremdes Programm an. Ein zweiter Absatz sagt, warum die **Fensterseite** dort trotzdem gelesen werden darf: ein stehendes Blatt weist jedes Kommando außer dem Abbruch ab und ist fenstermodal, die aktive Seite kann sich zwischen Frage und Antwort also nicht ändern.
- Die Auftragsdaten reisen durch den Rückruf in einer `Cell<Option<…>>`, weil `loeschbestaetigung::zeigen` ein `Fn` entgegennimmt und ein `Fn` nichts aus sich herausbewegen kann. Der Rückruf läuft genau einmal, also gibt `take()` den Inhalt genau einmal heraus.
- `in_den_papierkorb` ruft den neuen Rumpf mit `Art::InDenPapierkorb`, der Beschriftung „In den Papierkorb räumen" und `laut = false`. Sein Doc-Kommentar verliert den Satz „Sofort und ohne Rueckfrage" und den Verweis auf `shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`; an ihre Stelle tritt der neue Stand samt dem Schadensfall als Anlass.
- `endgueltig_loeschen` ruft denselben Rumpf mit seinen bisherigen beiden Texten, der Beschriftung „Endgültig löschen" und `laut = true`. Sein Doc-Kommentar nennt die zwei Punkte, in denen er sich doch ändert, beide zu seinen Gunsten und beide vom Plan verlangt: der laufende Vorgang wird vor dem Blatt gemeldet, und der bestätigte Auftrag trägt die gezeigte Auswahl.
- Zwei Zeilen mehr am Kopf: `use crate::kommandos::loeschwarnung;` und `Auswahl` in der bestehenden `operationen`-Zeile.

`crates/krk-ui/src/kommandos/loeschwarnung.rs`

- Das `#[cfg_attr(not(test), expect(dead_code, …))]` an `frage_und_erlaeuterung` ist weg. Es musste weg: mit dem Aufrufer wird die Erwartung unerfüllt, und unter `-D warnings` hätte der Bau angehalten. Genau dafür stand dort `expect` und nicht `allow`.
- Der Abschnitt „Der eine Aufrufer" nennt jetzt den, den es gibt. Der Plan hatte `loeschen_nach_rueckfrage` vorweggenommen; tatsächlich ruft `in_den_papierkorb` die Funktion und reicht die beiden Texte an den Rumpf weiter. Der Abschnitt über das Ablaufdatum des `expect` steht in der Vergangenheitsform da, statt zu verschwinden: er trägt die Begründung der Bauform, und die gilt für den nächsten, der sie braucht.

`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs`

- Der Hinweissatz lautet jetzt „Return und Esc brechen ab. Zum Bestätigen Cmd+Return." Er lautete „Zum Löschen Cmd+Return".

## Die Entscheidung zum Hinweissatz, und warum sie so ausfällt

Schritt 2 hat den Satz stehen lassen und die Frage an diesen Schritt weitergegeben. Er ist geändert, und zwar nicht wegen einer Unschärfe, sondern weil er einer Festlegung dieser Runde widersprach.

`loeschwarnung::frage_und_erlaeuterung` vermeidet das Wort „löschen" **ausdrücklich**: die Frage lautet „in den Papierkorb räumen", weil der Rückweg über den Papierkorb der Unterschied zu dem Weg ist, den diese Runde abschafft. Der Hinweissatz benutzte dasselbe Wort zwei Zeilen darunter im selben Blatt und nahm der Unterscheidung damit ihre Wirkung. Das war folgenlos, solange das Blatt einen einzigen Befehl bediente; seit Schritt 3 bedient es beide.

Der Satz benennt jetzt allein die Taste. Den Vorgang benennt die zweite Schaltfläche, und sie tut es in beiden Formen richtig — „In den Papierkorb räumen" beziehungsweise „Endgültig löschen". Ein Satz, der den Vorgang wiederholt, bräuchte je Form einen eigenen Wortlaut und wäre eine zweite Textquelle neben `loeschwarnung`; „Bestätigen" braucht keine und bleibt richtig, wenn mit Bündel D nur noch eine Form übrig ist. Die Begründung steht als eigener Absatz im Modulkopf von `loeschbestaetigung.rs`, damit der nächste Leser nicht dieselbe Frage noch einmal aufmacht.

## Was der Plan nicht bespricht und dieser Schritt entscheiden musste

Die Signatur aus dem Plan nimmt die beiden Texte fertig entgegen. Daraus folgt, dass der Aufrufer die Auswahl für sie liest und der Rumpf sie danach ein zweites Mal liest, für seine Prüfungen und für den Auftrag. Beide Lesungen liegen im selben Durchgang der Ereignisschleife, zwischen ihnen kann keine Auffrischung laufen, und der behobene Defekt bleibt behoben: er hing an der Lesung **nach** dem Blatt, und die gibt es nicht mehr.

Die Signatur ist trotzdem wörtlich so gebaut, wie der Plan sie nennt. Schritt 11 zieht das Bauen der Texte in den Rumpf, weil die Frage dort die Warngründe braucht, und nimmt die zweite Lesung mit; sie vorab zu beseitigen hieße, dieselbe Signatur zweimal zu ändern. Der Absatz an `loeschen_nach_rueckfrage` schreibt die Lage aus, statt sie unbemerkt zu lassen.

`loeschauftrag_stellen` liefert nichts zurück. Der Plan nennt für die Funktion keinen Rückgabewert, und im Rückruf des Blattes nimmt niemand mehr eine Antwort ab: der Tastendruck ist längst verbraucht.

## Der Weg der Rückschritt-Taste

Geprüft und unberührt. `kommandos/rueckschritt.rs` entscheidet weiter zwischen „ein Zeichen zurück", „nichts" und „in den Papierkorb"; die Regel selbst, ihre drei Eingänge und ihr einer Aufrufer sind nicht angefasst. Was sich hinter dem dritten Ausgang ändert, ist allein, dass dort jetzt ein Blatt steht.

Eine Nebenwirkung fällt dabei zugunsten des Nutzers aus: eine **gehaltene** Rückschritt-Taste ohne Filtertext räumte bisher Anschlag für Anschlag. Jetzt öffnet der erste Anschlag das Blatt, und jeder weitere wird von `blatt_steht` in `kommando_ausfuehren` abgewiesen. Die Prosa in `rueckschritt.rs`, die das Räumen noch „ohne Rückfrage" nennt, gehört zu Schritt 15 und ist hier nicht angefasst.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, Proben (1244 Proben), Clippy unter `-D warnings`, Formatprüfung.

## Was dieser Schritt nicht baut

Keine Papierkorbfrage, keine Zielprüfung, keine Zählung des Unterbaums, keine laute Form für `delete`. Vom endgültigen Löschen ist nichts entfernt. Bündel B bis E sind unberührt.

**Die Schutzschwelle ist erreicht.** KRK fragt ab jetzt vor jedem Löschvorgang nach, mit „Abbrechen" vorbelegt.

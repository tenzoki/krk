# Planung: der Fortschritt zieht in die Statuszeile, das Anlegen kommt in S17

**Datum:** 2026-08-04, 18:32
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`

---

## Auftrag

Zwei Nutzerentscheidungen vom 260804 aus der Abnahme von Schritt 16 in Spec und Plan einarbeiten. Die erste nimmt den Fortschritt einer Dateioperation aus dem Blatt und stellt ihn in die Statuszeile; sie ändert ein abgenommenes Ergebnis. Die zweite ordnet das Anlegen und das einzelne Umbenennen aus C4 dem Schritt 17 zu; sie schließt eine Lücke, für die kein Schritt eine Oberfläche vorsah.

Kein Eingriff in `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein Commit. Kein `[DONE]`-Vermerk geändert.

## Was entstanden ist

Zwei neue Planschritte, ein Entscheidungsdatensatz, ein Defekt.

- **S16b, Fortschritt in der Statuszeile statt im Blatt.** Eigener Schritt zwischen S16 und S17. Nimmt `crates/krk-ui/src/appkit/blaetter/fortschritt.rs` ersatzlos weg, gibt der Statuszeile eine dritte Quelle mit oberstem Rang und eigener Lebensdauer, schränkt die Tastensperre auf ein stehendes Blatt ein und bindet den Fortschritt an das Dateifenster, das die Operation begonnen hat. Zwei Diagramme: ein Entscheidungsbaum für die Auswahl der Quelle, ein Zustandsdiagramm für die Lebensdauer der Vorgangsanzeige.
- **S17b, Umbenennen eines einzelnen Eintrags in der Liste.** Abgeteilt von S17, weil das Umbenennen "direkt in der Liste" kein Blatt ist, sondern eine bearbeitbare Zelle der `NSTableView`.
- **S17 gewachsen** um das Anlegen von Ordner und Datei, mit einem gemeinsamen Namenseingabeblatt.
- `decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md` — die Entscheidung mit den drei Möglichkeiten, den gemessenen Zahlen und dem Haken, den die Wahl mitbringt.
- `issues/260804-1832_c_die-zahl-der-c4-abnahmekriterien-steht-im-plan-auf-sechzehn-und-im-spec-auf-achtzehn.md` — beim Durchzählen aufgefallen und im selben Zug behoben.

## Die vier Fragen, die der Auftrag gestellt hat

**Wo der Umbau landet.** In S16b, einem eigenen Schritt, nicht in S17. Der Umbau nimmt eine Ansicht weg und stellt eine Regel um; S17 baut eine Ansicht auf. Beide fassen `crates/krk-ui/src/kommandos/operationen.rs` an, was eine Reihenfolge verlangt und keine Zusammenlegung. Dazu wächst S17 mit der zweiten Entscheidung ohnehin. S16 bleibt abgenommen und bekommt eine Notiz, die die Abweichung benennt, wie schon bei der `unsafe`-Vorschrift aus S1 und beim Abnahmemaß aus C8.

**Die drei übrigen Blätter.** Alle drei bleiben. Konfliktfrage und Rückfrage vor dem endgültigen Löschen sollen sperren, weil die Operation ohne die Antwort nicht weiterläuft und die Vorbelegung auf Abbrechen eine Schaltfläche voraussetzt. Die Abschlussliste wartet auf keine Antwort, bleibt aber ein Blatt, weil sie bis zu zwölf Einträge mit Grund führt und in keine einzeilige Zeile passt, und weil sie erst erscheint, wenn nichts mehr läuft. Die 360 ms sind bei allen dreien folgenlos, weil keine der zehn Zusagen aus C8 sie misst.

**Die Vorrangregel aus S14.** Sie trägt den Fortschritt nicht und musste erweitert werden. Sie kennt zwei Quellen: die Fenstermeldung, die beim nächsten echten Ordner- oder Tabwechsel fällt, und die Tabmeldung. Der Fortschritt passt in keine von beiden. In die Fenstermeldung nicht, weil deren Löschregel die entgegengesetzte ist: eine Fenstermeldung soll beim Ordnerwechsel verschwinden, eine laufende Anzeige muss ihn überleben, weil die Operation weiterläuft und der Nutzer ab S16b navigieren darf. Ein Feld mit zwei Löschregeln wäre derselbe Fehler wie am 260804 bei der Auswurfmeldung. Die Regel wächst deshalb auf drei Ränge und behält ihr Ordnungsprinzip, das Alter der Aussage.

**C4 und L8.** Von den achtzehn Abnahmekriterien aus C4 ändern sich zwei, und eines kommt dazu. Nachgezogen sind das Kriterium zum Fortschritt und Abbruch, das jetzt den Ort und den sichtbaren Abbruchgriff nennt, und das Kriterium zum Fokusvorbehalt der Löschtasten, weil eine laufende Operation ab S16b kein Vorbehalt mehr ist. Neu ist ein Kriterium zur Bedienbarkeit während der Operation, samt der Regel, dass ein zweiter Operationsbefehl nichts startet und das sagt. Die Beschreibung von C4 bleibt wörtlich stehen; sie war richtig, die erste Umsetzung hielt sie nur in der schwächeren Lesart ein. **L8 bleibt bei 200 ms.** KRKs eigener Anteil von 152 bis 154 ms bleibt, die rund 360 ms für das Anhängen des Blattes fallen weg, eine Zeile erscheint mit dem nächsten Zeichendurchgang: die Zusage liegt bei rund 170 ms. Die neun übrigen Zahlen sind unberührt.

**Wie groß S17 wird.** Zu groß mit allen drei Funktionen, deshalb der Zuschnitt. Der Grund, den der Nutzer gewählt hat, trägt für das Anlegen und nicht für das Umbenennen: ein Eingabeblatt für den Ordnernamen ist dieselbe Form wie das Blatt des Stapel-Umbenennens, eine bearbeitbare Tabellenzelle ist es nicht. Mit beiden hätte S17 drei unabhängige Oberflächenmechanismen getragen. Die Zuordnung des Nutzers bleibt für zwei der drei Funktionen stehen; die dritte hat einen eigenen Schritt bekommen.

## Was nachgezogen ist

Im Plan: Kopfzeile und Nachzugsabsatz, die Directive mit 30 statt 28 Schritten, der Absatz über die Buchstabenschritte, `### Frage 6`, der Abhängigkeitsgraph mit zwei Knoten und fünf neuen Kanten bei drei weggefallenen, die drei Absätze unter dem Graphen, S16 mit der Abweichungsnotiz, S17, die Aufstellung der angelegten Defekte und die Diagramm-Selbstprüfung.

Im Spec: Kopfzeile und ein Standabsatz, C1 mit einem neuen Abnahmekriterium und zwei Festlegungen, C4 mit einem neuen und zwei nachgezogenen Kriterien und zwei Festlegungen, C8 mit der Zeile L8, dem Absatz über die beiden Änderungen und einem eigenen Absatz zur Begründung.

## Was ausdrücklich nicht angefasst ist

`issues/260804-1813_o_die-dateiliste-von-s16-nennt-drei-dateien-nicht-ohne-die-der-schritt-nicht-laeuft.md` und `issues/260804-1816_o_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung.md` liegen außerhalb des Auftrags und bleiben offen. Beide berühren die Schritte, um die es hier geht: der erste verlangt einen Nachzug der Dateiliste von S16, der zweite eine Änderung in `crates/krk-core/`, damit der Abbruchwunsch den Lauf ohne Umweg über den Vermittlerfaden erreicht. Der zweite wird durch S16b nicht schlimmer, weil der Weg des Abbruchs unverändert bleibt; allein sein Auslöser wandert von einer Schaltfläche auf die Taste, die er ohnehin trug.

Der Auftrag nennt "die vier genannten Defekte", benennt aber drei. Bearbeitet sind die drei benannten.

Ein Zeichendurchgang während einer offenen Umbenennung schreibt „Name/" in den Feldeditor

---

`zellenansicht` setzt die Anzeigeform in **jedem** Durchgang an die Zelle
(`tabelle.rs:2596`), auch wenn diese Zelle gerade der offene Editor einer Umbenennung ist.
Der `coder` hat am 260815 gemessen, dass ein `setStringValue:` während der Bearbeitung in
den Feldeditor zurückschreibt. Trifft ein Zeichendurchgang einen offenen Ordnernamen, steht
danach `Bilder/` im Feldeditor; Return liefert diesen Text an `umbenennung_beenden`, und
`name_pruefen` weist ihn mit „ein Name darf keinen Schrägstrich enthalten" ab — wegen eines
Zeichens, das der Nutzer nie getippt hat.

---

**Schwere:** mittel. Kein Datenverlust und keine falsche Dateioperation: `name_pruefen`
fängt den Schrägstrich sicher ab (`krk-core/src/operation/umbenennen.rs:72-74`). Falsch ist
die Meldung, und sie beschuldigt den Nutzer einer Eingabe, die die Anwendung selbst gemacht
hat. Vor dem Ordnerzeichen endete dieselbe Folge still als `Unveraendert`.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:2584-2596` (`zellenansicht`),
`:1743-1771` (`umbenennung_beenden`)
**Domain:** code

## Wie ein Zeichendurchgang während der Bearbeitung zustande kommt

Zwei Wege, beide ohne Zutun des Nutzers:

1. **Die Dateisystemwache.** Schreibt irgendein anderer Prozess in den angezeigten Ordner,
   ruft `auffrischung::ordner_neu_lesen` das Dateifenster, `neu_lesen` beginnt einen
   Lesevorgang und `nach_lesebeginn` ruft `reloadData` (`tabelle.rs:827-842`). Der Aufschub
   aus `schiebt_auffrischung_auf` greift nur für ein laufendes Stapel-Umbenennen
   (`auffrischung.rs:265-273`), nicht für eine offene Zelle.
2. **Der Takt des Lesevorgangs.** `einziehen` ruft `reloadData`, sobald ein Stapel den
   bisherigen Bestand ablöst oder die Sortierung steht (`tabelle.rs:2337`, `:2357`).

Der Klick in die Bereichsleiste scheidet als dritter Weg aus: er kommt während einer
Umbenennung ohnehin nicht durch (`shared/issues/260813-0311_o_…`).

## Der Unterschied zu vorher

| | vor `3b128c3` | seit `3b128c3` |
|---|---|---|
| in den Feldeditor geschrieben | `Bilder` | `Bilder/` |
| `umbenennung_pruefen` | `Unveraendert` | `Abgelehnt(Schraegstrich)` |
| was der Nutzer sieht | nichts | eine Fehlermeldung über sein Tippen |

Der getippte Text geht in beiden Fällen verloren; das ist der ältere Teil und gehört nicht
diesem Befund.

## Zwei Wege

1. **Die Zelle während der Bearbeitung nicht beschriften.** `zellenansicht` fragt vor dem
   `setStringValue:`, ob dieses Feld gerade den Feldeditor hält (`NSControl::currentEditor`,
   `NSControl.h:88`, seit 10.0), und lässt es dann stehen. Das erhält zugleich den getippten
   Text und behebt damit die ältere Hälfte mit.
2. **Die Anzeigeform beim Auslesen abschneiden.** `umbenennung_beenden` zieht
   `ohne_ordnerzeichen` über die Eingabe, bevor es prüft. Billiger, aber es schafft eine
   zweite Stelle, an der der Schrägstrich Bedeutung trägt, und nimmt dem Nutzer die
   Möglichkeit, den Schrägstrich als echten Tippfehler gemeldet zu bekommen.

Weg 1 ist der integrale: er beantwortet beide Hälften an einer Stelle. Wie jede Zusage an
dieser Datei gehört er am wirklichen Hauptfaden gemessen, bevor er behauptet wird.

---
Resolved: Nachgemessen am 260816, und **der Befund hält nicht.** Der Zeichendurchgang,
den er voraussetzt, findet nicht statt: AppKit reicht dem Delegierten nie eine Zelle mit
offenem Feldeditor.

**Was der Datensatz übersehen hat**, ist die Behandlung der bearbeiteten Zeile durch
`NSTableView` selbst. Gemessen am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm
auf dem wirklichen Hauptfaden, an einer `NSTableView` in einer `NSScrollView` mit derselben
Verdrahtung wie in der Datei (Zellenwiederverwendung über `makeViewWithIdentifier:owner:`,
`Namensfeld` als Zelle, Ziel und Aktion am Feld, 60 Zeilen, Schlüsselfenster, laufende
Ereignisschleife):

| Anlass | Bearbeitung danach | Durchgang der bearbeiteten Zeile | `currentEditor` darin |
|---|---|---|---|
| `reloadData` | beendet | ja, danach | `None` |
| `reloadDataForRowIndexes:columnIndexes:` | beendet | ja, danach | `None` |
| `noteNumberOfRowsChanged` | steht weiter | keiner | — |
| `selectRowIndexes:byExtendingSelection:` | beendet | keiner | — |
| Bildlauf aus dem Bild und zurück | steht weiter | keiner, die Zeile wird übersprungen | — |
| erstmaliger Aufbau einer Zeile | — | ja | `None` |

**In keinem einzigen Durchgang stand ein Feldeditor.** Beide Wege, die der Datensatz unter
„Wie ein Zeichendurchgang während der Bearbeitung zustande kommt" nennt, laufen über
`reloadData`, und `reloadData` beendet die Bearbeitung, bevor der erste Durchgang läuft.
Der Bildlauf, den der Datensatz nicht nennt, hält die bearbeitete Zeile aus dem Durchgang
heraus, statt sie zu beenden. Weg 1 aus „Zwei Wege" wäre damit toter Code, Weg 2 die
Antwort auf eine Lage, die es nicht gibt.

**Die Beobachtung über `setStringValue:` war für sich richtig und die Folgerung daraus
falsch.** Ein `setStringValue:` während der Bearbeitung schreibt tatsächlich in den
Feldeditor zurück; am 260816 wiederholt, das Feld stand danach auf `"Bilder/"` und der
Feldeditor ebenso. Nur erreicht `zellenansicht` diesen Zustand nie. Die Datei hat das
nirgends gesagt, und **das war der eigentliche Defekt**: er ist behoben, der Doc-Kommentar
von `zellenansicht` trägt jetzt die Messtabelle, die Regel und diesen Datensatz als den
Fehlschluss, der aus dem Schweigen entstanden ist; der Kopf von `Namensfeld::wird_ersthelfer`
trägt den Rückverweis, weil dort die richtige Hälfte der Beobachtung steht.

**Ein Kandidat ist dabei mitgemessen und gefallen:** `NSTableView::editedRow` und
`editedColumn` stehen während einer offenen Bearbeitung dieser Tabelle auf `-1`. Sie
gehören der zellenbasierten Tabelle, die hier ist ansichtsbasiert. Als Erkennung wären sie
unbrauchbar gewesen.

**Was die Messung stattdessen gefunden hat, ist ein echter Defekt und liegt im
Nachbardatensatz:** `reloadData` und `reloadDataForRowIndexes:columnIndexes:` beenden eine
offene Bearbeitung, ohne die Aktion `umbenennungBeendet:` zu schicken. Der getippte Text
wird fortgeworfen, umbenannt wird nichts. Das ist der dritte Ausgang aus
`shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-…`, und dieser
Datensatz hat gezeigt, dass er nicht nur an einem Klick des Nutzers hängt: die
Dateisystemwache und der Takt des Lesevorgangs lösen ihn ohne dessen Zutun aus. Der
Nachtrag steht dort.

Am Verhalten ist nichts geändert; die Änderung ist Prosa in
`crates/krk-ui/src/appkit/tabelle.rs`. Eine Probe kommt nicht dazu: die Zusage ist eine
über AppKit, sie braucht ein Fenster und einen Feldeditor, und `libtest` gibt den
Hauptfaden nicht her (`issues/260810-1001_*_…`).

Abnahme: `make check` — exit 0, „alle vier gruen".

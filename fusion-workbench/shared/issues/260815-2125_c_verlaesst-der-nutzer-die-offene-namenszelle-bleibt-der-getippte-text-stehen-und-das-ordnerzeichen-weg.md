Verlaesst der Nutzer die offene Namenszelle, bleibt der getippte Text stehen und das Ordnerzeichen weg

---

Das Umbenennen an Ort und Stelle aus C4 kennt drei Ausgaenge, aber nur zwei sind
verdrahtet. Return schickt die Aktion `umbenennungBeendet:`, Escape laeuft in
`abortEditing`. **Der dritte Ausgang ist der Fokusverlust** — ein Klick in eine andere
Zeile, in das andere Dateifenster, in eine andere Anwendung —, und er schickt weder das
eine noch das andere. Die Bearbeitung endet, `controlTextDidEndEditing:` kommt, eine
Aktion kommt nicht, und niemand zeichnet die Zeile neu.

Die Folge stand schon vor dem Ordnerzeichen im Baum und ist seit dem 260815 sichtbarer:
der getippte Text bleibt in der Zelle stehen, obwohl nicht umbenannt worden ist, und ein
Ordner steht dabei zusaetzlich ohne seinen Schraegstrich da. Beides bis zum naechsten
Zeichendurchgang der Zeile.

---

**Schwere:** mittel. Kein Datenverlust und keine falsche Dateioperation — umbenannt wird
nichts, was der Nutzer nicht mit Return bestaetigt hat. Falsch ist allein die Anzeige, und
sie behauptet eine Umbenennung, die nicht stattgefunden hat.
**Gefunden von:** coder, beim Einbau des Ordnerzeichens (Nutzerentscheid 260815-2058)
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `DateifensterQuelle::umbenennung_beenden`
und `Namensfeld`
**Domain:** code

## Was gemessen ist

Am 260815 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem Hauptfaden, an einer
`NSTableView` mit bearbeitbarer Zelle:

| Ausgang der Bearbeitung | Aktion | `controlTextDidEndEditing:` | `abortEditing` |
|---|---|---|---|
| Return (`insertNewline:`) | ja, **nach** der Meldung | ja | nein |
| Escape (`cancelOperation:`) | nein | **nein** | ja |
| Fokusverlust (`makeFirstResponder:` auf die Tabelle) | **nein** | ja | nein |

`sendsActionOnEndEditing` der Zelle steht auf 0, und das ist die Vorgabe des Systems.

## Warum es der Auftrag vom 260815 nicht mitbehoben hat

Zwei Gruende, beide inhaltlich:

1. **Der Nutzerentscheid nennt den Fall nicht.** Zugesagt sind drei Dinge: der Beginn ohne
   Zeichen, Return liest den getippten Text, Escape stellt die Anzeigeform her. Der
   Fokusverlust ist keines davon.
2. **Die naheliegende Behebung waere eine Reihenfolgefalle.** Ein Zeichendurchgang in
   `controlTextDidEndEditing:` kommt **vor** der Aktion; `umbenennung_beenden` laese danach
   die Anzeigeform statt des getippten Textes und wollte einen Ordner auf `Name/`
   umbenennen. Wer den Fall dort behebt, muss die Textbewegung aus `userInfo` auswerten
   (`NSTextMovement`), also eine Fallunterscheidung einziehen, oder die Verdrahtung des
   Endes umbauen.

## Die Frage, die vorher zu entscheiden ist

**Was soll ein Fokusverlust tun?** Die Antwort ist nicht offensichtlich und aendert das
Verhalten von C4:

- **Verwerfen wie Escape.** Dann ist die Zelle wieder die Anzeigeform, und ein
  Versehen kostet nichts. Der Finder macht es nicht so.
- **Uebernehmen wie Return.** Dann benennt ein Klick daneben um. Das ist das Verhalten des
  Finders und von ForkLift, und es ist die groessere Aenderung: `sendsActionOnEndEditing`
  auf 1, und die Umbenennung laeuft ohne ausdrueckliche Bestaetigung.

Solange das nicht entschieden ist, bleibt der Zustand am Code, wie er ist.

---

## Nachtrag der Durchsicht vom 260815-2202 (coderev, `3b128c3`)

**Die Anzeigehälfte lässt sich beheben, ohne die Nutzerfrage zu beantworten**, und die
Reihenfolgefalle oben steht dem nicht entgegen. Sie betrifft die **Delegiertenmeldung**
`controlTextDidEndEditing:`, die vor der Aktion kommt. Die Methode am Feld selbst,
`-[NSTextField textDidEndEditing:]` (`NSTextField.h:37`, ohne `API_AVAILABLE`, also seit
10.0), ist dagegen die Stelle, aus der AppKit die Aktion **schickt**. Eine Überschreibung
auf `Namensfeld`, die zuerst `super` ruft und danach die Zeile neu zeichnet, läuft damit
**nach** der Aktion; `umbenennung_beenden` hat den getippten Text dann bereits gelesen, und
der Zeichendurchgang holt anschließend die Anzeigeform. Der Return-Weg zeichnet die Zeile
dann zweimal, was folgenlos ist; der Escape-Weg erreicht `textDidEndEditing:` nicht und
bleibt bei `abortEditing`.

`inference:` Am wirklichen Hauptfaden nicht nachgefahren. Die Ordnung Aktion-vor-Rückkehr
ist aus dem Kopf des Systems abgelesen und gehört wie jede andere Zusage dieser Datei
gemessen, bevor sie behauptet wird.

**Das entscheidet die offene Frage nicht mit.** Ein Zeichendurchgang stellt die Anzeige auf
den Stand her, den das Modell trägt — und das Modell ist beim Fokusverlust unverändert, weil
nichts umbenannt wird. Die Anzeige holte damit nur nach, was ohnehin gilt. Ob ein Klick
daneben später verwerfen oder übernehmen soll, bleibt offen und träfe dann auch diesen
Zeichendurchgang.

**Nebenbefund, eigener Datensatz:** Der Doc-Kommentar von `umbenennung_beenden`
(`tabelle.rs:1727-1729`) nennt den Fokusverlust weiter als Aufrufer der Aktion und
widerspricht damit der Messtabelle oben —
`shared/issues/260815-2204_o_der-doc-kommentar-von-umbenennung-beenden-…`.

---

## Nachtrag der Messung vom 260816 (coder, bei der Nachprüfung von `260815-2203`)

**Der dritte Ausgang hängt nicht nur an einem Klick des Nutzers.** Zwei weitere Anlässe
lösen ihn aus, beide ohne dessen Zutun, und beide sind Programmwege dieser Datei:

| Anlass | Bearbeitung danach | Aktion `umbenennungBeendet:` | getippter Text |
|---|---|---|---|
| `reloadData` | beendet | **nein** | fort |
| `reloadDataForRowIndexes:columnIndexes:` | beendet | **nein** | fort |
| `selectRowIndexes:byExtendingSelection:` | beendet | **nein** | bleibt in der Zelle stehen |
| `noteNumberOfRowsChanged` | steht weiter | — | — |
| Bildlauf aus dem Bild und zurück | steht weiter | — | — |

Gemessen am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem wirklichen
Hauptfaden, an einer `NSTableView` in einer `NSScrollView` mit derselben Verdrahtung wie in
der Datei; zur Gegenprobe schickt `insertNewline:` am Feldeditor im selben Lauf die Aktion
mit dem getippten Text, die Verdrahtung trägt also.

Die Rufer der ersten beiden stehen in `crates/krk-ui/src/appkit/tabelle.rs`:
`nach_lesebeginn` (jede Navigation und jede Auffrischung durch die Dateisystemwache) und
`einziehen` (der Takt des Lesevorgangs, sobald ein Stapel den bisherigen Bestand ablöst
oder die Sortierung steht). Schreibt also irgendein anderer Prozess in den angezeigten
Ordner, während der Nutzer einen Namen tippt, endet seine Bearbeitung still und der
getippte Text ist fort.

**Das hebt die Schwere und ändert die offene Frage nicht.** Der Datensatz fragt, was ein
Fokusverlust tun soll, verwerfen oder übernehmen; die Antwort trägt diese beiden Anlässe
mit. Für sie ist „übernehmen" allerdings die unangenehmere Wahl: eine Umbenennung, die eine
Schreibbewegung eines fremden Prozesses auslöst, hat der Nutzer nicht bestätigt.

**Eine Zeile der Messtabelle oben widerspricht der Tabelle unter „Was gemessen ist" nicht,
sondern lässt sie offen:** ob `controlTextDidEndEditing:` bei diesen beiden Anlässen kommt,
ist hier **nicht** gemessen. Das Messprogramm hat den Delegierten der Tabelle geführt und
keinen Delegierten am Feld gesetzt, und `NSTextField` schickt die Meldung an seinen eigenen
Delegierten. Die Datei setzt ihn ebenso wenig; wer die Meldung als Aufhänger einer Behebung
nimmt, misst das zuerst.

---

## Nachtrag der Umsetzung vom 260816-0040 (coder, Nutzerentscheid 260816-0021)

**Zwei der drei Ausgänge sind abgetragen, der Befund bleibt offen.**
`crate::auffrischung::ordner_neu_lesen` lässt ein Dateifenster nicht mehr lesen, solange
darin eine Namenszelle in Bearbeitung steht; es merkt die Auffrischung stattdessen vor, und
das Ende der Bearbeitung holt sie nach. Damit erreichen weder die Dateisystemwache noch der
Abschluss einer Dateioperation die offene Zelle. Der getippte Text überlebt eine fremde
Schreibbewegung im angezeigten Ordner.

**Was bleibt:** der wirkliche Klick des Nutzers neben die Zelle — sowohl sein Ausgang
(verwerfen oder übernehmen,
`shared/decisions/260816-0021_o_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`)
als auch die Anzeigehälfte, die daran hängt. Der Zustand am Code ist für diesen Weg
unverändert.

**Ein dritter Weg ist gemessen und liegt getrennt:** `DateifensterQuelle::einziehen`, der
Takt eines schon laufenden Lesevorgangs, läuft an `ordner_neu_lesen` vorbei und ist vom
Aufschub nicht erreicht —
`shared/issues/260816-0040_o_der-takt-eines-laufenden-lesevorgangs-beendet-eine-offene-namenszelle-und-der-aufschub-erreicht-ihn-nicht.md`.

**Die Messtabelle der Enden ist in derselben Sitzung erweitert worden** und steht im
Sitzungsprotokoll
`shared/history/260816-0040-coder-aufschub-der-auffrischung-bei-offener-namenszelle.md`:
`-[NSTextField textDidEndEditing:]` trägt jedes Ende außer Escape, `abortEditing` trägt
Escape, und `textDidEndEditing:` **schickt** die Aktion, statt ihr vorauszulaufen. Damit ist
auch die Frage beantwortet, die der Nachtrag vom 260816 offengelassen hat: der Delegierte
am Feld wird für die Behebung nicht gebraucht.

---

**Resolved:** 260816-1017. Der dritte Ausgang ist verdrahtet, und beide Hälften sind
abgetragen.

**Die Verwerfen-Hälfte war schon da und ist nicht gebaut, sondern gemessen worden:** ein
Ende ohne Return schickt die Aktion `umbenennungBeendet:` nicht, also benennt ein Klick
neben die Zelle nichts um. Der Nutzer hat am 260816-0935 entschieden, dass es dabei
bleibt (`shared/decisions/260816-0021_*_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`,
Option 1: verwerfen wie Escape).

**Die Anzeigehälfte ist gebaut.** `-[Namensfeld textDidEndEditing:]` ruft nach `super`
bedingungslos `DateifensterQuelle::anzeigeform_herstellen` — dieselbe Methode, die Escape
schon rief, unter ihrem alten Namen `umbenennung_abgebrochen`. Damit stellt jedes Ende
einer Bearbeitung, dem keine Umbenennung folgt, die Anzeigeform wieder her: der getippte
Text ist fort, das Ordnerzeichen ist zurück, und die Zelle behauptet keine Umbenennung
mehr, die nicht stattgefunden hat.

Acht Ausgänge am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem
wirklichen Hauptfaden gemessen, mit derselben Verdrahtung wie in der Datei. Die Zeile
`Fokusverlust ohne Zeichendurchgang` reproduziert den Befund (`getippt` bleibt stehen),
die Zeile mit Durchgang zeigt die Behebung (`alpha/` steht wieder da). Der zweite
Durchgang nach Return ist folgenlos, und nach einer ausgelösten Auffrischung fällt der
Durchgang über `rowForView` = -1 still aus. Der Nachhol-Weg aus `27dca57` kommt ihm nicht
in die Quere: ein Zeichendurchgang ist kein Lesevorgang und fasst `auffrischung_vorgemerkt`
nicht an.

**Nicht mit abgetragen, und mit Absicht:** der Verlust des getippten Textes, wenn der Takt
eines laufenden Lesevorgangs die Bearbeitung beendet. Die **Anzeige** ist auch dort jetzt
richtig, denn `einziehen` endet ebenfalls über `textDidEndEditing:`; was bleibt, ist das
stille Ende selbst, und das führt
`shared/issues/260816-0040_o_der-takt-eines-laufenden-lesevorgangs-beendet-eine-offene-namenszelle-und-der-aufschub-erreicht-ihn-nicht.md`.

**Was der Nutzer noch von Hand abnimmt:** den wirklichen Klick mit der Maus. Jedes Ende
ist programmatisch nachgefahren; ein echtes Mausereignis kann kein Agent erzeugen.

`make check` — exit 0. Verlauf:
`shared/history/260816-1017-coder-anzeigeform-an-jedem-ende-ohne-umbenennung.md`

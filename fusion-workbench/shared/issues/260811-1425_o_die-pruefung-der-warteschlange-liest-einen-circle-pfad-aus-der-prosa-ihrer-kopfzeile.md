Die Prüfung der Warteschlange liest einen Circle-Pfad aus der Prosa ihrer Kopfzeile

---

Die kanonische Prüfung „Reading a queue" (`agents/orchestrator.md`, Abschnitt
`### The queue's ground` → `#### Reading a queue`) hat am 260811-1330 einen **Fehlbefund**
geliefert: sie meldete `queue: STALE — built for Circle 260802-0842-krk-mac-dateimanager-editor-git`,
während die Kopfzeile derselben Datei ausdrücklich `**Active Circle:** keiner` sagte.

Die Schlange war weder veraltet noch für jenen Circle gebaut. Sie war über `shared/` gebaut und
zum Zeitpunkt der Prüfung vollständig abgearbeitet.

---

**Schwere:** Mittel — die Prüfung existiert, um genau diese Frage zu beantworten, und beantwortet
sie falsch
**Gefunden:** orchestrator, beim Abschluss des Circles `260809-2040-tastenbelegung-als-markdown-in-downloads`
**Betroffen:** fusion, nicht KRK — `agents/orchestrator.md`; mitbenutzt von `/fusion:setup`
Schritt 3 und `/fusion:next` Schritt 5
**Domain:** code

## Die Ursache

Die Prüfung zieht den Circle-Namen so aus der Kopfzeile:

```sh
G=$(grep -m1 '^\*\*Active Circle:\*\*' "$Q" | grep -oE 'circles/[A-Za-z0-9._-]+|`[A-Za-z0-9._-]+`' | head -1 | tr -d '`' | sed 's|^circles/||')
```

Das zweite `grep` sucht **irgendwo in der Zeile** nach `circles/<name>` oder nach etwas in
Backticks. Die Zeile ist aber Prosa und darf es sein: die Vorlage schreibt kein Format vor. Im
gemessenen Fall lautete sie sinngemäß

> `**Active Circle:** keiner. Beim Bau dieser Schlange war kein Circle aktiv, … Die fünf Aufgaben
> aus dem Circle der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/`)
> stehen auf ausdrückliche Festlegung des Nutzers in der Liste …`

`head -1` nimmt den **ersten** Treffer, und der erste Treffer war der Pfad aus dem Nebensatz. Das
Wort „keiner" direkt hinter dem Doppelpunkt hat die Prüfung nie gelesen.

## Warum das mehr ist als ein Regex-Fehler

**Die Prüfung fällt in die falsche Richtung.** Der Fehlbefund lautet nicht „weiß nicht", sondern
`STALE` mit einem konkreten, falschen Circle-Namen — also die Zeile mit dem höchsten
Handlungsdruck. `/fusion:next` gibt sie dem Nutzer in dem Moment aus, in dem er entscheidet,
woran er als Nächstes arbeitet, und `agents/orchestrator.md` nennt genau das den Grund, warum
diese Prüfung dort steht.

Im gemessenen Fall hätte der Fehlbefund beinahe dazu geführt, eine abgearbeitete Schlange mit
einer erfundenen Begründung zurückzuziehen. Aufgefallen ist er nur, weil jemand die Kopfzeile
gelesen hat, statt der Prüfung zu glauben.

**Und er trifft die vierte Zeile der Verdikt-Tabelle mit.** „Kopf nennt keinen Circle" ist die
Bedingung für zwei der vier Zeilen; solange eine Prosa-Erwähnung als Nennung durchgeht, sind
beide unerreichbar, sobald die Kopfzeile irgendwo einen Pfad erwähnt.

## Denkbare Wege

1. **Den Treffer auf den Zeilenanfang binden.** Nur was unmittelbar hinter
   `**Active Circle:**` steht, zählt. Ein „keiner", „none" oder ein leerer Rest bedeutet: kein
   Circle genannt. Billigste Änderung, und sie deckt den gemessenen Fall.
   - Offen bleibt, welche Wörter als „kein Circle" gelten. Deutsch und Englisch stehen
     nebeneinander, und eine Wortliste ist genau die Sorte Aufzählung, die veraltet.
2. **Das Feld maschinenlesbar machen.** Die Kopfzeile trägt den Verzeichnisnamen oder nichts,
   und die Begründung geht in eine zweite Zeile darunter.
   - Löst die Frage an der Wurzel: eine Prüfung, die ein Feld liest, braucht ein Feld und keinen
     Absatz. Verlangt aber eine Änderung an `agents/taskplanner.md`, der die Zeile schreibt.
   - **Und es hängt an einem bereits offenen Befund:** `taskplanner` schreibt die Zeile heute gar
     nicht verbindlich, siehe
     `260810-0431_*_the-work-queue-does-not-record-the-ground-it-was-built-on` in fusions eigenem
     Arbeitsbereich. Wer das Format festlegt, beantwortet beide zusammen.
3. **Beides.** Das Feld festlegen und die Prüfung darauf binden.

## Was daran allgemein ist

Die Prüfung liest ein **Feld** aus einem Dokument, dessen Format niemand festgelegt hat. Das geht
so lange gut, wie der Schreiber sich zufällig knapp fasst. Es ist dieselbe Form wie bei den
Suchmustern, die `\.md` verlangen und Kurzformen übersehen
(`shared/issues/260810-1851_*_…`, geschlossen): ein Muster, das auf eine Gewohnheit trifft statt
auf eine Zusage.

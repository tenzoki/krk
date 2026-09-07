# Wie weit reicht die neue Regel für den Zustand eines Anforderungsdokuments in den Bestand zurück?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:**
`260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`
(die Frage, deren Antwort diese hier aufwirft),
`260812-2002_*_bleibt-der-vorspann-eines-containers-die-eine-luecke-in-der-deckungszusage-von-c4-3.md`
(eine beantwortete Frage, deren Umsetzung an dieser hängt),
`260828-0712_*_wie-erreicht-eine-us-tastaturbelegung-cmd-plus-wenn-das-pluszeichen-dort-die-umschalttaste-braucht.md`
(eine zweite, deren Plantext dieselbe Sperre trifft)

---

## Frage

Der Nutzer hat am 260907-0823 entschieden: **der Zustand eines Anforderungsdokuments folgt
der belegten Bauarbeit, und die Abnahme bekommt eine eigene Kopfzeile.** Ein Zustand mit vier
Werten kann nicht zwei unabhängige Fragen beantworten — ist es gebaut, ist es abgenommen —,
ohne bei jeder Wahl eine falsche Auskunft zu geben.

**Diese Antwort trifft auf eine Regel des Rahmenwerks, und beide lassen sich nicht zugleich
auf den Bestand anwenden.** `rules/circle-records.md` hält fest, dass das Anforderungsdokument
und der Plan einer geschlossenen Runde Aufzeichnung sind: sie werden als Beleg gelesen und
nicht mehr an Ort und Stelle nachgeführt, keine Schrittmarke, kein gehaktes Kriterium, keine
Kopfänderung nach dem Abschluss.

**Erhoben am 260907-0823:** alle 42 Anforderungsdokumente und Pläne dieses Projekts gehören
zu Runden, die geschlossen sind. Sechs davon liegen in `shared/planning/` und damit außerhalb
einer Runde; die übrigen liegen in Runden mit den Abschlussmarken `_b_`, `_c_` oder `_d_`. Der
Bestand zeigt den Streit, den die Antwort beenden sollte: sieben Dokumente stehen auf offen,
fünf auf erledigt, zwei auf „in Arbeit", und an keinem davon arbeitet jemand — der
Ausweichzustand behauptet eine Tätigkeit, die es nicht gibt.

**Die Frage ist heute dreimal aufgetaucht** und blockiert inzwischen zwei beantwortete
Fragen, deren Umsetzung ein solcher Text wäre: die Ergänzung der Deckungszusage aus der
Runde 6 um ihre zwei Ausnahmen, und die Angabe zur US-Tastatur im Plantext der Runde 20, der
mehr zusagt, als der Baum hält.

## Optionen

1. **Nur nach vorn.** Kein vorhandenes Dokument wird angefasst; die Regel gilt ab dem
   nächsten.
   - Pro: die Regel des Rahmenwerks bleibt unangetastet, und keine Aufzeichnung wird
     nachträglich umgeschrieben. Nichts ist zu tun.
   - Cons: der widersprüchliche Bestand bleibt stehen, jede Zählung offener Planungsarbeit
     meldet weiter sieben Posten, an denen niemand arbeitet, und die zwei blockierten
     Antworten bleiben unumsetzbar. Ihre Datensätze tragen dann dauerhaft eine Antwort ohne
     Umsetzung.
2. **Auch die sechs Dokumente außerhalb einer Runde.** Die Regel des Rahmenwerks spricht von
   Texten *in* einer geschlossenen Runde und erfasst diese sechs nicht.
   - Pro: räumt den Teil des Widerspruchs weg, den keine Regel schützt, und tastet keine
     Aufzeichnung an, die geschützt ist.
   - Cons: der Bestand ist danach nach zwei verschiedenen Regeln geführt, je nach Ort der
     Datei. Die zwei blockierten Antworten bleiben blockiert, denn ihre Texte liegen in
     Runden.
3. **Auf alle 42, samt der Zusage der Runde 6 und dem Plantext der Runde 20.**
   - Pro: der Bestand wird durchgängig, und beide blockierten Antworten werden umsetzbar.
   - Cons: Aufzeichnung wird nachträglich umgeschrieben, gegen eine Regel des Rahmenwerks,
     die dieses Projekt nicht ändern kann. Dazu die Nebenwirkung, die der vorbereitende Lauf
     am 260906 genannt hat: die sieben umbenannten Dateien werden beim nächsten Aufräumen
     archiviert, und Kurzverweise auf sie zeigen danach unbemerkt ins Leere.

## Randbedingungen

- `rules/circle-records.md` gehört dem Rahmenwerk und kann von hier aus nicht geändert werden.
- Die Antwort vom 260907-0823 steht und wird durch diese Frage nicht wieder aufgemacht; hier
  geht es allein um ihre Reichweite in den vorhandenen Bestand.
- Eine Kopfzeile für die Abnahme in einem Text anzulegen, der nach der Regel nicht mehr
  angefasst wird, ist derselbe Eingriff wie eine Umbenennung — die Antwort gilt für beide
  Hälften gleich.

## Empfehlung

Möglichkeit 2. Sie räumt den Teil weg, den keine Regel schützt, und lässt die Aufzeichnung in
Ruhe. Für die zwei blockierten Antworten heißt das: Antwort notiert, Text unverändert, Lücke
dokumentiert statt korrigiert — was schlechter ist als eine Korrektur und besser als eine
stille Unstimmigkeit zwischen Zusage und Baum.

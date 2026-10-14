# Wird der geschätzte Zeitstempel in einem Dateinamen prüfbar gemacht?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:**
`260906-0206_*_werden-dateinamen-mit-vorauslaufendem-zeitstempel-umbenannt-oder-vermerkt.md`
(die Frage, deren vierte Möglichkeit dieser Datensatz weiterführt),
`260824-1758_*_die-zeitstempel-in-dateinamen-laufen-der-uhr-voraus-bis-zu-drei-stunden.md`,
`260818-0343_*_zwei-dateien-dieser-sitzung-tragen-einen-zeitstempel-fast-zwei-stunden-in-der-zukunft.md`,
`260828-1044_*_fuenf-history-dateien-der-runde-20-tragen-zeitstempel-die-nach-ihrem-eigenen-commit-liegen.md`,
`260812-1805_*_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`
(die vier Befunde, die der Nutzer am 260907-1301 als Lage angenommen geschlossen hat)

---

## Frage

Der Nutzer hat am 260907-1301 entschieden, den Bestand nicht umzubenennen: neunzehn
Verlaufsdateien behalten ihre geschätzten Zeitstempel, und ein Vermerk am jeweiligen
Befund trägt die Berichtigung. **Er hat dabei ausdrücklich die vierte Möglichkeit als
eigene Frage bestellt: die Ursache prüfbar zu machen, damit der nächste Fall nicht
entsteht.** Dieser Datensatz ist diese Frage.

Die Ursache ist bekannt und in vier Befunden über vier Monate belegt: ein Agent schreibt
den Zeitstempel eines neuen Datensatzes aus dem Kopf, statt ihn mit `date +%y%m%d-%H%M`
abzufragen. Die Abweichung reicht bis zu drei Stunden, in beide Richtungen. Sichtbar wird
sie erst, wenn jemand den Verlaufsspeicher nach dem Namen sortiert liest — dort steht dann
ein späterer Schritt vor einem früheren.

**Die Regel dagegen steht längst und hält nicht.** `rules/fusion-workbench-conventions.md`
`## Timestamps` schreibt vor, den Stempel immer aus `date +%y%m%d-%H%M` zu nehmen, mit der
Begründung, dass ein Sprachmodell keine Uhr hat. Vier Befunde in vier Monaten sagen, dass
eine Anweisung in einem Prompt diese Klasse Fehler nicht abstellt.

## Optionen

1. **Eine Prüfung im Rahmenwerk**, die beim Anlegen einer Datei den Namensstempel gegen die
   Uhr hält und über einer Toleranz abweist.
   - Pro: fängt den Fehler an der einzigen Stelle, an der er entsteht, und zwar bevor die
     Datei existiert. Kein Bestand ist nachzupflegen.
   - Contra: liegt in fusion und nicht in diesem Projekt, ist also von hier aus nicht zu
     bauen. Braucht eine Toleranzgrenze, über die wieder zu entscheiden wäre — eine Datei
     darf legitim später committet als geschrieben werden.
2. **Eine Prüfung in diesem Projekt**, die den Bestand nachträglich gegen den Commitverlauf
   hält und Abweichungen meldet.
   - Pro: hier baubar, und sie sieht auch Dateien, die ein Rahmenwerkslauf nicht angelegt
     hat.
   - Contra: sie findet den Fehler erst, wenn er schon im Baum steht, und der Nutzer hat
     gerade entschieden, ihn dann **nicht** mehr zu berichtigen. Eine Meldung ohne
     zugehörige Handlung wird weggelesen.
3. **Die Toleranz zur Regel machen statt zur Prüfung**: der Stempel darf abweichen, und die
   Abfolge wird ausdrücklich aus dem Commitverlauf gelesen und nie aus der Namensliste.
   - Pro: kostet nichts und beschreibt, was der Nutzer am 260907 ohnehin entschieden hat.
   - Contra: gibt die Sortierbarkeit des Verlaufsspeichers auf, die heute die einzige
     Auskunft darüber ist, was worauf folgte, ohne ein zweites Werkzeug zu bemühen.

## Randbedingungen

- Die Regel selbst (`## Timestamps`) ist nicht dieses Projekts Eigentum und kann von hier
  aus nicht geändert werden.
- Der Bestand wird nach der Entscheidung vom 260907-1301 nicht mehr angefasst. Jede Antwort
  wirkt allein nach vorn.
- Eine Toleranzgrenze ist selbst eine Entscheidung und keine Messung; wer eine setzt, sagt
  dazu, woher die Zahl kommt.

## Empfehlung

Keine. Die drei Möglichkeiten liegen in verschiedenen Zuständigkeiten — die erste in
fusion, die zweite hier, die dritte in keiner Datei —, und welche davon die Mühe wert ist,
hängt daran, wie oft der Nutzer den Verlaufsspeicher tatsächlich nach dem Namen sortiert
liest. Diese Frage beantwortet nur er.

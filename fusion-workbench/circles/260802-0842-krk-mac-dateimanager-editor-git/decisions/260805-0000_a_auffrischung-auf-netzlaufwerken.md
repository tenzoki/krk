# Bekommt KRK einen zweiten Auffrischungsweg für Netzpfade, oder engt C9 die Zusage auf lokale Dateisysteme ein?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1451_c_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-0000_o_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C9), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (`### Frage 3`, S14)

---

## Question

C9 schließt die vom Finder eingehängten Netzlaufwerke ausdrücklich ein: "Ein vom Finder verbundenes Netzlaufwerk erscheint damit als gewöhnlicher Pfad und ist eingeschlossen." Die Dateisystem-Beobachtung aus S14 läuft über FSEvents, und FSEvents deckt Netzdateisysteme nicht ab. Ein Dateifenster auf einem SMB- oder NFS-Pfad zeigt fremde Änderungen deshalb erst, wenn der Nutzer den Ordner wechselt und wieder zurückkommt. `inference:` Nicht gemessen; zum Nachprüfen fehlte am 260804 ein Server.

## Options

1. **C9 auf lokale Dateisysteme einengen.** Der Zugriff bleibt eingeschlossen, die selbsttätige Auffrischung nicht, und C9 schreibt aus, was der Nutzer auf einem Netzpfad stattdessen erlebt.
   - Pros: ein Auffrischungsweg bleibt einer; die Zusage sagt, was KRK wirklich leistet; nichts Ungemessenes wird zugesagt.
   - Cons: eine Zusage wird kleiner, und der Nutzer eines Netzlaufwerks muss von Hand nachsehen.
2. **Ein zweiter Weg für Netzpfade**, etwa ein Abfragetakt, der am selben `ordner_neu_lesen` endet.
   - Pros: die Zusage bleibt, wie sie ist.
   - Cons: ein zweiter Auslöser mit eigener Bedingung, eigenem Takt und eigenem Rückfallweg; der Plan schließt genau das an vier Stellen aus. Nachprüfbar wäre er ohne eigens aufgesetzten Server nicht, und ein ungeprüfter Mechanismus in der Auslieferung ist schlechter als eine ehrlich verkleinerte Zusage.
3. **Ein Auffrischungsbefehl auf einer Taste.** Der Nutzer frischt selbst auf, überall.
   - Pros: kein zweiter selbsttätiger Weg.
   - Cons: eine Taste mehr in der Auslieferungsbelegung für einen Fall, den die ausgelieferte Lage nicht herstellt, und der Umfang der Runde wüchse ohne Auftrag.

## Constraints

- `### Frage 3` des Plans: ein Eintrittspunkt für fremde und eigene Änderungen, `ordner_neu_lesen`, mit zwei Auslösern. Ein zweiter Auffrischungsweg ist ausdrücklich ausgeschlossen, und das Abnahmekriterium von S14 prüft es am Diff.
- Die Maxime "supersimpel" wirkt als Ausschlussgrund gegen eine Lösung mit eigener Sonderregel und eigenem Rückfallweg.
- Der Zugriff selbst ist nicht betroffen: Lesen, Navigieren und die Dateioperationen aus C4 laufen auf einem eingehängten Netzpfad über gewöhnliche Systemaufrufe.

## Recommendation

Möglichkeit 1.

---
Answered: Nutzer am 260805-0000 — Möglichkeit 1. Begründung des Nutzers: ein zweiter Auffrischungsweg widerspräche der Ein-Weg-Regel, die dieser Plan an vier Stellen durchhält, und nachprüfen ließe er sich ohne Server ohnehin nicht.

**Was C9 jetzt sagt.** Der Zugriff bleibt eingeschlossen und ist im ersten Abnahmekriterium ausgeschrieben: Lesen, Navigieren und die Dateioperationen aus C4 wirken auf einem eingehängten Netzpfad wie auf einem lokalen. Ein neues Kriterium trennt davon die Auffrischung: fremde Änderungen erscheinen ohne Zutun **auf lokalen Dateisystemen**; auf einem Netzpfad sieht der Nutzer sie erst, wenn er den Ordner verlässt und wieder betritt. Eigene Änderungen erscheinen auch dort ohne Zutun, weil eine abgeschlossene Dateioperation die Auffrischung selbst anstößt und nicht FSEvents. Ein eigener Auffrischungsbefehl entsteht in dieser Runde nicht; die ausgelieferte Belegung kennt keinen.

**Was diese Antwort nicht mit erledigt.** Der beantwortete Defekt trug eine zweite, ungemessene Beobachtung: ein Netzpfad, dessen Server verschwindet, lässt Systemaufrufe hängen statt scheitern, und der Lesefaden prüft sein Abbruchkennzeichen zwischen zwei Aufrufen; je Versuch bliebe ein Faden liegen. Sie betrifft den Zugriff und nicht die Auffrischung und ist deshalb als eigener Defekt abgelegt, statt mit dem beantworteten zu verschwinden: `issues/260805-0000_o_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C9 (Beschreibung, ein neues Abnahmekriterium, eine Festlegung); `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` in `### Frage 3`. Kein neuer Schritt, kein Eingriff am Code.
Implemented: <offen — der Plantext trägt die Einengung; ein Codeeingriff folgt daraus nicht>

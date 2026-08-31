# Bekommt das Ablageprofil eine zweite Umfangszeile, obwohl die Zahl der archivierten Dateien nicht zählbar ist?

---
**Domain:** data
**Filed by:** ontocoder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml` (Profil `fusion-Werkbank: der Ablagespeicher`, und der Absatz „Der Platzhalter in der Ortsangabe"); `crates/krk-core/src/leseprofil/mod.rs` (`Ortsmangel::MehrerePlatzhalter`, `Baustein` mit Festlegung A7); `crates/krk-core/src/leseprofil/bausteine.rs:699` (`Anzeige::Titel => eintrag.typ == Typ::Datei`); `crates/krk-core/src/ablage/leseprofile.rs` (die zwei Proben, die die Zahl zwölf halten); `crates/krk-core/tests/leseprofil.rs` (`ausgelieferte()`)

---

## Question

Der Nutzer hat am 260831 zwei Leseprofile für `fusion-workbench/archive/` und `fusion-workbench/shared/` beauftragt und dabei verlangt, das Ablageprofil solle beide Lesarten von „Einträgen" zeigen, die Archivläufe und die archivierten Dateien. Beide Profile stehen bereits in der Auslieferungsfassung, und die zweite Lesart lässt sich mit dem festgelegten Bausteinsatz nicht zählen. Was das Ablageprofil an ihrer Stelle zeigt, ist offen.

Die Ablage liegt drei Ebenen tief: `archive/<lauf>/shared/<speicher>/<datensatz>.md`. Am Bestand des 260831 sind das fünf Läufe, 167 Dateien darunter und keine einzige Datei unmittelbar in `archive/`. Die Zählung läuft flach über eine Ebene, und eine Ortsangabe nimmt höchstens einen Platzhalter an; `*/shared/*` wäre der zweite und wird beim Laden abgewiesen. Erreichbar sind damit die fünf Läufe über `zaehlung = { }` und, eine Ebene tiefer, die abgelegten Speicherordner über `zaehlung = { ordner = "*/shared" }`, gemessen fünfzehn. Keine dieser Zahlen ist die der archivierten Dateien.

Die Frage hat einen zweiten Teil, der schon entschieden ist und hier nur zur Vollständigkeit steht: `zeigt = "titel"` liefert an diesem Speicher nicht den Ordnernamen mit seinem Zeitstempel, sondern den Platzhalter. Die Titelform sieht allein Einträge vom Typ Datei, und `archive/` enthält unmittelbar keine. Die vorhandene Zeile trägt `zeigt = "datum"`, und der Kommentar am Profil schreibt den Grund aus.

## Options

1. **Es bleibt bei der einen Zeile „Läufe".** Der Kommentar am Profil nennt zusätzlich, warum die Zahl der archivierten Dateien dort nicht steht, damit der nächste Leser die Lücke nicht für ein Versehen hält.
   - Pro: Die Beschriftung „Läufe" behauptet keine Dateizahl, verwechseln kann man sie also nicht. Das Profil bleibt bei einem Leselauf. Keine Probe ändert sich.
   - Contra: Der Nutzer sieht den Umfang der Ablage nicht, und die Frage kommt beim nächsten Blick auf den Ordner wieder.
2. **Eine zweite Zeile über die abgelegten Speicher**, `zaehlung = { ordner = "*/shared" }` unter einer Beschriftung wie „Abgelegte Speicher".
   - Pro: Entscheidbar, wächst mit dem Bestand, kostet einen zweiten Leselauf von zwölf erlaubten und keine Öffnung.
   - Contra: Die Zahl beantwortet eine dritte Frage, die niemand gestellt hat. Fünfzehn abgelegte Speicher neben fünf Läufen und 167 Dateien ist genau die Verwechslungslage, die der Auftrag vermeiden wollte. Die Zeile setzt außerdem voraus, dass jeder Lauf seinen Bestand unter `shared/` ablegt; ein Lauf, der einen Circle mitnimmt, fiele aus der Zählung.
3. **Der Bausteinsatz bekommt eine Tiefenangabe**, etwa `tiefe = 2` an `zaehlung`, und die Zeile zählt die Dateien im Unterbaum.
   - Pro: Beantwortet die gestellte Frage und nicht eine benachbarte.
   - Contra: Ein Eingriff in den Kern und nicht in die Datei. Festlegung A7 hält den Bausteinsatz bei vier, und der Haushalt zählt heute Leseläufe, die im Profil ablesbar sind; ein Unterbaum kostet, was erst am Bestand feststeht. Die Runde 16 hat die flache Zählung ausdrücklich gewählt (C3.2, Festlegung A2).

## Constraints

- Ein neues `[[profil]]` ist mit einer Änderung an dieser Datei allein nicht zu haben: drei Proben halten die Zahl zwölf (`die_eingebettete_fassung_besteht_ihre_eigene_pruefung`, `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt`, `ausgelieferte()`). Wer ein dreizehntes aufnimmt, zieht sie mit, und das ist Arbeit am Prüfcode.
- `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` verlangt, dass keine mitgelieferte Zeile `typ =` oder `versteckt =` trägt. Eine Zählzeile, die auf Dateien einschränkt, bricht die Probe.
- Das Ablageprofil kostet heute einen Leselauf. Von zwölf bleiben elf, die Kostengrenze ist an dieser Stelle nicht knapp.
- Die Auslieferungsfassung ist die alleinige Quelle; `~/Library/Application Support/KRK/readers.toml` gehört dem Nutzer und wird nach ihrer Anlage nie überschrieben. Eine Änderung hier erreicht eine bestehende Nutzerdatei nicht.

## Recommendation

Wir empfehlen Möglichkeit 1, mit dem erklärenden Satz im Kommentar. Der Grund liegt nicht am Aufwand, sondern an der Auskunft: die Zahl aus Möglichkeit 2 steht neben „Läufe" und liest sich wie deren feinere Auflösung, ist es aber nicht, und ein Leser, der 5 und 15 nebeneinander sieht, schließt auf drei abgelegte Datensätze je Lauf statt auf 33. Eine Zeile, die in die falsche Richtung gelesen wird, ist schlechter als keine.

Möglichkeit 3 halten wir für die sachlich richtige Antwort auf die gestellte Frage, aber nicht für diese Runde: sie berührt den Haushalt, dessen Zahlen die Runde 16 aus der Ablesbarkeit der Kosten hergeleitet hat, und die Runde 23 arbeitet am Git-Bereich. Wer sie später aufnimmt, entscheidet zuerst, was ein Unterbaumlauf im Haushalt kostet, und erst danach, wie er in der Datei steht.

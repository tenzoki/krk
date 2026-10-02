# Die Begründung am Kopf von `OHNE_KOMBINATION_AB_WERK` nennt sechs Einträge einer offen gelassenen Wahl; vier davon folgen einer getroffenen

---

Der Doc-Kommentar von `OHNE_KOMBINATION_AB_WERK` (`crates/krk-core/tests/belegung.rs`) sagt im Absatz zu `belegungsdatei_ansehen`: „Die sechs davor folgen einer offen gelassenen Wahl, diese einer getroffenen." Derselbe Kommentar sagt zwei Absätze früher über die Spaltenschalter: „Die Wahl ist eine Nutzerantwort und keine Auslassung." Beide Sätze können nicht zugleich stimmen.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was dasteht und was gilt

Die sechs Einträge vor `belegungsdatei_ansehen` zerfallen in zwei Gruppen mit verschiedener Herleitung:

- Die vier Spaltenschalter (`spalte_groesse_umschalten`, `spalte_datum_umschalten`, `spalte_typ_umschalten`, `spalte_marke_umschalten`) folgen einer **getroffenen** Nutzerantwort: `260812-0306_*_bekommen-die-spaltenschalter-tastenbefehle.md`, Möglichkeit 2, „in der Belegung geführt, ohne ausgelieferte Kombination", begründet mit der Knappheit der 39 frei gewählten Kombinationen. Der Datensatz trägt `## Antwort` ausgeschrieben und eine Korrektur vom 260812-0735.
- Nur `tiefe_suche_umschalten` und `inhaltssuche_umschalten` folgen einer **offen gelassenen** Wahl: der Nutzer hat am 260814-1610 keine der drei vorgeschlagenen Ebenen gewählt (`260814-1552_*_welche-tastenkombination-schaltet-die-tiefe-suche.md`).

Der Satz „die sechs davor" zieht damit vier Einträge auf die falsche Seite der Unterscheidung, die er selbst aufmacht. Richtig wäre „die zwei davor".

## Herkunft

Gefunden beim Zusammenlegen der beiden Listen der ab Werk tastenlosen Funktionen (Nutzerantwort vom 260907 auf `260814-2326_*_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`, Möglichkeit 3). Der Umzug hat die Begründungen der zwei Stellen an einer versammelt und dabei nichts umformuliert; der Widerspruch stand schon vorher da und ist mit dem Zusammenziehen sichtbar geworden.

## Abnahme

Der Absatz zu `belegungsdatei_ansehen` im Doc-Kommentar von `OHNE_KOMBINATION_AB_WERK` unterscheidet die vier Spaltenschalter (getroffene Wahl, Knappheit der Kombinationen) von den zwei Sucheinstellungen (offen gelassene Wahl), statt alle sechs derselben Herleitung zuzuschlagen. Kein Prüflauf hängt daran; die Prüfung ist das Lesen.

---
Resolved: Der Absatz zu `belegungsdatei_ansehen` im Doc-Kommentar von
`OHNE_KOMBINATION_AB_WERK` (`crates/krk-core/tests/belegung.rs`) trennt jetzt die vier
Spaltenschalter, die der getroffenen Antwort vom 260812-0306 folgen, von den zwei
Sucheinstellungen `tiefe_suche_umschalten` und `inhaltssuche_umschalten`, die der am
260814-1610 offen gelassenen Wahl folgen. Der Satz "die sechs davor folgen einer offen
gelassenen Wahl" steht nicht mehr da; der Widerspruch zum Absatz ueber die
Spaltenschalter ist damit weg.

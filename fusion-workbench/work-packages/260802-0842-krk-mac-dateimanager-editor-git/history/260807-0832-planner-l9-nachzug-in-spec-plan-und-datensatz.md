# Nachzug des L9-Entscheids in Spec, Plan und Datensatz

**Datum:** 2026-08-07, 08:32
**Agent:** planner
**Status:** Complete
**Auftrag:** Aufgabe R1 aus Turn 26. Dokumentenpflege nach dem Nutzerentscheid
vom 260807 zur Zusage L9. Kein neuer Plan, kein Code.

## Was der Nutzer entschieden hat

Der Datensatz
`decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`
empfahl Möglichkeit 1, die Ursache im Hauptfaden zu beheben. Der Nutzer hat
Möglichkeit 2 gewählt, die Zusage anzupassen. Neue Fassung: während einer
laufenden Kopie erreicht jede Eingabe spätestens das zweite Bild, und
mindestens 85 Prozent erreichen das erste.

## Nachprüfung an der Abnahmereihe

Nachgerechnet an den 100 Einzelwerten aus
`messungen/260805-2207-MacBookPro15-1-abnahme.txt`, Zeilen 288 bis 313, gegen
eine Bildlänge von 16,667 ms. Die neue Fassung hält in allen fünf Runden, in
beiden Hälften.

| Runde | Anteil im ersten Bild | größter Einzelwert | über zwei Bildlängen |
|---|---|---|---|
| 1 | 90,0 % (18/20) | 19,153 ms | keiner |
| 2 | 85,0 % (17/20) | 20,913 ms | keiner |
| 3 | 90,0 % (18/20) | 23,429 ms | keiner |
| 4 | 100,0 % (20/20) | 15,674 ms | keiner |
| 5 | 85,0 % (17/20) | 18,825 ms | keiner |

Zehn verpasste Eingaben insgesamt, zwischen 17,218 und 23,429 ms. Zwei
Bildlängen sind 33,333 ms; der größte Wert liegt bei 1,41 Bildlängen. Die
Angabe des Datensatzes, die Verfehlungen lägen zwischen 17,2 und 23,4 ms, ist
damit bestätigt.

## Geänderte Dateien

- `planning/260802-1036_*_spec-navigator-geruest.md` — Datums- und Statuszeile,
  neuer Standabsatz im Kopf, C8 mit der Zeile L9, dem Vorspann der
  Abnahmekriterien, der Messbedingung zu den zwanzig Wiederholungen, der
  Messvorschrift und einem erstmals angelegten Abschnitt
  `Getroffene Festlegungen`, dazu ein Absatz in
  `## Offene Nutzerentscheidungen`.
- `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` — Datums- und
  Statuszeile, neuer Nachzugsabsatz, `### Frage 5` mit der Auswertungsregel,
  `### Frage 6` mit dem zweimal überholten Wortlaut der Zusage, S21 mit dem
  Absatz zur Kopplung an L1, S22 mit einer neuen Notiz, dazu ein Eintrag in
  `## Angelegte Defekte und Entscheidungen`. Beide Schritte bleiben abgenommen.
- `decisions/260806-0014_a_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`
  — `Status:` auf `answered`, `Answered:`-Zeile angehängt, Marker `_o_` → `_a_`.
  Nicht `_i_`: das setzt der Orchestrator mit dem Commit.
- `issues/260807-0832_o_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md`
  — neu.

Die Marker von Spec und Plan bleiben `_o_`.

## Befunde

- **Die Messstrecke trägt die neue Fassung nicht.** `Abnahmemass::AnteilImBild`
  in `crates/krk-bench/src/messen.rs:387` führt allein die Bildlänge, der
  geforderte Anteil steht als Kistenkonstante `ANTEIL_IM_BILD_PROZENT` in
  `:67` und gilt für L1 und L9 gemeinsam; eine Obergrenze je Einzelwert kennt
  der Typ nicht. Als Defekt gemeldet, Umsetzung gehört dem `coder`.
- **Der mitgenannte Defekt wandert nicht mit.**
  `issues/260805-2335_c_l1-und-l9-verfehlen-den-anteil-im-ersten-gesamtlauf-unter-fremdlast.md`
  steht seit Turn 21 auf geschlossen, mit einer `Resolved:`-Notiz, die den
  L9-Teil ausdrücklich an den Entscheidungsdatensatz weitergibt. Die Übergabe
  ist mit diesem Entscheid eingelöst; der Marker ist unverändert richtig.
- **`### Frage 6` des Plans zitierte die Zusage in ihrer Fassung von vor dem
  260803-1810** ("keine Eingabe wartet länger als 16 ms"). Die Stelle war seit
  jenem Datum überholt und ist mitgezogen worden, weil sie den Wortlaut von L9
  trägt.
- **Die beiden Reconciliation-Log-Einträge vom 260806-0904 und 260806-1647
  bleiben unverändert.** Sie sind datierte Momentaufnahmen und waren zu ihrem
  Zeitpunkt richtig.

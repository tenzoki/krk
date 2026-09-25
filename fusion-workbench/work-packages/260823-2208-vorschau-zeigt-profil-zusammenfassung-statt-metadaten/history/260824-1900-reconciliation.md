# Abgleich zum Abschluss der Runde 16

**Datum:** 260824-1900
**Domäne:** `code`
**Baumstand:** `83026f6`, Sitzungsanker `278a008`, 25 Commits dazwischen
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Status:** Complete

---

## Was durchgesehen wurde

Zwei Planungsdatensätze (Spec und Plan), zehn Entscheidungsdatensätze dieser Runde,
vierzig Defektdatensätze dieser Runde, vier Durchsichten, dazu die aktive Grundlage aller
Speicher: 41 Entscheidungsdatensätze auf offen oder beantwortet in `shared/decisions` und in
den `decisions/` der achtzehn Runden.

**Geprüft ist gegen den Baum und nicht gegen den Plantext.** Jede der vierzehn behaupteten
Erledigungen ist an der Datei nachgelesen, die sie nennt; die Belege stehen als Tabelle im
`## Reconciliation Log` des Plans, je Schritt mit Commit und Fundstelle.

`make check` läuft grün, in einem Zug gefahren: 1520 Proben in 22 Zielen, keine rot, 11 Kindproben
übersprungen; `clippy` ohne Beanstandung, `fmt --check` ohne Ausgabe.

## Die Zahlen

| | |
|---|---|
| Planschritte auf `[DONE]`, einzeln am Baum belegt | 14 von 14 |
| Abnahmekriterien ohne Fenster belegt | 38 von 56 |
| Abnahmekriterien als Nutzerarbeit | 14 von 56 |
| Abnahmekriterien zur Hälfte oder gar nicht belegt | 4 von 56 |
| Entscheidungsdatensätze der Runde auf umgesetzt | 10 von 10 |
| Defektdatensätze der Runde geschlossen | 34 von 40 |
| Durchsichtsbefunde geräumt | 22 von 23 |
| Gefundene Abweichungen | 12 |
| Davon berichtigt | 6 |
| Davon als Datensatz abgelegt | 5 Defekte und 1 Entscheidung |

## Die sechs berichtigten Abweichungen

1. **Acht Entscheidungsdatensätze standen auf beantwortet, obwohl ihre Umsetzung im Baum steht.**
   Jeder trägt jetzt seine `Implemented:`-Zeile mit Commit und Beleg und den Marker umgesetzt
   (`_i_`). Damit ist die Buchführung geschlossen, die der Plan unter
   `## Welcher Schritt welchen Datensatz realisiert` angekündigt hat.
2. **Der Spec zitierte einen Defektdatensatz unter seinem alten Namen.**
   `shared/issues/260824-2115_o_ein-commit-des-orchestrators-…` ist am 260824-1758 auf
   `260824-1745` umbenannt worden, weil sein Zeitstempel der Uhr voraus lief. Das Zitat zeigte
   seither ins Leere und steht jetzt in der Sternform auf dem richtigen Namen. Es ist der erste
   gemessene Folgeschaden jener Umbenennung; der Datensatz
   `shared/issues/260824-1758_*_die-zeitstempel-…` hält ihn jetzt fest.
3. **Spec und Plan schrieben in vierzig Zitaten den Marker aus** statt der Sternform, und
   mehrere davon waren falsch geworden. Beide tragen jetzt durchweg die Sternform.
4. **Der Satz „Alle acht stehen auf `_a_`"** im Spec war mit diesem Abgleich falsch und sagt
   jetzt, was der Bestand sagt.
5. **Der offene Datensatz zu den Archivspeichern nannte eine überholte Zeilennummer**
   (`resources/default-readers.toml:216`, heute `:237`).
6. **Der Plan stand auf `_o_`, obwohl alle vierzehn Schritte auf `[DONE]` stehen.** Er steht jetzt
   auf `_p_`; warum nicht auf `_c_`, steht unten unter `## Marker`.

## Die fünf abgelegten Defekte

Alle fünf sind vom Typ „die Sache stimmt am Baum, die Zusage ist ungehalten". Keiner hält den
Rundenabschluss auf.

- `issues/260824-1852_*_die-meldung-der-teillesungsprobe-…` — die Fehlermeldung der Probe zur
  Teillesung (`crates/krk-core/tests/leseprofil.rs:1382`) sagt weiter „die Zaehlung sagt, dass es
  mehr sind"; die Berichtigung von C6.5 vom 260824-1722 sagt das Gegenteil. Letzter Rest der
  Fassung „über 2.000", die drei Räumungen aus Spec, Plan, Modulkopf und Kommentarzeilen genommen
  haben.
- `issues/260824-1852_*_c3-14-nennt-seinen-eigenen-nachweis-…` — C3.14 schreibt seinen Prüfweg
  selbst aus („keine neue Stelle im Baum öffnet eine Datei über ihren Pfad"), und nichts im Baum
  führt ihn. Die Sache stimmt heute, gemessen; ungehalten ist sie für die Zukunft.
- `issues/260824-1852_*_zwei-abnahmekriterien-aus-c5-…` — C5.8 und C5.9 sind weder durch eine Probe
  belegt noch stehen sie unter `## Nutzerarbeit`, entgegen der ersten Schlussbedingung des Plans.
  `ausgelieferte()` hat im ganzen Baum genau einen Rufer, die Messung zu C6.7.
- `issues/260824-1852_*_die-probe-zu-c5-10-…` — die Probe zu C5.10 sucht die vier Bausteinnamen im
  ganzen `AUSLIEFERUNGSTEXT` statt in seinen Kommentarzeilen und misst die Hälfte des Kriteriums
  („je einen an einem Beispiel zeigen") nicht.
- `issues/260824-1852_*_zwei-aussagen-in-claude-md-…` — `CLAUDE.md:146` sagt „Die Hülle hat zwei
  Aufrufer", es sind seit Schritt 4 drei; `CLAUDE.md:120` zählt auf, was KRK sich merkt, und nennt
  die Leseprofile nicht. Der Plan hatte beide in seiner Risikotabelle stehen; ein Eintrag dort ist
  kein Datensatz.

## Der abgelegte Entscheidungsdatensatz

`decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-…`

Der Spec sagt, die Runde falle in die Endbedingung von L7 und schulde denselben späteren Messlauf
wie die Runde 14. **Ein solcher Lauf könnte die Arbeit dieser Runde nicht messen**, aus zwei
voneinander unabhängigen Gründen: die Sitzungsstrecke wählt für L7 eine Datei
(`crates/krk-ui/src/messmodus.rs:832`) und die Zusammenfassung entsteht nur für einen Eintrag, der
keine Datei ist; und der Messmodus lädt die Ablage nicht, also bleibt der Profilsatz leer und ein
Ordner zeigt seine Metadaten wie bis zur Runde 15 (`crates/krk-ui/src/appkit/anwendung.rs:1446`).
Die Runde 14 lag anders: ihre Arbeit steckt im Renderweg einer Textdatei, also genau in dem Weg,
den L7 misst. Der Datensatz legt drei Möglichkeiten vor und empfiehlt die zweite.

## Misfiled — should be a decision

Keiner. Die vierzig Defektdatensätze dieser Runde sind sämtlich Defekte; keiner ist eine
Entscheidung in falschem Speicher.

## Was offen bleibt und offen bleiben soll

- `issues/260824-1655_*_sechs-speicher-unter-archive-…` — Nutzerentscheidung, kein Mangel. Die
  Beschreibung ist gegen alle 154 Verzeichnisse unter `fusion-workbench/` nachgemessen und stimmt
  unverändert: 16 ohne Profil, davon zehn keine Speicher und sechs Speicher unter `archive/`.
- `shared/issues/260824-1745_*_ein-commit-des-orchestrators-…` — stimmt Stelle für Stelle.
  `git show --name-status 79209c8` führt acht Dateien: die geänderte Quelldatei, die neue
  Verlaufsdatei und sechs Umbenennungen, davon fünf fremde.
- `shared/issues/260824-1758_*_die-zeitstempel-…` — stimmt; die sechs Verlaufsdateien tragen ihre
  vorauslaufenden Stempel weiter, die zwei berichtigten Stellen halten.

**Dieser Abgleich hat aus dem zweiten eine Vorkehrung gezogen** und die zehn Markerwechsel mit
`mv` statt `git mv` gefahren, damit der Index unberührt bleibt.

## Marker

- Plan: `_o_` → `_p_`. Alle vierzehn Schritte stehen auf `[DONE]` und sind einzeln am Baum
  gelesen. Auf `_c_` geht er nicht: sein `## Nutzerarbeit` führt sieben Kriterien mit Bündelanteil,
  sein Abhängigkeitsgraph endet auf dem Knoten „Nutzerarbeit am laufenden Bündel", und dieser
  Durchgang ist nicht gefahren. Dasselbe Vorgehen hat der Abgleich der Runde 14 gewählt.
- Spec: bleibt `_o_`. Was `_c_` an einem Spec heißt, ist offen
  (`shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-…`), und fünf der bisher
  geschlossenen Runden lassen ihren Spec ebenfalls auf `_o_`.
- Circle-Datensatz: bleibt `_t_`. Die Schließung ist Sache des Orchestrators, nicht dieses
  Abgleichs.

## Ergebnis

**Dem Rundenabschluss steht nichts entgegen.** Die Runde ist gebaut, ihre abzählbaren Grenzen sind
ohne Fenster belegt, und jede der vierzehn Erledigungen hält am Baum. Was aussteht, ist die
Nutzerarbeit am laufenden Bündel; ohne sie schließt die Runde beschränkt (`_b_`) und nicht
kohärent, wie ihr eigener Plan es unter `## Where this Circle stops` schon sagt.

Der Dreikantenbefund steht im `## Coherence`-Abschnitt von
`history/260824-0530-orchestrator-session.md`.

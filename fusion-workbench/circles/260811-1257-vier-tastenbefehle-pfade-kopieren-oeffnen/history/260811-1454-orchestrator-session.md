# Orchestrator Session — 260811-1454

**Directive:** Nach dieser Runde legt KRK auf Tastendruck zwei Sorten von Pfaden in die
Zwischenablage — den des angezeigten Ordners im aktiven Dateifenster und den des betroffenen
Eintrags. Eine Datei geht per Doppelklick und per Tastenkombination an das Standardprogramm des
Systems, und Cmd+W schließt den aktiven Tab auch dann, wenn der Fokus nicht in einem Bereich mit
Tabs steht. Alle vier Befehle laufen über die vorhandene Kommando-Maschinerie und über keine
zweite daneben.
**Mode:** (wird in Phase 0 aufgelöst)
**Status:** In Arbeit

## Setup

Gelaufen als `/fusion:setup` nach der Aktivierung des Circles über `/fusion:next`.

- Layout-Prüfung vor v4: `OLD=0`. Keine Migration nötig.
- Setup-Marke geschrieben, Plugin-Version 7.2.0. Monitor neu kopiert.
- Nebenläufigkeit: `none`, frische Marke geschrieben.
- Keine `agentstate.yaml` — frischer Start, keine unterbrochene Sitzung.
- Stilprofile, Plane-Vorlage und `fusion-guard.json` waren vorhanden.

## Aufgelöste Pfade

`fusion-paths orchestrator`, Exit 0. Der Circle ist aktiv, also zeigen alle `OUT_*` hinein und
jedes `SCAN_*` deckt Circle und gemeinsamen Speicher ab:

```
CIRCLE=circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen
OUT_PLAN / OUT_HISTORY / OUT_ISSUE / OUT_DECISION → circles/260811-1257-…/{planning,history,issues,decisions}
SCAN_* → circles/260811-1257-…/<art> und shared/<art>
```

## Momentaufnahme

**Git:** HEAD `55a4afa`.

**Bereich (Domain): `code`.** `bin/fusion-count-sources` zählt mit `git ls-files` 115 Codedateien
gegen 11 Datendateien.

**Arbeitsschlange:** keine am Wurzelort. Die vorige ist am 260811-1420 als abgearbeitet
zurückgezogen worden (`shared/planning/260811-1420_c_abgearbeitete-warteschlange-…`). Phase 1 baut
eine neue.

**Offene Arbeit:**

| Art | Zahl |
|---|---|
| Offene Defekte (Circle + gemeinsam) | 7 |
| Offene Pläne oder Specs | 0 |
| Offene Fragen im Circle | 4 |
| Offene Fragen gemeinsam | 2 |

Die vier Fragen des Circles sind **Zuschnittfragen an den Nutzer** und keine Untersuchungen. Sie
kommen vor jeder Planung: wie weit Cmd+W reichen soll, was der Pfadkopierer bei stehender
Markierung kopiert, was ein Doppelklick auf einen Ordner tut, und welche vier Kombinationen ab
Werk gelten.

Die sieben offenen Defekte gehören keinem dieser Punkte auf: fünf betreffen fusion selbst
(Aufgabenereignisse, Durchsichtsdokument, Circle-Kopffelder, `portfolio.md`-Erzeugung, die
Warteschlangen-Prüfung), zwei betreffen KRK (die `must_use`-Frage am `Auswahlversuch`, die
Vorschaubreite beim Navigieren — letztere gehört sachlich zum vorgesehenen Statusleisten-Circle).

**Wächter:** `haltActive: false`.

**Circles:** 1 aktiv, 2 vorgesehen, 3 beschränkt abgeschlossen.

**Häufig geänderte Dateien:** `crates/krk-ui/src/appkit/anwendung.rs` (147) und
`crates/krk-ui/src/appkit/editor.rs` (137) führen die Rangliste. Die erste ist für diese Runde
einschlägig — die vier Befehle hängen am Anwendungsdelegierten.

## Ein Befund des playmaker, der diese Runde unmittelbar betrifft

Der Portfolio-Lauf vom 260811-1415 hat festgehalten: **die Markierung fällt heute mit jedem
Lesevorgang**, weil sie eine Menge von Eintragsindizes ist. Der Pfadkopierer für den „betroffenen
Eintrag" setzt genau darauf auf, und die Frage `260811-1258_o_was-kopiert-der-pfadkopierer-bei-stehender-markierung.md`
hängt daran. Das gehört vor der Antwort geprüft und nicht angenommen.

## Verlauf

- 260811-1454 — Setup abgeschlossen. Vier Nutzerfragen stehen vor der Planung.

## Drei Defektdatensätze über fusion sind gelöscht, weil übertragen

Am 260811-1950 auf Weisung des Nutzers: die drei Befunde über das fusion-Plugin sind in dessen
eigenen Arbeitsbereich übertragen und hier gelöscht.

| Gelöscht | Gegenstand |
|---|---|
| `260811-0932_o_die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md` | Beim `_a_`→`_t_` bleiben `Status:`, `Active spec/plan:` und `Active session history:` stehen; niemandes Prompt beauftragt sie |
| `260811-1425_o_die-pruefung-der-warteschlange-liest-einen-circle-pfad-aus-der-prosa-ihrer-kopfzeile.md` | „Reading a queue" nimmt den ersten `circles/`-Treffer der Zeile, auch aus einem Nebensatz, und meldete `STALE` für eine Warteschlange, die keinen Circle nennt |
| `260810-1730_o_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md` | Weder die Vorlage noch die Anweisung des `playmaker` verlangen die Sternform; jede Handkorrektur an `portfolio.md` ist ein Aufschub |

**Die Wahl war Löschen und nicht Schließen**, und der Unterschied ist folgenreich genug, um ihn
festzuhalten. Ein geschlossener Datensatz bliebe hier auffindbar und zeigte auf den Ort der
Behebung; ein gelöschter tut das nicht. Wer in diesem Projekt später auf dieselbe Lage stößt —
etwa auf ein Kopffeld, das dem Marker widerspricht —, findet im Arbeitsbereich keinen Hinweis
darauf, dass der Befund schon erhoben und weitergegeben wurde.

**Der volle Text der drei bleibt in der Git-Historie** und ist über
`git log --diff-filter=D --name-only` oder `git show <commit>^:<pfad>` erreichbar; dieser
Abschnitt nennt die drei Dateinamen, damit die Suche danach möglich ist.

**Zwei weitere Datensätze betreffen fusion und sind ausdrücklich geblieben**, weil sie eine
andere Sorte sind: `shared/issues/260810-1907_*_die-durchsicht-von-turn-2-hat-kein-durchsichtsdokument-hinterlassen.md`
und `shared/issues/260810-1945_*_der-orchestrator-hat-in-drei-turns-keine-aufgabenereignisse-emittiert.md`
sind Befunde über das **Verhalten des Orchestrators in dieser Sitzung** und keine Fehler im
Prompt-Text des Plugins. Sie halten fest, was hier geschehen ist, und gehören deshalb hierher.
Wer sie ebenfalls für übertragbar hält, sagt es.

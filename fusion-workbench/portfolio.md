# Portfolio

**Generated:** 260802-0853 (by playmaker session 260802-0853-playmaker-direct-dispatch)
**Domain bias:** code

## Active (_t_)

(keiner)

Kein Circle trägt die Marke `_t_`, und `fusion-workbench/.active-circle` fehlt. Beides zusammen ist der reguläre Zustand vor der ersten Aktivierung, kein Fehler.

## Anticipated (_a_) — ranked

**Recommended next:** `260802-0842-krk-mac-dateimanager-editor-git` — einziger anticipated Circle, ohne Vorgänger-Abhängigkeiten, aber mit vier offenen Entscheidungen, die der Aktivierungs-Spec zuerst klären muss.

### 1. `260802-0842-krk-mac-dateimanager-editor-git`

**Directive:** KRK, ein nativer macOS-Dateimanager mit zwei Dateifenstern, eingebautem Editor und eingebautem Git, vollständig über die Tastatur bedienbar.
**Dependencies:** keine.
**Offene Entscheidungen im Grounding:** 4 bindende, 1 ausdrücklich nicht bindende.

Der Circle ist der empfohlene Kandidat, allerdings ohne Vergleich: er ist der einzige anticipated Circle, und eine Rangfolge mit einem Element sagt nichts über relative Reife. Die Empfehlung stützt sich auf zwei absolute Signale. Erstens nennt `## Dependencies` keinen Vorgänger, es ist also kein Abschluss abzuwarten. Zweitens ist die Grundlage vollständig dokumentiert: der Shaper hat am 260802-0842 eine Klärungsrunde geführt, deren Antworten zu Umfang, Bedienmodell und Laufwerken im Grounding stehen, und hat zwei Abgrenzungen bewusst nach draußen gelegt, den Datei- und Ordnervergleich sowie das Suchen und Ersetzen über mehrere Dateien.

Dagegen steht der Wert, den die Domain-Gewichtung `code` gerade abstraft: vier offene Entscheidungsdatensätze binden diesen Circle. `shared/decisions/260802-0842_o_f-tasten-unter-macos-systembelegung.md` klärt, wie KRK die von macOS belegten Tasten F3 bis F8 überhaupt erreicht. `shared/decisions/260802-0842_o_loeschen-papierkorb-oder-endgueltig.md` entscheidet, ob Shift+Delete in den Papierkorb oder endgültig löscht, und trägt damit ein Datenverlustrisiko. `shared/decisions/260802-0842_o_git-verwerfen-bedeutung.md` legt fest, welche der beiden Git-Bedeutungen von "revert" gemeint ist. `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` bestimmt, was die Formatansicht bei Code und einfachem Text zeigt. Der fünfte im Grounding zitierte Datensatz, `shared/decisions/260802-0842_o_code-sdk-fuer-ki-integration.md`, hält seine eigene Nichtbindung fest und zählt nicht mit.

Die Aktivierung ist dadurch nicht blockiert, ihr erster Schritt aber vorgezeichnet: der Shaper im portfolio-activation-Modus klärt die vier Fragen mit dir, bevor ein Plan entsteht. Der Grounding-Abschnitt nennt zusätzlich eine Lücke, die kein Entscheidungsdatensatz abdeckt. Die Maxime "superschnell" trägt bisher keine messbaren Abnahmekriterien, etwa eine Zeitvorgabe für die Anzeige eines Verzeichnisses mit zehntausend Einträgen oder eine Dateigröße, ab der der Editor die Ladestrategie wechselt.

## Recently closed (_c_ / _b_)

(keine)

Noch kein Circle wurde geschlossen. Das Projekt hat zum Zeitpunkt dieses Laufs keinen Commit und keinen Quelltext.

## Archived (_s_ / _d_)

(keine)

## Warnings

- `activation-blocked-on-decisions: 260802-0842-krk-mac-dateimanager-editor-git` — vier offene Entscheidungsdatensätze binden den empfohlenen Circle und sind laut seinem eigenen Grounding vor dem Aktivierungs-Spec zu beantworten: `shared/decisions/260802-0842_o_f-tasten-unter-macos-systembelegung.md`, `shared/decisions/260802-0842_o_loeschen-papierkorb-oder-endgueltig.md`, `shared/decisions/260802-0842_o_git-verwerfen-bedeutung.md`, `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md`.
- `project-language-undeclared` — im Projektwurzelverzeichnis fehlt `CLAUDE.md` und damit die Zeile `**Language:** de`. `bin/fusion-rules playmaker` hat deshalb die englischen Stilprofile ausgegeben, obwohl der Circle-Datensatz, alle Entscheidungsdatensätze und `idea.txt` deutsch sind. Dieser Lauf hat ersatzweise `fusion-workbench/stilwerk/default-voice-de.yaml` und `chat-voice-de.yaml` angewendet. Bereits als Befund abgelegt in `shared/issues/260802-0842_o_projektsprache-nicht-deklariert.md`.
- `circle-record-template-incomplete: 260802-0842-krk-mac-dateimanager-editor-git` — dem Datensatz fehlt der Abschnitt `## Closure note` aus der Vorlage in `rules/fusion-workbench-conventions.md`. Ohne Folgen, solange der Circle anticipated ist; der Orchestrator muss den Abschnitt beim Schließen anlegen.

Keine Abhängigkeitszyklen: der einzige nicht-terminale Circle nennt keine Vorgänger, der Graph hat eine Kante weniger als einen Zyklus braucht. Keine veralteten Eltern-Groundings: kein Circle trägt die Marke `_b_` (bounded closure), es gibt also nichts zu propagieren.

---

## Details

| Marke | Bedeutung | Anzahl |
|---|---|---|
| `_a_` | anticipated | 1 |
| `_t_` | aktiv | 0 |
| `_c_` | geschlossen-kohärent | 0 |
| `_b_` | bounded closure | 0 |
| `_s_` | superseded | 0 |
| `_d_` | deferred | 0 |

Weitere Speicherstände: 5 offene Entscheidungen (alle in `shared/decisions/`), 1 offener Befund (`shared/issues/`), 0 Pläne, 0 Analysen, 0 Reviews. `tasklist.md` existiert nicht.

Aktivierungsvorschlag im Datensatz: `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`, Abschnitt `## Activation proposal`.
Lauf-Protokoll: `shared/history/260802-0853-playmaker-direct-dispatch.md`.

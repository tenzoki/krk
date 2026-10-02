# 260813-1818 — coder — die .gitignore trennt Live-Zustand von Aufzeichnungen

**Status:** Complete
**Auftrag:** die `.gitignore` mit den fusion-Konventionen in Übereinstimmung bringen; Konventionsweg (Möglichkeit 1) vom Nutzer am 260813 gewählt.
**Datensatz:** `shared/issues/260813-1515_c_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md`
**Regel:** `rules/fusion-workbench-conventions.md`, `## fusion-workbench Layout` → `### Which of them a tracked workbench tracks`

## Was getan ist

1. `.gitignore` — der Werkbankabschnitt trägt jetzt den vollen Live-Zustand der Regel: `agentstate.yaml`, `orchestrator-live.md`, `.active-circle`, `.commit-lock/`, `.session-marker`, `monitor`, `.fusion-setup`, `orchestrator-events.jsonl.tmp` und `.guard-state/*` mit `!.guard-state/events.jsonl`. Der Kommentar nennt das Unterscheidungsmerkmal der Regel und den Abschnitt, in dem es steht.
2. Zehn Dateien mit `git rm --cached` aus dem Index, alle unverändert auf der Platte. Der Aufruf lief unter `fusion-commit-lock`, weil er den gemeinsamen Index anfasst.
3. Der Defektdatensatz trägt einen `Resolved:`-Abschnitt und den Marker `_c_`.

## Zwei Einordnungen, die die Regel offenlässt

`.fusion-setup` steht in keiner der beiden Gruppen der Regel. Eingeordnet nach dem Merkmal, das die Regel ihrer Zweiteilung voranstellt — beantwortet eine frühere Fassung noch etwas —, ist es Live-Zustand: bei jedem `/fusion:setup` an Ort und Stelle neu geschrieben, die vorige Fassung ohne Auskunft. Ausgeschlossen. Die Lücke gehört in die Regel gemeldet.

`orchestrator-events.jsonl.tmp` ist der Schreibrest eines atomaren Schreibens, mit `4e86c02` versehentlich hereingekommen, 0 Bytes. Ausgeschlossen. Es war der stillste der zehn: hätte ein atomares Schreiben ihn einmal wegbenannt, hätte er als *gelöscht* gemeldet.

## Der Rest

`.guard-state/events.jsonl` ist eine Aufzeichnung, bleibt verfolgt, wächst bei jedem Werkzeugaufruf. Die Auslieferungsprüfung schlägt also weiter an, mit einer Datei statt mit acht. Bewusste Folge des Weges. Die Konvention kennt auch dafür einen Ausweg — das lebende Protokoll unverfolgt lassen und die Beweise über die Rollkopien von `/fusion:archive` halten —, aber das ist eine Projektentscheidung und verlangt die Zusage, dass `/fusion:archive` läuft. Im Datensatz festgehalten, nicht getan.

## Prüfung

`git check-ignore -v` über vierzehn auszuschließende Pfade (alle getroffen, jeder mit der erwarteten Zeile) und fünf zu haltende (`\.guard-state/events.jsonl`, `orchestrator-events.jsonl`, `tasklist.md`, `portfolio.md`, `plane.config.yaml` — alle Exit 1, also nicht ignoriert). Exit 0.

Kein Quelltext berührt, kein Bau gefahren.

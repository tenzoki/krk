Die Auslieferungsprüfung schlägt nach jeder Agentensitzung an, weil vier Werkbankdateien verfolgt sind

---

Station 1 von `cargo xtask release`, gebaut in der achten Runde, bricht ab, sobald `git status --porcelain --untracked-files=no` eine Zeile meldet. Der Plan hat den Verzicht auf einen Pfadfilter ausdrücklich begründet: eine Liste der bauwirksamen Ordner müsste jemand pflegen, und sie zu ergänzen zu vergessen ist die zweite Art, eine Prüfung im Vorbeigehen zu verlieren.

Diese Begründung trägt, und trotzdem entsteht ein Nebeneffekt, den niemand gewollt hat. Vier Dateien unter `fusion-workbench/` sind in diesem Projekt versioniert, die die fusion-Konventionen als flüchtigen Sitzungszustand führen: `monitor`, `.fusion-setup`, `.guard-state/churn.json` und `orchestrator-live.md`. Jeder Agentenlauf schreibt sie neu. Nach jeder Sitzung meldet `git status` sie also als geändert, und die Auslieferungsprüfung weist den Lauf ab — aus einem Grund, der mit dem Bündel nichts zu tun hat.

---

**Gefunden am** 260813-1510 vom playmaker, nach dem Abschluss der achten Runde.

**Der Widerspruch im Einzelnen.** `rules/fusion-workbench-conventions.md`, Abschnitt `## Which of them a tracked workbench tracks`, teilt die wurzelverankerten Flächen in zwei Gruppen: Aufzeichnungen, die ein tracked workbench versioniert, und Live-Zustand, den er nicht versionieren soll. `orchestrator-live.md`, `.guard-state/` ohne `events.jsonl` und der `monitor` stehen ausdrücklich in der zweiten Gruppe. Dieses Projekt versioniert sie trotzdem; die `.gitignore` schließt allein `fusion-workbench/.commit-lock/` und `fusion-workbench/.session-marker` aus.

**Drei Wege, und sie haben verschiedene Reichweiten.**

1. **Die vier Dateien in die `.gitignore` aufnehmen.** Beseitigt die Ursache dort, wo sie sitzt, und bringt das Projekt mit den Konventionen in Übereinstimmung. Kosten: `git rm --cached` für vier verfolgte Dateien, und ihre Historie endet an dieser Stelle. Nichts davon beantwortet eine Frage, die später jemand stellt.
2. **Einen Pfadfilter an `git status` hängen.** Genau das, was der Plan mit Begründung verworfen hat. Die Begründung gilt weiter.
3. **Nichts tun und die Lage kennen.** Wer ausliefern will, fährt vorher `git stash` oder trägt die Werkbank ein. Das ist der Zustand von heute; er kostet bei jeder Auslieferung einen Handgriff und eine Erinnerung.

**Der Weg 1 ist keine Entscheidung des Bauwerkzeugs, sondern eine des Projekts** über den Umgang mit der Werkbank. Deshalb steht dieser Datensatz im gemeinsamen Speicher und nicht im Circle der achten Runde.

**Verwandt:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_c_plan-titelleiste-fuehrt-version-und-semantische-tags.md`, Schritt D2, der den Verzicht auf den Pfadfilter begründet; `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_i_reicht-ein-tag-auf-head-oder-muss-der-arbeitsbaum-sauber-sein.md`, die Nutzerantwort, aus der die Prüfung folgt.

---

**Resolved:** 260813-1818 — der Nutzer hat den **Konventionsweg** gewählt, also Möglichkeit 1. Die `.gitignore` trägt jetzt die Trennung, die `rules/fusion-workbench-conventions.md` unter `### Which of them a tracked workbench tracks` festlegt, und nennt diesen Abschnitt im Kommentar als ihre Quelle.

**Aus dem Index gegangen sind zehn Dateien**, alle mit `git rm --cached`, also unverändert auf der Platte:

- `fusion-workbench/.fusion-setup`
- `fusion-workbench/monitor`
- `fusion-workbench/orchestrator-live.md`
- `fusion-workbench/orchestrator-events.jsonl.tmp`
- `fusion-workbench/.guard-state/` — `churn.json`, `cross-file.json`, `escalation.json`, `review-coverage.json`, `staging-drift.json`, `state-drift.json`

Es sind zehn und nicht die vier des Befundes, weil der Befund nur zählte, was an jenem Tag als geändert gemeldet war. Drei weitere Zählerspeicher unter `.guard-state/` und der Schreibrest `orchestrator-events.jsonl.tmp` waren ebenso verfolgt und hätten dieselbe Meldung bei nächster Gelegenheit erzeugt; der Schreibrest sogar als *gelöscht*, sobald ein atomares Schreiben ihn einmal wegbenennt.

**Zwei Punkte, die die Regel nicht entscheidet, und wie sie hier entschieden sind.**

`.fusion-setup` steht in der Aufzählung der Regel in keiner der beiden Gruppen — es ist im Layoutbaum aufgeführt, aber weder unter „Records" noch unter „Live state". Eingeordnet ist es hier nach dem Merkmal, das die Regel ihrer Zweiteilung selbst voranstellt: ob eine frühere Fassung noch etwas beantwortet. Der Marker trägt Zeitstempel und Plugin-Version, wird bei jedem `/fusion:setup` an Ort und Stelle neu geschrieben, und die vorige Fassung beantwortet nichts, was die aktuelle nicht beantwortet. Also Live-Zustand, also ausgeschlossen. **Das ist eine Lücke der Konvention und keine Auslegung dieses Projekts** — wer die Regel pflegt, sollte den Marker dort namentlich einordnen, damit die nächste Werkbank nicht dieselbe Ableitung noch einmal führen muss.

`orchestrator-events.jsonl.tmp` nennt die Regel ebenfalls nicht; sie nennt nur das Protokoll selbst. Der `.tmp`-Nachbar ist der Schreibrest eines atomaren Schreibens, keine Aufzeichnung, und mit `4e86c02` versehentlich hereingekommen.

**Was als Rest bleibt, und zwar bewusst.** `.guard-state/events.jsonl` ist nach der Regel eine Aufzeichnung und bleibt verfolgt. Es wächst bei **jedem** Werkzeugaufruf. Der Arbeitsbaum ist nach einer Agentensitzung also weiterhin nicht sauber, und Station 1 von `cargo xtask release` schlägt weiter an — nur mit einer Datei statt mit acht. Das ist die Folge dieses Weges und kein Versehen: der Weg beseitigt den Widerspruch zur Konvention, nicht die Prüfmeldung.

**Die Konvention kennt einen Weg auch für diese eine Datei, und er ist eine Projektentscheidung.** Derselbe Abschnitt hält fest, dass nicht das Verfolgen der lebenden Datei die Beweise sichert, sondern `/fusion:archive`: es rollt das Protokoll unter datiertem Namen in den Archivspeicher und legt ein leeres neues an, und die gerollten Kopien sind gewöhnliche archivierte Dateien. Ein Projekt darf das lebende Protokoll deshalb unverfolgt lassen und die Aufzeichnung trotzdem halten — die Regel nennt das ausdrücklich die Einrichtung, die das fusion-Repository selbst fährt. Wer den Arbeitsbaum nach einer Sitzung sauber haben will, hat hier den Hebel; er verlangt aber die Zusage, dass `/fusion:archive` regelmäßig läuft, und diese Zusage gibt dieser Datensatz nicht.

**Geprüft** mit `git check-ignore -v` über vierzehn auszuschließende und fünf zu haltende Pfade, nicht angenommen. Die Wiederaufnahme steht als `fusion-workbench/.guard-state/*` mit `!fusion-workbench/.guard-state/events.jsonl` und nicht als ausgeschlossenes Verzeichnis, weil ein `!` unter einem ausgeschlossenen Elternverzeichnis wirkungslos bliebe.

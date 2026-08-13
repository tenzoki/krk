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

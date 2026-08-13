# Orchestrator-Sitzung — 260813-1006

**Directive:** Die Titelleiste von KRK trägt links einen eigenen Bereich mit Namen und Version (`KRK 0.1.0`), der absolute Pfad bleibt mittig und ungekürzt. Verbindlich wird die Zahl durch semantische Versionstags: Git-Tag `v<version>` je Auslieferung, ein Abschnitt in README.md über die Stufen, Abbruch in `cargo xtask release` ohne passenden Tag auf HEAD. Den Tag setzt der Nutzer.
**Modus:** (Phase 0 offen)
**Status:** Läuft

## Aufnahme beim Start

| Größe | Wert |
|---|---|
| Aktiver Circle | 260813-0939-titelleiste-fuehrt-version-und-semantische-tags (aktiviert 10:0x über /fusion:next) |
| git HEAD | 9d5fcfa |
| Turn-Budget | 5 |
| Erkannte Domäne | code |
| Offene Fragen im Circle | 3 (Über-KRK-Menüeintrag, wer setzt v0.1.0, Tag auf HEAD oder sauberer Baum) |
| Offene Fragen shared | 7 |
| Offene Defekte | 0 im Circle, 9 in shared |
| Offene Pläne | 0 im Circle, 1 in shared |
| Guard | haltActive: false |
| Arbeitswarteschlange | keine tasklist.md |

## Vorlauf dieser Sitzung

Die vorige Sitzung (shared/history/260813-0807-orchestrator-session.md) hat Setup gefahren, den Backlog-Eintrag 260813-0822 angelegt, den playmaker zweimal laufen lassen und über /fusion:direct diesen Circle anlegen lassen. Sie hat keinen Turn gefahren und keinen Commit gesetzt.

## Drei Fragen beantwortet (Nutzer, 260813-1010)

**Über-KRK-Eintrag im Anwendungsmenü: ja, Möglichkeit 2** — der Standard-Über-Dialog von AppKit. Ein Menüeintrag ohne Kürzel öffnet das Systemfenster, das Name, Version und Symbol aus der `Info.plist` des Bündels liest. Damit bleibt die Zahl einquellig, der Eintrag bleibt ein Sonderposten wie die Markdown-Ausgabe der Runde 3, und `Kommando` wächst nicht. Ein eigenes Über-Fenster ist verworfen.

**Erster Tag `v0.1.0`: Möglichkeit 1** — der Nutzer setzt ihn auf den Commit, der diese Runde schließt. Der Abschnitt in `README.md` sagt dazu, dass `v0.1.0` den ersten getaggten Stand benennt und keine Weitergabe. Damit ist die neue Prüfung in ihrer eigenen Runde einmal am grünen Fall gefahren und nicht nur am Abbruch. Rückwirkende Tags für die sieben geschlossenen Runden sind verworfen.

**Prüftiefe von `cargo xtask release`: Möglichkeit 2, beschränkt auf verfolgte Dateien** — der Lauf bricht ab, wenn HEAD keinen zur `Cargo.toml` passenden Tag trägt, und ebenso, wenn `git status` Änderungen an verfolgten Dateien meldet. Unbeachtete Dateien bleiben außen vor. `cargo xtask bundle` und `make check` bleiben unangetastet.

## Spec-Tor und die vierte Frage (Nutzer, 260813-1055)

**Spec freigegeben.** `planning/260813-1037_o_spec-titelleiste-fuehrt-version-und-semantische-tags.md`, sechs Fähigkeiten mit 59 Abnahmekriterien. Der conceptrev hat beide Diagramme gerendert und mit `acceptable` bewertet (0 Zyklen, kein Gott-Knoten, kein freistehender Knoten); die drei mittleren Befunde betreffen Beschriftungen und sind an Ort und Stelle zu beheben. Bericht: `reviews/260813-1049-conceptrev-spec-titelleiste-fuehrt-version-und-semantische-tags.md`.

**Blinder Fleck hinter dem Über-Dialog: Möglichkeit 2** — die Runde schließt die Lücke einmal und allgemein. Die Zulässigkeitsregel (`zulaessigkeit::zulaessig`, seit der Runde 7 eine reine Funktion mit drei Fragern) bekommt die zusätzliche Frage, ob das Schlüsselfenster KRKs Hauptfenster oder ein daran hängendes Blatt ist; ist es keines von beidem, wirkt kein Befehl. Der offene Defekt zum Freigabedialog der Runde 6 fällt damit mit weg. Der Nutzer nimmt die Abnahme in den ungemessenen Lagen auf sich.

## Die fünfte Frage: die Ausnahmeliste (Nutzer, 260813-1125)

**Möglichkeit 1** — die Ausnahmeliste `immer_erreichbar` hebt auch die neue Schlüsselfensterfrage auf. `beenden` und `fenster_schliessen` kommen weiter durch, solange der Über-Dialog oder der Freigabewähler vorn steht. Der Grund ist die ausgeschriebene Randbedingung des Spec, kein Verlust gegenüber heute: Cmd+Q beendet KRK heute auch vor dem Freigabewähler der Runde 6. Die Ausnahmeliste behält damit eine Bedeutung, die in einen Satz passt — sie hebt jede Sperre auf, die nach der Lage fragt, und keine, die nach dem Wirkungsbereich fragt. Cmd+W auf `tab_schliessen` steht nicht auf der Liste und bleibt vor einem fremden Schlüsselfenster gesperrt.

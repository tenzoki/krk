# Orchestrator-Sitzung — 260813-0807

**Directive:** (noch nicht gesetzt — /fusion:setup ohne Auftrag gestartet)
**Modus:** (noch nicht aufgelöst)
**Status:** Läuft

## Aufnahme beim Start

| Größe | Wert |
|---|---|
| Arbeitsverzeichnis | /Users/k1/Projects/productive/krk |
| Plugin-Version | 8.1.0 |
| git HEAD | 9d5fcfa |
| Turn-Budget | 5 (aus fusion-turn-budget) |
| Erkannte Domäne | code (128 Quelldateien, 11 Datendateien, gezählt über git-ls-files) |
| Offene Defekte (shared/issues, _o_/_p_) | 9 |
| Offene Pläne (shared/planning, _o_/_p_) | 1 |
| Offene Fragen (shared/decisions, _o_) | 7 |
| Circles | 1 vorgesehen (_a_), 7 beschränkt geschlossen (_b_), 0 aktiv |
| Aktiver Circle | keiner (.active-circle fehlt) |
| Arbeitswarteschlange | keine tasklist.md an der Wurzel |
| Guard | haltActive: false; die zehn Ereignisse im Eskalationsspeicher stammen vom 06./07.08. aus der inzwischen entfernten protected-path-Prüfung |
| Circle-Hinweis | ausgegeben (1 vorgesehener Circle) |

## Stilprofile

chat-voice-de.yaml und default-voice-de.yaml sind vorhanden und geladen; kein Rückfall auf die englische Variante.

## Hoher Durchsatz (fusion-churn-rank)

931 Einträge, davon 443 auf nicht mehr vorhandene Pfade und 2 auf Werkbank-Oberflächen. Die zehn gewichteten Spitzen führt crates/krk-ui/src/appkit/anwendung.rs (Score 189, 496 Änderungen) vor fenstermodell.rs, appkit/editor.rs und appkit/tabelle.rs an. CLAUDE.md steht mit Score 61 auf Rang 5.

## Befund beim Start

CLAUDE.md beschreibt vier gefahrene Runden. Auf der Platte liegen sieben beschränkt geschlossene Circles; die Runden 5 bis 7 (260811-1304 Statusleiste, 260812-1000 Teilen/Ordnersprung/Ablage/Vorschau, 260813-0100 Suche in der Belegung/Menü/weitere Instanz) fehlen in der Datei. Die Aufstellung der vorgesehenen Circles nennt zwei, vorhanden ist einer (260804-0933 Web-Betrachter).

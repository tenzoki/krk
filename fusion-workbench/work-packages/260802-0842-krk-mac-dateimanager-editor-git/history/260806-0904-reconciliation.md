# Reconciliation — 260806-0904

**Anlass:** Phase 3 der Orchestrator-Sitzung 260805-2128 (Turn 21, S19–S23). Domäne: code.

## Umfang

- 2 Planungsdateien geprüft (Spec und Plan), 1 aktualisiert (Plan: Statuszeile und Reconciliation Log).
- 123 Defektdateien geprüft (122 Circle, 1 shared), 0 Marker bewegt, 1 neuer Defekt gefiled.
- 34 Entscheidungsdatensätze geprüft (29 Circle, 5 shared), **16 Marker `_a_` → `_i_` bewegt** (14 Circle, 2 shared), je mit Implemented-Zeile und Commit-Beleg.
- 4 Prüfberichte gesichtet, keine Annotation nötig (die 5 Befunde aus `reviews/260806-0834-coderev-turn-21-s19-bis-s23.md` sind sämtlich als offene Defekte gefiled).

## Befunde

**Schrittmarker stimmen.** S19 `4886819`, S20 `89f962d`, S21 `d09c059`, S22 `e8626b6`, S23 `d577295` — alle fünf im Code belegt (`vorschau.rs`/`vorschaumodell.rs`, `belegungsansicht.rs`/`belegungsmodell.rs`, `messmodus.rs`/`xtask/src/messen.rs`, `messungen/260805-2207-*`, `xtask/src/release.rs`). 35 von 36 Schritten `[DONE]`, offen allein S6b. Der Plan bleibt `_o_`: Runde 1 schließt nicht vor der Klärung von `decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md` (Nutzerentscheid 260806).

**Resolved-Notizen stimmen.** Die drei in diesem Turn geschlossenen Defekte (Rechte-Vorschau 260803-2007, L4-Streuung 260803-1845, L1/L9 unter Fremdlast 260805-2335) zitieren Commits und Messberichte, die existieren und das Behauptete tragen.

**Keine beiläufigen Schließungen.** Keiner der elf vor dem Turn offenen Defekte ist durch S19–S23 erledigt: die Turn-Commits berühren weder `menue.rs` (Menü-Protokoll, Autofill) noch `auffrischung.rs` (Lesezeichen, Netzpfad) noch den Größenformatierer ("Zero KB"); das Merkmal `CFRunLoop` steht weiter in `Cargo.toml:36`; die Dateiliste von S17 nennt weiter den alten Modulpfad (Plan Zeile 1002).

**Hauptdrift: 16 Entscheidungsdatensätze hingen hinter der Umsetzung her.** Auf `_i_` gezogen:

| Datensatz | Beleg |
|---|---|
| `260802-1036` leistungszusagen-navigator | `e8626b6`, `messungen/260805-2207-*` |
| `260802-1036` umbenennen-im-stapel-umfang | `91b904e`, `3c7191a`, `stapelumbenennen/` |
| `260802-1428` was-l4-mit-wiederhergestellten-tabs-meint | `d09c059`, `e8626b6` |
| `260803-2007` was-krk-tut-wenn-das-letzte-fenster-geschlossen-wird | `537fda5`, `menue.rs:217`, `anwendung.rs:419` |
| `260804-1122` wandern-die-bereichsbreiten | `06dc48b`, `default-keymap.toml:383-391` |
| `260804-1832` traegt-der-fortschritt-ein-blatt-oder-die-statuszeile | `5a2f05d`, `c89ea66` |
| `260804-2318` fortschrittsschwelle-nach-zeit | `3c7191a`, `operationen.rs:26` |
| `260805-0000` auffrischung-auf-netzlaufwerken | `395e475` (Spec C9, kein Codeeingriff) |
| `260805-0000` menuekuerzel-in-die-konflikterkennung | `58465bf` |
| `260805-0000` nachweis-des-verworfenen-ausblendbefehls | Implemented-Zeile lag schon vor, Marker hing |
| `260805-0000` was-die-gemeldete-eintragszahl-zaehlt | Implemented-Zeile lag schon vor, Marker hing |
| `260805-0000` welcher-bereich-den-fokus-…-haben-muss | `7a0c0a6`, `4886819` |
| `260805-0000` zweites-kennzeichen-der-markierung | `3c7191a` |
| `260805-1623` taste-und-einstellbarkeit-des-terminal-befehls | `f850f30`, `48e69df` |
| shared `260802-0842` f-tasten-unter-macos-systembelegung | `6b4fb2d`, `d1a8ab1`, `default-keymap.toml:101-123` |
| shared `260802-0842` loeschen-papierkorb-oder-endgueltig | `daecb45`, `343a7f3`, `operationen.rs:422-429` |

`260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md` bleibt `_a_`: die Statuszeile (S12) steht, der `NSAlert` (S6b) nicht.

**Plan-Statuszeile war veraltet.** "Entwurf, zur Abnahme" stand seit dem 260802 unverändert; jetzt auf den Ausführungsstand gezogen.

## Neu gefiled

- `issues/260806-0904_o_claude-md-fuehrt-projektstand-und-entscheidungsstand-vom-260803.md` — `CLAUDE.md` führt 8/24 Schritte, 3 offene Defekte und 5 offene Fragen; tatsächlich 35/36, 17 und 10.

## Stand nach dem Abgleich

- Defekte: 17 `_o_` im Circle (mit dem neuen: 18), 0 in `shared/issues/`.
- Entscheidungen: 23 `_i_`, 1 `_a_`, 10 `_o_` (7 Circle + 3 shared). Die L9-Frage hält die Rundenschließung.
- Voice-Profil: `chat-voice-de.yaml` geladen; `default-voice-de.yaml` wird für den Reconciler nicht emittiert, kein Fallback nötig.

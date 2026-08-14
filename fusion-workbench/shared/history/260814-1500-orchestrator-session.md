# Orchestrator Session — 260814-1500

**Directive:** (noch nicht gestellt — Setup gelaufen, Auftrag steht aus)
**Mode:** (noch nicht aufgelöst)
**Status:** In Arbeit

## Bestandsaufnahme beim Start

| Größe | Wert |
|---|---|
| Arbeitsverzeichnis | `/Users/k1/Projects/productive/krk` |
| Plugin-Version | 8.2.0 |
| git HEAD | `43dfe90` |
| Aktiver Circle | keiner |
| Circles | 1 vorgesehen (`_a_`), 8 beschränkt geschlossen (`_b_`), 1 kohärent geschlossen (`_c_`) |
| Offene Defekte (gemeinsamer Speicher) | 11 |
| Offene Defekte (alle Circle-Speicher) | 78 |
| Offene Planschritte (gemeinsam) | 1 Plandatei |
| Offene Entscheidungsfragen (alle Speicher) | 19 |
| Analysen (gemeinsam) | 0 |
| Warteschlange `tasklist.md` | nicht vorhanden |
| Rundenbudget | 5 |
| Wächter | kein Halt aktiv (`haltActive: false`) |

## Erkannte Domäne

`code` — 135 Quelldateien gegen 11 Datendateien, gezählt über `git ls-files`
(`counted_by=git-ls-files`). Der Zweig `code_files > 0` greift; das Datenverhältnis
bleibt weit unter der Zweifachschwelle.

## Sitzungsmarker

Beim Start lag ein veralteter Sitzungsmarker vor (Herzschlag 3794 s alt, Schwelle 600 s),
also kein paralleler Lauf. Marker für diese Sitzung neu geschrieben.

## Häufig geänderte Dateien

Aus `fusion-churn-rank` (971 Einträge, 455 davon zu nicht mehr vorhandenen Dateien,
2 als Rauschen verworfen, 10 gewertet). Die vier obersten:

| Punkte | Datei |
|---|---|
| 183 | `crates/krk-ui/src/appkit/anwendung.rs` |
| 88 | `crates/krk-ui/src/appkit/editor.rs` |
| 71 | `crates/krk-ui/src/appkit/tabelle.rs` |
| 60 | `crates/krk-ui/src/appkit/vorschau.rs` |

## Hinweis Portfolio

Ein vorgesehener Circle liegt bereit. `/fusion:next` zeigt das Portfolio.

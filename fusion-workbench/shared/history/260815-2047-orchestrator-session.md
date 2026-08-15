# Orchestrator-Sitzung — 260815-2047

**Directive:** Ordner in der Dateiliste auf einen Blick von Dateien unterscheidbar machen
**Mode:** custom
**Status:** Läuft

## Aufnahme beim Start

| Größe | Wert |
|---|---|
| Arbeitsplatz | `/Users/k1/Projects/productive/krk` |
| git HEAD | `c27d845` |
| Offene Defekte (gemeinsam) | 17 |
| Offene Defekte (Circles) | 86 |
| Offene Planschritte (gemeinsam) | 1 |
| Offene Fragen (`_o_`, alle Speicher) | 24 |
| Circles | 1 vorgesehen, 9 beschränkt geschlossen, 1 kohärent geschlossen |
| Aktiver Circle | keiner |
| Arbeitswarteschlange | keine an der Wurzel |
| Turn-Budget | 5 |
| Wächter | kein Halt (`haltActive: false`) |

## Bereichserkennung

`code_files=138`, `data_files=11`, `counted_by=git-ls-files` → **domain = code**
(Quelldateien vorhanden, Datendateien nicht mehr als doppelt so viele).

## Hinweise

- Circle-Hinweis ausgegeben: 1 vorgesehener, 0 aktive Circles → `/fusion:next` zur
  Portfolio-Durchsicht angeboten.
- Stilprofile geladen: `chat-voice-de.yaml`, `default-voice-de.yaml`.
- Häufig geänderte Dateien (Top 3): `crates/krk-ui/src/appkit/anwendung.rs`,
  `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/tabelle.rs`.

## Verlauf

- 20:47 Setup abgeschlossen. Kein unterbrochener Vorgang gefunden.

## Turn 1

- Nutzerfrage: woran erkennt man in der Dateiliste einen Ordner?
- Drei Wege vorgelegt, jeder mit seinen Folgen am Code geprüft; Grundlage sind
  drei bestehende Festlegungen (Farbe allein zählt nicht, Fett/Orange/Blau sind
  vergeben, ein Zeichen vor dem Namen ist verworfen).
- Nutzerentscheid 260815-2058: Option 3, Schrägstrich hinter dem Ordnernamen.
  Datensatz `shared/decisions/260815-2056_a_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`.
- Aufgabe T1 an den `coder`.

# Planner: Implementierungsplan für die erste Umsetzungsrunde von KRK

**Datum:** 2026-08-02, 14:28
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Angefordert von:** Nutzer (über den Orchestrator)

---

## Auftrag

Den Implementierungsplan für Runde 1 schreiben: das Navigator-Gerüst nach Spec `260802-1036_o_spec-navigator-geruest.md`, Fähigkeiten C1 bis C9. Die sieben Fragen aus `## Offen für den Planner` beantworten, die Zeitzusagen aus C8 als Prüfschritte verankern, die Risiken des Technologieentscheids einplanen und die ungeprüfte Annahme zu macOS-26-Schnittstellen prüfen. Editor und Git bleiben draußen. Ausführende Agenten: `coder` und `ontocoder`.

## Gelesene Grundlagen

- `CLAUDE.md` im Projektwurzelverzeichnis, vollständig
- Spec `planning/260802-1036_o_spec-navigator-geruest.md`, vollständig
- Entscheidung `decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md` (bindend: Rust mit AppKit über `objc2`, sechs Randbedingungen)
- Analyse `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`, vollständig einschließlich der Gegengründe zu Rust
- Entscheidungen `260802-1036_a_leistungszusagen-navigator.md`, `260802-1036_a_umbenennen-im-stapel-umfang.md`, `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`, `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`
- Vorprüfung `spikes/fn-tasten/README.md`, `messung-A.txt`, `messung-A-neuauswertung.txt`
- Zwei offene Defekte `260802-1417_o_directive-zeile-sagt-freie-funktionstasten-zu.md` und `260802-1417_o_circle-datensatz-status-widerspricht-dem-marker.md`
- Circle-Datensatz `_t_circle.md`

## Selbst geprüfter Stand der Werkzeugkette

Der Nutzer hatte den Stand mitgeteilt; wir haben ihn nachgeprüft, weil der Bauzuschnitt daran hängt.

| Prüfung | Kommando | Ergebnis |
|---|---|---|
| Rust | `~/.cargo/bin/rustc --version`, `cargo --version` | 1.97.1, bestätigt |
| Ziele | `rustup target list --installed` | `aarch64-apple-darwin`, `x86_64-apple-darwin`, bestätigt |
| Pfad | `grep -n cargo ~/.zshrc` | Zeile 37, `. "$HOME/.cargo/env"`, bestätigt |
| Xcode | `xcode-select -p` | `/Library/Developer/CommandLineTools`, vollständiges Xcode fehlt, bestätigt |
| Swift, clang | `swift --version` | 6.1.2, clang 1700, bestätigt |
| System | `sw_vers`, Swift-Zielangabe | macOS 15.7.7, Ziel `x86_64-apple-macosx15.0` |

Der Rechner dieser Sitzung ist zugleich das Referenzgerät aus C8. Rust liegt nicht im Pfad nichtinteraktiver Schalen; die Bauskripte des Plans laufen über `cargo` und sind davon nicht betroffen, weil `~/.cargo/env` in `~/.zshrc` steht.

## Geprüfte Annahme: braucht Runde 1 eine macOS-26-Schnittstelle?

**Nein, die Annahme des Nutzers hält.** Wir haben die Schnittstellen durchgesehen, die der Plan benutzt. Die höchste Untergrenze liegt bei macOS 14.0 (`CADisplayLink` an einer `NSView`, für die L1-Messung), alle übrigen deutlich darunter: `NSTableView`, `NSSplitView`, `NSEvent`-Abgriffe, `NSMenu`, `NSWindow` aus macOS 10.x, `getattrlistbulk` aus 10.10, `NSFileManager.trashItemAtURL:` aus 10.8, `copyfile` mit `COPYFILE_CLONE` aus 10.12, FSEvents aus 10.5. Ein eigener Prüfschritt entfällt damit; die Prüfung leistet der Bau selbst über `MACOSX_DEPLOYMENT_TARGET=15.0` und den Nachweis am Binärformat mit `vtool -show-build-version` (Plan, Schritte S1 und S5).

**Die dahinterliegende Mechanismusfrage bleibt offen und ist als Entscheidungsdatensatz abgelegt.** Recherchiert: `objc2` bietet kein Gegenstück zu Swifts `if #available`. Die Frage wird im Vorhaben als Ausgabe 266 geführt und ist offen; eine Umsetzung liegt als Änderungsvorschlag 212 in Arbeit vor. Heute üblich sind `NSProcessInfo.isOperatingSystemAtLeastVersion` mit `respondsToSelector:` oder `NSClassFromString` für Objective-C-Teile und schwaches Binden über `-weak_framework` oder `dlsym` für C-Funktionen. Quelle: https://github.com/madsmtm/objc2/issues/266, am 260802 abgerufen.

## Was entstanden ist

**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

23 Schritte in sechs Phasen, jeder mit genannten Dateien und einem an Diff oder Kommando prüfbaren Abnahmekriterium. 21 Schritte gehen an den `coder`, 2 an den `ontocoder` (`resources/Info.plist` als S4, `resources/default-keymap.toml` als S9). Vier Mermaid-Diagramme: Modulschichtung, Ladepfad eines Ordners, Zustände einer Dateioperation, Abhängigkeitsgraph der Schritte.

**Die sieben Fragen aus `## Offen für den Planner` sind beantwortet:**

1. Sprache und Werkzeugkasten: bindend entschieden, Rust mit AppKit über `objc2`; die sechs Randbedingungen sind auf Schritte abgebildet.
2. Verzeichnis lesen, halten, darstellen: `getattrlistbulk` statt `readdir` plus `stat`; Einträge und Sichtreihenfolge getrennt; gestückeltes Lesen mit Generationsnummer, sodass L2 die erste Bildschirmseite und L3 die vollständige Sortierung trägt.
3. Fremde Änderungen: FSEvents mit 300 ms Sammelverzögerung; **ein** Auffrischungspfad `ordner_neu_lesen(pfad)` für fremde und eigene Änderungen; `NSWorkspace` für Datenträgerwechsel.
4. Ablage: drei TOML-Dateien unter `~/Library/Application Support/KRK/`, atomar geschrieben; die Auslieferungsbelegung als eigene Datei, über `include_str!` eingebettet.
5. Messungen: zwei Strecken (kopflos für L2, L3, L10; in der Anwendung für die übrigen), deterministischer Prüfordner-Erzeuger mit festem Startwert für 10.000 und 100.000 Einträge, `purge` für den kalten Fall mit Abbruch statt stiller Warmmessung, Bedingungskopf an jedem Bericht.
6. Nebenläufigkeit: der Hauptfaden führt keine Dateisystem-Arbeit aus; `copyfile` mit Statusrückruf liefert Fortschritt, Abbruch, Metadaten und APFS-Klonen in einem Mechanismus; das Fortschrittsfenster erscheint nach einer 150-ms-Zeitregel statt nach zwei Umfangsschwellen.
7. Signierung: außerhalb der Sandbox, Bündelbau von Hand ohne Xcode, stabile lokale Signaturidentität statt Ad-hoc (sonst fragt TCC bei jedem Bau erneut), fünf deutsche TCC-Rückfragetexte im `Info.plist`, keine Bedienungshilfen-Freigabe nötig.

**Frühes Laufendes und frühe Messung, wie beauftragt.** Ein Fenster mit echter Dateiliste steht als Schritt S6 von 23. Die Messstrecke steht kopflos als S3, die erste Messung am laufenden Durchstich als S8. S8 ist ein Gate: verfehlt L1, L2, L3, L4 oder L10 die Zusage, endet der Schritt mit einem Entscheidungsdatensatz und ohne Reparaturversuch, weil dann der Technologieentscheid zur Debatte steht.

**Risiken des Technologieentscheids sind verortet.** Fünf der sechs `define_class!`-Deklarationen liegen in Phase A, vier davon in S6. `unsafe` steht ausschließlich unter `crates/krk-ui/src/appkit/`, durchgesetzt über `#![forbid(unsafe_code)]` in `krk-core`. Vor dem bestandenen Gate S8 entsteht kein Layout-Code, der bei einem Wechsel verloren ginge.

## Beim Gegenlesen selbst korrigiert

Zwei eigene Fehler im ersten Entwurf des Plans, beide vor der Abgabe behoben:

- Der Entwurf schrieb `#![forbid(unsafe_code)]` für `krk-core` vor und ließ zugleich ein Modul mit `#[allow(unsafe_code)]` für `getattrlistbulk` zu. `forbid` lässt sich per `allow` nicht öffnen, das ist sein Zweck. Korrigiert auf `#![deny(unsafe_code)]` mit genau einem Modul `verzeichnis::sys`, das die beiden Systemaufrufe `getattrlistbulk` und `copyfile` bindet. Betroffen waren der Abschnitt `## Aufbau` sowie die Schritte S1, S2 und S15 und die Risikotabelle.
- Die Diagramm-Selbstprüfung nannte S5 nach S23 eine "phasenübergreifende Rückwärtskante". Die Kante läuft in Phasenreihenfolge vorwärts; der Abhängigkeitsgraph hat überhaupt keine Rückwärtskante. Korrigiert.

## Angelegte Defekte und Entscheidungen

- `issues/260802-1428_o_messbedingungen-c8-nennen-keinen-pruefordner-fuer-l10.md` — die Messbedingungen in C8 definieren nur den Ordner mit 10.000 Einträgen, L10 misst auf 100.000.
- `decisions/260802-1428_o_was-l4-mit-wiederhergestellten-tabs-meint.md` — unter der Lesart "vollständig gelesen" widersprechen sich L4 mit 1000 ms und L10 mit 4 s, sobald ein wiederhergestellter Tab groß ist. Empfehlung: Möglichkeit 1, die bedienbare erste Bildschirmseite.
- `decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` — wie KRK aus Rust eine Schnittstelle ab macOS 26 ansteuert. Bindet Runde 1 nicht.

## Was der Plan nicht enthält

Editor und Git-Anbindung, wie vom Spec unter `## Nicht in dieser Runde, aber im Circle` abgegrenzt. Alles unter `## Außerhalb des gesamten Circles`. Aufwandsschätzungen, weil der Nutzer keine verlangt hat.

## Nicht angefasst

Der Circle-Datensatz `_t_circle.md`. Das Feld `**Active spec/plan:**` trägt weiterhin den Spec und nicht diesen Plan; ein anderer Agent arbeitet parallel am Datensatz und behebt dabei die beiden gemeldeten Defekte. Der Pfad des Plans ist dem Nutzer stattdessen zurückgemeldet.

Nicht committet, wie beauftragt. Der Orchestrator committet.

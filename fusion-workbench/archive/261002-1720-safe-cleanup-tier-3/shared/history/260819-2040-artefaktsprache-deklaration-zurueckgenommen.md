# Artefaktsprache-Deklaration aus CLAUDE.md entfernt

**Status:** Complete
**Coder**, Domäne `code`.
**Anlass:** Der Nutzer hat den Entscheidungsdatensatz `shared/decisions/260819-1500_a_gilt-die-artefaktsprache-en-fuer-den-ganzen-bestand-oder-wird-die-deklaration-zurueckgenommen.md` am 260819-2032 mit Möglichkeit 2 beantwortet: die Deklaration der Artefaktsprache wird zurückgenommen, der ganze Bestand bleibt deutsch.

## Was geändert wurde

Genau zwei Stellen in `CLAUDE.md`, keine dritte:

1. **Zeile 4** `**Artifact language:** en` entfernt (nicht auf `de` gesetzt — der Datensatz hält unter `## Constraints` fest, dass das Entfernen der Weg ist, den die Regel für ein einsprachiges Projekt beschreibt). Die Zeile `**Language:** de` bleibt unverändert stehen.
2. **Abschnitt `## Sprache`** auf den Stand nach der Entfernung nachgezogen: beschreibt jetzt eine Deklarationszeile statt zwei, hält fest, dass `**Language:** de` nach der Fallback-Kette aus `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## Project language` beide Flächen steuert (Kurzform- **und** Langform-Profil, letzteres jetzt `default-voice-de.yaml` statt `default-voice-en.yaml`), behält die Formgebundenheit der Zeile `**Language:** de` (nicht umformulieren, nicht verschieben, nicht entfernen) und den wörtlichen Schlusssatz „Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen." Ein neuer Satz nennt die elf englischen Defektdatensätze aus der Zeit der Deklaration, die als Bestand stehen bleiben.

Kein Datensatz und kein Code wurde übersetzt oder sonst angefasst — der Entscheidungsdatensatz verbietet unter `## Constraints` ausdrücklich jeden Übersetzungsdurchgang, und die Aufgabenstellung selbst begrenzt die Änderung auf die zwei genannten Stellen.

Der Entscheidungsdatensatz selbst ist von diesem Auftrag ausdrücklich nicht erfasst (Marker bleibt bei `_a_`, keine dritte Datei) — der Orchestrator zieht ihn nach.

## Prüfung

`grep -c 'Artifact language' CLAUDE.md` → 0. `grep -n '^\*\*Language:\*\* de$' CLAUDE.md` → Zeile 3, unverändert. `git diff CLAUDE.md` zeigt ausschließlich die zwei genannten Stellen. `cargo fmt --all --check` — Rückgabewert 0 (Dokumentation, keine Codeänderung).

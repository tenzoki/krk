Der Ausschluss versteckter Fotos beruft sich auf einen Nutzerentscheid, den kein Datensatz trägt, und Spec und Plan sagen das Gegenteil
---
`ist_foto` nimmt Einträge aus, deren Name mit einem Punkt beginnt, und der Modulkopf von `bildfolge.rs` begründet das mit „Nutzerentscheid vom 260929“. Auf der Platte steht dieser Entscheid nirgends: der Spec zählt nach C2.1 jede Datei mit passender Endung, der Plan baut nach Entscheidung 5 „nach dem Wortlaut des Spec“ und führt die Frage unter `## Open Questions` weiter offen. Ein späterer Leser findet den Code im Widerspruch zu beiden Datensätzen und keinen Beleg für die Abweichung.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (C2.1); `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (Entscheidung 5, erste offene Frage „Versteckte Fotos“)

## Befund

- `crates/krk-core/src/leseprofil/bildfolge.rs`, Modulkopf „Was ein Foto ist“: „Ein versteckter Eintrag, dessen Name mit einem Punkt beginnt, zaehlt nicht (Nutzerentscheid vom 260929)“; `ist_foto` (Zeile 243) prüft `!eintrag.name.starts_with('.')`.
- Spec C2.1: „Als Fotos zählen genau die Dateien mit den Endungen … Ordner, Verknüpfungen und andere Dateien zählen nicht.“ Versteckte Dateien sind darin eingeschlossen.
- Plan, Entscheidung 5: „Versteckte Einträge zählen nach dem Wortlaut des Spec mit; siehe `## Open Questions`.“ Die Frage steht dort mit `[ ]`, und der Plan trägt `_c_`.
- Commit `f011f5b` nennt die Ausnahme ohne Beleg. `grep -rl AppleDouble fusion-workbench` findet allein den Plan.
- `resources/default-readers.toml` und `HowTo.md` beschreiben den Ausschluss so, wie der Code ihn baut.

**Nicht geprüft:** ob der Nutzer im Gespräch so entschieden hat. Der Befund ist das Fehlen des Datensatzes, nicht die Richtung der Regel.

## Richtung

Den Entscheid als Datensatz im Arbeitspaket niederlegen (Frage „zählen versteckte Fotos“, Antwort mit `ruled by user`), und der Modulkopf zitiert ihn statt „Nutzerentscheid vom 260929“. Hat der Nutzer nicht so entschieden, gilt C2.1, und `ist_foto` fällt auf den Wortlaut zurück.

## Abnahme

- Ein Entscheidungsdatensatz im Speicher des Arbeitspakets trägt die Antwort und wer sie gegeben hat.
- Der Modulkopf von `bildfolge.rs` zitiert ihn in Sternform.

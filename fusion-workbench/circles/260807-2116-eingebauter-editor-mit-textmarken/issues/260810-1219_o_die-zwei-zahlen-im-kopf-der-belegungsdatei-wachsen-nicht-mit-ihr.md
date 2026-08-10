Die zwei Zahlen im Kopf der Belegungsdatei wachsen nicht mit ihr

---

`resources/default-keymap.toml:30` nennt zwei Zählstände:

```
# Ausgeliefert sind 71 Funktionen mit zusammen 79 Kombinationen.
```

**Beide stimmen** (geprüft am 260810-1219: 71 `[[funktion]]`-Einträge, 79 Einträge über alle `tasten`-Listen zusammen). Der Defekt ist nicht die Zahl, sondern dass nichts sie hält. Wer einen Eintrag hinzufügt, bekommt keinen Hinweis, dass Zeile 30 nachzuziehen ist: die Zahl steht in einem Kommentar, und ein Kommentar hält keinen Bau an.

## Warum das gerade hier zählt

Die Datei wird über `include_str!` eingebunden und beim Bau eingelesen; ein Formfehler hält `cargo test --workspace` an, ein falscher Zählstand im Kommentar nicht. Genau das trennt diese Zahl von den vier Aufzählungen in `crates/`, die die Runde 2 erweitert hat (`Wirkungsbereich`, `Kommando`, `Bereich`, `Fokus`): dort nennt der Übersetzer jede Stelle, die nachzuziehen ist, weil die Fallunterscheidungen keinen Auffangzweig tragen. Der Dateikopf hat diese Sicherung nicht.

Zwei Kommentarstellen derselben Datei sind aus demselben Grund schon auseinandergelaufen und in dieser Durchsicht gefunden worden:

- `resources/default-keymap.toml:479` zählt fünf Kombinationen auf dem Buchstaben `e`, es sind vier (`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1217_*_der-editor-abschnitt-der-belegung-zaehlt-fuenf-e-tasten-und-hat-vier.md`)
- `resources/default-keymap.toml:25` nennt `belegung_ansehen` als Funktion ohne Kommando, sie trägt eines (`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1218_*_der-dateikopf-der-belegung-nennt-belegung-ansehen-als-funktion-ohne-kommando.md`)

Die Zahlen aus Zeile 30 sind heute richtig; die Frage ist, ob sie es nach dem nächsten Eintrag noch sind.

## Vorgeschlagene Behebung

Eine Probe neben den bestehenden in `crates/krk-core/src/tasten/belegung.rs`, die beide Zählstände an der eingebetteten Auslieferungsbelegung festnagelt:

- Anzahl der Funktionen ist 71
- Summe der Kombinationen über alle Funktionen ist 79

Der Kommentar der Probe nennt `resources/default-keymap.toml:30` als die Stelle, die mitzuziehen ist. Dann bricht die Probe beim nächsten hinzugefügten Eintrag, und wer sie repariert, kommt am Dateikopf vorbei. Das ist dieselbe Bauform, die dieses Projekt bei den Fallunterscheidungen ohne Auffangzweig schon fährt: eine Änderung soll eine bewusste Einordnung erzwingen, statt still durchzugehen.

Die vorhandene Probe `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` sitzt schon in dieser Datei und ist der richtige Nachbar.

## Zuständigkeit

`coder`. Es ist eine Probe in `crates/krk-core`, kein Datensatz. Die Zahlen in der TOML-Datei bleiben unangetastet, weil sie stimmen.

---

**Gefunden von:** ontorev, Durchsicht der Belegungsdatei 260810-1217
**Domain:** code
**Schwere:** Low
**Betroffen:** `crates/krk-core/src/tasten/belegung.rs` (die fehlende Probe), `resources/default-keymap.toml:30` (die ungesicherte Zahl)
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/reviews/260810-1217-ontorev-belegungsdatei-nach-den-drei-kommentarstellen.md` (der Bericht, aus dem dieser Defekt stammt)

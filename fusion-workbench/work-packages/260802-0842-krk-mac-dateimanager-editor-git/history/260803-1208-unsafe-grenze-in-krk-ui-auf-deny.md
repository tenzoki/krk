# Die `unsafe`-Grenze in `krk-ui` kommt auf `deny`, und der Plan steht darauf

**Agent:** planner
**Datum:** 260803-1208
**Status:** Complete
**Entscheidungsdatensatz:** `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-1208_a_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`
**Geänderte Plandatei:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`

---

## Auftrag

Eine Nutzerentscheidung festhalten und den Plan darauf ziehen. `crates/krk-ui/src/main.rs` trägt `#![warn(unsafe_code)]`; künftig gilt `#![deny(unsafe_code)]`, und das spätere Modul `appkit` trägt die Ausnahme `#[allow(unsafe_code)]`, genau wie `verzeichnis::sys` in `krk-core`. Der Codewechsel selbst gehört dem `coder`, nicht dieser Sitzung. Keine Änderung außerhalb der Plandatei, des neuen Entscheidungsdatensatzes, des Defekts zu Schritt 6 und dieses Protokolls. Nicht committen.

## Der Entscheidungsdatensatz

Angelegt als `_o_`, mit angehängter `Answered:`-Zeile sofort auf `_a_` gezogen, nicht weiter. Auf `_i_` zieht ihn der `coder`, sobald der Commit mit der geänderten Zeile liegt.

Der Datensatz stellt die Frage als Wahl zwischen erzwungener und beobachteter Grenze und führt drei Möglichkeiten: `deny` mit Ausnahme am Modul, `warn` mit einer Prüfvorschrift, die stattdessen den Code absucht, und `forbid` mit einer eigenen Kiste für die AppKit-Hüllen. Die dritte trägt den Gegengrund, den der Plan für `krk-core` schon einmal ausgeschrieben hat: eine eigene Kiste für einen Anteil, der ohnehin in einem eigenen Modul liegt, ist die teurere Antwort. Die Empfehlung geht auf Möglichkeit 1 und deckt sich mit der Wahl des Nutzers.

Ein Punkt aus dem auslösenden Defekt steht im Datensatz als eigener Absatz, weil er die Begründungslage beschreibt und nicht die Sache selbst: der Plan leitet für `krk-core` ausführlich her, warum dort `deny` und nicht `forbid` steht, und sagt zur `warn`-Wahl in `krk-ui` an keiner der drei Stellen etwas. Sie stand als Setzung da, dreifach wiederholt, nirgends hergeleitet.

## Was im Plan geändert ist

**Der Kopf.** Eine Zeile `**Nachzug 260803-1208:**` nach dem Muster der vier vorangegangenen Nachzüge, mit der Entscheidung, ihrem Grund und der Liste der nachgezogenen Stellen. Die Datumszeile führt den neuen Nachzug mit.

**Der Absatz über die zwei Übersetzerregeln in `## Aufbau`.** Aus einem Absatz sind drei geworden. Der erste nennt beide Regeln und die Stelle, an der jede geöffnet ist. Der zweite begründet `deny` gegen `warn` für `krk-ui`, also die Begründung, die der Plan bisher schuldig blieb. Der dritte begründet `deny` gegen `forbid` und gilt jetzt für beide Kisten statt nur für `krk-core`; das ist dieselbe Herleitung wie vorher, auf zwei Fälle gezogen.

Dazugekommen ist eine Angabe, die der Plan brauchte und nicht hatte: die Ausnahme liegt am Kopf von `appkit/mod.rs` und deckt den ganzen Teilbaum ab, weil Lint-Regeln in die eingebetteten Module durchschlagen. Ohne diesen Satz hätte ein Umsetzer das Attribut in jede Datei unter `src/appkit/` schreiben können, und das Abnahmekriterium, das genau eine Datei verlangt, wäre reihenweise gescheitert. In `krk-core` stellt sich die Frage nicht, weil `verzeichnis::sys` ein Blatt ist.

**Die Verzeichnisstruktur.** Die Zeile zu `krk-ui` nennt jetzt `deny(unsafe_code)`, wie die Zeile zu `krk-core` es seit jeher tut.

**Schritt 1.** Der `[DONE]`-Vermerk bleibt. Die `Änderungen` nennen `#![deny(unsafe_code)]` als die geltende Regel und halten in einem eigenen fett gesetzten Satz fest, dass S1 mit `warn` umgesetzt wurde und die Umstellung zu S6 gehört. Das Abnahmekriterium verlangt den Diff mit `deny` in beiden Kisten und schreibt daneben aus, dass die Zeile in `krk-ui` beim Abnehmen von S1 noch auf `warn` stand. Ein Leser, der Plantext und Commit-Historie nebeneinanderlegt, findet die Abweichung damit erklärt, statt sie für einen Fehler zu halten.

**Schritt 6.** Die Dateiliste vermerkt, dass `main.rs` in diesem Schritt nicht nur `mod appkit;` aufnimmt, sondern auch die Regel trägt. Ein neuer Punkt unter den `Änderungen` schreibt den Codewechsel aus: die Zeile in `main.rs`, der Modulkommentar darunter, der bisher sagt, außerhalb von `appkit` warne der Übersetzer, und der Kopf von `appkit/mod.rs` mit `#![allow(unsafe_code)]`. Der Grund für die Bündelung steht dabei: `deny` ohne das Modul mit seiner Ausnahme hat nichts zu erlauben und ließe den Bau der Kiste scheitern, sobald der erste AppKit-Aufruf entsteht.

Das Abnahmekriterium ersetzt die `grep`-Vorschrift durch die Form aus S2 und S15, auf `krk-ui` umgeschrieben: `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src` nennt genau eine Datei, `appkit/mod.rs`, und der erfolgreiche `cargo build -p krk-ui` trägt die andere Hälfte des Belegs.

**Die Risikotabelle.** Die Zeile zum unsicheren AppKit-Aufruf sagt jetzt `deny` für beide Kisten und benennt die Folge: ein Bau, der die Grenze überschreitet, scheitert. Das war der eigentliche Auslöser der Entscheidung, weil die Zeile "durchgesetzt" schon vorher zusagte und für `krk-ui` nur beobachtete.

**Der Abschnitt `## Angelegte Defekte und Entscheidungen`.** Die Einleitung steht auf fünf Punkten aus Planung und Nachziehen statt auf drei, und die Liste führt den geschlossenen Defekt zu Schritt 6 und den neuen Entscheidungsdatensatz mit.

## Die Prüfvorschrift ist am Dateibestand nachgeprüft

Am heutigen Bestand, in dem `src/appkit/` noch nicht existiert:

```
$ grep -rn 'allow(unsafe_code)' crates/krk-ui/src
crates/krk-ui/src/main.rs:5://! genau einer Stelle: das spaetere Modul `appkit` traegt `#[allow(unsafe_code)]`

$ grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src
(kein Treffer)
```

Der Modulkommentar von `main.rs` nennt das Attribut im Fließtext, genau wie der von `lib.rs` in `krk-core`. Ein `grep` ohne Anker fände ihn mit und ginge nach S6 auf zwei Dateien statt einer. Der verankerte Ausdruck lässt ihn aus, weil `//!` nicht mit `#` beginnt, und liefert heute erwartungsgemäß nichts, weil es die Ausnahme noch nicht gibt. Nach S6 muss er `appkit/mod.rs` nennen und sonst nichts.

Die Kommentarzeile 6 derselben Datei sagt "ausserhalb davon warnt der Uebersetzer" und wird mit dem Codewechsel falsch. Sie steht deshalb in den `Änderungen` von S6 mit drin.

## Was `grep` im Plan noch findet

Vier Stellen mit `warn(unsafe_code)`, alle gewollt: der neue Kopfvermerk, die `Änderungen` und das Abnahmekriterium von S1 sowie der neue Punkt in S6. Jede beschreibt den vorherigen Stand oder den Wechsel selbst. Keine Vorschrift verlangt noch `warn`.

## Buchführung

- `issues/260803-1200_c_abnahmekriterium-von-schritt-6-traegt-denselben-grep-fehler.md` trägt eine `Resolved:`-Zeile und den Marker `_c_`.
- Der Entscheidungsdatensatz trägt `**Status:** answered`, die `Answered:`-Zeile und den Marker `_a_`.
- Der Entwurf ist unverändert. Keine Schrittnummer, keine Abhängigkeit und kein `[DONE]`-Vermerk wurde angefasst.
- Kein Eingriff in `crates/`, `xtask/`, `resources/`, `README.md` oder `CLAUDE.md`. Der Codewechsel steht aus und gehört dem `coder`.
- Nicht committet, wie beauftragt.

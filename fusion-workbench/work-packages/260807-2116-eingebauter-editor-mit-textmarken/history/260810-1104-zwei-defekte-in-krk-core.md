# Zwei Defekte in `krk-core`: die Eingangskopie der Versatzrechnung und der Modulkopf von `sys.rs`

**Agent:** coder
**Status:** Complete
**Circle:** `260807-2116-eingebauter-editor-mit-textmarken`
**Bearbeitete Datensätze:**
`issues/260810-0424_*_das-richten-der-flaeche-kopiert-den-text-eines-16-mb-dokuments-dreimal.md`,
`issues/260810-1017_*_der-kopf-von-sys-rs-nennt-vier-fremdaufrufe-und-bindet-acht-funktionen.md`

---

## Was getan wurde

### 1. Die Eingangskopie der Versatzrechnung ist weg (`260810-0424`)

`crates/krk-core/src/text/datei.rs` trägt jetzt zwei Namen für **eine** Wandlung:

```
gehaltene_form(&str) -> Cow<'_, str>     die Regeln, leiht was schon paßt
        ^
        │  eine Fallunterscheidung darüber
        │
in_gehaltene_form(String) -> String      unverändert in der Signatur
```

`versatz_nach_der_wandlung` ruft `gehaltene_form(rest)` statt
`in_gehaltene_form(rest.to_owned())`. Der `Cow`-Vorschlag des Datensatzes ist damit
umgesetzt, **ohne** die Signatur von `in_gehaltene_form` anzufassen: die drei
weiteren Aufrufer bleiben unberührt, `datei::einlesen` voran, der die Funktion als
Wert an `Option::map` reicht, und `krk-ui` musste nicht angefaßt werden. Das war
die Frage, die der Datensatz ausdrücklich in die Antwort gelegt hat.

Die Fallunterscheidung in `in_gehaltene_form` ist die Stelle, an der der kurze Weg
kopienfrei bleibt: `Cow::Borrowed` heißt „nichts zu wandeln", und dann geht die
übernommene Zeichenkette zurück, statt aus der Leihe abgeschrieben zu werden.

### 2. Der Modulkopf von `sys.rs` nennt beide Zahlen (`260810-1017`)

Erste Zeile jetzt „die vier Schnittstellen, die KRK braucht, und die acht
Funktionen, die sie binden". Das Diagramm trägt unter `copyfile(3)` eine
eingerückte Zeile `copyfile_state_{alloc,free,set,get}`. Darunter steht der Satz,
den `lib.rs` und `verzeichnis/mod.rs` wortgleich tragen, mit der Begründung aus
dem Datensatz und dem Verweis, daß dieser Defekt die dritte Stelle nachgezogen
hat. Beide anderen Köpfe blieben unangetastet — sie lagen außerhalb der
Schreibgrenze und tragen die Zahl schon richtig.

## Die Messung, und wo sie von der Vorwegmessung abweicht

Neu: `crates/krk-core/tests/textkopien.rs`. Ein eigener `#[global_allocator]`
zählt die Anlagen ab 1 MB, also die Kopien in der Größenordnung der ganzen Datei;
die Probe führt die Fassung von vorher als `versatz_mit_kopie` mit und prüft, daß
beide denselben Versatz liefern.

| Fall | vorher | nachher |
|---|---|---|
| Rest in gehaltener Form (der Regelfall) | 1 Anlage, 16.777.196 Bytes | 0 Anlagen, 0 Bytes |
| Rest trägt selbst ein `\r\n` | 2 Anlagen, 33.554.396 Bytes | 1 Anlage, 16.777.198 Bytes |

**Der Regelfall trifft die Vorwegmessung vom 260810-1044 genau, der Gegenfall
nicht.** Sie führt dort für beide Fassungen zwei Kopien und schließt mit „daran ist
nichts zu holen"; gemessen sind es eine gegen zwei, denn die Eingangskopie
`rest.to_owned()` fällt in **beiden** Fällen weg und nur die Anlage der Wandlung
selbst bleibt. Welche Fassung dort gemessen wurde, ist nicht mehr feststellbar;
vermutlich eine, die den Text weiter übernahm. Die Abweichung steht in der
`Resolved:`-Zeile des Datensatzes, damit die Zahl nicht unkorrigiert weiterläuft.

Beim zweiten Datensatz habe ich die acht Bindungen selbst gezählt, statt die Zahl
zu übernehmen. Die Tabelle des Datensatzes stimmt in jeder Zeile, samt
Zeilennummern (jetzt um dreizehn verschoben, weil der Kopf um dreizehn Zeilen
gewachsen ist). Zwei Stellen zählen absichtlich nicht mit, und das steht jetzt in
der `Resolved:`-Zeile: `type Statusrueckruf = extern "C" fn(…)` ist ein Typalias,
und `extern "C" fn statusrueckruf(…)` ist eine Funktion von KRK mit C-Aufrufweg,
also ein Übergang in der anderen Richtung.

## Was offen bleibt

**Der Rest der Kopienkette liegt in `krk-ui` und ist nicht angefaßt.** Die Summe
der Kopien voller Länge geht von fünf auf vier; die drei verbleibenden Stellen in
`krk-ui/src/appkit/editor.rs` sind `text_zurueckschreiben`, `flaeche_richten` und
`NSString::from_str`. **In `krk-core` steckt kein Rest mehr:** die vierte Kopie ist
die Wandlung in `bearbeiten` selbst, also die eine Anlage, die `gehaltene_form`
bauen **muß**, wenn ein `\r\n` zu wandeln ist. Deshalb ist kein neuer Datensatz
angelegt.

## Geänderte Dateien

- `crates/krk-core/src/text/datei.rs`
- `crates/krk-core/src/verzeichnis/sys.rs`
- `crates/krk-core/tests/textkopien.rs` (neu)

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt -p krk-core -- --check` | exit 0 |

Die Marker der beiden Datensätze benennt der Nutzer um; nicht commitiert.

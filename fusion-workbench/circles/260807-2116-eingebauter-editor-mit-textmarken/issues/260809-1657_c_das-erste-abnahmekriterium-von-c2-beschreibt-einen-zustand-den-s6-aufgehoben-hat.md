# Das erste Abnahmekriterium von C2 beschreibt einen Zustand, den S6 aufgehoben hat

---
**Domain:** planning
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, C2, erstes Abnahmekriterium
**Cross-references:** `resources/default-keymap.toml:130-145`, `crates/krk-ui/src/belegungsmodell.rs:360-374`, S6 (erledigt), S42

---

## Der Befund

Das erste Abnahmekriterium von C2 lautet:

> F4 im Dateifenster öffnet den ausgewählten Eintrag im Editor. Die Taste ist
> dafür seit der Runde 1 freigehalten: `resources/default-keymap.toml:130-137`
> führt die Funktion `bearbeiten` mit leerer Tastenliste und dem Feld
> `reserviert_fuer = "editor"`.

S6 hat genau das aufgehoben: `bearbeiten` trägt seit dem 260809
`tasten = ["f4"]`, und `reserviert_fuer = "editor"` ist entfernt. Der zweite Satz
des Kriteriums beschreibt damit einen Zustand, den es nicht mehr gibt, und
verweist auf Zeilennummern, die inzwischen anderen Text tragen.

## Warum das zählt

Klein, aber an einer Stelle, die zählt: der Spec ist das Maß, an dem S42 abnimmt.
Wer das Kriterium beim Abnahmelauf liest, sucht ein Feld, das entfernt wurde, und
hält seine Abwesenheit für den Befund statt für die Erfüllung.

Der Fall ist derselbe wie bei `belegungsmodell.rs:363-368`, wo der Code die Frage
schon beantwortet hat: der Zweig für `reserviert_fuer` bleibt stehen, weil eine
`keymap.toml` aus einer älteren Fassung das Feld weiterhin tragen kann, aber die
Auslieferungsbelegung führt keine reservierte Funktion mehr. Der Spec sagt das
nicht.

## Vorschlag

Der zweite Satz wird auf die eingelöste Form gezogen:

> Die Taste war dafür seit der Runde 1 freigehalten; seit der Editor-Runde trägt
> `bearbeiten` die Taste F4 und das zugehörige Kommando.

S42 führt ohnehin einen Nachtrag am Spec (das Sicherungsform-Kriterium für C4 aus
S9). Dieser Satz gehört in denselben Nachtrag, damit der Spec nicht zweimal
angefasst wird.

Gemeldet von: `coderev`, Durchsicht Turn 2.

---
Resolved: Der zweite Satz des ersten Abnahmekriteriums von C2 ist am 260810-0714
auf die eingelöste Form gezogen, im selben Nachtrag wie die übrigen. Er lautet
jetzt: "Die Taste war dafür seit der Runde 1 freigehalten; seit dieser Runde trägt
die Funktion `bearbeiten` in `resources/default-keymap.toml` die Tastenliste
`["f4"]` und das zugehörige Kommando, und das Feld `reserviert_fuer = "editor"`
steht nicht mehr dort."

Ein Satz mehr als vorgeschlagen steht dahinter, und er ist der eigentliche Zweck
des Nachtrags: "Wer beim Abnahmelauf nach dem Feld sucht, sucht den eingelösten
Zustand und nicht den Befund." Die festen Zeilennummern `130-137` sind ersatzlos
entfallen; sie zeigten schon auf anderen Text und wären beim nächsten Einschub
wieder falsch.

Der Zweig für `reserviert_fuer` in `belegungsmodell.rs` bleibt unberührt, und der
Grund steht dort weiter im Doc-Kommentar: eine `keymap.toml` aus einer älteren
Fassung kann das Feld weiterhin tragen.

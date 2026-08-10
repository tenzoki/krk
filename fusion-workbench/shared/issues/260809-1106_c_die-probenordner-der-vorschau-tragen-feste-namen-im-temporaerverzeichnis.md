Die Probenordner der Vorschau tragen feste Namen im Temporaerverzeichnis

---

Sieben Proben in `crates/krk-ui/src/vorschaumodell.rs` legen ihren Probenordner
unter einem **festen** Namen im Temporaerverzeichnis an, ohne Prozesskennung,
ohne Laufnummer und ohne `Drop`, das ihn wieder abraeumt:

```
704  krk-vorschau-probe-text
716  krk-vorschau-probe-ordner
729  krk-vorschau-probe-gross
745  krk-vorschau-probe-bild-klein
767  krk-vorschau-probe-bild-gross
782  krk-vorschau-probe-binaer
818  krk-vorschau-probe-faden
```

Das ist die Gegenform zu `Pruefordner` in `crates/krk-core/tests/verzeichnis.rs:26-45`,
und `CLAUDE.md` beschreibt im Abschnitt „Was man nicht sieht, wenn man es nicht
weiß" die dortige Form als die des Projekts: „Prüfordner einzelner Testläufe …
tragen Prozesskennung und Laufnummer und räumen sich in `Drop` selbst auf."

`$TMPDIR` ist auf diesem Gerät pro **Nutzer** vergeben und nicht pro Sitzung
(gemessen: `/var/folders/1d/…/T/`). Zwei gleichzeitig laufende `cargo test`
schreiben und lesen deshalb dieselben sieben Dateien.

## Was beobachtet wurde

Am 260809 um etwa 11:00 scheiterten in einem `cargo test --workspace` genau zwei
Proben, beide aus dieser Gruppe und beide im selben Lauf:

```
test vorschaumodell::tests::keine_utf8_datei_faellt_auf_die_metadaten ... FAILED
test vorschaumodell::tests::eine_textdatei_ueber_der_grenze_faellt_auf_die_metadaten ... FAILED
test result: FAILED. 198 passed; 2 failed
```

Ein Lauf unmittelbar davor und mehrere danach waren gruen. Zu dieser Zeit
arbeiteten mehrere Agenten gleichzeitig am selben Arbeitsbereich, also liefen
plausibel zwei `cargo test` nebeneinander.

## Der naheliegende Weg, und was an ihm ungeprueft ist

`inference:` `fs::write` kuerzt die Zieldatei erst auf null und schreibt dann.
Wer in dieser Luecke liest, sieht eine leere Datei. Beide gescheiterten Proben
scheitern genau daran und an nichts anderem:

- `roh.bin` mit vier Bytes: leer gelesen ist `String::from_utf8(vec![])` ein
  `Ok("")`, also `Inhalt::Text`, und die Probe verlangt `Inhalt::Metadaten`.
- `gross.txt` mit `TEXTGRENZE + 1` Bytes: kuerzer gelesen liegt die Datei unter
  der Grenze, also wieder `Inhalt::Text` statt `Metadaten`.

**Erzwingen liess sich der Fall nicht.** Gemessen wurden 240 Laeufe der
Probengruppe in vier gleichzeitigen Schleifen, dazu 24 gleichzeitige Laeufe aus
geleertem Temporaerverzeichnis und dreimal zwei gleichzeitige
`cargo test --workspace`: alle gruen. Die Luecke ist demnach sehr schmal, und
**der Weg oben ist damit der naheliegendste Verdacht und keine belegte
Ursache.** Wer den Defekt anfasst, sollte das im Kopf behalten: bleibt der
Fehler nach dem Umbau, war es etwas anderes.

## Warum es trotzdem zu beheben ist

Die festen Namen sind unabhaengig vom beobachteten Fehlschlag eine Abweichung
von der Form, die das Projekt fuer Pruefordner festgelegt hat, und sie lassen
Muell im Temporaerverzeichnis liegen. Die Behebung ist klein: `Pruefordner` aus
`crates/krk-core/tests/verzeichnis.rs` ist nicht wiederverwendbar, weil er in
einem `tests/`-Baum eines anderen Pakets steht, aber die zwoelf Zeilen dorthin
zu uebernehmen kostet nichts. Dieselbe Form steht seit dem 260809 auch in
`crates/krk-core/tests/text.rs`.

**Nicht angefasst,** weil `vorschaumodell.rs` ausserhalb des Umfangs von S9 lag
und zur selben Zeit mehrere Agenten am Arbeitsbereich arbeiteten.

**Aufgefallen bei:** der Umsetzung von S9 des Editor-Plans am 260809-1100.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/history/260809-1106-coder-s9-einlesen-und-sicherungsform.md`

---
Resolved: 0140df7 — behoben durch `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1256_*_die-proben-des-vorschaumodells-legen-ihre-ordner-unter-festen-namen-an.md`, das denselben Gegenstand im aktiven Circle führte. Die sieben festen Namen in `crates/krk-ui/src/vorschaumodell.rs` sind fort; alle sieben Rufe gehen über `Pruefordner::neu`, der Name trägt Prozesskennung und Laufnummer, und `Drop` räumt ab. Am Code geprüft im Abschluss-Abgleich der Sitzung 260810-0845: acht `Pruefordner::neu`-Rufe in der Datei, kein `temp_dir().join("krk-vorschau-probe-…")` mehr, `pfad(&self) -> &Path` an beiden Fassungen.

**Der Weg weicht in einer Nebensache ab, die Wirkung nicht.** Dieser Datensatz schlug vor, die zwölf Zeilen aus `crates/krk-core/tests/verzeichnis.rs` zu übernehmen; genommen ist die Fassung, die bei der Behebung von `260810-1247` schon in `vorschaumodell.rs` stand.

**Zwei Dinge bleiben und sind nicht mit geschlossen.** Erstens: die beiden Fehlschläge vom 260809 um 11:00 sind bis heute nur mit dem naheliegendsten Verdacht erklärt, dass `fs::write` die Zieldatei vor dem Schreiben kürzt, und nicht mit einer belegten Ursache. Dieser Datensatz sagt es selbst — bleibt der Fehler nach dem Umbau, war es etwas anderes. Zweitens: dass dieselbe Bauform zwölfmal getrennt im Baum steht, unter zwei Namen, ist ein anderer Gegenstand und läuft weiter als `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1330_o_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.

Geschlossen vom `reconciler` im Abschluss-Abgleich, nicht in der Sitzung, die den Code geändert hat: der Marker war der einzige im ganzen Bestand, der nach dieser Sitzung nicht mehr zum Code passte.

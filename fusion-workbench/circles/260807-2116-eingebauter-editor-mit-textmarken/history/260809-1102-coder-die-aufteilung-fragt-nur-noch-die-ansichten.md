# Die Aufteilung fragt nur noch die Ansichten

- Agent: `coder`
- Datum: 260809-1102
- Anlass: `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260808-1413_c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md` und `.../260808-1413_c_breite-aendern-traegt-einen-auffangzweig-ueber-bereich-und-hat-den-fuenften-wert-geschluckt.md`
- Status: Complete

## Der Befund und die gewählte Lösung

Der eigentliche Befund war nicht, dass `auslegen` einen Bereich mitzählt, den es
anschließend überspringt. Er war, dass zwei Stellen dieselbe Frage verschieden
beantworteten: `sichtbar_im` aus dem Modell, `gemessene_sichtbarkeit` aus den
Unteransichten — und beide speisten dasselbe `auslegen`. Die Zählung stand
deshalb je nach Auslöser anders da als die Schleife darunter.

**Gewählt: die Aufteilung hat einen Erzeuger dieser Antwort, und es sind die
Unteransichten.** `auslegen` nimmt keine `&Sichtbarkeit` mehr entgegen, sondern
ruft `gemessene_sichtbarkeit(teiler)` selbst. `Aufteilung::anwenden` schreibt
den Wunsch des Modells vorher mit `setHidden` in die Ansichten und liest ihn von
dort zurück; `neu_auslegen` hatte ihn ohnehin nur von dort. Der Umweg über die
Ansicht kostet eine Zeile und ist der Preis dafür, dass die Frage nur eine
Antwort hat.

Die gemeinsame Wurzel darunter: der Ausdruck
`bereichsansicht(...).is_some_and(|a| !a.isHidden())` stand dreimal
ausgeschrieben da — in `gemessene_sichtbarkeit`, `grenze_links` und
`grenze_rechts` — und `auslegen` fragte als vierte statt der Ansichten das
Modell. Er steht jetzt einmal, als `steht_im(teiler, bereich)`. Der
`is_some`-Teil ist die Aussage selbst und keine Vorsichtsmaßnahme: ein Bereich,
dessen Unteransicht die Aufteilung nicht trägt, steht nicht im Fenster.

`sichtbar_im` bleibt und ist jetzt nur noch, was es immer war: die Abbildung
`Bereich` → Feld in `Sichtbarkeit`. Es beantwortet nicht mehr, ob ein Bereich
steht.

## Warum nicht die beiden im Defekt genannten Wege

**Nicht `Fenstermodell::aus_sitzung`.** Der Weg deckt den Tastaturweg ab S5/S6
nicht ab — `fokus_holen` blendet den Bereich ein, bevor `fokus_setzen` abweist.
Und "ein Editor ohne gehaltene Datei ist nicht sichtbar" ist eine Aussage, die
`fenstermodell.rs` heute nicht treffen kann: es kennt keine Datei.

**Nicht die leere fünfte Unteransicht.** S16 reicht die fünfte Ansicht von außen
herein (`bauen` nimmt sie entgegen, gebaut wird sie in `anwendung.rs`); ein
Platzhalter wäre ein zweites Stück zum Entfernen. Und er nähme den
Dateifenstern 460 Punkte, um nichts zu zeigen — derselbe Verlust wie im Defekt,
nur absichtlich. Der Widerspruch wäre außerdem nur zugedeckt: `sichtbar_im` und
`gemessene_sichtbarkeit` stimmten dann zufällig überein, und der nächste Bereich
vor seiner Unteransicht risse ihn wieder auf.

## Der zweite Defekt

`breite_aendern` führte das Gegenüber über einen `match` mit `_ => Bereich::Links`.
Der `match` ist nicht richtig ausgeschrieben, sondern **entfallen**:

```rust
if let Some(seite) = bereich.seite() {
    let anderer = Bereich::von_seite(seite.andere());
```

Dazu kam `Bereich::seite() -> Option<Fensterseite>`, die Umkehrung von
`Bereich::von_seite`, als vollständige Fallunterscheidung ohne Auffangzweig.
`ist_beweglich` zählt seitdem nicht mehr selbst auf, sondern lautet
`self.seite().is_some()`: beweglich ist ein Bereich genau dann, wenn er ein
Dateifenster ist. Die Aufzählung steht an einer Stelle statt an dreien.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/aufteilung.rs`
- `crates/krk-ui/src/fenstermodell.rs`

## Proben

Zwei neu in `fenstermodell.rs`:
`beweglich_ist_genau_ein_dateifenster_und_die_zuordnung_laeuft_in_beide_richtungen`
und `ein_fester_bereich_aendert_nur_seine_eigene_breite`. Die Aufteilung selbst
erreicht keine Probe: sie braucht eine `NSSplitView` und einen
`MainThreadMarker`; die Abnahme ist Nutzerarbeit am laufenden Bündel.

## Abnahme

`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` laufen
grün; 589 Proben bestanden, keine gescheitert, eine übersprungen.

## Was offen bleibt

Die Aufteilung führt den Editor bis S16 als nicht stehend. Das Modell führt ihn
weiter als sichtbar, wenn `session.toml` es sagt oder ab S5/S6 ein Tastendruck
es setzt — der Nutzer sieht dann nichts, und das ist die richtige Auskunft,
solange es nichts zu sehen gibt. **S16 löst das ohne weitere Änderung an
`auslegen` auf:** sobald die fünfte Unteransicht hängt, liefert `steht_im` für
den Editor `true`, und Zähler wie Zuteilung nehmen ihn auf.

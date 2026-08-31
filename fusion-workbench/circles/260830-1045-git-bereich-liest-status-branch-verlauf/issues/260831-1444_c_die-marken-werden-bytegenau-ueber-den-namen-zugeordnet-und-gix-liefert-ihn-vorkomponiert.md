Die Marken werden bytegenau über den Namen zugeordnet, und `gix` liefert ihn vorkomponiert

---
`Ordnermodell::gitmarken_setzen` (`crates/krk-core/src/verzeichnis/modell.rs:1260-1281`) baut ein Nachschlagewerk über `eintrag.name.as_str()` und schlägt den gemeldeten Namen bytegenau nach:

```rust
let stellen: HashMap<&str, usize> = self.eintraege.iter().enumerate()
    .map(|(index, eintrag)| (eintrag.name.as_str(), index)).collect();
…
let Some(index) = stellen.get(name.as_str()) else { continue; };
```

Die beiden Seiten stammen aus verschiedenen Quellen. Der Bestand kommt aus dem Verzeichnisleser, also unverändert aus `readdir`. Der gemeldete Name kommt aus dem Statusstrom von `gix` und läuft über `eintragsname` (`git/leser.rs:344-367`); `gix` wendet auf den Verzeichnisdurchlauf `core.precomposeUnicode` an (`gix-0.87.1/src/dirwalk/mod.rs:79`, `dirwalk/options.rs:9`), und `git` selbst setzt den Schlüssel auf macOS ab Werk auf `true`, schreibt also vorkomponierte Namen in den Index.

Ein Eintrag, dessen Name auf der Platte zerlegt vorliegt (NFD, etwa nach einem Entpacken oder einer Übertragung von einem älteren Dateisystem), trägt damit in KRKs Bestand eine andere Bytefolge als im Befund und bekommt **keine Marke**; der Eintrag zählt auch nicht in der Zusammenfassung mit, die aus derselben Liste rechnet.

Das ist derselbe Vergleich, den der offene Defekt `shared/issues/260826-1221_*_die-kollisionspruefung-vergleicht-bytegenau-und-uebersieht-jede-kollision-in-schreibweise-und-normalform.md` an der Konfliktprüfung führt, an einer zweiten Stelle.

**Abnahmetest:** ein Prüfrepository mit einer geänderten Datei, deren Name ein zerlegtes Zeichen trägt (etwa `U+0055 U+0308` statt `U+00DC`); `marken` und `gitmarken_setzen` zusammen setzen die Marke an die Zeile dieses Eintrags. Die Probe `die_fuenf_zustaende_tragen_ihre_fuenf_buchstaben` (`crates/krk-core/tests/git.rs:345`) arbeitet heute mit reinen ASCII-Namen und kann den Fall nicht sehen.

**Resolved:** 260831. `Ordnermodell::gitmarken_setzen` fragt jetzt zweimal: erst bytegenau wie
bisher, und erst bei einem Fehlschlag über ein zweites Nachschlagewerk, dessen Schlüssel
`verzeichnis::kollation::namensschluessel` baut. Zwei solche Schlüssel sind genau dann gleich, wenn
die Namen kanonisch gleich sind.

**Nichts Neues gebaut, wo etwas dastand.** `verzeichnis/kollation.rs` ist die eine Stelle, an der
`krk-core` Unicode über den Bytevergleich hinaus befragt, und `icu_collator` ist die eine fremde
Unicode-Kiste des Kerns; der neue Schlüssel ist derselbe Kollator mit `Strength::Identical`, die
laut Kistenquelle die NFD-Form an den Schlüssel anhängt. **Keine Abhängigkeit kommt hinzu**, keine
zweite Normalisierungsstelle steht neben der ersten.

**Zwei Eigenschaften, die die Bauform trägt.** Der bytegenaue Treffer geht vor, weil APFS beide
Schreibweisen desselben Namens nebeneinander hält und der Befund dann der Zeile gehört, deren
Bytes er trägt. Und das zweite Werk entsteht **erst beim ersten Fehlschlag**: ein Ordner, dessen
Namen alle bytegenau treffen, zahlt keinen einzigen Kollationsschlüssel.

**Gemessen und nicht mehr gelesen.** Die neue Probe `ein_zerlegt_benannter_eintrag_bekommt_seine_marke`
(`crates/krk-core/tests/git.rs`) legt `U+0055 U+0308 bung.txt` an, committet und ändert sie; sie hält
vor dem eigentlichen Vergleich beide Voraussetzungen ausdrücklich fest — die Platte trägt den
zerlegten Namen, `gix` meldet den vorkomponierten — und beide standen im Lauf. Gegen den alten Stand
scheiterte sie an „kein einziger Befund ist eingetragen worden"; die Vermutung des Datensatzes ist
damit bestätigt. Daneben stehen drei Proben ohne `git`:
`ein_zerlegt_benannter_eintrag_bekommt_den_vorkomponierten_befund` und
`bei_zwei_schreibweisen_gewinnt_der_bytegenaue_treffer` in `verzeichnis/modell.rs`, dazu
`der_namensschluessel_zieht_genau_die_kanonisch_gleichen_namen_zusammen` und
`der_sortierschluessel_zieht_zusammen_was_der_namensschluessel_trennt` in `verzeichnis/kollation.rs`;
die letzte begründet, warum der Nachschlag nicht den Sortierschlüssel nimmt.

**Die zweite Stelle bleibt offen.** `shared/issues/260826-1221_*_die-kollisionspruefung-vergleicht-bytegenau-…`
führt denselben Vergleich an der Konfliktprüfung; sie ist nicht Gegenstand dieses Auftrags und
nicht mitgezogen. Wer sie behebt, hat den Schlüssel jetzt dastehen.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23. **Gelesen und nicht gemessen:** die Vorkomposition steht in der Quelle von `gix` und in der Voreinstellung von `git`, ein Lauf gegen eine zerlegt benannte Datei ist nicht gefahren. Der Abnahmetest oben entscheidet die Frage.

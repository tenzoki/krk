Die Proben des Vorschaumodells legen ihre Ordner unter festen Namen an

---

Sieben Proben in `crates/krk-ui/src/vorschaumodell.rs` legen ihren Pruefordner unter einem festen Namen im Temporaerverzeichnis an, etwa `krk-vorschau-probe-gross`. Zwei gleichzeitige Laeufe derselben Kiste schreiben damit in denselben Ordner, und keine der sieben raeumt ihn wieder ab: die Dateien bleiben nach dem Lauf stehen, eine von ihnen mit 1 MB.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung des Defekts `260810-1247`
**Betroffen:** `crates/krk-ui/src/vorschaumodell.rs`
**Domain:** code

## Belegstellen

Die sieben Stellen tragen alle dieselbe Form:

```rust
let ordner = std::env::temp_dir().join("krk-vorschau-probe-gross");
std::fs::create_dir_all(&ordner).expect("Probenordner");
```

Betroffen sind die Proben `eine_textdatei_erscheint_mit_ihrem_inhalt`, `ein_ordner_erscheint_als_metadaten`, `eine_textdatei_ueber_der_grenze_faellt_auf_die_metadaten`, `ein_bild_unter_der_grenze_kommt_mit_seinen_bytes`, `ein_bild_ueber_der_grenze_faellt_auf_die_metadaten`, `keine_utf8_datei_faellt_auf_die_metadaten` und `das_laden_erreicht_den_tab_der_es_bestellt_hat`.

## Fehlszenario

Zwei `cargo test`-Laeufe auf demselben Geraet, wie sie bei parallel arbeitenden Agenten vorkommen, treffen sich in `krk-vorschau-probe-gross`. Der eine Lauf schreibt `gross.txt` mit `TEXTGRENZE + 1` Bytes, der andere liest sie; loescht einer der beiden den Ordner, sieht der andere `laden` auf einen fehlenden Pfad und bekommt `Inhalt::Hinweis` statt `Inhalt::Metadaten`. Ein Fehlschlag daraus benennt nichts, was am Code falsch waere.

## Vorgeschlagene Behebung

Der `Pruefordner` derselben Datei, angelegt bei der Behebung von `260810-1247`, traegt Prozesskennung und Laufnummer im Namen und raeumt sich in `Drop` ab; er ist dieselbe Bauform wie `Pruefordner` in `crates/krk-core/tests/verzeichnis.rs`. Die sieben Proben darauf umzustellen ist eine mechanische Aenderung ohne Aussagewechsel.

**Nicht in derselben Aenderung behoben**, weil die Behebung von `260810-1247` an einem Diff gemessen wird, der die Umstellung des Lesewegs zeigen soll, und sieben umgeschriebene Proben daneben genau diesen Blick verstellen.

## Zustaendigkeit

`coder`.

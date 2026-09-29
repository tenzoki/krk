Vier Doc-Kommentare der Quelltextproben in `anwendung.rs` hängen am Modul `sprungproben`
---
Vor `mod sprungproben` stehen vier Doc-Blöcke ohne Leerzeile hintereinander: der des Angleichens (Runde 13), der der Zoomproben (Runde 20), der der Blätterprobe und der der Sprungprobe. Rust hängt alle vier an `sprungproben`. `zoomproben` und `blaetterproben` stehen ohne Doc-Kommentar da, und die Aussage „Was sie nicht sieht“ der Zoomprobe beschreibt in `cargo doc` die Sprungprobe.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code

## Befund

- `crates/krk-ui/src/appkit/anwendung.rs`, die Zeilen unmittelbar vor `#[cfg(test)] mod sprungproben`: vier `///`-Blöcke, jeder beginnt direkt unter dem Ende des vorigen.
- `mod blaetterproben` (Zeile 11760) und `mod zoomproben` (Zeile 11781) tragen nur `#[cfg(test)]`.
- Vorher bestand dasselbe Muster zu zweit: an `52be37a` hing der Block des Angleichens schon am Block der Zoomprobe vor `mod zoomproben` (dort Zeile 11607). Die Bildfolge (`75468b7`, `5618322`) hat zwei weitere Blöcke davorgeschoben, statt sie an ihr Modul zu setzen.

`cargo doc` meldet das nicht, und keine Probe liest es.

## Richtung

Jeden Block über sein eigenes `#[cfg(test)] mod …` setzen; der Angleichen-Block gehört zu `mod angleichproben` (Zeile 11825), das heute ebenfalls ohne Doc-Kommentar dasteht.

## Abnahme

- Vor jedem der Module `angleichproben`, `zoomproben`, `blaetterproben` und `sprungproben` steht genau der Doc-Block, der von ihm spricht.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` bleibt grün.

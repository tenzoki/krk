Der Doc-Kommentar von `Ablage::pfad` nennt vier Dateien, die Aufzählung führt sechs
---
`crates/krk-core/src/ablage/mod.rs` beschreibt `Ablage::pfad` und `Zugang::pfad` an drei Stellen als „Der Pfad einer der vier Dateien" (Zeilen 45, 427 und 468). Beide Methoden nehmen ein beliebiges `Datei` entgegen, und `Datei::ALLE` in `crates/krk-core/src/ablage/pfade.rs` führt seit der Runde 9 sechs Werte.
---
Der Befund ist von den Stellen zu trennen, an denen „vier" richtig steht. `Zugang::laden` und `Zugang::sichern` sind der TOML-Weg und betreffen wirklich nur die vier TOML-Dateien; ihre Doc-Kommentare bleiben, wie sie sind. Falsch sind allein die drei Stellen zu `pfad`, die über jede Ablagedatei sprechen.

Gefunden beim Lesen des Ablagemoduls für die Planung der zwölften Runde, nicht durch deren Directive verursacht: die Abweichung stammt aus der Runde 9, die die Ablage von vier auf sechs Dateien gebracht hat. Deshalb liegt der Datensatz im gemeinsamen Speicher.

Die zwölfte Runde bringt die siebte Ablagedatei und zieht dabei nach ihrem Abnahmekriterium C2.1 jede Prosastelle nach, die heute **sechs** nennt. Die drei Stellen hier nennen vier und fallen aus jenem Kriterium heraus; wer sie beim Nachziehen mitnimmt, hat den Befund erledigt, und wer nicht, findet ihn hier wieder.

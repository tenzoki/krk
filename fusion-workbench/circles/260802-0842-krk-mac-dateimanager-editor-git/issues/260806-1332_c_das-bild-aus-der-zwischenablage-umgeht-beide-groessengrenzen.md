Das Bild aus der Zwischenablage umgeht beide Größengrenzen

---

`inhalt_lesen` (`crates/krk-ui/src/appkit/zwischenablage.rs:87-98`) liest die
PNG- oder TIFF-Daten der Zwischenablage über `daten.to_vec()` vollständig in
den Speicher, ohne jede Grenze. `Vorschaumodell::zwischenablage_anzeigen`
(`crates/krk-ui/src/vorschaumodell.rs:350-365`) übernimmt sie unbesehen. Die
Bildgrenze aus `fd5e3c5` greift allein im Dateiweg (`laden`,
`vorschaumodell.rs:471`).

---

**Warum das auffällt.** Der Modulkopf von `vorschaumodell.rs` schreibt seit
`fd5e3c5`: "Beide Grenzen sind dieselbe Regel mit zwei Zahlen." Es gibt aber
einen dritten Weg in dieselbe Anzeigefläche, und der trägt keine der beiden
Zahlen. Ein in einem Bildbearbeitungsprogramm kopiertes TIFF liegt ohne
Weiteres über 100 MB; `shift+f3` legt es dann als Ganzes in den Arbeitsspeicher
des Referenzgeräts von 2018 — genau die Wirkung, gegen die die Bildgrenze
gebaut wurde.

**Warum es leichter wiegt als der behobene Defekt.** Der Nutzer hat das Bild
selbst kopiert und drückt selbst `shift+f3`; es ist keine Nebenwirkung einer
Zeilenbewegung. Die Daten liegen zudem bereits im Pasteboard-Server, die Kopie
verdoppelt also einen Verbrauch, der schon da ist, statt ihn zu erzeugen.

**Der Fix.** `inhalt_lesen` fragt vor `to_vec()` die Länge des `NSData` ab und
gibt oberhalb von `BILDGRENZE` einen Hinweis statt der Bytes zurück — dieselbe
Zahl aus `vorschaumodell.rs`, keine zweite. Der Rückfallweg steht schon da:
`Inhalt::Hinweis` ist der Fall, den `zwischenablage_anzeigen` für die leere
Zwischenablage bereits benutzt.

Der Fix hängt an der Klärung der Zahl selbst, siehe
`260806-1329_o_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md`.

**Betrifft:** `krk-ui` (`appkit/zwischenablage.rs`, `vorschaumodell.rs`). C6 und
C10. Keine Zeitzusage aus C8 berührt.

---
Resolved: Die Länge des NSData wird vor to_vec() gegen BILDGRENZE geprüft; oberhalb erscheint ein Hinweis statt der Bytes. Der Weg über die Zwischenablage trägt damit dieselbe Grenze wie der über die Datei.

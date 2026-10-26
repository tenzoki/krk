Der Ladeplatz der Vorschau bleibt über das Lesen hinaus belegt, bis Markdown und Trenntext aufbereitet sind
---
`laden` hält die `Belegung` bis zum Ende der Funktion und nicht bis zur Rückkehr des Lesens; der Plan und zwei Kommentare sagen das Gegenteil.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `261025-1444-was-heisst-abbrechen-fuer-einen-download-den-die-vorschau-schon-angestossen-hat.md`

**Befund.** In `crates/krk-ui/src/vorschaumodell.rs` (`fn laden`, Stand `0650dd47`) entsteht der Wächter als `let _belegung = if platzhalter { … Some(umstand.regel.ladeplatz.belegen(umstand.weiter)?) } else { None };` und lebt bis zum Ende von `laden`. Zwischen Lesen und Ende stehen in den Armen von `match leseart` die Aufbereitungen: `text_als_inhalt` (darin `markdown::rendern` bis 1 MB), `jsonzeilen_aus_bytes`, `endzeichen_der_datei` und `trenntext_aus_bytes`. Keine davon fragt `weiter`.

Der Kommentar über dem Wächter sagt „Der Waechter haelt den Platz bis zum Ende dieser Funktion, also bis das Lesen darunter zurueckgekehrt ist“; der Modulkopf (`# Eine Datei, die nur in der Cloud liegt`) sagt „allein das Lesen haelt den Ladeplatz“; der Plan sagt in `## Approach`, Schritt 5: „Den Platz gibt er frei, wenn sein Lesen zurückgekehrt ist.“ Alle drei stimmen mit dem Bau nicht überein.

**Folge.** „Höchstens einer zugleich“ hält weiter. Die nächste Cloud-Vorschau wartet aber zusätzlich auf die Aufbereitung des vorigen Fadens, auch eines überholten, dessen Marke erst nach seinem letzten Lesestück gesetzt wurde. `inference:` bei einer Markdown-Datei nahe 1 MB sind das Zehntel von Sekunden; gemessen ist es nicht.

**Richtung.** Die Bytes in einem eigenen Schritt lesen, die Belegung danach fallen lassen und erst dann aufbereiten; oder die Kommentare und den Plan auf das Verhalten ziehen, wenn das Halten gewollt ist.

**Abnahme.** Eine Probe in `vorschaumodell.rs` mit eingespritzter Platzhalterfrage zeigt, dass der Ladeplatz frei ist, sobald das Lesen zurückgekehrt ist, und nicht erst nach der Aufbereitung; oder die drei Textstellen sagen ausdrücklich, dass der Platz bis zum Ende der Aufbereitung gehalten wird, und warum.

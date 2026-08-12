`merkzeichen_einloesen` kostet bei tiefer Verschachtelung das Zweieinhalbfache, und L7 wird jetzt schon bei 12 kB verfehlt

---

Der Nachzug in `Zerlegung::merkzeichen_einloesen`
(`crates/krk-ui/src/markdown.rs:679-695`) läuft für jedes eingelöste
Merkzeichen ein zweites Mal über alle inneren offenen Einträge. Bei einer
tief verschachtelten Liste ist die Zahl der inneren Einträge selbst die
Verschachtelungstiefe, und die Gesamtarbeit wächst quadratisch mit ihr. Die
Vorfassung hatte einen einzigen Durchlauf ohne inneren Nachzug.

Die Zusage L7 aus C8 der Runde 1 — „Vorschau einer Textdatei bis 1 MB
sichtbar", 100 ms — wird dadurch bei rund 12 kB Quelle verfehlt, wo die
Vorfassung sie bis rund 19 kB hielt.

---

**Gemessen.** `markdown::rendern` aus `1e4e01f` und aus `2c0b2a6`, beide
unverändert in dasselbe Prüfprogramm kopiert, `pulldown-cmark 0.13.4`,
Profil `release`, Median aus 15 Läufen nach zwei Aufwärmläufen, dieselbe
Maschine im selben Lauf. Quelle: `"- "` mal Tiefe, dahinter `"x\n"`.

| Tiefe | Quelle | `1e4e01f` | `2c0b2a6` | Faktor |
|---|---|---|---|---|
| 1 000 | 2 kB | 0,80 ms | 1,72 ms | 2,2 |
| 4 000 | 8 kB | 26,6 ms | 40,9 ms | 1,5 |
| 6 000 | 12 kB | 38,0 ms | **95,5 ms** | 2,5 |
| 8 000 | 16 kB | 69,1 ms | **163,8 ms** | 2,4 |
| 10 000 | 20 kB | **113,4 ms** | **253,7 ms** | 2,3 |
| 20 000 | 40 kB | 460,9 ms | 1 075,5 ms | 2,3 |

Fett steht, was die 100 ms von L7 überschreitet. Die Grenze wandert von
rund 9 500 Ebenen (19 kB) auf rund 6 100 Ebenen (12 kB).

**Die Vorschau nimmt solche Dateien an.** `TEXTGRENZE` in
`crates/krk-ui/src/vorschaumodell.rs:121` steht bei 1 MiB; eine Datei aus
lauter `"- "` erreicht dort rund eine halbe Million Ebenen. Der Durchgang
läuft auf dem Arbeitsfaden `krk-vorschau`, friert die Oberfläche also nicht
ein, aber die Vorschau steht nicht, und ein Kern rechnet.

**Die Gegenprobe: flache Listen sind schneller geworden.** Über
`"- x\n"` mal 20 000 (80 kB) kostet `1e4e01f` 9,64 ms und `2c0b2a6` 8,94 ms.
Auch auf der 1,05-MiB-Musterquelle des Turns ist die neue Fassung schneller
(29,3 ms gegen 25,9 ms im selben Prüfprogramm). Der Befund betrifft allein
die Tiefe, nicht die Länge — und die Messung des Turns hat nur die Länge
gemessen.

**Die Ursache.**

```rust
for stufe in 0..self.offen.len() {
    let (bis_hierhin, dahinter) = self.offen.split_at_mut(stufe + 1);
    let Some(merkzeichen) = bis_hierhin[stufe].merkzeichen.take() else { continue; };
    …
    for eintrag in dahinter {
        if eintrag.anfang == vorher { eintrag.anfang = self.stelle; }
    }
}
```

Der innere Nachzug läuft je eingelöstem Merkzeichen über den ganzen Rest des
Stapels. Über ein Dokument summiert sich das auf ein Quadrat der Tiefe. Die
Komplexitätsklasse des Ganzen ändert sich dadurch nicht — `Zerlegung::tiefe`
und `Zerlegung::absetzen` laufen ebenfalls je Ereignis über `offen`, also war
der Durchgang schon vorher quadratisch in der Tiefe —, wohl aber der Faktor,
und zwar um das Zweieinhalbfache.

**Ein Zuschnitt** (nicht gewählt): `Zerlegung::offen` ist nach `anfang`
aufsteigend sortiert, denn `oeffnen` schreibt `self.stelle`, und `stelle`
wächst nur; `absetzen` und `merkzeichen_einloesen` heben ausschließlich
Einträge an, die schon auf dem Höchstwert stehen. Die Einträge mit
`anfang == self.stelle` bilden damit stets ein Endstück von `offen`, und
dessen Anfang wandert im Verlauf der Schleife nur nach rechts. Ein einziger
mitgeführter Index statt des inneren Durchlaufs macht den Nachzug linear in
der Tiefe. Zu prüfen wäre, ob die Sortierung als Zusage irgendwo festzuhalten
ist, damit der nächste Leser sie nicht neu herleiten muss.

**Gewicht: mittel.** Kein Absturz und keine falsche Auskunft über den Inhalt.
Eine Liste mit 6 000 Ebenen ist keine Quelle, die jemand schreibt — aber die
Datei kommt vom Nutzer, die Vorschau nimmt sie an, und eine benannte
Zeitzusage wird jetzt bei einer Datei von 12 kB verfehlt statt bei einer von
19 kB. Der Turn berichtet einen Geschwindigkeitsgewinn; er gilt für die eine
gemessene Quelle, und der schlechteste Fall ist ungemessen in die andere
Richtung gegangen.

**Herkunft:** Circle der Runde 6, Turn 5, `2c0b2a6`.

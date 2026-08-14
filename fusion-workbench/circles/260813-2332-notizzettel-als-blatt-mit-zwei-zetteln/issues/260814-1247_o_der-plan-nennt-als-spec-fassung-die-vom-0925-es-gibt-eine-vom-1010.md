Der Plan nennt als Spec-Fassung die vom 260814-0925; es gibt eine vom 260814-1010

---

Der Kopf des Plans führt:

> **Spec:** `circles/260813-2332-…/planning/260813-2348_o_spec-…md` (Fassung vom 260814-0925,
> mit dem Nachtrag an C4)

Der Spec trägt seit `a6098d9` einen dritten Nachtrag vom 260814-1010 an C5 und am Abschnitt zu
den zehn Zeitzusagen: drei neue Abnahmekriterien, drei neue Festlegungen, eine geänderte Zeile.
Der Plan ist mit dem Nachtrag nicht mitgezogen worden; er steht seit dem 260814-1004 auf `_c_`
mit Status Complete.

---

**Schwere:** niedrig. Der Plan ist geschlossen, und die drei neuen Kriterien sind gebaut — nur
eben nicht über einen Planschritt, sondern über die Behebung des Defekts `260814-0910_c_` in
Turn 3. Am Baum belegt: `Zugang::beiseite_legen` mit `quelle.by_ref().take(EDITORGRENZE)`
(`crates/krk-core/src/ablage/mod.rs:720`), `Beiseite::Gekuerzt` (`:261`) und die zwei Proben
`eine_zu_grosse_zetteldatei_wird_nicht_geladen_und_geht_gekuerzt_beiseite` und
`eine_zetteldatei_genau_auf_der_grenze_geht_ganz_beiseite`
(`crates/krk-core/tests/ablage.rs:1587`, `:1644`).

**Warum es trotzdem aufgeschrieben ist.** Die Fassungszeile im Plankopf ist die Stelle, an der
eine spätere Runde nachsieht, gegen welchen Stand der Plan geschrieben wurde. Wer ihr folgt,
liest C5 ohne die Kopiergrenze und hält den `Gekuerzt`-Zweig für einen Zusatz ohne Zusage.

**Was zu tun ist.** Die Fassungszeile auf 260814-1010 ziehen und in einem Halbsatz vermerken,
dass die drei Kriterien des dritten Nachtrags außerhalb der sechzehn Planschritte gebaut sind.

**Kontext**

- Gefunden beim zweiten Abgleich der Runde 9, `history/260814-1247-reconciliation.md`.

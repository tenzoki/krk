C6.5, A5 und Planschritt 6 sagen weiter „über 2.000", und kein offener Datensatz trägt diese Hälfte

---

Der Befund `260824-1215_c_die-abgeschnittene-zaehlung-zeigt-ueber-treffer-und-c6-5-verlangt-ueber-2000.md`
nennt zwei Punkte und ist auf `_c_` geschlossen. Seine `Resolved:`-Notiz sagt selbst: **„Punkt 2
bleibt offen und gehört nicht hierher."** Punkt 2 ist seither an keiner Stelle nachgezogen, und
kein offener Datensatz trägt ihn. Er fällt damit aus jeder Suche nach offener Arbeit heraus.

---

## Der Stand am 260824-1645

Gebaut (`crates/krk-core/src/leseprofil/mod.rs:566-580`):

```rust
Wert::UeberGrenze(gezaehlt) => {
    format!(
        "mindestens {gezaehlt} (Lesung bei {HOECHSTENS_EINTRAEGE} Einträgen abgebrochen)"
    )
}
```

Am Bestand nachgelesen, drei Prosastellen sagen etwas anderes:

| Stelle | Wortlaut |
|---|---|
| Spec, C6.5 (Zeile 309) | „zeigt „über 2.000" statt einer Zahl" |
| Spec, Festlegung A5 (Zeile 86) | „Über der Grenze zeigt eine Zählung „über 2.000" statt einer Zahl." |
| Plan, `### Was eine unvollständige Lesung sagen darf` (Zeile 198) | „Die Zählung zeigt `über 2.000` statt einer Zahl." |

Die vierte Stelle, Planschritt 6 (Zeile 319), nennt nur noch den Klammerausdruck
„(`über 2.000`)" als Beispiel für einen Satz, der aus der Konstanten entsteht; sie ist
mitzuziehen.

Beide Planungsdateien sind **nach** dem Schluss des Befundes bearbeitet worden — die Datei
`260824-0613_o_spec-…` zuletzt um 15:41, die Datei `260824-0640_o_plan-…` um 16:32, der Befund
ist um 12:43 geschlossen. Die Buchführung ist also nicht bloß noch nicht drangewesen, sondern
in zwei Durchgängen daran vorbeigegangen.

## Warum das zählt

Der Spec ist am 260824-0625 vom Nutzer freigegeben, und er ist die Liste, an der die Abnahme
läuft. C6.5 steht heute so da, dass der Nutzer beim Abhaken eine Anzeige suchen wird, die es
nicht gibt. Sieben andere Kriterien dieser Runde tragen für genau diese Lage eine
**Berichtigung unter ihrer Liste** (C3.8, C3.9, C3.14, C4.3, C5.2, C5.3, C5.6); C6.5 ist das
einzige geänderte Kriterium ohne eine.

**Der Bau ist nicht der Gegenstand.** Die gebaute Fassung ist sachlich richtiger als die
geplante, und der geschlossene Befund schreibt das aus. Offen ist allein die Buchführung.

## Was zu tun ist

Eine Berichtigung unter der Kriterienliste von C6, in der Form der übrigen sieben: welcher
Wortlaut galt, was gebaut ist, warum, und der Verweis auf
`issues/260824-1215_c_die-abgeschnittene-zaehlung-…`. A5 bleibt im Wortlaut stehen und bekommt
einen Verweis, so wie A7 es für die vierte Zustandszeile bekommen hat. Der Plan zieht an seinen
zwei Stellen nach.

**Der Schreibweise wegen daneben:** der Spec schreibt „2.000" mit Punkt, `als_text` schreibt
`2000` ohne. Die Zahl kommt aus `HOECHSTENS_EINTRAEGE` und soll dort auch herkommen; wer den
Punkt will, formatiert ihn an der einen Stelle in `als_text`.

**Schwere:** mittel. Kein Fehlverhalten, aber ein freigegebenes Abnahmekriterium, das der Bau
nicht erfüllt, ohne dass die Abweichung irgendwo steht.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1645.

**Betroffen:**
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (C6.5, A5),
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Zeilen 198 und 319)

**Domain:** code

---
Resolved:

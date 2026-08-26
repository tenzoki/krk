Die zwei Abbruchwege des Lesefadens behandeln den angefangenen Stapel verschieden

---

`lesen_und_senden` (`crates/krk-core/src/verzeichnis/leser.rs:275-325`) prueft das
Abbruchkennzeichen an zwei Stellen. Die erste schickt den angesammelten Rest noch los, die
zweite laesst ihn fallen. Beide melden danach denselben `Abschluss::Abgebrochen`. Ein Grund
fuer den Unterschied steht nirgends.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/verzeichnis/leser.rs:287-291`, `:311-323`
**Tree state:** `004ff72`
**Domain:** code

## Die zwei Wege nebeneinander

```rust
// leser.rs:288-291 — erster Weg: der Rest geht noch raus
if abbruch.load(Ordering::Relaxed) {
    rest_senden(generation, sender, &mut gesammelt)?;
    return Some(Abschluss::Abgebrochen);
}
```

```rust
// leser.rs:311-314 — zweiter Weg: der Rest bleibt liegen
while gesammelt.len() >= STAPELGROESSE {
    if abbruch.load(Ordering::Relaxed) {
        return Some(Abschluss::Abgebrochen);
    }
```

Am zweiten Weg steht `gesammelt` per Schleifenbedingung bei **mindestens** `STAPELGROESSE`
Eintraegen, also bei mindestens 1.024. Genau die fallen dort weg. Der Kommentar unmittelbar
darueber (`leser.rs:306-310`) begruendet, warum an dieser Stelle ueberhaupt geprueft wird, und
sagt zum Rest nichts.

## Warum das heute nichts kaputt macht

`Abschluss::Abgebrochen` ist dokumentiert als „die bis dahin gesendeten Stapel sind gueltig,
der Bestand ist unvollstaendig" (`leser.rs:60-62`). Beide Ausgaenge halten diese Zusage. Und
der einzige Aufrufer, dessen Abbruch nicht ohnehin den Empfaenger mitnimmt, ist der Messmodus.

## Warum es trotzdem ein Defekt ist und keine Geschmacksfrage

Es sind zwei Stellen, die dieselbe Frage stellen und verschieden antworten, und die
Verschiedenheit ist an keiner von beiden begruendet. Der Modulbaum fuehrt genau diese Sorte
Doppelung als Fehler: `modell.rs:78-84` beschreibt den Pruefschritt, der bis zur Runde 10
zweimal wortgleich dastand, und `filter.rs:16-17` begruendet die Zusammenlegung der drei Regeln
damit, dass zwei Fassungen „bei zwei Fassungen nicht mehr dieselbe Antwort gaeben". Hier sind
es zwei Fassungen mit zwei Antworten.

Praktisch heisst der Unterschied: ein Nutzer, der waehrend eines Lesevorgangs navigiert, sieht
je nachdem, an welcher der beiden Grenzen der Abbruch faellt, bis zu 1.024 Zeilen mehr oder
weniger von einem Ordner, den er ohnehin verlaesst. Nicht sichtbar, aber auch nicht gewaehlt.

## Richtung

Entweder `rest_senden` auch an der zweiten Stelle, oder an beiden nicht — und die Wahl bekommt
eine Zeile Begruendung, so wie die Nachbarstelle sie hat. Fuer „an beiden senden" spricht,
dass der Rest bereits gelesen ist und das Senden nichts kostet als einen Kanalplatz; fuer „an
beiden nicht", dass ein abgebrochener Ordner ohnehin verworfen wird. Die Zusage aus `leser.rs:60-62`
traegt beides.

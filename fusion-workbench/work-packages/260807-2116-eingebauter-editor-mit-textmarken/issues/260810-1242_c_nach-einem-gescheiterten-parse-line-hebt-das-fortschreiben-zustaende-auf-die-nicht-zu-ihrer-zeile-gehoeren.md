Nach einem gescheiterten `parse_line` hebt das Fortschreiben Zustaende auf, die nicht zu ihrer Zeile gehoeren
---
Bricht der Zerleger an einer Zeile ab, setzt `rechnen` `faerben = false` und faerbt nicht weiter — legt aber weiter Haltepunkte an, und zwar mit dem in diesem Augenblick eingefrorenen Zustand. Ein spaeterer Durchgang steigt an einem dieser Haltepunkte ein und faerbt einen Schwanz ein, den ein voller Durchgang nicht einfaerbt. Damit bricht die eine Zusage, an der das Fortschreiben haengt: „von vorn" und „fortgeschrieben" liefern nicht dasselbe.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/hervorhebung.rs`

## Belegstelle

`crates/krk-ui/src/hervorhebung.rs:959-985`, in `rechnen`:

```rust
while nummer < neu.len() {
    let zeile = neu[nummer];
    zeilen.push(Zeilenanfang { byte, utf16: stelle });
    if haltepunkte
        .last()
        .is_none_or(|letzter| nummer - letzter.zeile >= ZUSTANDSABSTAND)
    {
        haltepunkte.push(Haltepunkt {
            zeile: nummer,
            zustand: zustand.clone(),
            stapel: stapel.clone(),
        });
    }

    if faerben {
        match zustand.parse_line(zeile, satz) {
            Err(_) => faerben = false,
            ...
```

Der Haltepunkt entsteht **vor** der Abfrage auf `faerben` und ohne sie. Nach dem Abbruch wachsen `zustand` und `stapel` nicht mehr; jeder weitere Haltepunkt traegt den Zustand der Abbruchzeile und behauptet ihn fuer seine eigene.

Dazu: `ParseState::parse_line` veraendert `self` auf dem Weg zum `Err` bereits (`syntect-5.3.0/src/parsing/parser.rs:223-230` setzt `self.first_line = false` vor dem ersten `?`). Der eingefrorene Zustand ist also nicht einmal der Zustand am Anfang der Abbruchzeile.

## Fehlszenario, gemessen

Nachgebaut in einer Abschrift des Moduls, mit einer eingesetzten Abbruchzeile an Zeile 40 einer Datei von 200 Zeilen, danach eine Aenderung an Zeile 150:

```text
erster Durchgang: eingefaerbt bis Zeichen Some(777) von 4167
fortgeschrieben: 144 Stuecke, eingefaerbt bis Some(3283)
voll:            80 Stuecke, eingefaerbt bis Some(777)
erstes abweichendes Stueck bei Index 80:
  fortgeschrieben: Einfaerbung { anfang: 2583, laenge: 3, ... }
  voll:            None
```

Der volle Durchgang hoert bei Zeichen 777 auf, wie es der Zweig `Err(_) => faerben = false` vorsieht. Der fortgeschriebene steigt am Haltepunkt der Zeile 128 oder 160 ein, dessen aufgehobener Zustand aus Zeile 40 stammt, und faerbt von dort weiter — 64 Stuecke, die der volle Durchgang nicht liefert, und ihre Farben sind gegen den falschen Zerlegerzustand gerechnet.

Der Nutzer sieht Farben, die sich mit der Bearbeitungsgeschichte aendern statt mit dem Text.

## Erreichbarkeit: ungeklaert, und eher gering

Ich habe **keinen** Weg gefunden, `parse_line` mit dem eingebundenen Sprachsatz zum Abbruch zu bringen. Gemessen, beide Male 0 Fehler:

- alle 213 Sprachdefinitionen aus `two_face::syntax::extra_newlines()`, je 25 Probezeilen (offene Zeichenketten, offene Bloecke, NUL, Bildzeichen, sehr lange Zeilen, `\r\n`, Auszeichnungszeichen);
- jede der 213 als Zaun in Markdown (` ```<endung> `), weil `ParsingError::MissingContext` laut `syntect` genau dort entsteht — ein Sprachverweis auf eine Sprache, die der Satz nicht fuehrt (`syntect`-Ausgabe 421).

Der Zweig ist damit heute vermutlich unerreichbar. Er steht im Code, er ist begruendet, und er wird beim naechsten Wechsel der Kistenfassung oder des Sprachsatzes erreichbar oder nicht — das entscheidet dann `two-face`, nicht KRK.

## Vorschlag

Eine Zeile: den Haltepunkt nur anlegen, solange gefaerbt wird.

```rust
if faerben
    && haltepunkte
        .last()
        .is_none_or(|letzter| nummer - letzter.zeile >= ZUSTANDSABSTAND)
{
```

Der Rest traegt schon: die Zeilentafel laeuft weiter (also bleibt `Formatierung::laenge` richtig und die Lieferung wird angenommen), und ein Stand mit leerer Haltepunktliste faellt in `einstieg_finden` als Vorlage aus (`hervorhebung.rs:716-718`), sodass der naechste Durchgang von vorn rechnet — langsam, aber gleich dem vollen.

Dazu eine Probe, die den Abbruch einsetzt statt auf ihn zu warten. Die vorhandene Zusicherung `fortschreiben_gleicht_vollem_durchgang` (`hervorhebung.rs:1635`) deckt 14 Aenderungen ab und keinen Abbruch; ohne eingesetzten Abbruch ist die Zusage an dieser Stelle nicht gemessen.

## Was ausdruecklich **nicht** der Befund ist

Das Fortschreiben ist im Uebrigen gleichwertig zum vollen Durchgang. Gemessen an derselben Abschrift des Moduls:

- 18 000 Laeufe mit zufaelligen Aenderungen (Rust und Markdown, 3 bis 143 Zeilen, sechs Aenderungen je Datei hintereinander, Zeichen mitten in der Zeile, Zeile geteilt, Zeile fort, Zeile dazu): 0 Abweichungen;
- 940 gezielte Faelle rings um jede Haltepunktgrenze in einer Datei mit zwoelf Haltepunkten — Anfuehrungszeichen, geoeffneter und geschlossener Blockkommentar, offene Rohkette, Zeile von 20 000 Zeichen, leere Zeile, Zeile entfernt, Zeile eingefuegt, je an `k*32-2` bis `k*32+2`; Blockkommentare ueber 1, 31, 32, 33, 96 und 167 Zeilen, an einem Ende geoeffnet und geschlossen; Markdown-Zaeune an Haltepunktgrenzen; Schlussumbruch dazu und fort; leerer Text in beide Richtungen: 0 Abweichungen.

Verglichen wurde die Wirkung Zeichen fuer Zeichen, also mit demselben Maßstab wie `wirkung` in den Proben des Moduls.

---
Resolved: Der Stand des Zerlegers liegt in `rechnen` jetzt in einem `Option` und wird nach einem gescheiterten Ruf **fallen gelassen**, statt eingefroren weitergetragen zu werden — der falsche Zustand ist damit nicht verboten, sondern nicht vorhanden. Neuer Typ `Zerlegerstand { zustand, stapel }`, weil das Paar an drei Stellen als Paar gebraucht wird; `Haltepunkt` traegt ihn als ein Feld, `Rest::anschluss` nimmt einen Verweis darauf, und die Marke `faerben` ist fort.

Der Befund reichte dabei eine Stelle weiter, als er geschrieben war: **nicht nur die Haltepunkte** lasen den eingefrorenen Stand, sondern auch `Rest::anschluss`. Ein Treffer dort haengte den eingefaerbten Schwanz der Vorlage an, den ein voller Durchgang nach dem Abbruch nicht mehr einfaerbt — dieselbe gebrochene Gleichheit auf einem zweiten Weg. Der vorgeschlagene `if faerben && …` haette den ersten Weg geschlossen und den zweiten offen gelassen; das `Option` schliesst beide, weil beide einen Stand brauchen, den es nicht mehr gibt.

Der Haltepunkt der Abbruchzeile selbst bleibt stehen: er entsteht vor dem Ruf und traegt den Stand am Anfang dieser Zeile, und ein Durchgang, der dort einsteigt, bricht an derselben Zeile ebenso ab.

Probe wie verlangt, mit eingesetztem Abbruch statt Warten darauf: `das_fortschreiben_haelt_nach_einem_abbruch_der_kiste` (Abbruch an Zeile 40 von 192 ueber `ABBRUCHZEILE` unter `cfg(test)`, eingesetzt in der Huelle `zerlegen`). Sie misst die Haltepunktliste (`[0, 32]` statt sechs Eintraege) und die Zusage an vier Aenderungsstellen in beiden Richtungen. **Beide Haelften fallen gegen die alte Fassung**, gemessen durch vorübergehendes Wiederherstellen der alten Semantik: `left: [0, 32, 64, 96, 128, 160]` und „die Wirkung weicht ab".

Abnahme in einem Pruefbaum auf `HEAD` mit allein dieser Datei geaendert: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`, `cargo fmt -p krk-ui -- --check` je exit 0. Bericht: `history/260810-1305-coder-der-zerlegerstand-faellt-nach-einem-abbruch.md`.

Nicht angefasst und ausdruecklich geprueft: der zweite Fehlerausgang `stapel.apply(befehl).is_err() => break` laesst den Wortartenstapel teilangewandt stehen, bricht die Gleichheit aber nicht — derselbe Eingang liefert denselben teilangewandten Stapel, und ein voller Durchgang erreicht an derselben Zeile dasselbe Paar.

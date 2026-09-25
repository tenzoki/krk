# Der Zerlegerstand fällt nach einem Abbruch, statt aufgehoben zu werden

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1305
**Datei:** `crates/krk-ui/src/hervorhebung.rs` (die einzige)

## Auftrag

Behebung des Befundes 4 der Durchsicht Turn 3,
`issues/260810-1242_o_nach-einem-gescheiterten-parse-line-hebt-das-fortschreiben-zustaende-auf-die-nicht-zu-ihrer-zeile-gehoeren.md`.
Verlangt war ausdrücklich nicht die Behandlung des falschen Zustandes, sondern
eine Form, die ihn unmöglich macht.

## Befund, nachgeprüft

Der Befund trägt. `rechnen` legte den Haltepunkt vor der Abfrage auf `faerben`
an und ohne sie; nach `Err(_) => faerben = false` wuchsen `zustand` und `stapel`
nicht mehr, und jeder weitere Haltepunkt behauptete den eingefrorenen Stand für
seine eigene Zeile. Die Belegstelle in `syntect` habe ich gelesen:
`syntect-5.3.0/src/parsing/parser.rs:229` setzt `self.first_line = false` vor dem
`?` der Zeile 225, der eingefrorene Stand gehört also zu keiner Zeile mehr.

**Der Befund reicht eine Stelle weiter, als er geschrieben ist.** Nicht nur die
Haltepunkte lasen den eingefrorenen Stand, sondern auch `Rest::anschluss`: es
verglich ihn gegen die aufgehobenen Stände der Vorlage. Ein Treffer dort hängte
den eingefärbten Schwanz der Vorlage an, den ein voller Durchgang nach dem
Abbruch nicht mehr einfärbt — dieselbe gebrochene Gleichheit auf einem zweiten
Weg. Der Vorschlag des Datensatzes (eine Zeile, `if faerben && …`) schließt den
ersten Weg und den zweiten nicht.

## Was gebaut ist

**Der Stand liegt in einem `Option` und wird nach einem Abbruch fallen
gelassen.** Damit sind beide Wege nicht verboten, sondern nicht mehr gehbar: wo
kein Stand ist, entsteht kein Haltepunkt und wird kein Wiederanschluss gesucht.

- Neuer Typ `Zerlegerstand { zustand: ParseState, stapel: ScopeStack }`. Die
  beiden lagen an drei Stellen als Paar (Haltepunkt, Wiederanschluss, laufender
  Durchgang) und stehen jetzt einmal zusammen. `Haltepunkt` trägt ihn als ein
  Feld `stand`, `Rest::anschluss` nimmt einen Verweis darauf statt zweier, und
  der Vergleich ist eine Bedingung statt zweier.
- `rechnen` führt `zerleger: Option<Zerlegerstand>` statt `zustand`, `stapel`
  und `faerben`. Je Zeile wird der Stand mit `take()` herausgenommen und nur im
  Zweig `Ok` wieder eingesetzt; im Zweig `Err` fällt er, und das ist der ganze
  Fehlerzweig. Die Marke `faerben` ist damit fort.
- Der Haltepunkt der Abbruchzeile selbst bleibt stehen, und das ist richtig: er
  entsteht vor dem Ruf und trägt den Stand am Anfang dieser Zeile. Ein Durchgang,
  der dort einsteigt, ruft dieselbe Zeile und bricht ebenso ab.
- `zerlegen` als Hülle um `ParseState::parse_line`, die eine Stelle, an der der
  Prüfcode einen Abbruch einsetzen kann. Im gebauten Programm bleibt nach
  `cfg(test)` allein der Ruf stehen.

## Die Probe, und dass sie den Fehler sieht

`das_fortschreiben_haelt_nach_einem_abbruch_der_kiste` setzt den Abbruch an
Zeile 40 einer Datei von 192 Zeilen (`ABBRUCHZEILE`, `#[cfg(test)]`) und misst
zweierlei: die Liste der Haltepunkte ist `[0, 32]` und nicht sechs Einträge lang,
und die Zusage „von vorn gleicht fortgeschrieben" hält an vier Änderungsstellen
in beiden Richtungen — vor der Abbruchzeile, unmittelbar davor, unmittelbar
danach und weit dahinter.

**Beide Hälften der Probe habe ich gegen die alte Fassung laufen lassen**, indem
ich die alte Semantik (`faerben`-Marke, Stand bleibt stehen) vorübergehend
wiederhergestellt habe. Beide fallen dann:

```text
Haltepunkte    left: [0, 32, 64, 96, 128, 160]   right: [0, 32]
Wirkung        „die Wirkung weicht ab, a.rs: …"
```

Danach die geprüfte Fassung aus der Ablage zurückgeschrieben und die Gleichheit
der Dateien bestätigt.

## Ausdrücklich geprüft und **kein** Befund

Der zweite Fehlerausgang der Zeilenschleife, `stapel.apply(befehl).is_err() =>
break`, lässt den Wortartenstapel mitten in einer Zeile teilangewandt stehen und
rechnet weiter. Das bricht die Gleichheit **nicht**: derselbe Eingang liefert
denselben teilangewandten Stapel, ein voller Durchgang erreicht an derselben
Zeile dasselbe Paar, und ein Einstieg an einem Haltepunkt dahinter gibt den
vollen Durchgang wieder. Unverändert gelassen.

## Abnahme

Der gemeinsame Arbeitsbaum trug zur Zeit der Abnahme fremde, laufende Änderungen
an `crates/krk-ui/src/appkit/editor.rs` (gesperrt, parallel bearbeitet). Deshalb
zweimal gemessen.

Ein Prüfbaum auf `HEAD` (`1472846`) mit **allein** dieser Datei geändert,
bytegleich mit der abgelieferten:

```text
cargo build --workspace                  → exit 0
cargo test --workspace                   → exit 0   (16 Läufe, 0 fehlgeschlagen)
cargo clippy --workspace --all-targets   → exit 0   (0 Meldungen)
cargo fmt -p krk-ui -- --check           → exit 0
```

Der gemeinsame Arbeitsbaum, nachdem die fremde Änderung sich gesetzt hatte:

```text
cargo build --workspace                  → exit 0
cargo test --workspace                   → exit 0
cargo clippy --workspace --all-targets   → exit 0
cargo fmt -p krk-ui -- --check           → exit 1
```

Die eine `fmt`-Abweichung liegt in `crates/krk-ui/src/appkit/editor.rs:1841`
(`let gesucht = modell.suchlauf().map(…)` über drei Zeilen statt einer) und damit
in der gesperrten Datei. Sie ist nicht angefasst worden; `hervorhebung.rs` ist
`fmt`-rein, belegt durch den Prüfbaum.

## Grenzen

Die Datei `crates/krk-ui/src/hervorhebung.rs` ist die einzige geänderte. Die
Behebung reichte nicht darüber hinaus: `Zerlegerstand`, `Haltepunkt` und `Rest`
sind modulintern, und die öffentliche Schnittstelle der Datei ist unverändert.

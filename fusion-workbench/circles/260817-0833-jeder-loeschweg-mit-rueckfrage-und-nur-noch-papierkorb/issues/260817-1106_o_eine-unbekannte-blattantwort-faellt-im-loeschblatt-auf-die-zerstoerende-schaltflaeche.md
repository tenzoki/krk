# Eine unbekannte Blattantwort fällt im Löschblatt auf die zerstörende Schaltfläche

**Datum:** 260817-1106
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 1
**Schwere:** Hoch
**Betrifft:** `crates/krk-ui/src/appkit/blaetter/mod.rs`, `crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs`
**Baumstand:** `472eb81`

## Der Befund

`blaetter/mod.rs:567-580` rechnet die gedrückte Schaltfläche aus dem Rückgabewert von
`NSAlert` zurück und fängt eine unbekannte Antwort auf die **letzte** Schaltfläche ab:

```rust
// Eine unbekannte Antwort gilt als die letzte Schaltflaeche, und
// die ist in jedem Blatt dieser Runde die abbrechende. Lieber
// nichts tun als raten.
let stelle = antworten
    .iter()
    .position(|kandidat| *kandidat == antwort)
    .unwrap_or(antworten.len().saturating_sub(1));
```

Im Löschblatt ist die letzte Schaltfläche die zerstörende. `loeschbestaetigung.rs:98-105`:

```rust
Schaltflaeche::neu("Abbrechen", Taste::Eingabe),
Schaltflaeche::neu(schaltflaeche, Taste::EingabeMitBefehl),
```

und der Rückruf ist `fertig(stelle == 1)`. Eine unbekannte Antwort ergibt `stelle == 1`, also
`bestaetigt == true`, also `Anwendungsdelegierter::loeschauftrag_stellen`.

Das Löschblatt ist die einzige Stelle im Baum, an der die Annahme des Kommentars nicht gilt.
Nachgezählt: `konflikt.rs:86-89` und `ungesichert.rs:90-92` setzen „Abbrechen" ans Ende,
`uebersprungen.rs:41` und `zettel.rs:411` tragen nur eine Schaltfläche.

Dieselbe Datei trägt für dieselbe Frage die entgegengesetzte Vorbelegung:
`Blattgriff.abbruchcode` fällt auf `NSAlertFirstButtonReturn` zurück, also auf die **erste**
Schaltfläche (`mod.rs:599-601`). Für das Löschblatt trifft die eine und die andere nicht.
`abbruchstelle` ist dort `None`, weil keine der beiden Schaltflächen `Taste::Escape` trägt.

## Was nicht belegt ist

Ein erreichbarer Auslöser. Verfolgt und ausgeschlossen: `Blattgriff::abbrechen` und
`abbruchweg` schicken beide `NSAlertFirstButtonReturn`; `performClose:` schließt ein Fenster
mit anhängendem Blatt nicht; `terminate:` beendet den Prozess, ohne den Abschlussblock zu
fahren. Der Befund steht trotzdem hoch, weil die Vorbelegung in die zerstörende Richtung
zeigt und der Spec dieselbe Frage anderswo ausdrücklich andersherum entscheidet
(`shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, „Unentschieden gilt
als laut").

## Richtung

Die Vorbelegung soll die abbrechende Stelle nehmen statt die letzte. Weil das Löschblatt
keine Schaltfläche mit `Taste::Escape` trägt, kann die abbrechende Stelle nicht länger aus
der Escape-Taste abgeleitet werden; entweder gibt das Blatt sie ausdrücklich mit, oder
`mit_schaltflaechen` bekommt eine eigene Angabe dafür. Die beiden widersprechenden
Vorbelegungen in `blaetter/mod.rs` sollten dabei zu einer werden.

Die Entscheidung gehört nach `blaetter/mod.rs` und trifft alle sechs Blätter.

Der neue Nachzug läuft vor dem Fokusumzug und betritt den Ring, den eine Probe offen hält

---

`df8163d` setzt `aufteilung_nachziehen` in `sichtbarkeit_aendern` **vor** die Schleife über
`nach_dem_sichtbarkeitswechsel`. Für das Einblenden ist das begründet und richtig. Für das
**Ausblenden** dreht es die Reihenfolge um: `setHidden(true)` trifft jetzt eine Ansicht, die
den Ersthelfer noch hält, und erst danach zieht `nach_dem_sichtbarkeitswechsel` den Fokus
absichtlich weg. Damit läuft die Rangneuvergabe durch AppKit vor dem gewollten Umzug, und der
Ring, den `fokusnachzugproben` (`anwendung.rs:8043-8080`) ausdrücklich offen hält, wird von
einer neuen Stelle aus betreten.

---

**Gemessen am Baumstand `df8163d`. Nicht am laufenden Bündel bestätigt** — die Abnahme verlangt
KRK im Vordergrund und ist Nutzerarbeit. Was hier steht, ist aus dem Baum gelesen und an einer
Stelle auf eine Annahme über AppKit gestützt, die dieser Baum selbst schriftlich führt.

## Die Umkehrung

`crates/krk-ui/src/appkit/anwendung.rs:4194-4212`, nach `df8163d`:

```rust
let nachher = self.ivars().modell.borrow().sichtbarkeit();
self.aufteilung_nachziehen();                 // <- neu, setzt setHidden
for bereich in Bereich::ALLE {
    if sichtbar_in(&vorher, bereich) != sichtbar_in(&nachher, bereich) {
        self.nach_dem_sichtbarkeitswechsel(bereich);   // <- setzt den Fokus
    }
}
```

`nach_dem_sichtbarkeitswechsel` (`:4222-4252`) enthält:

```rust
if bereich.seite().is_none() && !self.ivars().modell.borrow().sichtbar(bereich) {
    self.fokus_setzen(Fokus::Dateifenster);
}
```

Das ist der gewollte Umzug beim Ausblenden eines Randbereichs. Vor `df8163d` lief er, **bevor**
irgendetwas `setHidden` schrieb — das geschah erst am Ende von `kommando_ausfuehren`. Jetzt
läuft er danach.

## Der Ring, den der Baum selbst führt

`anwendung.rs:8046-8062`, der Doc-Kommentar von
`der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an`:

```
    /// **Der Ring, den diese Probe offen haelt.** `anwenden` setzt `setHidden`,
    /// und eine ausgeblendete Ansicht, die den Ersthelfer haelt, laesst AppKit
    /// den Rang neu vergeben — also `makeFirstResponder:` erneut rufen und
    /// damit die Meldung des Hauptfensters ein zweites Mal ausloesen, ...
    /// Seither haengt
    /// `aktives_dem_ersthelfer_nachziehen` als zweiter daran, und **der** geht
    /// ueber `aktives_setzen` sehr wohl bis `anwenden` durch.
```

Der Baum nimmt also an: `setHidden(true)` auf einer Ansicht mit dem Ersthelferrang löst
`makeFirstResponder:` aus, und der erreicht über den Melder (`:1130-1131`)
`aktives_dem_ersthelfer_nachziehen` → `aktives_setzen` → `aufteilung_nachziehen` → `anwenden`.
Der Ring bricht daran ab, dass `aktiv_setzen` beim zweiten Mal `false` liefert; er ist begrenzt
und nicht endlos.

**Neu ist, von wo aus er betreten wird.** Bisher lief er aus `kommando_ausfuehren` heraus,
nachdem `nach_dem_sichtbarkeitswechsel` den Fokus schon gesetzt hatte. Jetzt läuft er **davor**,
und `aktives_setzen` kann dabei `modell.aktiv()` umsetzen. Das anschließende
`fokus_setzen(Fokus::Dateifenster)` löst sein Ziel über genau dieses `modell.aktiv()` auf
(`:2233-2237`). Der Fokus kann damit in einem anderen Dateifenster landen als vor `df8163d`.

**Was ich nicht sagen kann:** ob AppKit nach dem Ausblenden einen Rang innerhalb eines
Dateifensters vergibt oder das Fenster selbst nimmt. Im zweiten Fall liefert
`bereich_des_ersthelfers().and_then(Bereich::seite)` `None`, `aktives_dem_ersthelfer_nachziehen`
kehrt sofort um, und nichts von dem oben Beschriebenen tritt ein. Der Fall ist ohne laufendes
Bündel nicht zu entscheiden.

**Zu bedenken ist auch, dass die alte Reihenfolge nicht sauber war**, nur anders: dort lief die
Rangneuvergabe am Ende von `kommando_ausfuehren`, also nach dem gewollten Umzug, und konnte
`aktiv` auf eine Seite setzen, in der der Fokus gerade nicht steht. `df8163d` hat nicht eine
saubere Ordnung durch eine schmutzige ersetzt, sondern eine Unstimmigkeit durch eine andere.
Deshalb steht hier eine Prüfaufgabe und keine Behauptung eines Rückschritts.

## Der zweite Nachzug je Umschaltbefehl

`df8163d` hat die Zusage „auf jedem Weg genau einmal" ausdrücklich zurückgenommen
(`:4559-4565`). Ein Befehl, der einen Bereich umschaltet, läuft jetzt zweimal durch
`aufteilung_nachziehen` — einmal aus `sichtbarkeit_aendern`, einmal am Ende von
`kommando_ausfuehren`.

**Die Wiederholung ist folgenlos, soweit sie aus dem Baum zu beurteilen ist.** `anwenden`
schreibt beide Male dieselben Werte: `setHidden` mit unverändertem Wert und `auslegen` mit
identischen Rahmen sind in AppKit ohne Wirkung, `wuensche_merken` setzt eine `Cell`,
`statuszeile_nachziehen` (`:4700-4733`) baut Zeichenketten und nimmt nichts weg,
`bereichsleiste_nachziehen` schreibt zehn Schalterzustände aus demselben Modell. Gezeichnet
wird erst am Ende des Durchgangs, also einmal.

**Ungemessen ist es trotzdem.** Der zweite Durchgang kostet eine Handvoll Obj-C-Nachrichten und
zwei Zeichenkettenbauten je Umschalttastendruck, und L1 („Tastendruck bis Ende des
Zeichendurchgangs", 95 Prozent im ersten Bild) ist zuletzt am 260810 gemessen — vor jeder
seither geschlossenen Runde. Das ist kein Verdacht, sondern eine Zeile für die Liste des
nächsten Abnahmelaufs.

## Prüfen im Abnahmelauf

1. Fokus in die Vorschau setzen, dann `f3` (Vorschau umschalten): landet der Fokus im **vorher
   aktiven** Dateifenster, und steht der Rahmen desselben Fensters auf der Akzentfarbe?
2. Dasselbe für den Editor mit `opt+cmd+b`, und für die Lesezeichenleiste.
3. `f3` mehrfach hintereinander: bleibt das aktive Dateifenster dasselbe?
4. L1 mit Umschaltbefehlen in der Reihe, gegen den Lauf `messungen/260810-1918-alle-zusagen.txt`.

**Schwere:** niedrig bis mittel, unentschieden bis zum Abnahmelauf. Keine Probe im Baum kann
die Frage beantworten; sie hängt am Verhalten von AppKit.

**Gefunden:** coderev, Durchsicht des Commits `df8163d` am 260823-0732, Bereich
`ab11eb8..df8163d`

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:4194-4212`, `:4222-4252`, `:8043-8080`

**Domain:** code

**Verwandt:**
`shared/issues/260823-0730_o_drei-prosastellen-um-den-neuen-nachzug-sind-mit-df8163d-falsch-geworden.md`
— dieselbe Änderung von der Prosaseite.
`shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`
— dieselbe Lage für eine andere Zusage: Arbeit innerhalb einer Endbedingung ohne eigenen
Abnahmelauf.

Der Modulkopf von spalten.rs zählt `beschreibbar` unter die vollständigen Fallunterscheidungen, und die ist keine

---

Der Kopf der neuen Datei `crates/krk-ui/src/spalten.rs` sagt zu, dass jede Fallunterscheidung
über `Spalte` ausgeschrieben ist und eine fünfte Spalte den Bau anhält — und zählt „die Frage,
ob man in ihr schreiben darf" ausdrücklich mit auf. `Spalte::beschreibbar` ist ein
`matches!(self, Spalte::Name)` und antwortet einer fünften Spalte still mit `false`. Weder der
Übersetzer noch die Probe daneben schlägt an.

---

**Schwere:** niedrig (kein falsches Verhalten heute; die Zusage des Modulkopfs trägt für eine
von sieben genannten Stellen nicht, und genau darauf verlässt sich, wer die Aufzählung
erweitert)
**Gefunden:** coderev, zweite Durchsicht der Runde, Bereich `8ffaac2..0342445`
**Betroffen:** `crates/krk-ui/src/spalten.rs`
**Domain:** code

## Die beiden Stellen nebeneinander

Modulkopf, `crates/krk-ui/src/spalten.rs:16-24`:

> # Vollstaendige Fallunterscheidung ohne Auffangzweig
>
> [`Spalte`] traegt vier Werte und behaelt sie. Jede Fallunterscheidung darueber ist
> ausgeschrieben, keine hat einen Auffangzweig, und das ist Absicht: eine fuenfte Spalte haelt
> den Bau an und erzwingt fuer jede der Stellen eine bewusste Antwort — Kennung, Ueberschrift,
> Beschriftung, Breiten, Ausrichtung, Zellentext **und die Frage, ob man in ihr schreiben
> darf**.

`crates/krk-ui/src/spalten.rs:98-100`:

```rust
pub const fn beschreibbar(self) -> bool {
    matches!(self, Spalte::Name)
}
```

`matches!` ist ein `match` mit einem `_ => false` darunter. Eine fünfte Spalte bekäme still
„nicht beschreibbar", also genau die stille Antwort, die der Modulkopf ausschließt.

## Die Probe fängt es nicht ab

`spalten::tests::genau_die_namensspalte_ist_beschreibbar` (`:110`) filtert `Spalte::ALLE` über
`beschreibbar` und vergleicht mit `vec![Spalte::Name]`. Eine fünfte, still nicht beschreibbare
Spalte lässt diese Gleichheit unberührt; die Probe bleibt grün.

Die sechs anderen genannten Stellen halten dagegen wirklich an: `kennung`, `titel`, `breiten`,
`ausrichtung` und `aus_kennung` in `crates/krk-ui/src/appkit/tabelle.rs:194/218/227/240/251`
sowie `spalte_sichtbar_in` in `crates/krk-ui/src/fenstermodell.rs`. `ausrichtung` hat mit
diesem Commit sogar seinen Auffangzweig `_ => NSTextAlignment::Left` verloren und nennt jetzt
die drei Werte einzeln — dieselbe Änderung an `beschreibbar` ist unterblieben.

## Woher es kommt

Die Fassung mit `matches!` stammt aus `appkit/tabelle.rs` und ist mit Schritt 6 unverändert
umgezogen. **Neu in diesem Bereich ist die Zusage, nicht der Code.** Der Modulkopf ist mit dem
Umzug geschrieben worden und behauptet seither etwas, das für diese eine Stelle nicht gilt.

## Vorschlag

Entweder `beschreibbar` als ausgeschriebenes `match` über alle vier Werte, wie `ausrichtung`
es jetzt hält, oder die Aufzählung im Modulkopf um „ob man in ihr schreiben darf" kürzen. Das
erste ist zwei Zeilen teurer und löst die Zusage ein; das zweite gibt sie auf. Der Baum hat
mit `ausrichtung` in derselben Runde die erste Wahl getroffen.

---
Resolved: Behoben am 260812-0745 auf dem ersten der beiden Wege: **der Code hält die Zusage,
der Modulkopf bleibt, wie er steht.** `Spalte::beschreibbar` (`crates/krk-ui/src/spalten.rs:106`)
ist ein ausgeschriebenes `match` über alle vier Werte, wie `appkit::tabelle::ausrichtung` es
seit Schritt 6 hält.

**Warum nicht die Zusage kürzen.** Die Kürzung wäre billiger zu schreiben und teurer zu
tragen: der Modulkopf schreibt die Regel dieses Projekts auf, nach der eine neue Variante den
Bau anhält und eine bewusste Antwort erzwingt, und eine Datei, die davon eine Stelle
ausnimmt, verlangt bei jeder Erweiterung ein Nachzählen, welche der sieben genannten Stellen
denn nun gemeint sind. Dieselbe Runde hat an `ausrichtung` in derselben Bewegung so
entschieden; zwei entgegengesetzte Wahlen an zwei Nachbarfunktionen wären der teurere Zustand.
Der Preis ist eine Zeile.

**Die Probe daneben kann das nicht halten, und sie sagt es jetzt selbst.**
`genau_die_namensspalte_ist_beschreibbar` liefe auch mit einem `_ => false` grün; sie hält die
Wahrheitswerte der vier Spalten, die Vollständigkeit hält der Übersetzer. Der Doc-Kommentar
der Probe nennt diese Teilung, damit die nächste Lesung sie nicht der Probe zutraut. Der
Doc-Kommentar von `beschreibbar` nennt den Grund gegen `matches!`.

Abnahme: `make check` Exit 0.

Die Generationsprüfung kann nicht greifen und verdeckt den Mechanismus, der wirklich schützt

---

Die Bedingung `modell.gehoert_dazu(meldung.generation())` in
`crates/krk-ui/src/appkit/tabelle.rs:353` ist immer wahr. Sie kann keinen Stapel
verwerfen. Verworfen werden veraltete Stapel von etwas anderem: dem Fallenlassen
des alten Lesevorgangs samt seinem Empfänger. Modulkopf und Plan schreiben die
Wirkung der Prüfung zu, die sie nicht hat.

---

## Der Nachweis

`stapel_uebernehmen` liest ausschließlich aus dem Kanal des **gerade gehaltenen**
Lesevorgangs (`tabelle.rs:345-352`):

```rust
let vorgang = self.ivars().lesevorgang.borrow();
let Some(vorgang) = vorgang.as_ref() else { return (false, true); };
…
for meldung in vorgang.meldungen().try_iter() {
    if !modell.gehoert_dazu(meldung.generation()) { continue; }
```

Ein Lesevorgang entsteht an genau einer Stelle, `tabelle.rs:215-229`:

```rust
let generation = self.ivars().letzte_generation.get() + 1;
self.ivars().letzte_generation.set(generation);
…
*self.ivars().lesevorgang.borrow_mut() = None;
self.ivars().modell.borrow_mut().leeren(generation);
self.ivars().tabelle.reloadData();
*self.ivars().lesevorgang.borrow_mut() = Some(Lesevorgang::starten(pfad, generation));
```

`leeren(generation)` setzt `modell.generation` auf denselben Wert, den
`Lesevorgang::starten` bekommt (`crates/krk-core/src/verzeichnis/modell.rs:52-56`).
`modell.generation` ändert sich sonst nirgends: `abschliessen`,
`sortierung_setzen` und `verstecke_ausblenden_setzen` lassen es unberührt. Der
Leser stempelt jede Meldung mit der Generation, die er beim Start bekommen hat
(`crates/krk-core/src/verzeichnis/leser.rs:181-263`).

Also gilt für jede Meldung, die `stapel_uebernehmen` je zu sehen bekommt:
`meldung.generation() == modell.generation`. Der `continue`-Zweig ist
unerreichbar. Nachgeprüft durch Lesen, nicht abgeleitet.

## Was stattdessen schützt, und dass es wirklich beendet statt zu ignorieren

Zeile `tabelle.rs:223` lässt den alten `Lesevorgang` fallen. Damit fällt sein
`Receiver`, und alle Meldungen, die noch im Kanal stehen oder noch gesendet
werden, sind unerreichbar. Der alte Arbeitsfaden endet auch wirklich, auf zwei
Wegen zugleich:

- `Lesevorgang::drop` ruft `abbrechen()` und setzt das Abbruchkennzeichen
  (`leser.rs:154-164`). Der Lesefaden prüft es vor jedem Systemaufruf und
  zwischen zwei Stapeln (`leser.rs:208`, `leser.rs:232`).
- Scheitert das Kennzeichen im Rennen, scheitert spätestens das nächste `send`
  am verschwundenen Empfänger und beendet die Schleife über `.ok()?`
  (`leser.rs:242`, `leser.rs:262`).

Die im Auftrag gestellte Frage ist damit beantwortet: ein Ordnerwechsel während
eines laufenden Lesevorgangs **beendet** den alten Lauf, er ignoriert ihn nicht.
Das ist die gute Nachricht dieses Datensatzes.

## Warum das trotzdem zu bereinigen ist

**Der Plan und der Programmtext beschreiben verschiedene Mechanismen.**
`### Frage 2` des Plans sagt: "Wer schnell durch Ordner navigiert, hat mehrere
Lesevorgänge gleichzeitig unterwegs. Ohne die Nummer bräuchte jeder davon eine
eigene Abbruchbehandlung; mit ihr verwirft der Hauptfaden jeden Stapel, dessen
Generation nicht mehr die aktuelle ist." Der Modulkopf von `tabelle.rs:19-22`
wiederholt das wörtlich. Die Umsetzung hält aber nie mehr als einen Lesevorgang;
sie hat sich für den einfacheren Weg entschieden, ohne das zu sagen.

Die Wahl der Umsetzung ist die bessere: ein einziger gehaltener Lesevorgang
braucht weder Nummer noch Prüfung, und "der alte Kanal ist weg" ist eine
stärkere Zusage als "wir schauen bei jedem Stapel nach". Falsch ist nur, dass
beide Mechanismen nebeneinander im Text stehen und der Leser den unwirksamen für
den tragenden hält.

**Der Preis fällt später an.** S12 bringt zwei Dateifenster mit je mehreren Tabs.
Wer dann mehrere Lesevorgänge gegen ein Modell führen will, findet eine
Generationsprüfung vor, die aussieht, als wäre sie erprobt, und die nie einen
einzigen Stapel verworfen hat. Genau das ist der Musterfehler, den diese Prüfung
vor S8 abfangen soll.

## Was zu tun ist

Eine von zwei Antworten, nicht beide:

1. **Die Prüfung behalten und belegen.** Dann braucht sie einen Fall, in dem sie
   greift, und der ist ohne Fenster prüfbar: ein `Ordnermodell`, das
   `leeren(2)` gesehen hat, und eine Meldung mit Generation 1. Die Prüfung gehört
   in `krk-core`, wo `gehoert_dazu` wohnt, und der Kommentar in `tabelle.rs:19-22`
   ist auf "Vorsorge für mehrere gleichzeitige Lesevorgänge, heute nicht
   erreicht" umzuschreiben.
2. **Die Prüfung entfernen.** Dann sagt der Modulkopf, was wirklich trägt: der
   alte Empfänger ist weg, der alte Faden endet von selbst. `Meldung::generation`
   und `Ordnermodell::gehoert_dazu` blieben ungenutzt und wären mitzunehmen.

In beiden Fällen zieht der `planner` `### Frage 2` nach, damit Plan und
Programmtext denselben Mechanismus nennen.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.

---
Resolved: 260803-2025. Möglichkeit 2 gewählt: die Prüfung ist entfernt, und der Text nennt jetzt den wirksamen Mechanismus.

**Warum nicht Möglichkeit 1.** Eine Prüfung zu belegen, die die Umsetzung nie erreicht, hieße einen zweiten Mechanismus neben dem tragenden stehen zu lassen. Der Datensatz nennt die Wahl der Umsetzung selbst die bessere: ein einziger gehaltener Lesevorgang braucht weder Nummer noch Prüfung, und "der alte Kanal ist weg" ist die stärkere Zusage.

**Geändert.** In `crates/krk-ui/src/appkit/tabelle.rs` ist die Bedingung `modell.gehoert_dazu(meldung.generation())` aus `stapel_uebernehmen` heraus; an ihrer Stelle steht eine Zeile, die sagt, warum keine Prüfung nötig ist. Der Modulkopf beschreibt statt der Nummer den Weg, der wirklich trägt: `ordner_lesen` lässt den alten `Lesevorgang` fallen, damit fällt sein Empfänger, `Lesevorgang::drop` setzt das Abbruchkennzeichen, der Lesefaden prüft es vor jedem Systemaufruf und zwischen zwei Stapeln, und spätestens das nächste `send` scheitert am verschwundenen Empfänger. Er hält dazu fest, dass die Prüfung bis zum 260803 danebenstand und warum sie nicht greifen konnte, damit sie nicht unbesehen zurückkehrt. Die Dokumentation von `Ordnermodell::generation` (`crates/krk-core/src/verzeichnis/modell.rs`) sagte denselben Satz und ist mitgezogen.

**Was die Nummer weiter trägt** und deshalb bleibt: sie benennt den Lesefaden, und sie sagt dem Modell beim Leeren, zu welchem Lauf sein Inhalt gehört. `Ordnermodell::gehoert_dazu` bleibt ebenfalls, es steht in `crates/krk-core/tests/verzeichnis.rs` und in `crates/krk-bench/src/messen.rs:228`.

**Zwei Nachzüge, beide als eigener Datensatz.** Der Plan sagt in `### Frage 2` weiter das Alte; er war für diesen Auftrag gesperrt, weil der `planner` zur selben Zeit darin arbeitet: `issues/260803-2025_o_frage-2-des-plans-nennt-den-unwirksamen-mechanismus.md`. `Meldung::generation` und `Lesevorgang::generation` haben jetzt keinen Aufrufer mehr, und `leser.rs` lag außerhalb des Auftragsumfangs: `issues/260803-2025_o_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr.md`.

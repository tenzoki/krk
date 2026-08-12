Der Nachzug der Bereichsleiste läuft nach einem angenommenen Klick zweimal, und der Kommentar bestreitet es

---

`bereichsleiste_nachziehen` wird auf dem Klickweg zweimal gerufen: einmal aus
`aufteilung_nachziehen`, das jedem ausgeführten Kommando folgt, und danach noch einmal aus dem
Melder der Leiste. Acht `setState:` werden also doppelt geschrieben. Der Doc-Kommentar der
Funktion sagt „Der zweite Anlass ist keine Verdopplung des ersten"; für den angenommenen Klick
ist er genau das.

---

**Schwere:** niedrig (idempotent, kein falscher Zustand; doppelte Zeichenarbeit auf einem Weg,
den die Risikotafel des Plans ausdrücklich gegen L1 abwägt, und ein Kommentar, der das
Gegenteil des Codes sagt)
**Gefunden:** coderev, zweite Durchsicht der Runde, Bereich `8ffaac2..0342445`
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs`
**Domain:** code

## Der Weg, am Baum gelesen

Der Melder, eingetragen beim Aufbau (`crates/krk-ui/src/appkit/anwendung.rs:777`):

```rust
bereichsleiste.melder_setzen(Box::new(move |kommando| {
    if let Some(selbst) = schwach.load() {
        selbst.kommando_ausfuehren(kommando);
        selbst.bereichsleiste_nachziehen();
    }
}));
```

`kommando_ausfuehren` schließt mit (`:2201`):

```rust
if ausgefuehrt {
    self.aufteilung_nachziehen();
    self.sitzung_vormerken();
}
```

und `aufteilung_nachziehen` endet mit `self.bereichsleiste_nachziehen();` (`:2761`).

Für einen **abgewiesenen** Klick trifft der Kommentar zu: `aufteilung_nachziehen` wird nicht
erreicht, und der zweite Ruf ist der einzige. Für einen **angenommenen** läuft der Nachzug
zweimal hintereinander mit demselben Modellstand.

## Warum es überhaupt zählt

Die Risikotafel des Plans wägt die Kosten dieses Nachzugs ausdrücklich gegen die Zeitzusage L1
ab: „Der Nachzug schreibt acht Schalterzustände nach jedem ausgeführten Befehl und kostet damit
Zeichenarbeit auf dem Weg, den L1 misst … Dieselbe Größenordnung wie `rahmen_setzen`." Auf dem
Klickweg sind es sechzehn und nicht acht. Der Tastenweg ist nicht betroffen.

Wichtiger als die Zeichenarbeit ist der Kommentar: dieses Projekt hat dreimal eine Sitzung an
einem Kommentar verloren, der etwas anderes sagte als der Code, und der Satz „keine
Verdopplung" ist der Grund, aus dem die nächste Lesung nicht nachzählt.

## Vorschlag

Die billigste Behebung ist der Kommentar: „Auf dem Klickweg läuft der Nachzug nach einem
angenommenen Befehl zweimal; das ist der Preis dafür, dass der abgewiesene Klick überhaupt
einen Anlass hat." Damit stimmt die Aussage, und die Doppelung ist benannt statt bestritten.

Die andere Möglichkeit, den Melder nur bei `!kommando_ausfuehren(kommando)` nachziehen zu
lassen, spart die acht Schreibvorgänge, bindet den Nachzug aber an den Rückgabewert und macht
aus einer bedingungslosen Zeile eine Fallunterscheidung. Der Modulkopf der Bereichsleiste nennt
die Bedingungslosigkeit als tragend für C2.4; sie aufzugeben ist der teurere Weg.

---
Resolved: Behoben am 260812-0745, und nicht auf einem der beiden im Datensatz genannten Wege.
Weder der Kommentar allein noch eine Bedingung `!kommando_ausfuehren(kommando)` am Melder: die
Frage "war der Klick angenommen?" entfällt, statt beantwortet zu werden.

**Die Selbstkippung wird dort zurückgenommen, wo sie entsteht.**
`Leistenquelle::geklickt` (`crates/krk-ui/src/appkit/bereichsleiste.rs`) ist der neue Trichter
beider Aktionsmethoden und ruft `selbstkippung_zuruecknehmen(absender)`, bevor es das Kommando
meldet — auch dann, wenn aus der `tag` kein Kommando wird. Danach zeigt die Leiste wieder
genau den Stand des Modells, und der Melder im Anwendungsdelegierten
(`crates/krk-ui/src/appkit/anwendung.rs:777`) ruft nur noch `kommando_ausfuehren`.

Damit zieht auf jedem Weg genau `bereichsleiste_nachziehen` nach, und genau einmal:
angenommener Klick 1 + 8 statt 16 Schreibvorgänge, abgewiesener Klick 1 statt 8, Tastendruck
unverändert 8. `bereichsleiste_nachziehen` hat wieder **einen** Anlass, und sein
Doc-Kommentar sagt das, statt eine Verdopplung zu bestreiten. Nachgezogen sind der Modulkopf
von `bereichsleiste.rs` samt seiner Skizze, der Kommentar am Melder, der Doc-Kommentar an
`bereichsleiste_nachziehen`, die Begründung an `zustaende_setzen` und im Plan der Fluss unter
`### Der eine Weg vom Eingang bis zur Anzeige` samt Schritt 8.

**Der Preis ist benannt und nicht wegverhandelt:** die Rücknahme setzt voraus, dass das
Ankreuzfeld seinen Zustand vor der Aktion wirklich selbst kippt. Dieselbe Annahme trug schon
den unbedingten Nachzug — sie ist der einzige Grund, aus dem es ihn gab —, aber die Folge
eines Irrtums ist eine andere: bisher wäre der zweite Ruf nur überflüssig gewesen, jetzt
stünde ein abgewiesener Klick falsch. Gemessen ist die Annahme in diesem Baum nicht; sie steht
in der Dokumentation von AppKit und ist am laufenden Bündel zu sehen. C2.4 ist ohnehin ein
Bündel-Kriterium und steht auf der Liste unter `## Abnahme am laufenden Bündel`.

**Keine Probe.** Die Rücknahme lebt an einem `NSButton`; eine Probe dafür bräuchte eine
Instanz und damit `MainThreadMarker::new_unchecked`, also eine fünfte Stelle der Bauart, die
Schritt 8 aus demselben Grund vermieden hat (`issues/260810-1001_*`, als Lage angenommen;
`decisions/260810-1044_*`, zurückgestellt).

Abnahme: `make check` Exit 0.

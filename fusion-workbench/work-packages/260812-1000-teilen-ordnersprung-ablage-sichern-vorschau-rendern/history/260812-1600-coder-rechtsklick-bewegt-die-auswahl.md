# Der Rechtsklick bewegt die Auswahl: Nachzug auf den Nutzerentscheid vom 260812-1200

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Grundlage:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1145_i_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`, Abschnitt `## Antwort 260812-1200`
**Behebt:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-1500_c_der-rechtsklick-bewegt-die-auswahl-nicht-obwohl-der-nutzerentscheid-es-verlangt.md`
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0

---

## Was gebaut wurde

Möglichkeit 2 des Datensatzes. Ein Rechtsklick in der Dateiliste setzt die
Auswahl auf die angeklickte Zeile, es sei denn, diese Zeile ist markiert; dann
bleiben Auswahl und Markierung stehen. Danach gilt
`kommandos::operationen::betroffene` unverändert.

Schritt 6 hatte Möglichkeit 1 gebaut, weil der Plan um 1145 geschrieben ist und
die Antwort um 1200 gegeben wurde. Die Grundlage bindet, der Plan ist an dieser
Stelle veraltet.

## Die Aufteilung: Regel ohne Fenster, AppKit daneben

```text
menuNeedsUpdate:                          appkit/tabelle.rs
   │
   ├─ rechtsklick_auswahl_nachziehen()    appkit/tabelle.rs
   │     ├─ NSTableView::clickedRow  ─────────────┐
   │     ├─ operationen::rechtsklick_zielzeile ◄──┘   kommandos/operationen.rs
   │     └─ zeile_setzen  →  auswahl_merken           (ohne Fenster prüfbar)
   │
   └─ betroffene_eintraege()  →  operationen::betroffene   unangetastet
```

**`crates/krk-ui/src/kommandos/operationen.rs`**

Neu ist `rechtsklick_zielzeile`, eine reine Funktion unmittelbar neben
`betroffene`. Sie nimmt das Ordnermodell und den Wert von `clickedRow` und
liefert die Zeile, auf die die Auswahl rücken soll, oder `None`. Sie trägt
`#[must_use]`: ihr Rückgabewert ist ihre einzige Wirkung, ein stilles
Fallenlassen bliebe unbemerkt.

Die Fallunterscheidung ist vollständig und überschneidungsfrei. Drei Fälle
liefern `None`, einer liefert die Zeile:

| Fall | Antwort |
|---|---|
| `angeklickt` negativ (Klick auf keine Zeile) | `None` |
| Zeile außerhalb der Sichtreihenfolge (leere Liste, Zeile hinter dem Ende) | `None` |
| Zeile markiert | `None` |
| sonst | `Some(zeile)` |

Der Platz neben `betroffene` ist gewählt, damit ein späterer Leser beide
zusammen sieht. Genau dort entstünde sonst die zweite Auswahlregel, die der
Datensatz ausschließt.

**`crates/krk-ui/src/appkit/tabelle.rs`**

`DateifensterQuelle::rechtsklick_auswahl_nachziehen` trägt den AppKit-Anteil und
sonst nichts: `clickedRow` lesen, die reine Funktion fragen, die Zeile setzen.
Gerufen wird es in `menuNeedsUpdate:` als **erste** Zeile, also vor
`betroffene_eintraege`. Gesetzt wird über `zeile_setzen`, damit über
`auswahl_merken`; der Datensatz verlangt diesen einen Weg ausdrücklich, weil die
Vorschau aus C6 sonst nichts von der neuen Auswahl erführe.

Die Ausleihe des Tabmodells endet vor dem ersten Objective-C-Aufruf, wie es der
Modulkopf für jede Ausleihe verlangt. Der Grund ist hier derselbe wie beim Menü
selbst: `zeile_setzen` ruft über den Auswahlrückruf in dieselbe Quelle zurück.

Drei Texte sind nachgezogen, die noch Möglichkeit 1 begründeten: der
Doc-Kommentar an `menuNeedsUpdate:` und zwei Stellen im Modulkopf.

## Die Probe prüft die Regel, nicht den Klick

Fünf Prüfungen in `kommandos::operationen::tests`, alle ohne Fenster:

| Probe | Was sie hält |
|---|---|
| `der_rechtsklick_setzt_die_auswahl_auf_die_angeklickte_zeile` | der Regelfall |
| `auf_einer_markierten_zeile_bewegt_der_rechtsklick_nichts` | die Ausnahme, über drei markierte Zeilen |
| `eine_markierung_anderswo_haelt_den_rechtsklick_nicht_auf` | die Ausnahme fragt nach der angeklickten Zeile, nicht danach, ob überhaupt etwas markiert ist |
| `ein_klick_auf_keine_zeile_setzt_keine_auswahl` | `clickedRow` liefert `-1` |
| `eine_zeile_jenseits_der_liste_setzt_keine_auswahl` | Zeile hinter dem Ende und leere Liste |

Ein Klickversuch wäre etwas anderes und stünde ohne laufendes Bündel nicht zur
Verfügung. Geprüft ist die Entscheidungsregel selbst.

## Am SDK gegengelesen

`clickedRow` steht in `NSTableView.h:276` und trägt **kein** `API_AVAILABLE`,
steht also seit macOS 10.0. Am SDK gelesen
(`/Applications/Xcode.app/…/MacOSX.sdk`), nicht aus dem Plan übernommen. Das
Bündel zielt auf 15.0. Der Modulkopf von `tabelle.rs` nennt die Zahl jetzt
ausdrücklich und hält fest, dass die Eigenschaft seit dem 260812 zwei Abnehmer
hat: den Doppelklick aus C3 der Runde 4 und die Auswahl vor dem Rechtsklick aus
C1 der Runde 6.

Nicht am SDK zu lesen und deshalb hier benannt: `inference:` dass `clickedRow`
während `menuNeedsUpdate:` die angeklickte Zeile trägt, steht in keiner
Kopfdatei. Es ist die dokumentierte Bedeutung der Eigenschaft und der Weg, den
`NSTableView` beim Aufbau des Kontextmenüs nimmt. Am laufenden Bündel gesehen
ist es nicht; das gehört zu C1.1 und C1.6, die ohnehin Nutzerarbeit bleiben.

## Ein Preis, den der Datensatz nicht nennt

Ein Rechtsklick auf eine **un**markierte Zeile rückt die Auswahl nach, ändert
aber nichts am Ergebnis, solange anderswo in der Liste etwas markiert ist: in
`betroffene` behält die Markierung den Vorrang. Der Klick zeigt dann weiterhin
auf A und wirkt auf B, also genau das, was Möglichkeit 2 abstellen sollte.

Die Pro-Zeile des Datensatzes zu Möglichkeit 2 („der Klick zeigt und wirkt auf
dasselbe") gilt damit nur, solange nichts markiert ist. Aufzuheben wäre die
Markierung, und das ist die ausdrücklich abgelehnte Möglichkeit 3. Der Fall ist
deshalb so gebaut, wie die Antwort ihn beschreibt, und im Doc-Kommentar an
`rechtsklick_zielzeile` ausgeschrieben, damit ihn niemand für ein Versehen hält.
Ob er eine eigene Frage verdient, gehört dem Nutzer vorgelegt und ist hier nicht
entschieden.

## Der Plan bleibt stehen

Schritt 6 des Plans vom 260812-1145 führt den Datensatz weiterhin als offen und
verlangt Möglichkeit 1 mit dem Zusatz „solange sie offen ist, gilt die Regel
ohne Ausnahme". Der Wortlaut ist seit dem 260812-1200 falsch und seit dieser
Aufgabe auch gegenüber dem Code falsch. Angefasst ist er nicht: der Plan gehört
dem `planner`, und die Aufgabenstellung nennt allein die beiden Datensätze.

## Die Deckung des Untergrenzen-Abschnitts, rekursiv nachgezählt

Der Bericht zu Schritt 6 nannte „23 von 25 Dateien" und erklärte daraufhin die
Zahl in CLAUDE.md und die im Bericht zu Schritt 5 für beide falsch. Gezählt war
dort `crates/krk-ui/src/appkit/*.rs`, also allein die oberste Ebene; unter
`appkit/` liegt daneben `blaetter/`. Rekursiv nachgezählt:

```sh
grep -rL "# Ab welchem macOS die angesprochenen Klassen stehen" \
  --include='*.rs' crates/krk-ui/src/appkit
```

**33 von 35 Dateien**, ohne den Abschnitt weiterhin allein `koordinaten.rs` und
`mod.rs`, beide begründet. Der Bericht zu Schritt 5 hat damit recht behalten,
offen bleibt allein die Zahl in CLAUDE.md. Die falsche Zahl ist im Bericht zu
Schritt 6 berichtigt und die Berichtigung als solche gekennzeichnet. Der Defekt
`shared/issues/260812-1438_o_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`
trägt die Frage an CLAUDE.md schon und ist nicht angefasst.

## Datensätze

- `decisions/260812-1145_a_…` → `_i_`, mit `Implemented:`-Zeile und beiden
  Codestellen. Umbenannt mit `git mv`.
- `issues/260812-1500_o_…` → `_c_`, mit `Resolved:`-Absatz. Umbenannt mit
  einfachem `mv`: die Datei ist im vorigen Schritt entstanden und noch nicht
  committet, `git mv` weist eine unversionierte Datei ab.

## Abnahme

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

410 Proben im Binärziel `krk` gegenüber 405 vorher, also die fünf neuen. Kein
bestehender Prüffall ist angefasst worden. Kein Vordergrundlauf, kein
Bündelbau, keine Messung. Nicht committet; der Nutzer committet.

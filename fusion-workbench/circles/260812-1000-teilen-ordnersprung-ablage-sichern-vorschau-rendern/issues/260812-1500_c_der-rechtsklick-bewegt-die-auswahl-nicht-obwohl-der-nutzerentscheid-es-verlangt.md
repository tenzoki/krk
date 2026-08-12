Der Rechtsklick bewegt die Auswahl nicht, obwohl der Nutzerentscheid es verlangt

---

Schritt 6 dieser Runde hat das Kontextmenü an die drei Flächen gehängt und dabei
**Möglichkeit 1** umgesetzt: der Rechtsklick in der Dateiliste bewegt weder die
Auswahl noch die Markierung, das Menü wirkt auf `betroffene` wie jeder
Tastenbefehl. Der Datensatz
`decisions/260812-1145_a_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
ist am 260812-1200 aber mit **Möglichkeit 2** beantwortet worden, und er lehnt
Möglichkeit 1 ausdrücklich ab. Der gebaute Zustand widerspricht damit einem
bindenden Nutzerentscheid.

Die Ursache ist ein Zeitversatz und keine Erwägung: der Plan ist um 1145
geschrieben, die Antwort um 1200 gegeben, und der Wortlaut von Schritt 6 führt
den Datensatz seither als offen und mit dem Zusatz „solange sie offen ist, gilt
die Regel ohne Ausnahme". Die Aufgabenstellung an den `coder` hat diesen Wortlaut
wiederholt und ausdrücklich verlangt, die Frage nicht nebenbei zu beantworten.
Sie ist nicht nebenbei zu beantworten — sie **ist** beantwortet, und die Antwort
ist nicht umgesetzt.

---

**Was zu tun ist**

Der Datensatz schreibt die Regel aus: der Rechtsklick setzt die Auswahl auf die
angeklickte Zeile, **es sei denn, diese Zeile ist bereits markiert**; danach gilt
`kommandos::operationen::betroffene` unverändert. Die angeklickte Zeile liefert
`NSTableView` über `clickedRow`; der Weg besteht in diesem Baum schon, der
Doppelklick aus C3 der Runde 4 nimmt ihn
(`crates/krk-ui/src/appkit/tabelle.rs`, `doppelklick:`).

Die Änderung sitzt in `DateifensterQuelle::menue_auffrischen`
(`crates/krk-ui/src/appkit/tabelle.rs`) und ist klein: vor dem Nachschlagen der
betroffenen Einträge die angeklickte Zeile lesen und, wenn sie nicht markiert
ist, die Auswahl auf sie setzen. Zwei Nebenbedingungen nennt der Datensatz
selbst:

- Die Auswahl geht durch **dieselbe** Stelle wie eine über die Tastatur bewegte,
  `DateifensterQuelle::auswahl_merken`, sonst erfährt die Vorschau nichts davon.
- Eine zweite Auswahlregel entsteht nicht. `betroffene` bleibt unangetastet;
  geändert wird die Auswahl **vor** ihr.

**Kontext**

- Der Schaden ist heute gering und wächst mit dem zweiten Eintrag im Menü. Teilen
  zerstört nichts, wer die falsche Datei teilt, bricht den Systemdialog ab. Der
  Circle sagt weitere Einträge für spätere Runden zu, und genau darauf stützt der
  Datensatz seine Ablehnung von Möglichkeit 1.
- Der Modulkopf von `tabelle.rs` und der Doc-Kommentar an `menue_auffrischen`
  führen zurzeit die Begründung für Möglichkeit 1 samt Verweis auf den Datensatz
  als offene Frage. Beide sind mit der Behebung nachzuziehen; der Modulkopf
  bekommt daneben die macOS-Untergrenze von `clickedRow`, wie der Datensatz es
  verlangt.
- Betroffen ist allein die Dateiliste. Im Editor und in der Vorschau gibt es
  keine Zeile unter dem Zeiger, auf die etwas zu bewegen wäre; der Datensatz
  hält das ausdrücklich fest.
- Gefunden bei der Umsetzung von Schritt 6, nicht behoben: die Aufgabenstellung
  verlangt Möglichkeit 1 wörtlich, und eine Abweichung davon auf eigene Faust
  wäre eine Entscheidung des Agenten über eine Verhaltensfrage, die der Nutzer
  bereits entschieden hat. Der Widerspruch gehört ihm vorgelegt, nicht still
  aufgelöst.

---
Resolved: Behoben am 260812, Möglichkeit 2 ist gebaut. Der Rechtsklick in der Dateiliste setzt die Auswahl auf die angeklickte Zeile, es sei denn, diese Zeile ist markiert; dann bleiben Auswahl und Markierung stehen.

Die Entscheidungsregel steht als reine Funktion `rechtsklick_zielzeile` in `crates/krk-ui/src/kommandos/operationen.rs`, unmittelbar neben `betroffene`, und ist mit fünf Prüfungen belegt: der Regelfall, die Ausnahme über drei markierte Zeilen, eine Markierung anderswo in der Liste, der Klick auf keine Zeile (`clickedRow` liefert `-1`) und eine Zeilennummer jenseits der Liste. Ohne Fenster geprüft, wie es der Datensatz verlangt: kein Klickversuch, sondern die Regel selbst.

`DateifensterQuelle::rechtsklick_auswahl_nachziehen` in `crates/krk-ui/src/appkit/tabelle.rs` trägt den AppKit-Anteil, also `NSTableView::clickedRow` zu lesen und die Zeile zu setzen. Gerufen wird es in `menuNeedsUpdate:` **vor** `betroffene_eintraege`. Gesetzt wird über `zeile_setzen` und damit über `auswahl_merken`, den einen Weg, den auch die Tastatur nimmt; ohne ihn erführe die Vorschau aus C6 nichts von der neuen Auswahl. Die Ausleihe des Tabmodells endet vor dem ersten Objective-C-Aufruf, wie es der Modulkopf jener Datei für jede Ausleihe verlangt.

Die beiden Nebenbedingungen des Datensatzes halten. `kommandos::operationen::betroffene` ist nicht angefasst und beantwortet weiterhin allein, worauf ein Befehl wirkt; eine zweite Auswahlregel entsteht nicht.

Nachgezogen sind daneben die Texte, die noch Möglichkeit 1 begründeten: der Doc-Kommentar an `menuNeedsUpdate:` und zwei Stellen im Modulkopf von `tabelle.rs`. Der Modulkopf nennt jetzt die macOS-Untergrenze von `clickedRow`, am SDK gelesen und nicht übernommen: `NSTableView.h:276` trägt kein `API_AVAILABLE`, die Eigenschaft steht damit seit 10.0, das Bündel zielt auf 15.0.

**Ein Preis ist beim Bauen sichtbar geworden, den der Datensatz nicht nennt.** Ein Rechtsklick auf eine *un*markierte Zeile rückt die Auswahl zwar nach, ändert aber nichts am Ergebnis, solange anderswo in der Liste etwas markiert ist: in `betroffene` behält die Markierung den Vorrang. Der Klick zeigt dann weiterhin auf A und wirkt auf B. Das Aufheben der Markierung wäre die ausdrücklich abgelehnte Möglichkeit 3, also ist der Fall so gewollt und nicht behoben; er steht im Doc-Kommentar an `rechtsklick_zielzeile` ausgeschrieben, damit ihn niemand für ein Versehen hält. Ob er eine eigene Frage verdient, gehört dem Nutzer vorgelegt und ist hier nicht entschieden.

Abnahme am 260812: `cargo build --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace` je Exit 0. 410 Proben im Binärziel `krk` gegenüber 405 vorher, also die fünf neuen; kein bestehender Prüffall ist angefasst worden.

Der Datensatz `decisions/260812-1145_i_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md` ist mit derselben Änderung von beantwortet auf umgesetzt gezogen. Noch nicht committet, der Nutzer committet nach der Aufgabe.

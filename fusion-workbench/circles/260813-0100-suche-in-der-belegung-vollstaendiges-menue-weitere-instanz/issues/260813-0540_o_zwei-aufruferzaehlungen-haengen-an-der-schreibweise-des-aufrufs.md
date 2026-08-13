Zwei Aufruferzählungen hängen an der Schreibweise des Aufrufs

---

Zwei Proben zählen Aufrufer über eine Zeichenkette, die die **Schreibweise** des Aufrufs
enthält, nicht den Aufruf selbst:

- `beide_frager_rufen_die_eine_regel` sucht `zulaessigkeit::zulaessig(` und erwartet zwei
  Fundstellen (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:174-200`).
- `der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten` sucht
  `self.kommando_ausfuehren(` und `selbst.kommando_ausfuehren(` und erwartet drei
  (`crates/krk-ui/src/appkit/menue.rs:1155-1188`).

Beide sind heute richtig: die zwei Frager stehen als `zulaessigkeit::zulaessig(kommando,
self.lage())` in `crates/krk-ui/src/appkit/anwendung.rs:740` und `:2586`, die drei
Ausführungswege als `self.kommando_ausfuehren(` beziehungsweise `selbst.kommando_ausfuehren(`.

**Ein dritter Frager entgeht der ersten Probe, sobald er anders geschrieben ist.** Ein
`use crate::kommandos::zulaessigkeit::zulaessig;` und ein unqualifizierter Aufruf `zulaessig(…)`
lassen die Zahl bei zwei, und die Probe bleibt grün. Dasselbe bei der zweiten Probe für jeden
Empfängernamen außer `self` und `selbst`; der Baum bindet den Delegierten in Rückrufen bereits
unter wechselnden Namen (`selbst`, `delegierter`).

---

**Schwere:** gering. Aufruferzählungen sind nach `crates/krk-ui/src/quellbaum.rs:22-28`
ohnehin „in beide Richtungen blind" und stehen nur da, wo ein Kriterium die Zahl selbst
zusagt (C2.16, C2.14). Der Befund benennt eine zusätzliche, vermeidbare Blindheit: die Probe
wird nicht rot, obwohl der Fall eintritt, für den sie gebaut ist.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/kommandos/zulaessigkeit.rs:174-200`,
`crates/krk-ui/src/appkit/menue.rs:1155-1188`

**Domain:** code

## Vorschlag

Die Nadel auf den Funktionsnamen mit Klammer verkürzen (`zulaessig(` beziehungsweise
`kommando_ausfuehren(`) und die bekannten Fundstellen, die keine Aufrufe sind, ausdrücklich
abziehen — die Erklärung, die Doc-Kommentare und die Weiterreichungen an Tabelle, Leiste und
Vorschau. Das ist mehr Arbeit als der heutige Einzeiler und hält, was der Name der Probe
verspricht. Wo das zu teuer ist: den Doc-Kommentar um den Satz ergänzen, dass die Probe an
der Schreibweise hängt.

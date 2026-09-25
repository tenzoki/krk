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

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813, auf dem im Datensatz genannten teureren Weg und nicht mit einem Satz im Doc-Kommentar.

**Ein gemeinsames Werkzeug traegt beide Zaehlungen:** `crate::quellbaum::aufrufstellen` zaehlt die Aufrufe eines Namens in einer Datei und zieht die drei Sorten Fundstellen ab, die keine Aufrufe sind — Treffer mitten in einem laengeren Namen (`Unzulaessig(` ist kein Aufruf von `zulaessig`), die Erklaerung selbst, und Nennungen in Kommentaren. Jede Empfaengerform und jeder Pfad bleiben damit drin. Eine eigene Probe, `eine_aufrufzaehlung_sieht_jede_schreibweise_und_keine_nennung`, haelt die drei Abzuege fest; ohne sie waere „jede Schreibweise wird erfasst" selbst wieder nur behauptet.

**`beide_frager_rufen_die_eine_regel`** zaehlt jetzt die Aufrufstellen von `zulaessig` in allen Dateien ausser `kommandos/zulaessigkeit.rs` und erwartet zwei. Ein dritter Frager mit `use` und unqualifiziertem Aufruf macht die Probe rot. Die eigene Datei bleibt aussen vor, so wie `das_menue_wird_an_zwei_anlaessen_gebaut` `menue.rs` aussen vor laesst: dort stehen die Erklaerung und die Tafel aus 140 Faellen, die die Regel hundertvierzigmal ruft.

**`der_delegierte_wird_an_genau_drei_stellen_um_einen_befehl_gebeten`** traegt jetzt zwei Zahlen. Die drei mit Empfaenger `self.`/`selbst.` bleiben und **benennen**, wer fragt; daneben steht die Gesamtzahl aller Aufrufe des Namens im Baum, heute acht — die drei und fuenf Weiterreichungen an Tabelle, Leiste und Vorschau. Die zweite Zahl haengt an keinem Empfaenger: ein vierter Frager unter einem beliebigen Bindungsnamen macht sie rot, gleich wie er geschrieben ist. Den Delegierten umzubenennen, damit eine einzige Zahl reichte, waere der sauberere Schnitt gewesen und haette `CLAUDE.md` und mehrere Datensaetze mitgerissen; die zwei Zahlen kosten weniger und leisten dasselbe.

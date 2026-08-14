# `befund_setzen` baut die ganze Sicht neu auf, und der Durchlauf ruft es je Ordner

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritt A1 („Jeder baut die Sicht neu auf") und Schritt F2 („Ein eingetroffener Befund geht über `befund_setzen` in das Modell, das die Sicht neu aufbaut"); `crates/krk-core/src/verzeichnis/modell.rs` (`Ordnermodell::befund_setzen`, `sicht_neu_aufbauen`); C3.11, C3.12

---

`Ordnermodell::befund_setzen` trägt genau einen Befund ein und ruft danach `sicht_neu_aufbauen`, also einen vollständigen Durchlauf über alle Einträge samt `sort_unstable_by`. So steht es im Plan, und A1 hat es so umgesetzt.

**Die Kosten hängen an der Zahl der Unterordner des angezeigten Ordners.** Jeder Auftrag des Durchlaufs liefert genau einen Befund, also ist die Zahl der Neuaufbauten je Durchlauf die Zahl der Ordner, für die er läuft. Für einen angezeigten Ordner mit `n` Einträgen davon `k` Unterordnern sind das `k` Neuaufbauten zu je `O(n log n)`. Bei einem Ordner mit zweitausend Unterordnern unter zehntausend Einträgen ist das zweitausendmal ein Sortierlauf über zehntausend Elemente — auf dem **Hauptfaden**, im Einzugstakt.

**Gemessen ist das nicht**, und es kann sein, dass es nie auffällt: der Einzugstakt räumt den Kanal mit `try_iter` leer, und ein Ordner mit tausenden Unterordnern ist selten. Der Satz steht hier, weil C3.12 („Der Durchlauf hält KRK nicht an") eine Bündelbeobachtung ist und diese Stelle der einzige Ort im Entwurf ist, an dem die Arbeit des Durchlaufs auf den Hauptfaden zurückschlägt.

**Die Gegenmaßnahme wäre klein**, falls sie gebraucht wird: ein zweiter Setzer, der eine Reihe von Befunden entgegennimmt und **einmal** neu aufbaut, gerufen vom Einzugstakt nach dem `try_iter`. Das zweite Bild des Plans zeichnet den Takt bereits so („Befund in das Modell, Sicht neu aufbauen, Zeile nachziehen" als ein Schritt); allein die Signatur des Setzers folgt ihm nicht.

**Nicht in A1 geändert**, weil Schritt A1 die Setzer so aufzählt, wie sie umgesetzt sind, und ein zweiter Setzer ohne Aufrufer eine Zeile ohne Frager wäre. Die Stelle, an der es sich entscheidet, ist F2.

**Kontext.** Aufgefallen beim Umsetzen von Schritt A1. Aus dieser Directive entstanden, deshalb im Circle und nicht im gemeinsamen Speicher.

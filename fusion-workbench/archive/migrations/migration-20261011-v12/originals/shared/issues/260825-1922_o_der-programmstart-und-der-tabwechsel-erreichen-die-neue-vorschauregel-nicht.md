# Der Programmstart und der Tabwechsel erreichen die neue Vorschauregel nicht

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Schritt 7); `shared/decisions/260825-1725_*_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md`; `crates/krk-ui/src/appkit/tabelle.rs` (`nach_lesebeginn`, `auswahl_merken`, `tab_gewechselt`); `crates/krk-ui/src/appkit/anwendung.rs` (`lesevorgaenge_starten`, `sichtbaren_lesen`); Commit `9322d5d`

---

## Was ist

Seit `9322d5d` gilt: die Vorschau beschreibt den ausgewählten Eintrag, und ohne
Auswahl den angezeigten Ordner. Der Anstoß dafür sitzt in `nach_lesebeginn`,
der einen Stelle, die Navigation und Auffrischung gemeinsam nachzieht.

**Zwei Wege in einen Ordner gehen nicht durch diese Stelle**, und der Coder der
Aufgabe T-7 hat beide beim Umsetzen benannt statt sie zu improvisieren:

- **Der Programmstart.** `lesevorgaenge_starten` → `sichtbaren_lesen` läuft an
  `nach_lesebeginn` vorbei. Ein frischer Start ohne gemerkte Sitzungsauswahl
  zeigt deshalb eine leere Vorschau, wo die Regel eine Ordnerzusammenfassung
  verlangt.
- **Der Tabwechsel.** `tab_gewechselt` ebenso wenig. Er ist nur so weit gedeckt,
  wie AppKit eine echte Änderung der Auswahl meldet; war im Zieltab nichts
  ausgewählt, meldet es nichts.

## Warum das zählt

Der Nutzer hat am 260825 ausdrücklich den Fall genannt, „nach Eintritt in den
Ordner, bevor der Zeilencursor bewegt wurde". Der Eintritt über die Navigation
ist gedeckt; der Eintritt über den Programmstart ist es nicht. Wer KRK startet
und die Werkbank vor sich hat, sieht damit weiterhin nichts, und das ist genau
die Lage, in der die Übersicht am meisten wert wäre.

Der Tabwechsel ist der leisere der beiden: er trifft nur einen Tab, in dem
nichts ausgewählt war.

## Was zu tun wäre

Nicht ohne Entscheidung. Der Plan nennt `nach_lesebeginn` als **die eine**
Stelle, und zwei weitere Anstöße daneben wären drei Türen zu derselben Regel.
Ob die zwei Wege durch `nach_lesebeginn` geführt werden oder einen eigenen
Anstoß bekommen, gehört entschieden, bevor es gebaut wird.

## Status

Offen. Gemessen ist der Codeweg, nicht das Gesehene: dass der Start wirklich
eine leere Vorschau zeigt, ist aus dem Baum gelesen und nicht am laufenden
Bündel beobachtet.

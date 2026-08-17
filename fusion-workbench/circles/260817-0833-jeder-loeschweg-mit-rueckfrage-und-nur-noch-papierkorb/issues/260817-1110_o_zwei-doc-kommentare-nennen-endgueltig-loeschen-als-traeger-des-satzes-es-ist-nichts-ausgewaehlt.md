# Zwei Doc-Kommentare nennen `endgueltig_loeschen` als Träger des Satzes „es ist nichts ausgewählt"

**Datum:** 260817-1110
**Gefunden von:** coderev, Durchsicht `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 5
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/appkit/anwendung.rs`
**Baumstand:** `472eb81`

## Der Befund

Zwei Doc-Kommentare berufen sich darauf, dass `endgueltig_loeschen` den Satz „es ist nichts
ausgewählt" für die leere Auswahl führe:

- `anwendung.rs:5526`, im Zweig der leeren Auswahl von `im_editor_oeffnen`:
  „sondern derselbe Satz, den `endgueltig_loeschen` seit der Runde 1 fuer die leere Auswahl
  fuehrt"
- `anwendung.rs:6276`, an `editormeldung_zeigen`:
  „`Self::endgueltig_loeschen` liest `aktiv` und meldet „es ist nichts ausgewählt" dorthin"

Seit Schritt 3 steht die Prüfung dort nicht mehr. Sie ist nach `loeschen_nach_rueckfrage`
(`:4620`) gewandert; daneben trägt sie weiterhin `auftrag_stellen` (`:5093`).

## Richtung

Beide Verweise auf die Stelle ziehen, an der der Satz jetzt steht. Sie stehen nicht auf der
Liste der 46 Nennungen, die Bündel E nachzieht: diese Liste ist gegen `b8e198e` aufgestellt
und kennt die beiden Verschiebungen dieses Bündels nicht.

---
Abgleich 260817-1129 (reconciler): **offen, am Baum nachgelesen.** Beide Stellen stehen unverändert: `anwendung.rs:5526` („derselbe Satz, den `endgueltig_loeschen` seit der Runde 1 fuer die leere Auswahl fuehrt") und `:6276` (`Self::endgueltig_loeschen` liest `aktiv`).

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

---
Resolved 260817-1302 (coder, T2): **behoben, beide Stellen.** Der Kommentar im Zweig der
leeren Auswahl von `im_editor_oeffnen` nennt den Satz jetzt als den, den KRK seit der Runde 1
fuehrt, und dazu die Stelle, die ihn fuer den Loeschweg seit dem 260817 traegt:
`loeschen_nach_rueckfrage` und nicht mehr `endgueltig_loeschen`, das ihn an jenen gemeinsamen
Rumpf abgegeben hat. Der Doc-Kommentar von `editormeldung_zeigen` nennt statt
`endgueltig_loeschen` jetzt `loeschen_nach_rueckfrage` als den einen Rumpf jedes
Loeschbefehls, die beiden Operationsbefehle ueber `auftrag_stellen`, und sagt in einem Satz,
dass die Meldung bis zum 260817 in `endgueltig_loeschen` stand — der Bezug auf `aktiv` hat
sich mit ihr nicht geaendert.

**Abweichung von der Aufzaehlung im Datensatz:** der Satz „es ist nichts ausgewählt" steht am
Baum an **vier** Stellen und nicht an zwei. Neben `loeschen_nach_rueckfrage` (`:4622`) und
`auftrag_stellen` (`:5093`), die der Datensatz nennt, tragen ihn `stapel_umbenennen`
(`:4893`) und `im_editor_oeffnen` (`:5530`) selbst. Nachzuzaehlen mit
`grep -n 'es ist nichts ausgewählt' crates/krk-ui/src/appkit/anwendung.rs`. Der berichtigte
Kommentar in `im_editor_oeffnen` nennt `loeschen_nach_rueckfrage`, `auftrag_stellen` und
`stapel_umbenennen`; die vierte Stelle ist er selbst.

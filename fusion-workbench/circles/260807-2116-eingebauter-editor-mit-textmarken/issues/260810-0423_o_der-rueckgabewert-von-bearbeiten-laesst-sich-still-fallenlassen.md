# Der Rückgabewert von `bearbeiten` lässt sich still fallenlassen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/editormodell.rs:530` (`Editormodell::bearbeiten`)
**Cross-references:** `issues/260810-0215_c_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`, Commit `d5993f1`

---

## Der Befund

`bearbeiten` liefert seit `d5993f1` ein `bool`, und der Doc-Kommentar sagt, was
daran hängt:

> Wer diesen Bestand fuehrt, hat ihn danach auf den Stand zu bringen; tut er es
> nicht, zeigt dieselbe Stelle in den beiden Texten von der Wandlung an auf
> Verschiedenes.

Die Funktion trägt kein `#[must_use]`. Ein Aufrufer, der den Wert fallenlässt,
übersetzt ohne Warnung; genau das tun heute siebzehn der einundzwanzig Aufrufe
unter `mod tests` in derselben Datei (Zeilen 1313, 1372, 1420, …, 2100 — allein
die vier in `ein_eingefuegtes_crlf_meldet_sich_und_ein_gewoehnlicher_anschlag_nicht`
lesen ihn). In der Anwendung
gibt es genau einen Aufrufer, `Editorbereich::text_zurueckschreiben`
(`editor.rs:1054`), und der wertet ihn aus.

## Warum das mehr ist als Stilfrage

Das `bool` ist die **einzige** Meldung, dass Stand und Textfläche
auseinanderliefen. Es gibt keine zweite Stelle, an der ein Vergessen auffiele:
der Defekt `260810-0215` war genau die Lage, in der niemand nachzog, und der
Bau war dabei grün. Ein zweiter Aufrufer, der später dazukommt und den Wert
nicht liest, stellt sie ohne eine einzige Warnung wieder her.

Das Projekt fährt an drei anderen Stellen bewusst vollständige
Fallunterscheidungen, damit eine neue Variante den Bau anhält. Hier ist
dieselbe Absicht ohne die Erzwingung aufgeschrieben.

## Was zu tun ist

`#[must_use = "wandelte das Bearbeiten, ist die Textflaeche nachzuziehen"]` an
die Funktion. Die siebzehn Stellen brauchen dann ein `let _ =` — und das ist
der Gegenwert, nicht der Preis: an jeder dieser Zeilen steht dann ausdrücklich,
dass die Prüfung die Meldung nicht braucht.

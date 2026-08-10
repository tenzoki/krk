# Der Rückgängigverwalter des Fensters trägt auch den Feldeditor der Umbenennung

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:297-326` (`rueckgaengigstapel_leeren`, Doc-Kommentar), `crates/krk-ui/src/appkit/tabelle.rs:1852-1868`
**Cross-references:** `issues/260809-1727_c_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`, Commit `2123e52`

---

## Der Befund

Der Doc-Kommentar von `rueckgaengigstapel_leeren` nennt den Umstand, dass der
Verwalter dem Fenster gehört, und schließt die Folge daraus aus:

> **Der Verwalter gehoert dem Fenster und nicht der Textflaeche.** Wer sonst
> noch in demselben Fenster Rueckgaengig-Handlungen anmeldet, verliert sie
> hier mit. Heute ist das niemand: der Editor ist die einzige Flaeche in KRK,
> die `allowsUndo` einschaltet, und der Feldeditor eines Suchfeldes fuehrt
> seinen Verlauf je Bearbeitung und nicht ueber diesen Verwalter.

Der Ausschluss redet über ein Suchfeld. Das beschreibbare Feld im Hauptfenster
ist ein anderes: `tabelle.rs:1858` schaltet für die Namensspalte
`feld.setEditable(true)` ein — das Umbenennen „direkt in der Liste" aus C4 der
Runde 1. Ein beschreibbares `NSTextField` bekommt beim Bearbeiten den
Feldeditor des **Fensters**, und der hängt an derselben Antwortkette wie die
Textfläche des Editors.

Damit ist der Satz „Heute ist das niemand" nicht geprüft, sondern über die
falsche Fläche geführt.

## Wie er erreichbar wäre

`stand_einsetzen` läuft nicht nur auf Befehl. Der Ladeausgang des
Arbeitsfadens kommt über den Einzugstakt und trifft den Delegierten zu einem
Zeitpunkt, den der Nutzer nicht wählt. Wer F4 auf eine große Datei drückt und
in der Lesespanne eine Umbenennung beginnt, tippt in den Feldeditor, während
`stand_erneuern` den Verwalter des Fensters leert.

**Inference, nicht gemessen:** dass AppKits Feldeditor seine
Rückgängig-Handlungen tatsächlich beim Verwalter des Fensters anmeldet, ist die
übliche Bauart von `NSTextField`, hier aber nicht am laufenden Bündel geprüft.
Die Messung ist Nutzerarbeit: umbenennen beginnen, ein paar Zeichen tippen, das
Eintreffen eines Ladeausgangs abwarten, `cmd+z` im Feld drücken.

## Die Wirkung, falls die Annahme trägt

Ein `cmd+z` im Umbenennungsfeld tut nichts. Kein Datenverlust: der Name steht
im Feld, wie der Nutzer ihn getippt hat, und `esc` bricht die Umbenennung
weiterhin ab.

## Was zu tun ist

Der Doc-Kommentar ist entweder auf das beschreibbare Namensfeld zu erweitern
oder — falls die Messung zeigt, dass der Feldeditor mitfällt — die Grenze ist
enger zu ziehen. `NSTextView` gibt über `undoManagerForTextView:` am
Delegierten die Möglichkeit, dem Editor einen **eigenen** Verwalter zu geben,
der nichts sonst im Fenster mitnimmt; das wäre die Bauart, die die Aussage „der
Aufrufer ist einer" auch für die Wirkung hielte und nicht nur für den Aufruf.

---
Resolved: Die Wirkung ist gemessen, und sie tritt nicht ein; der Doc-Kommentar
nennt jetzt die richtige Flaeche und die Messung statt der Vermutung.

**Die Messung, die dieser Datensatz als Nutzerarbeit ausgewiesen hat, ist
gefahren** — am 260810 auf macOS 15.7.7 (Build 24G720), an einem Fenster mit
einer `NSTextView` und einer beschreibbaren `NSTextField` darin, also der Lage,
die `tabelle.rs:1858` mit `feld.setEditable(true)` herstellt:

```
  Verwalter des Feldeditors      NSCellUndoManager   ─┐ zwei Objekte,
  Verwalter des Fensters         NSUndoManager       ─┘ nicht dasselbe
  removeAllActions am Fenster ─> Feldeditor: canUndo bleibt wahr
  undo im Feld danach         ─> der getippte Name ist zurueckgenommen
```

Der Feldeditor ist eine `NSTextView` — soweit war die Beschreibung richtig —,
**bekommt seinen Rueckgaengigverwalter aber von der `NSTextField`, die ihn
ausleiht, und nicht aus der Antwortkette.** `NSTextField` liefert ihm einen
eigenen `NSCellUndoManager`. Ein `rueckgaengigstapel_leeren` am Verwalter des
Fensters nimmt ihm deshalb nichts fort, und der Fall "F4 auf eine grosse Datei,
Umbenennung in der Lesespanne, `cmd+z` im Feld" hat die vermutete Wirkung nicht.

**Behoben ist damit der Satz und nicht der Code.** Der Doc-Kommentar von
`rueckgaengigstapel_leeren` in `crates/krk-ui/src/appkit/editor.rs` redete ueber
ein Suchfeld und fuehrte den Ausschluss damit ueber die falsche Flaeche; er nennt
jetzt das beschreibbare Namensfeld der Umbenennung aus C4 der Runde 1, zitiert
die Messung samt Datum und Systemfassung und sagt, was daraus folgt: die
Textflaeche des Editors ist die einzige in KRK, die diesen Verwalter benutzt.

**Der zweite Vorschlag dieses Datensatzes ist mitgemessen und nicht genommen.**
`undoManagerForTextView:` gaebe der Flaeche einen eigenen Verwalter, und dass er
vom Menueeintrag aus erreichbar blieb, war die offene Frage daran: gemessen
beantwortet `undo:` in der ganzen Antwortkette allein `NSWindow` — nicht
`NSTextView`, nicht `NSApplication`, nicht `NSResponder` —, und `NSWindow` nimmt
dabei den Verwalter des **Ersthelfers** und nicht seinen eigenen. Der Weg steht
also offen. Er wird nicht genommen, weil es keinen zweiten Anmelder gibt und ein
Verwalter mehr ein Mechanismus ohne Fall waere; ausserdem truege er den Umbau
des Ersetzens (`260810-0303`) in einen anderen Stapel als das Tippen der
Flaeche. Beides steht im Doc-Kommentar, damit ein spaeterer Leser den Vorschlag
nicht fuer ungeprueft haelt.

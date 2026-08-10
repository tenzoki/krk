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

# Ein eingefügtes CRLF bleibt nicht rücknehmbar, und der Grund liegt am Eingang der Fläche

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, abgetrennt beim Beheben von `260810-0303`
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::flaeche_richten`)
**Cross-references:** `issues/260810-0303_*_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`, `issues/260810-0215_c_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`, `issues/260810-0424_o_das-richten-der-flaeche-kopiert-den-text-eines-16-mb-dokuments-dreimal.md`

---

## Der Befund

Wer Text mit `\r\n` aus einer Windows-Quelle in den Editor einfügt, kann das
Einfügen nicht zurücknehmen: `cmd+z` danach tut nichts. Das ist der Rest von
`260810-0303`, nachdem dessen schwerere Hälfte — das Ersetzen aus S37 — behoben
ist.

Der Weg: `text_zurueckschreiben` gibt den Text der Fläche an
`Editormodell::bearbeiten`, das ihn in die gehaltene Form wandelt und `true`
meldet; `flaeche_richten` bringt die Fläche daraufhin auf den gewandelten Stand,
und weil dieser Schreibweg über `setString:` läuft und `setString:` keine
Rückgängig-Handlung anmeldet (gemessen, `260810-0303`), muss der Verlauf fallen.
Bliebe er stehen, zeigte die Handlung des Einfügens auf einen Bereich, der um die
Zahl der weggefallenen `\r` zu lang ist, und ein `cmd+z` löschte Zeichen hinter
dem Eingefügten mit.

## Warum die Behebung von 260810-0303 ihn nicht mit erledigt hat

Zwei Stücke fehlen, und jedes für sich genügt:

- **Der Text, den die Fläche vor dem Richten trug, ist kein gültiger Stand.** Er
  trägt das `\r`, das der Stand nach dem Modulkopf von `krk_core::text` nie
  trägt. Ein Umkehrpunkt darauf ließe sich herstellen, aber nur an der Fläche und
  nicht im Modell — und damit liefen die beiden genau so auseinander, wie
  `260810-0215` es beschreibt.
- **Der Stand vor dem Einfügen ist an dieser Stelle schon fort.**
  `Editormodell::bearbeiten` hat ihn überschrieben, bevor
  `text_zurueckschreiben` zu `flaeche_richten` kommt. Ihn vorher abzuschreiben
  hieße, den ganzen Stand **je Tastendruck** zu kopieren, also bis zu 16 MB pro
  Anschlag; `260810-0424` führt diese Kette bereits als zu teuer.

## Was zu prüfen wäre

Was der Nutzer zurückhaben will, ist nicht die Wandlung, sondern das Einfügen.
Aufzuzeichnen ist es deshalb dort, wo es geschieht: am **Eingang** der Fläche,
über `textView:shouldChangeTextInRanges:replacementStrings:`. Genau dieser Weg
ist bei der Behebung von `260810-0215` mit Gründen **nicht** genommen worden, und
die Gründe stehen unverändert an `flaeche_richten`: er müsste die Regeln der
Wandlung ein zweites Mal tragen, und dabei wären sie nicht dieselben, weil die
Bytefolgenmarke nach ihrer Stelle im ganzen Text fällt und ein eingefügtes Stück
seine Stelle nur beim Einfügen kennt.

Zwei Wege wären zu prüfen, und beide reichen über `editor.rs` hinaus:

1. **Ein Verlauf im `Editormodell`**, der den Stand vor einer Bearbeitung hält
   und dabei mit dem Speicher haushaltet (also nicht je Anschlag den ganzen Stand
   kopiert, sondern die geänderte Stelle). Das ist eine eigene Bauart und keine
   Zeile.
2. **Ein Eingangsfilter, der allein das `\r` abfängt** und die Wandlung damit vor
   das Einfügen zieht, statt sie danach nachzuholen. Dann bliebe die Handlung der
   Fläche gültig, weil Fläche und Stand nie auseinanderlaufen. Der Preis ist die
   zweite Formulierung der Wandlungsregeln, und ob sie sich auf die eine Regel
   „kein `\r`" beschränken lässt, ohne die Bytefolgenmarke mitzunehmen, ist die
   offene Frage.

## Was heute hält

Kein Verlust von Text und keine falsche Wirkung: nach dem Richten trägt die
Fläche den Stand, den das Modell hält, und `cmd+z` tut nichts. Der Doc-Kommentar
von `flaeche_richten` hält den Preis und seine beiden Gründe fest.

---
Resolved: Ein `cmd+z` nach einem eingefügten `\r\n` nimmt das Einfügen jetzt
zurück. Gebaut ist **keiner** der beiden geprüften Wege, sondern ein dritter, der
in dieser Datei bleibt und beide Einwände dieses Datensatzes entkräftet.

**Der Umkehrpunkt entsteht vor der Wandlung, und nur dann, wenn sie bevorsteht.**
`Editorbereich::text_zurueckschreiben` fragt vor dem Ruf an
`Editormodell::bearbeiten` mit `krk_core::text::datei::ist_in_gehaltener_form` —
derselben Bedingung, an der `bearbeiten` seine Wandlung entscheidet — und schreibt
den Stand des Modells nur dann ab, wenn die Antwort „nein" lautet. Damit fällt der
zweite Einwand dieses Datensatzes: der Stand vor dem Einfügen ist **nicht** mehr
fort, und abgeschrieben wird er nicht je Tastendruck, sondern nur auf dem Weg, der
die Fläche ohnehin neu beschreibt. Der zusätzliche Durchlauf ist gemessen
(260810, `--release`): 0,017 ms bei 229 kB, 0,13 ms bei 1,8 MB, 1,8 ms bei 19 MB,
gegen 0,98 / 7,6 / 88 ms für das Umschreiben aus UTF-16, das jedem Ruf vorausgeht.
Zwei Prozent Aufpreis.

**Der erste Einwand fällt mit ihm.** Der Umkehrpunkt trägt nicht den `\r`-Text der
Fläche, sondern den gehaltenen Stand aus dem Modell; er ist damit ein gültiger
Stand, und Fläche und Modell laufen nicht auseinander. `Editorbereich::umkehren`
setzt ihn über denselben einen Schreibweg, den das Ersetzen aus S37 benutzt.

**Der Verlauf davor kann trotzdem nicht bleiben, und das ist der benannte Preis.**
Die Handlung, die die Fläche für das Einfügen selbst angemeldet hat, zeigt auf
einen Bereich, der um die Zahl der weggefallenen `\r` zu lang ist; ein
`NSUndoManager` lässt keine einzelne Handlung herausnehmen. Deshalb trägt
`Verlauf` seit dem 260810 einen dritten Wert, `TraegtNurDiese`: der Stapel fällt,
und die eine gültige Handlung wird danach neu angemeldet. **Was der Nutzer
merkt:** das erste `cmd+z` nimmt das Einfügen zurück, ein zweites tut nichts. Vor
dieser Behebung tat schon das erste nichts, und der Verlauf davor fiel ebenso —
die Änderung ist damit in jeder Hinsicht ein Gewinn und in keiner ein Verlust.

**Dass eine Anmeldung nach `removeAllActions` steht und wirkt, ist gemessen und
nicht angenommen.** Die Frage war offen: die bestehende Probe
`ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung` belegt, dass
`removeAllActions` die vom Verwalter selbst geöffnete Ereignisgruppe mit abräumt,
und daraus folgt nicht, dass er im selben Ereignis eine neue öffnet. Die neue
Probe `eine_anmeldung_nach_dem_leeren_steht_im_stapel` in
`crates/krk-ui/src/appkit/editor.rs` fährt die Betriebsart der Laufzeit samt
Umlauf der Laufschleife und misst beides: die Handlung steht, sie wirkt, und die
Handlung davor ist fort.

**Der Eingangsfilter über `textView:shouldChangeTextInRanges:replacementStrings:`
bleibt ungenommen**, und die Gründe an `flaeche_richten` stehen unverändert. Er
wäre der Weg, auch den Verlauf **vor** dem Einfügen zu retten; er kostete die
Wandlungsregeln ein zweites Mal und verlangte, das Einfügen selbst noch einmal
anzumelden, damit der Verlauf hält. Der Rest, den er lösen würde, ist damit
kleiner geworden: nicht mehr „das Einfügen ist nicht rücknehmbar", sondern „das
Einfügen ist rücknehmbar, was davor liegt nicht mehr".

**Was Nutzerarbeit bleibt:** die Wirkung am laufenden Bündel, also dass ein
`cmd+z` nach einem eingefügten `\r\n` den vorigen Stand samt Schreibmarke zeigt.
Die Schreibmarke ist dabei die genauere Frage: der Umkehrpunkt merkt sie **nach**
dem Einfügen, weil `textDidChange:` erst danach kommt, und
`Editorbereich::auswahl_setzen` beschneidet sie auf die Länge des
wiederhergestellten Textes. Beim Anfügen am Dateiende ist das genau die
Einfügestelle, mitten im Text kann sie dahinter liegen.

# CLAUDE.md nennt die Freigabe des Rückgängig-Blocks ungemessen — seit heute hält sie eine Probe

---
Der Absatz „Der Rückgängigstapel des Editors trägt ein Budget in Bytes, keine Tiefengrenze"
unter `## Was man nicht sieht, wenn man es nicht weiß` schließt mit dem Satz:

> Ob die Freigabe eines angemeldeten Blocks wirklich abträgt, ist in diesem Baum durch nichts
> gemessen; der Datensatz dazu ist als Lage angenommen geschlossen, nicht behoben
> (`issues/260810-1341_*_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`).

Beide Hälften stimmen seit dem 260908 nicht mehr.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:**
`260826-1327_*_die-freigabe-des-rueckgaengig-blocks-ist-mit-verwalter-ohne-fenster-messbar-und-die-begruendung-fuer-ungemessen-traegt-nicht-mehr.md`
(geschlossen, die Messung), `260810-1341_*_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`
(die zitierte Lage)

## Was der Baum trägt

`crates/krk-ui/src/appkit/editor.rs`, Prüfmodul:
`der_verwalter_gibt_den_block_auf_allen_vier_wegen_frei` meldet einen Block an, der eine
`Stapellast` hält, und fährt die vier Wege des Kopfes von `Stapellast` je an einem eigenen
`NSUndoManager`: `removeAllActions`, `undo`, die neue Anmeldung nach einem `undo` und das
Fallenlassen des Verwalters. Alle vier geben frei, gemessen am Zähler.

Der Kopf von `Stapellast` trägt seither den Abschnitt „Die Freigabe des Blocks ist seit dem
260908 gemessen" statt „ist geschlossen und nicht gemessen".

## Was die Messung daneben ergeben hat

**Ohne einen offenen `autoreleasepool` fällt der Zähler auf keinem der vier Wege.** Der
Freigabeverbund hält den Block bis zu seinem Ende; der Editor bekommt seinen von der
Ereignisschleife der Anwendung, je Ereignis einen. Der Kopf von `Stapellast` hielt das bis
dahin unter „Genauigkeit im Augenblick leistet die Hülle ohnehin nicht" nur für **möglich**
und trägt es jetzt als gemessen. Ein Umlauf über `runMode:beforeDate:` ersetzt den Verbund
nicht: er kehrt ohne Eingabequelle sofort zurück.

## Abnahmeprüfung

Der Absatz in `CLAUDE.md` sagt, wo die Messung steht, oder nennt die Freigabe nicht mehr
ungemessen. Der Verweis auf `260810-1341` darf bleiben — er hält den Stand von damals fest —,
sagt dann aber, dass die Lage sich geändert hat.

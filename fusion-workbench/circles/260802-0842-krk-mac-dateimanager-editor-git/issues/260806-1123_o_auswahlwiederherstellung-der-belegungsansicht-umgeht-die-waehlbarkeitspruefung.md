Die Auswahlwiederherstellung der Belegungsansicht umgeht die Wählbarkeitsprüfung

---

`crates/krk-ui/src/appkit/belegungsansicht.rs:286-296` (`nachziehen`)
stellt die Auswahl nach `reloadData` per
`selectRowIndexes_byExtendingSelection` am alten Zeilenindex wieder her.
Der programmatische Weg fragt `tableView:shouldSelectRow:` nicht — die
Sperre für Überschriftszeilen (`belegungsansicht.rs:152-155`) greift nur
für Maus und Tastatur. Nach einem Zurücksetzen baut das Modell seine
Zeilenliste neu (`belegungsmodell.rs:377-381`); landete der alte Index
dabei auf einer Überschrift, wäre eine nicht wählbare Zeile ausgewählt,
und "Zuweisen" meldete eine Aufforderung mit leerem Funktionsnamen
(`belegungsansicht.rs:169-179`, `unwrap_or_default`).

Heute ist das nicht auslösbar: jede geladene Belegung trägt exakt den
Funktionsbestand der Auslieferung, weil das Einlesen unbekannte
Kennungen abweist (`crates/krk-core/src/tasten/belegung.rs:713-718`) und
fehlende ergänzt (`belegung.rs:751-763`). Gleicher Bestand heißt gleiche
Gruppengrößen, also identische Überschrift-Indizes vor und nach dem
Zurücksetzen. Die Absicherung hängt aber an einem Invariant in einem
anderen Crate, den an der Aufrufstelle nichts festhält.

Fix-Richtung: in `nachziehen` den wiederhergestellten Index gegen
`ist_ueberschrift` prüfen und notfalls auf
`Belegungsmodell::erste_funktionszeile` ausweichen — eine Zeile, kein
neuer Mechanismus. Alternativ genügt ein Kommentar an der Aufrufstelle,
der den tragenden Invariant benennt.

---

Gefunden bei der Coderev-Durchsicht des Commits ccaf821. Kein heute
auslösbarer Defekt, sondern eine unbenannte crate-übergreifende
Kopplung; Schwere: niedrig. Die Leiste hat dieselbe Bauart
(Überschriften über `shouldSelectRow`), hält ihre Auswahl aber im
eigenen Modell auf wählbaren Zeilen fest (`leistenmodell.rs:140`) und
ist nicht betroffen. Adressat: coder.

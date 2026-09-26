`cmd+z` bei offener Zelle nimmt einen Tabellenumbau zurück und verliert oder verschiebt den getippten Text

---

Solange eine Zelle der Eintragstabelle offen ist, erreicht `cmd+z` als Menükürzel den Rückgängigverwalter des Fensters, den die Zelle mit den Tabellenhandlungen teilt. Ist in der Zelle nichts Getipptes mehr zurückzunehmen, nimmt `cmd+z` den letzten Tabellenumbau zurück; die Tabelle lädt neu, ohne nach der offenen Zelle zu fragen (`crates/krk-ui/src/appkit/eintragsansicht.rs`, das Neuladen in `zeilen_zeigen`). Der offene Text geht dann still verloren oder landet an derselben Zeilennummer, an der nach der Rücknahme eine andere Aufgabe steht. Das Dateifenster sperrt denselben Fall (`crates/krk-ui/src/appkit/tabelle.rs`, Sperre vor dem Neuladen bei laufender Bearbeitung); die Tabelle im Editor hat keine Sperre.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Befund aus `260926-0811-zweitlesung-stufe-3-aufgabeneditor.md`, Frage 1 und „Vor Stufe 4 beheben“; der geteilte Rückgängigverwalter ist dort als Inference markiert und vor dem Umbau zu messen.

**Abnahme**

- Solange eine Zelle offen ist, ändert nichts von außen den Stand: `cmd+z` endet am Anfang der Zelle (Empfehlung der Zweitlesung: eigener Feldeditor mit eigenem Rückgängigverwalter über `windowWillReturnFieldEditor:toObject:`, vorher gemessen), und jedes Neuladen der Tabelle verwirft oder übernimmt eine offene Zelle vorher.
- Eine Probe hält fest, dass `umkehren` und das Eintreffen einer geladenen Datei keine offene Zelle stehen lassen.
- Die drei Nutzerprüfungen der Zweitlesung stehen im Plan: B über A schieben, B doppelklicken, ohne Tippen `cmd+z` → Reihenfolge A, B, Texte unverändert; drei Zeichen tippen, `cmd+z` viermal; während des Tippens `cmd+w`, Fenster einblenden, `cmd+q` → Datei trägt den Text oder die Rückfrage kommt.

---
Resolved: Beide Hälften der Abnahme stehen am Baum, gemessen vor dem Bau (`messungen/260926-0828-zellen-rueckgaengig.txt`, Programm und Einzelbelege unter `spikes/zellen-rueckgaengig/`). Die Messung hat den Defekt am laufenden Programm nachgestellt, und zwar als Fall A: ohne Getipptes nimmt `cmd+z` in der offenen Zelle den Umbau zurück und schließt sie, und `reloadData` unter einer offenen Zelle beendet sie mit Zeile -1, der Text fällt still. Der Feldeditor hat dabei einen eigenen Verwalter (`NSCellUndoManager`, der Befund vom 260810 stimmt), aber `NSWindow.undo:` geht bei leerem Zellenstapel an den Verwalter des Fensters.

**`cmd+z` endet am Anfang der Zelle.** Die Zellen der Eintragstabelle bekommen einen eigenen Feldeditor, `Zelleneditor` in `crates/krk-ui/src/appkit/eintragsansicht.rs`, über `windowWillReturnFieldEditor:toObject:` beim Fensterdelegierten (`crates/krk-ui/src/appkit/fenster.rs`) und `Eintragsansicht::feldeditor_fuer`, allein für Felder unter der Tabelle. **Abweichung von der Empfehlung der Zweitlesung, aus der Messung:** ein eigener Verwalter allein hilft nicht, `NSWindow.undo:` fragt beim Feldeditor dessen `undoManager` nicht. Der Zelleneditor beantwortet deshalb `undo:` und `redo:` selbst an seinem Verwalter und leert ihn bei jedem Beginn einer Zelle (`becomeFirstResponder`). AppKit richtet ihn ein wie den eigenen (21 Eigenschaften gleich), und die mehrzeilige Form aus 4.2 wächst mit ihm genauso (20/36/52 pt). Der Menüeintrag „Rückgängig“ bleibt am Anfang der Zelle bedienbar und tut dort nichts: grau würde er nur mit einer zweiten Antwort auf `validateMenuItem:`, und die Ausgrauung entscheidet in diesem Baum genau eine Stelle (C2.17 der Runde 7).

**Jedes Neuladen verwirft eine offene Zelle vorher.** `Eintragsansicht::zeilen_zeigen`, die eine Stelle mit `reloadData`, verwirft zuerst eine Zelle, unter der sich der Stand von außen geändert hat (`fremde_zelle_verwerfen` über `bearbeitung_verwerfen`), vor dem Vergleich der Zeilen. Ausgenommen ist das eigene Ende: während `controlTextDidEndEditing:` meldet `laufende_zelle` die Zelle noch (gemessen), und eine Marke `endet` für die Dauer des Festschreibens nimmt es aus. `zelle_uebernehmen` und seine Rufer sind unverändert.

**Proben** in `crates/krk-ui/src/appkit/editor.rs`, zerlegt, weil ein Fenster unter `libtest` abstürzt: `den_eigenen_feldeditor_bekommen_allein_die_felder_der_eintragstabelle`, `cmd_z_in_der_zelle_endet_am_anfang_der_zelle_und_laesst_den_umbau` (die Nutzerprüfung 1 ohne Fenster: `undo:` am Zelleneditor mit leerem Stapel lässt einen Umbau im zweiten Verwalter stehen), `jede_zelle_beginnt_mit_leerem_stapel` (am Rumpf) und `umkehren_und_eine_eingetroffene_datei_lassen_keine_offene_zelle_stehen` (die Kette von `umkehren` und `ladeausgang_einziehen` bis `zeilen_zeigen`, das Verwerfen vor Vergleich und `reloadData`, `reloadData` genau einmal, dazu an einer echten Zelle die Frage vor dem Neuladen, auch im eigenen Ende). Die drei Nutzerprüfungen der Zweitlesung stehen im Plan unter Stufe 3; die Erwartung der ersten ist dem gebauten Verhalten angepasst (Zelle bleibt offen, Reihenfolge B, A; nach `esc` nimmt `cmd+z` das Verschieben zurück). `make check` endet mit 0.

Dieselbe Falle hat vermutlich das Umbenennen im Dateifenster: `260926-0840_*_cmd-z-im-umbenennungsfeld-des-dateifensters-erreicht-bei-leerem-feldstapel-den-verlauf-des-editors.md`.

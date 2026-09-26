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

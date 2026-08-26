Die Frage, welche Tasten die Schaltflächen der Belegungsansicht behalten, ist seit Runde 7 gebaut und steht noch offen
---
`shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md` trägt `_o_` und leere `Answered:`/`Implemented:`-Zeilen. Der Baum hat Möglichkeit 1 gebaut: „Zuweisen“ auf Cmd+T, „Fertig“ auf Cmd+Eingabe, jedes Zeichen einschließlich Leertaste in die Suche. Der Abgleich der Runde 7 hat das am 260813 notiert („Gebaut auf der Empfehlung“), ohne den Datensatz zu bewegen.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/appkit/belegungsansicht.rs:22` („Zuweisen“ (Cmd+T)), `:27` („Fertig“ (Cmd+Eingabe) / esc), `:710` und `:745` (die Begründung: Leertaste und Eingabetaste sind Zeichen und gehören der Suche).
- `crates/krk-ui/src/belegungsmodell.rs:701-709`: `zeichen_anhaengen` nimmt jedes Zeichen aus `traegt_ein_dateiname`, die Leertaste eingeschlossen; `:1684-1693` misst „spalte änderungsdatum“ mit Leerzeichen.
- `fusion-workbench/circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0647-reconciliation.md:132`: „Gebaut auf der Empfehlung: Cmd+T, Cmd+Eingabe, Cmd+R“.

`CLAUDE.md` zählt den Datensatz unter den offenen Fragen, die künftige Arbeit binden; gebunden ist hier nichts mehr Offenes. Der Nutzer hat die Empfehlung nie ausdrücklich angenommen, also ist die richtige Zeile `Implemented:` mit dem Hinweis, dass die Antwort aus dem Bau und nicht aus einer Nutzerantwort stammt — oder eine Nutzerfrage, ob er den Bau so annimmt.

## Vorschlag

Reconciler: `Implemented: crates/krk-ui/src/appkit/belegungsansicht.rs:710` eintragen und auf `_i_` setzen, oder dem Nutzer die Annahme vorlegen.

Der Doc-Kommentar von `umbenennung_beenden` nennt den Fokusverlust als Aufrufer, die Messung derselben Sitzung widerlegt das

---

`crates/krk-ui/src/appkit/tabelle.rs:1727-1729` sagt: „Gerufen aus der Aktion des Feldes,
also wenn der Nutzer die Eingabe mit Return abschliesst **oder die Zelle verlaesst**." Die
Messung vom 260815, festgehalten in
`shared/issues/260815-2125_o_verlaesst-der-nutzer-die-offene-namenszelle-…`, sagt das
Gegenteil: beim Fokusverlust kommt **keine** Aktion. Der Commit `3b128c3` hat genau diesen
Absatz angefasst (Satz zu Escape angehängt) und den falschen Halbsatz stehen lassen.

---

**Schwere:** mittel. Der Satz ist die Stelle, an der ein Leser nachsieht, welche Ausgänge
der Bearbeitung verdrahtet sind. Er nennt zwei von dreien und behauptet den dritten als
erledigt; genau daraus entsteht der Befund, den niemand mehr sucht.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:1725-1741`
**Domain:** code

## Die Messung

Aus der Tabelle in `260815-2125_o_…`, am 260815 auf macOS 15.7.7 an einer `NSTableView` mit
bearbeitbarer Zelle:

| Ausgang | Aktion |
|---|---|
| Return (`insertNewline:`) | ja |
| Escape (`cancelOperation:`) | nein |
| Fokusverlust (`makeFirstResponder:` auf die Tabelle) | **nein** |

`sendsActionOnEndEditing` steht auf 0, und das ist die Vorgabe des Systems.

## Vorschlag

Den Halbsatz durch die drei Ausgänge ersetzen, jeder mit seinem Weg: Return schickt die
Aktion und landet hier; Escape läuft über `abortEditing` und landet bei
`Namensfeld::bearbeitung_abbrechen`; der Fokusverlust schickt nichts und ist offen
(`shared/issues/260815-2125_o_…`). Damit steht die vollständige Fallunterscheidung an der
Stelle, an der heute eine unvollständige steht.

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

---

**Resolved:** 260816-1017, der Vorschlag ist umgesetzt. Der Doc-Kommentar von
`umbenennung_beenden` zählt jetzt die drei Ausgänge einzeln auf, jeden mit seinem Weg:
Return schickt die Aktion und landet dort; Escape läuft über `abortEditing` und landet bei
`Namensfeld::bearbeitung_abbrechen`; jedes übrige Ende schickt keine Aktion und landet bei
`Namensfeld::bearbeitung_beendet`. Der falsche Halbsatz "oder die Zelle verlässt" steht
nicht mehr da, und der Kommentar benennt ausdrücklich, dass er dort stand.

Zwei Dinge kommen dazu, die es bei der Meldung noch nicht gab. Der dritte Ausgang ist
nicht mehr offen: der Nutzer hat ihn am 260816-0935 entschieden
(`shared/decisions/260816-0021_*_verwirft-oder-uebernimmt-ein-klick-neben-die-offene-namenszelle.md`,
verwerfen wie Escape), und die Anzeigehälfte ist gebaut
(`shared/issues/260815-2125_c_…`). Und die Zuschreibung an C4 ist berichtigt: der Satz
"Return übernimmt, Escape verwirft" stammt aus dem **Plan** der Runde 1, nicht aus dem
Abnahmekriterium.

Behoben in derselben Änderung wie `260815-2125`, weil es derselbe Absatz derselben Datei
ist.

`make check` — exit 0. Verlauf:
`shared/history/260816-1017-coder-anzeigeform-an-jedem-ende-ohne-umbenennung.md`

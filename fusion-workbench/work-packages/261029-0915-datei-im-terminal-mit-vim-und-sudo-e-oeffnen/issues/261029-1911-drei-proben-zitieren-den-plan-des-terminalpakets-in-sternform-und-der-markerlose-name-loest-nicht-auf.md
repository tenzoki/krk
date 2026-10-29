Drei Proben zitieren den Plan des Terminalpakets in Sternform, und der markerlose Name löst nicht auf
---
Der Plan heißt `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md` und trägt keinen Marker. Drei Doc-Kommentare im Bereich `fd82056a..7a69a051` zitieren ihn als `261029-0920_*_plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`. Die Sternform verlangt `_<buchstabe>_` nach dem Stempel, der Name hat dort `-`; `find fusion-workbench -name '261029-0920_*_plan-datei-im-terminal*'` liefert nichts.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

Fundstellen (Stand `7a69a051`):

- `crates/krk-core/tests/belegung.rs:3361` (Doc von `die_auslieferung_teilt_opt_cmd_e_zwischen_terminal_und_editor`)
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs:2707` (Doc von `opt_cmd_e_oeffnet_im_dateifenster_das_terminal_und_schliesst_im_editor`)
- `crates/krk-ui/src/menuemodell.rs:1123` (Doc von `opt_cmd_e_zeigt_im_menue_das_terminal_und_nicht_das_schliessen`)

Die übrigen Zitate desselben Pakets im Bereich (`terminalskript.rs`, `terminal.rs`, `default-keymap.toml`, `menuemodell.rs:1124`) stehen richtig in der markerlosen Form. Regel: `fusion-workbench-conventions.md` `## Filename Patterns`, ein markerloser Name wird zitiert, wie er steht.

Abnahme: An den drei Stellen steht `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`. `git grep -n '261029-09[0-9][0-9]_\*_' -- crates resources HowTo.md` gibt nichts aus.
